<script lang="ts">
  // The Shift field: a combobox that knows times. "730", "7:30 am", "8-6", "17:30", "night"
  // each find their shift; every typed word must start one of the shift's keys (shifts.rs search_keys).
  import Chevron from "./Chevron.svelte";
  import type { ShiftOption } from "./lib/api";

  let {
    id,
    shifts,
    value,
    disabled = false,
    onpick,
  }: { id: string; shifts: ShiftOption[]; value: number; disabled?: boolean; onpick: (id: number) => void } = $props();

  const FILLER = ["to", "till", "until", "from", "at", "start", "starts", "end", "ends", "shift"];
  const TIME = /^\d[\d:]*(?:[ap]m?)?$/; // "10" means a time, never the "10h" in "CW 10H"
  const words = (q: string) =>
    q
      .toLowerCase()
      .replace(/(?<![a-z])([ap])\.m\b\.?/g, "$1m") // "p.m." -> "pm"
      .replace(/(\d)\.(\d\d)\b/g, "$1:$2") // "7.30" -> "7:30"
      .replace(/(\d)\s+(?=[ap]m?\b)/g, "$1") // "7:30 am" -> "7:30am"
      .split(/[\s,–-]+/)
      .filter((w) => w && !FILLER.includes(w));
  const hit = (k: string, w: string) => k.startsWith(w) && (!TIME.test(w) || TIME.test(k));

  let query = $state<string | null>(null); // null until you type: the field shows the picked shift
  let open = $state(false);
  let active = $state(0);
  const picked = $derived(shifts.find((s) => s.id === value));
  const matches = $derived.by(() => {
    const ws = words(query ?? "");
    return shifts.filter((s) => ws.every((w) => s.keys.some((k) => hit(k, w))));
  });
  const optionId = (s: ShiftOption | undefined) => (s ? `${id}-${s.id}` : undefined);

  $effect(() => {
    if (disabled) close(); // filing started
  });
  $effect(() => {
    if (open) document.getElementById(optionId(matches[active]) ?? "")?.scrollIntoView({ block: "nearest" });
  });

  function show() {
    open = true;
    active = Math.max(0, matches.findIndex((s) => s.id === value));
  }

  function close() {
    open = false;
    query = null;
  }

  function pick(s: ShiftOption) {
    if (s.id !== value) onpick(s.id); // re-picking would reset the OB times you typed
    close();
  }

  function key(e: KeyboardEvent) {
    const n = matches.length;
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      if (!open) show();
      else if (n) active = (active + (e.key === "ArrowDown" ? 1 : n - 1)) % n;
    } else if (e.key === "Enter" && !e.ctrlKey) {
      e.preventDefault(); // pick or open, never file: that's Ctrl+Enter
      if (!open) show();
      else if (matches[active]) pick(matches[active]);
    } else if (open && e.key === "Escape") {
      e.preventDefault(); // close the list, not the dialog
      close();
    } else if (open && e.key === "Tab" && matches[active]) pick(matches[active]);
  }
</script>

<div class="combo">
  <input
    {id}
    role="combobox"
    aria-expanded={open}
    aria-controls="{id}-list"
    aria-autocomplete="list"
    aria-activedescendant={open ? optionId(matches[active]) : undefined}
    autocomplete="off"
    spellcheck="false"
    placeholder={shifts.length ? "Search shifts" : "Loading shifts…"}
    value={query ?? picked?.name ?? ""}
    {disabled}
    oninput={(e) => {
      query = e.currentTarget.value;
      open = true;
      active = 0;
    }}
    onfocus={(e) => e.currentTarget.select()}
    onclick={(e) => {
      if (open) return;
      show();
      e.currentTarget.select(); // the click's mouseup undid the focus select
    }}
    onkeydown={key}
    onblur={() => (query !== null && matches[active] ? pick(matches[active]) : close())}
  />
  <span class="chevron"><Chevron /></span>
  {#if open}
    <!-- mousedown would blur the input and close the list before the click lands -->
    <ul id="{id}-list" role="listbox" aria-label="Shifts" onmousedown={(e) => e.preventDefault()}>
      {#each matches as s, i (s.id)}
        <!-- svelte-ignore a11y_click_events_have_key_events (keys stay on the input: aria-activedescendant) -->
        <li id={optionId(s)} role="option" aria-selected={i === active} class:active={i === active} class:picked={s.id === value} onclick={() => pick(s)}>
          {s.name}
        </li>
      {:else}
        <li class="none muted" role="option" aria-selected="false" aria-disabled="true">No shift matches “{query}”. Try a time like 730, 8am or 17:30.</li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .combo {
    position: relative;
    min-width: 0;
  }
  input {
    padding-right: 30px; /* clear of the chevron */
    text-overflow: ellipsis;
  }
  .chevron {
    position: absolute;
    right: 10px;
    top: 50%;
    translate: 0 -50%;
    display: flex;
    pointer-events: none;
  }
  ul {
    position: absolute;
    z-index: 1;
    top: calc(100% + 6px);
    left: 0;
    right: 0;
    max-height: 236px;
    overflow: auto;
    margin: 0;
    padding: 4px;
    list-style: none;
    border: var(--line);
    background: var(--surface);
    box-shadow: var(--shadow-3);
  }
  li {
    padding: 6px 8px;
    border-radius: var(--r-pill);
    cursor: pointer;
  }
  li:hover {
    background: var(--hover);
  }
  li.picked {
    background: var(--lemon);
    font-weight: 800;
  }
  li.active {
    outline: 3px solid var(--accent);
    outline-offset: -3px;
  }
  li.none {
    cursor: default;
    background: none;
  }
</style>
