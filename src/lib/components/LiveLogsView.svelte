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

<div class="flex-1 flex flex-col min-h-0 bg-[#161616] font-mono text-xs select-none">
  <!-- Top Log Toolbar -->
  <div class="min-h-[44px] py-2 px-4 border-b border-border-dim bg-surface/50 flex flex-wrap items-center justify-between gap-2.5 text-xs">
    <div class="flex items-center gap-3 flex-wrap">
      <div class="flex items-center gap-2">
        <span class="w-2 h-2 rounded-full bg-coral animate-ping"></span>
        <span class="font-sans font-semibold text-tx-header text-xs">Logs</span>
      </div>

      <div class="h-4 w-[1px] bg-border-dim hidden sm:block"></div>

      <!-- Custom Provider Filter Chip (No Native OS Select) -->
      <div class="relative">
        <button
          type="button"
          onclick={() => (isFilterOpen = !isFilterOpen)}
          class="flex items-center gap-1.5 bg-surface-elevated hover:bg-surface-hover text-tx-header px-2.5 py-1.5 rounded-lg text-xs border border-border-dim focus:outline-none transition-colors"
        >
          <span>{providerFilterOptions.find((p) => p.value === selectedFilter)?.label ?? 'All Providers'} ({logs.length})</span>
          <Icons name="chevron-down" size={10} class="text-tx-meta transition-transform {isFilterOpen ? 'rotate-180' : ''}" />
        </button>

        {#if isFilterOpen}
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (isFilterOpen = false)}
            aria-label="Close menu"
          ></button>

          <div class="absolute top-9 left-0 min-w-[200px] bg-[#18181f] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 text-xs animate-in fade-in zoom-in-95 duration-100">
            {#each providerFilterOptions as opt}
              <button
                onclick={() => {
                  selectedFilter = opt.value;
                  isFilterOpen = false;
                }}
                class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {selectedFilter === opt.value ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
              >
                <span>{opt.label}</span>
                {#if selectedFilter === opt.value}
                  <Icons name="check" size={12} class="text-coral" />
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
        class="bg-surface-elevated text-tx-header text-xs rounded-lg px-2.5 py-1.5 border border-border-dim focus:outline-none focus:border-coral w-36 sm:w-44 placeholder:text-tx-meta"
      />

      <!-- Auto scroll toggle -->
      <button
        onclick={() => (autoScroll = !autoScroll)}
        class="px-2.5 py-1.5 rounded-lg border text-xs transition-colors {autoScroll
          ? 'bg-surface-elevated text-tx-header border-border-dim hover:bg-surface-hover'
          : 'bg-[#2a1c1c] text-coral border-coral/30'}"
        title="Toggle automatic scrolling"
      >
        <span>{autoScroll ? 'Scroll: ON' : 'Paused'}</span>
      </button>

      <!-- Copy Action -->
      <button
        onclick={onCopy}
        class="p-1.5 rounded-lg hover:bg-surface-hover text-tx-body hover:text-tx-header border border-border-dim transition-colors"
        title="Copy visible log lines"
      >
        <Icons name="copy" size={13} />
      </button>

      <!-- Clear Action -->
      <button
        onclick={onClear}
        class="p-1.5 rounded-lg hover:bg-surface-hover text-tx-body hover:text-tx-header border border-border-dim transition-colors"
        title="Clear log buffer"
      >
        <Icons name="x" size={13} />
      </button>
    </div>
  </div>

  <!-- Terminal Stream Canvas Box -->
  <div class="flex-1 overflow-y-auto p-3 sm:p-4 space-y-1 select-text bg-[#121212]">
    {#if filteredLogs.length > 0}
      {#each filteredLogs as log (log.id)}
        <div class="flex items-start gap-2.5 hover:bg-surface-elevated/40 px-2.5 py-1 rounded-lg leading-relaxed text-[11px] group transition-colors">
          <!-- Timestamp -->
          <span class="text-tx-meta shrink-0 select-none">[{log.timestamp}]</span>

          <!-- Provider Badge (Standard Charcoal) -->
          <span
            class="px-1.5 py-0.2 rounded text-[10px] uppercase font-mono tracking-wider shrink-0 border border-border-dim bg-surface-elevated text-tx-body group-hover:border-coral/40 select-none"
          >
            {log.provider}
          </span>

          <!-- Service Identifier Tag -->
          <span class="text-tx-body font-medium shrink-0 select-none">[{log.serviceName}]</span>

          <!-- Log Message Content -->
          <span class="text-tx-header break-all">{log.message}</span>
        </div>
      {/each}
    {:else}
      <div class="h-full flex items-center justify-center text-tx-meta select-none">
        <span>No matching log events received in buffer.</span>
      </div>
    {/if}
  </div>
</div>
