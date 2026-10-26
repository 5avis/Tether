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
      class="w-full max-w-md bg-[#121629]/95 backdrop-blur-2xl border border-white/[0.12] rounded-3xl shadow-[0_0_35px_rgba(139,92,246,0.25)] text-xs select-none relative overflow-hidden"
    >
      <!-- Modal Header -->
      <div class="px-6 py-5 border-b border-white/[0.08] flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div
            class="w-8 h-8 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center text-[#8b5cf6] shadow-[0_0_12px_rgba(139,92,246,0.3)] shrink-0"
          >
            <Icons name="folder" size={14} />
          </div>
          <div class="min-w-0">
            <h3 class="text-sm font-semibold text-white truncate">
              {project ? 'Edit Project' : 'Create New Project'}
            </h3>
            <p class="text-[10px] text-[#94a3b8] truncate">
              {project ? 'Update project settings' : 'Configure a new isolated workspace'}
            </p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="w-7 h-7 rounded-full flex items-center justify-center text-[#94a3b8] hover:text-white hover:bg-white/[0.08] transition-colors shrink-0"
          aria-label="Close dialog"
        >
          <Icons name="x" size={14} />
        </button>
      </div>

      <!-- Modal Body -->
      <form onsubmit={handleSubmit} class="p-6 space-y-4">
        <!-- Project Name Input -->
        <div>
          <label
            class="block text-[#94a3b8] text-[11px] uppercase font-mono tracking-wider mb-2"
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
            class="w-full bg-white/[0.04] text-white px-4 py-2.5 rounded-full border border-white/[0.08] focus:border-[#00f2fe]/60 focus:outline-none placeholder:text-[#94a3b8] text-xs transition-colors"
          />
        </div>

        <!-- Simple Clean Note (No Shield Symbol, Simple Content) -->
        <div class="p-3.5 rounded-2xl bg-white/[0.03] border border-white/[0.08]">
          <p class="text-[11px] text-[#94a3b8] leading-relaxed">
            Each project maintains its own isolated cloud targets, deployments, and logs.
          </p>
        </div>

        <!-- Actions Footer -->
        <div class="pt-3 border-t border-white/[0.08] flex items-center justify-between gap-3">
          <div>
            {#if project && totalProjects > 1}
              <button
                type="button"
                onclick={handleDelete}
                class="px-3.5 py-1.5 rounded-full text-xs transition-colors font-medium {confirmDelete
                  ? 'bg-rose-500/20 text-rose-400 border border-rose-500/40 hover:bg-rose-500/30'
                  : 'text-rose-400/80 hover:text-rose-400 hover:bg-white/[0.05]'}"
              >
                {confirmDelete ? 'Confirm Delete Project?' : 'Delete'}
              </button>
            {/if}
          </div>

          <div class="flex items-center gap-2 shrink-0">
            <button
              type="button"
              onclick={onClose}
              class="px-4 py-2 rounded-full text-[#94a3b8] hover:text-white hover:bg-white/[0.06] transition-colors text-xs font-medium"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={!name.trim()}
              class="px-5 py-2 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white font-semibold shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all border border-transparent active:scale-[0.98] disabled:opacity-40 flex items-center gap-1.5 text-xs"
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
