<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';

  const publicRoutes = ['/login', '/register'];
  $: isPublic = publicRoutes.includes($page.url.pathname);
</script>

<svelte:head>
  <link rel="preconnect" href="https://fonts.googleapis.com" />
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
  <link href="https://fonts.googleapis.com/css2?family=DM+Mono:wght@300;400;500&family=Syne:wght@400;600;700;800&display=swap" rel="stylesheet" />
</svelte:head>

<div class="min-h-screen bg-bg text-white font-sans">
  {#if !isPublic}
    <nav class="border-b border-border px-6 py-3 flex items-center justify-between">
      <a href="/dashboard" class="font-mono text-sm text-accent font-medium tracking-widest uppercase">
        Polyglot
      </a>
      <div class="flex items-center gap-6 font-mono text-xs text-gray-400">
        <a href="/dashboard" class="hover:text-white transition-colors">Dashboard</a>
        <a href="/projects" class="hover:text-white transition-colors">Projects</a>
        <a href="/compile" class="hover:text-white transition-colors">Compile</a>
        <a href="/research" class="hover:text-orange-400 transition-colors">Research</a>
        <a href="/ml" class="hover:text-white transition-colors">ML Studio</a>
        <a href="/logout" class="text-danger hover:text-red-400 transition-colors">Logout</a>
      </div>
    </nav>
  {/if}

  <main class="max-w-7xl mx-auto px-6 py-8">
    <slot />
  </main>
</div>
