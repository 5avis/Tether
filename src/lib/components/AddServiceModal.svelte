<script lang="ts">
  import type { CloudProvider, Environment } from '$lib/types';
  import { verifyProviderToken } from '$lib/storage';
  import Icons from './Icons.svelte';

  let {
    isOpen = false,
    workspaceId = 'ws-default',
    onClose = () => {},
    onSave = (data: any) => {},
  }: {
    isOpen: boolean;
    workspaceId?: string;
    onClose: () => void;
    onSave: (data: any) => void;
  } = $props();

  let name = $state('');
  let provider = $state<CloudProvider>('cloudflare');
  let environment = $state<Environment>('production');
  let targetType = $state<'api' | 'frontend' | 'database' | 'worker'>('api');
  let localPort = $state(3000);
  let apiToken = $state('');
  let verifying = $state(false);
  let verifyResult = $state<{ valid: boolean; message: string; details?: string } | null>(null);
  let openDropdown = $state<'provider' | 'environment' | null>(null);

  const providerOptions: Array<{ value: CloudProvider; label: string }> = [
    { value: 'cloudflare', label: 'Cloudflare Tunnel' },
    { value: 'flyio', label: 'Fly.io' },
    { value: 'supabase', label: 'Supabase' },
    { value: 'neon', label: 'Neon Postgres' },
    { value: 'render', label: 'Render' },
    { value: 'vercel', label: 'Vercel' },
    { value: 'railway', label: 'Railway' },
  ];

  const environmentOptions: Array<{ value: Environment; label: string }> = [
    { value: 'production', label: 'Production' },
    { value: 'staging', label: 'Staging' },
    { value: 'preview', label: 'Preview' },
    { value: 'development', label: 'Development' },
  ];

  let currentProviderLabel = $derived(
    providerOptions.find((p) => p.value === provider)?.label ?? 'Cloudflare Tunnel'
  );
  let currentEnvironmentLabel = $derived(
    environmentOptions.find((e) => e.value === environment)?.label ?? 'Production'
  );

  // Update targetType automatically when provider changes
  $effect(() => {
    if (provider === 'neon' || provider === 'supabase') {
      targetType = 'database';
    } else if (provider === 'cloudflare' || provider === 'vercel') {
      targetType = 'frontend';
    } else {
      targetType = 'api';
    }
    verifyResult = null;
  });

  async function handleVerify() {
    if (!apiToken) return;
    verifying = true;
    verifyResult = null;
    try {
      const res = await verifyProviderToken(provider, apiToken);
      verifyResult = {
        valid: res.valid,
        message: res.message,
        details: res.details,
      };
    } catch (e: any) {
      verifyResult = {
        valid: false,
        message: e?.message || 'Verification failed',
      };
    } finally {
      verifying = false;
    }
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name) return;
    onSave({
      name,
      workspaceId,
      provider,
      environment,
      targetType,
      tunnelPort: provider === 'cloudflare' ? Number(localPort) : undefined,
      apiToken,
    });
    // Reset form
    name = '';
    apiToken = '';
    verifyResult = null;
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
      class="w-full max-w-md bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-3xl shadow-[0_0_35px_rgba(139,92,246,0.25)] text-xs select-none relative overflow-hidden"
    >
      <!-- Modal Header -->
      <div class="px-6 py-5 border-b border-white/[0.08] flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center shadow-[0_0_12px_rgba(139,92,246,0.3)]">
            <Icons name="plus" size={14} class="text-[#8b5cf6]" strokeWidth={2.5} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-white">Add Target</h3>
            <p class="text-[10px] text-[#94a3b8]">Configure new service binding</p>
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

      <!-- Modal Body Form -->
      <form onsubmit={handleSubmit} class="p-6 space-y-4">
        <div>
          <label class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2" for="service-name">
            Target Service Name
          </label>
          <input
            id="service-name"
            type="text"
            bind:value={name}
            placeholder="e.g. auth-gateway, billing-db, edge-worker"
            required
            class="w-full bg-white/[0.04] text-white px-4 py-2.5 rounded-full border border-white/[0.08] focus:border-[#00f2fe]/60 focus:outline-none placeholder:text-[#94a3b8] transition-colors"
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <!-- Custom Cloud Provider Dropdown -->
          <div class="relative">
            <span class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2">
              Cloud Provider
            </span>
            <button
              id="provider-select"
              type="button"
              onclick={() => (openDropdown = openDropdown === 'provider' ? null : 'provider')}
              class="w-full flex items-center justify-between bg-white/[0.04] text-white px-4 py-2.5 rounded-full border border-white/[0.08] hover:border-white/[0.15] focus:border-[#00f2fe]/60 transition-colors cursor-pointer text-xs"
            >
              <span class="truncate">{currentProviderLabel}</span>
              <Icons name="chevron-down" size={11} class="text-[#94a3b8] shrink-0 ml-1 transition-transform {openDropdown === 'provider' ? 'rotate-180' : ''}" />
            </button>

            {#if openDropdown === 'provider'}
              <button
                type="button"
                class="fixed inset-0 z-40 cursor-default bg-transparent"
                onclick={() => (openDropdown = null)}
                aria-label="Close menu"
              ></button>

              <div class="absolute top-full left-0 right-0 mt-1.5 bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-2xl shadow-[0_0_30px_rgba(139,92,246,0.25)] p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
                {#each providerOptions as opt}
                  <button
                    type="button"
                    onclick={() => {
                      provider = opt.value;
                      openDropdown = null;
                    }}
                    class="w-full flex items-center justify-between px-3 py-1.5 rounded-xl text-left transition-colors {provider === opt.value ? 'bg-white/[0.08] text-white font-medium' : 'text-[#94a3b8] hover:text-white hover:bg-white/[0.04]'}"
                  >
                    <span>{opt.label}</span>
                    {#if provider === opt.value}
                      <Icons name="check" size={12} class="text-[#00f2fe]" />
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Custom Environment Dropdown -->
          <div class="relative">
            <span class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2">
              Environment
            </span>
            <button
              id="env-select"
              type="button"
              onclick={() => (openDropdown = openDropdown === 'environment' ? null : 'environment')}
              class="w-full flex items-center justify-between bg-white/[0.04] text-white px-4 py-2.5 rounded-full border border-white/[0.08] hover:border-white/[0.15] focus:border-[#00f2fe]/60 transition-colors cursor-pointer text-xs"
            >
              <span class="truncate">{currentEnvironmentLabel}</span>
              <Icons name="chevron-down" size={11} class="text-[#94a3b8] shrink-0 ml-1 transition-transform {openDropdown === 'environment' ? 'rotate-180' : ''}" />
            </button>

            {#if openDropdown === 'environment'}
              <button
                type="button"
                class="fixed inset-0 z-40 cursor-default bg-transparent"
                onclick={() => (openDropdown = null)}
                aria-label="Close menu"
              ></button>

              <div class="absolute top-full left-0 right-0 mt-1.5 bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-2xl shadow-[0_0_30px_rgba(139,92,246,0.25)] p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
                {#each environmentOptions as opt}
                  <button
                    type="button"
                    onclick={() => {
                      environment = opt.value;
                      openDropdown = null;
                    }}
                    class="w-full flex items-center justify-between px-3 py-1.5 rounded-xl text-left transition-colors {environment === opt.value ? 'bg-white/[0.08] text-white font-medium' : 'text-[#94a3b8] hover:text-white hover:bg-white/[0.04]'}"
                  >
                    <span>{opt.label}</span>
                    {#if environment === opt.value}
                      <Icons name="check" size={12} class="text-[#00f2fe]" />
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        </div>

        <!-- Port or Token Configuration -->
        {#if provider === 'cloudflare'}
          <div>
            <label class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2" for="tunnel-port">
              Local Port to Expose (Quick Tunnel)
            </label>
            <div class="flex items-center gap-2">
              <span class="text-[#94a3b8] font-mono text-xs">http://localhost:</span>
              <input
                id="tunnel-port"
                type="number"
                bind:value={localPort}
                placeholder="3000"
                min="1024"
                max="65535"
                class="flex-1 bg-white/[0.04] text-white font-mono px-4 py-2.5 rounded-full border border-white/[0.08] focus:border-[#00f2fe]/60 focus:outline-none transition-colors"
              />
            </div>
            <p class="text-[10px] text-[#94a3b8] mt-1.5">
              Zero configuration required. Generates an instant trycloudflare.com preview URL.
            </p>
          </div>
        {:else}
          <div>
            <label class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2" for="api-token">
              API Token or Deploy Hook URL
            </label>
            <div class="flex items-center gap-2">
              <input
                id="api-token"
                type="password"
                bind:value={apiToken}
                placeholder="Paste API token, secret key, or Deploy Hook URL..."
                class="flex-1 bg-white/[0.04] text-white px-4 py-2.5 rounded-full border border-white/[0.08] focus:border-[#00f2fe]/60 focus:outline-none placeholder:text-[#94a3b8] font-mono transition-colors"
              />
              <button
                type="button"
                onclick={handleVerify}
                disabled={verifying || !apiToken}
                class="px-4 py-2.5 rounded-full border border-white/[0.08] hover:border-white/[0.15] bg-white/[0.04] hover:bg-white/[0.08] text-white disabled:opacity-40 transition-colors flex items-center gap-1.5 shrink-0"
              >
                {#if verifying}
                  <Icons name="refresh" size={12} class="animate-spin text-[#00f2fe]" />
                  <span>Checking...</span>
                {:else}
                  <Icons name="shield" size={12} class="text-[#00f2fe]" />
                  <span>Verify</span>
                {/if}
              </button>
            </div>

            {#if provider === 'render'}
              <p class="text-[10px] text-[#94a3b8] mt-1.5">
                Tip: Enter a Render API Key (<span class="text-[#00f2fe] font-mono">rnd_...</span>) for full control (Suspend, Resume, Auto-Deploy, Live Sync), or a Deploy Hook URL for deploy-only.
              </p>
            {:else if provider === 'vercel'}
              <p class="text-[10px] text-[#94a3b8] mt-1.5">
                Tip: Enter a Vercel Deploy Hook URL or Personal Access Token (<span class="text-[#00f2fe] font-mono">vercel_...</span>) to link your edge project.
              </p>
            {/if}

            <!-- Inline Verification Feedback -->
            {#if verifyResult}
              <div
                class="mt-2.5 p-3 rounded-2xl border text-[11px] flex items-center justify-between {verifyResult.valid
                  ? 'bg-white/[0.04] text-white border-white/[0.1]'
                  : 'bg-rose-500/10 text-rose-400 border-rose-500/30'}"
              >
                <div class="flex items-center gap-2">
                  <span class="w-2 h-2 rounded-full {verifyResult.valid ? 'bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]' : 'bg-rose-400 animate-ping'}"></span>
                  <span class="font-medium">{verifyResult.message}</span>
                </div>
                {#if verifyResult.details}
                  <span class="font-mono text-[10px] text-[#94a3b8]">{verifyResult.details}</span>
                {/if}
              </div>
            {/if}
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
            class="px-5 py-2.5 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-semibold shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all border border-transparent active:scale-[0.98] text-xs"
          >
            Add Target
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
