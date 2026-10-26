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
  <div class="w-16 h-16 rounded-full bg-white/[0.04] border border-white/[0.08] flex items-center justify-center mb-4 text-[#8b5cf6] shadow-[0_0_20px_rgba(139,92,246,0.25)]">
    <Icons name="layers" size={26} class="text-[#8b5cf6]" />
  </div>

  <h3 class="text-sm font-semibold text-white mb-1 tracking-tight">{title}</h3>
  <p class="text-xs text-[#94a3b8] max-w-sm mb-5 leading-relaxed">{description}</p>

  <button
    onclick={onAction}
    class="px-5 py-2.5 rounded-full bg-gradient-to-r from-[#8b5cf6] via-[#a855f7] to-[#ec4899] hover:opacity-95 text-white text-xs font-semibold shadow-[0_0_20px_rgba(139,92,246,0.35)] transition-all active:scale-[0.98]"
  >
    {buttonText}
  </button>
</div>
