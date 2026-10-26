<script lang="ts">
  import Icons from './Icons.svelte';
  import type { Workspace } from '$lib/types';

  let {
    selectedWorkspace = $bindable('all'),
    selectedProvider = $bindable('all'),
    selectedEnvironment = $bindable('all'),
    activeOnly = $bindable(false),
    searchQuery = $bindable(''),
    workspaces = [],
    onFilterChange = () => {},
  }: {
    selectedWorkspace?: string;
    selectedProvider?: string;
    selectedEnvironment?: string;
    activeOnly?: boolean;
    searchQuery?: string;
    workspaces?: Workspace[];
    onFilterChange?: (filters: {
      workspace: string;
      provider: string;
      environment: string;
      activeOnly: boolean;
      search: string;
    }) => void;
  } = $props();

  let openDropdown = $state<'workspace' | 'provider' | 'environment' | null>(null);

  function triggerUpdate() {
    onFilterChange({
      workspace: selectedWorkspace,
      provider: selectedProvider,
      environment: selectedEnvironment,
      activeOnly,
      search: searchQuery,
    });
  }

  function handleSearchInput(e: Event) {
    searchQuery = (e.target as HTMLInputElement).value;
    triggerUpdate();
  }

  function toggleActiveOnly() {
    activeOnly = !activeOnly;
    triggerUpdate();
  }

  const isFiltered = $derived(
    selectedWorkspace !== 'all' ||
    selectedProvider !== 'all' ||
    selectedEnvironment !== 'all' ||
    activeOnly ||
    searchQuery.trim().length > 0
  );

  function resetAll() {
    selectedWorkspace = 'all';
    selectedProvider = 'all';
    selectedEnvironment = 'all';
    activeOnly = false;
    searchQuery = '';
    openDropdown = null;
    triggerUpdate();
  }

  const providerOptions = [
    { value: 'all', label: 'All Providers' },
    { value: 'cloudflare', label: 'Cloudflare Quick Tunnels' },
    { value: 'flyio', label: 'Fly.io Containers' },
    { value: 'supabase', label: 'Supabase DB' },
    { value: 'neon', label: 'Neon Postgres' },
    { value: 'render', label: 'Render Web' },
    { value: 'vercel', label: 'Vercel App' },
    { value: 'railway', label: 'Railway Service' },
  ];

  const environmentOptions = [
    { value: 'all', label: 'All Environments' },
    { value: 'production', label: 'Production' },
    { value: 'staging', label: 'Staging' },
    { value: 'preview', label: 'Preview' },
    { value: 'development', label: 'Development' },
  ];

  const currentWorkspaceLabel = $derived(
    selectedWorkspace === 'all'
      ? 'All Workspaces'
      : workspaces.find((w) => w.id === selectedWorkspace)?.name ?? selectedWorkspace
  );

  const currentProviderLabel = $derived(
    providerOptions.find((p) => p.value === selectedProvider)?.label ?? 'All Providers'
  );

  const currentEnvironmentLabel = $derived(
    selectedEnvironment === 'all'
      ? 'Environment'
      : environmentOptions.find((e) => e.value === selectedEnvironment)?.label ?? 'Environment'
  );
</script>

<div class="min-h-[48px] py-2 px-4 border-b border-border-dim flex flex-wrap items-center justify-between gap-2.5 bg-surface/50 text-xs select-none relative z-30">
  <!-- Left: Filter custom chips (No native OS select boxes) -->
  <div class="flex items-center gap-2 flex-wrap">
    <!-- Custom Workspace Dropdown Chip -->
    <div class="relative">
      <button
        type="button"
        onclick={() => (openDropdown = openDropdown === 'workspace' ? null : 'workspace')}
        class="flex items-center gap-2 px-3 py-1.5 rounded-xl border text-xs transition-colors shadow-subtle {selectedWorkspace !== 'all'
          ? 'bg-surface-elevated border-coral/50 text-coral font-medium'
          : 'bg-surface-elevated hover:bg-surface-hover text-tx-header border-border-dim/80 hover:border-border-subtle'}"
      >
        <span>{currentWorkspaceLabel}</span>
        <Icons name="chevron-down" size={11} class="text-tx-meta transition-transform {openDropdown === 'workspace' ? 'rotate-180' : ''}" />
      </button>

      {#if openDropdown === 'workspace'}
        <!-- Backdrop to close on click outside -->
        <button
          type="button"
          class="fixed inset-0 z-40 cursor-default bg-transparent"
          onclick={() => (openDropdown = null)}
          aria-label="Close menu"
        ></button>

        <!-- Custom Dropdown Menu Box -->
        <div class="absolute top-10 left-0 min-w-[200px] bg-[#18181f] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
          <div class="px-2.5 py-1 text-[10px] uppercase font-mono text-tx-meta tracking-wider">Workspaces</div>
          <button
            onclick={() => {
              selectedWorkspace = 'all';
              openDropdown = null;
              triggerUpdate();
            }}
            class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {selectedWorkspace === 'all' ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
          >
            <span>All Workspaces</span>
            {#if selectedWorkspace === 'all'}
              <Icons name="check" size={12} class="text-coral" />
            {/if}
          </button>
          {#each workspaces as ws}
            <button
              onclick={() => {
                selectedWorkspace = ws.id;
                openDropdown = null;
                triggerUpdate();
              }}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {selectedWorkspace === ws.id ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
            >
              <span>{ws.name}</span>
              {#if selectedWorkspace === ws.id}
                <Icons name="check" size={12} class="text-coral" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Custom Provider Dropdown Chip -->
    <div class="relative">
      <button
        type="button"
        onclick={() => (openDropdown = openDropdown === 'provider' ? null : 'provider')}
        class="flex items-center gap-2 px-3 py-1.5 rounded-xl border text-xs transition-colors shadow-subtle {selectedProvider !== 'all'
          ? 'bg-surface-elevated border-coral/50 text-coral font-medium'
          : 'bg-surface-elevated hover:bg-surface-hover text-tx-header border-border-dim/80 hover:border-border-subtle'}"
      >
        <span>{currentProviderLabel}</span>
        <Icons name="chevron-down" size={11} class="text-tx-meta transition-transform {openDropdown === 'provider' ? 'rotate-180' : ''}" />
      </button>

      {#if openDropdown === 'provider'}
        <button
          type="button"
          class="fixed inset-0 z-40 cursor-default bg-transparent"
          onclick={() => (openDropdown = null)}
          aria-label="Close menu"
        ></button>

        <div class="absolute top-10 left-0 min-w-[210px] bg-[#18181f] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
          <div class="px-2.5 py-1 text-[10px] uppercase font-mono text-tx-meta tracking-wider">Cloud Providers</div>
          {#each providerOptions as opt}
            <button
              onclick={() => {
                selectedProvider = opt.value;
                openDropdown = null;
                triggerUpdate();
              }}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {selectedProvider === opt.value ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
            >
              <span>{opt.label}</span>
              {#if selectedProvider === opt.value}
                <Icons name="check" size={12} class="text-coral" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Custom Environment Dropdown Chip -->
    <div class="relative">
      <button
        type="button"
        onclick={() => (openDropdown = openDropdown === 'environment' ? null : 'environment')}
        class="flex items-center gap-2 px-3 py-1.5 rounded-xl border text-xs transition-colors shadow-subtle {selectedEnvironment !== 'all'
          ? 'bg-surface-elevated border-coral/50 text-coral font-medium'
          : 'bg-surface-elevated hover:bg-surface-hover text-tx-header border-border-dim/80 hover:border-border-subtle'}"
      >
        <span>{currentEnvironmentLabel}</span>
        <Icons name="chevron-down" size={11} class="text-tx-meta transition-transform {openDropdown === 'environment' ? 'rotate-180' : ''}" />
      </button>

      {#if openDropdown === 'environment'}
        <button
          type="button"
          class="fixed inset-0 z-40 cursor-default bg-transparent"
          onclick={() => (openDropdown = null)}
          aria-label="Close menu"
        ></button>

        <div class="absolute top-10 left-0 min-w-[180px] bg-[#18181f] border border-border-dim rounded-xl shadow-modal p-1.5 z-50 animate-in fade-in zoom-in-95 duration-100 text-xs">
          <div class="px-2.5 py-1 text-[10px] uppercase font-mono text-tx-meta tracking-wider">Environments</div>
          {#each environmentOptions as opt}
            <button
              onclick={() => {
                selectedEnvironment = opt.value;
                openDropdown = null;
                triggerUpdate();
              }}
              class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg text-left transition-colors {selectedEnvironment === opt.value ? 'bg-surface-elevated text-tx-header font-medium' : 'text-tx-body hover:text-tx-header hover:bg-surface-hover'}"
            >
              <span>{opt.label}</span>
              {#if selectedEnvironment === opt.value}
                <Icons name="check" size={12} class="text-coral" />
              {/if}
            </button>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Active Only Toggle Chip -->
    <button
      onclick={toggleActiveOnly}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl border text-xs transition-all {activeOnly
        ? 'bg-coral-light text-coral border-coral/50 font-medium'
        : 'bg-surface-elevated hover:bg-surface-hover text-tx-body border-border-dim/80 hover:border-border-subtle'}"
    >
      <span>Active Only</span>
      {#if activeOnly}
        <Icons name="x" size={12} class="text-coral" />
      {:else}
        <span class="w-1.5 h-1.5 rounded-full bg-border-subtle"></span>
      {/if}
    </button>

    <!-- Reset Filters Button -->
    {#if isFiltered}
      <button
        onclick={resetAll}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-xl bg-surface-elevated hover:bg-surface-hover text-tx-meta hover:text-coral border border-border-dim/80 transition-colors text-[11px]"
        title="Reset all active filters"
      >
        <Icons name="refresh" size={11} />
        <span>Reset</span>
      </button>
    {/if}
  </div>

  <!-- Right: Search Input Filter -->
  <div class="relative w-full sm:w-64 max-w-xs">
    <div class="absolute left-2.5 top-1/2 -translate-y-1/2 text-tx-meta pointer-events-none">
      <Icons name="search" size={13} />
    </div>
    <input
      type="text"
      placeholder="Filter targets, ports, or URLs..."
      value={searchQuery}
      oninput={handleSearchInput}
      class="w-full bg-surface-elevated hover:bg-surface-hover focus:bg-surface text-tx-header text-xs rounded-xl pl-8 pr-7 py-1.5 border border-border-dim/80 focus:border-coral focus:outline-none transition-colors placeholder:text-tx-meta"
    />
    {#if searchQuery}
      <button
        onclick={() => {
          searchQuery = '';
          triggerUpdate();
        }}
        class="absolute right-2.5 top-1/2 -translate-y-1/2 text-tx-meta hover:text-tx-header"
      >
        <Icons name="x" size={12} />
      </button>
    {/if}
  </div>
</div>
