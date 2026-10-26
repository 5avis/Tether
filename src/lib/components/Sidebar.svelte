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
  class="{isCollapsed ? 'w-14' : 'w-[245px]'} shrink-0 h-full bg-[#0e111a] border-r border-white/[0.08] flex flex-col justify-between select-none font-sans z-40 text-xs transition-all duration-200"
>
  <!-- Top Section: Standard Profile Header & Primary Action Button -->
  <div class="p-2.5 pb-2 flex flex-col gap-2.5">
    {#if !isCollapsed}
      <!-- Workspace / Project Selector Header -->
      <div class="relative">
        <div class="flex items-center gap-1.5 w-full">
          <button
            onclick={() => (showWorkspaceMenu = !showWorkspaceMenu)}
            class="flex-1 flex items-center justify-between p-2 px-3 rounded-2xl bg-white/[0.03] hover:bg-white/[0.06] border border-white/[0.08] hover:border-white/20 transition-all text-left shadow-sm group"
            title="Switch Project"
          >
            <div class="flex items-center gap-2 min-w-0">
              <span
                class="w-2.5 h-2.5 rounded-full shrink-0 ring-1 ring-white/20"
                style="background-color: {currentWorkspace?.color || '#8b5cf6'};"
              ></span>
              <div class="min-w-0 flex flex-col">
                <span class="text-xs font-semibold text-white truncate">
                  {currentWorkspace?.name ?? 'Main Project'}
                </span>
                <span class="text-[10px] text-[#94a3b8] font-mono truncate">
                  {currentWorkspace?.servicesCount ?? 0} active targets
                </span>
              </div>
            </div>
            <Icons
              name="chevron-down"
              size={11}
              class="text-[#94a3b8] group-hover:text-white transition-transform duration-150 {showWorkspaceMenu ? 'rotate-180' : ''}"
            />
          </button>

          {#if currentWorkspace}
            <button
              onclick={() => onEditProject(currentWorkspace)}
              class="w-8 h-8 rounded-full bg-white/[0.03] hover:bg-white/[0.06] border border-white/[0.08] hover:border-white/20 text-[#94a3b8] hover:text-white transition-all shadow-sm shrink-0 flex items-center justify-center"
              title="Edit project settings"
              aria-label="Edit Project"
            >
              <Icons name="settings" size={13} />
            </button>
          {/if}
        </div>

        <!-- Dropdown for Switching Workspaces with Auto-Dismiss on Mouse Leave -->
        {#if showWorkspaceMenu}
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (showWorkspaceMenu = false)}
            aria-label="Close menu"
          ></button>

          <div
            role="menu"
            tabindex="-1"
            onmouseleave={() => (showWorkspaceMenu = false)}
            class="absolute top-12 left-0 right-0 bg-[#151926] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/80 p-2 z-50 animate-in fade-in zoom-in-95 duration-100"
          >
            <div class="px-3 py-1.5 text-[10px] uppercase font-mono text-[#94a3b8] tracking-wider flex items-center justify-between">
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
                    ? 'bg-white/[0.1] text-white font-medium border border-white/20'
                    : 'text-[#94a3b8] hover:text-white border border-transparent'}"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="w-2 h-2 rounded-full shrink-0 ring-1 ring-white/10"
                      style="background-color: {ws.color || '#8b5cf6'};"
                    ></span>
                    <span class="truncate">{ws.name}</span>
                  </div>
                  <span class="text-[10px] text-[#94a3b8] font-mono px-2 py-0.2 rounded-full bg-white/[0.04] border border-white/[0.08] shrink-0">
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
                class="w-full flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-full bg-white/[0.05] hover:bg-white/[0.1] text-white font-medium text-xs border border-white/10 transition-all shadow-sm"
              >
                <Icons name="plus" size={12} strokeWidth={2.5} />
                <span>Create New Project</span>
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Primary Action: Clean Desktop Button -->
      <button
        onclick={onAddService}
        class="w-full py-2 px-3 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white font-medium text-xs flex items-center justify-center gap-2 shadow-sm transition-all active:scale-[0.98] border border-white/10"
      >
        <Icons name="plus" size={13} strokeWidth={2.5} />
        <span>Add Target</span>
      </button>
    {:else}
      <!-- Collapsed Icon Rail Header (Add Target) -->
      <div class="flex flex-col items-center gap-2">
        <button
          onclick={onAddService}
          class="w-9 h-9 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white flex items-center justify-center shadow-sm transition-all border border-white/10 active:scale-[0.95]"
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
            ? 'bg-white/[0.08] text-white font-medium border border-white/[0.12] shadow-sm'
            : 'text-[#94a3b8] hover:text-white hover:bg-white/[0.04] border border-transparent'}"
          title={isCollapsed ? item.label : undefined}
        >
          <div class="flex items-center gap-2.5">
            <Icons
              name={item.icon}
              size={15}
              class={isActive ? 'text-[#00f2fe]' : 'text-[#94a3b8] group-hover:text-white'}
            />
            {#if !isCollapsed}
              <span>{item.label}</span>
            {/if}
          </div>

          {#if !isCollapsed && item.count !== undefined}
            <span
              class="px-2 py-0.5 rounded-full text-[10px] font-mono {isActive
                ? 'bg-white/[0.1] text-white font-semibold border border-white/20'
                : 'bg-white/[0.04] text-[#94a3b8] border border-white/[0.08]'}"
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
        <div class="flex items-center justify-between px-3 py-1 text-[#94a3b8]">
          <button
            onclick={() => (isProjectsCollapsed = !isProjectsCollapsed)}
            class="flex items-center gap-1.5 text-[10px] font-medium tracking-wider uppercase font-mono hover:text-white transition-colors"
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
            class="w-6 h-6 rounded-full flex items-center justify-center text-[#94a3b8] hover:text-[#00f2fe] hover:bg-white/[0.06] transition-colors"
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
                    ? 'bg-white/[0.06] text-white font-medium border border-white/[0.12] shadow-sm'
                    : 'text-[#94a3b8] hover:text-white hover:bg-white/[0.03] border border-transparent'}"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="w-2 h-2 rounded-full shrink-0 ring-1 ring-white/10"
                      style="background-color: {ws.color || '#8b5cf6'};"
                    ></span>
                    <span class="truncate">{ws.name}</span>
                  </div>
                  <span class="text-[10px] text-[#94a3b8] font-mono">{ws.servicesCount}</span>
                </button>

                <button
                  onclick={() => onEditProject(ws)}
                  class="opacity-0 group-hover:opacity-100 w-6 h-6 rounded-full flex items-center justify-center text-[#94a3b8] hover:text-white hover:bg-white/[0.06] transition-all shrink-0"
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
    <div class="p-3 flex items-center {isCollapsed ? 'justify-center' : 'justify-between'} text-[11px] text-[#94a3b8]">
      {#if !isCollapsed}
        <span class="font-mono text-[10px]">Tether v0.1</span>
        <span class="text-slate-300 font-mono text-[10px] flex items-center gap-1.5">
          <span class="w-1.5 h-1.5 rounded-full bg-[#10b981]"></span>
          Local-First
        </span>
      {:else}
        <span class="w-2 h-2 rounded-full bg-[#10b981]" title="Tether Local-First"></span>
      {/if}
    </div>
  </div>
</aside>
