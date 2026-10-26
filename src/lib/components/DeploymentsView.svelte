<script lang="ts">
  import Icons from './Icons.svelte';
  import type { ServiceTarget } from '$lib/types';

  export interface DeployJobState {
    serviceId: string;
    serviceName: string;
    provider: string;
    status: 'idle' | 'building' | 'deploying' | 'live' | 'failed';
    progress: number;
    message: string;
    durationMs: number;
  }

  let {
    services = [],
    isDeploying = false,
    deployJobs = [],
    isAdmin = false,
    onDeployAll = () => {},
    onOpenSecretSync = () => {},
    onDeploySingle = (service: ServiceTarget) => {},
    onAddTarget = () => {},
  }: {
    services: ServiceTarget[];
    isDeploying: boolean;
    deployJobs: DeployJobState[];
    isAdmin?: boolean;
    onDeployAll: () => void;
    onOpenSecretSync: () => void;
    onDeploySingle: (service: ServiceTarget) => void;
    onAddTarget?: () => void;
  } = $props();

  const deployableTargets = $derived(
    services.filter((s) => s.targetType === 'api' || s.targetType === 'frontend' || s.targetType === 'worker')
  );

  function getJob(serviceId: string): DeployJobState {
    const found = deployJobs.find((j) => j.serviceId === serviceId);
    if (found) return found;
    return {
      serviceId,
      serviceName: '',
      provider: '',
      status: 'idle',
      progress: 0,
      message: 'Idle - Ready for deployment',
      durationMs: 0,
    };
  }
</script>

<div class="flex-1 flex flex-col min-h-0 bg-canvas text-xs select-none">
  <!-- Top Banner: Controls & Status -->
  <div class="p-4 sm:p-6 border-b border-border-dim bg-surface/30">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2 flex-wrap">
          <h1 class="text-base font-semibold text-tx-header tracking-tight">Deployments</h1>
          {#if isDeploying}
            <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-[#2a1d1b] text-coral border border-coral/40 flex items-center gap-1.5 animate-pulse">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              Deploying
            </span>
          {:else}
            <span class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-header border border-border-dim flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              Ready
            </span>
          {/if}
        </div>
        <p class="text-xs text-tx-meta mt-1">
          Trigger concurrent builds and edge container distributions.
        </p>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2.5 flex-wrap sm:flex-nowrap">
        <button
          onclick={onAddTarget}
          class="px-3.5 py-1.5 rounded-xl bg-coral hover:bg-coral-hover text-white font-medium text-xs shadow-sm transition-all border border-coral/60 flex items-center gap-1.5 active:scale-[0.98]"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>New Deployment</span>
        </button>

        <button
          onclick={onOpenSecretSync}
          class="px-3.5 py-1.5 rounded-xl bg-surface-elevated hover:bg-surface-hover border border-border-dim hover:border-border-subtle text-tx-header font-medium text-xs transition-all flex items-center gap-1.5 active:scale-[0.98]"
        >
          <Icons name="key" size={13} class="text-coral" />
          <span>Sync Secrets</span>
        </button>

        <button
          onclick={onDeployAll}
          disabled={isDeploying || deployableTargets.length === 0}
          class="px-4 py-1.5 rounded-xl bg-surface-elevated hover:bg-surface-hover text-tx-header font-medium text-xs shadow-sm transition-all border border-border-dim hover:border-coral/50 flex items-center gap-1.5 disabled:opacity-50 active:scale-[0.98]"
        >
          <Icons name="rocket" size={13} class={isDeploying ? 'animate-bounce' : 'text-coral'} />
          <span>{isDeploying ? 'Deploying...' : 'Deploy All'}</span>
        </button>
      </div>
    </div>
  </div>

  <!-- Deployable Targets Cards List -->
  <div class="flex-1 overflow-y-auto p-2 sm:p-3">
    {#if deployableTargets.length > 0}
      <div class="px-3 py-1.5 text-[10px] uppercase font-mono tracking-wider text-tx-meta flex items-center justify-between">
        <span>Target Container & Edge Distributions ({deployableTargets.length})</span>
        <span>Build Pipeline • Live State • Actions</span>
      </div>

      {#each deployableTargets as s (s.id)}
        {@const job = getJob(s.id)}
        <div class="group mx-1 my-2 p-3.5 rounded-xl bg-surface/40 hover:bg-surface-elevated/70 border border-border-dim/70 hover:border-border-dim transition-all shadow-subtle flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <!-- Left: Service identity & provider badge -->
          <div class="flex items-center gap-3.5 min-w-0 sm:w-64 shrink-0">
            <!-- Status icon dot -->
            <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
              {#if job.status === 'live'}
                <span class="w-2.5 h-2.5 rounded-full bg-tx-header ring-2 ring-border-dim"></span>
              {:else if job.status === 'deploying'}
                <span class="w-2.5 h-2.5 rounded-full bg-coral animate-ping"></span>
              {:else if job.status === 'building'}
                <span class="w-2.5 h-2.5 rounded-full bg-coral/70 animate-pulse"></span>
              {:else}
                <span class="w-2 rounded-full bg-tx-meta"></span>
              {/if}
            </div>

            <div class="flex flex-col min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-xs font-semibold text-tx-header truncate">{s.name}</span>
                <span class="text-[10px] font-mono px-1.5 py-0.2 rounded bg-surface-elevated text-tx-meta border border-border-dim uppercase">
                  {s.provider}
                </span>
              </div>
              <div class="flex items-center gap-1.5 text-[11px] text-tx-meta">
                <span class="font-mono text-[10px]">{s.region}</span>
                <span class="text-border-subtle">•</span>
                <span>{s.environment}</span>
              </div>
            </div>
          </div>

          <!-- Center: Live Pulse Progress and Message -->
          <div class="flex-1 px-1 sm:px-4 flex flex-col gap-1.5 min-w-0">
            <div class="flex items-center justify-between text-[11px] flex-wrap gap-1">
              <span class="font-mono text-tx-body truncate max-w-md">{job.message}</span>
              <div class="flex items-center gap-2">
                {#if job.status !== 'idle'}
                  <span class="font-mono text-[10px] text-tx-meta">{job.durationMs}ms</span>
                {/if}
                <span
                  class="px-2 py-0.5 rounded-md text-[10px] font-mono uppercase {job.status === 'live'
                    ? 'bg-surface-elevated text-tx-header border border-border-dim font-medium'
                    : job.status === 'deploying'
                    ? 'bg-coral/15 text-coral animate-pulse border border-coral/30'
                    : job.status === 'building'
                    ? 'bg-surface-elevated text-tx-body border border-border-dim'
                    : 'bg-surface-elevated text-tx-meta border border-border-dim'}"
                >
                  {job.status}
                </span>
              </div>
            </div>

            <!-- Progress bar -->
            <div class="w-full h-1.5 bg-surface-elevated rounded-full overflow-hidden border border-border-dim">
              <div
                class="h-full transition-all duration-300 {job.status === 'live'
                  ? 'bg-tx-header'
                  : job.status === 'deploying'
                  ? 'bg-coral'
                  : job.status === 'building'
                  ? 'bg-tx-body'
                  : 'bg-transparent'}"
                style="width: {job.status === 'idle' ? 0 : job.progress}%"
              ></div>
            </div>
          </div>

          <!-- Right: Individual Deploy Button -->
          <div class="shrink-0 sm:pl-3 flex justify-end">
            <button
              onclick={() => onDeploySingle(s)}
              disabled={isDeploying || job.status === 'building' || job.status === 'deploying'}
              class="px-3.5 py-1.5 rounded-lg bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim hover:border-coral/50 font-medium text-xs flex items-center gap-1.5 transition-all active:scale-[0.98] disabled:opacity-40"
            >
              <Icons name="rocket" size={12} class="text-coral" />
              <span>Deploy</span>
            </button>
          </div>
        </div>
      {/each}
    {:else}
      <!-- Empty State -->
      <div class="h-64 flex flex-col items-center justify-center p-8 text-center select-none animate-in fade-in duration-200">
        <div class="w-14 h-14 rounded-2xl bg-surface-elevated border border-border-dim flex items-center justify-center mb-3 text-coral shadow-subtle">
          <Icons name="rocket" size={24} />
        </div>
        <h3 class="text-sm font-semibold text-tx-header mb-1 tracking-tight">No deployable targets</h3>
        <p class="text-xs text-tx-meta max-w-sm mb-4 leading-relaxed">
          Add an API, Frontend, or Worker target to configure automated parallel builds and edge deployments.
        </p>
        <button
          onclick={onAddTarget}
          class="px-4 py-2 rounded-xl bg-coral hover:bg-coral-hover text-white text-xs font-medium shadow-sm transition-all border border-coral/50 active:scale-[0.98] flex items-center gap-1.5"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>Create New Deployment</span>
        </button>
      </div>
    {/if}
  </div>
</div>
