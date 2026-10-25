import type { ServiceTarget, Workspace } from './types';

export const initialWorkspaces: Workspace[] = [
  { id: 'ws-default', name: 'Main Workspace', color: '#e25c48', servicesCount: 0 },
];

export const initialServices: ServiceTarget[] = [];
