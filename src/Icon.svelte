<script lang="ts">
  // Drawn header icons, never font glyphs (DESIGN.md): one 24-unit grid, one round ink stroke.
  // The gear has six chunky teeth so its gaps survive the stroke at header size.
  let { name }: { name: "gear" | "refresh" | "plus" | "download" } = $props();
</script>

<svg width="18" height="18" viewBox="0 0 24 24" aria-hidden="true">
  <g fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round">
    {#if name === "gear"}
      <path
        d="M9.87 5.02L10.09 2.18A10 10 0 0 1 13.91 2.18L14.13 5.02A7.3 7.3 0 0 1 16.98 6.66L19.55 5.44A10 10 0 0 1 21.46 8.74L19.11 10.36A7.3 7.3 0 0 1 19.11 13.64L21.46 15.26A10 10 0 0 1 19.55 18.56L16.98 17.34A7.3 7.3 0 0 1 14.13 18.98L13.91 21.82A10 10 0 0 1 10.09 21.82L9.87 18.98A7.3 7.3 0 0 1 7.02 17.34L4.45 18.56A10 10 0 0 1 2.54 15.26L4.89 13.64A7.3 7.3 0 0 1 4.89 10.36L2.54 8.74A10 10 0 0 1 4.45 5.44L7.02 6.66A7.3 7.3 0 0 1 9.87 5.02Z"
      />
      <circle cx="12" cy="12" r="3" />
    {:else if name === "refresh"}
      <path d="M17.82 16.89A7.6 7.6 0 1 1 15.80 5.42" />
      <path d="M15.42 1.64L16.84 6.02L12.34 6.97" />
    {:else if name === "download"}
      <path d="M12 3.5v11M7.5 10l4.5 4.5 4.5-4.5" />
      <path d="M4.5 16v3.5h15V16" />
    {:else}
      <path d="M12 4.5v15M4.5 12h15" />
    {/if}
  </g>
</svg>
