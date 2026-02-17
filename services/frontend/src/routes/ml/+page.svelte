<script lang="ts">
  import { onMount } from 'svelte';
  import { ml, type MlModel, type MlJob } from '$lib/api';

  let models: MlModel[] = [];
  let jobs: MlJob[] = [];
  let loading = true;
  let errorMsg = '';
  let showModelForm = false;
  let showJobForm = false;

  let newModelName = '';
  let newModelFramework = 'sklearn';
  let newModelVersion = '1.0.0';

  let selectedModelId = '';
  let jobType = 'train';
  let jobConfigRaw = '{}';

  const frameworks = ['pytorch', 'tensorflow', 'sklearn', 'xgboost', 'custom'];
  const jobTypes = ['train', 'infer', 'eval', 'finetune'];

  const statusBadge: Record<string, string> = {
    untrained: 'badge-default', training: 'badge-warning', trained: 'badge-success',
    failed: 'badge-danger', deprecated: 'badge-default',
    queued: 'badge-info', running: 'badge-warning', completed: 'badge-success',
  };

  onMount(async () => {
    try {
      [models, jobs] = await Promise.all([ml.listModels(), ml.listJobs()]);
    } catch (e: any) {
      errorMsg = e.message;
    } finally {
      loading = false;
    }
  });

  async function createModel() {
    try {
      const m = await ml.createModel({ name: newModelName, framework: newModelFramework, version: newModelVersion });
      models = [m, ...models];
      showModelForm = false;
      newModelName = '';
    } catch (e: any) {
      errorMsg = e.message;
    }
  }

  async function createJob() {
    if (!selectedModelId) return;
    try {
      let config = {};
      try { config = JSON.parse(jobConfigRaw); } catch {}
      const j = await ml.createJob({ model_id: selectedModelId, job_type: jobType, config });
      jobs = [j, ...jobs];
      showJobForm = false;
    } catch (e: any) {
      errorMsg = e.message;
    }
  }
</script>

<svelte:head><title>ML Studio — Polyglot</title></svelte:head>

<div class="flex items-center justify-between mb-6">
  <div>
    <h1 class="text-2xl font-bold text-white">ML Studio</h1>
    <p class="font-mono text-xs text-gray-500 mt-1">Python FastAPI · Model registry &amp; job management</p>
  </div>
  <div class="flex gap-3">
    <button on:click={() => showModelForm = !showModelForm} class="btn-secondary">+ Model</button>
    <button on:click={() => showJobForm = !showJobForm} class="btn-primary">+ Job</button>
  </div>
</div>

{#if errorMsg}
  <div class="mb-4 card border-danger/40 font-mono text-xs text-danger">{errorMsg}</div>
{/if}

{#if showModelForm}
  <div class="card mb-4">
    <h3 class="font-semibold text-sm text-white mb-3">Register Model</h3>
    <form on:submit|preventDefault={createModel} class="flex flex-col gap-3">
      <input type="text" bind:value={newModelName} class="input" placeholder="Model name *" required />
      <div class="flex gap-3">
        <select bind:value={newModelFramework} class="input flex-1">
          {#each frameworks as fw}<option value={fw}>{fw}</option>{/each}
        </select>
        <input type="text" bind:value={newModelVersion} class="input w-32" placeholder="1.0.0" />
      </div>
      <div class="flex gap-3">
        <button type="submit" class="btn-primary">Register</button>
        <button type="button" on:click={() => showModelForm = false} class="btn-secondary">Cancel</button>
      </div>
    </form>
  </div>
{/if}

{#if showJobForm}
  <div class="card mb-4">
    <h3 class="font-semibold text-sm text-white mb-3">Submit Job</h3>
    <form on:submit|preventDefault={createJob} class="flex flex-col gap-3">
      <select bind:value={selectedModelId} class="input" required>
        <option value="">Select model *</option>
        {#each models as m}<option value={m.id}>{m.name} ({m.framework} · {m.status})</option>{/each}
      </select>
      <select bind:value={jobType} class="input">
        {#each jobTypes as jt}<option value={jt}>{jt}</option>{/each}
      </select>
      <div>
        <label class="block font-mono text-xs text-gray-400 mb-1">Config JSON</label>
        <textarea bind:value={jobConfigRaw} class="input font-mono text-xs h-20 resize-none" placeholder='{"epochs": 10}'></textarea>
      </div>
      <div class="flex gap-3">
        <button type="submit" class="btn-primary">Submit</button>
        <button type="button" on:click={() => showJobForm = false} class="btn-secondary">Cancel</button>
      </div>
    </form>
  </div>
{/if}

{#if loading}
  <p class="font-mono text-xs text-gray-500 animate-pulse">Loading ML studio...</p>
{:else}
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <div>
      <h2 class="font-semibold text-sm text-white mb-3">Models ({models.length})</h2>
      {#if models.length === 0}
        <div class="card text-center py-8">
          <p class="font-mono text-xs text-gray-500">No models registered yet.</p>
        </div>
      {:else}
        <div class="flex flex-col gap-2">
          {#each models as model}
            <div class="card">
              <div class="flex items-center justify-between">
                <div>
                  <p class="font-semibold text-sm text-white">{model.name}</p>
                  <p class="font-mono text-xs text-gray-500">{model.framework} · v{model.version}</p>
                </div>
                <span class="badge {statusBadge[model.status] ?? 'badge-default'}">{model.status}</span>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <div>
      <h2 class="font-semibold text-sm text-white mb-3">Jobs ({jobs.length})</h2>
      {#if jobs.length === 0}
        <div class="card text-center py-8">
          <p class="font-mono text-xs text-gray-500">No jobs submitted yet.</p>
        </div>
      {:else}
        <div class="flex flex-col gap-2">
          {#each jobs as job}
            <div class="card">
              <div class="flex items-center justify-between mb-1">
                <span class="font-mono text-xs text-accent uppercase">{job.job_type}</span>
                <span class="badge {statusBadge[job.status] ?? 'badge-default'}">{job.status}</span>
              </div>
              <p class="font-mono text-xs text-gray-600">{job.id.slice(0, 16)}...</p>
              {#if job.duration_ms}
                <p class="font-mono text-xs text-gray-500 mt-1">{job.duration_ms}ms</p>
              {/if}
              {#if job.result_metrics}
                <pre class="font-mono text-xs text-success mt-2 bg-bg rounded p-2 overflow-auto">{JSON.stringify(job.result_metrics, null, 2)}</pre>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}
