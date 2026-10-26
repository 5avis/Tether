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

<div class="flex-1 flex flex-col min-h-0 bg-transparent text-xs select-none">
  <!-- Top Control Banner with Finished Hero Box -->
  <div class="p-4 sm:p-6 border-b border-white/[0.08] bg-white/[0.02] backdrop-blur-md">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2 flex-wrap">
          <h1 class="text-base font-semibold text-white tracking-tight">Tunnels</h1>
          {#if cloudflared.installed}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08] flex items-center gap-1.5">
              <span class="w-1.5 h-1.5 rounded-full bg-[#00f2fe] shadow-[0_0_6px_#00f2fe]"></span>
              cloudflared
            </span>
          {:else}
            <span class="px-3 py-1 rounded-full text-[10px] font-mono bg-[#ff5e62]/15 text-[#ff5e62] border border-[#ff5e62]/30">
              cloudflared missing
            </span>
          {/if}
        </div>
        <p class="text-xs text-[#94a3b8] mt-1">
          Instant public preview URLs via <code class="text-[#00f2fe] font-mono">trycloudflare.com</code>.
        </p>
      </div>

      <!-- Quick Launch Box -->
      <div class="flex items-center gap-2 p-1.5 rounded-full bg-white/[0.03] backdrop-blur-md border border-white/[0.08] shadow-subtle flex-wrap sm:flex-nowrap">
        <div class="flex items-center bg-white/[0.04] rounded-full border border-white/[0.08] focus-within:border-[#00f2fe]/60 px-3.5 py-1.5 transition-colors">
          <span class="text-[#94a3b8] font-mono text-xs">localhost:</span>
          <input
            type="number"
            bind:value={newPort}
            min="1024"
            max="65535"
            placeholder="3000"
            class="w-16 bg-transparent text-white font-mono px-1.5 focus:outline-none text-xs"
          />
        </div>

        <button
          onclick={handleStart}
          disabled={isLaunching}
          class="px-5 py-2 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-semibold text-xs shadow-[0_0_20px_rgba(139,92,246,0.35)] flex items-center gap-2 disabled:opacity-50 active:scale-[0.98] shrink-0 transition-all"
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
  <div class="flex-1 overflow-y-auto p-2 sm:p-4">
    {#if tunnels.length > 0}
      <div class="px-3 py-1.5 text-[10px] uppercase font-mono tracking-wider text-[#94a3b8] flex items-center justify-between">
        <span>Active Process Handles ({tunnels.length})</span>
        <span>Local Port • Public Endpoint • Edge Status</span>
      </div>

      {#each tunnels as tun (tun.id)}
        <div
          class="group mx-1 my-2.5 p-4 rounded-3xl bg-white/[0.03] hover:bg-white/[0.06] backdrop-blur-xl border border-white/[0.08] hover:border-[#8b5cf6]/40 hover:shadow-[0_0_20px_rgba(139,92,246,0.18)] transition-all flex flex-col sm:flex-row sm:items-center justify-between gap-3.5"
        >
          <!-- Left: Circular status indicator & Port info -->
          <div class="flex items-center gap-3.5 min-w-0">
            <!-- Circular Indicator Dot -->
            <div class="relative flex items-center justify-center shrink-0 w-4 h-4">
              <span
                class="w-2.5 h-2.5 rounded-full {tun.status === 'running'
                  ? 'bg-[#00f2fe] shadow-[0_0_8px_#00f2fe]'
                  : 'bg-[#ff5e62]'} transition-transform group-hover:scale-110"
              ></span>
              {#if tun.status === 'running'}
                <span class="absolute w-4 h-4 rounded-full bg-[#00f2fe]/30 animate-ping"></span>
              {/if}
            </div>

            <!-- Identity and Port info -->
            <div class="flex flex-col gap-0.5 min-w-0">
              <div class="flex items-center gap-2 flex-wrap">
                <span class="font-mono text-xs text-white font-semibold">
                  http://localhost:{tun.port}
                </span>

                {#if tun.pid}
                  <span class="text-[10px] px-2.5 py-0.5 rounded-full font-mono bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]">
                    PID {tun.pid}
                  </span>
                {/if}

                <span
                  class="text-[10px] uppercase font-mono tracking-wider px-2.5 py-0.5 rounded-full bg-white/[0.05] text-[#94a3b8] border border-white/[0.08]"
                >
                  {tun.status}
                </span>
              </div>

              <!-- Sub-metadata -->
              <div class="flex items-center gap-2 text-[11px] text-[#94a3b8] mt-0.5">
                <span>Anycast Edge</span>
                <span class="text-white/20">•</span>
                <span>Subprocess Managed</span>
                <span class="text-white/20">•</span>
                <span>{tun.started_at}</span>
              </div>
            </div>
          </div>

          <!-- Center/Right: Active URL Pill with Copy, Open & Stop -->
          <div class="flex items-center gap-2 flex-wrap sm:flex-nowrap justify-between sm:justify-end">
            {#if tun.url}
              <div class="flex items-center bg-white/[0.04] border border-white/[0.08] hover:border-[#00f2fe]/40 rounded-full px-3.5 py-1.5 text-white font-mono text-xs max-w-sm truncate transition-colors">
                <Icons name="globe" size={12} class="shrink-0 mr-1.5 text-[#00f2fe]" />
                <span class="truncate">{tun.url}</span>
              </div>

              <!-- Action button: Copy URL -->
              <button
                onclick={() => handleCopy(tun.id, tun.url!)}
                class="w-7 h-7 rounded-full hover:bg-white/[0.08] flex items-center justify-center text-[#94a3b8] hover:text-white transition-colors"
                title="Copy public URL"
              >
                <Icons name={copiedId === tun.id ? 'check' : 'copy'} size={13} class={copiedId === tun.id ? 'text-[#00f2fe]' : ''} />
              </button>

              <!-- Action button: Open URL in Browser -->
              <button
                onclick={() => handleOpenUrl(tun.url!)}
                class="w-7 h-7 rounded-full hover:bg-white/[0.08] flex items-center justify-center text-[#94a3b8] hover:text-white transition-colors"
                title="Open preview in browser"
              >
                <Icons name="external-link" size={13} />
              </button>
            {:else}
              <div class="flex items-center gap-2 text-[#00f2fe] font-mono text-xs">
                <Icons name="refresh" size={12} class="animate-spin text-[#00f2fe]" />
                <span>Negotiating trycloudflare.com tunnel route...</span>
              </div>
            {/if}

            <!-- Stop Process Button -->
            <button
              onclick={() => handleStop(tun.id)}
              class="ml-1 px-3.5 py-1.5 rounded-full bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 transition-all font-mono text-[11px] flex items-center gap-1.5 active:scale-[0.98]"
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
        <div class="w-14 h-14 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center mb-3 text-[#00f2fe] shadow-[0_0_20px_rgba(0,242,254,0.25)]">
          <Icons name="radio" size={24} />
        </div>
        <h3 class="text-sm font-semibold text-white mb-1 tracking-tight">No active preview tunnels</h3>
        <p class="text-xs text-[#94a3b8] max-w-sm mb-4 leading-relaxed">
          Create an instant public <code class="text-[#00f2fe]">trycloudflare.com</code> URL for local servers.
        </p>
        <button
          onclick={() => {
            newPort = 3000;
            handleStart();
          }}
          class="px-5 py-2.5 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white text-xs font-semibold shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all border border-transparent active:scale-[0.98] flex items-center gap-2"
        >
          <Icons name="plus" size={13} strokeWidth={2.5} />
          <span>Launch Tunnel on :3000</span>
        </button>
      </div>
    {/if}
  </div>
</div>
