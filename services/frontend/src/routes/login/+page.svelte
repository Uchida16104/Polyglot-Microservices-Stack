<script lang="ts">
  import { auth, type ApiError } from '$lib/api';
  import { goto } from '$app/navigation';

  let email = '';
  let password = '';
  let loading = false;
  let error = '';

  async function handleLogin() {
    loading = true;
    error = '';
    try {
      await auth.login(email, password);
      goto('/dashboard');
    } catch (e) {
      error = (e as ApiError).message ?? 'Login failed';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head><title>Login — Polyglot</title></svelte:head>

<div class="min-h-screen bg-bg flex items-center justify-center px-4">
  <div class="w-full max-w-sm">
    <div class="mb-8 text-center">
      <p class="font-mono text-accent text-xs tracking-widest uppercase mb-2">Polyglot ◆</p>
      <h1 class="text-2xl font-bold text-white">Sign in</h1>
      <p class="text-gray-500 font-mono text-xs mt-1">access your workspace</p>
    </div>

    <div class="card">
      {#if error}
        <div class="mb-4 p-3 rounded-lg border border-danger/40 bg-red-950/20 font-mono text-xs text-danger">
          {error}
        </div>
      {/if}

      <form on:submit|preventDefault={handleLogin} class="flex flex-col gap-4">
        <div>
          <label for="email" class="block font-mono text-xs text-gray-400 mb-1">Email</label>
          <input id="email" type="email" bind:value={email} class="input" required autocomplete="email" />
        </div>
        <div>
          <label for="password" class="block font-mono text-xs text-gray-400 mb-1">Password</label>
          <input id="password" type="password" bind:value={password} class="input" required autocomplete="current-password" />
        </div>
        <button type="submit" class="btn-primary w-full mt-2" disabled={loading}>
          {loading ? 'Signing in...' : 'Sign in'}
        </button>
      </form>
    </div>

    <p class="mt-4 text-center font-mono text-xs text-gray-500">
      No account? <a href="/register" class="text-accent hover:underline">Register</a>
    </p>
  </div>
</div>
