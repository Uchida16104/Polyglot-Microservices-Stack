<script lang="ts">
  import { onMount } from 'svelte';
  import { projects as projectsApi, type Project } from '$lib/api';

  let projectList: Project[] = [];
  let loading = true;
  let creating = false;
  let showForm = false;
  let newName = '';
  let newDesc = '';
  let newVisibility = 'private';
  let errorMsg = '';

  onMount(async () => {
    await loadProjects();
  });

  async function loadProjects() {
    loading = true;
    try {
      projectList = await projectsApi.list(1);
    } catch (e: any) {
      errorMsg = e.message;
    } finally {
      loading = false;
    }
  }

  async function handleCreate() {
    if (!newName.trim()) return;
    creating = true;
    try {
      const p = await projectsApi.create({ name: newName, description: newDesc, visibility: newVisibility });
      projectList = [p, ...projectList];
      showForm = false;
      newName = '';
      newDesc = '';
    } catch (e: any) {
      errorMsg = e.message;
    } finally {
      creating = false;
    }
  }

  async function handleDelete(id: string) {
    if (!confirm('Delete this project?')) return;
    try {
      await projectsApi.delete(id);
      projectList = projectList.filter(p => p.id !== id);
    } catch (e: any) {
      errorMsg = e.message;
    }
  }

  const statusBadge: Record<string, string> = {
    active: 'badge-success', archived: 'badge-default', deleted: 'badge-danger',
  };
</script>

<svelte:head><title>Projects — Polyglot</title></svelte:head>

<div class="flex items-center justify-between mb-6">
  <div>
    <h1 class="text-2xl font-bold text-white">Projects</h1>
    <p class="font-mono text-xs text-gray-500 mt-1">C# Business Logic Service</p>
  </div>
  <button on:click={() => showForm = !showForm} class="btn-primary">+ New Project</button>
</div>

{#if errorMsg}
  <div class="mb-4 card border-danger/40 font-mono text-xs text-danger">{errorMsg}</div>
{/if}

{#if showForm}
  <div class="card mb-6">
    <h2 class="font-semibold text-sm text-white mb-4">Create Project</h2>
    <form on:submit|preventDefault={handleCreate} class="flex flex-col gap-3">
      <div>
        <label class="block font-mono text-xs text-gray-400 mb-1">Name *</label>
        <input type="text" bind:value={newName} class="input" required />
      </div>
      <div>
        <label class="block font-mono text-xs text-gray-400 mb-1">Description</label>
        <textarea bind:value={newDesc} class="input resize-none h-20" ></textarea>
      </div>
      <div>
        <label class="block font-mono text-xs text-gray-400 mb-1">Visibility</label>
        <select bind:value={newVisibility} class="input">
          <option value="private">Private</option>
          <option value="team">Team</option>
          <option value="public">Public</option>
        </select>
      </div>
      <div class="flex gap-3 mt-2">
        <button type="submit" class="btn-primary" disabled={creating}>
          {creating ? 'Creating...' : 'Create'}
        </button>
        <button type="button" on:click={() => showForm = false} class="btn-secondary">Cancel</button>
      </div>
    </form>
  </div>
{/if}

{#if loading}
  <p class="font-mono text-xs text-gray-500 animate-pulse">Loading projects...</p>
{:else if projectList.length === 0}
  <div class="card text-center py-12">
    <p class="text-gray-500 font-mono text-sm">No projects yet.</p>
    <button on:click={() => showForm = true} class="btn-primary mt-4">Create your first project</button>
  </div>
{:else}
  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
    {#each projectList as project}
      <div class="card hover:border-accent/40 transition-colors">
        <div class="flex items-start justify-between mb-2">
          <a href="/projects/{project.id}" class="font-semibold text-white hover:text-accent transition-colors">
            {project.name}
          </a>
          <span class="badge {statusBadge[project.status] ?? 'badge-default'}">{project.status}</span>
        </div>
        {#if project.description}
          <p class="font-mono text-xs text-gray-500 mb-3 line-clamp-2">{project.description}</p>
        {/if}
        <div class="flex items-center justify-between mt-auto">
          <span class="font-mono text-xs text-gray-600">{project.visibility}</span>
          <button
            on:click={() => handleDelete(project.id)}
            class="font-mono text-xs text-gray-600 hover:text-danger transition-colors"
          >
            Delete
          </button>
        </div>
      </div>
    {/each}
  </div>
{/if}
