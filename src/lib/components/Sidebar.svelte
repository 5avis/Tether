<script lang="ts">
  import { type NavItem, type Workspace, type UserProfile, isAdminUser } from '$lib/types';
  import Icons from './Icons.svelte';

  let {
    activeNav = 'overview',
    activeWorkspaceId = 'ws-default',
    workspaces = [],
    profile = null,
    counts = {
      overview: 0,
      tunnels: 0,
      deployments: 0,
      watchdog: 0,
      logs: 0,
    },
    isCollapsed = false,
    onSelectNav = (nav: NavItem) => {},
    onSelectWorkspace = (id: string) => {},
    onAddService = () => {},
    onOpenProfile = () => {},
    onSignOut = () => {},
  }: {
    activeNav: NavItem;
    activeWorkspaceId: string;
    workspaces: Workspace[];
    profile?: UserProfile | null;
    counts?: Partial<Record<NavItem, number>>;
    isCollapsed?: boolean;
    onSelectNav: (nav: NavItem) => void;
    onSelectWorkspace: (id: string) => void;
    onAddService: () => void;
    onOpenProfile?: () => void;
    onSignOut?: () => void;
  } = $props();

  let isProjectsCollapsed = $state(false);
  let showWorkspaceMenu = $state(false);

  const isAdmin = $derived(isAdminUser(profile));

  const navItems = $derived<{ id: NavItem; label: string; icon: any; count?: number }[]>([
    { id: 'overview', label: 'Overview', icon: 'layers', count: counts.overview },
    { id: 'tunnels', label: 'Tunnels', icon: 'radio', count: counts.tunnels },
    { id: 'deployments', label: 'Deployments', icon: 'rocket', count: counts.deployments },
    { id: 'watchdog', label: 'Watchdog', icon: 'shield', count: counts.watchdog },
    { id: 'logs', label: 'Logs', icon: 'terminal' },
  ]);

  const currentWorkspace = $derived(
    workspaces.find((w) => w.id === activeWorkspaceId) || workspaces[0]
  );
</script>

<aside
  class="{isCollapsed ? 'w-14' : 'w-[240px]'} shrink-0 h-full bg-surface border-r border-border-dim flex flex-col justify-between select-none font-sans z-40 text-xs transition-all duration-200"
>
  <!-- Top Section: Standard Profile Header & Primary Action Button -->
  <div class="p-2.5 pb-2 flex flex-col gap-2.5">
    {#if !isCollapsed}
      <!-- Workspace Selector Header (Clean, dedicated workspace switch without avatar or signin/logout buttons) -->
      <div class="relative">
        <button
          onclick={() => (showWorkspaceMenu = !showWorkspaceMenu)}
          class="w-full flex items-center justify-between p-2 px-3 rounded-xl bg-surface-elevated/70 hover:bg-surface-elevated border border-border-dim/80 hover:border-border-subtle transition-all text-left shadow-subtle group"
          title="Switch Workspace"
        >
          <div class="flex items-center gap-2 min-w-0">
            <span class="w-2 h-2 rounded-full bg-coral shrink-0"></span>
            <div class="min-w-0 flex flex-col">
              <span class="text-xs font-semibold text-tx-header truncate">
                {currentWorkspace?.name ?? 'Main Workspace'}
              </span>
              <span class="text-[10px] text-tx-meta font-mono truncate">
                {currentWorkspace?.servicesCount ?? 0} active targets
              </span>
            </div>
          </div>
          <Icons name="chevron-down" size={11} class="text-tx-meta group-hover:text-tx-body transition-transform duration-150 {showWorkspaceMenu ? 'rotate-180' : ''}" />
        </button>

        <!-- Dropdown for Switching Workspaces with Backdrop -->
        {#if showWorkspaceMenu}
          <!-- Backdrop to close dropdown on click outside -->
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (showWorkspaceMenu = false)}
            aria-label="Close menu"
          ></button>

          <div
            class="absolute top-12 left-0 right-0 bg-[#222222] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100"
          >
            <div class="px-2 py-1 text-[10px] uppercase font-mono text-tx-meta tracking-wider">Select Workspace</div>
            {#each workspaces as ws}
              <button
                onclick={() => {
                  onSelectWorkspace(ws.id);
                  showWorkspaceMenu = false;
                }}
                class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg hover:bg-surface-hover text-left transition-colors {activeWorkspaceId === ws.id ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body'}"
              >
                <div class="flex items-center gap-2">
                  <span class="w-1.5 h-1.5 rounded-full {activeWorkspaceId === ws.id ? 'bg-coral' : 'bg-border-subtle'}"></span>
                  <span class="truncate">{ws.name}</span>
                </div>
                <span class="text-[10px] text-tx-meta font-mono">{ws.servicesCount}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>

      {#if isAdmin}
        <!-- Primary Action: Pill-shaped coral button (Admin Only) -->
        <button
          onclick={onAddService}
          class="w-full py-1.5 px-3 rounded-xl bg-coral hover:bg-coral-hover text-white font-medium text-xs flex items-center justify-center gap-2 shadow-sm transition-all active:scale-[0.98] border border-coral/60 hover:shadow-[0_0_12px_rgba(226,92,72,0.3)]"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>Add Target</span>
        </button>
      {/if}
    {:else}
      {#if isAdmin}
        <!-- Collapsed Icon Rail Header (Add Target only) -->
        <div class="flex flex-col items-center gap-2">
          <button
            onclick={onAddService}
            class="w-8 h-8 rounded-lg bg-coral hover:bg-coral-hover text-white flex items-center justify-center shadow-sm transition-all border border-coral/60 active:scale-[0.95]"
            title="Add Target"
          >
            <Icons name="plus" size={14} strokeWidth={2.5} />
          </button>
        </div>
      {/if}
    {/if}
  </div>

  <!-- Middle Section: Navigation Links & Standard Workspaces List -->
  <div class="flex-1 overflow-y-auto px-2 py-1 space-y-4">
    <!-- Main Navigation List -->
    <nav class="space-y-1">
      {#each navItems as item}
        {@const isActive = activeNav === item.id}
        <button
          onclick={() => onSelectNav(item.id)}
          class="w-full flex items-center {isCollapsed ? 'justify-center p-2' : 'justify-between px-2.5 py-1.5'} rounded-xl text-xs transition-all {isActive
            ? 'bg-coral-light text-coral font-medium border border-coral/30 shadow-subtle'
            : 'text-tx-body hover:text-tx-header hover:bg-surface-hover border border-transparent'}"
          title={isCollapsed ? item.label : undefined}
        >
          <div class="flex items-center gap-2.5">
            <Icons
              name={item.icon}
              size={15}
              class={isActive ? 'text-coral' : 'text-tx-meta group-hover:text-tx-body'}
            />
            {#if !isCollapsed}
              <span>{item.label}</span>
            {/if}
          </div>

          {#if !isCollapsed && item.count !== undefined}
            <span
              class="px-1.5 py-0.2 rounded-md text-[10px] font-mono {isActive
                ? 'bg-coral/20 text-coral font-semibold'
                : 'bg-surface-elevated text-tx-meta border border-border-dim'}"
            >
              {item.count}
            </span>
          {/if}
        </button>
      {/each}
    </nav>

    <!-- Section: Workspaces (Hidden when collapsed) -->
    {#if !isCollapsed}
      <div class="pt-2 border-t border-border-dim/60">
        <div class="flex items-center justify-between px-2.5 py-1 text-tx-meta">
          <button
            onclick={() => (isProjectsCollapsed = !isProjectsCollapsed)}
            class="flex items-center gap-1.5 text-[10px] font-medium tracking-wider uppercase font-mono hover:text-tx-body transition-colors"
          >
            <Icons
              name="chevron-down"
              size={11}
              class="transition-transform duration-200 {isProjectsCollapsed ? '-rotate-90' : ''}"
            />
            <span>Workspaces</span>
          </button>
        </div>

        {#if !isProjectsCollapsed}
          <div class="mt-1 space-y-0.5">
            {#each workspaces as ws}
              {@const isWsActive = activeWorkspaceId === ws.id}
              <button
                onclick={() => onSelectWorkspace(ws.id)}
                class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-xs transition-colors {isWsActive
                  ? 'bg-surface-elevated text-tx-header font-medium border border-border-dim/70'
                  : 'text-tx-body hover:text-tx-header hover:bg-surface-hover border border-transparent'}"
              >
                <div class="flex items-center gap-2 min-w-0">
                  <span class="w-1.5 h-1.5 rounded-full shrink-0 {isWsActive ? 'bg-coral' : 'bg-border-subtle'}"></span>
                  <span class="truncate">{ws.name}</span>
                </div>
                <span class="text-[10px] text-tx-meta font-mono">{ws.servicesCount}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Bottom Footer -->
  <div class="border-t border-border-dim">
    <div class="p-2.5 flex items-center {isCollapsed ? 'justify-center' : 'justify-between'} text-[11px] text-tx-meta">
      {#if !isCollapsed}
        <span class="font-mono text-[10px]">Tether v0.1</span>
        <span class="text-coral font-mono text-[10px]">Local-First</span>
      {:else}
        <span class="w-2 h-2 rounded-full bg-coral" title="Tether Local-First"></span>
      {/if}
    </div>
  </div>
</aside>
