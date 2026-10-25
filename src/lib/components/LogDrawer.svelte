<script lang="ts">
  import Icons from './Icons.svelte';
  import type { LogEntry } from './LiveLogsView.svelte';

  let {
    isOpen = false,
    logs = [],
    onClose = () => {},
    onNavigateLogs = () => {},
  }: {
    isOpen: boolean;
    logs: LogEntry[];
    onClose: () => void;
    onNavigateLogs: () => void;
  } = $props();
</script>

{#if isOpen}
  <div
    class="border-t border-border-dim bg-[#181818] text-xs font-mono select-none flex flex-col h-56 transition-all duration-200 z-30 shadow-modal"
  >
    <!-- Drawer Header -->
    <div class="h-8 px-4 bg-surface flex items-center justify-between border-b border-border-dim select-none">
      <div class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-coral animate-ping"></span>
        <span class="font-sans font-semibold text-tx-header text-[11px]">Interleaved Log Drawer</span>
        <span class="text-[10px] text-tx-meta font-mono">({logs.length} events)</span>
      </div>

      <div class="flex items-center gap-2">
        <button
          onclick={onNavigateLogs}
          class="text-[11px] text-tx-meta hover:text-coral transition-colors flex items-center gap-1"
        >
          <span>Open Full Terminal</span>
          <Icons name="external-link" size={11} />
        </button>
        <div class="h-3 w-[1px] bg-border-dim"></div>
        <button
          onclick={onClose}
          class="p-1 rounded text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-colors"
          title="Close drawer"
        >
          <Icons name="x" size={12} />
        </button>
      </div>
    </div>

    <!-- Live Stream Lines -->
    <div class="flex-1 overflow-y-auto p-3 space-y-1 select-text">
      {#each logs.slice(0, 25) as log (log.id)}
        <div class="flex items-start gap-2 text-[11px] leading-tight hover:bg-surface-elevated/40 px-1 py-0.5 rounded">
          <span class="text-tx-meta select-none">[{log.timestamp}]</span>
          <span class="text-coral font-medium select-none">[{log.provider}:{log.serviceName}]</span>
          <span class="text-tx-header break-all">{log.message}</span>
        </div>
      {/each}
    </div>
  </div>
{/if}
