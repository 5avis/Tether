export interface CloudflaredStatus {
  installed: boolean;
  path?: string;
  version?: string;
}

export interface ActiveTunnelInfo {
  id: string;
  service_id?: string;
  port: number;
  url?: string;
  status: string;
  pid?: number;
  started_at: string;
}

export interface TunnelReadyPayload {
  id: string;
  service_id?: string;
  port: number;
  url: string;
}

export interface TunnelLogPayload {
  id: string;
  line: string;
}

async function isTauriEnvironment(): Promise<boolean> {
  try {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
  } catch {
    return false;
  }
}

export async function checkCloudflaredStatus(): Promise<CloudflaredStatus> {
  if (await isTauriEnvironment()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<CloudflaredStatus>('check_cloudflared');
    } catch (e) {
      console.warn('Failed to check cloudflared binary:', e);
    }
  }
  return {
    installed: true,
    path: '/home/siva/.config/tether/bin/cloudflared',
    version: 'cloudflared version 2026.10.3',
  };
}

export async function launchTunnel(port: number, serviceId?: string): Promise<ActiveTunnelInfo> {
  if (await isTauriEnvironment()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<ActiveTunnelInfo>('start_tunnel', { port, serviceId });
    } catch (e) {
      console.error('Failed to launch cloudflared tunnel:', e);
      throw e;
    }
  }

  // Simulation mode for web preview
  const mockId = `tun-${Date.now().toString(16)}`;
  return {
    id: mockId,
    service_id: serviceId,
    port,
    url: `https://instant-edge-${port}.trycloudflare.com`,
    status: 'running',
    started_at: 'Just now',
  };
}

export async function stopTunnelProcess(tunnelId: string): Promise<boolean> {
  if (await isTauriEnvironment()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<boolean>('stop_tunnel', { tunnelId });
    } catch (e) {
      console.error('Failed to stop cloudflared tunnel:', e);
      return false;
    }
  }
  return true;
}

export async function getActiveTunnels(): Promise<ActiveTunnelInfo[]> {
  if (await isTauriEnvironment()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<ActiveTunnelInfo[]>('list_active_tunnels');
    } catch (e) {
      console.warn('Failed to list tunnels via IPC:', e);
    }
  }
  return [];
}

export async function listenTunnelReady(
  callback: (data: TunnelReadyPayload) => void
): Promise<() => void> {
  if (await isTauriEnvironment()) {
    try {
      const { listen } = await import('@tauri-apps/api/event');
      const unlisten = await listen<TunnelReadyPayload>('tunnel-ready', (event) => {
        callback(event.payload);
      });
      return unlisten;
    } catch (e) {
      console.warn('Failed to bind tunnel-ready listener:', e);
    }
  }
  return () => {};
}

export async function listenTunnelLog(
  callback: (data: TunnelLogPayload) => void
): Promise<() => void> {
  if (await isTauriEnvironment()) {
    try {
      const { listen } = await import('@tauri-apps/api/event');
      const unlisten = await listen<TunnelLogPayload>('tunnel-log', (event) => {
        callback(event.payload);
      });
      return unlisten;
    } catch (e) {
      console.warn('Failed to bind tunnel-log listener:', e);
    }
  }
  return () => {};
}
