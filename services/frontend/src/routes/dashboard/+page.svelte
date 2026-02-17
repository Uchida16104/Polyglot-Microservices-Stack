<script lang="ts">
  import { onMount } from 'svelte';
  import { auth, projects, ml, type Project, type MlJob } from '$lib/api';
  import { goto } from '$app/navigation';

  let user: { name: string; email: string; role: string } | null = null;
  let recentProjects: Project[] = [];
  let recentJobs: MlJob[] = [];
  let loading = true;

  onMount(async () => {
    try {
      const [meData, projData, jobData] = await Promise.all([
        auth.me(),
        projects.list(1),
        ml.listJobs(),
      ]);
      user = meData.user;
      recentProjects = projData.slice(0, 4);
      recentJobs = jobData.slice(0, 5);
    } catch {
      goto('/login');
    } finally {
      loading = false;
    }
  });

  const statusClass: Record<string, string> = {
    active: 'badge-success', archived: 'badge-default',
    completed: 'badge-success', failed: 'badge-danger',
    queued: 'badge-info', running: 'badge-warning',
  };
</script>

<svelte:head><title>Dashboard — Polyglot</title></svelte:head>

{#if loading}
  <div class="flex items-center justify-center h-64">
    <p class="font-mono text-xs text-gray-500 animate-pulse">Loading workspace...</p>
  </div>
{:else}
  <div class="mb-8">
    <h1 class="text-2xl font-bold text-white">
      Welcome back, <span class="text-accent">{user?.name}</span>
    </h1>
    <p class="font-mono text-xs text-gray-500 mt-1">{user?.email} · {user?.role}</p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h2 class="font-semibold text-sm text-white">Recent Projects</h2>
        <a href="/projects" class="font-mono text-xs text-accent hover:underline">View all →</a>
      </div>
      {#if recentProjects.length === 0}
        <p class="font-mono text-xs text-gray-500">No projects yet. <a href="/projects" class="text-accent hover:underline">Create one</a>.</p>
      {:else}
        <ul class="flex flex-col gap-2">
          {#each recentProjects as project}
            <li>
              <a href="/projects/{project.id}" class="flex items-center justify-between p-3 rounded-lg border border-border hover:border-accent/50 transition-colors group">
                <span class="font-medium text-sm text-gray-200 group-hover:text-white">{project.name}</span>
                <span class="badge {statusClass[project.status] ?? 'badge-default'}">{project.status}</span>
              </a>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h2 class="font-semibold text-sm text-white">ML Jobs</h2>
        <a href="/ml" class="font-mono text-xs text-accent hover:underline">View all →</a>
      </div>
      {#if recentJobs.length === 0}
        <p class="font-mono text-xs text-gray-500">No ML jobs yet. <a href="/ml" class="text-accent hover:underline">Start training</a>.</p>
      {:else}
        <ul class="flex flex-col gap-2">
          {#each recentJobs as job}
            <li class="flex items-center justify-between p-3 rounded-lg border border-border">
              <div>
                <span class="font-mono text-xs text-gray-300">{job.job_type}</span>
                <span class="font-mono text-xs text-gray-600 ml-2">{job.id.slice(0, 8)}...</span>
              </div>
              <span class="badge {statusClass[job.status] ?? 'badge-default'}">{job.status}</span>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="card md:col-span-2">
      <h2 class="font-semibold text-sm text-white mb-4">Quick Actions</h2>
      <div class="flex flex-wrap gap-3">
        <a href="/projects" class="btn-primary">+ New Project</a>
        <a href="/compile" class="btn-secondary">⚡ Compile Code</a>
        <a href="/ml" class="btn-secondary">🧠 ML Studio</a>
      </div>
    </div>
  </div>
{/if}
