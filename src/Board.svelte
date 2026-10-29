<script lang="ts">
  import { onMount, tick, untrack } from "svelte";
  import { api, ApiError, type Me, type PeriodView, type Row, type Verdict } from "./lib/api";
  import { currentPeriod, dayLabel, isoDate, samePeriod, stepPeriod, MONTHS, halfRange, type Period } from "./lib/period";
  import PeriodPicker from "./PeriodPicker.svelte";
  import FileDialog from "./FileDialog.svelte";
  import Profile from "./Profile.svelte";
  import Settings from "./Settings.svelte";
  import Icon from "./Icon.svelte";
  import Confirm from "./Confirm.svelte";
  import WhatsNew, { markSeen, newest, unseen } from "./WhatsNew.svelte";

  let { me, onsignedout }: { me: Me; onsignedout: (message?: string) => void } = $props();

  let period = $state<Period>(currentPeriod());
  let view = $state<PeriodView | null>(null);
  let loading = $state(true);
  let loadError = $state("");
  let footer = $state("");
  let selected = $state<string | null>(null);
  let fileDate = $state<string | null>(null);
  let showProfile = $state(false);
  let showSettings = $state(false);
  let update = $state(""); // a newer version on GitHub; empty when up to date or offline
  let confirmUpdate = $state(false);
  let updating = $state(false);
  let updateError = $state(""); // shown in the update dialog, which stays open so it can't be missed
  let defaultsVersion = $state(0); // bumped on save so an open File CS/OB picks up the new defaults
  let news = $state(unseen()); // What's new steps on screen: once after an update, or reopened from Settings
  let seq = 0; // newest load wins; stale answers are dropped
  let tbody: HTMLTableSectionElement | undefined = $state();

  const shortLabel = (p: Period) => `${MONTHS[p.month - 1]} ${halfRange(p)}`;
  const initials = $derived(
    me.display_name
      .split(/\s+/)
      .filter(Boolean)
      .map((w) => w[0])
      .filter((_, i, all) => i === 0 || i === all.length - 1)
      .join("")
      .toUpperCase(),
  );

  /** Shared error handling: an ended session goes back to sign-in. */
  function failed(e: unknown): string {
    const err = e as ApiError;
    if (err?.kind === "auth") onsignedout(err.message);
    return err?.message ?? String(e);
  }

  async function load(note = "") {
    const mine = ++seq;
    const p = period;
    loading = true;
    loadError = "";
    footer = `Loading ${shortLabel(p)}…`;
    try {
      const v = await api.loadPeriod(p.year, p.month, p.half);
      if (mine !== seq) return;
      view = v;
      const at = new Date().toLocaleTimeString("en-US", { hour: "numeric", minute: "2-digit" });
      footer = `${note ? `${note} ` : ""}Loaded ${shortLabel(p)} at ${at}.`;
    } catch (e) {
      if (mine !== seq) return;
      view = null;
      loadError = failed(e);
      footer = `Couldn't load ${shortLabel(p)}: ${loadError}`;
    } finally {
      if (mine === seq) loading = false;
    }
  }

  $effect(() => {
    period; // reload whenever the period changes
    untrack(() => load());
  });

  // Offline or GitHub blocked: no button, nothing else changes.
  onMount(() => {
    api.checkUpdate().then((v) => (update = v ?? ""), () => {});
  });

  async function installUpdate() {
    updating = true;
    updateError = "";
    try {
      await api.installUpdate(); // on success NBSBuddy closes and reopens, so this only returns on failure
    } catch (e) {
      updateError = (e as Error).message;
    }
    updating = false;
  }

  function closeUpdate() {
    confirmUpdate = false;
    updateError = "";
  }

  const strip = $derived.by(() => {
    if (loading && !view) return { tone: "muted", headline: `Loading ${shortLabel(period)}…`, detail: "", meta: "", tip: "" };
    if (loadError)
      return {
        tone: "danger",
        headline: `Couldn't load ${shortLabel(period)}`,
        detail: "Check that ZeroTier is connected, then press F5 to reload.",
        meta: "",
        tip: loadError,
      };
    const s = view!.strip;
    return { tone: s.tone, headline: s.headline, detail: s.detail, meta: `${s.cutoff} · ${s.progress}`, tip: s.detail };
  });

  /** Shrink the headline (38px down to 24px) until it fits beside the rail. */
  function fit(node: HTMLElement, _text: string) {
    let width = -1;
    const run = () => {
      for (let px = 38; px >= 24; px--) {
        node.style.fontSize = `${px}px`;
        if (node.scrollWidth <= node.clientWidth) break;
      }
    };
    const ro = new ResizeObserver(([entry]) => {
      if (entry.contentRect.width !== width) (width = entry.contentRect.width), requestAnimationFrame(run);
    });
    ro.observe(node.parentElement!);
    document.fonts.ready.then(run);
    return { update: run, destroy: () => ro.disconnect() };
  }

  const pillKind: Record<Verdict, string> = { "": "", Done: "done", Waiting: "", "Due today": "due", "Needs action": "act" };

  function rowFor(date: string): Row | undefined {
    return view?.rows.find((r) => r.date === date);
  }

  async function focusRow(date: string) {
    selected = date;
    await tick();
    const tr = tbody?.querySelector<HTMLElement>(`tr[data-date="${date}"]`);
    tr?.focus();
    tr?.scrollIntoView({ block: "nearest" });
  }

  function rowKey(e: KeyboardEvent, i: number) {
    const rows = view?.rows ?? [];
    if (e.key === "Enter") fileDate = rows[i].date;
    else if (e.key === "ArrowDown" && i + 1 < rows.length) focusRow(rows[i + 1].date);
    else if (e.key === "ArrowUp" && i > 0) focusRow(rows[i - 1].date);
    else return;
    e.preventDefault();
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === "F5") e.preventDefault(); // never reload the whole page
    if (e.ctrlKey && e.key === "," && !showSettings && !showProfile && !news.length) {
      e.preventDefault();
      showSettings = true; // also over File CS/OB, to fix the customer mid-filing
      return;
    }
    if (fileDate || showProfile || showSettings || confirmUpdate || news.length) return;
    if (e.key === "F5") load();
    else if (e.ctrlKey && e.key.toLowerCase() === "n") fileDate = isoDate();
    else if (e.altKey && e.key === "ArrowLeft") period = stepPeriod(period, -1);
    else if (e.altKey && e.key === "ArrowRight") period = stepPeriod(period, 1);
    else return;
    e.preventDefault(); // F5 would otherwise reload the whole page
  }

  function filed(date: string, note: string) {
    if (note) fileDate = null; // all went through: done here
    selected = date;
    load(note);
  }

  function deleted(date: string, kinds: string[]) {
    load(`Deleted ${dayLabel(date)}'s ${kinds.join(" + ")}. File it again whenever you're ready.`);
  }
</script>

<svelte:window onkeydown={onKey} />

<div class="board">
  <header class="bar">
    <PeriodPicker bind:value={period} />
    {#if !samePeriod(period, currentPeriod())}
      <button class="link" onclick={() => (period = currentPeriod())}>Back to current period</button>
    {/if}
    <span class="grow"></span>
    {#if update}
      <button class="btn primary" onclick={() => (confirmUpdate = true)} title="NBSBuddy {update} is out. Click to update; the app restarts.">
        <Icon name="download" />Update to {update}
      </button>
    {/if}
    <button class="btn icon" onclick={() => load()} disabled={loading} aria-label="Refresh (F5)" title="Reload this period from NBS (F5)">
      <Icon name="refresh" />
    </button>
    <button
      class="btn primary icon"
      onclick={() => (fileDate = isoDate())}
      aria-label="File CS/OB (Ctrl+N)"
      title="File CS/OB: file today's OB and CS (Ctrl+N). To file another day, double-click its row."
    >
      <Icon name="plus" />
    </button>
    <button
      class="btn icon"
      onclick={() => (showSettings = true)}
      aria-label="Settings (Ctrl+,)"
      title="Settings: default reasons and customer for new filings (Ctrl+,)"
    >
      <Icon name="gear" />
    </button>
    <button class="btn me" onclick={() => (showProfile = true)} title="Your NBS profile">
      <span class="avatar">{initials}</span>{me.first_name || me.display_name}
    </button>
  </header>

  <section class="card strip" aria-live="polite">
    <div class="words">
      <h1 class="display headline {strip.tone}" title={strip.headline} use:fit={strip.headline}>{strip.headline}</h1>
      {#if strip.detail}<p class="detail" title={strip.tip}>{strip.detail}</p>{/if}
      {#if strip.meta}<p class="meta">{strip.meta}</p>{/if}
    </div>
    {#if view && !loadError}
      <div class="rail" aria-label="Workdays of this period">
        {#each view.rail as d (d.date)}
          <button
            class="tile {d.kind}"
            class:today={d.today}
            class:struck={d.struck}
            title={d.tip}
            aria-label={d.tip}
            onclick={() => focusRow(d.date)}
          >
            {d.day}
            {#if d.cutoff}<i>CUTOFF</i>{/if}
          </button>
        {/each}
      </div>
    {/if}
  </section>

  <section class="card table-card" class:loading aria-busy={loading}>
    <div class="scroll">
      <table aria-label="Payroll period days">
        <colgroup>
          <col style:width="104px" /><col style:width="128px" /><col style:width="104px" /><col style:width="72px" />
          <col style:width="252px" /><col style:width="236px" /><col />
        </colgroup>
        <thead>
          <tr><th>Date</th><th class="c">Status</th><th>OB time</th><th class="c">OB hrs</th><th>Shift</th><th>Note</th><th>Reasons</th></tr>
        </thead>
        <tbody bind:this={tbody}>
          {#each view?.rows ?? [] as row, i (row.date)}
            <tr
              data-date={row.date}
              class="tone-{row.tone || 'plain'}"
              class:today={row.today}
              class:selected={selected === row.date}
              tabindex={selected === row.date || (!selected && row.today) ? 0 : -1}
              aria-selected={selected === row.date}
              onclick={() => (selected = row.date)}
              ondblclick={() => (fileDate = row.date)}
              onkeydown={(e) => rowKey(e, i)}
            >
              <td title={row.today ? "Today" : ""}>{row.label}</td>
              <td class="c" title={row.status_tip}>
                {#if row.verdict}<span class="pill {pillKind[row.verdict]}">{row.verdict}</span>{/if}
              </td>
              <td>{row.ob_time}</td>
              <td class="c">{row.hours}</td>
              <td title={row.shift}>{row.shift}</td>
              <td title={row.note}>{row.note}</td>
              <td title={row.reasons_tip}>{row.reasons}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <p class="footer" title={footer} role="status">{footer}</p>
</div>

{#if fileDate}
  <FileDialog
    date={fileDate}
    {rowFor}
    {defaultsVersion}
    onclose={() => (fileDate = null)}
    onfiled={filed}
    ondeleted={deleted}
    onerror={failed}
    onsettings={() => (showSettings = true)}
  />
{/if}
{#if showProfile}
  <Profile {me} onclose={() => (showProfile = false)} {onsignedout} onerror={failed} />
{/if}
{#if showSettings}
  <Settings
    onclose={() => (showSettings = false)}
    onerror={failed}
    onsaved={() => {
      showSettings = false;
      defaultsVersion++;
      footer = "Saved on this PC. New filings use your new defaults.";
    }}
    onnews={() => (news = newest())}
  />
{/if}
{#if news.length}
  <WhatsNew
    steps={news}
    onclose={() => {
      markSeen();
      news = [];
    }}
  />
{/if}
{#if confirmUpdate}
  <Confirm
    title="Update to NBSBuddy {update}?"
    confirm={updating ? "Updating…" : updateError ? "Try again" : "Update now"}
    cancel={updateError ? "Close" : "Later"}
    busy={updating}
    onconfirm={installUpdate}
    oncancel={closeUpdate}
  >
    {#if updateError}
      <p class="sticker danger" role="alert">{updateError}</p>
    {:else}
      <p>NBSBuddy closes, installs the update and opens again. It takes a few seconds.</p>
    {/if}
  </Confirm>
{/if}

<style>
  .board {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 14px 16px 8px;
  }
  .bar {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .grow {
    flex: 1;
  }
  .icon {
    width: 34px;
    padding: 0;
  }
  .me {
    padding-left: 6px;
  }
  .avatar {
    display: grid;
    place-items: center;
    width: 24px;
    height: 24px;
    border: var(--line);
    border-radius: 6px;
    background: var(--mint);
    font-size: 11px;
    font-weight: 900;
  }

  /* ---- status strip ---- */
  .strip {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 8px 24px;
    padding: 14px 18px;
  }
  .words {
    min-width: 0;
    display: grid;
    gap: 7px;
    justify-items: start;
  }
  .headline {
    max-width: 100%;
    padding: 3px 10px;
    border: 2px solid transparent;
    border-radius: var(--r-control);
    font-size: 38px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .headline.danger {
    background: var(--tomato);
    border-color: var(--ink);
  }
  .headline.attention {
    background: var(--lemon);
    border-color: var(--ink);
  }
  .headline.success {
    background: var(--mint);
    border-color: var(--ink);
  }
  .headline.muted {
    padding-left: 0;
    color: var(--text-muted);
  }
  .detail {
    font-weight: 500;
  }
  .meta {
    font-weight: 800;
  }

  .rail {
    display: flex;
    gap: 5px;
    padding: 0 2px 18px 0;
  }
  .tile {
    position: relative;
    width: 32px;
    height: 42px;
    padding: 0;
    border: var(--line);
    border-radius: var(--r-tile);
    background: var(--surface);
    box-shadow: var(--shadow-1);
    color: var(--ink);
    font: 800 13px var(--font);
    cursor: pointer;
  }
  .tile.done {
    background: var(--mint);
  }
  .tile.due {
    background: var(--lemon);
  }
  .tile.act {
    background: var(--tomato);
    border-width: 3px;
  }
  .tile.up {
    border-style: dashed;
    background: transparent;
    box-shadow: none;
  }
  .tile.today {
    border-width: 4px;
  }
  .tile.struck::after {
    content: "";
    position: absolute;
    left: 3px;
    right: 3px;
    top: calc(50% - 1.25px);
    height: 2.5px;
    border-radius: 2px;
    background: var(--ink);
    transform: rotate(-55deg);
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

  /* ---- table ---- */
  .table-card {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }
  .table-card.loading table {
    opacity: 0.55;
  }
  .scroll {
    height: 100%;
    overflow: auto;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
  }
  th {
    position: sticky;
    top: 0;
    z-index: 1;
    padding: 8px;
    background: var(--lemon);
    border-bottom: var(--line);
    text-align: left;
    font-size: 12px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }
  td {
    height: 31px;
    padding: 0 8px;
    border-bottom: var(--line);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .c {
    text-align: center;
  }
  tr {
    cursor: default;
    outline: none;
  }
  tr.today td:first-child {
    font-weight: 900;
  }
  tr.tone-danger td {
    background: var(--row-mismatch);
  }
  tr.tone-attention td {
    background: var(--row-warning);
  }
  tr.tone-muted td {
    background: var(--row-weekend);
    color: var(--text-muted);
  }
  /* selection: an indigo outline, never a fill, so the row's wash stays readable */
  tr.selected td {
    box-shadow:
      inset 0 3px 0 var(--accent),
      inset 0 -3px 0 var(--accent);
  }
  tr.selected td:first-child {
    box-shadow:
      inset 3px 3px 0 var(--accent),
      inset 0 -3px 0 var(--accent);
  }
  tr.selected td:last-child {
    box-shadow:
      inset -3px 3px 0 var(--accent),
      inset 0 -3px 0 var(--accent);
  }

  .footer {
    padding: 5px 2px 0;
    border-top: var(--line);
    color: var(--on-canvas-muted);
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
