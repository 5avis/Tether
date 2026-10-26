<script lang="ts">
  import { isAdminUser } from '$lib/types';
  import Icons from './Icons.svelte';

  let {
    workspaceName = 'Production Core',
    currentView = 'Overview',
    onSync = () => {},
    isSyncing = false,
    onToggleSidebar = () => {},
    onNavigateHome = () => {},
    onOpenProfile = () => {},
    onSignOut = () => {},
    isSidebarCollapsed = false,
    profile = null,
  }: {
    workspaceName?: string;
    currentView?: string;
    onSync?: () => void;
    isSyncing?: boolean;
    onToggleSidebar?: () => void;
    onNavigateHome?: () => void;
    onOpenProfile?: () => void;
    onSignOut?: () => void;
    isSidebarCollapsed?: boolean;
    profile?: any;
  } = $props();

  let showProfileMenu = $state(false);

  const isAdmin = $derived(isAdminUser(profile));
</script>

<header
  data-tauri-drag-region
  class="h-11 shrink-0 bg-[#0e111a] border-b border-white/[0.08] flex items-center justify-between px-3.5 select-none z-50 relative"
>
  <!-- Left: Toggle Sidebar, Tether Brand & Breadcrumbs -->
  <div class="flex items-center gap-2.5 min-w-0">
    <!-- Sidebar collapse/expand toggle button -->
    <button
      onclick={onToggleSidebar}
      class="p-1.5 rounded-lg text-[#94a3b8] hover:text-white hover:bg-white/[0.06] transition-colors"
      title={isSidebarCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      aria-label="Toggle sidebar"
    >
      <Icons name="sidebar" size={14} class="text-[#94a3b8] hover:text-white" />
    </button>

    <!-- Tether Brand Diamond Icon & Label -->
    <button
      onclick={onNavigateHome}
      data-tauri-drag-region
      class="flex items-center gap-2 cursor-pointer hover:opacity-90 transition-opacity"
      title="Go to Overview"
    >
      <img
        src="/tether-icon.svg"
        alt="Tether"
        class="w-6 h-6 object-contain shrink-0"
      />
      <span class="text-xs font-semibold tracking-wide text-white">Tether</span>
      <span class="text-[9px] px-2 py-0.5 rounded-full bg-white/[0.04] text-[#94a3b8] font-mono border border-white/[0.08] hidden sm:inline">v0.1</span>
    </button>

    <div class="h-4 w-[1px] bg-white/[0.08] hidden sm:block"></div>

    <!-- Clean Breadcrumb Navigation -->
    <div class="hidden sm:flex items-center gap-1.5 text-xs text-[#94a3b8] truncate">
      <button
        onclick={onNavigateHome}
        class="text-[#94a3b8] hover:text-white transition-colors truncate"
      >
        {workspaceName}
      </button>
      <span class="text-white/20">/</span>
      <span class="text-white font-semibold truncate">{currentView}</span>
    </div>
  </div>

  <!-- Center Window Drag Handle -->
  <div data-tauri-drag-region class="flex-1 h-full cursor-grab"></div>

  <!-- Right Actions: Standard Live Status & Refresh Sync -->
  <div class="flex items-center gap-2 shrink-0 relative">
    <div class="hidden sm:flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-white/[0.04] text-[11px] border border-white/[0.08]">
      <span class="w-1.5 h-1.5 rounded-full bg-[#10b981]"></span>
      <span class="text-white text-[11px] font-mono">Live</span>
    </div>

    {#if isAdmin}
      <button
        onclick={onSync}
        class="flex items-center gap-1.5 px-3 py-1 rounded-full text-xs bg-white/[0.04] hover:bg-white/[0.08] text-white border border-white/[0.08] hover:border-white/20 transition-all active:scale-[0.98] shadow-sm"
        title="Sync status"
      >
        <Icons name="refresh" size={12} class="text-[#8b5cf6] {isSyncing ? 'animate-spin' : ''}" />
        <span>Sync</span>
      </button>
    {/if}

    <!-- Profile Button (Directly right of Sync) -->
    <div class="relative">
      <button
        onclick={() => (showProfileMenu = !showProfileMenu)}
        class="flex items-center gap-1.5 p-1 pl-1.5 pr-2.5 rounded-full border transition-all active:scale-[0.98] {currentView === 'Profile' || showProfileMenu
          ? 'bg-white/[0.1] border-white/20 text-white shadow-sm'
          : 'bg-white/[0.04] hover:bg-white/[0.08] border-white/[0.08] hover:border-white/20 text-slate-200'}"
        title="Profile"
      >
        {#if profile?.picture}
          <img
            src={profile.picture}
            alt={profile.name}
            class="w-5 h-5 rounded-full object-cover border {isAdmin ? 'border-[#8b5cf6]' : 'border-white/10'} shrink-0"
          />
        {:else}
          <div class="w-5 h-5 rounded-full bg-white/[0.04] border {isAdmin ? 'border-[#8b5cf6] text-[#8b5cf6]' : 'border-white/10 text-white'} flex items-center justify-center text-[10px] font-bold shrink-0">
            {profile?.name ? profile.name.charAt(0).toUpperCase() : 'S'}
          </div>
        {/if}
        <span class="text-xs max-w-[90px] truncate">{profile?.signed_in ? profile.name : 'Profile'}</span>
        <Icons name="chevron-down" size={10} class="text-[#94a3b8] transition-transform {showProfileMenu ? 'rotate-180' : ''}" />
      </button>

      <!-- Dropdown Popover Menu with onmouseleave Auto-Dismiss -->
      {#if showProfileMenu}
        <!-- Backdrop for click-outside dismissal -->
        <button
          type="button"
          class="fixed inset-0 z-40 cursor-default bg-transparent"
          onclick={() => (showProfileMenu = false)}
          aria-label="Close menu"
        ></button>

        <div
          role="menu"
          tabindex="-1"
          onmouseleave={() => (showProfileMenu = false)}
          class="absolute right-0 top-11 w-64 bg-[#151926] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/80 p-3.5 z-50 text-xs animate-in fade-in zoom-in-95 duration-100 space-y-3"
        >
          <!-- User Info Header -->
          <div class="flex items-center gap-2.5 pb-2.5 border-b border-white/[0.08]">
            {#if profile?.picture}
              <img
                src={profile.picture}
                alt={profile.name}
                class="w-9 h-9 rounded-full object-cover border border-white/20 shrink-0"
              />
            {:else}
              <div class="w-9 h-9 rounded-full bg-white/[0.05] border border-white/20 flex items-center justify-center text-[#8b5cf6] font-bold text-sm shrink-0">
                {profile?.name ? profile.name.charAt(0).toUpperCase() : 'S'}
              </div>
            {/if}
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-1.5 flex-wrap">
                <span class="font-bold text-white truncate">{profile?.signed_in ? profile.name : 'Guest Session'}</span>
              </div>
              <p class="text-[11px] text-[#94a3b8] font-mono truncate">{profile?.signed_in ? profile.email : 'Local only'}</p>

              {#if isAdmin}
                <div class="mt-1 flex items-center gap-1 px-2 py-0.5 rounded-full bg-[#8b5cf6]/20 border border-[#8b5cf6]/40 text-[#8b5cf6] font-mono text-[9px] w-fit font-semibold">
                  <Icons name="crown" size={10} />
                  <span>Admin</span>
                </div>
              {:else if profile?.signed_in}
                <div class="mt-1 flex items-center gap-1 px-2 py-0.5 rounded-full bg-white/[0.05] border border-white/10 text-[#94a3b8] font-mono text-[9px] w-fit">
                  <span>Member</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Navigation Links -->
          <div class="space-y-1">
            <button
              onclick={() => {
                showProfileMenu = false;
                onOpenProfile();
              }}
              class="w-full flex items-center gap-2 px-3 py-2 rounded-full text-left hover:bg-white/[0.06] text-white transition-colors"
            >
              <Icons name="shield" size={13} class="text-[#8b5cf6]" />
              <span>Profile</span>
            </button>
          </div>

          <!-- Sign Out / In Action -->
          <div class="pt-2 border-t border-white/[0.08]">
            {#if profile?.signed_in}
              <button
                onclick={() => {
                  showProfileMenu = false;
                  onSignOut();
                }}
                class="w-full flex items-center gap-2 px-3 py-2 rounded-full text-left text-rose-400 hover:bg-rose-950/30 transition-colors font-medium"
              >
                <Icons name="logout" size={13} />
                <span>Log Out</span>
              </button>
            {:else}
              <button
                onclick={() => {
                  showProfileMenu = false;
                  onOpenProfile();
                }}
                class="w-full flex items-center justify-center gap-2 px-3 py-2 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white transition-all font-medium shadow-sm"
              >
                <Icons name="google" size={13} />
                <span>Sign in with Google</span>
              </button>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
</header>
