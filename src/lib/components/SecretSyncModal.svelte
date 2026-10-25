<script lang="ts">
  import Icons from './Icons.svelte';

  let {
    isOpen = false,
    workspaceId = 'ws-default',
    onClose = () => {},
    onSync = (content: string) => {},
  }: {
    isOpen: boolean;
    workspaceId?: string;
    onClose: () => void;
    onSync: (content: string) => void;
  } = $props();

  let envText = $state(`PORT=8080
DATABASE_URL=postgres://primary.cloud.internal:5432/main
AUTH_SECRET=tether_sec_k92m0x
REDIS_URL=redis://default:token@edge.internal:6379
NODE_ENV=production
DEBUG=false
`);

  let parsedKeys = $derived(
    envText
      .split('\n')
      .map((l) => l.trim())
      .filter((l) => l && !l.startsWith('#') && l.includes('='))
      .map((l) => l.split('=')[0].trim())
  );

  let isSyncing = $state(false);

  async function handleConfirm() {
    isSyncing = true;
    try {
      await onSync(envText);
      onClose();
    } finally {
      isSyncing = false;
    }
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-lg bg-[#242424] border border-border-dim rounded-xl shadow-modal overflow-hidden text-xs select-none"
    >
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-border-dim flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div class="w-6 h-6 rounded-md bg-coral/15 border border-coral/30 flex items-center justify-center">
            <Icons name="key" size={13} class="text-coral" />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-tx-header">Synchronize Environment Secrets</h3>
            <p class="text-[10px] text-tx-meta">Pushes local .env to all linked cloud providers in 1 parallel operation</p>
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

      <!-- Content Area -->
      <div class="p-5 space-y-4">
        <div>
          <div class="flex items-center justify-between mb-1.5">
            <label class="text-tx-meta text-[11px] uppercase font-mono tracking-wider" for="env-editor">
              Local .env Variables
            </label>
            <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-surface-elevated text-tx-header border border-border-dim">
              {parsedKeys.length} secrets detected
            </span>
          </div>

          <textarea
            id="env-editor"
            bind:value={envText}
            rows={7}
            class="w-full bg-[#1c1c1c] text-tx-header font-mono text-[11px] p-3 rounded-lg border border-border-dim focus:border-coral focus:outline-none resize-none leading-relaxed select-text"
            placeholder="KEY=VALUE"
          ></textarea>
        </div>

        <!-- Target Cloud Checkmarks -->
        <div>
          <span class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-2">
            Target Destinations (Concurrent Broadcast)
          </span>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            <div class="flex items-center gap-2 p-2 rounded-lg bg-surface-elevated border border-border-dim">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              <span class="text-tx-header font-medium">Fly.io Secrets</span>
              <span class="text-[10px] text-tx-meta font-mono ml-auto">API</span>
            </div>
            <div class="flex items-center gap-2 p-2 rounded-lg bg-surface-elevated border border-border-dim">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              <span class="text-tx-header font-medium">Cloudflare Workers</span>
              <span class="text-[10px] text-tx-meta font-mono ml-auto">Edge</span>
            </div>
            <div class="flex items-center gap-2 p-2 rounded-lg bg-surface-elevated border border-border-dim">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              <span class="text-tx-header font-medium">Render Web Services</span>
              <span class="text-[10px] text-tx-meta font-mono ml-auto">Container</span>
            </div>
            <div class="flex items-center gap-2 p-2 rounded-lg bg-surface-elevated border border-border-dim">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              <span class="text-tx-header font-medium">Supabase / Neon Env</span>
              <span class="text-[10px] text-tx-meta font-mono ml-auto">DB Pool</span>
            </div>
          </div>
        </div>

        <!-- Footer Actions -->
        <div class="pt-3 border-t border-border-dim flex items-center justify-between">
          <span class="text-[10px] text-tx-meta font-mono">Zero-telemetry • Encrypted memory transfer</span>
          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={onClose}
              class="px-3.5 py-1.5 rounded-lg text-tx-body hover:text-tx-header hover:bg-surface-elevated transition-colors"
            >
              Cancel
            </button>
            <button
              type="button"
              onclick={handleConfirm}
              disabled={isSyncing || parsedKeys.length === 0}
              class="px-4 py-1.5 rounded-lg bg-coral hover:bg-coral-hover text-white font-medium shadow-sm transition-all border border-coral/50 active:scale-[0.98] disabled:opacity-40 flex items-center gap-1.5"
            >
              {#if isSyncing}
                <Icons name="refresh" size={12} class="animate-spin" />
                <span>Broadcasting...</span>
              {:else}
                <Icons name="key" size={12} />
                <span>Push {parsedKeys.length} Secrets Now</span>
              {/if}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}
