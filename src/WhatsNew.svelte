<script module lang="ts">
  import { local } from "./lib/api";

  /**
   * The version each step below ships in, in step order. A PC shows a step once, after the update that brings it.
   * Every release with a new feature adds its own steps here (RELEASING.md): one per feature of that version only,
   * each with a small demo drawn with the board's stickers, then its version appended below. Old steps stay.
   */
  const VERSIONS = ["0.3.0", "0.3.0", "0.3.0"];
  export const LATEST = VERSIONS[VERSIONS.length - 1];
  const after = (v: string, seen: string) => v.localeCompare(seen, undefined, { numeric: true }) > 0;
  /** Steps this PC hasn't seen. Nothing saved means an update from 0.2.0, which had no What's new: all of them. */
  export const unseen = () => VERSIONS.flatMap((v, i) => (after(v, local.get("newsSeen")) ? [i] : []));
  /** The newest release's steps, for Settings. */
  export const newest = () => VERSIONS.flatMap((v, i) => (v === LATEST ? [i] : []));
  /** Also called after the first-run tour, so a new install skips What's new. */
  export const markSeen = () => local.set("newsSeen", LATEST);
</script>

<script lang="ts">
  // What's new after an update: the first-run tour's look, one step per feature, over the whole window.
  import { dayLabel, isoDate } from "./lib/period";
  import Chevron from "./Chevron.svelte";
  import Icon from "./Icon.svelte";
  import Modal from "./Modal.svelte";
  import Tour, { slap } from "./Tour.svelte";

  let { steps, onclose }: { steps: number[]; onclose: () => void } = $props();

  let step = $state(0);

  // Last week's Monday to Wednesday for the sample board rows: always past days, like the verdicts they show.
  const monday = new Date();
  monday.setDate(monday.getDate() - ((monday.getDay() + 6) % 7) - 7);
  const day = (n: number) => dayLabel(isoDate(new Date(monday.getFullYear(), monday.getMonth(), monday.getDate() + n)));
</script>

<Modal label="What's new in NBSBuddy" full {onclose}>
  <Tour count={steps.length} bind:step last="Got it" ondone={onclose} onskip={onclose}>
    <span class="pill due">New in {VERSIONS[steps[step]]}</span>
    {#if steps[step] === 0}
      <h1 class="display">Find a shift by typing.</h1>
      <p>In <b>File CS/OB</b>, type in the Shift field instead of scrolling. I understand times however you write them.</p>
      <div class="demo" aria-hidden="true" in:slap={{ delay: 180 }}>
        <div class="field">730<i class="caret"></i><span class="chevron"><Chevron /></span></div>
        <ul class="list"><li>CW 9.5H 730AM-530PM SP 30MIN BRK</li></ul>
      </div>
      <p class="try">Try <kbd>730</kbd> <kbd>7:30 pm</kbd> <kbd>8-6</kbd> <kbd>17:30</kbd> or <kbd>night</kbd>.</p>
    {:else if steps[step] === 1}
      <h1 class="display">Leave on the board.</h1>
      <p>
        A leave you filed in NBS now appears on its day. A whole-day leave counts as filed; a half day still needs OB and CS.
        You still file leave in NBS.
      </p>
      <table class="mini" in:slap={{ delay: 180 }}>
        <thead><tr><th>Date</th><th>Status</th><th>Note</th></tr></thead>
        <tbody>
          <tr><td>{day(0)}</td><td><span class="pill done">Done</span></td><td>Vacation Leave</td></tr>
          <tr><td>{day(1)}</td><td><span class="pill">Waiting</span></td><td>Sick Leave · Leave awaiting approval</td></tr>
          <tr><td>{day(2)}</td><td><span class="pill act">Needs action</span></td><td>Vacation Leave (half day) · Not filed</td></tr>
        </tbody>
      </table>
    {:else}
      <h1 class="display">I update myself.</h1>
      <p>When a new version is out, you get it without downloading anything.</p>
      <ul class="ways">
        <li in:slap={{ delay: 180 }}>
          <span class="sticker">Updating NBSBuddy…</span>
          <span><b>Automatic</b> (the default): I update while I start, then open as usual.</span>
        </li>
        <li in:slap={{ delay: 290 }}>
          <span class="btn primary fake" aria-hidden="true"><Icon name="download" />Update</span>
          <span><b>Your call:</b> turn automatic updates off in Settings, and this button shows up on the board instead.</span>
        </li>
      </ul>
    {/if}
  </Tour>
</Modal>

<style>
  .pill {
    justify-self: start;
  }
  .demo {
    display: grid;
    gap: 6px;
    max-width: 360px;
  }
  /* the Shift field mid-search, drawn like ShiftPicker */
  .field {
    display: flex;
    align-items: center;
    min-height: 34px;
    padding: 4px 10px 4px 8px;
    border: 2px solid var(--accent);
    border-radius: var(--r-control);
    box-shadow: 0 0 0 1px var(--accent);
    background: var(--surface);
  }
  .caret {
    width: 2px;
    height: 18px;
    margin-left: 1px;
    background: var(--ink);
    animation: caret 1.1s steps(1) infinite;
  }
  @keyframes caret {
    50% {
      opacity: 0;
    }
  }
  .chevron {
    display: flex;
    margin-left: auto;
  }
  .list {
    margin: 0;
    padding: 4px;
    list-style: none;
    border: var(--line);
    background: var(--surface);
    box-shadow: var(--shadow-3);
  }
  .list li {
    padding: 6px 8px;
    border-radius: var(--r-pill);
    outline: 3px solid var(--accent);
    outline-offset: -3px;
    font-size: 14px;
  }
  .try {
    line-height: 2;
  }
  .try kbd {
    margin: 0 3px;
  }

  .mini {
    border-collapse: separate;
    border-spacing: 0;
    border: var(--line);
    border-radius: var(--r-control);
    overflow: hidden;
    box-shadow: var(--shadow-3);
  }
  .mini th {
    padding: 6px 10px;
    background: var(--lemon);
    border-bottom: var(--line);
    text-align: left;
    font-size: 12px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
  .mini td {
    height: 34px;
    padding: 0 10px;
    white-space: nowrap;
  }
  .mini tr + tr td {
    border-top: var(--line);
  }

  .ways {
    margin: 0;
    padding: 0;
    list-style: none;
    display: grid;
    gap: 16px;
  }
  .ways li {
    display: grid;
    grid-template-columns: 190px 1fr;
    gap: 14px;
    align-items: center;
  }
  .ways .sticker,
  .fake {
    justify-self: start;
    white-space: nowrap;
  }
  .fake {
    pointer-events: none; /* a picture of the board's button: no hover or press */
  }
  @media (prefers-reduced-motion: reduce) {
    .caret {
      animation: none;
    }
  }
</style>
