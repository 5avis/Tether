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
    } else if (provider === 'cloudflare') {
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
    class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#242424] border border-border-dim rounded-xl shadow-modal text-xs select-none relative"
    >
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-border-dim flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="w-6 h-6 rounded-md bg-coral/15 border border-coral/30 flex items-center justify-center">
            <Icons name="plus" size={13} class="text-coral" strokeWidth={2.5} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-tx-header">Add Target</h3>
            <p class="text-[10px] text-tx-meta">Configure new service</p>
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

      <!-- Modal Body Form -->
      <form onsubmit={handleSubmit} class="p-5 space-y-4">
        <div>
          <label class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5" for="service-name">
            Target Service Name
          </label>
          <input
            id="service-name"
            type="text"
            bind:value={name}
            placeholder="e.g. auth-gateway, billing-db, edge-worker"
            required
            class="w-full bg-surface-elevated text-tx-header px-3 py-2 rounded-lg border border-border-dim focus:border-coral focus:outline-none placeholder:text-tx-meta"
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <!-- Custom Cloud Provider Dropdown -->
          <div class="relative">
            <span class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5">
              Cloud Provider
            </span>
            <button
              id="provider-select"
              type="button"
              onclick={() => (openDropdown = openDropdown === 'provider' ? null : 'provider')}
              class="w-full flex items-center justify-between bg-surface-elevated text-tx-header px-3 py-2 rounded-lg border border-border-dim hover:border-border-subtle focus:border-coral transition-colors cursor-pointer text-xs"
            >
              <span class="truncate">{currentProviderLabel}</span>
              <Icons name="chevron-down" size={11} class="text-tx-meta shrink-0 ml-1 transition-transform {openDropdown === 'provider' ? 'rotate-180' : ''}" />
            </button>

            {#if openDropdown === 'provider'}
              <button
                type="button"
                class="fixed inset-0 z-40 cursor-default bg-transparent"
                onclick={() => (openDropdown = null)}
                aria-label="Close menu"
              ></button>

              <div class="absolute top-full left-0 right-0 mt-1.5 bg-[#242424] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
                {#each providerOptions as opt}
                  <button
                    type="button"
                    onclick={() => {
                      provider = opt.value;
                      openDropdown = null;
                    }}
                    class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {provider === opt.value ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
                  >
                    <span>{opt.label}</span>
                    {#if provider === opt.value}
                      <Icons name="check" size={12} class="text-coral" />
                    {/if}
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Custom Environment Dropdown -->
          <div class="relative">
            <span class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5">
              Environment
            </span>
            <button
              id="env-select"
              type="button"
              onclick={() => (openDropdown = openDropdown === 'environment' ? null : 'environment')}
              class="w-full flex items-center justify-between bg-surface-elevated text-tx-header px-3 py-2 rounded-lg border border-border-dim hover:border-border-subtle focus:border-coral transition-colors cursor-pointer text-xs"
            >
              <span class="truncate">{currentEnvironmentLabel}</span>
              <Icons name="chevron-down" size={11} class="text-tx-meta shrink-0 ml-1 transition-transform {openDropdown === 'environment' ? 'rotate-180' : ''}" />
            </button>

            {#if openDropdown === 'environment'}
              <button
                type="button"
                class="fixed inset-0 z-40 cursor-default bg-transparent"
                onclick={() => (openDropdown = null)}
                aria-label="Close menu"
              ></button>

              <div class="absolute top-full left-0 right-0 mt-1.5 bg-[#242424] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
                {#each environmentOptions as opt}
                  <button
                    type="button"
                    onclick={() => {
                      environment = opt.value;
                      openDropdown = null;
                    }}
                    class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {environment === opt.value ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
                  >
                    <span>{opt.label}</span>
                    {#if environment === opt.value}
                      <Icons name="check" size={12} class="text-coral" />
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
            <label class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5" for="tunnel-port">
              Local Port to Expose (Quick Tunnel)
            </label>
            <div class="flex items-center gap-2">
              <span class="text-tx-meta font-mono">http://localhost:</span>
              <input
                id="tunnel-port"
                type="number"
                bind:value={localPort}
                placeholder="3000"
                min="1024"
                max="65535"
                class="flex-1 bg-surface-elevated text-tx-header font-mono px-3 py-2 rounded-lg border border-border-dim focus:border-coral focus:outline-none"
              />
            </div>
            <p class="text-[10px] text-tx-meta mt-1">
              Zero configuration required. Generates an instant trycloudflare.com preview URL.
            </p>
          </div>
        {:else}
          <div>
            <label class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5" for="api-token">
              Provider API Token / Secret
            </label>
            <div class="flex items-center gap-2">
              <input
                id="api-token"
                type="password"
                bind:value={apiToken}
                placeholder="Paste API token or service key..."
                class="flex-1 bg-surface-elevated text-tx-header px-3 py-2 rounded-lg border border-border-dim focus:border-coral focus:outline-none placeholder:text-tx-meta font-mono"
              />
              <button
                type="button"
                onclick={handleVerify}
                disabled={verifying || !apiToken}
                class="px-3 py-2 rounded-lg border border-border-dim hover:border-border-subtle bg-surface hover:bg-surface-elevated text-tx-header disabled:opacity-40 transition-colors flex items-center gap-1.5 shrink-0"
              >
                {#if verifying}
                  <Icons name="refresh" size={12} class="animate-spin text-coral" />
                  <span>Checking...</span>
                {:else}
                  <Icons name="shield" size={12} />
                  <span>Verify</span>
                {/if}
              </button>
            </div>

            <!-- Inline Verification Feedback -->
            {#if verifyResult}
              <div
                class="mt-2 p-2.5 rounded-lg border text-[11px] flex items-center justify-between {verifyResult.valid
                  ? 'bg-surface-elevated text-tx-header border-border-dim'
                  : 'bg-[#291b1b] text-coral border-coral/40'}"
              >
                <div class="flex items-center gap-2">
                  <span class="w-1.5 h-1.5 rounded-full {verifyResult.valid ? 'bg-coral' : 'bg-coral animate-ping'}"></span>
                  <span class="font-medium">{verifyResult.message}</span>
                </div>
                {#if verifyResult.details}
                  <span class="font-mono text-[10px] text-tx-meta">{verifyResult.details}</span>
                {/if}
              </div>
            {/if}
          </div>
        {/if}

        <!-- Actions Footer -->
        <div class="pt-3 border-t border-border-dim flex items-center justify-end gap-2.5">
          <button
            type="button"
            onclick={onClose}
            class="px-4 py-2 rounded-lg text-tx-body hover:text-tx-header hover:bg-surface-elevated transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="px-4 py-2 rounded-lg bg-coral hover:bg-coral-hover text-white font-medium shadow-sm transition-all border border-coral/50 active:scale-[0.98]"
          >
            Add Target
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
