<script lang="ts">
  import { backOut } from "svelte/easing";
  import { api } from "./lib/api";
  import { dayLabel, isoDate, lastDay } from "./lib/period";
  import Buddy from "./Buddy.svelte";
  import Icon from "./Icon.svelte";

  let { ondone }: { ondone: () => void } = $props();

  const STEPS = 4;
  let step = $state(0);
  let reach = $state<"checking" | "ok" | "no">("checking");

  async function check() {
    reach = "checking";
    reach = (await api.serverReachable().catch(() => false)) ? "ok" : "no";
  }

  $effect(() => {
    if (step === 1) check();
  });

  const go = (to: number) => {
    if (to >= 0 && to < STEPS) step = to;
  };
  const next = () => (step < STEPS - 1 ? go(step + 1) : ondone());

  function key(e: KeyboardEvent) {
    if (e.key === "ArrowRight") go(step + 1);
    else if (e.key === "ArrowLeft") go(step - 1);
    else return;
    e.preventDefault();
  }

  // Today's real payroll period (1-15 or 16-end), drawn with the board's own day tiles.
  const now = new Date();
  now.setHours(0, 0, 0, 0);
  const [from, to] = now.getDate() <= 15 ? [1, 15] : [16, lastDay(now.getFullYear(), now.getMonth() + 1)];
  const days = Array.from({ length: to - from + 1 }, (_, i) => new Date(now.getFullYear(), now.getMonth(), from + i)).filter(
    (d) => d.getDay() % 6,
  );
  const cutoff = days[days.length - 1];
  const left = days.filter((d) => d >= now).length;
  const label = (d: Date) => dayLabel(isoDate(d));
  const until = Math.round((+cutoff - +now) / 864e5); // calendar days to cutoff
  const todayLine = `Today is ${label(now)}. ${
    until < 0
      ? `Cutoff was ${label(cutoff)}.`
      : until === 0
        ? "Cutoff is today."
        : until === 1
          ? `Cutoff is tomorrow, ${label(cutoff)}.`
          : `Cutoff is ${label(cutoff)}, ${left} workdays left.`
  }`;

  const legend = [
    ["done", "Done", "OB and CS are both approved."],
    ["", "Waiting", "Filed, waiting for approval."],
    ["due", "Due today", "Today isn't fully filed yet."],
    ["act", "Needs action", "Missing, disapproved, or the times don't match the shift."],
  ];

  const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
  // Transitions move, never hide: if an animation stalls, the content is still there to read.
  /** A sticker slapped onto the board: lifted first, then pressed onto its shadow. */
  function slap(_: Element, { delay = 0 } = {}) {
    return {
      delay,
      duration: still ? 0 : 560,
      easing: backOut,
      css: (_t: number, u: number) =>
        `transform: translate(${-16 * u}px, ${-20 * u}px) scale(${1 + 0.06 * u})`,
    };
  }
  /** Every tile of the row lands on one downbeat, each with its own weight. */
  function beat(_: Element, { delay = 0 } = {}) {
    return {
      delay,
      duration: still ? 0 : 700,
      easing: backOut,
      css: (_t: number, u: number) => `transform: translateY(${-30 * u}px) rotate(${-8 * u}deg)`,
    };
  }
</script>

<svelte:window onkeydown={key} />

<main class="wrap">
  {#key step}
    <div class="buddy" in:slap><Buddy size={300} alive /></div>
  {/key}

  <section class="card bubble" aria-live="polite">
    {#key step}
      <div class="say" in:slap={{ delay: 70 }}>
        {#if step === 0}
          <h1 class="display">Hi, I'm NBSBuddy.</h1>
          <p>
            I file your work-from-home <b>Official Business (OB)</b> and <b>Change Shift (CS)</b> in NBS in a couple of
            clicks, and show which days still need action before payroll cutoff.
          </p>
          <p class="now">{todayLine}</p>
          <div class="rail" role="img" aria-label="This payroll period's workdays, with today and the cutoff day marked">
            {#each days as d, i (+d)}
              <span
                class="tile"
                class:due={+d === +now}
                class:today={+d === +now}
                class:up={d > now}
                in:beat={{ delay: 220 + Math.round(Math.sin(i * 1.7) * 40 + 40) }}
              >
                {d.getDate()}{#if +d === +cutoff}<i>CUTOFF</i>{/if}
              </span>
            {/each}
          </div>
        {:else if step === 1}
          <h1 class="display">Connect ZeroTier first.</h1>
          <p>NBS only answers on the company ZeroTier network. Keep ZeroTier connected whenever you use NBSBuddy.</p>
          <div class="row">
            {#if reach === "checking"}
              <span class="sticker">Looking for NBS…</span>
            {:else if reach === "ok"}
              <span class="sticker success" in:slap>NBS is reachable. You're set.</span>
            {:else}
              <span class="sticker danger" in:slap>Can't reach NBS yet. Connect ZeroTier.</span>
              <button class="btn" onclick={check}>Check again</button>
            {/if}
          </div>
        {:else if step === 2}
          <h1 class="display">Read the board.</h1>
          <p>Every workday of the payroll period gets one sticker:</p>
          <dl class="legend">
            {#each legend as [kind, name, meaning], i}
              <dt><span class="pill {kind}" in:slap={{ delay: 180 + i * 110 }}>{name}</span></dt>
              <dd>{meaning}</dd>
            {/each}
          </dl>
          <ul>
            <li>Double-click a day, or select it and press Enter, to file it.</li>
            <li>Filed by mistake? Open that day and delete it while it's still <i>For Approval</i>, then file again.</li>
          </ul>
        {:else}
          <h1 class="display">Meet the buttons.</h1>
          <p>They sit in the top corner of the board:</p>
          <ul class="keys">
            <li in:slap={{ delay: 180 }}>
              <span class="icon plus"><Icon name="plus" /></span>
              <span><b>File today.</b> <kbd>Ctrl</kbd> + <kbd>N</kbd>, or double-click any day.</span>
            </li>
            <li in:slap={{ delay: 290 }}>
              <span class="icon"><Icon name="refresh" /></span>
              <span><b>Reload the period</b> from NBS. <kbd>F5</kbd></span>
            </li>
            <li in:slap={{ delay: 400 }}>
              <span class="icon"><Icon name="gear" /></span>
              <span><b>Settings:</b> your usual reasons and NBS customer. <kbd>Ctrl</kbd> + <kbd>,</kbd></span>
            </li>
          </ul>
          <p class="muted">In any dialog, <kbd>Ctrl</kbd> + <kbd>Enter</kbd> files or saves.</p>
        {/if}
      </div>
    {/key}

    <footer>
      <div class="dots" role="img" aria-label="Step {step + 1} of {STEPS}">
        {#each { length: STEPS } as _, i}<span class:on={i === step}></span>{/each}
      </div>
      {#if step > 0}<button class="link" onclick={() => go(step - 1)}>Back</button>{/if}
      <!-- one button for Next and Sign in, so keyboard focus stays put between steps -->
      <!-- svelte-ignore a11y_autofocus -->
      <button class="btn primary" onclick={next} autofocus>{step < STEPS - 1 ? "Next" : "Sign in"}</button>
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
  h1 {
    font-size: 38px;
    letter-spacing: -0.02em;
  }
  p,
  li,
  dd {
    font-size: 15px;
  }
  .now {
    margin-top: 8px;
    font-weight: 800;
  }
  .rail {
    display: flex;
    gap: 6px;
    padding-bottom: 20px;
  }
  .tile {
    position: relative;
    display: grid;
    place-items: center;
    width: 40px;
    height: 50px;
    border: var(--line);
    border-radius: var(--r-tile);
    background: var(--surface);
    box-shadow: var(--shadow-1);
    font: 800 15px var(--font);
  }
  .tile.due {
    background: var(--lemon);
  }
  .tile.today {
    border-width: 4px;
  }
  .tile.up {
    border-style: dashed;
    background: transparent;
    box-shadow: none;
  }
  .tile i {
    position: absolute;
    right: -4px;
    top: calc(100% + 5px);
    padding: 1px 4px;
    border-radius: 3px;
    background: var(--ink);
    color: var(--lemon);
    font: 900 9px var(--font);
    font-style: normal;
    letter-spacing: 0.05em;
  }
  .row {
    display: flex;
    gap: 12px;
    align-items: center;
    flex-wrap: wrap;
  }
  .legend {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 10px 14px;
    align-items: center;
    margin: 0;
  }
  dd {
    margin: 0;
  }
  ul {
    margin: 0;
    padding-left: 20px;
    display: grid;
    gap: 6px;
  }
  .keys {
    list-style: none;
    padding: 0;
    gap: 14px;
  }
  .keys li {
    display: flex;
    align-items: center;
    gap: 14px;
  }
  .icon {
    flex: none;
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    border: var(--line);
    border-radius: var(--r-control);
    background: var(--surface);
    box-shadow: var(--shadow-2);
  }
  .icon.plus {
    background: var(--lemon);
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
