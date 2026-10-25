<script lang="ts">
  import { type UserProfile, type Workspace, type ServiceTarget, isAdminUser } from '$lib/types';
  import { startGoogleOAuth, signOutProfile, getOAuthCredentials } from '$lib/storage';
  import Icons from './Icons.svelte';

  let {
    profile = null,
    workspaces = [],
    services = [],
    onProfileUpdated = (p: UserProfile | null) => {},
    onNavigate = (view: string) => {},
    onToast = (msg: string) => {},
  }: {
    profile: UserProfile | null;
    workspaces: Workspace[];
    services: ServiceTarget[];
    onProfileUpdated: (p: UserProfile | null) => void;
    onNavigate: (view: any) => void;
    onToast: (msg: string) => void;
  } = $props();

  let isAuthenticating = $state(false);
  let showClientSecret = $state(false);
  let copiedField = $state<string | null>(null);

  let clientId = $state('');
  let clientSecret = $state('');
  let projectId = $state('tether-app');
  let redirectMode = $state('Loopback Port');

  const isAdmin = $derived(isAdminUser(profile));

  $effect(() => {
    if (isAdmin) {
      getOAuthCredentials().then((creds) => {
        if (creds) {
          clientId = creds.client_id;
          clientSecret = creds.client_secret;
          projectId = creds.project_id;
          redirectMode = creds.redirect_mode;
        }
      });
    }
  });

  async function handleGoogleSignIn() {
    isAuthenticating = true;
    onToast('Opening browser for Google sign-in...');
    try {
      const user = await startGoogleOAuth();
      onProfileUpdated(user);
      onToast(`Connected as ${user.name}`);
    } catch (e: any) {
      onToast(`Google sign-in error: ${e?.message || e}`);
    } finally {
      isAuthenticating = false;
    }
  }

  async function handleSignOut() {
    await signOutProfile();
    onProfileUpdated(null);
    onToast('Signed out from Google account.');
  }

  function handleCopy(text: string, fieldName: string) {
    navigator.clipboard?.writeText(text);
    copiedField = fieldName;
    onToast(`Copied ${fieldName} to clipboard`);
    setTimeout(() => {
      copiedField = null;
    }, 1800);
  }

  function handleExportBackup() {
    const backupData = {
      profile: profile ? { name: profile.name, email: profile.email, role: profile.role } : null,
      workspacesCount: workspaces.length,
      servicesCount: services.length,
      exportedAt: new Date().toISOString(),
      storageFile: '~/.config/tether/config.json',
    };
    navigator.clipboard?.writeText(JSON.stringify(backupData, null, 2));
    onToast('Copied configuration snapshot to clipboard.');
  }
</script>

<div class="flex-1 flex flex-col min-h-0 bg-canvas text-xs select-none overflow-y-auto">
  <!-- Top Header Banner -->
  <div class="p-4 sm:p-6 border-b border-border-dim bg-surface/30">
    <div class="max-w-4xl mx-auto flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2.5 flex-wrap">
          <h1 class="text-base font-semibold text-tx-header tracking-tight">Profile</h1>
          <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-header border border-border-dim flex items-center gap-1.5">
            <span class="w-1.5 h-1.5 rounded-full {profile?.signed_in ? 'bg-coral' : 'bg-border-subtle'}"></span>
            {profile?.signed_in ? (isAdmin ? 'Admin' : 'Member') : 'Guest'}
          </span>
        </div>
      </div>

      <!-- Quick back action -->
      <button
        onclick={() => onNavigate('overview')}
        class="px-3.5 py-1.5 rounded-xl bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim hover:border-border-subtle text-xs transition-all flex items-center gap-1.5 active:scale-[0.98] self-start sm:self-auto"
      >
        <Icons name="chevron-left" size={12} />
        <span>Back to Overview</span>
      </button>
    </div>
  </div>

  <!-- Content Deck -->
  <div class="p-4 sm:p-6 max-w-4xl mx-auto w-full space-y-4">
    <!-- Hero Identity Card -->
    <div class="p-5 sm:p-6 rounded-2xl bg-surface/70 border border-border-dim/80 shadow-subtle flex flex-col sm:flex-row sm:items-center justify-between gap-5">
      <div class="flex items-center gap-4 min-w-0">
        <!-- Avatar -->
        {#if profile?.picture}
          <img
            src={profile.picture}
            alt={profile.name}
            class="w-14 h-14 rounded-2xl object-cover border-2 {isAdmin ? 'border-coral shadow-coral/20' : 'border-border-dim'} shadow-sm shrink-0"
          />
        {:else}
          <div
            class="w-14 h-14 rounded-2xl bg-surface-elevated border-2 {isAdmin ? 'border-coral text-coral' : 'border-border-dim text-tx-header'} flex items-center justify-center font-bold text-lg shadow-sm shrink-0"
          >
            {profile?.name ? profile.name.charAt(0).toUpperCase() : 'S'}
          </div>
        {/if}

        <div class="min-w-0 flex flex-col gap-1">
          <div class="flex items-center gap-2 flex-wrap">
            <h2 class="text-base font-semibold text-tx-header truncate">
              {profile?.signed_in ? profile.name : 'Guest Session'}
            </h2>

            {#if profile?.signed_in}
              {#if isAdmin}
                <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-coral/15 text-coral border border-coral/30 flex items-center gap-1 font-semibold">
                  <Icons name="crown" size={11} class="text-coral" />
                  Admin
                </span>
              {:else}
                <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-body border border-border-dim flex items-center gap-1">
                  Member
                </span>
              {/if}
            {:else}
              <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-meta border border-border-dim">
                Unlinked
              </span>
            {/if}
          </div>

          <p class="text-xs text-tx-meta truncate font-mono">
            {profile?.signed_in ? profile.email : 'Local session only'}
          </p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2 shrink-0">
        {#if profile?.signed_in}
          <button
            onclick={handleGoogleSignIn}
            disabled={isAuthenticating}
            class="px-3.5 py-2 rounded-xl bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim hover:border-coral/40 font-medium text-xs transition-all active:scale-[0.98] flex items-center gap-1.5"
            title="Re-verify authentication token"
          >
            <Icons name="refresh" size={13} class={isAuthenticating ? 'animate-spin text-coral' : ''} />
            <span>Re-verify</span>
          </button>

          <button
            onclick={handleSignOut}
            class="px-4 py-2 rounded-xl bg-red-950/20 hover:bg-red-950/50 text-red-400 border border-red-900/30 hover:border-red-600/50 font-medium text-xs transition-all active:scale-[0.98] flex items-center gap-1.5 shadow-sm"
            title="Log out"
          >
            <Icons name="logout" size={13} />
            <span>Log Out</span>
          </button>
        {:else}
          <button
            onclick={handleGoogleSignIn}
            disabled={isAuthenticating}
            class="px-5 py-2.5 rounded-xl bg-coral hover:bg-coral-hover text-white font-medium text-xs shadow-sm transition-all border border-coral/60 flex items-center gap-2 active:scale-[0.98] disabled:opacity-50"
          >
            {#if isAuthenticating}
              <Icons name="refresh" size={13} class="animate-spin" />
              <span>Awaiting browser sign-in...</span>
            {:else}
              <Icons name="google" size={14} />
              <span>Continue with Google</span>
            {/if}
          </button>
        {/if}
      </div>
    </div>

    <!-- Details Grid: OAuth Credentials (STRICTLY Admin Only) & Storage -->
    <div class="grid grid-cols-1 {isAdmin ? 'md:grid-cols-2' : ''} gap-4">
      <!-- Google OAuth Credentials Card (ONLY shown to Admin) -->
      {#if isAdmin}
        <div class="p-5 rounded-2xl bg-surface/60 border border-border-dim/80 shadow-subtle space-y-3.5">
          <div class="flex items-center justify-between pb-2 border-b border-border-dim/60">
            <div class="flex items-center gap-2">
              <div class="w-6 h-6 rounded-lg bg-coral/15 border border-coral/30 flex items-center justify-center text-coral">
                <Icons name="google" size={13} />
              </div>
              <h3 class="text-xs font-semibold text-tx-header">OAuth Credentials</h3>
            </div>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded-full bg-surface-elevated text-coral border border-coral/30 font-semibold">
              oauth.json
            </span>
          </div>

          <div class="space-y-2.5 text-xs font-mono">
            <div>
              <div class="flex items-center justify-between text-tx-meta text-[11px] mb-1">
                <span>Client ID</span>
                <button
                  onclick={() => handleCopy(clientId, 'Client ID')}
                  class="hover:text-tx-header transition-colors flex items-center gap-1"
                >
                  <Icons name={copiedField === 'Client ID' ? 'check' : 'copy'} size={11} class={copiedField === 'Client ID' ? 'text-coral' : ''} />
                  <span>{copiedField === 'Client ID' ? 'Copied' : 'Copy'}</span>
                </button>
              </div>
              <div class="p-2 rounded-lg bg-surface-elevated border border-border-dim text-[11px] text-tx-header truncate select-text">
                {clientId}
              </div>
            </div>

            <div>
              <div class="flex items-center justify-between text-tx-meta text-[11px] mb-1">
                <span>Client Secret</span>
                <div class="flex items-center gap-2">
                  <button
                    onclick={() => (showClientSecret = !showClientSecret)}
                    class="hover:text-tx-header transition-colors"
                  >
                    {showClientSecret ? 'Hide' : 'Reveal'}
                  </button>
                  <span class="text-border-subtle">•</span>
                  <button
                    onclick={() => handleCopy(clientSecret, 'Client Secret')}
                    class="hover:text-tx-header transition-colors flex items-center gap-1"
                  >
                    <Icons name={copiedField === 'Client Secret' ? 'check' : 'copy'} size={11} class={copiedField === 'Client Secret' ? 'text-coral' : ''} />
                    <span>{copiedField === 'Client Secret' ? 'Copied' : 'Copy'}</span>
                  </button>
                </div>
              </div>
              <div class="p-2 rounded-lg bg-surface-elevated border border-border-dim text-[11px] text-tx-header font-mono select-text">
                {showClientSecret ? clientSecret : '••••••••••••••••••••••••••••••••••••'}
              </div>
            </div>

            <div class="grid grid-cols-2 gap-2 text-[11px] pt-1">
              <div class="p-2 rounded-lg bg-surface-elevated border border-border-dim">
                <span class="block text-tx-meta text-[10px]">Project ID</span>
                <span class="font-semibold text-tx-header">{projectId}</span>
              </div>
              <div class="p-2 rounded-lg bg-surface-elevated border border-border-dim">
                <span class="block text-tx-meta text-[10px]">Redirect Mode</span>
                <span class="font-semibold text-tx-header">{redirectMode}</span>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Storage & Environment Settings Card -->
      <div class="p-5 rounded-2xl bg-surface/60 border border-border-dim/80 shadow-subtle space-y-3.5">
        <div class="flex items-center justify-between pb-2 border-b border-border-dim/60">
          <div class="flex items-center gap-2">
            <div class="w-6 h-6 rounded-lg bg-surface-elevated border border-border-dim flex items-center justify-center text-coral">
              <Icons name="shield" size={13} />
            </div>
            <h3 class="text-xs font-semibold text-tx-header">Storage</h3>
          </div>
          <span class="text-[10px] font-mono px-2 py-0.5 rounded-full bg-surface-elevated text-tx-header border border-border-dim">
            Local-First
          </span>
        </div>

        <div class="space-y-2.5 text-xs">
          <div class="p-2.5 rounded-xl bg-surface-elevated border border-border-dim flex items-center justify-between">
            <div>
              <span class="block text-tx-header font-medium text-xs">Config File</span>
              <span class="font-mono text-[11px] text-tx-meta">~/.config/tether/config.json</span>
            </div>
            <span class="px-2 py-0.5 rounded font-mono text-[10px] bg-surface text-tx-body border border-border-dim">
              Local
            </span>
          </div>

          <div class="p-2.5 rounded-xl bg-surface-elevated border border-border-dim flex items-center justify-between">
            <div>
              <span class="block text-tx-header font-medium text-xs">Config Directory</span>
              <span class="font-mono text-[11px] text-tx-meta">~/.config/tether/</span>
            </div>
            <span class="px-2 py-0.5 rounded font-mono text-[10px] bg-surface text-tx-body border border-border-dim">
              Directory
            </span>
          </div>

          <div class="p-2.5 rounded-xl bg-surface-elevated border border-border-dim flex items-center justify-between">
            <div>
              <span class="block text-tx-header font-medium text-xs">Telemetry</span>
              <span class="text-[11px] text-tx-meta">No external tracking relays</span>
            </div>
            <span class="px-2 py-0.5 rounded font-mono text-[10px] bg-surface text-tx-header border border-border-dim">
              Disabled
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Non-Admin Notice for Members -->
    {#if profile?.signed_in && !isAdmin}
      <div class="p-4 rounded-xl bg-surface/50 border border-border-dim/80 flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-surface-elevated border border-border-dim flex items-center justify-center text-tx-meta shrink-0">
          <Icons name="shield" size={14} />
        </div>
        <div>
          <span class="font-semibold text-tx-header text-xs">Member Access</span>
          <p class="text-[11px] text-tx-meta mt-0.5">
            You are signed in with read-only view. Cloud deployments, target creation, and OAuth credentials are restricted to the administrator.
          </p>
        </div>
      </div>
    {/if}

    <!-- Backup & Export -->
    <div class="p-4 rounded-xl bg-surface/40 border border-border-dim/70 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div>
        <span class="font-semibold text-tx-header text-xs">Configuration Backup</span>
        <p class="text-[11px] text-tx-meta mt-0.5">
          Copy a JSON snapshot of your workspaces, cloud bindings, and settings.
        </p>
      </div>

      <button
        onclick={handleExportBackup}
        class="px-4 py-2 rounded-xl bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim hover:border-coral/50 font-medium text-xs transition-all active:scale-[0.98] flex items-center gap-1.5 shrink-0"
      >
        <Icons name="copy" size={13} class="text-coral" />
        <span>Export Config</span>
      </button>
    </div>
  </div>
</div>
