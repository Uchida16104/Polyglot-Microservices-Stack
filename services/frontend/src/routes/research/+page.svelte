<script lang="ts">
  import { onMount } from 'svelte';

  type ResearchRuntime = 'zig' | 'mojo' | 'dafny' | 'fstar';

  interface ResearchJob {
    id: string;
    runtime: string;
    status: string;
    verification_output: string | null;
    execution_output: string | null;
    error_output: string | null;
    exit_code: number | null;
    duration_ms: number | null;
    created_at: string;
  }

  const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:8000/api';

  const runtimeStarters: Record<ResearchRuntime, string> = {
    zig: `const std = @import("std");

pub fn main() void {
    std.debug.print("Hello from Zig!\\n", .{});
}`,
    mojo: `fn main():
    print("Hello from Mojo!")`,
    dafny: `method Main() {
    print "Hello from Dafny!\\n";
}

method Factorial(n: nat) returns (result: nat)
    ensures result >= 1
{
    if n == 0 { return 1; }
    var r := Factorial(n - 1);
    return n * r;
}`,
    fstar: `module Hello

let main () : ML unit =
    FStar.IO.print_string "Hello from F*!\\n"`,
  };

  const runtimeDescriptions: Record<ResearchRuntime, string> = {
    zig: 'Systems programming language. Compiles via zig build-exe, then executes the binary.',
    mojo: 'AI-accelerated Python superset by Modular. Runs via mojo run.',
    dafny: 'Formal verification language. Verifies proof obligations first, then executes via dafny run.',
    fstar: 'Proof assistant and dependently-typed language. Runs SMT-based verification via fstar.exe.',
  };

  let runtime: ResearchRuntime = 'zig';
  let sourceCode = runtimeStarters['zig'];
  let flags = '';
  let jobs: ResearchJob[] = [];
  let loading = true;
  let submitting = false;
  let errorMsg = '';

  function getToken(): string {
    return document.cookie.split('; ').find(r => r.startsWith('access_token='))?.split('=')[1] ?? '';
  }

  async function apiGet(path: string): Promise<unknown> {
    const res = await fetch(`${API_BASE}${path}`, {
      headers: { Authorization: `Bearer ${getToken()}`, Accept: 'application/json' },
    });
    if (!res.ok) throw new Error((await res.json()).message ?? res.statusText);
    return res.json();
  }

  async function apiPost(path: string, body: unknown): Promise<unknown> {
    const res = await fetch(`${API_BASE}${path}`, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${getToken()}`,
        'Content-Type': 'application/json',
        Accept: 'application/json',
      },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error((await res.json()).message ?? res.statusText);
    return res.json();
  }

  onMount(async () => {
    try {
      jobs = (await apiGet('/research/compile')) as ResearchJob[];
    } catch (e: unknown) {
      errorMsg = (e as Error).message;
    } finally {
      loading = false;
    }
  });

  function handleRuntimeChange() {
    sourceCode = runtimeStarters[runtime];
    flags = '';
  }

  async function handleSubmit() {
    submitting = true;
    errorMsg = '';
    try {
      const submitted = (await apiPost('/research/compile', {
        runtime,
        source_code: sourceCode,
        flags: flags || null,
      })) as ResearchJob;

      jobs = [submitted, ...jobs];

      let attempts = 0;
      while (attempts < 90) {
        await new Promise(r => setTimeout(r, 1500));
        const updated = (await apiGet(`/research/compile/${submitted.id}`)) as ResearchJob;
        jobs = jobs.map(j => (j.id === updated.id ? updated : j));
        if (updated.status === 'completed' || updated.status === 'failed') break;
        attempts++;
      }
    } catch (e: unknown) {
      errorMsg = (e as Error).message;
    } finally {
      submitting = false;
    }
  }

  const statusBadge: Record<string, string> = {
    queued: 'badge-info',
    running: 'badge-warning',
    completed: 'badge-success',
    failed: 'badge-danger',
    cancelled: 'badge-default',
  };

  const runtimeColor: Record<string, string> = {
    zig: 'text-orange-400',
    mojo: 'text-red-400',
    dafny: 'text-blue-400',
    fstar: 'text-purple-400',
  };
</script>

<svelte:head><title>Research Runtimes — Polyglot</title></svelte:head>

<div class="flex items-center justify-between mb-6">
  <div>
    <h1 class="text-2xl font-bold text-white">Research Runtimes</h1>
    <p class="font-mono text-xs text-gray-500 mt-1">Python FastAPI :8004 · Zig · Mojo · Dafny · F*</p>
  </div>
</div>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <div class="flex flex-col gap-4">
    <div class="card">
      <div class="flex gap-3 mb-4">
        {#each ['zig', 'mojo', 'dafny', 'fstar'] as rt}
          <button
            on:click={() => { runtime = rt as ResearchRuntime; handleRuntimeChange(); }}
            class="flex-1 py-2 font-mono text-xs rounded-lg border transition-colors {runtime === rt ? 'border-accent bg-accent/10 text-accent' : 'border-border text-gray-500 hover:border-gray-400'}"
          >
            {rt}
          </button>
        {/each}
      </div>

      <p class="font-mono text-xs text-gray-500 mb-4 leading-relaxed border-l-2 border-accent/40 pl-3">
        {runtimeDescriptions[runtime]}
      </p>

      <div class="mb-3">
        <label class="block font-mono text-xs text-gray-400 mb-1">Source Code</label>
        <textarea
          bind:value={sourceCode}
          class="input font-mono text-xs h-64 resize-none"
          spellcheck="false"
        ></textarea>
      </div>

      <div class="mb-4">
        <label class="block font-mono text-xs text-gray-400 mb-1">Flags (optional)</label>
        <input type="text" bind:value={flags} class="input font-mono text-xs" placeholder="--no-verify" />
      </div>

      {#if errorMsg}
        <div class="mb-3 p-3 rounded-lg border border-danger/40 bg-red-950/20 font-mono text-xs text-danger">
          {errorMsg}
        </div>
      {/if}

      <button on:click={handleSubmit} class="btn-primary w-full" disabled={submitting}>
        {submitting ? 'Submitting...' : `Run ${runtime}`}
      </button>
    </div>
  </div>

  <div class="flex flex-col gap-3">
    <h2 class="font-semibold text-sm text-white">Job History</h2>

    {#if loading}
      <p class="font-mono text-xs text-gray-500 animate-pulse">Loading jobs...</p>
    {:else if jobs.length === 0}
      <div class="card text-center py-10">
        <p class="font-mono text-xs text-gray-500">No research jobs yet.</p>
      </div>
    {:else}
      {#each jobs as job}
        <div class="card">
          <div class="flex items-center justify-between mb-2">
            <span class="font-mono text-xs font-bold uppercase {runtimeColor[job.runtime] ?? 'text-gray-400'}">
              {job.runtime}
            </span>
            <span class="badge {statusBadge[job.status] ?? 'badge-default'}">{job.status}</span>
          </div>

          <p class="font-mono text-xs text-gray-600 mb-2">{job.id.slice(0, 20)}...</p>

          {#if job.duration_ms}
            <p class="font-mono text-xs text-gray-500 mb-2">
              {job.duration_ms}ms · exit {job.exit_code}
            </p>
          {/if}

          {#if job.verification_output}
            <div class="mb-2">
              <p class="font-mono text-xs text-gray-400 mb-1">Verification</p>
              <pre class="bg-bg rounded p-3 font-mono text-xs text-blue-300 overflow-auto max-h-32 whitespace-pre-wrap">{job.verification_output}</pre>
            </div>
          {/if}

          {#if job.execution_output}
            <div class="mb-2">
              <p class="font-mono text-xs text-gray-400 mb-1">Output</p>
              <pre class="bg-bg rounded p-3 font-mono text-xs text-success overflow-auto max-h-32 whitespace-pre-wrap">{job.execution_output}</pre>
            </div>
          {/if}

          {#if job.error_output}
            <div>
              <p class="font-mono text-xs text-gray-400 mb-1">Errors</p>
              <pre class="bg-bg rounded p-3 font-mono text-xs text-danger overflow-auto max-h-32 whitespace-pre-wrap">{job.error_output}</pre>
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</div>
