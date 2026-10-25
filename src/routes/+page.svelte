<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import TitleBar from '$lib/components/TitleBar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import FilterBar from '$lib/components/FilterBar.svelte';
  import ServiceRow from '$lib/components/ServiceRow.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import AddServiceModal from '$lib/components/AddServiceModal.svelte';
  import TunnelView from '$lib/components/TunnelView.svelte';
  import WatchdogView from '$lib/components/WatchdogView.svelte';
  import DeploymentsView, { type DeployJobState } from '$lib/components/DeploymentsView.svelte';
  import LiveLogsView, { type LogEntry } from '$lib/components/LiveLogsView.svelte';
  import LogDrawer from '$lib/components/LogDrawer.svelte';
  import SecretSyncModal from '$lib/components/SecretSyncModal.svelte';
  import ProfileModal from '$lib/components/ProfileModal.svelte';
  import ProfileView from '$lib/components/ProfileView.svelte';
  import Icons from '$lib/components/Icons.svelte';
  import {
    loadTetherConfig,
    saveServiceBinding,
    deleteServiceBinding,
    refreshWorkspaceMetrics,
    sendKeepAlivePing,
    sendAllKeepAlive,
    triggerParallelDeploy,
    syncEnvironmentSecrets,
    listenDeployProgress,
    listenServiceLog,
    getUserProfile,
    signOutProfile,
  } from '$lib/storage';
  import {
    listenTunnelReady,
    listenTunnelLog,
    getActiveTunnels,
    type ActiveTunnelInfo,
  } from '$lib/tunnel';
  import { initialWorkspaces, initialServices } from '$lib/data';
  import { type NavItem, type ServiceTarget, type Workspace, type UserProfile, isAdminUser } from '$lib/types';

  // State Management (Svelte 5 Runes)
  let activeNav = $state<NavItem>('overview');
  let activeWorkspaceId = $state<string>('ws-default');
  let workspaces = $state<Workspace[]>(initialWorkspaces);
  let services = $state<ServiceTarget[]>(initialServices);
  let userProfile = $state<UserProfile | null>(null);
  let activeTunnels = $state<ActiveTunnelInfo[]>([]);
  let deployJobs = $state<DeployJobState[]>([]);
  let logsList = $state<LogEntry[]>([]);
  let isProfileModalOpen = $state(false);
  let isLogDrawerOpen = $state(false);
  let isSecretSyncOpen = $state(false);
  let isDeploying = $state(false);
  let isSidebarCollapsed = $state(false);

  const isAdmin = $derived(isAdminUser(userProfile));

  const deployableCount = $derived(
    services.filter((s) => s.targetType === 'api' || s.targetType === 'frontend' || s.targetType === 'worker').length
  );

  const watchdogCount = $derived(
    services.filter((s) => s.targetType === 'database' || s.dormancyHoursLeft !== undefined || s.quotaUsedPercent > 50).length
  );

  const watchdogRemainingHours = $derived(
    (() => {
      const dbServices = services.filter((s) => s.dormancyHoursLeft !== undefined);
      if (dbServices.length === 0) return null;
      return Math.min(...dbServices.map((s) => s.dormancyHoursLeft ?? 168));
    })()
  );

  const navCounts = $derived({
    overview: services.length,
    tunnels: activeTunnels.length,
    deployments: deployableCount,
    watchdog: watchdogCount,
    logs: logsList.length,
  });

  async function handleDeleteService(serviceId: string) {
    if (!isAdmin) {
      showToast('Access restricted: Administrator privileges required.');
      return;
    }
    services = services.filter((s) => s.id !== serviceId);
    try {
      await deleteServiceBinding(serviceId);
      showToast(`Removed target "${serviceId}"`);
    } catch (e) {
      showToast(`Failed to remove target: ${e}`);
    }
  }

  let isLoading = $state(true);
  let zoomLevel = $state(1.0);

  function applyZoom(newZoom: number) {
    const clamped = Math.round(Math.min(Math.max(newZoom, 0.75), 1.6) * 20) / 20;
    zoomLevel = clamped;
    // CRITICAL: NEVER use style.zoom on documentElement or body.
    // CSS zoom breaks WebKitGTK viewport calculations and creates an empty void around the screen (as seen in 8.png).
    document.documentElement.style.zoom = '';
    if (document.body) document.body.style.zoom = '';
    // Scale root font size (16px base) so all rem-based typography, bars, paddings, inputs and icons scale seamlessly
    // while the window/screen ALWAYS remains 100% full width and full height!
    document.documentElement.style.fontSize = `${clamped * 16}px`;
    try {
      localStorage.setItem('tether_ui_zoom', String(clamped));
    } catch {}
    showToast(`UI Scaling: ${Math.round(clamped * 100)}%`);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.ctrlKey || e.metaKey) {
      if (e.key === '=' || e.key === '+' || e.code === 'NumpadAdd' || e.code === 'Equal') {
        e.preventDefault();
        applyZoom(zoomLevel + 0.05);
      } else if (e.key === '-' || e.key === '_' || e.code === 'NumpadSubtract' || e.code === 'Minus') {
        e.preventDefault();
        applyZoom(zoomLevel - 0.05);
      } else if (e.key === '0' || e.code === 'Numpad0' || e.code === 'Digit0') {
        e.preventDefault();
        applyZoom(1.0);
      }
    }
  }

  function handleWheel(e: WheelEvent) {
    if (e.ctrlKey || e.metaKey) {
      e.preventDefault();
      if (e.deltaY < 0) {
        applyZoom(zoomLevel + 0.05);
      } else if (e.deltaY > 0) {
        applyZoom(zoomLevel - 0.05);
      }
    }
  }

  // Filter States
  let filterWorkspace = $state('all');
  let filterProvider = $state('all');
  let filterEnvironment = $state('all');
  let filterActiveOnly = $state(false);
  let filterSearch = $state('');

  // UI Modals & Actions
  let isAddModalOpen = $state(false);
  let isSyncing = $state(false);
  let toastMessage = $state<string | null>(null);
  let toastTimer: any = null;
  let unlistenFns: Array<() => void> = [];

  function showToast(msg: string) {
    if (toastTimer) clearTimeout(toastTimer);
    toastMessage = msg;
    toastTimer = setTimeout(() => {
      toastMessage = null;
    }, 2800);
  }

  onMount(async () => {
    // Purge any legacy style.zoom from previous sessions that caused the viewport to shrink (as in 8.png)
    document.documentElement.style.zoom = '';
    if (document.body) document.body.style.zoom = '';

    // Restore saved UI scaling if present
    try {
      const savedZoom = localStorage.getItem('tether_ui_zoom');
      if (savedZoom) {
        const parsed = parseFloat(savedZoom);
        if (!isNaN(parsed) && parsed >= 0.75 && parsed <= 1.6) {
          zoomLevel = parsed;
          document.documentElement.style.fontSize = `${parsed * 16}px`;
        } else {
          document.documentElement.style.fontSize = '16px';
        }
      } else {
        document.documentElement.style.fontSize = '16px';
      }
    } catch {
      document.documentElement.style.fontSize = '16px';
    }

    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('wheel', handleWheel, { passive: false });

    try {
      const cfg = await loadTetherConfig();
      if (cfg.workspaces && cfg.workspaces.length > 0) {
        workspaces = cfg.workspaces;
      }
      if (cfg.services && cfg.services.length > 0) {
        services = cfg.services;
      }
      if (cfg.activeWorkspaceId) {
        activeWorkspaceId = cfg.activeWorkspaceId;
      }

      // Load Google developer profile if connected
      const existingProfile = await getUserProfile();
      if (existingProfile) {
        userProfile = existingProfile;
      }

      // Check running tunnels
      const running = await getActiveTunnels();
      if (running && running.length > 0) {
        activeTunnels = running;
      }

      // Listen for tunnel-ready events
      const unlistenReady = await listenTunnelReady((payload) => {
        const existingIdx = activeTunnels.findIndex(
          (t) => t.id === payload.id || t.port === payload.port
        );
        if (existingIdx >= 0) {
          activeTunnels[existingIdx].url = payload.url;
          activeTunnels[existingIdx].status = 'running';
        } else {
          activeTunnels = [
            {
              id: payload.id,
              port: payload.port,
              url: payload.url,
              status: 'running',
              started_at: 'Just now',
            },
            ...activeTunnels,
          ];
        }

        const srvIdx = services.findIndex((s) => s.tunnelPort === payload.port);
        if (srvIdx >= 0) {
          services[srvIdx].tunnelUrl = payload.url;
          services[srvIdx].status = 'healthy';
        }

        showToast(`Tunnel Live: ${payload.url}`);
      });
      unlistenFns.push(unlistenReady);

      // Listen for tunnel logs
      const unlistenTunnelL = await listenTunnelLog((payload) => {
        const entry: LogEntry = {
          id: `tun-log-${Date.now()}-${Math.random()}`,
          timestamp: new Date().toLocaleTimeString(),
          tag: `tunnel:${payload.id.slice(0, 8)}`,
          serviceName: 'Quick Tunnel',
          provider: 'cloudflare',
          level: 'INFO',
          message: payload.line,
          color: payload.line.includes('ERR') ? 'text-status-critical' : 'text-tx-meta',
        };
        logsList = [entry, ...logsList.slice(0, 199)];
      });
      unlistenFns.push(unlistenTunnelL);

      // Listen for deploy progress events
      const unlistenDeploy = await listenDeployProgress((event) => {
        const idx = deployJobs.findIndex((j) => j.serviceId === event.service_id);
        const updated: DeployJobState = {
          serviceId: event.service_id,
          serviceName: event.service_name,
          provider: event.provider,
          status: event.status,
          progress: event.progress_percent,
          message: event.message,
          durationMs: event.duration_ms,
        };

        if (idx >= 0) {
          deployJobs[idx] = updated;
        } else {
          deployJobs = [...deployJobs, updated];
        }
      });
      unlistenFns.push(unlistenDeploy);

      // Listen for service multiplexed logs
      const unlistenServLogs = await listenServiceLog((event) => {
        const entry: LogEntry = {
          id: event.id || `serv-log-${Date.now()}`,
          timestamp: new Date().toLocaleTimeString(),
          tag: `${event.provider}:${event.service_name}`,
          serviceName: event.service_name,
          provider: event.provider,
          level: event.level,
          message: event.message,
          color: 'text-tx-header',
        };
        logsList = [entry, ...logsList.slice(0, 199)];
      });
      unlistenFns.push(unlistenServLogs);
    } catch (e) {
      console.warn('Error loading initial config from disk:', e);
    } finally {
      isLoading = false;
    }
  });

  onDestroy(() => {
    unlistenFns.forEach((fn) => fn());
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('wheel', handleWheel);
  });

  // Filter Handlers
  function handleFilterChange(filters: {
    workspace: string;
    provider: string;
    environment: string;
    activeOnly: boolean;
    search: string;
  }) {
    filterWorkspace = filters.workspace;
    filterProvider = filters.provider;
    filterEnvironment = filters.environment;
    filterActiveOnly = filters.activeOnly;
    filterSearch = filters.search.trim().toLowerCase();
  }

  function resetFilters() {
    filterWorkspace = 'all';
    filterProvider = 'all';
    filterEnvironment = 'all';
    filterActiveOnly = false;
    filterSearch = '';
  }

  // Filtered Services Computation
  const filteredServices = $derived(
    services.filter((s) => {
      if (filterWorkspace !== 'all' && s.workspaceId !== filterWorkspace) {
        return false;
      }
      if (filterProvider !== 'all' && s.provider !== filterProvider) {
        return false;
      }
      if (filterEnvironment !== 'all' && s.environment !== filterEnvironment) {
        return false;
      }
      if (filterActiveOnly && s.status !== 'healthy') {
        return false;
      }
      if (filterSearch) {
        const matchesName = s.name.toLowerCase().includes(filterSearch);
        const matchesProvider = s.provider.toLowerCase().includes(filterSearch);
        const matchesType = s.targetType.toLowerCase().includes(filterSearch);
        const matchesUrl = (s.tunnelUrl || s.endpointUrl || '').toLowerCase().includes(filterSearch);
        return matchesName || matchesProvider || matchesType || matchesUrl;
      }
      return true;
    })
  );

  const activeWorkspace = $derived(
    workspaces.find((w) => w.id === activeWorkspaceId) || workspaces[0]
  );

  async function handleSyncAll() {
    isSyncing = true;
    showToast('Refreshing multi-cloud metrics concurrently via tokio::join!...');
    try {
      const updated = await refreshWorkspaceMetrics(activeWorkspaceId);
      if (updated.length > 0) {
        services = updated;
      }
      showToast('All multi-cloud metrics synced from REST endpoints.');
    } finally {
      isSyncing = false;
    }
  }

  async function handleSingleKeepAlive(serviceId: string) {
    const res = await sendKeepAlivePing(serviceId);
    const srvIdx = services.findIndex((s) => s.id === serviceId);
    if (srvIdx >= 0) {
      services[srvIdx].dormancyHoursLeft = res.dormancy_hours_left;
      services[srvIdx].status = 'healthy';
      services[srvIdx].latencyMs = res.latency_ms;
    }
    showToast(res.message);
  }

  async function handleAllKeepAlive() {
    const results = await sendAllKeepAlive(activeWorkspaceId);
    for (const r of results) {
      const srvIdx = services.findIndex((s) => s.id === r.service_id);
      if (srvIdx >= 0) {
        services[srvIdx].dormancyHoursLeft = r.dormancy_hours_left;
        services[srvIdx].status = 'healthy';
        services[srvIdx].latencyMs = r.latency_ms;
      }
    }
    showToast(`Keep-alive heartbeat delivered to ${results.length} database targets.`);
  }

  async function handleSignOut() {
    try {
      await signOutProfile();
      userProfile = null;
      showToast('Signed out from Google account. Session closed.');
    } catch (e: any) {
      showToast(`Error signing out: ${e?.message || e}`);
    }
  }

  async function handleDeployAll() {
    if (!isAdmin) {
      showToast('Access restricted: Administrator privileges required.');
      return;
    }
    isDeploying = true;
    showToast('Triggering parallel deployment across all linked services...');
    try {
      const summary = await triggerParallelDeploy(activeWorkspaceId);
      showToast(`Parallel deploy complete: ${summary.succeeded}/${summary.total} targets live in ${summary.duration_ms}ms`);
    } finally {
      isDeploying = false;
    }
  }

  async function handleDeploySingle(service: ServiceTarget) {
    if (!isAdmin) {
      showToast('Access restricted: Administrator privileges required.');
      return;
    }
    showToast(`Triggering deploy pipeline for ${service.name}...`);
    await triggerParallelDeploy(activeWorkspaceId, [service.id]);
    showToast(`Target ${service.name} deployed successfully.`);
  }

  async function handleSecretSync(content: string) {
    if (!isAdmin) {
      showToast('Access restricted: Administrator privileges required.');
      return;
    }
    showToast('Pushing secrets across all cloud targets...');
    const result = await syncEnvironmentSecrets(activeWorkspaceId, content);
    showToast(`Synchronized ${result.total_keys} secrets in ${result.duration_ms}ms.`);
  }

  function handleViewLogs(service: ServiceTarget) {
    activeNav = 'logs';
    showToast(`Streaming logs for ${service.name}`);
  }

  function handleCopyUrl(url: string) {
    navigator.clipboard?.writeText(url);
    showToast(`Copied tunnel URL to clipboard: ${url}`);
  }

  async function handleAddServiceSave(newBinding: any) {
    if (!isAdmin) {
      showToast('Access restricted: Administrator privileges required.');
      return;
    }
    const created: ServiceTarget = {
      id: `srv-${Date.now()}`,
      name: newBinding.name,
      workspaceId: activeWorkspaceId,
      provider: newBinding.provider,
      environment: newBinding.environment,
      status: 'healthy',
      targetType: newBinding.targetType,
      tunnelPort: newBinding.tunnelPort,
      tunnelUrl:
        newBinding.provider === 'cloudflare'
          ? `https://${newBinding.name.toLowerCase().replace(/[^a-z0-9]/g, '-')}-preview.trycloudflare.com`
          : undefined,
      endpointUrl:
        newBinding.provider !== 'cloudflare'
          ? `https://${newBinding.name}.cloud.internal`
          : undefined,
      quotaUsedPercent: 12,
      lastDeployedAt: 'Just now',
      latencyMs: Math.floor(Math.random() * 20) + 15,
      region: 'auto-mesh',
    };

    services = [created, ...services];
    await saveServiceBinding(created);

    if (newBinding.provider === 'cloudflare' && newBinding.tunnelPort) {
      activeTunnels = [
        {
          id: `tun-${Date.now().toString(16)}`,
          port: Number(newBinding.tunnelPort),
          url: created.tunnelUrl,
          status: 'running',
          started_at: 'Just now',
        },
        ...activeTunnels,
      ];
    }

    showToast(`Bound target "${created.name}"`);
  }

  function handleTunnelCreated(tun: ActiveTunnelInfo) {
    activeTunnels = [tun, ...activeTunnels.filter((t) => t.id !== tun.id)];
  }

  function handleTunnelStopped(id: string) {
    activeTunnels = activeTunnels.filter((t) => t.id !== id);
  }

  function handleCopyLogs() {
    const text = logsList.map((l) => `[${l.timestamp}] [${l.provider}:${l.serviceName}] ${l.message}`).join('\n');
    navigator.clipboard?.writeText(text);
    showToast('Copied log buffer to clipboard.');
  }

  function handleClearLogs() {
    logsList = [];
    showToast('Log buffer cleared.');
  }

  const viewTitles: Record<NavItem, string> = {
    overview: 'Overview',
    tunnels: 'Tunnels',
    deployments: 'Deployments',
    watchdog: 'Watchdog',
    logs: 'Logs',
    profile: 'Profile',
  };
</script>

<!-- Frameless Top Window Header with Window-Drag and Controls -->
<TitleBar
  workspaceName={activeWorkspace?.name}
  currentView={viewTitles[activeNav]}
  onSync={handleSyncAll}
  {isSyncing}
  isSidebarCollapsed={isSidebarCollapsed}
  profile={userProfile}
  onToggleSidebar={() => (isSidebarCollapsed = !isSidebarCollapsed)}
  onNavigateHome={() => (activeNav = 'overview')}
  onOpenProfile={() => (activeNav = 'profile')}
  onSignOut={handleSignOut}
/>

<!-- Main Shell: Left Sidebar + Main Deck -->
<div class="flex-1 flex overflow-hidden relative">
  <!-- Left Persistent / Collapsible Sidebar -->
  <Sidebar
    {activeNav}
    {activeWorkspaceId}
    {workspaces}
    profile={userProfile}
    counts={navCounts}
    isCollapsed={isSidebarCollapsed}
    onSelectNav={(nav) => (activeNav = nav)}
    onSelectWorkspace={(id) => (activeWorkspaceId = id)}
    onAddService={() => (isAddModalOpen = true)}
    onOpenProfile={() => (activeNav = 'profile')}
    onSignOut={handleSignOut}
  />

  <!-- Main Deck Dynamic Content Area -->
  <main class="flex-1 flex flex-col min-w-0 bg-canvas overflow-hidden relative">
    <!-- Top Filter Bar with interactive dropdown chips -->
    {#if activeNav === 'overview'}
      <FilterBar
        {workspaces}
        selectedWorkspace={filterWorkspace}
        selectedProvider={filterProvider}
        selectedEnvironment={filterEnvironment}
        activeOnly={filterActiveOnly}
        searchQuery={filterSearch}
        onFilterChange={handleFilterChange}
      />
    {/if}

    <!-- Dynamic View Deck -->
    <div class="flex-1 overflow-y-auto flex flex-col">
      {#if activeNav === 'overview'}
        <div class="p-4 sm:p-6 border-b border-border-dim bg-surface/30">
          <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 mb-4">
            <div>
              <h1 class="text-base font-semibold text-tx-header tracking-tight">
                {activeWorkspace?.name ?? 'Main Workspace'}
              </h1>
              <p class="text-xs text-tx-meta mt-0.5">
                Cloud infrastructure and preview tunnels
              </p>
            </div>
            <div class="flex items-center gap-2">
              <button
                onclick={() => (isLogDrawerOpen = !isLogDrawerOpen)}
                class="px-3 py-1.5 rounded-xl bg-surface-elevated hover:bg-surface-hover text-tx-header border border-border-dim text-xs font-medium transition-all flex items-center gap-1.5 shadow-subtle active:scale-[0.98]"
              >
                <Icons name="terminal" size={12} class="text-coral" />
                <span>Logs</span>
              </button>

              {#if isAdmin}
                <button
                  onclick={() => (isAddModalOpen = true)}
                  class="px-3.5 py-1.5 rounded-xl bg-coral hover:bg-coral-hover text-white text-xs font-medium transition-all shadow-sm flex items-center gap-1.5 border border-coral/50 active:scale-[0.98]"
                >
                  <Icons name="plus" size={13} strokeWidth={2.5} />
                  <span>Add Target</span>
                </button>
              {/if}
            </div>
          </div>

          <!-- Quick Metric Highlights Cards -->
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3.5">
            <div class="p-3.5 rounded-xl bg-surface/80 hover:bg-surface-elevated/80 border border-border-dim/80 hover:border-coral/40 shadow-subtle flex flex-col justify-between transition-all duration-150">
              <div class="flex items-center justify-between text-tx-meta text-xs">
                <span class="font-medium text-tx-body">Active Tunnels</span>
                <div class="w-6 h-6 rounded-lg bg-surface-elevated border border-border-dim flex items-center justify-center text-coral">
                  <Icons name="radio" size={13} />
                </div>
              </div>
              <div class="mt-3 flex items-baseline justify-between">
                <span class="text-2xl font-bold text-tx-header font-mono">{activeTunnels.length}</span>
                <span class="text-[11px] text-tx-meta font-mono">Quick preview</span>
              </div>
            </div>

            <div class="p-3.5 rounded-xl bg-surface/80 hover:bg-surface-elevated/80 border border-border-dim/80 hover:border-coral/40 shadow-subtle flex flex-col justify-between transition-all duration-150">
              <div class="flex items-center justify-between text-tx-meta text-xs">
                <span class="font-medium text-tx-body">Bound Services</span>
                <div class="w-6 h-6 rounded-lg bg-surface-elevated border border-border-dim flex items-center justify-center text-coral">
                  <Icons name="server" size={13} />
                </div>
              </div>
              <div class="mt-3 flex items-baseline justify-between">
                <span class="text-2xl font-bold text-tx-header font-mono">{services.length}</span>
                <span class="text-[11px] text-tx-meta font-mono">Active targets</span>
              </div>
            </div>

            <div class="p-3.5 rounded-xl bg-surface/80 hover:bg-surface-elevated/80 border border-border-dim/80 hover:border-coral/40 shadow-subtle flex flex-col justify-between transition-all duration-150">
              <div class="flex items-center justify-between text-tx-meta text-xs">
                <span class="font-medium text-tx-body">Watchdog Quota</span>
                <div class="w-6 h-6 rounded-lg bg-surface-elevated border border-border-dim flex items-center justify-center text-coral">
                  <Icons name="shield" size={13} />
                </div>
              </div>
              <div class="mt-3 flex items-baseline justify-between">
                {#if watchdogRemainingHours !== null}
                  <span class="text-2xl font-bold text-tx-header font-mono">{watchdogRemainingHours}h</span>
                  <span class="text-[11px] text-tx-meta font-mono">Lowest left</span>
                {:else}
                  <span class="text-2xl font-bold text-tx-header font-mono">0h</span>
                  <span class="text-[11px] text-tx-meta font-mono">No targets</span>
                {/if}
              </div>
            </div>

            <div class="p-3.5 rounded-xl bg-surface/80 hover:bg-surface-elevated/80 border border-border-dim/80 hover:border-coral/40 shadow-subtle flex flex-col justify-between transition-all duration-150">
              <div class="flex items-center justify-between text-tx-meta text-xs">
                <span class="font-medium text-tx-body">Keep-Alive Guard</span>
                <div class="w-6 h-6 rounded-lg bg-surface-elevated border border-border-dim flex items-center justify-center {watchdogCount > 0 ? 'text-coral' : 'text-tx-meta'}">
                  <Icons name="check" size={13} />
                </div>
              </div>
              <div class="mt-3 flex items-baseline justify-between">
                {#if watchdogCount > 0}
                  <span class="text-base font-bold text-tx-header font-mono flex items-center gap-1.5">
                    <span class="w-2 h-2 rounded-full bg-coral"></span>
                    Active
                  </span>
                  <span class="text-[11px] text-tx-meta font-mono">{watchdogCount} guarded</span>
                {:else}
                  <span class="text-base font-bold text-tx-meta font-mono flex items-center gap-1.5">
                    <span class="w-2 h-2 rounded-full bg-border-subtle"></span>
                    Idle
                  </span>
                  <span class="text-[11px] text-tx-meta font-mono">0 targets</span>
                {/if}
              </div>
            </div>
          </div>
        </div>

        <!-- Service Rows List Section -->
        <div class="py-2">
          {#if filteredServices.length > 0}
            <div class="px-5 py-2 text-[10px] uppercase font-mono tracking-wider text-tx-meta flex items-center justify-between">
              <span>Bound Infrastructure Targets ({filteredServices.length})</span>
              <span>Status • Metrics • Actions</span>
            </div>
            {#each filteredServices as service (service.id)}
              <ServiceRow
                {service}
                {isAdmin}
                onDeploy={handleDeploySingle}
                onSyncEnv={() => (isSecretSyncOpen = true)}
                onViewLogs={handleViewLogs}
                onCopyUrl={handleCopyUrl}
                onDelete={handleDeleteService}
              />
            {/each}
          {:else}
            <EmptyState
              title={filterSearch ? 'No services match search' : 'No active targets'}
              description={filterSearch
                ? `No services or tunnels match "${filterSearch}". Try resetting your filter criteria.`
                : 'Configure a new cloud service binding or launch an instant preview tunnel to get started.'}
              buttonText="Reset Filters"
              onAction={resetFilters}
            />
          {/if}
        </div>
      {:else if activeNav === 'tunnels'}
        <!-- Dedicated Cloudflare Quick Tunnel Management View -->
        <TunnelView
          tunnels={activeTunnels}
          {isAdmin}
          onTunnelCreated={handleTunnelCreated}
          onTunnelStopped={handleTunnelStopped}
          onToast={showToast}
        />
      {:else if activeNav === 'watchdog'}
        <!-- Dedicated Watchdog and Database Dormancy Keep-Alive View -->
        <WatchdogView
          {services}
          workspaceId={activeWorkspaceId}
          {isAdmin}
          onKeepAlive={handleSingleKeepAlive}
          onKeepAliveAll={handleAllKeepAlive}
          onToast={showToast}
        />
      {:else if activeNav === 'deployments'}
        <!-- Dedicated Parallel Deployments View -->
        <DeploymentsView
          {services}
          {isDeploying}
          {deployJobs}
          {isAdmin}
          onDeployAll={handleDeployAll}
          onOpenSecretSync={() => (isSecretSyncOpen = true)}
          onDeploySingle={handleDeploySingle}
        />
      {:else if activeNav === 'logs'}
        <!-- Dedicated Full Live Logs Multiplexer View -->
        <LiveLogsView
          logs={logsList}
          onClear={handleClearLogs}
          onCopy={handleCopyLogs}
        />
      {:else if activeNav === 'profile'}
        <!-- Dedicated Developer Profile & Local-First Security View -->
        <ProfileView
          profile={userProfile}
          {workspaces}
          {services}
          onProfileUpdated={(p) => (userProfile = p)}
          onNavigate={(view) => (activeNav = view)}
          onToast={showToast}
        />
      {/if}
    </div>

    <!-- Slide-up Log Drawer Component -->
    <LogDrawer
      isOpen={isLogDrawerOpen}
      logs={logsList}
      onClose={() => (isLogDrawerOpen = false)}
      onNavigateLogs={() => {
        isLogDrawerOpen = false;
        activeNav = 'logs';
      }}
    />
  </main>
</div>

<!-- Modal: Add Service / New Tunnel with Inline Token Verification -->
<AddServiceModal
  isOpen={isAddModalOpen}
  workspaceId={activeWorkspaceId}
  onClose={() => (isAddModalOpen = false)}
  onSave={handleAddServiceSave}
/>

<!-- Modal: Secret Synchronizer -->
<SecretSyncModal
  isOpen={isSecretSyncOpen}
  workspaceId={activeWorkspaceId}
  onClose={() => (isSecretSyncOpen = false)}
  onSync={handleSecretSync}
/>

<!-- Modal: Developer Google OAuth Profile -->
<ProfileModal
  isOpen={isProfileModalOpen}
  profile={userProfile}
  onClose={() => (isProfileModalOpen = false)}
  onProfileUpdated={(p) => (userProfile = p)}
  onToast={showToast}
/>

<!-- Toast Notification Popup -->
{#if toastMessage}
  <div
    class="fixed bottom-4 right-4 bg-surface-elevated text-tx-header px-4 py-2.5 rounded-lg border border-border-subtle shadow-modal text-xs flex items-center gap-2.5 z-50 animate-in fade-in slide-in-from-bottom-2 duration-150"
  >
    <div class="w-2 h-2 rounded-full bg-coral animate-pulse"></div>
    <span>{toastMessage}</span>
  </div>
{/if}
