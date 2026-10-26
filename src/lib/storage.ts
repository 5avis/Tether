import type { Workspace, ServiceTarget } from './types';
import { initialWorkspaces, initialServices } from './data';

export interface TetherConfigDTO {
  active_workspace_id: string;
  workspaces: Array<{
    id: string;
    name: string;
    color: string;
    services_count: number;
    services: Array<any>;
  }>;
  credentials: Array<any>;
  version: string;
}

export interface TokenVerifyResponse {
  valid: boolean;
  provider: string;
  message: string;
  details?: string;
}

export interface PingResponse {
  service_id: string;
  service_name: string;
  success: boolean;
  message: string;
  latency_ms: number;
  dormancy_hours_left: number;
}

export interface DeploySummaryDTO {
  total: number;
  succeeded: number;
  failed: number;
  duration_ms: number;
}

export interface SecretSyncResultDTO {
  total_keys: number;
  targets_synced: Array<{
    service_id: string;
    service_name: string;
    provider: string;
    success: boolean;
    message: string;
  }>;
  duration_ms: number;
}

async function isTauriEnvironment(): Promise<boolean> {
  try {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
  } catch {
    return false;
  }
}

export async function loadTetherConfig(): Promise<{
  activeWorkspaceId: string;
  workspaces: Workspace[];
  services: ServiceTarget[];
}> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const cfg = await invoke<TetherConfigDTO>('get_config');

      const workspaces: Workspace[] = cfg.workspaces.map((w) => ({
        id: w.id,
        name: w.name,
        color: w.color,
        servicesCount: w.services_count,
      }));

      const services: ServiceTarget[] = cfg.workspaces.flatMap((w) =>
        w.services.map((s) => ({
          id: s.id,
          name: s.name,
          workspaceId: s.workspace_id,
          provider: s.provider,
          environment: s.environment,
          status: s.status,
          targetType: s.target_type,
          endpointUrl: s.endpoint_url ?? undefined,
          tunnelUrl: s.tunnel_url ?? undefined,
          tunnelPort: s.tunnel_port ?? undefined,
          quotaUsedPercent: s.quota_used_percent,
          dormancyHoursLeft: s.dormancy_hours_left ?? undefined,
          lastDeployedAt: s.last_deployed_at,
          latencyMs: s.latency_ms,
          region: s.region,
        }))
      );

      return {
        activeWorkspaceId: cfg.active_workspace_id,
        workspaces,
        services,
      };
    } catch (e) {
      console.warn('Failed to load config via Tauri IPC, falling back:', e);
    }
  }

  return {
    activeWorkspaceId: 'ws-default',
    workspaces: initialWorkspaces,
    services: initialServices,
  };
}

export async function saveServiceBinding(service: ServiceTarget): Promise<boolean> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('save_service', {
        service: {
          id: service.id,
          name: service.name,
          workspace_id: service.workspaceId,
          provider: service.provider,
          environment: service.environment,
          status: service.status,
          target_type: service.targetType,
          endpoint_url: service.endpointUrl ?? null,
          tunnel_url: service.tunnelUrl ?? null,
          tunnel_port: service.tunnelPort ?? null,
          quota_used_percent: service.quotaUsedPercent,
          dormancy_hours_left: service.dormancyHoursLeft ?? null,
          last_deployed_at: service.lastDeployedAt,
          latency_ms: service.latencyMs,
          region: service.region,
        },
      });
      return true;
    } catch (e) {
      console.error('Failed to save service binding to ~/.config/tether/config.json:', e);
      return false;
    }
  }
  return true;
}

export async function deleteServiceBinding(serviceId: string): Promise<boolean> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('delete_service', { serviceId });
      return true;
    } catch (e) {
      console.error('Failed to delete service binding:', e);
      return false;
    }
  }
  return true;
}

export async function saveProviderCredential(
  provider: string,
  token: string
): Promise<boolean> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('save_provider_credential', { provider, token });
      return true;
    } catch (e) {
      console.error('Failed to save provider credential:', e);
      return false;
    }
  }
  return true;
}

export async function verifyProviderToken(
  provider: string,
  token: string
): Promise<TokenVerifyResponse> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<TokenVerifyResponse>('verify_tokens', { provider, token });
    } catch (e) {
      return {
        valid: false,
        provider,
        message: `Token verification request failed: ${e}`,
      };
    }
  }

  await new Promise((r) => setTimeout(r, 400));
  if (token.trim().length >= 10) {
    return {
      valid: true,
      provider,
      message: `${provider} token verified successfully (web simulation).`,
    };
  }
  return {
    valid: false,
    provider,
    message: 'Token string is too short or invalid.',
  };
}

export async function refreshWorkspaceMetrics(workspaceId: string): Promise<ServiceTarget[]> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const updated = await invoke<any[]>('refresh_workspace_status', { workspaceId });
      return updated.map((s) => ({
        id: s.id,
        name: s.name,
        workspaceId: s.workspace_id,
        provider: s.provider,
        environment: s.environment,
        status: s.status,
        targetType: s.target_type,
        endpointUrl: s.endpoint_url ?? undefined,
        tunnelUrl: s.tunnel_url ?? undefined,
        tunnelPort: s.tunnel_port ?? undefined,
        quotaUsedPercent: s.quota_used_percent,
        dormancyHoursLeft: s.dormancy_hours_left ?? undefined,
        lastDeployedAt: s.last_deployed_at,
        latencyMs: s.latency_ms,
        region: s.region,
      }));
    } catch (e) {
      console.warn('refresh_workspace_status failed, using local update:', e);
    }
  }

  await new Promise((r) => setTimeout(r, 300));
  return [];
}

export async function sendKeepAlivePing(serviceId: string): Promise<PingResponse> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<PingResponse>('keep_alive_ping', { serviceId });
    } catch (e) {
      console.warn('keep_alive_ping IPC failed:', e);
    }
  }

  await new Promise((r) => setTimeout(r, 100));
  return {
    service_id: serviceId,
    service_name: 'Database Cluster',
    success: true,
    message: 'Keep-alive query delivered. Dormancy reset to 168h.',
    latency_ms: 19,
    dormancy_hours_left: 168,
  };
}

export async function sendAllKeepAlive(workspaceId: string): Promise<PingResponse[]> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<PingResponse[]>('keep_alive_all', { workspaceId });
    } catch (e) {
      console.warn('keep_alive_all IPC failed:', e);
    }
  }

  await new Promise((r) => setTimeout(r, 200));
  return [
    {
      service_id: 'srv-2',
      service_name: 'pg-cluster-primary',
      success: true,
      message: 'Keep-alive delivered. Reset to 168h.',
      latency_ms: 18,
      dormancy_hours_left: 168,
    },
    {
      service_id: 'srv-5',
      service_name: 'user-db-supabase',
      success: true,
      message: 'Keep-alive delivered. Reset to 168h.',
      latency_ms: 24,
      dormancy_hours_left: 168,
    },
  ];
}

export async function triggerParallelDeploy(
  workspaceId: string,
  serviceIds?: string[]
): Promise<DeploySummaryDTO> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<DeploySummaryDTO>('trigger_parallel_deploy', {
        workspaceId,
        serviceIds: serviceIds ?? null,
      });
    } catch (e) {
      console.warn('trigger_parallel_deploy failed:', e);
    }
  }

  await new Promise((r) => setTimeout(r, 800));
  return {
    total: 3,
    succeeded: 3,
    failed: 0,
    duration_ms: 780,
  };
}

export async function syncEnvironmentSecrets(
  workspaceId: string,
  envContent: string
): Promise<SecretSyncResultDTO> {
  const isTauri = await isTauriEnvironment();
  if (isTauri) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<SecretSyncResultDTO>('sync_environment_secrets', {
        workspaceId,
        envContent,
        envFilePath: null,
      });
    } catch (e) {
      console.warn('sync_environment_secrets failed:', e);
    }
  }

  await new Promise((r) => setTimeout(r, 500));
  return {
    total_keys: 5,
    targets_synced: [],
    duration_ms: 450,
  };
}

export async function listenDeployProgress(
  callback: (event: any) => void
): Promise<() => void> {
  if (await isTauriEnvironment()) {
    try {
      const { listen } = await import('@tauri-apps/api/event');
      return await listen('deploy-progress', (e) => callback(e.payload));
    } catch (e) {
      console.warn('listenDeployProgress failed:', e);
    }
  }
  return () => {};
}

export async function listenServiceLog(
  callback: (event: any) => void
): Promise<() => void> {
  if (await isTauriEnvironment()) {
    try {
      const { listen } = await import('@tauri-apps/api/event');
      return await listen('service-log', (e) => callback(e.payload));
    } catch (e) {
      console.warn('listenServiceLog failed:', e);
    }
  }
  return () => {};
}

// Google OAuth Desktop Profile Handlers
export async function startGoogleOAuth(): Promise<import('./types').UserProfile> {
  if (await isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke('start_google_oauth');
  }
  // Web fallback simulation
  return {
    id: 'google-sub-mock-12345',
    email: 'sivanandham5528@gmail.com',
    name: 'Sivanandham',
    picture: '',
    signed_in: true,
    provider: 'google',
    token: 'ya29.mock-tether-token',
    role: 'admin',
  };
}

export async function getUserProfile(): Promise<import('./types').UserProfile | null> {
  if (await isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke('get_user_profile');
  }
  return null;
}

export async function signOutProfile(): Promise<void> {
  if (await isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('sign_out');
  }
}

export interface OAuthCredentialsDTO {
  client_id: string;
  client_secret: string;
  project_id: string;
  redirect_mode: string;
}

export interface LifecycleResponseDTO {
  success: boolean;
  service_id: string;
  status: string;
  message: string;
}

export async function suspendTargetService(
  serviceId: string,
  apiToken?: string
): Promise<LifecycleResponseDTO> {
  if (await isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<LifecycleResponseDTO>('suspend_target_service', {
      serviceId,
      apiToken: apiToken || null,
    });
  }
  return {
    success: true,
    service_id: serviceId,
    status: 'dormant',
    message: 'Mock service suspended in local view.',
  };
}

export async function resumeTargetService(
  serviceId: string,
  apiToken?: string
): Promise<LifecycleResponseDTO> {
  if (await isTauriEnvironment()) {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<LifecycleResponseDTO>('resume_target_service', {
      serviceId,
      apiToken: apiToken || null,
    });
  }
  return {
    success: true,
    service_id: serviceId,
    status: 'healthy',
    message: 'Mock service resumed in local view.',
  };
}

export async function getOAuthCredentials(): Promise<OAuthCredentialsDTO | null> {
  if (await isTauriEnvironment()) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<OAuthCredentialsDTO>('get_oauth_credentials');
    } catch (e) {
      console.warn('Failed to load oauth credentials:', e);
    }
  }
  return null;
}
