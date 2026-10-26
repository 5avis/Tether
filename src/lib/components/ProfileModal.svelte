<script lang="ts">
  import { type UserProfile, isAdminUser } from '$lib/types';
  import { startGoogleOAuth, signOutProfile } from '$lib/storage';
  import Icons from './Icons.svelte';

  let {
    isOpen = false,
    profile = null,
    onClose = () => {},
    onProfileUpdated = (p: UserProfile | null) => {},
    onToast = (msg: string) => {},
  }: {
    isOpen: boolean;
    profile: UserProfile | null;
    onClose: () => void;
    onProfileUpdated: (p: UserProfile | null) => void;
    onToast: (msg: string) => void;
  } = $props();

  let isAuthenticating = $state(false);

  const isAdmin = $derived(isAdminUser(profile));

  async function handleGoogleSignIn() {
    isAuthenticating = true;
    onToast('Opening default browser for Google OAuth sign-in...');
    try {
      const user = await startGoogleOAuth();
      onProfileUpdated(user);
      onToast(`Successfully authenticated as ${user.name} (${user.email})`);
      onClose();
    } catch (e: any) {
      onToast(`Google OAuth error: ${e?.message || e}`);
    } finally {
      isAuthenticating = false;
    }
  }

  async function handleSignOut() {
    await signOutProfile();
    onProfileUpdated(null);
    onToast('Signed out from Google account.');
    onClose();
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#18181f] border border-border-dim rounded-2xl shadow-modal overflow-hidden text-xs select-none"
    >
      <!-- Header -->
      <div class="px-5 py-4 border-b border-border-dim flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="w-6 h-6 rounded-md bg-coral/15 border border-coral/30 flex items-center justify-center text-coral">
            <Icons name="google" size={13} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-tx-header">Profile</h3>
            <p class="text-[10px] text-tx-meta">Google OAuth</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded-lg text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-colors"
          aria-label="Close dialog"
        >
          <Icons name="x" size={14} />
        </button>
      </div>

      <!-- Body -->
      <div class="p-5 space-y-4">
        {#if profile && profile.signed_in}
          <!-- User Connected State -->
          <div class="p-4 rounded-xl bg-surface border border-border-dim shadow-subtle flex items-center gap-3.5">
            {#if profile.picture}
              <img
                src={profile.picture}
                alt={profile.name}
                class="w-12 h-12 rounded-xl object-cover border border-border-dim"
              />
            {:else}
              <div
                class="w-12 h-12 rounded-xl bg-coral/15 border border-coral/40 flex items-center justify-center text-coral font-bold text-base"
              >
                {profile.name.charAt(0).toUpperCase()}
              </div>
            {/if}

            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-sm font-semibold text-tx-header truncate">{profile.name}</span>
                {#if isAdmin}
                  <span class="px-2 py-0.2 rounded-full text-[10px] font-mono bg-coral/20 text-coral border border-coral/40 flex items-center gap-1 font-bold">
                    <Icons name="crown" size={10} />
                    Admin
                  </span>
                {:else}
                  <span class="px-2 py-0.2 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-body border border-border-dim">
                    Member
                  </span>
                {/if}
              </div>
              <p class="text-xs text-tx-meta truncate mt-0.5">{profile.email}</p>
            </div>
          </div>

          <!-- OAuth Details Box -->
          <div class="p-3.5 rounded-xl bg-surface/50 border border-border-dim/70 space-y-2 text-[11px]">
            <div class="flex items-center justify-between text-tx-meta">
              <span>Auth Provider</span>
              <span class="text-tx-header font-medium">Google Identity (OAuth 2.0)</span>
            </div>
            <div class="flex items-center justify-between text-tx-meta">
              <span>Storage</span>
              <span class="text-tx-header font-mono text-[10px]">~/.config/tether</span>
            </div>
            {#if isAdmin}
              <div class="flex items-center justify-between text-tx-meta">
                <span>OAuth Credentials</span>
                <span class="font-mono text-[10px] text-coral font-semibold">oauth.json</span>
              </div>
            {/if}
            <div class="flex items-center justify-between text-tx-meta">
              <span>Target Project</span>
              <span class="font-mono text-[10px] text-tx-body">Tether Cloud</span>
            </div>
          </div>

          <!-- Actions Footer -->
          <div class="pt-2 flex items-center justify-end gap-2.5">
            <button
              type="button"
              onclick={onClose}
              class="px-4 py-2 rounded-xl text-tx-body hover:text-tx-header hover:bg-surface-elevated transition-colors"
            >
              Done
            </button>
            <button
              type="button"
              onclick={handleSignOut}
              class="px-4 py-2 rounded-xl bg-red-950/20 hover:bg-red-950/50 text-red-400 border border-red-900/40 hover:border-red-600/50 font-medium transition-all active:scale-[0.98] flex items-center gap-1.5"
            >
              <Icons name="logout" size={13} />
              <span>Log Out</span>
            </button>
          </div>
        {:else}
          <!-- Sign In State -->
          <div class="text-center py-2">
            <div class="w-12 h-12 rounded-2xl bg-surface-elevated border border-border-dim mx-auto flex items-center justify-center text-coral shadow-subtle mb-3">
              <Icons name="google" size={22} />
            </div>
            <h4 class="text-sm font-semibold text-tx-header">Sign in to Tether with Google</h4>
            <p class="text-xs text-tx-meta max-w-xs mx-auto mt-1 leading-relaxed">
              Authenticate via your Google Cloud account to connect your developer profile. Redirects to your browser and listens on loopback.
            </p>
          </div>

          <!-- Action Button -->
          <div class="pt-2 flex flex-col gap-2">
            <button
              type="button"
              onclick={handleGoogleSignIn}
              disabled={isAuthenticating}
              class="w-full py-2.5 px-4 rounded-xl bg-coral hover:bg-coral-hover text-white font-medium text-xs shadow-sm transition-all border border-coral/60 flex items-center justify-center gap-2 active:scale-[0.98] disabled:opacity-50"
            >
              {#if isAuthenticating}
                <Icons name="refresh" size={13} class="animate-spin" />
                <span>Awaiting browser sign-in...</span>
              {:else}
                <Icons name="google" size={13} />
                <span>Continue with Google</span>
              {/if}
            </button>

            <button
              type="button"
              onclick={onClose}
              disabled={isAuthenticating}
              class="w-full py-1.5 text-center text-tx-meta hover:text-tx-header text-xs transition-colors"
            >
              Cancel
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
