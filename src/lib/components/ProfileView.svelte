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

<div class="flex-1 flex flex-col min-h-0 bg-transparent text-xs select-none overflow-y-auto">
  <!-- Top Header Banner -->
  <div class="p-4 sm:p-6 border-b border-white/[0.08] bg-white/[0.02] backdrop-blur-md">
    <div class="max-w-4xl mx-auto flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2.5 flex-wrap">
          <h1 class="text-base font-semibold text-white tracking-tight">Profile</h1>
          <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08] flex items-center gap-1.5">
            <span class="w-1.5 h-1.5 rounded-full {profile?.signed_in ? 'bg-[#00f2fe]' : 'bg-white/20'}"></span>
            {profile?.signed_in ? (isAdmin ? 'Admin' : 'Member') : 'Guest'}
          </span>
        </div>
      </div>

      <!-- Quick back action -->
      <button
        onclick={() => onNavigate('overview')}
        class="px-4 py-2 rounded-full bg-white/[0.04] hover:bg-white/[0.08] text-white border border-white/[0.08] hover:border-white/[0.15] text-xs transition-all flex items-center gap-2 active:scale-[0.98] self-start sm:self-auto"
      >
        <Icons name="chevron-left" size={12} />
        <span>Back to Overview</span>
      </button>
    </div>
  </div>

  <!-- Content Deck -->
  <div class="p-4 sm:p-6 max-w-4xl mx-auto w-full space-y-4">
    <!-- Hero Identity Card -->
    <div class="p-5 sm:p-6 rounded-2xl bg-white/[0.03] border border-white/[0.08] shadow-sm flex flex-col sm:flex-row sm:items-center justify-between gap-5">
      <div class="flex items-center gap-4 min-w-0">
        <!-- Avatar -->
        {#if profile?.picture}
          <img
            src={profile.picture}
            alt={profile.name}
            class="w-14 h-14 rounded-full object-cover border-2 {isAdmin ? 'border-[#8b5cf6]' : 'border-white/[0.1]'} shadow-sm shrink-0"
          />
        {:else}
          <div
            class="w-14 h-14 rounded-full bg-white/[0.05] border-2 {isAdmin ? 'border-[#8b5cf6] text-[#8b5cf6]' : 'border-white/[0.1] text-white'} flex items-center justify-center font-bold text-lg shadow-sm shrink-0"
          >
            {profile?.name ? profile.name.charAt(0).toUpperCase() : 'S'}
          </div>
        {/if}

        <div class="min-w-0 flex flex-col gap-1">
          <div class="flex items-center gap-2 flex-wrap">
            <h2 class="text-base font-semibold text-white truncate">
              {profile?.signed_in ? profile.name : 'Guest Session'}
            </h2>

            {#if profile?.signed_in}
              {#if isAdmin}
                <span class="px-3 py-0.5 rounded-full text-[10px] font-mono bg-[#8b5cf6]/15 text-[#8b5cf6] border border-[#8b5cf6]/30 flex items-center gap-1.5 font-semibold">
                  <Icons name="crown" size={11} class="text-[#8b5cf6]" />
                  Admin
                </span>
              {:else}
                <span class="px-3 py-0.5 rounded-full text-[10px] font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08] flex items-center gap-1">
                  Member
                </span>
              {/if}
            {:else}
              <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
                Unlinked
              </span>
            {/if}
          </div>

          <p class="text-xs text-[#94a3b8] truncate font-mono">
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
            class="px-4 py-2 rounded-full bg-white/[0.04] hover:bg-white/[0.08] text-white border border-white/[0.08] hover:border-white/[0.15] font-medium text-xs transition-all active:scale-[0.98] flex items-center gap-2"
            title="Re-verify authentication token"
          >
            <Icons name="refresh" size={13} class={isAuthenticating ? 'animate-spin text-[#00f2fe]' : 'text-[#00f2fe]'} />
            <span>Re-verify</span>
          </button>

          <button
            onclick={handleSignOut}
            class="px-4 py-2 rounded-full bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 font-medium text-xs transition-all active:scale-[0.98] flex items-center gap-2"
            title="Log out"
          >
            <Icons name="logout" size={13} />
            <span>Log Out</span>
          </button>
        {:else}
          <button
            onclick={handleGoogleSignIn}
            disabled={isAuthenticating}
            class="px-6 py-2.5 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white font-semibold text-xs shadow-sm transition-all border border-transparent flex items-center gap-2 active:scale-[0.98] disabled:opacity-50"
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
        <div class="p-5 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] shadow-subtle space-y-3.5">
          <div class="flex items-center justify-between pb-2 border-b border-white/[0.08]">
            <div class="flex items-center gap-2">
              <div class="w-7 h-7 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#8b5cf6]">
                <Icons name="google" size={13} />
              </div>
              <h3 class="text-xs font-semibold text-white">OAuth Credentials</h3>
            </div>
            <span class="text-[10px] font-mono px-2.5 py-0.5 rounded-full bg-white/[0.05] text-[#8b5cf6] border border-[#8b5cf6]/30 font-semibold">
              oauth.json
            </span>
          </div>

          <div class="space-y-2.5 text-xs font-mono">
            <div>
              <div class="flex items-center justify-between text-[#94a3b8] text-[11px] mb-1">
                <span>Client ID</span>
                <button
                  onclick={() => handleCopy(clientId, 'Client ID')}
                  class="hover:text-white transition-colors flex items-center gap-1"
                >
                  <Icons name={copiedField === 'Client ID' ? 'check' : 'copy'} size={11} class={copiedField === 'Client ID' ? 'text-[#00f2fe]' : ''} />
                  <span>{copiedField === 'Client ID' ? 'Copied' : 'Copy'}</span>
                </button>
              </div>
              <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08] text-[11px] text-white truncate select-text">
                {clientId}
              </div>
            </div>

            <div>
              <div class="flex items-center justify-between text-[#94a3b8] text-[11px] mb-1">
                <span>Client Secret</span>
                <div class="flex items-center gap-2">
                  <button
                    onclick={() => (showClientSecret = !showClientSecret)}
                    class="hover:text-white transition-colors"
                  >
                    {showClientSecret ? 'Hide' : 'Reveal'}
                  </button>
                  <span class="text-white/20">•</span>
                  <button
                    onclick={() => handleCopy(clientSecret, 'Client Secret')}
                    class="hover:text-white transition-colors flex items-center gap-1"
                  >
                    <Icons name={copiedField === 'Client Secret' ? 'check' : 'copy'} size={11} class={copiedField === 'Client Secret' ? 'text-[#00f2fe]' : ''} />
                    <span>{copiedField === 'Client Secret' ? 'Copied' : 'Copy'}</span>
                  </button>
                </div>
              </div>
              <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08] text-[11px] text-white font-mono select-text">
                {showClientSecret ? clientSecret : '••••••••••••••••••••••••••••••••••••'}
              </div>
            </div>

            <div class="grid grid-cols-2 gap-2 text-[11px] pt-1">
              <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08]">
                <span class="block text-[#94a3b8] text-[10px]">Project ID</span>
                <span class="font-semibold text-white">{projectId}</span>
              </div>
              <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08]">
                <span class="block text-[#94a3b8] text-[10px]">Redirect Mode</span>
                <span class="font-semibold text-white">{redirectMode}</span>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- Storage & Environment Settings Card -->
      <div class="p-5 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] shadow-subtle space-y-3.5">
        <div class="flex items-center justify-between pb-2 border-b border-white/[0.08]">
          <div class="flex items-center gap-2">
            <div class="w-7 h-7 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#00f2fe]">
              <Icons name="shield" size={13} />
            </div>
            <h3 class="text-xs font-semibold text-white">Storage</h3>
          </div>
          <span class="text-[10px] font-mono px-2.5 py-0.5 rounded-full bg-white/[0.05] text-white border border-white/[0.08]">
            Local-First
          </span>
        </div>

        <div class="space-y-2.5 text-xs">
          <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08] flex items-center justify-between">
            <div>
              <span class="block text-white font-medium text-xs">Config File</span>
              <span class="font-mono text-[11px] text-[#94a3b8]">~/.config/tether/config.json</span>
            </div>
            <span class="px-2.5 py-0.5 rounded-full font-mono text-[10px] bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
              Local
            </span>
          </div>

          <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08] flex items-center justify-between">
            <div>
              <span class="block text-white font-medium text-xs">Config Directory</span>
              <span class="font-mono text-[11px] text-[#94a3b8]">~/.config/tether/</span>
            </div>
            <span class="px-2.5 py-0.5 rounded-full font-mono text-[10px] bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
              Directory
            </span>
          </div>

          <div class="p-2.5 rounded-2xl bg-white/[0.04] border border-white/[0.08] flex items-center justify-between">
            <div>
              <span class="block text-white font-medium text-xs">Telemetry</span>
              <span class="text-[11px] text-[#94a3b8]">No external tracking relays</span>
            </div>
            <span class="px-2.5 py-0.5 rounded-full font-mono text-[10px] bg-white/[0.05] text-[#00f2fe] border border-white/[0.08]">
              Disabled
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Non-Admin Notice for Members -->
    {#if profile?.signed_in && !isAdmin}
      <div class="p-4 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] flex items-center gap-3">
        <div class="w-8 h-8 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#94a3b8] shrink-0">
          <Icons name="shield" size={14} />
        </div>
        <div>
          <span class="font-semibold text-white text-xs">Member Access</span>
          <p class="text-[11px] text-[#94a3b8] mt-0.5">
            You are signed in with read-only view. Cloud deployments, target creation, and OAuth credentials are restricted to the administrator.
          </p>
        </div>
      </div>
    {/if}

    <!-- Backup & Export -->
    <div class="p-4 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div>
        <span class="font-semibold text-white text-xs">Configuration Backup</span>
        <p class="text-[11px] text-[#94a3b8] mt-0.5">
          Copy a JSON snapshot of your workspaces, cloud bindings, and settings.
        </p>
      </div>

      <button
        onclick={handleExportBackup}
        class="px-5 py-2 rounded-full bg-white/[0.04] hover:bg-white/[0.08] text-white border border-white/[0.08] hover:border-white/[0.15] font-medium text-xs transition-all active:scale-[0.98] flex items-center gap-2 shrink-0 shadow-sm"
      >
        <Icons name="copy" size={13} class="text-[#00f2fe]" />
        <span>Export Config</span>
      </button>
    </div>
  </div>
</div>
