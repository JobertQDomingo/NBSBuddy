<script lang="ts">
  import { onMount, tick } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { api, autoUpdate, defaults, DEFAULTS, type Defaults } from "./lib/api";
  import Modal from "./Modal.svelte";

  let {
    onclose,
    onsaved,
    onerror,
    onnews,
  }: { onclose: () => void; onsaved: () => void; onerror: (e: unknown) => string; /** open What's new */ onnews: () => void } = $props();

  const saved = defaults.get();
  let d = $state({ ...saved });
  let customers = $state<string[] | null>(null); // null while loading
  let listError = $state(""); // kept through a retry, so its focused Try again button stays put
  let retryButton: HTMLButtonElement | undefined = $state();
  let touched = $state<Partial<Record<keyof Defaults, boolean>>>({});
  let attempted = $state(false);

  async function loadCustomers() {
    if (customers === null && listError) return; // a retry is already running
    const refocus = !!retryButton && document.activeElement === retryButton;
    customers = null;
    try {
      const list = [...new Set(await api.customers())].sort((a, b) => a.localeCompare(b));
      const match = list.find((c) => same(c, d.customer));
      if (match) d.customer = match; // NBS's own spelling, so the dropdown shows it
      customers = list;
      listError = "";
      if (refocus) {
        await tick();
        document.getElementById("s-customer")?.focus(); // Try again is gone: land on what it unlocked
      }
    } catch (e) {
      listError = onerror(e); // an ended session goes back to sign-in
      customers = [];
    }
  }
  onMount(loadCustomers);

  // Updates: their own card, saved the moment they change (not part of Save defaults).
  let version = $state("");
  let latest = $state<string | null | undefined>(undefined); // undefined until checked, null when up to date
  let updateNote = $state(""); // the last check's or install's error
  let busy = $state(false);
  onMount(() => getVersion().then((v) => (version = v)));

  async function checkNow() {
    busy = true;
    updateNote = "";
    try {
      latest = await api.checkUpdate();
    } catch (e) {
      updateNote = (e as Error).message;
    }
    busy = false;
  }

  async function installNow() {
    busy = true;
    try {
      await api.installUpdate(); // NBSBuddy closes and reopens on success
    } catch (e) {
      updateNote = (e as Error).message;
    }
    busy = false;
  }

  const same = (a: string, b: string) => a.trim().toLowerCase() === b.trim().toLowerCase();
  const picker = $derived(!!customers?.length); // the customer only changes against NBS's own list
  const unknown = $derived(picker && !customers!.some((c) => same(c, d.customer)));
  const errors = $derived<Record<keyof Defaults, string>>({
    obReason: d.obReason.trim() ? "" : "Enter an OB reason.",
    csReason: d.csReason.trim() ? "" : "Enter a CS reason.",
    customer: unknown ? "NBS has no customer with the saved name. Pick one from the Customer list." : "",
  });
  // Shown once you leave a field or press Save, never while typing. An unknown saved customer shows at once.
  const shown = (k: keyof Defaults) => !!errors[k] && (k === "customer" || attempted || !!touched[k]);
  const first = $derived((["obReason", "csReason", "customer"] as const).find(shown));
  const dirty = $derived((Object.keys(saved) as (keyof Defaults)[]).some((k) => d[k].trim() !== saved[k]));
  const builtIn = $derived((Object.keys(DEFAULTS) as (keyof Defaults)[]).every((k) => d[k].trim() === DEFAULTS[k]));
  const ids = { obReason: "s-ob", csReason: "s-cs", customer: "s-customer" };
  const describe = (k: keyof Defaults, hint = "") => [first === k ? "s-problem" : "", hint].filter(Boolean).join(" ") || undefined;

  function save(e: SubmitEvent) {
    e.preventDefault();
    if (!dirty) return; // Ctrl+Enter submits even while Save is disabled
    attempted = true;
    const bad = (["obReason", "csReason", "customer"] as const).find((k) => errors[k]);
    if (bad) return document.getElementById(ids[bad])?.focus();
    defaults.set(d);
    onsaved();
  }
</script>

<Modal label="Settings" width={560} {onclose}>
  <form class="body" onsubmit={save}>
    <h2 class="display">Settings</h2>
    <p class="note">Every new filing starts with these, and you can still change the reasons for one filing in File CS/OB. Saved on this PC.</p>

    <fieldset class="card">
      <legend class="overline">Filing defaults</legend>
      <label for="s-ob">OB reason</label>
      <textarea
        id="s-ob"
        bind:value={d.obReason}
        rows="1"
        aria-invalid={first === "obReason"}
        aria-describedby={describe("obReason")}
        onblur={() => (touched.obReason = true)}
      ></textarea>
      <label for="s-cs">CS reason</label>
      <textarea
        id="s-cs"
        bind:value={d.csReason}
        rows="1"
        aria-invalid={first === "csReason"}
        aria-describedby={describe("csReason")}
        onblur={() => (touched.csReason = true)}
      ></textarea>
      <label for="s-customer">Customer</label>
      {#if picker}
        <select id="s-customer" bind:value={d.customer} aria-invalid={first === "customer"} aria-describedby={describe("customer", "s-customer-hint")}>
          {#if unknown}<option value={d.customer}>{d.customer} (not in NBS)</option>{/if}
          {#each customers! as c}<option value={c}>{c}</option>{/each}
        </select>
      {:else}
        <select id="s-customer" disabled aria-describedby="s-customer-hint"><option>{d.customer}</option></select>
      {/if}
      <p id="s-customer-hint" class="hint muted">
        {customers === null
          ? "Loading NBS's customer list…"
          : picker
            ? "Every OB you file goes to this NBS customer."
            : listError
              ? "Connect to NBS to change the customer."
              : "NBS sent no customers, so this can't be changed right now."}
      </p>
    </fieldset>

    <fieldset class="card">
      <legend class="overline">Updates</legend>
      <span class="key">Version</span>
      <div class="row">
        <span>{version} · <button type="button" class="link" onclick={onnews}>What's new</button></span>
        <button type="button" class="btn" onclick={checkNow} disabled={busy}>{busy ? "Working…" : "Check for updates"}</button>
      </div>
      {#if updateNote}
        <p class="sticker danger wide" role="alert">{updateNote}</p>
      {:else}
        <p class="hint" role="status">
          {#if latest}{latest} is out. <button type="button" class="link" onclick={installNow} disabled={busy}>Update now</button> NBSBuddy restarts.
          {:else if latest === null}You're up to date.{/if}
        </p>
      {/if}
      <label class="auto">
        <input type="checkbox" checked={autoUpdate.get()} onchange={(e) => autoUpdate.set(e.currentTarget.checked)} />
        Install updates automatically when NBSBuddy starts
      </label>
    </fieldset>

    {#if listError}
      <p class="sticker attention" role="status">
        {listError}
        <button type="button" class="link" onclick={loadCustomers} bind:this={retryButton}>{customers === null ? "Trying…" : "Try again"}</button>
      </p>
    {/if}
    {#if first}<p id="s-problem" class="sticker danger" role="alert">{errors[first]}</p>{/if}

    <footer>
      <button
        type="button"
        class="btn"
        onclick={() => (d = { ...DEFAULTS })}
        disabled={builtIn}
        title="{DEFAULTS.obReason} · {DEFAULTS.csReason} · {DEFAULTS.customer}"
      >
        Reset to app defaults
      </button>
      <span class="grow"></span>
      <button class="btn primary" type="submit" disabled={!dirty} title="Ctrl+Enter">Save defaults</button>
      <button type="button" class="btn" onclick={onclose}>Cancel</button>
    </footer>
  </form>
</Modal>

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
  fieldset label {
    font-weight: 600;
  }
  .hint {
    grid-column: 2;
    margin-top: -4px;
    font-size: 12px;
  }
  .attention .link,
  .row .link {
    padding: 0 2px;
  }
  .key {
    font-weight: 600;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }
  .wide {
    grid-column: 1 / -1;
  }
  .auto {
    grid-column: 2;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  footer {
    display: flex;
    gap: 10px;
    align-items: center;
  }
  .grow {
    flex: 1;
  }
</style>
