<script lang="ts">
  import Icons from './Icons.svelte';
  import type { ActiveTunnelInfo, CloudflaredStatus } from '$lib/tunnel';
  import { launchTunnel, stopTunnelProcess, checkCloudflaredStatus } from '$lib/tunnel';
  import { onMount } from 'svelte';

  let {
    tunnels = [],
    isAdmin = false,
    onTunnelCreated = (tunnel: ActiveTunnelInfo) => {},
    onTunnelStopped = (id: string) => {},
    onToast = (msg: string) => {},
  }: {
    tunnels: ActiveTunnelInfo[];
    isAdmin?: boolean;
    onTunnelCreated?: (tunnel: ActiveTunnelInfo) => void;
    onTunnelStopped?: (id: string) => void;
    onToast?: (msg: string) => void;
  } = $props();

  let newPort = $state<number>(3000);
  let isLaunching = $state(false);
  let cloudflared = $state<CloudflaredStatus>({ installed: false });
  let copiedId = $state<string | null>(null);

  onMount(async () => {
    cloudflared = await checkCloudflaredStatus();
  });

  async function handleStart() {
    if (!newPort || newPort < 1 || newPort > 65535) {
      onToast('Please enter a valid port between 1 and 65535');
      return;
    }
    isLaunching = true;
    onToast(`Spawning cloudflared for localhost:${newPort}...`);
    try {
      const tun = await launchTunnel(Number(newPort));
      onTunnelCreated(tun);
      onToast(`Tunnel spawned for port ${newPort}. Waiting for Cloudflare edge route...`);
    } catch (e: any) {
      onToast(`Failed to launch tunnel: ${e}`);
    } finally {
      isLaunching = false;
    }
  }

  async function handleStop(id: string) {
    onToast('Terminating cloudflared process...');
    const ok = await stopTunnelProcess(id);
    if (ok) {
      onTunnelStopped(id);
      onToast('Tunnel process terminated gracefully.');
    }
  }

  function handleCopy(id: string, url: string) {
    navigator.clipboard?.writeText(url);
    copiedId = id;
    onToast(`Copied to clipboard: ${url}`);
    setTimeout(() => {
      copiedId = null;
    }, 1800);
  }

  async function handleOpenUrl(url: string) {
    try {
      const { openUrl } = await import('@tauri-apps/plugin-opener');
      await openUrl(url);
    } catch {
      window.open(url, '_blank');
    }
  }
</script>

<div class="flex-1 flex flex-col min-h-0 bg-canvas text-xs select-none">
  <!-- Top Control Banner with Finished Hero Box -->
  <div class="p-4 sm:p-6 border-b border-border-dim bg-surface/30">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2 flex-wrap">
          <h1 class="text-base font-semibold text-tx-header tracking-tight">Tunnels</h1>
          {#if cloudflared.installed}
            <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-tx-header border border-border-dim flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-coral"></span>
              cloudflared
            </span>
          {:else}
            <span class="px-2 py-0.5 rounded-full text-[10px] font-mono bg-surface-elevated text-coral border border-border-dim">
              cloudflared missing
            </span>
          {/if}
        </div>
        <p class="text-xs text-tx-meta mt-1">
          Instant public preview URLs via <code class="text-coral font-mono">trycloudflare.com</code>.
        </p>
      </div>

      <!-- Quick Launch Box -->
      <div class="flex items-center gap-2 p-1.5 rounded-xl bg-surface border border-border-dim shadow-subtle flex-wrap sm:flex-nowrap">
        <div class="flex items-center bg-surface-elevated rounded-lg border border-border-dim/80 focus-within:border-coral px-3 py-1.5">
          <span class="text-tx-meta font-mono text-xs">localhost:</span>
          <input
            type="number"
            bind:value={newPort}
            min="1024"
            max="65535"
            placeholder="3000"
            class="w-16 bg-transparent text-tx-header font-mono px-1.5 focus:outline-none text-xs"
          />
        </div>

        <button
          onclick={handleStart}
          disabled={isLaunching}
          class="px-4 py-1.5 rounded-xl bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white font-medium text-xs shadow-sm transition-all border border-[#c084fc]/30 flex items-center gap-1.5 disabled:opacity-50 active:scale-[0.98] shrink-0 shadow-[0_0_12px_rgba(168,85,247,0.25)]"
        >
          {#if isLaunching}
            <Icons name="refresh" size={13} class="animate-spin" />
            <span>Establishing...</span>
          {:else}
            <Icons name="plus" size={13} strokeWidth={2.5} />
            <span>Launch Tunnel</span>
          {/if}
        </button>
      </div>
    </div>
  </div>

  <!-- Tunnel Row Items as Finished Cards -->
  <div class="flex-1 overflow-y-auto p-2 sm:p-3">
    {#if tunnels.length > 0}
      <div class="px-3 py-1.5 text-[10px] uppercase font-mono tracking-wider text-tx-meta flex items-center justify-between">
        <span>Active Process Handles ({tunnels.length})</span>
        <span>Local Port • Public Endpoint • Edge Status</span>
      </div>

      {#each tunnels as tun (tun.id)}
        <div
          class="group mx-1 my-2 p-3.5 rounded-2xl bg-surface/60 hover:bg-surface-elevated/70 border border-border-dim hover:border-border-subtle transition-all shadow-subtle flex flex-col sm:flex-row sm:items-center justify-between gap-3"
        >
          <!-- Left: Circular status indicator & Port info -->
          <div class="flex items-center gap-3.5 min-w-0">
            <!-- Circular Indicator Dot -->
            <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
              <span
                class="w-2 rounded-full {tun.status === 'running'
                  ? 'bg-tx-header ring-2 ring-border-dim'
                  : 'bg-coral'} transition-transform group-hover:scale-110"
              ></span>
              {#if tun.status === 'running'}
                <span class="absolute w-4 h-4 rounded-full bg-coral/20 animate-ping"></span>
              {/if}
            </div>

            <!-- Identity and Port info -->
            <div class="flex flex-col gap-0.5 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="font-mono text-xs text-tx-header font-semibold">
                  http://localhost:{tun.port}
                </span>

                {#if tun.pid}
                  <span class="text-[10px] px-1.5 py-0.2 rounded font-mono bg-surface-elevated text-tx-meta border border-border-dim">
                    PID {tun.pid}
                  </span>
                {/if}

                <span
                  class="text-[10px] uppercase font-mono tracking-wider px-1.5 py-0.2 rounded bg-surface-elevated text-tx-meta border border-border-dim"
                >
                  {tun.status}
                </span>
              </div>

              <!-- Sub-metadata -->
              <div class="flex items-center gap-2 text-[11px] text-tx-meta">
                <span>Anycast Edge</span>
                <span class="text-border-subtle">•</span>
                <span>Subprocess Managed</span>
                <span class="text-border-subtle">•</span>
                <span>{tun.started_at}</span>
              </div>
            </div>
          </div>

          <!-- Center/Right: Active URL Pill with Copy, Open & Stop -->
          <div class="flex items-center gap-2 flex-wrap sm:flex-nowrap justify-between sm:justify-end">
            {#if tun.url}
              <div class="flex items-center bg-surface-elevated border border-border-dim hover:border-coral/50 rounded-lg px-2.5 py-1 text-tx-header font-mono text-xs max-w-sm truncate transition-colors">
                <Icons name="globe" size={12} class="shrink-0 mr-1.5 text-coral" />
                <span class="truncate">{tun.url}</span>
              </div>

              <!-- Action button: Copy URL -->
              <button
                onclick={() => handleCopy(tun.id, tun.url!)}
                class="p-1.5 rounded-lg hover:bg-surface-elevated text-tx-body hover:text-tx-header border border-transparent hover:border-border-dim transition-colors"
                title="Copy public URL"
              >
                <Icons name={copiedId === tun.id ? 'check' : 'copy'} size={13} class={copiedId === tun.id ? 'text-coral' : ''} />
              </button>

              <!-- Action button: Open URL in Browser -->
              <button
                onclick={() => handleOpenUrl(tun.url!)}
                class="p-1.5 rounded-lg hover:bg-surface-elevated text-tx-body hover:text-tx-header border border-transparent hover:border-border-dim transition-colors"
                title="Open preview in browser"
              >
                <Icons name="external-link" size={13} />
              </button>
            {:else}
              <div class="flex items-center gap-2 text-coral font-mono text-xs">
                <Icons name="refresh" size={12} class="animate-spin text-coral" />
                <span>Negotiating trycloudflare.com tunnel route...</span>
              </div>
            {/if}

            <!-- Stop Process Button -->
            <button
              onclick={() => handleStop(tun.id)}
              class="ml-1 px-2.5 py-1 rounded-lg bg-surface-elevated hover:bg-rose-950/40 text-tx-body hover:text-rose-400 border border-border-dim hover:border-rose-500/40 transition-all font-mono text-[11px] flex items-center gap-1 active:scale-[0.98]"
              title="Stop process"
            >
              <Icons name="power" size={11} class="text-rose-400" />
              <span>Stop</span>
            </button>
          </div>
        </div>
      {/each}
    {:else}
      <!-- Empty State -->
      <div class="h-64 flex flex-col items-center justify-center p-8 text-center select-none animate-in fade-in duration-200">
        <div class="w-14 h-14 rounded-2xl bg-surface-elevated border border-border-dim flex items-center justify-center mb-3 text-coral shadow-subtle">
          <Icons name="radio" size={24} />
        </div>
        <h3 class="text-sm font-semibold text-tx-header mb-1 tracking-tight">No active preview tunnels</h3>
        <p class="text-xs text-tx-meta max-w-sm mb-4 leading-relaxed">
          Create an instant public <code class="text-coral">trycloudflare.com</code> URL for local servers.
        </p>
        <button
          onclick={() => {
            newPort = 3000;
            handleStart();
          }}
          class="px-4 py-2 rounded-xl bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white text-xs font-medium shadow-sm transition-all border border-[#c084fc]/30 active:scale-[0.98] flex items-center gap-1.5 shadow-[0_0_12px_rgba(168,85,247,0.25)]"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>Launch Tunnel on :3000</span>
        </button>
      </div>
    {/if}
  </div>
</div>
