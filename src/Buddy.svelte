<script lang="ts">
  // The NBSBuddy mascot: a lemon sticker with a face. Same drawing as the app icon.
  // `alive` adds a blink and eyes that follow the pointer (both off under reduced motion).
  let { size = 64, alive = false }: { size?: number; alive?: boolean } = $props();

  let svg: SVGSVGElement;
  let look = $state("");

  function follow(e: PointerEvent) {
    const r = svg.getBoundingClientRect();
    const dx = e.clientX - (r.left + r.width / 2);
    const dy = e.clientY - (r.top + r.height * 0.42);
    const k = Math.min(1, Math.hypot(dx, dy) / 600);
    const a = Math.atan2(dy, dx);
    look = `translate(${Math.cos(a) * 34 * k}px, ${Math.sin(a) * 26 * k}px)`;
  }
</script>

<svelte:window onpointermove={alive ? follow : undefined} />

<svg bind:this={svg} class:alive width={size} height={size} viewBox="0 0 1024 1024" aria-hidden="true">
  <rect x="120" y="120" width="840" height="840" rx="180" fill="#111" />
  <rect x="64" y="64" width="840" height="840" rx="180" fill="#ffe14d" stroke="#111" stroke-width="48" />
  <g class="look" style:transform={look}>
    <g class="eyes">
      <circle cx="350" cy="410" r="66" fill="#111" />
      <circle cx="618" cy="410" r="66" fill="#111" />
    </g>
  </g>
  <path d="M300 580 Q484 760 668 580" fill="none" stroke="#111" stroke-width="60" stroke-linecap="round" />
</svg>

<style>
  svg {
    display: block;
    overflow: visible;
  }
  .look {
    transition: transform 0.35s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .alive .eyes {
    transform-box: fill-box;
    transform-origin: center;
    animation: blink 5.2s infinite;
  }
  @keyframes blink {
    0%,
    93%,
    100% {
      transform: scaleY(1);
    }
    95% {
      transform: scaleY(0.12);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .alive .eyes {
      animation: none;
    }
    .look {
      transform: none !important;
    }
  }
</style>
