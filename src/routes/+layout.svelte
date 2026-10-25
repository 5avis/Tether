<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<div class="w-screen h-screen flex flex-col bg-canvas text-tx-body overflow-hidden font-sans">
  {@render children()}
</div>
