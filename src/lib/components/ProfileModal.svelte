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
    class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#151926] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/80 overflow-hidden text-xs select-none"
    >
      <!-- Header -->
      <div class="px-6 py-5 border-b border-white/[0.08] flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#8b5cf6]">
            <Icons name="google" size={14} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-white">Profile</h3>
            <p class="text-[10px] text-[#94a3b8]">Google OAuth</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="w-7 h-7 rounded-full flex items-center justify-center text-[#94a3b8] hover:text-white hover:bg-white/[0.08] transition-colors"
          aria-label="Close dialog"
        >
          <Icons name="x" size={14} />
        </button>
      </div>

      <!-- Body -->
      <div class="p-6 space-y-4">
        {#if profile && profile.signed_in}
          <!-- User Connected State -->
          <div class="p-4 rounded-3xl bg-white/[0.03] border border-white/[0.08] shadow-subtle flex items-center gap-3.5">
            {#if profile.picture}
              <img
                src={profile.picture}
                alt={profile.name}
                class="w-12 h-12 rounded-full object-cover border border-white/[0.1]"
              />
            {:else}
              <div
                class="w-12 h-12 rounded-full bg-white/[0.05] border border-[#8b5cf6]/40 flex items-center justify-center text-[#8b5cf6] font-bold text-base"
              >
                {profile.name.charAt(0).toUpperCase()}
              </div>
            {/if}

            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-sm font-semibold text-white truncate">{profile.name}</span>
                {#if isAdmin}
                  <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-[#8b5cf6]/20 text-[#8b5cf6] border border-[#8b5cf6]/40 flex items-center gap-1 font-bold">
                    <Icons name="crown" size={10} />
                    Admin
                  </span>
                {:else}
                  <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
                    Member
                  </span>
                {/if}
              </div>
              <p class="text-xs text-[#94a3b8] truncate mt-0.5">{profile.email}</p>
            </div>
          </div>

          <!-- OAuth Details Box -->
          <div class="p-4 rounded-2xl bg-white/[0.03] border border-white/[0.08] space-y-2 text-[11px]">
            <div class="flex items-center justify-between text-[#94a3b8]">
              <span>Auth Provider</span>
              <span class="text-white font-medium">Google Identity (OAuth 2.0)</span>
            </div>
            <div class="flex items-center justify-between text-[#94a3b8]">
              <span>Storage</span>
              <span class="text-white font-mono text-[10px]">~/.config/tether</span>
            </div>
            {#if isAdmin}
              <div class="flex items-center justify-between text-[#94a3b8]">
                <span>OAuth Credentials</span>
                <span class="font-mono text-[10px] text-[#8b5cf6] font-semibold">oauth.json</span>
              </div>
            {/if}
            <div class="flex items-center justify-between text-[#94a3b8]">
              <span>Target Project</span>
              <span class="font-mono text-[10px] text-slate-200">Tether Cloud</span>
            </div>
          </div>

          <!-- Actions Footer -->
          <div class="pt-2 flex items-center justify-end gap-2.5">
            <button
              type="button"
              onclick={onClose}
              class="px-4 py-2 rounded-full text-[#94a3b8] hover:text-white hover:bg-white/[0.06] transition-colors"
            >
              Done
            </button>
            <button
              type="button"
              onclick={handleSignOut}
              class="px-4 py-2 rounded-full bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 font-medium transition-all active:scale-[0.98] flex items-center gap-1.5"
            >
              <Icons name="logout" size={13} />
              <span>Log Out</span>
            </button>
          </div>
        {:else}
          <!-- Sign In State -->
          <div class="text-center py-2">
            <div class="w-14 h-14 rounded-full bg-white/[0.04] border border-white/[0.08] mx-auto flex items-center justify-center text-[#8b5cf6] mb-3">
              <Icons name="google" size={22} />
            </div>
            <h4 class="text-sm font-semibold text-white">Sign in to Tether with Google</h4>
            <p class="text-xs text-[#94a3b8] max-w-xs mx-auto mt-1 leading-relaxed">
              Authenticate via your Google Cloud account to connect your developer profile. Redirects to your browser and listens on loopback.
            </p>
          </div>

          <!-- Action Button -->
          <div class="pt-2 flex flex-col gap-2">
            <button
              type="button"
              onclick={handleGoogleSignIn}
              disabled={isAuthenticating}
              class="w-full py-2.5 px-4 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white font-semibold text-xs shadow-sm flex items-center justify-center gap-2 active:scale-[0.98] disabled:opacity-50 transition-all"
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
              class="w-full py-1.5 text-center text-[#94a3b8] hover:text-white text-xs transition-colors"
            >
              Cancel
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
