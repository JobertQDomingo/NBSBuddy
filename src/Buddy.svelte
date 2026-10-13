<script lang="ts">
  // The NBSBuddy mascot: a lemon sticker with a face. Same drawing as the app icon.
  let { size = 64 }: { size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 1024 1024" aria-hidden="true">
  <rect x="120" y="120" width="840" height="840" rx="180" fill="#111" />
  <rect x="64" y="64" width="840" height="840" rx="180" fill="#ffe14d" stroke="#111" stroke-width="48" />
  <circle cx="350" cy="410" r="66" fill="#111" />
  <circle cx="618" cy="410" r="66" fill="#111" />
  <path d="M300 580 Q484 760 668 580" fill="none" stroke="#111" stroke-width="60" stroke-linecap="round" />
</svg>
