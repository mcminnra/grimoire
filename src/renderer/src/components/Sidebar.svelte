<script lang="ts">
  import { Power } from '@lucide/svelte'
  import { NAV_ITEMS, type View } from '@lib/nav'

  let { activeView, onNavigate }: { activeView: View; onNavigate: (v: View) => void } = $props()
</script>

<aside class="sidebar">
  <nav>
    {#each NAV_ITEMS as item (item.id)}
      {@const Icon = item.icon}
      <button
        class="nav-link"
        class:active={item.id === activeView}
        aria-current={item.id === activeView ? 'page' : undefined}
        onclick={() => onNavigate(item.id)}
      >
        <Icon size={18} />
        {item.label}
      </button>
    {/each}
  </nav>

  <div class="bottom-bar">
    <div class="brand-group">
      <button class="brand" aria-label="Home" onclick={() => onNavigate('home')}>Grimoire</button>
      <span class="version">v{__APP_VERSION__}</span>
    </div>
    <button
      class="nav-link quit"
      aria-label="Quit"
      title="Quit"
      onclick={() => window.api.app.quit()}
    >
      <Power size={18} />
    </button>
  </div>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    width: 14rem;
    height: 100vh;
    flex-shrink: 0;
    padding: var(--space-4);
    background: var(--surface-2);
    border-right: 1px solid var(--border);
    user-select: none;
  }

  .bottom-bar {
    margin-top: auto;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
    padding-top: var(--space-2);
    border-top: 1px solid var(--border);
  }

  .brand-group {
    display: flex;
    align-items: baseline;
    gap: var(--space-2);
  }

  .version {
    color: var(--text-muted);
    font-size: 0.7rem;
    opacity: 0.7;
  }

  .brand {
    padding: var(--space-2);
    border: none;
    background: none;
    color: var(--text-muted);
    font-family: inherit;
    font-size: 0.8rem;
    letter-spacing: 0.02em;
    cursor: pointer;
  }

  nav {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    text-align: left;
    padding: var(--space-2) var(--space-4);
    border: none;
    border-radius: 0.5rem;
    background: transparent;
    color: var(--text-muted);
    font: inherit;
    cursor: pointer;
  }

  .nav-link:hover {
    background: var(--border);
    color: var(--text);
  }

  .nav-link.active {
    background: var(--accent);
    color: #fff;
  }

  /* Compact icon button, tint hover toward danger */
  .quit {
    width: auto;
    padding: var(--space-2);
  }

  .quit:hover {
    background: rgba(220, 90, 90, 0.14);
    color: #e59a9a;
  }
</style>
