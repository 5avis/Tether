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
    class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-3xl shadow-[0_0_35px_rgba(139,92,246,0.25)] text-xs select-none relative overflow-hidden"
    >
      <!-- Modal Header -->
      <div class="px-6 py-5 border-b border-white/[0.08] flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#8b5cf6] shadow-[0_0_12px_rgba(139,92,246,0.3)]">
            <Icons name="key" size={14} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-white">{actionTitle}</h3>
            <p class="text-[10px] text-[#94a3b8]">
              {service ? service.name : 'Render Service'} • Live Cloud Management
            </p>
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

      <!-- Modal Body -->
      <form onsubmit={handleConnect} class="p-6 space-y-4">
        <!-- Render Cloud Notice -->
        <div class="p-3.5 rounded-2xl bg-white/[0.03] border border-white/[0.08] text-[11px] space-y-2">
          <div class="flex items-center gap-2 text-white font-medium">
            <Icons name="cloud" size={13} class="text-[#00f2fe]" />
            <span>Why is this key required?</span>
          </div>
          <p class="text-[#94a3b8] leading-relaxed">
            A Deploy Hook URL only triggers builds. Controlling service state (<span class="text-amber-400 font-mono">Suspend</span>, <span class="text-emerald-400 font-mono">Resume</span>, <span class="text-white font-mono">Auto-Resume</span>, and <span class="text-white font-mono">Live Sync</span>) requires a Render Personal API Key.
          </p>
          <div class="pt-2 border-t border-white/[0.08] text-[10px] text-[#94a3b8] space-y-1">
            <p class="font-mono text-slate-200">How to get your Render API key:</p>
            <ol class="list-decimal list-inside space-y-0.5 text-[#94a3b8] font-mono">
              <li>Log in to <span class="text-white">dashboard.render.com</span></li>
              <li>Click your avatar (top right) → <span class="text-white">Account Settings</span></li>
              <li>Select <span class="text-white">API Keys</span> → <span class="text-[#00f2fe]">Create API Key</span></li>
              <li>Copy the key starting with <span class="text-[#00f2fe] font-bold">rnd_</span> and paste it below.</li>
            </ol>
          </div>
        </div>

        <!-- Token Input -->
        <div>
          <label
            class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2"
            for="render-api-key"
          >
            Render API Key (<span class="font-mono text-[#00f2fe] lowercase">rnd_...</span>)
          </label>
          <input
            id="render-api-key"
            type="password"
            bind:value={apiKey}
            placeholder="rnd_xxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            required
            class="w-full bg-white/[0.04] text-white px-4 py-2.5 rounded-full border border-white/[0.08] focus:border-[#00f2fe]/60 focus:outline-none placeholder:text-[#94a3b8] font-mono text-xs transition-colors"
          />
        </div>

        {#if errorMsg}
          <div class="p-3 rounded-2xl bg-rose-500/10 border border-rose-500/30 text-rose-400 text-[11px] flex items-center gap-2">
            <Icons name="shield" size={13} class="shrink-0" />
            <span>{errorMsg}</span>
          </div>
        {/if}

        <!-- Actions Footer -->
        <div class="pt-3 border-t border-white/[0.08] flex items-center justify-end gap-2.5">
          <button
            type="button"
            onclick={onClose}
            class="px-4 py-2 rounded-full text-[#94a3b8] hover:text-white hover:bg-white/[0.06] transition-colors text-xs font-medium"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isSaving || !apiKey}
            class="px-5 py-2.5 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-semibold shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all border border-transparent active:scale-[0.98] disabled:opacity-40 flex items-center gap-1.5 text-xs"
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
