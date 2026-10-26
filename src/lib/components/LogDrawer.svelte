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
    class="border-t border-white/[0.08] bg-[#0b0d19]/95 backdrop-blur-2xl text-xs font-mono select-none flex flex-col h-56 transition-all duration-200 z-30 shadow-[0_-10px_30px_rgba(0,0,0,0.6)]"
  >
    <!-- Drawer Header -->
    <div class="h-9 px-4 bg-white/[0.03] backdrop-blur-md flex items-center justify-between border-b border-white/[0.08] select-none">
      <div class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe] animate-ping"></span>
        <span class="font-sans font-semibold text-white text-[11px]">Interleaved Log Drawer</span>
        <span class="text-[10px] text-[#94a3b8] font-mono">({logs.length} events)</span>
      </div>

      <div class="flex items-center gap-2">
        <button
          onclick={onNavigateLogs}
          class="text-[11px] text-[#94a3b8] hover:text-[#00f2fe] transition-colors flex items-center gap-1"
        >
          <span>Open Full Terminal</span>
          <Icons name="external-link" size={11} />
        </button>
        <div class="h-3 w-[1px] bg-white/[0.08]"></div>
        <button
          onclick={onClose}
          class="w-6 h-6 rounded-full flex items-center justify-center text-[#94a3b8] hover:text-white hover:bg-white/[0.08] transition-colors"
          title="Close drawer"
        >
          <Icons name="x" size={12} />
        </button>
      </div>
    </div>

    <!-- Live Stream Lines -->
    <div class="flex-1 overflow-y-auto p-3 space-y-1 select-text">
      {#each logs.slice(0, 25) as log (log.id)}
        <div class="flex items-start gap-2 text-[11px] leading-tight hover:bg-white/[0.04] px-2 py-0.5 rounded-lg">
          <span class="text-[#94a3b8] select-none">[{log.timestamp}]</span>
          <span class="text-[#8b5cf6] font-medium select-none">[{log.provider}:{log.serviceName}]</span>
          <span class="text-white break-all">{log.message}</span>
        </div>
      {/each}
    </div>
  </div>
{/if}
