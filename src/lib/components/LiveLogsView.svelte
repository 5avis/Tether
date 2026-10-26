<script lang="ts">
  import Icons from './Icons.svelte';

  export interface LogEntry {
    id: string;
    timestamp: string;
    tag: string;
    serviceName: string;
    provider: string;
    level: string;
    message: string;
    color: string;
  }

  let {
    logs = [],
    onClear = () => {},
    onCopy = () => {},
  }: {
    logs: LogEntry[];
    onClear: () => void;
    onCopy: () => void;
  } = $props();

  let selectedFilter = $state('all');
  let filterSearch = $state('');
  let autoScroll = $state(true);
  let isFilterOpen = $state(false);

  const providerFilterOptions = [
    { value: 'all', label: 'All Providers' },
    { value: 'cloudflare', label: 'Cloudflare Quick Tunnels' },
    { value: 'flyio', label: 'Fly.io Containers' },
    { value: 'neon', label: 'Neon Serverless' },
    { value: 'supabase', label: 'Supabase Postgres' },
    { value: 'render', label: 'Render Web' },
  ];

  const filteredLogs = $derived(
    logs.filter((log) => {
      if (selectedFilter !== 'all' && log.provider !== selectedFilter) {
        return false;
      }
      if (filterSearch) {
        const q = filterSearch.toLowerCase();
        return log.message.toLowerCase().includes(q) || log.serviceName.toLowerCase().includes(q);
      }
      return true;
    })
  );
</script>

<div class="flex-1 flex flex-col min-h-0 bg-[#0b0d19] font-mono text-xs select-none">
  <!-- Top Log Toolbar -->
  <div class="min-h-[48px] py-2 px-4 border-b border-white/[0.08] bg-[#0e111a] flex flex-wrap items-center justify-between gap-2.5 text-xs">
    <div class="flex items-center gap-3 flex-wrap">
      <div class="flex items-center gap-2">
        <span class="w-1.5 h-1.5 rounded-full bg-[#10b981]"></span>
        <span class="font-sans font-semibold text-white text-xs">Logs</span>
      </div>

      <div class="h-4 w-[1px] bg-white/[0.08] hidden sm:block"></div>

      <!-- Custom Provider Filter Chip (No Native OS Select) -->
      <div class="relative">
        <button
          type="button"
          onclick={() => (isFilterOpen = !isFilterOpen)}
          class="flex items-center gap-2 bg-white/[0.04] hover:bg-white/[0.08] text-white px-3.5 py-1.5 rounded-full text-xs border border-white/[0.08] focus:outline-none transition-colors"
        >
          <span>{providerFilterOptions.find((p) => p.value === selectedFilter)?.label ?? 'All Providers'} ({logs.length})</span>
          <Icons name="chevron-down" size={10} class="text-[#94a3b8] transition-transform {isFilterOpen ? 'rotate-180' : ''}" />
        </button>

        {#if isFilterOpen}
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (isFilterOpen = false)}
            aria-label="Close menu"
          ></button>

          <div
            role="menu"
            tabindex="-1"
            onmouseleave={() => (isFilterOpen = false)}
            class="absolute top-9 left-0 min-w-[200px] bg-[#151926] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/80 p-1.5 z-50 text-xs animate-in fade-in zoom-in-95 duration-100"
          >
            {#each providerFilterOptions as opt}
              <button
                onclick={() => {
                  selectedFilter = opt.value;
                  isFilterOpen = false;
                }}
                class="w-full flex items-center justify-between px-3 py-1.5 rounded-xl text-left transition-colors {selectedFilter === opt.value ? 'bg-white/[0.08] text-white font-medium' : 'text-[#94a3b8] hover:text-white hover:bg-white/[0.04]'}"
              >
                <span>{opt.label}</span>
                {#if selectedFilter === opt.value}
                  <Icons name="check" size={12} class="text-[#00f2fe]" />
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Search + Auto Scroll + Actions -->
    <div class="flex items-center gap-2 flex-wrap sm:flex-nowrap">
      <!-- Search Input -->
      <input
        type="text"
        bind:value={filterSearch}
        placeholder="Filter logs..."
        class="bg-white/[0.04] text-white text-xs rounded-full px-3.5 py-1.5 border border-white/[0.08] focus:outline-none focus:border-[#00f2fe]/60 w-36 sm:w-44 placeholder:text-[#94a3b8]"
      />

      <!-- Auto scroll toggle -->
      <button
        onclick={() => (autoScroll = !autoScroll)}
        class="px-3.5 py-1.5 rounded-full border text-xs transition-colors {autoScroll
          ? 'bg-white/[0.05] text-[#00f2fe] border-[#00f2fe]/30'
          : 'bg-rose-500/10 text-rose-400 border-rose-500/30'}"
        title="Toggle automatic scrolling"
      >
        <span>{autoScroll ? 'Scroll: ON' : 'Paused'}</span>
      </button>

      <!-- Copy Action -->
      <button
        onclick={onCopy}
        class="w-7 h-7 rounded-full hover:bg-white/[0.08] text-[#94a3b8] hover:text-white border border-white/[0.08] flex items-center justify-center transition-colors"
        title="Copy visible log lines"
      >
        <Icons name="copy" size={13} />
      </button>

      <!-- Clear Action -->
      <button
        onclick={onClear}
        class="w-7 h-7 rounded-full hover:bg-white/[0.08] text-[#94a3b8] hover:text-white border border-white/[0.08] flex items-center justify-center transition-colors"
        title="Clear log buffer"
      >
        <Icons name="x" size={13} />
      </button>
    </div>
  </div>

  <!-- Terminal Stream Canvas Box -->
  <div class="flex-1 overflow-y-auto p-3 sm:p-4 space-y-1 select-text bg-[#0b0d19]">
    {#if filteredLogs.length > 0}
      {#each filteredLogs as log (log.id)}
        <div class="flex items-start gap-2.5 hover:bg-white/[0.04] px-3 py-1.5 rounded-xl leading-relaxed text-[11px] group transition-colors">
          <!-- Timestamp -->
          <span class="text-[#94a3b8] shrink-0 select-none">[{log.timestamp}]</span>

          <!-- Provider Badge (Standard Charcoal) -->
          <span
            class="px-2 py-0.5 rounded-full text-[10px] uppercase font-mono tracking-wider shrink-0 border border-white/[0.08] bg-white/[0.04] text-[#94a3b8] group-hover:border-[#8b5cf6]/40 select-none"
          >
            {log.provider}
          </span>

          <!-- Service Identifier Tag -->
          <span class="text-[#8b5cf6] font-medium shrink-0 select-none">[{log.serviceName}]</span>

          <!-- Log Message Content -->
          <span class="text-white break-all">{log.message}</span>
        </div>
      {/each}
    {:else}
      <div class="h-full flex items-center justify-center text-[#94a3b8] select-none">
        <span>No matching log events received in buffer.</span>
      </div>
    {/if}
  </div>
</div>
