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

<div class="flex-1 flex flex-col min-h-0 bg-transparent text-xs select-none">
  <!-- Top Watchdog Summary & Quota Dashboard -->
  <div class="p-4 sm:p-6 border-b border-white/[0.08] bg-white/[0.02] backdrop-blur-md">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-4">
      <div>
        <div class="flex items-center gap-2 flex-wrap">
          <h1 class="text-base font-semibold text-white tracking-tight">Watchdog</h1>
          {#if criticalCount > 0}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-[#ec4899]/15 text-[#ec4899] border border-[#ec4899]/40 flex items-center gap-1.5 shadow-[0_0_12px_rgba(236,72,153,0.3)] animate-pulse">
              <span class="w-1.5 h-1.5 rounded-full bg-[#ec4899]"></span>
              {criticalCount} pause impending
            </span>
          {:else if warningCount > 0}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-[#ff5e62]/15 text-[#ff5e62] border border-[#ff5e62]/30 flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-[#ff5e62]"></span>
              {warningCount} approaching dormancy
            </span>
          {:else}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-[#00f2fe]/10 text-[#00f2fe] border border-[#00f2fe]/30 flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
              All databases active & safe
            </span>
          {/if}
        </div>
        <p class="text-xs text-[#94a3b8] mt-1">
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
            class="flex items-center gap-2 px-4 py-2 rounded-full bg-white/[0.04] border border-white/[0.08] hover:border-white/[0.15] text-xs transition-colors"
          >
            <span class="w-2 h-2 rounded-full {autoGuardEnabled ? 'bg-[#00f2fe] shadow-[0_0_8px_#00f2fe]' : 'bg-white/20'}"></span>
            <span class="text-slate-200">Auto-Guard: {autoGuardEnabled ? 'ON' : 'OFF'}</span>
          </button>

          <button
            onclick={handleAll}
            disabled={isPingingAll}
            class="px-5 py-2 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-semibold text-xs shadow-[0_0_20px_rgba(139,92,246,0.35)] flex items-center gap-2 disabled:opacity-50 active:scale-[0.98] transition-all"
          >
            <Icons name="power" size={13} class={isPingingAll ? 'animate-spin' : ''} />
            <span>Keep-Alive All</span>
          </button>
        </div>
      {:else}
        <span class="px-3 py-1.5 rounded-full bg-white/[0.04] text-[#94a3b8] border border-white/[0.08] text-xs">Read-Only View</span>
      {/if}
    </div>

    <!-- Metrics Cards -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
      <div class="p-3.5 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] hover:border-[#8b5cf6]/40 hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] transition-all flex flex-col justify-between">
        <div class="flex items-center justify-between text-[#94a3b8] text-xs">
          <span class="font-medium text-slate-200">Compute Pool</span>
          <div class="w-6 h-6 rounded-full bg-white/[0.04] flex items-center justify-center text-[#ff5e62]">
            <Icons name="shield" size={12} />
          </div>
        </div>
        <div class="mt-2.5 flex items-baseline justify-between">
          <span class="text-2xl font-bold text-white font-mono">{lowestHours !== null ? lowestHours + 'h' : '0h'}</span>
          <span class="text-[11px] text-[#94a3b8] font-mono">{lowestHours !== null ? 'lowest left' : 'no targets'}</span>
        </div>
      </div>

      <div class="p-3.5 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] hover:border-[#8b5cf6]/40 hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] transition-all flex flex-col justify-between">
        <div class="flex items-center justify-between text-[#94a3b8] text-xs">
          <span class="font-medium text-slate-200">Inactivity Threshold</span>
          <div class="w-6 h-6 rounded-full bg-white/[0.04] flex items-center justify-center text-[#00f2fe]">
            <Icons name="activity" size={12} />
          </div>
        </div>
        <div class="mt-2.5 flex items-baseline justify-between">
          <span class="text-2xl font-bold text-white font-mono">168h</span>
          <span class="text-[11px] text-[#94a3b8] font-mono">7-day lifecycle</span>
        </div>
      </div>

      <div class="p-3.5 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] hover:border-[#8b5cf6]/40 hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] transition-all flex flex-col justify-between">
        <div class="flex items-center justify-between text-[#94a3b8] text-xs">
          <span class="font-medium text-slate-200">Databases</span>
          <div class="w-6 h-6 rounded-full bg-white/[0.04] flex items-center justify-center text-[#8b5cf6]">
            <Icons name="database" size={12} />
          </div>
        </div>
        <div class="mt-2.5 flex items-baseline justify-between">
          <span class="text-2xl font-bold text-white font-mono">{watchdogTargets.length}</span>
          <span class="text-[11px] text-[#94a3b8] font-mono">active</span>
        </div>
      </div>

      <div class="p-3.5 rounded-3xl bg-white/[0.03] backdrop-blur-xl border border-white/[0.08] hover:border-[#8b5cf6]/40 hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] transition-all flex flex-col justify-between">
        <div class="flex items-center justify-between text-[#94a3b8] text-xs">
          <span class="font-medium text-slate-200">Latency</span>
          <div class="w-6 h-6 rounded-full bg-white/[0.04] flex items-center justify-center text-[#00f2fe]">
            <Icons name="check" size={12} />
          </div>
        </div>
        <div class="mt-2.5 flex items-baseline justify-between">
          <span class="text-2xl font-bold text-white font-mono">{avgLatency !== null ? avgLatency + 'ms' : '--'}</span>
          <span class="text-[11px] text-[#94a3b8] font-mono">{avgLatency !== null ? 'Avg probe' : 'no probes'}</span>
        </div>
      </div>
    </div>
  </div>

  <!-- Watchdog Cards List -->
  <div class="flex-1 overflow-y-auto p-2 sm:p-4">
    <div class="px-3 py-1.5 text-[10px] uppercase font-mono tracking-wider text-[#94a3b8] flex items-center justify-between">
      <span>Lifecycle Retention Targets ({watchdogTargets.length})</span>
      <span>Dormancy Countdown • Compute Quota • Actions</span>
    </div>

    {#each watchdogTargets as s (s.id)}
      {@const hours = s.dormancyHoursLeft ?? 168}
      {@const isCrit = hours <= 24}
      {@const isWarn = hours > 24 && hours <= 72}
      <div
        class="group mx-1 my-2.5 p-4 rounded-3xl bg-white/[0.03] hover:bg-white/[0.06] backdrop-blur-xl border border-white/[0.08] hover:border-[#8b5cf6]/40 hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-3.5"
      >
        <!-- Left: Task-style status dot & Service Metadata -->
        <div class="flex items-center gap-3.5 min-w-0">
          <!-- Circular Status Ring -->
          <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
            <span
              class="w-2.5 h-2.5 rounded-full {isCrit
                ? 'bg-[#ec4899] shadow-[0_0_8px_#ec4899]'
                : isWarn
                ? 'bg-[#ff5e62] shadow-[0_0_8px_#ff5e62]'
                : 'bg-[#00f2fe] shadow-[0_0_8px_#00f2fe]'} transition-transform group-hover:scale-110"
            ></span>
            {#if isCrit}
              <span class="absolute w-4 h-4 rounded-full bg-[#ec4899]/30 animate-ping"></span>
            {/if}
          </div>

          <!-- Service Identity -->
          <div class="flex flex-col gap-0.5 min-w-0">
            <div class="flex items-center gap-2 flex-wrap">
              <span class="text-xs font-semibold text-white">{s.name}</span>
              <span class="text-[10px] uppercase font-mono px-2.5 py-0.5 rounded-full bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
                {s.provider}
              </span>
              <span class="text-[10px] font-mono text-[#94a3b8]">{s.region}</span>
            </div>
            <div class="flex items-center gap-2 text-[11px] text-[#94a3b8] flex-wrap mt-0.5">
              <span class="font-mono text-[10px] truncate max-w-xs">{s.endpointUrl ?? 'Active cluster'}</span>
              <span class="text-white/20">•</span>
              <span>Checked {s.lastDeployedAt}</span>
            </div>
          </div>
        </div>

        <!-- Center/Right: Inactivity Countdown, Quota Bar & Ping Action -->
        <div class="flex items-center gap-4 flex-wrap sm:flex-nowrap justify-between sm:justify-end">
          <!-- Inactivity Countdown Badge -->
          <div class="flex items-center gap-2">
            {#if isCrit}
              <span class="px-3 py-1 rounded-full font-mono text-xs bg-[#ec4899]/15 text-[#ec4899] border border-[#ec4899]/30 font-semibold flex items-center gap-1.5 shadow-[0_0_10px_rgba(236,72,153,0.25)]">
                <Icons name="power" size={11} class="text-[#ec4899]" />
                <span>Pause in {hours}h!</span>
              </span>
            {:else if isWarn}
              <span class="px-3 py-1 rounded-full font-mono text-xs bg-[#ff5e62]/15 text-[#ff5e62] border border-[#ff5e62]/30 font-medium flex items-center gap-1.5">
                <Icons name="activity" size={11} class="text-[#ff5e62]" />
                <span>Pause in {hours}h</span>
              </span>
            {:else}
              <span class="px-3 py-1 rounded-full font-mono text-xs bg-[#00f2fe]/10 text-[#00f2fe] border border-[#00f2fe]/30 flex items-center gap-1.5">
                <Icons name="check" size={11} class="text-[#00f2fe]" />
                <span>Safe ({hours}h left)</span>
              </span>
            {/if}
          </div>

          <!-- Quota Progress Bar -->
          <div class="flex items-center gap-2 text-[11px] text-[#94a3b8]">
            <div class="w-20 sm:w-24 h-1.5 bg-white/[0.05] rounded-full overflow-hidden border border-white/[0.08]">
              <div
                class="h-full bg-gradient-to-r from-[#00f2fe] to-[#8b5cf6] transition-all duration-300"
                style="width: {s.quotaUsedPercent}%"
              ></div>
            </div>
            <span class="font-mono text-[10px] w-9 text-right text-slate-200">{s.quotaUsedPercent}%</span>
          </div>

          <!-- Latency -->
          <span class="font-mono text-[10px] text-[#94a3b8] w-10 text-right">{s.latencyMs}ms</span>

          <!-- Individual Keep-Alive Action Button (Admin Only) -->
          {#if isAdmin}
            <button
              onclick={() => handleSinglePing(s)}
              disabled={pingingId === s.id}
              class="px-4 py-1.5 rounded-full bg-white/[0.04] hover:bg-gradient-to-r hover:from-[#8b5cf6] hover:to-[#ec4899] text-white border border-white/[0.08] hover:border-transparent font-medium text-xs flex items-center gap-1.5 transition-all active:scale-[0.98] disabled:opacity-40 shrink-0 shadow-[0_0_12px_rgba(139,92,246,0.15)]"
              title="Execute query to reset inactivity pause countdown"
            >
              {#if pingingId === s.id}
                <Icons name="refresh" size={12} class="animate-spin text-[#00f2fe]" />
                <span>Pinging...</span>
              {:else}
                <Icons name="power" size={12} class="text-[#00f2fe]" />
                <span>Keep-Alive Ping</span>
              {/if}
            </button>
          {/if}
        </div>
      </div>
    {:else}
      <div class="h-64 flex flex-col items-center justify-center p-8 text-center select-none animate-in fade-in duration-200">
        <div class="w-14 h-14 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center mb-3 text-[#ff5e62] shadow-[0_0_20px_rgba(255,94,98,0.25)]">
          <Icons name="shield" size={24} />
        </div>
        <h3 class="text-sm font-semibold text-white mb-1 tracking-tight">No database targets found</h3>
        <p class="text-xs text-[#94a3b8] max-w-sm mb-4 leading-relaxed">
          Add a Neon or Supabase PostgreSQL database target to automatically monitor inactivity pauses and deliver keep-alive heartbeat pings.
        </p>
      </div>
    {/each}
  </div>
</div>
