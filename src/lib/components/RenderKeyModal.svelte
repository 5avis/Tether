<script lang="ts">
  import Icons from './Icons.svelte';
  import type { ServiceTarget } from '$lib/types';
  import { saveProviderCredential, verifyProviderToken } from '$lib/storage';

  let {
    isOpen = false,
    service = null,
    action = 'settings',
    onClose = () => {},
    onSuccess = (token: string) => {},
    onToast = (msg: string) => {},
  }: {
    isOpen: boolean;
    service: ServiceTarget | null;
    action: 'suspend' | 'resume' | 'deploy' | 'settings';
    onClose: () => void;
    onSuccess: (token: string) => void;
    onToast?: (msg: string) => void;
  } = $props();

  let apiKey = $state('');
  let isSaving = $state(false);
  let errorMsg = $state<string | null>(null);

  const actionTitle = $derived.by(() => {
    switch (action) {
      case 'suspend':
        return 'Connect Render Key to Suspend';
      case 'resume':
        return 'Connect Render Key to Resume';
      case 'deploy':
        return 'Connect Render Key for Auto-Resume & Deploy';
      default:
        return 'Connect Render API Key';
    }
  });

  const buttonLabel = $derived.by(() => {
    if (isSaving) return 'Connecting...';
    switch (action) {
      case 'suspend':
        return 'Save & Suspend Service';
      case 'resume':
        return 'Save & Resume Service';
      case 'deploy':
        return 'Save & Deploy Service';
      default:
        return 'Save & Connect';
    }
  });

  async function handleConnect(e: Event) {
    e.preventDefault();
    errorMsg = null;
    const trimmed = apiKey.trim();

    if (!trimmed) {
      errorMsg = 'Please enter your Render API Key.';
      return;
    }

    if (!trimmed.startsWith('rnd_')) {
      errorMsg = "Render API Key must start with 'rnd_'. Find it in Account Settings -> API Keys.";
      return;
    }

    isSaving = true;
    try {
      // 1. Verify token with Rust backend
      const verifyRes = await verifyProviderToken('render', trimmed);
      if (!verifyRes.valid) {
        errorMsg = verifyRes.message || 'Render token verification failed.';
        isSaving = false;
        return;
      }

      // 2. Save securely to Tether configuration
      await saveProviderCredential('render', trimmed);
      onToast('Render API Key saved securely.');

      // 3. Trigger callback with token so pending action executes immediately
      onSuccess(trimmed);
      apiKey = '';
      onClose();
    } catch (err: any) {
      errorMsg = String(err?.message || err || 'Failed to save Render key.');
    } finally {
      isSaving = false;
    }
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#161326] border border-border-dim rounded-2xl shadow-modal text-xs select-none relative overflow-hidden"
    >
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-border-dim flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="w-7 h-7 rounded-lg bg-coral/15 border border-coral/30 flex items-center justify-center text-coral">
            <Icons name="key" size={14} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-tx-header">{actionTitle}</h3>
            <p class="text-[10px] text-tx-meta">
              {service ? service.name : 'Render Service'} • Live Cloud Management
            </p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-colors"
          aria-label="Close dialog"
        >
          <Icons name="x" size={14} />
        </button>
      </div>

      <!-- Modal Body -->
      <form onsubmit={handleConnect} class="p-5 space-y-4">
        <!-- Render Cloud Notice -->
        <div class="p-3 rounded-lg bg-surface/60 border border-border-dim text-[11px] space-y-2">
          <div class="flex items-center gap-2 text-tx-header font-medium">
            <Icons name="cloud" size={13} class="text-coral" />
            <span>Why is this key required?</span>
          </div>
          <p class="text-tx-meta leading-relaxed">
            A Deploy Hook URL only triggers builds. Controlling service state (<span class="text-amber-400 font-mono">Suspend</span>, <span class="text-emerald-400 font-mono">Resume</span>, <span class="text-tx-header font-mono">Auto-Resume</span>, and <span class="text-tx-header font-mono">Live Sync</span>) requires a Render Personal API Key.
          </p>
          <div class="pt-1 border-t border-border-dim/60 text-[10px] text-tx-meta space-y-1">
            <p class="font-mono text-tx-body">How to get your Render API key:</p>
            <ol class="list-decimal list-inside space-y-0.5 text-tx-meta font-mono">
              <li>Log in to <span class="text-tx-header">dashboard.render.com</span></li>
              <li>Click your avatar (top right) → <span class="text-tx-header">Account Settings</span></li>
              <li>Select <span class="text-tx-header">API Keys</span> → <span class="text-coral">Create API Key</span></li>
              <li>Copy the key starting with <span class="text-coral font-bold">rnd_</span> and paste it below.</li>
            </ol>
          </div>
        </div>

        <!-- Token Input -->
        <div>
          <label
            class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5"
            for="render-api-key"
          >
            Render API Key (<span class="font-mono text-coral lowercase">rnd_...</span>)
          </label>
          <input
            id="render-api-key"
            type="password"
            bind:value={apiKey}
            placeholder="rnd_xxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            required
            class="w-full bg-surface-elevated text-tx-header px-3 py-2 rounded-lg border border-border-dim focus:border-coral focus:outline-none placeholder:text-tx-meta font-mono text-xs"
          />
        </div>

        {#if errorMsg}
          <div class="p-2.5 rounded-xl bg-red-500/10 border border-red-500/30 text-red-400 text-[11px] flex items-center gap-2">
            <Icons name="shield" size={13} class="shrink-0" />
            <span>{errorMsg}</span>
          </div>
        {/if}

        <!-- Actions Footer -->
        <div class="pt-3 border-t border-border-dim flex items-center justify-end gap-2.5">
          <button
            type="button"
            onclick={onClose}
            class="px-4 py-2 rounded-xl text-tx-body hover:text-tx-header hover:bg-surface-elevated transition-colors text-xs font-medium"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isSaving || !apiKey}
            class="px-4 py-2 rounded-xl bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white font-medium shadow-sm transition-all border border-[#c084fc]/30 active:scale-[0.98] disabled:opacity-40 flex items-center gap-1.5 text-xs shadow-[0_0_12px_rgba(168,85,247,0.25)]"
          >
            {#if isSaving}
              <Icons name="refresh" size={12} class="animate-spin" />
            {:else}
              <Icons name="check" size={12} strokeWidth={2.5} />
            {/if}
            <span>{buttonLabel}</span>
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
