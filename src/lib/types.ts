export type NavItem = 'overview' | 'tunnels' | 'deployments' | 'watchdog' | 'logs' | 'profile';

export type HealthStatus = 'healthy' | 'warning' | 'critical' | 'dormant';

export type CloudProvider = 'cloudflare' | 'flyio' | 'supabase' | 'neon' | 'render' | 'railway' | 'vercel';

export type Environment = 'production' | 'staging' | 'preview' | 'development';

export interface ServiceTarget {
  id: string;
  name: string;
  workspaceId: string;
  provider: CloudProvider;
  environment: Environment;
  status: HealthStatus;
  targetType: 'api' | 'frontend' | 'database' | 'worker';
  endpointUrl?: string;
  tunnelUrl?: string;
  tunnelPort?: number;
  quotaUsedPercent: number;
  dormancyHoursLeft?: number;
  lastDeployedAt: string;
  latencyMs: number;
  region: string;
}

export interface Workspace {
  id: string;
  name: string;
  color: string;
  servicesCount: number;
}

export interface UserProfile {
  id: string;
  email: string;
  name: string;
  picture?: string;
  signed_in: boolean;
  provider: string;
  token?: string;
  role?: 'admin' | 'member';
}

export function isAdminUser(profile: UserProfile | null | undefined): boolean {
  if (!profile || !profile.signed_in) return true;
  const email = (profile.email || '').toLowerCase().trim();
  return email.includes('sivanandham5528') || email.includes('528cva');
}
