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
  class="h-10 shrink-0 bg-surface border-b border-border-dim flex items-center justify-between px-3 select-none z-50 relative"
>
  <!-- Left: Toggle Sidebar, Tether Brand & Breadcrumbs -->
  <div class="flex items-center gap-2.5 min-w-0">
    <!-- Sidebar collapse/expand toggle button -->
    <button
      onclick={onToggleSidebar}
      class="p-1 rounded-md text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-colors"
      title={isSidebarCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
      aria-label="Toggle sidebar"
    >
      <Icons name="sidebar" size={14} class="text-tx-meta hover:text-tx-header" />
    </button>

    <!-- Tether Brand Knot Icon & Label -->
    <button
      onclick={onNavigateHome}
      data-tauri-drag-region
      class="flex items-center gap-2 cursor-pointer hover:opacity-90 transition-opacity"
      title="Go to Overview"
    >
      <img
        src="/tether-icon.png"
        alt="Tether"
        class="w-5 h-5 rounded-md object-contain shrink-0 shadow-sm"
      />
      <span class="text-xs font-semibold tracking-wide text-tx-header">Tether</span>
      <span class="text-[9px] px-1.5 py-0.2 rounded bg-surface-elevated text-tx-meta font-mono border border-border-dim hidden sm:inline">v0.1</span>
    </button>

    <div class="h-4 w-[1px] bg-border-dim hidden sm:block"></div>

    <!-- Clean Breadcrumb Navigation -->
    <div class="hidden sm:flex items-center gap-1.5 text-xs text-tx-meta truncate">
      <button
        onclick={onNavigateHome}
        class="text-tx-meta hover:text-tx-header transition-colors truncate"
      >
        {workspaceName}
      </button>
      <span class="text-border-subtle">/</span>
      <span class="text-tx-header font-semibold truncate">{currentView}</span>
    </div>
  </div>

  <!-- Center Window Drag Handle -->
  <div data-tauri-drag-region class="flex-1 h-full cursor-grab"></div>

  <!-- Right Actions: Standard Live Status & Refresh Sync -->
  <div class="flex items-center gap-2 shrink-0 relative">
    <div class="hidden sm:flex items-center gap-1.5 px-2 py-1 rounded-full bg-surface-elevated text-[11px] border border-border-dim">
      <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
      <span class="text-tx-header text-[11px] font-mono">Live</span>
    </div>

    {#if isAdmin}
      <button
        onclick={onSync}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim hover:border-border-subtle transition-all active:scale-[0.98]"
        title="Sync status"
      >
        <Icons name="refresh" size={12} class="text-coral {isSyncing ? 'animate-spin' : ''}" />
        <span>Sync</span>
      </button>
    {/if}

    <!-- Profile Button (Directly right of Sync) -->
    <div class="relative">
      <button
        onclick={() => (showProfileMenu = !showProfileMenu)}
        class="flex items-center gap-1.5 p-1 pl-1.5 pr-2.5 rounded-full border transition-all active:scale-[0.98] {currentView === 'Profile' || showProfileMenu
          ? 'bg-coral-light border-coral/50 text-coral font-medium shadow-subtle'
          : 'bg-surface-elevated hover:bg-surface-hover border-border-dim hover:border-border-subtle text-tx-header'}"
        title="Profile"
      >
        {#if profile?.picture}
          <img
            src={profile.picture}
            alt={profile.name}
            class="w-5 h-5 rounded-full object-cover border {isAdmin ? 'border-coral' : 'border-border-dim'} shrink-0"
          />
        {:else}
          <div class="w-5 h-5 rounded-full bg-surface border {isAdmin ? 'border-coral text-coral' : 'border-border-dim text-tx-header'} flex items-center justify-center text-[10px] font-bold shrink-0">
            {profile?.name ? profile.name.charAt(0).toUpperCase() : 'S'}
          </div>
        {/if}
        <span class="text-xs max-w-[90px] truncate">{profile?.signed_in ? profile.name : 'Profile'}</span>
        <Icons name="chevron-down" size={10} class="text-tx-meta transition-transform {showProfileMenu ? 'rotate-180' : ''}" />
      </button>

      <!-- Dropdown Popover Menu -->
      {#if showProfileMenu}
        <!-- Backdrop for click-outside dismissal -->
        <button
          type="button"
          class="fixed inset-0 z-40 cursor-default bg-transparent"
          onclick={() => (showProfileMenu = false)}
          aria-label="Close menu"
        ></button>

        <div
          class="absolute right-0 top-11 w-64 bg-[#222222] border border-border-dim rounded-2xl shadow-modal p-3 z-50 text-xs animate-in fade-in zoom-in-95 duration-100 space-y-3"
        >
          <!-- User Info Header -->
          <div class="flex items-center gap-2.5 pb-2.5 border-b border-border-dim/60">
            {#if profile?.picture}
              <img
                src={profile.picture}
                alt={profile.name}
                class="w-9 h-9 rounded-xl object-cover border border-coral/40 shrink-0"
              />
            {:else}
              <div class="w-9 h-9 rounded-xl bg-surface-elevated border border-coral/40 flex items-center justify-center text-coral font-bold text-sm shrink-0">
                {profile?.name ? profile.name.charAt(0).toUpperCase() : 'S'}
              </div>
            {/if}
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-1.5 flex-wrap">
                <span class="font-bold text-tx-header truncate">{profile?.signed_in ? profile.name : 'Guest Session'}</span>
              </div>
              <p class="text-[11px] text-tx-meta font-mono truncate">{profile?.signed_in ? profile.email : 'Local only'}</p>

              {#if isAdmin}
                <div class="mt-1 flex items-center gap-1 px-1.5 py-0.5 rounded-full bg-coral/15 border border-coral/30 text-coral font-mono text-[9px] w-fit font-semibold">
                  <Icons name="crown" size={10} />
                  <span>Admin</span>
                </div>
              {:else if profile?.signed_in}
                <div class="mt-1 flex items-center gap-1 px-1.5 py-0.5 rounded-full bg-surface-elevated border border-border-dim text-tx-meta font-mono text-[9px] w-fit">
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
              class="w-full flex items-center gap-2 px-2.5 py-2 rounded-xl text-left hover:bg-surface-elevated text-tx-header transition-colors"
            >
              <Icons name="shield" size={13} class="text-coral" />
              <span>Profile</span>
            </button>
          </div>

          <!-- Sign Out / In Action -->
          <div class="pt-2 border-t border-border-dim/60">
            {#if profile?.signed_in}
              <button
                onclick={() => {
                  showProfileMenu = false;
                  onSignOut();
                }}
                class="w-full flex items-center gap-2 px-2.5 py-2 rounded-xl text-left text-red-400 hover:bg-red-950/30 transition-colors font-medium"
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
                class="w-full flex items-center justify-center gap-2 px-3 py-2 rounded-xl bg-coral hover:bg-coral-hover text-white transition-all font-medium"
              >
                <Icons name="google" size={13} />
                <span>Sign in with Google</span>
              </button>
            {/if}
          </div>
        </div>
      {/if}
    </div>

    <!-- Quick Logout Header Icon Button (Visible when signed in) -->
    {#if profile?.signed_in}
      <button
        onclick={onSignOut}
        class="p-1.5 rounded-full bg-surface-elevated hover:bg-red-950/30 text-tx-meta hover:text-red-400 border border-border-dim hover:border-red-900/40 transition-colors shrink-0"
        title="Log Out"
        aria-label="Log Out"
      >
        <Icons name="logout" size={12} />
      </button>
    {/if}
  </div>
</header>
