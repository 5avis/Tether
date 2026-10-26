<script lang="ts">
  import Icons from './Icons.svelte';

  let {
    title = 'No active targets found',
    description = 'No services match your active filter criteria.',
    buttonText = 'Reset Filters',
    onAction = () => {},
  }: {
    title?: string;
    description?: string;
    buttonText?: string;
    onAction?: () => void;
  } = $props();
</script>

<div class="flex-1 flex flex-col items-center justify-center p-12 text-center select-none animate-in fade-in duration-200">
  <!-- Minimalist vector graphic -->
  <div class="w-16 h-16 rounded-2xl bg-surface-elevated border border-border-dim flex items-center justify-center mb-4 text-tx-meta shadow-subtle">
    <Icons name="layers" size={28} class="text-coral/80" />
  </div>

  <h3 class="text-sm font-semibold text-tx-header mb-1 tracking-tight">{title}</h3>
  <p class="text-xs text-tx-meta max-w-sm mb-5 leading-relaxed">{description}</p>

  <button
    onclick={onAction}
    class="px-4 py-2 rounded-xl bg-gradient-to-r from-[#9333ea] to-[#7c3aed] hover:from-[#a855f7] hover:to-[#9333ea] text-white text-xs font-medium shadow-sm transition-all border border-[#c084fc]/30 active:scale-[0.98] shadow-[0_0_12px_rgba(168,85,247,0.25)]"
  >
    {buttonText}
  </button>
</div>
