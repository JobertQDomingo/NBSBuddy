<script lang="ts">
  // Drawn chevrons, never font glyphs (DESIGN.md).
  let { dir = "down" }: { dir?: "left" | "right" | "down" } = $props();
  const d = $derived({ left: "M10 3 5 8l5 5", right: "M6 3l5 5-5 5", down: "M3 6l5 5 5-5" }[dir]);
</script>

<svg width="14" height="14" viewBox="0 0 16 16" aria-hidden="true">
  <path {d} fill="none" stroke="currentColor" stroke-width="2.25" stroke-linecap="round" stroke-linejoin="round" />
</svg>
