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
  let confirmDelete = $state(false);

  $effect(() => {
    if (isOpen) {
      if (project) {
        name = project.name;
      } else {
        name = '';
      }
      confirmDelete = false;
    }
  });

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (!name.trim()) return;
    const id = project ? project.id : `ws-${Date.now()}`;
    const color = project?.color || '#a855f7';
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
    class="fixed inset-0 bg-black/80 backdrop-blur-md z-50 flex items-center justify-center p-4 animate-in fade-in duration-150"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md bg-[#161326] border border-border-dim rounded-2xl shadow-modal text-xs select-none relative overflow-hidden"
    >
      <!-- Modal Header -->
      <div class="px-5 py-4 border-b border-border-dim flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <div
            class="w-7 h-7 rounded-lg bg-coral/15 border border-coral/30 flex items-center justify-center text-coral shrink-0"
          >
            <Icons name="folder" size={14} />
          </div>
          <div class="min-w-0">
            <h3 class="text-sm font-semibold text-tx-header truncate">
              {project ? 'Edit Project' : 'Create New Project'}
            </h3>
            <p class="text-[10px] text-tx-meta truncate">
              {project ? 'Update project settings' : 'Configure a new isolated workspace'}
            </p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded-lg text-tx-meta hover:text-tx-header hover:bg-surface-elevated transition-colors shrink-0"
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
            class="w-full bg-surface-elevated text-tx-header px-3.5 py-2.5 rounded-xl border border-border-dim focus:border-coral focus:outline-none placeholder:text-tx-meta text-xs transition-colors"
          />
        </div>

        <!-- Simple Clean Note (No Shield Symbol, Simple Content) -->
        <div class="p-3 rounded-xl bg-surface-elevated/40 border border-border-dim/60">
          <p class="text-[11px] text-tx-meta leading-relaxed">
            Each project maintains its own isolated cloud targets, deployments, and logs.
          </p>
        </div>

        <!-- Actions Footer -->
        <div class="pt-3 border-t border-border-dim flex items-center justify-between gap-3">
          <div>
            {#if project && totalProjects > 1}
              <button
                type="button"
                onclick={handleDelete}
                class="px-3 py-1.5 rounded-xl text-xs transition-colors font-medium {confirmDelete
                  ? 'bg-rose-500/20 text-rose-400 border border-rose-500/40 hover:bg-rose-500/30'
                  : 'text-rose-400/80 hover:text-rose-400 hover:bg-surface-elevated'}"
              >
                {confirmDelete ? 'Confirm Delete Project?' : 'Delete'}
              </button>
            {/if}
          </div>

          <div class="flex items-center gap-2 shrink-0">
            <button
              type="button"
              onclick={onClose}
              class="px-4 py-2 rounded-xl text-tx-body hover:text-tx-header hover:bg-surface-elevated transition-colors text-xs font-medium"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={!name.trim()}
              class="px-4 py-2 rounded-xl bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white font-medium shadow-sm transition-all border border-[#c084fc]/30 active:scale-[0.98] disabled:opacity-40 flex items-center gap-1.5 text-xs shadow-[0_0_12px_rgba(168,85,247,0.25)]"
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
