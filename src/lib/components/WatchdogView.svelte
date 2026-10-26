<script lang="ts">
  import type { ServiceTarget } from '$lib/types';
  import Icons from './Icons.svelte';

  let {
    services = [],
    workspaceId = 'ws-default',
    isAdmin = false,
    onKeepAlive = (serviceId: string) => {},
    onKeepAliveAll = () => {},
    onToast = (msg: string) => {},
  }: {
    services: ServiceTarget[];
    workspaceId?: string;
    isAdmin?: boolean;
    onKeepAlive?: (serviceId: string) => void;
    onKeepAliveAll?: () => void;
    onToast?: (msg: string) => void;
  } = $props();

  let autoGuardEnabled = $state(true);
  let pingingId = $state<string | null>(null);
  let isPingingAll = $state(false);

  const watchdogTargets = $derived(
    services.filter(
      (s) => s.targetType === 'database' || s.dormancyHoursLeft !== undefined || s.quotaUsedPercent > 50
    )
  );

  const criticalCount = $derived(
    watchdogTargets.filter((s) => (s.dormancyHoursLeft ?? 999) <= 24).length
  );

  const warningCount = $derived(
    watchdogTargets.filter((s) => {
      const h = s.dormancyHoursLeft ?? 999;
      return h > 24 && h <= 72;
    }).length
  );

  const lowestHours = $derived(
    watchdogTargets.length === 0
      ? null
      : Math.min(...watchdogTargets.map((s) => s.dormancyHoursLeft ?? 168))
  );

  const avgLatency = $derived(
    watchdogTargets.length === 0
      ? null
      : Math.round(
          watchdogTargets.reduce((acc, s) => acc + (s.latencyMs || 0), 0) /
            watchdogTargets.length
        )
  );

  async function handleSinglePing(s: ServiceTarget) {
    pingingId = s.id;
    onToast(`Dispatching keep-alive query to ${s.name}...`);
    try {
      await onKeepAlive(s.id);
    } finally {
      pingingId = null;
    }
  }

  async function handleAll() {
    isPingingAll = true;
    onToast('Broadcasting keep-alive heartbeat queries across all databases...');
    try {
      await onKeepAliveAll();
    } finally {
      isPingingAll = false;
    }
  }
</script>

<div class="flex-1 flex flex-col min-h-0 bg-canvas text-xs select-none">
  <!-- Top Watchdog Summary & Quota Dashboard -->
  <div class="p-6 border-b border-border-dim bg-surface/30">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-4">
      <div>
        <div class="flex items-center gap-2">
          <h1 class="text-base font-semibold text-tx-header tracking-tight">Watchdog</h1>
          {#if criticalCount > 0}
            <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-coral border border-border-dim flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-coral animate-pulse"></span>
              {criticalCount} pause impending
            </span>
          {:else if warningCount > 0}
            <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-body border border-border-dim flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              {warningCount} approaching dormancy
            </span>
          {:else}
            <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-body border border-border-dim flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-tx-header"></span>
              All databases active & safe
            </span>
          {/if}
        </div>
        <p class="text-xs text-tx-meta mt-1">
          Prevents database pausing on Neon & Supabase by executing automated keep-alive queries before 7-day retention limits.
        </p>
      </div>

      <!-- Actions (Admin Only) -->
      {#if isAdmin}
        <div class="flex items-center gap-3">
          <button
            onclick={() => {
              autoGuardEnabled = !autoGuardEnabled;
              onToast(autoGuardEnabled ? 'Auto-Guard enabled' : 'Auto-Guard disabled');
            }}
            class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-surface-elevated border border-border-dim hover:border-border-subtle text-xs transition-colors"
          >
            <span class="w-1.5 h-1.5 rounded-full {autoGuardEnabled ? 'bg-coral' : 'bg-tx-meta'}"></span>
            <span class="text-tx-body">Auto-Guard: {autoGuardEnabled ? 'ON' : 'OFF'}</span>
          </button>

          <button
            onclick={handleAll}
            disabled={isPingingAll}
            class="px-4 py-1.5 rounded-full bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white font-medium text-xs shadow-sm transition-all border border-[#c084fc]/30 flex items-center gap-1.5 disabled:opacity-50 active:scale-[0.98] shadow-[0_0_12px_rgba(168,85,247,0.25)]"
          >
            <Icons name="power" size={13} class={isPingingAll ? 'animate-spin' : ''} />
            <span>Keep-Alive All</span>
          </button>
        </div>
      {:else}
        <span class="px-3 py-1.5 rounded-xl bg-surface-elevated text-tx-meta border border-border-dim text-xs">Read-Only View</span>
      {/if}
    </div>

    <!-- Metrics Cards -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
      <div class="p-3.5 rounded-2xl bg-surface/80 border border-border-dim shadow-subtle flex flex-col justify-between">
        <div class="flex items-center justify-between text-tx-meta text-xs">
          <span>Compute Pool</span>
          <Icons name="shield" size={12} class="text-coral" />
        </div>
        <div class="mt-2 flex items-baseline gap-2">
          <span class="text-xl font-bold text-tx-header font-mono">{lowestHours !== null ? lowestHours + 'h' : '0h'}</span>
          <span class="text-[11px] text-tx-meta font-mono">{lowestHours !== null ? 'lowest remaining' : 'no targets'}</span>
        </div>
      </div>

      <div class="p-3.5 rounded-2xl bg-surface/80 border border-border-dim shadow-subtle flex flex-col justify-between">
        <div class="flex items-center justify-between text-tx-meta text-xs">
          <span>Inactivity Threshold</span>
          <Icons name="activity" size={12} class="text-coral" />
        </div>
        <div class="mt-2 flex items-baseline gap-2">
          <span class="text-xl font-bold text-tx-header font-mono">168h</span>
          <span class="text-[11px] text-tx-meta font-mono">7-day lifecycle</span>
        </div>
      </div>

      <div class="p-3.5 rounded-2xl bg-surface/80 border border-border-dim shadow-subtle flex flex-col justify-between">
        <div class="flex items-center justify-between text-tx-meta text-xs">
          <span>Databases</span>
          <Icons name="database" size={12} class="text-coral" />
        </div>
        <div class="mt-2 flex items-baseline gap-2">
          <span class="text-xl font-bold text-tx-header font-mono">{watchdogTargets.length}</span>
          <span class="text-[11px] text-tx-meta font-mono">active</span>
        </div>
      </div>

      <div class="p-3.5 rounded-2xl bg-surface/80 border border-border-dim shadow-subtle flex flex-col justify-between">
        <div class="flex items-center justify-between text-tx-meta text-xs">
          <span>Latency</span>
          <Icons name="check" size={12} class="text-coral" />
        </div>
        <div class="mt-2 flex items-baseline gap-2">
          <span class="text-xl font-bold text-tx-header font-mono">{avgLatency !== null ? avgLatency + 'ms' : '--'}</span>
          <span class="text-[11px] text-tx-meta font-mono">{avgLatency !== null ? 'Avg probe' : 'no probes'}</span>
        </div>
      </div>
    </div>
  </div>

  <!-- Watchdog Cards List -->
  <div class="flex-1 overflow-y-auto p-2 sm:p-3">
    <div class="px-3 py-1.5 text-[10px] uppercase font-mono tracking-wider text-tx-meta flex items-center justify-between">
      <span>Lifecycle Retention Targets ({watchdogTargets.length})</span>
      <span>Dormancy Countdown • Compute Quota • Actions</span>
    </div>

    {#each watchdogTargets as s (s.id)}
      {@const hours = s.dormancyHoursLeft ?? 168}
      {@const isCrit = hours <= 24}
      {@const isWarn = hours > 24 && hours <= 72}
      <div
        class="group mx-1 my-2 p-3.5 rounded-xl bg-surface/40 hover:bg-surface-elevated/70 border border-border-dim/70 hover:border-border-dim transition-all shadow-subtle flex flex-col sm:flex-row sm:items-center justify-between gap-3"
      >
        <!-- Left: Task-style status dot & Service Metadata -->
        <div class="flex items-center gap-3.5 min-w-0">
          <!-- Circular Status Ring -->
          <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
            <span
              class="w-2 rounded-full {isCrit
                ? 'bg-coral'
                : isWarn
                ? 'bg-coral/70'
                : 'bg-tx-header ring-2 ring-border-dim'} transition-transform group-hover:scale-110"
            ></span>
            {#if isCrit}
              <span class="absolute w-3.5 h-3.5 rounded-full bg-coral/25 animate-ping"></span>
            {/if}
          </div>

          <!-- Service Identity -->
          <div class="flex flex-col gap-0.5 min-w-0">
            <div class="flex items-center gap-2 flex-wrap">
              <span class="text-xs font-semibold text-tx-header">{s.name}</span>
              <span class="text-[10px] uppercase font-mono px-1.5 py-0.2 rounded bg-surface-elevated text-tx-meta border border-border-dim">
                {s.provider}
              </span>
              <span class="text-[10px] font-mono text-tx-meta">{s.region}</span>
            </div>
            <div class="flex items-center gap-2 text-[11px] text-tx-meta flex-wrap">
              <span class="font-mono text-[10px] truncate max-w-xs">{s.endpointUrl ?? 'Active cluster'}</span>
              <span class="text-border-subtle">•</span>
              <span>Checked {s.lastDeployedAt}</span>
            </div>
          </div>
        </div>

        <!-- Center/Right: Inactivity Countdown, Quota Bar & Ping Action -->
        <div class="flex items-center gap-4 flex-wrap sm:flex-nowrap justify-between sm:justify-end">
          <!-- Inactivity Countdown Badge -->
          <div class="flex items-center gap-2">
            {#if isCrit}
              <span class="px-2.5 py-1 rounded-lg font-mono text-xs bg-surface-elevated text-coral border border-border-dim font-semibold flex items-center gap-1.5 shadow-sm">
                <Icons name="power" size={11} class="text-coral" />
                <span>Pause in {hours}h!</span>
              </span>
            {:else if isWarn}
              <span class="px-2.5 py-1 rounded-lg font-mono text-xs bg-surface-elevated text-tx-body border border-border-dim font-medium flex items-center gap-1.5">
                <Icons name="activity" size={11} class="text-tx-meta" />
                <span>Pause in {hours}h</span>
              </span>
            {:else}
              <span class="px-2.5 py-1 rounded-lg font-mono text-xs bg-surface-elevated text-tx-meta border border-border-dim flex items-center gap-1.5">
                <Icons name="check" size={11} class="text-coral" />
                <span>Safe ({hours}h left)</span>
              </span>
            {/if}
          </div>

          <!-- Quota Progress Bar -->
          <div class="flex items-center gap-2 text-[11px] text-tx-meta">
            <div class="w-20 sm:w-24 h-1.5 bg-surface-elevated rounded-full overflow-hidden border border-border-dim">
              <div
                class="h-full bg-tx-body transition-all duration-300"
                style="width: {s.quotaUsedPercent}%"
              ></div>
            </div>
            <span class="font-mono text-[10px] w-9 text-right">{s.quotaUsedPercent}%</span>
          </div>

          <!-- Latency -->
          <span class="font-mono text-[10px] text-tx-meta w-10 text-right">{s.latencyMs}ms</span>

          <!-- Individual Keep-Alive Action Button (Admin Only) -->
          {#if isAdmin}
            <button
              onclick={() => handleSinglePing(s)}
              disabled={pingingId === s.id}
              class="px-3 py-1.5 rounded-lg bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim hover:border-coral/50 font-medium text-xs flex items-center gap-1.5 transition-all active:scale-[0.98] disabled:opacity-40 shrink-0"
              title="Execute query to reset inactivity pause countdown"
            >
              {#if pingingId === s.id}
                <Icons name="refresh" size={12} class="animate-spin text-coral" />
                <span>Pinging...</span>
              {:else}
                <Icons name="power" size={12} class="text-coral" />
                <span>Keep-Alive Ping</span>
              {/if}
            </button>
          {/if}
        </div>
      </div>
    {:else}
      <div class="h-64 flex flex-col items-center justify-center p-8 text-center select-none animate-in fade-in duration-200">
        <div class="w-14 h-14 rounded-2xl bg-surface-elevated border border-border-dim flex items-center justify-center mb-3 text-coral shadow-subtle">
          <Icons name="shield" size={24} />
        </div>
        <h3 class="text-sm font-semibold text-tx-header mb-1 tracking-tight">No database targets found</h3>
        <p class="text-xs text-tx-meta max-w-sm mb-4 leading-relaxed">
          Add a Neon or Supabase PostgreSQL database target to automatically monitor inactivity pauses and deliver keep-alive heartbeat pings.
        </p>
      </div>
    {/each}
  </div>
</div>
