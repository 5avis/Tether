<script lang="ts">
  import type { ServiceTarget } from '$lib/types';
  import Icons from './Icons.svelte';

  let {
    service,
    isAdmin = false,
    onDeploy = (s: ServiceTarget) => {},
    onSuspend = (s: ServiceTarget) => {},
    onResume = (s: ServiceTarget) => {},
    onSyncEnv = (s: ServiceTarget) => {},
    onViewLogs = (s: ServiceTarget) => {},
    onCopyUrl = (url: string) => {},
    onDelete = (id: string) => {},
    onOpenRenderKey = (s: ServiceTarget) => {},
  }: {
    service: ServiceTarget;
    isAdmin?: boolean;
    onDeploy?: (s: ServiceTarget) => void;
    onSuspend?: (s: ServiceTarget) => void;
    onResume?: (s: ServiceTarget) => void;
    onSyncEnv?: (s: ServiceTarget) => void;
    onViewLogs?: (s: ServiceTarget) => void;
    onCopyUrl?: (url: string) => void;
    onDelete?: (id: string) => void;
    onOpenRenderKey?: (s: ServiceTarget) => void;
  } = $props();

  let copied = $state(false);
  let showMenu = $state(false);

  function handleCopy(url: string) {
    onCopyUrl(url);
    copied = true;
    setTimeout(() => {
      copied = false;
    }, 1800);
  }

  const providerNames: Record<string, string> = {
    cloudflare: 'Cloudflare Tunnel',
    flyio: 'Fly.io App',
    supabase: 'Supabase DB',
    neon: 'Neon Serverless',
    render: 'Render Web',
    railway: 'Railway Service',
  };
</script>

<div
  class="group relative mx-3.5 my-1.5 p-3 rounded-xl bg-surface/40 hover:bg-surface-elevated/70 border border-border-dim/70 hover:border-border-dim transition-all text-xs select-none shadow-subtle flex flex-col sm:flex-row sm:items-center justify-between gap-3"
>
  <!-- Left Side: Status Dot, Service Identity, Target & Provider -->
  <div class="flex items-center gap-3 min-w-0">
    <!-- Clean Standard Status Dot -->
    <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
      <span
        class="w-2 h-2 rounded-full {service.status === 'healthy'
          ? 'bg-emerald-400 ring-2 ring-emerald-950'
          : service.status === 'dormant'
          ? 'bg-amber-400 ring-2 ring-amber-950'
          : service.status === 'warning'
          ? 'bg-coral/70'
          : 'bg-coral'} transition-transform group-hover:scale-125"
      ></span>
      {#if service.tunnelUrl}
        <span class="absolute w-4 h-4 rounded-full bg-coral/20 animate-ping"></span>
      {/if}
    </div>

    <!-- Service Title & Standard Metadata -->
    <div class="min-w-0 flex flex-col gap-0.5">
      <div class="flex items-center gap-2 flex-wrap">
        <span class="text-tx-header font-semibold tracking-tight text-xs truncate">{service.name}</span>

        <!-- Target Type Pill (Standard Charcoal) -->
        <span
          class="px-1.5 py-0.2 rounded text-[10px] uppercase font-mono tracking-wider bg-surface-elevated text-tx-meta border border-border-dim"
        >
          {service.targetType}
        </span>

        <!-- Environment Pill (Standard Charcoal) -->
        <span
          class="px-1.5 py-0.2 rounded text-[10px] font-mono bg-surface-elevated text-tx-meta border border-border-dim"
        >
          {service.environment}
        </span>

        <!-- Live / Suspended Lifecycle Pill -->
        {#if service.status === 'dormant'}
          <span
            class="px-1.5 py-0.2 rounded text-[10px] uppercase font-mono tracking-wider bg-amber-500/10 text-amber-400 border border-amber-500/20"
          >
            Suspended
          </span>
        {:else if service.status === 'healthy'}
          <span
            class="px-1.5 py-0.2 rounded text-[10px] uppercase font-mono tracking-wider bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
          >
            Live
          </span>
        {/if}
      </div>

      <!-- Provider & Region info -->
      <div class="flex items-center gap-2 text-[11px] text-tx-meta flex-wrap">
        <span class="text-tx-body">{providerNames[service.provider] ?? service.provider}</span>
        <span class="text-border-subtle">•</span>
        <span class="font-mono text-[10px]">{service.region}</span>
        <span class="text-border-subtle">•</span>
        <span>Last deployed {service.lastDeployedAt}</span>
      </div>
    </div>
  </div>

  <!-- Center: URL / Tunnel & Standard Metrics -->
  <div class="flex items-center gap-3.5 text-tx-meta flex-wrap justify-between sm:justify-end">
    <!-- Tunnel / Endpoint Preview -->
    {#if service.tunnelUrl}
      <button
        onclick={() => handleCopy(service.tunnelUrl!)}
        class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-surface-elevated border border-border-dim hover:border-coral/60 text-tx-header transition-colors font-mono text-[11px] max-w-[240px] truncate"
        title="Click to copy preview tunnel URL"
      >
        <Icons name="radio" size={12} class="shrink-0 text-coral" />
        <span class="truncate">{service.tunnelUrl.replace('https://', '')}</span>
        <Icons name={copied ? 'check' : 'copy'} size={11} class="shrink-0 ml-1 text-tx-meta" />
      </button>
    {:else if service.endpointUrl}
      <span class="font-mono text-[11px] text-tx-meta max-w-[200px] truncate">
        {service.endpointUrl.replace('https://', '')}
      </span>
    {/if}

    <!-- Free-Tier Watchdog Metrics Preview -->
    {#if service.dormancyHoursLeft !== undefined}
      <div class="flex items-center gap-1.5 text-[11px]">
        <Icons name="shield" size={12} class={service.dormancyHoursLeft < 72 ? 'text-coral' : 'text-tx-meta'} />
        <span class={service.dormancyHoursLeft < 72 ? 'text-coral font-medium' : 'text-tx-body'}>
          Pause in {service.dormancyHoursLeft}h
        </span>
      </div>
    {:else}
      <div class="flex items-center gap-2 text-[11px]">
        <div class="w-16 h-1.5 bg-surface-elevated rounded-full overflow-hidden border border-border-dim">
          <div
            class="h-full bg-tx-body"
            style="width: {service.quotaUsedPercent}%"
          ></div>
        </div>
        <span class="font-mono text-[10px] text-tx-meta">{service.quotaUsedPercent}%</span>
      </div>
    {/if}

    <!-- Latency -->
    <span class="font-mono text-[10px] text-tx-meta w-10 text-right">{service.latencyMs}ms</span>

    <!-- Right: Quick Action Icons -->
    <div class="flex items-center gap-1 relative">
      <div
        class="flex items-center gap-1 bg-surface-elevated px-1.5 py-1 rounded-lg border border-border-dim shadow-subtle"
      >
        {#if isAdmin}
          {#if service.status === 'dormant'}
            <button
              onclick={() => onResume(service)}
              class="p-1 rounded hover:bg-surface-hover text-emerald-400 hover:text-emerald-300 transition-colors"
              title="Resume / Start service"
              aria-label="Resume Service"
            >
              <Icons name="play" size={13} />
            </button>
          {:else}
            <button
              onclick={() => onSuspend(service)}
              class="p-1 rounded hover:bg-surface-hover text-tx-body hover:text-amber-400 transition-colors"
              title="Suspend / Stop service"
              aria-label="Suspend Service"
            >
              <Icons name="pause" size={13} />
            </button>
          {/if}

          <button
            onclick={() => onDeploy(service)}
            class="p-1 rounded hover:bg-surface-hover text-tx-body hover:text-coral transition-colors"
            title="Trigger instant deployment (auto-resumes if suspended)"
            aria-label="Deploy"
          >
            <Icons name="rocket" size={13} />
          </button>

          {#if service.provider === 'render'}
            <button
              onclick={() => onOpenRenderKey(service)}
              class="p-1 rounded hover:bg-surface-hover text-tx-body hover:text-coral transition-colors"
              title="Render Cloud API Key (rnd_...) - Configure live access"
              aria-label="Render API Key"
            >
              <Icons name="key" size={13} />
            </button>
          {/if}

          <button
            onclick={() => onSyncEnv(service)}
            class="p-1 rounded hover:bg-surface-hover text-tx-body hover:text-tx-header transition-colors"
            title="Sync secrets"
            aria-label="Sync Secrets"
          >
            <Icons name="key" size={13} />
          </button>
        {/if}

        <button
          onclick={() => onViewLogs(service)}
          class="p-1 rounded hover:bg-surface-hover text-tx-body hover:text-tx-header transition-colors"
          title="View live logs"
          aria-label="View Logs"
        >
          <Icons name="terminal" size={13} />
        </button>

        {#if service.tunnelUrl}
          <button
            onclick={() => handleCopy(service.tunnelUrl!)}
            class="p-1 rounded hover:bg-surface-hover text-tx-body hover:text-tx-header transition-colors"
            title="Copy tunnel URL"
            aria-label="Copy Tunnel URL"
          >
            <Icons name={copied ? 'check' : 'copy'} size={13} class={copied ? 'text-coral' : ''} />
          </button>
        {/if}
      </div>

      <!-- Functional More Options Context Menu Button -->
      <div class="relative">
        <button
          onclick={() => (showMenu = !showMenu)}
          class="p-1.5 rounded-lg hover:bg-surface-elevated text-tx-meta hover:text-tx-header border border-transparent hover:border-border-dim transition-colors"
          title="More options"
          aria-label="Target options"
        >
          <Icons name="more" size={13} />
        </button>

        {#if showMenu}
          <!-- Backdrop to close menu on click outside -->
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (showMenu = false)}
            aria-label="Close menu"
          ></button>

          <!-- Context Menu Dropdown -->
          <div
            class="absolute right-0 top-8 bg-[#242424] border border-border-dim rounded-xl shadow-modal p-1 z-50 w-44 animate-in fade-in zoom-in-95 duration-100 text-xs"
          >
            {#if isAdmin}
              {#if service.status === 'dormant'}
                <button
                  onclick={() => {
                    showMenu = false;
                    onResume(service);
                  }}
                  class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-emerald-400 hover:text-emerald-300 hover:bg-surface-elevated text-left transition-colors"
                >
                  <Icons name="play" size={12} class="text-emerald-400" />
                  <span>Resume Service</span>
                </button>
              {:else}
                <button
                  onclick={() => {
                    showMenu = false;
                    onSuspend(service);
                  }}
                  class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-amber-400 hover:text-amber-300 hover:bg-surface-elevated text-left transition-colors"
                >
                  <Icons name="pause" size={12} class="text-amber-400" />
                  <span>Suspend Service</span>
                </button>
              {/if}

              <button
                onclick={() => {
                  showMenu = false;
                  onDeploy(service);
                }}
                class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-tx-body hover:text-tx-header hover:bg-surface-elevated text-left transition-colors"
              >
                <Icons name="rocket" size={12} class="text-coral" />
                <span>Trigger Deploy</span>
              </button>

              <button
                onclick={() => {
                  showMenu = false;
                  onSyncEnv(service);
                }}
                class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-tx-body hover:text-tx-header hover:bg-surface-elevated text-left transition-colors"
              >
                <Icons name="key" size={12} />
                <span>Sync Secrets</span>
              </button>
            {/if}

            <button
              onclick={() => {
                showMenu = false;
                onViewLogs(service);
              }}
              class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-tx-body hover:text-tx-header hover:bg-surface-elevated text-left transition-colors"
            >
              <Icons name="terminal" size={12} />
              <span>Stream Logs</span>
            </button>

            <button
              onclick={() => {
                showMenu = false;
                navigator.clipboard?.writeText(service.id);
              }}
              class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-tx-body hover:text-tx-header hover:bg-surface-elevated text-left transition-colors font-mono text-[11px]"
            >
              <Icons name="copy" size={12} />
              <span>Copy Service ID</span>
            </button>

            {#if isAdmin}
              <div class="h-[1px] bg-border-dim my-1"></div>

              <button
                onclick={() => {
                  showMenu = false;
                  onDelete(service.id);
                }}
                class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-coral hover:bg-[#351a1a] text-left transition-colors font-medium"
              >
                <Icons name="trash" size={12} class="text-coral" />
                <span>Remove Target</span>
              </button>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
