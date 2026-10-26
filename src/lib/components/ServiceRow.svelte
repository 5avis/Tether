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
    vercel: 'Vercel App',
    railway: 'Railway Service',
  };
</script>

<div
  class="group relative mx-3.5 my-2 p-3 px-4 rounded-3xl bg-white/[0.03] hover:bg-white/[0.06] border border-white/[0.08] hover:border-[#8b5cf6]/40 transition-all text-xs select-none shadow-subtle hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] backdrop-blur-md flex flex-col md:flex-row md:items-center justify-between gap-3"
>
  <!-- Left Side: Status Dot, Service Identity, Target & Provider -->
  <div class="flex items-center gap-3 min-w-0 flex-1">
    <!-- Clean Standard Status Dot (Circular with neon glow) -->
    <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
      <span
        class="w-2.5 h-2.5 rounded-full {service.status === 'healthy'
          ? 'bg-[#10b981] shadow-[0_0_8px_#10b981]'
          : service.status === 'dormant'
          ? 'bg-[#f59e0b] shadow-[0_0_8px_#f59e0b]'
          : 'bg-[#8b5cf6] shadow-[0_0_8px_#8b5cf6]'} transition-transform group-hover:scale-125"
      ></span>
      {#if service.tunnelUrl}
        <span class="absolute w-4 h-4 rounded-full bg-[#00f2fe]/30 animate-ping"></span>
      {/if}
    </div>

    <!-- Service Title & Standard Metadata -->
    <div class="min-w-0 flex flex-col gap-0.5">
      <div class="flex items-center gap-2 flex-wrap">
        <span class="text-tx-header font-semibold tracking-tight text-xs truncate max-w-[200px] sm:max-w-[260px]">{service.name}</span>

        <!-- Target Type Pill (Fully Pill-Shaped) -->
        <span
          class="px-2.5 py-0.5 rounded-full text-[10px] uppercase font-mono tracking-wider bg-white/[0.04] text-tx-meta border border-white/[0.08] shrink-0"
        >
          {service.targetType}
        </span>

        <!-- Environment Pill (Fully Pill-Shaped) -->
        <span
          class="px-2.5 py-0.5 rounded-full text-[10px] font-mono bg-white/[0.04] text-tx-meta border border-white/[0.08] shrink-0"
        >
          {service.environment}
        </span>

        <!-- Live / Suspended Lifecycle Pill (Fully Pill-Shaped) -->
        {#if service.status === 'dormant'}
          <span
            class="px-2.5 py-0.5 rounded-full text-[10px] uppercase font-mono tracking-wider bg-amber-500/15 text-amber-300 border border-amber-500/30 shrink-0 font-medium shadow-[0_0_8px_rgba(245,158,11,0.2)]"
          >
            Suspended
          </span>
        {:else if service.status === 'healthy'}
          <span
            class="px-2.5 py-0.5 rounded-full text-[10px] uppercase font-mono tracking-wider bg-emerald-500/15 text-emerald-300 border border-emerald-500/30 shrink-0 font-medium shadow-[0_0_8px_rgba(16,185,129,0.2)]"
          >
            Live
          </span>
        {/if}
      </div>

      <!-- Provider & Region info -->
      <div class="flex items-center gap-2 text-[11px] text-tx-meta flex-wrap">
        <span class="text-tx-body font-medium">{providerNames[service.provider] ?? service.provider}</span>
        <span class="text-white/20">•</span>
        <span class="font-mono text-[10px]">{service.region}</span>
        <span class="text-white/20">•</span>
        <span class="truncate">Last deployed {service.lastDeployedAt}</span>
      </div>
    </div>
  </div>

  <!-- Center & Right: URL, Metrics, Actions -->
  <div class="flex items-center gap-3 text-tx-meta shrink-0 justify-between md:justify-end flex-wrap sm:flex-nowrap">
    <!-- Tunnel / Endpoint Preview (Pill-Shaped with Neon Cyan Glow) -->
    {#if service.tunnelUrl}
      <button
        onclick={() => handleCopy(service.tunnelUrl!)}
        class="flex items-center gap-1.5 px-3 py-1 rounded-full bg-white/[0.04] border border-white/[0.08] hover:border-[#00f2fe]/60 text-tx-header transition-all font-mono text-[11px] max-w-[200px] truncate shadow-[0_0_12px_rgba(0,242,254,0.15)]"
        title="Click to copy preview tunnel URL"
      >
        <Icons name="radio" size={12} class="shrink-0 text-[#00f2fe]" />
        <span class="truncate">{service.tunnelUrl.replace('https://', '')}</span>
        <Icons name={copied ? 'check' : 'copy'} size={11} class="shrink-0 ml-1 text-tx-meta" />
      </button>
    {:else if service.endpointUrl}
      <span class="font-mono text-[11px] text-tx-meta max-w-[180px] truncate" title={service.endpointUrl}>
        {service.endpointUrl.replace('https://', '')}
      </span>
    {/if}

    <!-- Free-Tier Watchdog Metrics Preview -->
    {#if service.dormancyHoursLeft !== undefined}
      <div class="flex items-center gap-1.5 text-[11px] shrink-0">
        <Icons name="shield" size={12} class={service.dormancyHoursLeft < 72 ? 'text-[#ff5e62]' : 'text-tx-meta'} />
        <span class={service.dormancyHoursLeft < 72 ? 'text-[#ff5e62] font-semibold' : 'text-tx-body'}>
          Pause in {service.dormancyHoursLeft}h
        </span>
      </div>
    {:else}
      <div class="flex items-center gap-2 text-[11px] shrink-0">
        <div class="w-16 h-1.5 bg-white/[0.08] rounded-full overflow-hidden border border-white/[0.08]">
          <div
            class="h-full bg-gradient-to-r from-[#00f2fe] via-[#8b5cf6] to-[#ec4899] transition-all duration-300"
            style="width: {service.quotaUsedPercent}%"
          ></div>
        </div>
        <span class="font-mono text-[10px] text-tx-meta w-7">{service.quotaUsedPercent}%</span>
      </div>
    {/if}

    <!-- Latency -->
    <span class="font-mono text-[10px] text-tx-meta w-9 text-right shrink-0">{service.latencyMs}ms</span>

    <!-- Quick Action Icons Toolbar (Fully Pill-Shaped) -->
    <div
      class="flex items-center gap-1 bg-white/[0.04] p-1 rounded-full border border-white/[0.08] shadow-subtle shrink-0 backdrop-blur-md"
    >
      {#if isAdmin}
        {#if service.status === 'dormant'}
          <button
            onclick={() => onResume(service)}
            class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-emerald-400 hover:text-emerald-300 transition-colors"
            title="Resume / Start service"
            aria-label="Resume Service"
          >
            <Icons name="play" size={13} />
          </button>
        {:else}
          <button
            onclick={() => onSuspend(service)}
            class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-tx-body hover:text-amber-400 transition-colors"
            title="Suspend / Stop service"
            aria-label="Suspend Service"
          >
            <Icons name="pause" size={13} />
          </button>
        {/if}

        <button
          onclick={() => onDeploy(service)}
          class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-tx-body hover:text-[#00f2fe] transition-colors"
          title="Trigger instant deployment"
          aria-label="Deploy"
        >
          <Icons name="rocket" size={13} />
        </button>

        {#if service.provider === 'render'}
          <button
            onclick={() => onOpenRenderKey(service)}
            class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-tx-body hover:text-[#8b5cf6] transition-colors"
            title="Render Cloud API Key (rnd_...)"
            aria-label="Render API Key"
          >
            <Icons name="key" size={13} />
          </button>
        {/if}

        <button
          onclick={() => onSyncEnv(service)}
          class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-tx-body hover:text-tx-header transition-colors"
          title="Sync secrets"
          aria-label="Sync Secrets"
        >
          <Icons name="key" size={13} />
        </button>
      {/if}

      <button
        onclick={() => onViewLogs(service)}
        class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-tx-body hover:text-tx-header transition-colors"
        title="View live logs"
        aria-label="View Logs"
      >
        <Icons name="terminal" size={13} />
      </button>

      <!-- Context Menu Dropdown Trigger -->
      <div class="relative">
        <button
          onclick={() => (showMenu = !showMenu)}
          class="w-7 h-7 rounded-full flex items-center justify-center hover:bg-white/[0.08] text-tx-meta hover:text-tx-header transition-colors"
          title="More options"
          aria-label="More Options"
        >
          <Icons name="more" size={13} />
        </button>

        {#if showMenu}
          <button
            type="button"
            class="fixed inset-0 z-40 cursor-default bg-transparent"
            onclick={() => (showMenu = false)}
            aria-label="Close menu"
          ></button>

          <div
            class="absolute right-0 top-9 bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-3xl shadow-[0_0_35px_rgba(139,92,246,0.25)] p-2 z-50 w-48 animate-in fade-in zoom-in-95 duration-100 text-xs space-y-0.5"
          >
            {#if service.tunnelUrl}
              <button
                onclick={() => {
                  handleCopy(service.tunnelUrl!);
                  showMenu = false;
                }}
                class="w-full flex items-center gap-2 px-3 py-1.5 rounded-full text-tx-body hover:text-tx-header hover:bg-white/[0.06] transition-colors text-left"
              >
                <Icons name="copy" size={12} class="text-[#00f2fe]" />
                <span>Copy Tunnel URL</span>
              </button>
            {/if}

            <button
              onclick={() => {
                onDeploy(service);
                showMenu = false;
              }}
              class="w-full flex items-center gap-2 px-3 py-1.5 rounded-full text-tx-body hover:text-tx-header hover:bg-white/[0.06] transition-colors text-left"
            >
              <Icons name="rocket" size={12} class="text-[#8b5cf6]" />
              <span>Redeploy Target</span>
            </button>

            {#if service.status === 'dormant'}
              <button
                onclick={() => {
                  onResume(service);
                  showMenu = false;
                }}
                class="w-full flex items-center gap-2 px-3 py-1.5 rounded-full text-emerald-400 hover:text-emerald-300 hover:bg-white/[0.06] transition-colors text-left"
              >
                <Icons name="play" size={12} />
                <span>Resume Service</span>
              </button>
            {:else}
              <button
                onclick={() => {
                  onSuspend(service);
                  showMenu = false;
                }}
                class="w-full flex items-center gap-2 px-3 py-1.5 rounded-full text-amber-400 hover:text-amber-300 hover:bg-white/[0.06] transition-colors text-left"
              >
                <Icons name="pause" size={12} />
                <span>Suspend Service</span>
              </button>
            {/if}

            <div class="h-[1px] bg-white/[0.08] my-1"></div>

            {#if isAdmin}
              <button
                onclick={() => {
                  onDelete(service.id);
                  showMenu = false;
                }}
                class="w-full flex items-center gap-2 px-3 py-1.5 rounded-full text-rose-400 hover:text-rose-300 hover:bg-rose-500/10 transition-colors text-left"
              >
                <Icons name="trash" size={12} />
                <span>Remove Target</span>
              </button>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
