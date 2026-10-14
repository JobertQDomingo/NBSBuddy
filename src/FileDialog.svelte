<script lang="ts">
  import { untrack } from "svelte";
  import { api, defaults, DEFAULTS, type FileOutcome, type Row, type ShiftOption } from "./lib/api";
  import { dayLabel, parseIso } from "./lib/period";
  import Modal from "./Modal.svelte";
  import Confirm from "./Confirm.svelte";

  let {
    date: initialDate,
    rowFor,
    defaultsVersion = 0,
    onclose,
    onfiled,
    ondeleted,
    onerror,
    onsettings,
  }: {
    date: string;
    /** The loaded period's row for a date; undefined outside that period. */
    rowFor: (date: string) => Row | undefined;
    /** Bumped when Settings saves while this dialog is open. */
    defaultsVersion?: number;
    onclose: () => void;
    /** note is empty when something failed (the dialog stays open). */
    onfiled: (date: string, note: string) => void;
    ondeleted: (date: string, kinds: string[]) => void;
    onerror: (e: unknown) => string;
    onsettings: () => void;
  } = $props();

  let date = $state(untrack(() => initialDate));
  let shifts = $state<ShiftOption[]>([]);
  let shiftId = $state(0);
  let timeIn = $state("");
  let timeOut = $state("");
  const initial = defaults.get(); // edits here are for this filing only; Settings owns the defaults
  let start = $state(initial);
  let obReason = $state(initial.obReason);
  let csReason = $state(initial.csReason);
  let busy = $state(false);
  let error = $state("");
  let confirmWeekend = $state(false);
  let deleting = $state<string[] | null>(null); // kinds ticked in the delete confirm

  const row = $derived(rowFor(date));
  const ob = $derived(row?.ob ?? null);
  const cs = $derived(row?.cs ?? null);
  const filedView = $derived(!!(ob && cs)); // nothing left to file: show what NBS has
  const weekend = $derived([0, 6].includes(parseIso(date).getDay()));
  const deletable = $derived([ob?.deletable ? "OB" : "", cs?.deletable ? "CS" : ""].filter(Boolean));
  const shift = $derived(shifts.find((s) => s.id === shiftId));
  const ownCustomer = $derived(start.customer !== DEFAULTS.customer);

  // Settings saved while this dialog is open: take the new customer, and new reasons you haven't edited here.
  $effect(() => {
    defaultsVersion;
    untrack(() => {
      const next = defaults.get();
      if (obReason === start.obReason) obReason = next.obReason;
      if (csReason === start.csReason) csReason = next.csReason;
      start = next;
    });
  });

  $effect(() => {
    const d = date;
    untrack(() => loadShifts(d));
  });

  async function loadShifts(d: string) {
    if (!d) return;
    try {
      const list = await api.shiftsFor(d);
      if (d !== date) return;
      shifts = list;
      pickShift(list[0]?.id ?? 0);
    } catch (e) {
      error = onerror(e);
    }
  }

  function pickShift(id: number) {
    shiftId = id;
    const s = shifts.find((x) => x.id === id);
    timeIn = s?.time_in ?? "";
    timeOut = s?.time_out ?? "";
  }

  const minutes = (t: string) => {
    const m = /^(\d{1,2}):(\d{2})/.exec(t);
    return m ? +m[1] * 60 + +m[2] : null;
  };
  const hours = $derived.by(() => {
    const a = minutes(timeIn), b = minutes(timeOut);
    if (a === null || b === null) return null;
    let span = b - a;
    const overnight = !!shift?.time_in && shift.time_out <= shift.time_in;
    if (span <= 0 && overnight) span += 1440;
    return span > 0 ? Math.round((span / 60) * 100) / 100 : 0;
  });

  const problem = $derived.by(() => {
    if (filedView) return "";
    if (!shift) return shifts.length || !date ? "Pick a shift." : "";
    if (!timeIn) return "Enter a time in, like 07:30.";
    if (!timeOut) return "Enter a time out, like 17:30.";
    if (!hours) return "Time out must be later than time in.";
    return "";
  });

  const note = $derived.by(() => {
    const out: string[] = [];
    if (weekend) out.push(`${parseIso(date).toLocaleDateString("en-US", { weekday: "long" })} is a weekend; you'll be asked to confirm.`);
    if (row) {
      if (ob && cs) out.push("OB and CS are already filed for this date. This shows what NBS has.");
      else if (ob || cs) {
        const [kind, f] = ob ? ["OB", ob] : ["CS", cs!];
        out.push(`${kind} is already filed (${f.status}), so only ${kind === "OB" ? "CS" : "OB"} will be filed.`);
      } else out.push("Nothing filed for this date yet.");
      const bad = [ob?.status === "Disapproved" ? "OB" : "", cs?.status === "Disapproved" ? "CS" : ""].filter(Boolean);
      if (bad.length)
        out.push(`${bad.join(" and ")} ${bad.length === 1 ? "was" : "were"} disapproved: refile in NBS, since NBSBuddy skips dates that already have a filing.`);
    }
    return out.join(" ");
  });
  const noteTone = $derived(weekend || ob || cs ? "attention" : "");

  function filedText(o: FileOutcome) {
    const part = (kind: string, created: boolean, what: string) => (created ? `${kind} ${what}` : `${kind} already filed, skipped`);
    return `Filed ${dayLabel(date)}: ${part("OB", o.ob.state === "created", `${timeIn}–${timeOut}`)}, ${part("CS", o.cs.state === "created", shift!.name)}.`;
  }

  async function file() {
    confirmWeekend = false;
    busy = true;
    error = "";
    try {
      const out = await api.fileDay({
        date,
        shift_id: shift!.id,
        shift_name: shift!.name,
        time_in: timeIn,
        time_out: timeOut,
        ob_reason: obReason,
        cs_reason: csReason,
        customer: start.customer,
      });
      const failed = (["OB", "CS"] as const).filter((k) => out[k === "OB" ? "ob" : "cs"].state === "failed");
      if (failed.length) {
        error =
          failed.map((k) => `${k} didn't go through: ${out[k === "OB" ? "ob" : "cs"].message}`).join(" ") +
          " Anything that went through is saved in NBS and will be skipped next time.";
        onfiled(date, "");
      } else onfiled(date, filedText(out));
    } catch (e) {
      error = onerror(e);
    } finally {
      busy = false;
    }
  }

  function submit(e: SubmitEvent) {
    e.preventDefault();
    if (problem || busy || filedView) return; // Ctrl+Enter submits even a fully filed day's form
    if (weekend) confirmWeekend = true;
    else file();
  }

  async function remove() {
    if (!deleting?.length) return;
    busy = true;
    error = "";
    try {
      const done = await api.deleteFilings(date, deleting);
      deleting = null;
      ondeleted(date, done);
    } catch (e) {
      deleting = null;
      error = onerror(e);
    } finally {
      busy = false;
    }
  }

  const toggle = (kind: string, on: boolean) =>
    (deleting = on ? [...(deleting ?? []), kind] : (deleting ?? []).filter((k) => k !== kind));
</script>

<Modal label="File CS/OB" width={560} onclose={() => !busy && onclose()}>
  <form class="body" onsubmit={submit}>
    <h2 class="display">File CS/OB</h2>
    {#if note}<p class="note" class:sticker={noteTone} class:attention={noteTone}>{note}</p>{/if}

    <fieldset class="card">
      <legend class="overline">Day and shift</legend>
      <label for="f-date">Date</label>
      <input id="f-date" type="date" bind:value={date} required disabled={busy} />

      <label for="f-shift">Shift</label>
      {#if filedView}
        <input id="f-shift" value={cs?.shift ?? ""} readonly />
      {:else}
        <select id="f-shift" value={shiftId} onchange={(e) => pickShift(+e.currentTarget.value)} disabled={busy || !shifts.length}>
          {#if !shifts.length}<option value={0}>Loading shifts…</option>{/if}
          {#each shifts as s (s.id)}<option value={s.id}>{s.name}</option>{/each}
        </select>
      {/if}

      <label for="f-in">OB time</label>
      <div class="times">
        {#if filedView}
          <input id="f-in" value={ob?.time_in ?? ""} readonly aria-label="OB time in" />
          <span class="muted">to</span>
          <input value={ob?.time_out ?? ""} readonly aria-label="OB time out" />
          <b class="hours">{ob?.hours ?? ""}</b>
        {:else}
          <input id="f-in" type="time" bind:value={timeIn} aria-label="OB time in" aria-invalid={!timeIn && !!shift} disabled={busy} />
          <span class="muted">to</span>
          <input type="time" bind:value={timeOut} aria-label="OB time out" aria-invalid={!!shift && (!timeOut || !hours)} disabled={busy} />
          <b class="hours">{hours ? String(hours) : ""}</b>
        {/if}
        <span class="muted">hours</span>
      </div>
    </fieldset>

    <fieldset class="card">
      <legend class="overline">Reasons and customer</legend>
      <label for="f-ob">OB reason</label>
      {#if filedView}
        <textarea id="f-ob" value={ob?.reason ?? ""} readonly rows="1"></textarea>
      {:else}
        <textarea id="f-ob" bind:value={obReason} rows="1" disabled={busy}></textarea>
      {/if}
      <label for="f-cs">CS reason</label>
      {#if filedView}
        <textarea id="f-cs" value={cs?.reason ?? ""} readonly rows="1"></textarea>
      {:else}
        <textarea id="f-cs" bind:value={csReason} rows="1" disabled={busy}></textarea>
      {/if}
      <span class="label">Customer</span>
      <p class="customer">
        {#if ob}
          <b>{ob.customer}</b>
        {:else}
          <b class:own={ownCustomer}>{start.customer}</b>
          <span class="muted">{ownCustomer ? "your pick ·" : "·"}</span>
          <button type="button" class="link" onclick={onsettings} disabled={busy}>Change in Settings</button>
        {/if}
      </p>
    </fieldset>

    {#if error || (problem && shifts.length)}
      <p class="sticker danger" role="alert">{error || problem}</p>
    {/if}

    <footer>
      {#if deletable.length}
        <button type="button" class="btn danger" disabled={busy} onclick={() => (deleting = [...deletable])}>
          Delete {deletable.join(" + ")}…
        </button>
      {/if}
      <span class="grow"></span>
      {#if !filedView}
        <!-- svelte-ignore a11y_autofocus -->
        <button class="btn primary" type="submit" disabled={busy || !!problem} autofocus title="Ctrl+Enter">{busy ? "Filing…" : "File CS/OB"}</button>
      {/if}
      <!-- svelte-ignore a11y_autofocus -->
      <button type="button" class="btn" onclick={onclose} disabled={busy} autofocus={filedView}>{filedView ? "Close" : "Cancel"}</button>
    </footer>
  </form>
</Modal>

{#if confirmWeekend}
  <Confirm title="File on a weekend?" confirm="File anyway" onconfirm={file} oncancel={() => (confirmWeekend = false)}>
    <p>{dayLabel(date)} is a weekend. File OB and CS for it anyway?</p>
  </Confirm>
{/if}

{#if deleting}
  <Confirm
    title="Delete {dayLabel(date)}'s filing?"
    confirm={busy ? "Deleting…" : "Delete"}
    cancel="Keep it"
    danger
    {busy}
    disabled={!deleting.length}
    onconfirm={remove}
    oncancel={() => (deleting = null)}
  >
    <p>NBS voids what you tick. The day can be filed again right after.</p>
    <div class="ticks">
      {#each deletable as kind}
        {@const f = kind === "OB" ? ob : cs}
        <label>
          <input type="checkbox" checked={deleting.includes(kind)} onchange={(e) => toggle(kind, e.currentTarget.checked)} />
          <span><b>{kind}</b> · {f?.status} · {kind === "OB" ? `${f?.time_in}–${f?.time_out}` : f?.shift}</span>
        </label>
      {/each}
    </div>
  </Confirm>
{/if}

<style>
  .body {
    padding: 20px;
    display: grid;
    gap: 14px;
  }
  h2 {
    font-size: 24px;
  }
  .note {
    color: var(--on-canvas-muted);
    font-weight: 600;
  }
  .note.sticker {
    color: var(--ink);
  }
  fieldset {
    margin: 0;
    padding: 12px 14px 14px;
    box-shadow: var(--shadow-3);
    display: grid;
    grid-template-columns: 92px minmax(0, 1fr);
    gap: 10px 12px;
    align-items: center;
    min-width: 0;
  }
  legend {
    float: left;
    grid-column: 1 / -1;
    padding: 0;
  }
  fieldset label,
  .label {
    font-weight: 600;
  }
  .customer {
    overflow-wrap: anywhere;
  }
  .customer .link {
    padding: 0 2px;
  }
  .own {
    padding: 1px 6px;
    border: var(--line);
    border-radius: var(--r-pill);
    background: var(--lemon);
    box-shadow: var(--shadow-1);
    margin-right: 4px; /* clear of its hard shadow */
  }
  .times {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .times input {
    width: 124px;
  }
  .hours {
    margin-left: auto;
    min-width: 40px;
    text-align: right;
    font-weight: 800;
  }
  footer {
    display: flex;
    gap: 10px;
    align-items: center;
  }
  .grow {
    flex: 1;
  }
  .ticks {
    display: grid;
    gap: 8px;
  }
  .ticks label {
    display: flex;
    align-items: center;
    gap: 9px;
  }
</style>
