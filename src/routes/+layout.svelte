<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<div class="w-full h-full min-w-full min-h-full flex flex-col bg-canvas text-tx-body overflow-hidden font-sans">
  {@render children()}
</div>
