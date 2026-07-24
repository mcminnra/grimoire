<script lang="ts">
  import Sidebar from '@components/Sidebar.svelte'
  import Home from '@views/Home.svelte'
  import Stats from '@views/Stats.svelte'
  import type { View } from '@lib/nav'

  let currentView = $state<View>('home')
</script>

<div class="app">
  <Sidebar activeView={currentView} onNavigate={(v) => (currentView = v)} />
  <main>
    {#if currentView === 'home'}
      <Home />
    {:else}
      <Stats />
    {/if}
  </main>
</div>

<style>
  .app {
    display: flex;
    height: 100vh;
  }

  main {
    flex: 1;
    overflow: auto;
    padding: var(--space-4);
    background: var(--surface);
  }
</style>
