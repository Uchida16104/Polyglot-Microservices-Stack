<script lang="ts">
  import { onMount } from 'svelte';
  import { compile, projects, type Project, type CompileJob, type ExecuteResult } from '$lib/api';

  let projectId = '';
  let language = 'rust';
  let sourceCode = `fn main() {\n    println!("Hello, polyglot!");\n}`;
  let compilerFlags = '';
  let stdinData = '';

  let userProjects: Project[] = [];
  let compileJob: CompileJob | null = null;
  let executeResult: ExecuteResult | null = null;
  let compiling = false;
  let executing = false;
  let errorMsg = '';

  const languages = ['rust', 'c', 'cpp', 'go', 'python', 'javascript'];

  const starters: Record<string, string> = {
    rust: `fn main() {\n    println!("Hello, polyglot!");\n}`,
    c: `#include <stdio.h>\nint main() {\n    printf("Hello, polyglot!\\n");\n    return 0;\n}`,
    cpp: `#include <iostream>\nint main() {\n    std::cout << "Hello, polyglot!" << std::endl;\n    return 0;\n}`,
    go: `package main\nimport "fmt"\nfunc main() {\n    fmt.Println("Hello, polyglot!")\n}`,
    python: `print("Hello, polyglot!")`,
    javascript: `console.log("Hello, polyglot!");`,
  };

  onMount(async () => {
    try {
      userProjects = await projects.list(1);
      if (userProjects.length > 0) projectId = userProjects[0].id;
    } catch {}
  });

  function handleLangChange() {
    sourceCode = starters[language] ?? '';
  }

  async function handleCompile() {
    if (!projectId) return;
    compiling = true;
    errorMsg = '';
    compileJob = null;
    executeResult = null;

    try {
      const submitted = await compile.submit({
        project_id: projectId,
        language,
        source_code: sourceCode,
        compiler_flags: compilerFlags || undefined,
      });

      let attempts = 0;
      while (attempts < 60) {
        await new Promise(r => setTimeout(r, 1000));
        const status = await compile.status(submitted.job_id);
        compileJob = status;
        if (status.status === 'completed' || status.status === 'failed') break;
        attempts++;
      }
    } catch (e: any) {
      errorMsg = e.message;
    } finally {
      compiling = false;
    }
  }

  async function handleExecute() {
    if (!compileJob || compileJob.status !== 'completed') return;
    executing = true;
    errorMsg = '';
    try {
      executeResult = await compile.execute(compileJob.id, stdinData || undefined);
    } catch (e: any) {
      errorMsg = e.message;
    } finally {
      executing = false;
    }
  }
</script>

<svelte:head><title>Compile — Polyglot</title></svelte:head>

<div class="mb-6">
  <h1 class="text-2xl font-bold text-white">Compile & Execute</h1>
  <p class="font-mono text-xs text-gray-500 mt-1">Rust Core Service · Sandboxed execution</p>
</div>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
  <div class="flex flex-col gap-4">
    <div class="card">
      <div class="flex gap-3 mb-4">
        <div class="flex-1">
          <label class="block font-mono text-xs text-gray-400 mb-1">Project</label>
          <select bind:value={projectId} class="input">
            {#each userProjects as p}
              <option value={p.id}>{p.name}</option>
            {/each}
          </select>
        </div>
        <div>
          <label class="block font-mono text-xs text-gray-400 mb-1">Language</label>
          <select bind:value={language} on:change={handleLangChange} class="input">
            {#each languages as lang}
              <option value={lang}>{lang}</option>
            {/each}
          </select>
        </div>
      </div>

      <div class="mb-3">
        <label class="block font-mono text-xs text-gray-400 mb-1">Source Code</label>
        <textarea
          bind:value={sourceCode}
          class="input font-mono text-xs h-64 resize-none"
          spellcheck="false"
        ></textarea>
      </div>

      <div class="mb-4">
        <label class="block font-mono text-xs text-gray-400 mb-1">Compiler Flags (optional)</label>
        <input type="text" bind:value={compilerFlags} class="input font-mono text-xs" placeholder="-O2 -Wall" />
      </div>

      <button on:click={handleCompile} class="btn-primary w-full" disabled={compiling || !projectId}>
        {compiling ? 'Compiling...' : '⚡ Compile'}
      </button>
    </div>

    <div class="card">
      <label class="block font-mono text-xs text-gray-400 mb-1">Stdin (optional)</label>
      <textarea bind:value={stdinData} class="input font-mono text-xs h-20 resize-none mb-3" placeholder="Input data..."></textarea>
      <button on:click={handleExecute} class="btn-secondary w-full"
        disabled={executing || !compileJob || compileJob.status !== 'completed'}>
        {executing ? 'Running...' : '▶ Execute'}
      </button>
    </div>
  </div>

  <div class="flex flex-col gap-4">
    {#if errorMsg}
      <div class="card border-danger/40 font-mono text-xs text-danger">{errorMsg}</div>
    {/if}

    {#if compileJob}
      <div class="card">
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-semibold text-sm text-white">Compile Result</h3>
          <span class="badge {compileJob.status === 'completed' ? 'badge-success' : compileJob.status === 'failed' ? 'badge-danger' : 'badge-warning'}">
            {compileJob.status}
          </span>
        </div>
        {#if compileJob.duration_ms}
          <p class="font-mono text-xs text-gray-500 mb-2">{compileJob.duration_ms}ms · exit {compileJob.exit_code}</p>
        {/if}
        {#if compileJob.error_output}
          <pre class="bg-bg rounded p-3 font-mono text-xs text-danger overflow-auto max-h-48 whitespace-pre-wrap">{compileJob.error_output}</pre>
        {/if}
      </div>
    {/if}

    {#if executeResult}
      <div class="card">
        <div class="flex items-center justify-between mb-3">
          <h3 class="font-semibold text-sm text-white">Execution Output</h3>
          <span class="badge {executeResult.exit_code === 0 ? 'badge-success' : 'badge-danger'}">
            exit {executeResult.exit_code}
          </span>
        </div>
        <p class="font-mono text-xs text-gray-500 mb-2">{executeResult.duration_ms}ms</p>
        {#if executeResult.stdout}
          <pre class="bg-bg rounded p-3 font-mono text-xs text-success overflow-auto max-h-64 whitespace-pre-wrap">{executeResult.stdout}</pre>
        {/if}
        {#if executeResult.stderr}
          <pre class="bg-bg rounded p-3 font-mono text-xs text-warning overflow-auto max-h-32 whitespace-pre-wrap mt-2">{executeResult.stderr}</pre>
        {/if}
      </div>
    {/if}
  </div>
</div>
