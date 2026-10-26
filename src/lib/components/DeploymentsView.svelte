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
    onOpenRenderKey = (service: ServiceTarget) => {},
  }: {
    services: ServiceTarget[];
    isDeploying: boolean;
    deployJobs: DeployJobState[];
    isAdmin?: boolean;
    onDeployAll: () => void;
    onOpenSecretSync: () => void;
    onDeploySingle: (service: ServiceTarget) => void;
    onAddTarget?: () => void;
    onOpenRenderKey?: (service: ServiceTarget) => void;
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

<div class="flex-1 flex flex-col min-h-0 bg-transparent text-xs select-none">
  <!-- Top Banner: Controls & Status -->
  <div class="p-4 sm:p-6 border-b border-white/[0.08] bg-white/[0.02] backdrop-blur-md">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2 flex-wrap">
          <h1 class="text-base font-semibold text-white tracking-tight">Deployments</h1>
          {#if isDeploying}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-[#ec4899]/15 text-[#ec4899] border border-[#ec4899]/40 flex items-center gap-1.5 animate-pulse">
              <span class="w-1.5 h-1.5 rounded-full bg-[#ec4899]"></span>
              Deploying
            </span>
          {:else}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08] flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-[#10b981]"></span>
              Ready
            </span>
          {/if}
        </div>
        <p class="text-xs text-[#94a3b8] mt-1">
          Trigger concurrent builds and edge container distributions.
        </p>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2.5 flex-wrap sm:flex-nowrap">
        <button
          onclick={onAddTarget}
          class="px-4 py-2 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white font-medium text-xs shadow-sm flex items-center gap-2 active:scale-[0.98] transition-all border border-white/10"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>New Deployment</span>
        </button>

        <button
          onclick={onOpenSecretSync}
          class="px-4 py-2 rounded-full bg-white/[0.04] hover:bg-white/[0.08] border border-white/[0.08] hover:border-white/[0.15] text-white font-medium text-xs transition-all flex items-center gap-2 active:scale-[0.98]"
        >
          <Icons name="key" size={13} class="text-[#00f2fe]" />
          <span>Sync Secrets</span>
        </button>

        <button
          onclick={onDeployAll}
          disabled={isDeploying || deployableTargets.length === 0}
          class="px-4 py-2 rounded-full bg-white/[0.04] hover:bg-white/[0.08] text-white font-medium text-xs shadow-sm transition-all border border-white/[0.08] hover:border-white/20 flex items-center gap-2 disabled:opacity-40 active:scale-[0.98]"
        >
          <Icons name="rocket" size={13} class={isDeploying ? 'animate-bounce text-[#ec4899]' : 'text-[#8b5cf6]'} />
          <span>{isDeploying ? 'Deploying...' : 'Deploy All'}</span>
        </button>
      </div>
    </div>
  </div>

  <!-- Deployable Targets Cards List -->
  <div class="flex-1 overflow-y-auto p-2 sm:p-4">
    {#if deployableTargets.length > 0}
      <div class="px-3 py-1.5 text-[10px] uppercase font-mono tracking-wider text-[#94a3b8] flex items-center justify-between">
        <span>Target Container & Edge Distributions ({deployableTargets.length})</span>
        <span>Build Pipeline • Live State • Actions</span>
      </div>

      {#each deployableTargets as s (s.id)}
        {@const job = getJob(s.id)}
        <div class="group mx-1 my-2.5 p-4 rounded-2xl bg-white/[0.02] hover:bg-white/[0.05] border border-white/[0.07] hover:border-white/[0.15] transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-3.5 shadow-sm">
          <!-- Left: Service identity & provider badge -->
          <div class="flex items-center gap-3.5 min-w-0 sm:w-64 shrink-0">
            <!-- Status icon dot -->
            <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
              {#if job.status === 'live'}
                <span class="w-2.5 h-2.5 rounded-full bg-[#10b981]"></span>
              {:else if job.status === 'deploying'}
                <span class="w-2.5 h-2.5 rounded-full bg-[#ec4899] animate-ping"></span>
              {:else if job.status === 'building'}
                <span class="w-2.5 h-2.5 rounded-full bg-[#ff5e62] animate-pulse"></span>
              {:else}
                <span class="w-2 rounded-full bg-white/20"></span>
              {/if}
            </div>

            <div class="flex flex-col min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="text-xs font-semibold text-white truncate">{s.name}</span>
                <span class="text-[10px] font-mono px-2.5 py-0.5 rounded-full bg-white/[0.05] text-[#94a3b8] border border-white/[0.08] uppercase">
                  {s.provider}
                </span>
              </div>
              <div class="flex items-center gap-1.5 text-[11px] text-[#94a3b8] mt-0.5">
                <span class="font-mono text-[10px]">{s.region}</span>
                <span class="text-white/20">•</span>
                <span>{s.environment}</span>
              </div>
            </div>
          </div>

          <!-- Center: Live Pulse Progress and Message -->
          <div class="flex-1 px-1 sm:px-4 flex flex-col gap-1.5 min-w-0">
            <div class="flex items-center justify-between text-[11px] flex-wrap gap-1">
              <span class="font-mono text-slate-200 truncate max-w-md">{job.message}</span>
              <div class="flex items-center gap-2">
                {#if job.status !== 'idle'}
                  <span class="font-mono text-[10px] text-[#94a3b8]">{job.durationMs}ms</span>
                {/if}
                <span
                  class="px-2.5 py-0.5 rounded-full text-[10px] font-mono uppercase {job.status === 'live'
                    ? 'bg-[#00f2fe]/10 text-[#00f2fe] border border-[#00f2fe]/30 font-medium'
                    : job.status === 'deploying'
                    ? 'bg-[#ec4899]/15 text-[#ec4899] animate-pulse border border-[#ec4899]/30'
                    : job.status === 'building'
                    ? 'bg-[#ff5e62]/15 text-[#ff5e62] border border-[#ff5e62]/30'
                    : 'bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]'}"
                >
                  {job.status}
                </span>
              </div>
            </div>

            <!-- Progress bar -->
            <div class="w-full h-1.5 bg-white/[0.05] rounded-full overflow-hidden border border-white/[0.08]">
              <div
                class="h-full transition-all duration-300 {job.status === 'live'
                  ? 'bg-gradient-to-r from-[#00f2fe] to-[#8b5cf6]'
                  : job.status === 'deploying'
                  ? 'bg-gradient-to-r from-[#8b5cf6] to-[#ec4899]'
                  : job.status === 'building'
                  ? 'bg-[#ff5e62]'
                  : 'bg-transparent'}"
                style="width: {job.status === 'idle' ? 0 : job.progress}%"
              ></div>
            </div>
          </div>

          <!-- Right: Individual Deploy Button -->
          <div class="shrink-0 sm:pl-3 flex items-center justify-end gap-1.5">
            {#if s.provider === 'render'}
              <button
                onclick={() => onOpenRenderKey(s)}
                class="px-3 py-1.5 rounded-full bg-white/[0.04] hover:bg-white/[0.08] text-[#94a3b8] hover:text-white border border-white/[0.08] font-medium text-xs flex items-center gap-1.5 transition-all"
                title="Configure Render Cloud API Key (rnd_...)"
              >
                <Icons name="key" size={12} class="text-[#00f2fe]" />
                <span class="hidden sm:inline">Render Key</span>
              </button>
            {/if}
            <button
              onclick={() => onDeploySingle(s)}
              disabled={isDeploying || job.status === 'building' || job.status === 'deploying'}
              class="px-4 py-1.5 rounded-full bg-white/[0.04] hover:bg-white/[0.08] text-white border border-white/[0.08] hover:border-white/20 font-medium text-xs flex items-center gap-1.5 transition-all active:scale-[0.98] disabled:opacity-40 shadow-sm"
            >
              <Icons name="rocket" size={12} class="text-[#8b5cf6]" />
              <span>Deploy</span>
            </button>
          </div>
        </div>
      {/each}
    {:else}
      <!-- Empty State -->
      <div class="h-64 flex flex-col items-center justify-center p-8 text-center select-none animate-in fade-in duration-200">
        <div class="w-14 h-14 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center mb-3 text-[#ec4899]">
          <Icons name="rocket" size={24} />
        </div>
        <h3 class="text-sm font-semibold text-white mb-1 tracking-tight">No deployable targets</h3>
        <p class="text-xs text-[#94a3b8] max-w-sm mb-4 leading-relaxed">
          Add an API, Frontend, or Worker target to configure automated parallel builds and edge deployments.
        </p>
        <button
          onclick={onAddTarget}
          class="px-5 py-2.5 rounded-full bg-[#7c3aed] hover:bg-[#6d28d9] text-white text-xs font-semibold shadow-sm transition-all border border-white/10 active:scale-[0.98] flex items-center gap-2"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>Create New Deployment</span>
        </button>
      </div>
    {/if}
  </div>
</div>
