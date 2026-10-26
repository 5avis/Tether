<script lang="ts">
  import Icons from './Icons.svelte';
  import type { Workspace } from '$lib/types';

  let {
    isOpen = false,
    project = null,
    totalProjects = 1,
    onClose = () => {},
    onSave = (data: { id: string; name: string; color: string }) => {},
    onDelete = (id: string) => {},
  }: {
    isOpen: boolean;
    project: Workspace | null;
    totalProjects?: number;
    onClose: () => void;
    onSave: (data: { id: string; name: string; color: string }) => void;
    onDelete?: (id: string) => void;
  } = $props();

  let name = $state('');
  let color = $state('#8b5cf6');
  let confirmDelete = $state(false);

  const colorPresets = [
    { label: 'Violet', value: '#8b5cf6' },
    { label: 'Emerald', value: '#10b981' },
    { label: 'Ocean', value: '#3b82f6' },
    { label: 'Rose', value: '#f43f5e' },
    { label: 'Amber', value: '#f59e0b' },
    { label: 'Cyan', value: '#06b6d4' },
    { label: 'Indigo', value: '#6366f1' },
    { label: 'Slate', value: '#71717a' },
  ];

  $effect(() => {
    if (isOpen) {
      if (project) {
        name = project.name;
        color = project.color || '#8b5cf6';
      } else {
        name = '';
        color = '#8b5cf6';
      }
      confirmDelete = false;
    }
  });

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name.trim()) return;
    const id = project ? project.id : `ws-${Date.now()}`;
    onSave({
      id,
      name: name.trim(),
      color,
    });
    onClose();
  }

  function handleDelete() {
    if (!project) return;
    if (!confirmDelete) {
      confirmDelete = true;
      return;
    }
    onDelete(project.id);
    onClose();
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/75 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#18181f] border border-border-dim rounded-2xl shadow-modal text-xs select-none relative overflow-hidden"
    >
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-border-dim flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div
            class="w-7 h-7 rounded-lg flex items-center justify-center transition-colors"
            style="background-color: {color}25; border: 1px solid {color}50; color: {color};"
          >
            <Icons name="folder" size={14} />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-tx-header">
              {project ? 'Edit Project' : 'Create New Project'}
            </h3>
            <p class="text-[10px] text-tx-meta">
              {project ? 'Update workspace settings' : 'Each workspace isolates its own cloud services'}
            </p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded-lg text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-colors"
          aria-label="Close dialog"
        >
          <Icons name="x" size={14} />
        </button>
      </div>

      <!-- Modal Body -->
      <form onsubmit={handleSubmit} class="p-5 space-y-4">
        <!-- Project Name Input -->
        <div>
          <label
            class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-1.5"
            for="project-name"
          >
            Project Name
          </label>
          <input
            id="project-name"
            type="text"
            bind:value={name}
            placeholder="e.g. Green-log ERP, Mobile Backend, Web Portal"
            required
            class="w-full bg-surface-elevated text-tx-header px-3.5 py-2 rounded-xl border border-border-dim focus:border-coral focus:outline-none placeholder:text-tx-meta text-xs transition-colors"
          />
        </div>

        <!-- Project Color Picker -->
        <div>
          <span class="block text-tx-meta text-[11px] uppercase font-mono tracking-wider mb-2">
            Project Theme Color
          </span>
          <div class="grid grid-cols-4 gap-2">
            {#each colorPresets as preset}
              <button
                type="button"
                onclick={() => (color = preset.value)}
                class="flex items-center gap-2 p-2 rounded-xl border transition-all text-left text-xs {color === preset.value
                  ? 'bg-surface-elevated border-tx-header text-tx-header font-medium'
                  : 'bg-surface/50 border-border-dim text-tx-meta hover:text-tx-body hover:bg-surface-elevated'}"
              >
                <span
                  class="w-3.5 h-3.5 rounded-full shrink-0 ring-1 ring-white/10"
                  style="background-color: {preset.value};"
                ></span>
                <span class="text-[11px] truncate">{preset.label}</span>
              </button>
            {/each}
          </div>
        </div>

        <!-- Feature Note -->
        <div class="p-3 rounded-xl bg-surface-elevated/40 border border-border-dim text-[11px] space-y-1">
          <div class="flex items-center gap-1.5 text-tx-header font-medium">
            <Icons name="shield" size={12} class="text-coral" />
            <span>Multi-Project Isolation</span>
          </div>
          <p class="text-tx-meta leading-relaxed text-[10px]">
            Cloud targets, deploy triggers, and logs belong to this project only. You can easily switch between projects in the sidebar at any time.
          </p>
        </div>

        <!-- Actions Footer -->
        <div class="pt-3 border-t border-border-dim flex items-center justify-between">
          <div>
            {#if project && totalProjects > 1}
              <button
                type="button"
                onclick={handleDelete}
                class="px-3 py-1.5 rounded-xl text-xs transition-colors {confirmDelete
                  ? 'bg-red-500/20 text-red-400 border border-red-500/40 hover:bg-red-500/30'
                  : 'text-red-400/80 hover:text-red-400 hover:bg-surface-elevated'}"
              >
                {confirmDelete ? 'Confirm Delete Project?' : 'Delete'}
              </button>
            {/if}
          </div>

          <div class="flex items-center gap-2">
            <button
              type="button"
              onclick={onClose}
              class="px-4 py-2 rounded-xl text-tx-body hover:text-tx-header hover:bg-surface-elevated transition-colors text-xs"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={!name.trim()}
              class="px-4 py-2 rounded-xl bg-coral hover:bg-coral-hover text-white font-medium shadow-sm transition-all border border-coral/60 active:scale-[0.98] disabled:opacity-40 flex items-center gap-1.5 text-xs"
            >
              <Icons name="check" size={13} strokeWidth={2.5} />
              <span>{project ? 'Save Changes' : 'Create Project'}</span>
            </button>
          </div>
        </div>
      </form>
    </div>
  </div>
{/if}
