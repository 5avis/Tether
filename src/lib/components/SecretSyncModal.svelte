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
    class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-lg bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-3xl shadow-[0_0_35px_rgba(139,92,246,0.25)] overflow-hidden text-xs select-none"
    >
      <!-- Modal Header -->
      <div class="px-6 py-5 border-b border-white/[0.08] flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#8b5cf6] shadow-[0_0_12px_rgba(139,92,246,0.3)]">
            <Icons name="key" size={14} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-white">Synchronize Environment Secrets</h3>
            <p class="text-[10px] text-[#94a3b8]">Pushes local .env to all linked cloud providers in 1 parallel operation</p>
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

      <!-- Content Area -->
      <div class="p-6 space-y-4">
        <div>
          <div class="flex items-center justify-between mb-2">
            <label class="text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider" for="env-editor">
              Local .env Variables
            </label>
            <span class="text-[10px] font-mono px-3 py-1 rounded-full bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
              {parsedKeys.length} secrets detected
            </span>
          </div>

          <textarea
            id="env-editor"
            bind:value={envText}
            rows={7}
            class="w-full bg-white/[0.04] text-white font-mono text-[11px] p-4 rounded-2xl border border-white/[0.08] focus:border-[#00f2fe]/60 focus:outline-none resize-none leading-relaxed select-text transition-colors"
            placeholder="KEY=VALUE"
          ></textarea>
        </div>

        <!-- Target Cloud Checkmarks -->
        <div>
          <span class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2">
            Target Destinations (Concurrent Broadcast)
          </span>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            <div class="flex items-center gap-2.5 p-3 rounded-2xl bg-white/[0.03] border border-white/[0.08]">
              <span class="w-2 h-2 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
              <span class="text-white font-medium">Fly.io Secrets</span>
              <span class="text-[10px] text-[#94a3b8] font-mono ml-auto">API</span>
            </div>
            <div class="flex items-center gap-2.5 p-3 rounded-2xl bg-white/[0.03] border border-white/[0.08]">
              <span class="w-2 h-2 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
              <span class="text-white font-medium">Cloudflare Workers</span>
              <span class="text-[10px] text-[#94a3b8] font-mono ml-auto">Edge</span>
            </div>
            <div class="flex items-center gap-2.5 p-3 rounded-2xl bg-white/[0.03] border border-white/[0.08]">
              <span class="w-2 h-2 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
              <span class="text-white font-medium">Render Web Services</span>
              <span class="text-[10px] text-[#94a3b8] font-mono ml-auto">Container</span>
            </div>
            <div class="flex items-center gap-2.5 p-3 rounded-2xl bg-white/[0.03] border border-white/[0.08]">
              <span class="w-2 h-2 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
              <span class="text-white font-medium">Supabase / Neon Env</span>
              <span class="text-[10px] text-[#94a3b8] font-mono ml-auto">DB Pool</span>
            </div>
          </div>
        </div>

        <!-- Footer Actions -->
        <div class="pt-3 border-t border-white/[0.08] flex items-center justify-between">
          <span class="text-[10px] text-[#94a3b8] font-mono">Zero-telemetry • Encrypted memory transfer</span>
          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={onClose}
              class="px-4 py-2 rounded-full text-[#94a3b8] hover:text-white hover:bg-white/[0.06] transition-colors text-xs font-medium"
            >
              Cancel
            </button>
            <button
              type="button"
              onclick={handleConfirm}
              disabled={isSyncing || parsedKeys.length === 0}
              class="px-5 py-2.5 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-semibold shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all border border-transparent active:scale-[0.98] disabled:opacity-40 flex items-center gap-2 text-xs"
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
