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
    onNewProject = () => {},
    onEditProject = (ws: Workspace) => {},
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
    onNewProject?: () => void;
    onEditProject?: (ws: Workspace) => void;
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
  class="{isCollapsed ? 'w-14' : 'w-[245px]'} shrink-0 h-full bg-surface border-r border-border-dim flex flex-col justify-between select-none font-sans z-40 text-xs transition-all duration-200"
>
  <!-- Top Section: Standard Profile Header & Primary Action Button -->
  <div class="p-2.5 pb-2 flex flex-col gap-2.5">
    {#if !isCollapsed}
      <!-- Workspace / Project Selector Header -->
      <div class="relative">
        <div class="flex items-center gap-1.5 w-full">
          <button
            onclick={() => (showWorkspaceMenu = !showWorkspaceMenu)}
            class="flex-1 flex items-center justify-between p-2 px-3 rounded-xl bg-surface-elevated/70 hover:bg-surface-elevated border border-border-dim/80 hover:border-border-subtle transition-all text-left shadow-subtle group"
            title="Switch Project"
          >
            <div class="flex items-center gap-2 min-w-0">
              <span
                class="w-2.5 h-2.5 rounded-full shrink-0 ring-1 ring-white/10"
                style="background-color: {currentWorkspace?.color || '#8b5cf6'};"
              ></span>
              <div class="min-w-0 flex flex-col">
                <span class="text-xs font-semibold text-tx-header truncate">
                  {currentWorkspace?.name ?? 'Main Project'}
                </span>
                <span class="text-[10px] text-tx-meta font-mono truncate">
                  {currentWorkspace?.servicesCount ?? 0} active targets
                </span>
              </div>
            </div>
            <Icons
              name="chevron-down"
              size={11}
              class="text-tx-meta group-hover:text-tx-body transition-transform duration-150 {showWorkspaceMenu ? 'rotate-180' : ''}"
            />
          </button>

          {#if currentWorkspace}
            <button
              onclick={() => onEditProject(currentWorkspace)}
              class="p-2 rounded-xl bg-surface-elevated/70 hover:bg-surface-elevated border border-border-dim/80 hover:border-border-subtle text-tx-meta hover:text-tx-header transition-all shadow-subtle shrink-0"
              title="Edit project name"
              aria-label="Edit Project"
            >
              <Icons name="settings" size={13} />
            </button>
          {/if}
        </div>

        <!-- Dropdown for Switching Workspaces with Backdrop -->
        {#if showWorkspaceMenu}
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (showWorkspaceMenu = false)}
            aria-label="Close menu"
          ></button>

          <div
            class="absolute top-12 left-0 right-0 bg-[#161326] border border-border-dim rounded-2xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100"
          >
            <div class="px-2.5 py-1.5 text-[10px] uppercase font-mono text-tx-meta tracking-wider flex items-center justify-between">
              <span>All Projects</span>
              <span class="text-[9px] text-coral font-mono">{workspaces.length} total</span>
            </div>

            <div class="space-y-0.5 max-h-48 overflow-y-auto">
              {#each workspaces as ws}
                {@const isWsActive = activeWorkspaceId === ws.id}
                <button
                  onclick={() => {
                    onSelectWorkspace(ws.id);
                    showWorkspaceMenu = false;
                  }}
                  class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-xl hover:bg-surface-hover text-left transition-colors {isWsActive
                    ? 'bg-surface-elevated text-tx-header font-medium border border-border-dim'
                    : 'text-tx-body hover:text-tx-header'}"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="w-2 h-2 rounded-full shrink-0 ring-1 ring-white/10"
                      style="background-color: {ws.color || '#8b5cf6'};"
                    ></span>
                    <span class="truncate">{ws.name}</span>
                  </div>
                  <span class="text-[10px] text-tx-meta font-mono px-1.5 py-0.2 rounded bg-surface border border-border-dim shrink-0">
                    {ws.servicesCount}
                  </span>
                </button>
              {/each}
            </div>

            <div class="pt-1.5 mt-1 border-t border-border-dim/80">
              <button
                type="button"
                onclick={() => {
                  showWorkspaceMenu = false;
                  onNewProject();
                }}
                class="w-full flex items-center justify-center gap-1.5 px-2.5 py-1.5 rounded-xl bg-coral/15 hover:bg-coral/25 text-coral font-medium text-xs border border-coral/30 transition-all"
              >
                <Icons name="plus" size={12} strokeWidth={2.5} />
                <span>Create New Project</span>
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Primary Action: Pill-shaped purple button -->
      <button
        onclick={onAddService}
        class="w-full py-2 px-3 rounded-xl bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white font-medium text-xs flex items-center justify-center gap-2 shadow-sm transition-all active:scale-[0.98] border border-[#c084fc]/30 shadow-[0_0_14px_rgba(168,85,247,0.25)]"
      >
        <Icons name="plus" size={13} strokeWidth={2.5} />
        <span>Add Target</span>
      </button>
    {:else}
      <!-- Collapsed Icon Rail Header (Add Target) -->
      <div class="flex flex-col items-center gap-2">
        <button
          onclick={onAddService}
          class="w-8 h-8 rounded-lg bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white flex items-center justify-center shadow-sm transition-all border border-[#c084fc]/30 active:scale-[0.95] shadow-[0_0_12px_rgba(168,85,247,0.25)]"
          title="Add Target"
        >
          <Icons name="plus" size={14} strokeWidth={2.5} />
        </button>
      </div>
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
            ? 'bg-[#1c1833] text-white font-medium border border-coral/40 shadow-[0_0_12px_rgba(168,85,247,0.15)]'
            : 'text-tx-meta hover:text-tx-header hover:bg-surface-elevated/60 border border-transparent'}"
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

    <!-- Section: Workspaces / Projects (Hidden when collapsed) -->
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
            <span>Projects</span>
          </button>

          <button
            onclick={onNewProject}
            class="p-1 rounded-md text-tx-meta hover:text-coral hover:bg-surface-elevated transition-colors"
            title="Create New Project"
            aria-label="New Project"
          >
            <Icons name="plus" size={11} strokeWidth={2.5} />
          </button>
        </div>

        {#if !isProjectsCollapsed}
          <div class="mt-1 space-y-0.5">
            {#each workspaces as ws}
              {@const isWsActive = activeWorkspaceId === ws.id}
              <div class="group flex items-center gap-1">
                <button
                  onclick={() => onSelectWorkspace(ws.id)}
                  class="flex-1 flex items-center justify-between px-2.5 py-1.5 rounded-xl text-xs transition-colors {isWsActive
                    ? 'bg-surface-elevated text-tx-header font-medium border border-border-dim/80 shadow-subtle'
                    : 'text-tx-body hover:text-tx-header hover:bg-surface-hover border border-transparent'}"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="w-2 h-2 rounded-full shrink-0 ring-1 ring-white/10"
                      style="background-color: {ws.color || '#8b5cf6'};"
                    ></span>
                    <span class="truncate">{ws.name}</span>
                  </div>
                  <span class="text-[10px] text-tx-meta font-mono">{ws.servicesCount}</span>
                </button>

                <button
                  onclick={() => onEditProject(ws)}
                  class="opacity-0 group-hover:opacity-100 p-1.5 rounded-lg text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-all shrink-0"
                  title="Project settings"
                  aria-label="Project Settings"
                >
                  <Icons name="settings" size={11} />
                </button>
              </div>
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
