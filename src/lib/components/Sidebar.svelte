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
  class="{isCollapsed ? 'w-14' : 'w-[245px]'} shrink-0 h-full bg-[#0b0d19]/80 backdrop-blur-xl border-r border-white/[0.08] flex flex-col justify-between select-none font-sans z-40 text-xs transition-all duration-200"
>
  <!-- Top Section: Standard Profile Header & Primary Action Button -->
  <div class="p-2.5 pb-2 flex flex-col gap-2.5">
    {#if !isCollapsed}
      <!-- Workspace / Project Selector Header -->
      <div class="relative">
        <div class="flex items-center gap-1.5 w-full">
          <button
            onclick={() => (showWorkspaceMenu = !showWorkspaceMenu)}
            class="flex-1 flex items-center justify-between p-2 px-3 rounded-2xl bg-white/[0.03] hover:bg-white/[0.06] border border-white/[0.08] hover:border-white/20 transition-all text-left shadow-subtle group backdrop-blur-md"
            title="Switch Project"
          >
            <div class="flex items-center gap-2 min-w-0">
              <span
                class="w-2.5 h-2.5 rounded-full shrink-0 ring-2 ring-white/10 shadow-[0_0_8px_#8b5cf6]"
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
              class="w-8 h-8 rounded-full bg-white/[0.03] hover:bg-white/[0.06] border border-white/[0.08] hover:border-white/20 text-tx-meta hover:text-tx-header transition-all shadow-subtle shrink-0 flex items-center justify-center"
              title="Edit project settings"
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
            class="absolute top-12 left-0 right-0 bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-3xl shadow-[0_0_35px_rgba(139,92,246,0.25)] p-2 z-50 animate-in fade-in zoom-in-95 duration-100"
          >
            <div class="px-3 py-1.5 text-[10px] uppercase font-mono text-tx-meta tracking-wider flex items-center justify-between">
              <span>All Projects</span>
              <span class="text-[9px] text-[#00f2fe] font-mono font-semibold">{workspaces.length} total</span>
            </div>

            <div class="space-y-1 max-h-48 overflow-y-auto">
              {#each workspaces as ws}
                {@const isWsActive = activeWorkspaceId === ws.id}
                <button
                  onclick={() => {
                    onSelectWorkspace(ws.id);
                    showWorkspaceMenu = false;
                  }}
                  class="w-full flex items-center justify-between px-3 py-1.5 rounded-full hover:bg-white/[0.06] text-left transition-colors {isWsActive
                    ? 'bg-[#8b5cf6]/20 text-white font-medium border border-[#8b5cf6]/50 shadow-[0_0_12px_rgba(139,92,246,0.2)]'
                    : 'text-tx-body hover:text-tx-header border border-transparent'}"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="w-2 h-2 rounded-full shrink-0 ring-1 ring-white/10"
                      style="background-color: {ws.color || '#8b5cf6'};"
                    ></span>
                    <span class="truncate">{ws.name}</span>
                  </div>
                  <span class="text-[10px] text-tx-meta font-mono px-2 py-0.2 rounded-full bg-white/[0.04] border border-white/[0.08] shrink-0">
                    {ws.servicesCount}
                  </span>
                </button>
              {/each}
            </div>

            <div class="pt-2 mt-1.5 border-t border-white/[0.08]">
              <button
                type="button"
                onclick={() => {
                  showWorkspaceMenu = false;
                  onNewProject();
                }}
                class="w-full flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-full bg-[#8b5cf6]/20 hover:bg-[#8b5cf6]/30 text-[#00f2fe] font-medium text-xs border border-[#8b5cf6]/40 transition-all shadow-[0_0_15px_rgba(0,242,254,0.15)]"
              >
                <Icons name="plus" size={12} strokeWidth={2.5} />
                <span>Create New Project</span>
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Primary Action: Pill-shaped neon gradient button -->
      <button
        onclick={onAddService}
        class="w-full py-2 px-3 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-medium text-xs flex items-center justify-center gap-2 shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all active:scale-[0.98] border border-white/20"
      >
        <Icons name="plus" size={13} strokeWidth={2.5} />
        <span>Add Target</span>
      </button>
    {:else}
      <!-- Collapsed Icon Rail Header (Add Target) -->
      <div class="flex flex-col items-center gap-2">
        <button
          onclick={onAddService}
          class="w-9 h-9 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] text-white flex items-center justify-center shadow-[0_0_15px_rgba(139,92,246,0.35)] transition-all border border-white/20 active:scale-[0.95]"
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
          class="w-full flex items-center {isCollapsed ? 'justify-center p-2 rounded-full' : 'justify-between px-3 py-2 rounded-full'} text-xs transition-all {isActive
            ? 'bg-gradient-to-r from-[#8b5cf6]/25 to-[#00f2fe]/10 text-white font-medium border border-[#8b5cf6]/50 shadow-[0_0_18px_rgba(139,92,246,0.25)]'
            : 'text-tx-meta hover:text-tx-header hover:bg-white/[0.04] border border-transparent'}"
          title={isCollapsed ? item.label : undefined}
        >
          <div class="flex items-center gap-2.5">
            <Icons
              name={item.icon}
              size={15}
              class={isActive ? 'text-[#00f2fe]' : 'text-tx-meta group-hover:text-tx-body'}
            />
            {#if !isCollapsed}
              <span>{item.label}</span>
            {/if}
          </div>

          {#if !isCollapsed && item.count !== undefined}
            <span
              class="px-2 py-0.5 rounded-full text-[10px] font-mono {isActive
                ? 'bg-[#8b5cf6]/30 text-[#00f2fe] font-semibold border border-[#8b5cf6]/40'
                : 'bg-white/[0.04] text-tx-meta border border-white/[0.08]'}"
            >
              {item.count}
            </span>
          {/if}
        </button>
      {/each}
    </nav>

    <!-- Section: Workspaces / Projects (Hidden when collapsed) -->
    {#if !isCollapsed}
      <div class="pt-2.5 border-t border-white/[0.08]">
        <div class="flex items-center justify-between px-3 py-1 text-tx-meta">
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
            class="w-6 h-6 rounded-full flex items-center justify-center text-tx-meta hover:text-[#00f2fe] hover:bg-white/[0.06] transition-colors"
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
                  class="flex-1 flex items-center justify-between px-3 py-1.5 rounded-full text-xs transition-colors {isWsActive
                    ? 'bg-white/[0.06] text-tx-header font-medium border border-white/[0.12] shadow-subtle'
                    : 'text-tx-body hover:text-tx-header hover:bg-white/[0.03] border border-transparent'}"
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
                  class="opacity-0 group-hover:opacity-100 w-6 h-6 rounded-full flex items-center justify-center text-tx-meta hover:text-tx-header hover:bg-white/[0.06] transition-all shrink-0"
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
  <div class="border-t border-white/[0.08]">
    <div class="p-3 flex items-center {isCollapsed ? 'justify-center' : 'justify-between'} text-[11px] text-tx-meta">
      {#if !isCollapsed}
        <span class="font-mono text-[10px]">Tether v0.1</span>
        <span class="text-[#00f2fe] font-mono text-[10px] flex items-center gap-1">
          <span class="w-1.5 h-1.5 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
          Local-First
        </span>
      {:else}
        <span class="w-2 h-2 rounded-full bg-[#00f2fe] shadow-[0_0_8px_#00f2fe]" title="Tether Local-First"></span>
      {/if}
    </div>
  </div>
</aside>
