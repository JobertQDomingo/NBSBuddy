<script lang="ts">
  // [<][ Jul 1–15, 2026 v ][>] as one sticker; the label opens every half-month of
  // the year at once (a native popover, anchored under the label).
  import Chevron from "./Chevron.svelte";
  import { MONTHS, currentPeriod, halfRange, periodLabel, samePeriod, stepPeriod, type Period } from "./lib/period";

  let { value = $bindable() }: { value: Period } = $props();

  const now = currentPeriod();
  let year = $state(now.year);
  let grid: HTMLElement;

  function pick(p: Period) {
    value = p;
    grid.hidePopover();
  }
</script>

<div class="stepper">
  <button class="btn" aria-label="Previous period (Alt+Left)" title="Previous period (Alt+Left)" onclick={() => (value = stepPeriod(value, -1))}>
    <Chevron dir="left" />
  </button>
  <button class="btn label" popovertarget="period-grid" aria-haspopup="dialog" onclick={() => (year = value.year)}>
    {periodLabel(value)} <Chevron />
  </button>
  <button class="btn" aria-label="Next period (Alt+Right)" title="Next period (Alt+Right)" onclick={() => (value = stepPeriod(value, 1))}>
    <Chevron dir="right" />
  </button>
</div>

<div id="period-grid" class="grid" popover bind:this={grid} aria-label="Pick a payroll period">
  <header>
    <button class="btn step" aria-label="Previous year" disabled={year <= now.year - 1} onclick={() => year--}><Chevron dir="left" /></button>
    <strong>{year}</strong>
    <button class="btn step" aria-label="Next year" disabled={year >= now.year + 1} onclick={() => year++}><Chevron dir="right" /></button>
  </header>
  <div class="months">
    {#each MONTHS as name, i}
      <div class="month">
        <span class="muted">{name}</span>
        {#each [1, 2] as half}
          {@const p = { year, month: i + 1, half }}
          <button class="btn cell" class:selected={samePeriod(p, value)} class:current={samePeriod(p, now)} onclick={() => pick(p)}>
            {halfRange(p)}
          </button>
        {/each}
      </div>
    {/each}
  </div>
</div>

<style>
  .stepper {
    display: flex;
    border-radius: var(--r-control);
    box-shadow: var(--shadow-2);
  }
  .stepper .btn {
    box-shadow: none;
    border-radius: 0;
    padding: 5px 9px;
  }
  .stepper .btn:active {
    transform: none;
    background: var(--surface-alt);
  }
  .stepper .btn:first-child {
    border-radius: var(--r-control) 0 0 var(--r-control);
  }
  .stepper .btn:last-child {
    border-radius: 0 var(--r-control) var(--r-control) 0;
  }
  .label {
    anchor-name: --period;
    min-width: 186px;
    border-left: 0;
    border-right: 0;
    font-weight: 800;
  }

  .grid {
    position-anchor: --period;
    inset: auto;
    top: anchor(bottom);
    left: anchor(left);
    margin: 10px 0 0 -40px;
    padding: 12px 14px 14px;
    border: var(--line);
    background: var(--surface);
    box-shadow: var(--shadow-4);
    color: var(--ink);
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
  }
  header strong {
    font-weight: 900;
    font-size: 16px;
  }
  .step {
    min-height: 28px;
    padding: 3px 6px;
  }
  .months {
    display: grid;
    grid-template-columns: repeat(2, auto);
    gap: 9px 22px;
  }
  .month {
    display: grid;
    grid-template-columns: 36px auto auto;
    gap: 8px;
    align-items: center;
  }
  .month span {
    font-weight: 700;
  }
  .cell {
    min-height: 28px;
    min-width: 64px;
    padding: 2px 8px;
    font-weight: 600;
  }
  .cell.current {
    border-width: 3px;
    font-weight: 900;
  }
  .cell.selected {
    background: var(--lemon);
    font-weight: 900;
  }
  .cell.selected:hover {
    background: var(--lemon-hover);
  }
</style>
