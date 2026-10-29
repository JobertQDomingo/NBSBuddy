<script module lang="ts">
  import { backOut } from "svelte/easing";

  export const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
  // Transitions move, never hide: if an animation stalls, the content is still there to read.
  /** A sticker slapped onto the board: lifted first, then pressed onto its shadow. */
  export function slap(_: Element, { delay = 0 } = {}) {
    return {
      delay,
      duration: still ? 0 : 560,
      easing: backOut,
      css: (_t: number, u: number) => `transform: translate(${-16 * u}px, ${-20 * u}px) scale(${1 + 0.06 * u})`,
    };
  }
</script>

<script lang="ts">
  // Buddy beside a speech bubble, one step at a time (the first-run tour and What's new).
  // Next/Back or the arrow keys move between steps; each one is slapped on like a sticker.
  import type { Snippet } from "svelte";
  import Buddy from "./Buddy.svelte";

  let {
    count,
    step = $bindable(0),
    last,
    ondone,
    onskip,
    children,
  }: {
    count: number;
    step?: number;
    /** the last step's button */
    last: string;
    ondone: () => void;
    /** shows a Skip link before the last step */
    onskip?: () => void;
    children: Snippet;
  } = $props();

  const go = (to: number) => {
    if (to >= 0 && to < count) step = to;
  };
  const next = () => (step < count - 1 ? go(step + 1) : ondone());

  function key(e: KeyboardEvent) {
    if (e.key === "ArrowRight") go(step + 1);
    else if (e.key === "ArrowLeft") go(step - 1);
    else return;
    e.preventDefault();
  }
</script>

<svelte:window onkeydown={key} />

<main class="wrap">
  {#key step}
    <div class="buddy" in:slap><Buddy size={300} alive /></div>
  {/key}

  <section class="card bubble" aria-live="polite">
    {#key step}
      <div class="say" in:slap={{ delay: 70 }}>{@render children()}</div>
    {/key}

    <footer>
      <div class="dots" role="img" aria-label="Step {step + 1} of {count}">
        {#each { length: count } as _, i}<span class:on={i === step}></span>{/each}
      </div>
      {#if onskip && step < count - 1}<button class="link skip" onclick={onskip} title="Esc">Skip</button>{/if}
      {#if step > 0}<button class="link" onclick={() => go(step - 1)}>Back</button>{/if}
      <!-- one button for Next and the last step, so keyboard focus stays put between steps -->
      <!-- svelte-ignore a11y_autofocus -->
      <button class="btn primary" onclick={next} autofocus>{step < count - 1 ? "Next" : last}</button>
    </footer>
  </section>
</main>

<style>
  .wrap {
    height: 100%;
    display: grid;
    grid-template-columns: 320px minmax(0, 620px);
    justify-content: center;
    align-items: center;
    gap: 64px;
    padding: 32px;
  }
  .buddy {
    justify-self: end;
    rotate: -6deg;
  }
  .bubble {
    position: relative;
    min-height: 480px;
    padding: 36px 40px 26px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    box-shadow: var(--shadow-4);
  }
  /* the speech-bubble tail, pointing at Buddy */
  .bubble::before {
    content: "";
    position: absolute;
    left: -14px;
    top: 42%;
    width: 24px;
    height: 24px;
    background: var(--surface);
    border-left: var(--line);
    border-bottom: var(--line);
    transform: rotate(45deg);
  }
  .say {
    display: grid;
    gap: 16px;
    align-content: start;
  }
  .say :global(h1) {
    font-size: 38px;
    letter-spacing: -0.02em;
  }
  .say :global(:is(p, li, dd)) {
    font-size: 15px;
  }
  footer {
    margin-top: auto;
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .dots {
    display: flex;
    gap: 6px;
    margin-right: auto;
  }
  .dots:has(+ .skip) {
    margin-right: 8px;
  }
  .skip {
    margin-right: auto;
  }
  .dots span {
    width: 14px;
    height: 14px;
    border: var(--line);
    border-radius: 4px;
    background: var(--surface);
  }
  .dots span.on {
    background: var(--lemon);
  }
</style>
