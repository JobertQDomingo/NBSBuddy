<script lang="ts">
  import { api } from "./lib/api";
  import Buddy from "./Buddy.svelte";

  let { ondone }: { ondone: () => void } = $props();

  let step = $state(0);
  let reach = $state<"checking" | "ok" | "no">("checking");

  async function check() {
    reach = "checking";
    reach = (await api.serverReachable().catch(() => false)) ? "ok" : "no";
  }

  $effect(() => {
    if (step === 1) check();
  });

  const legend = [
    ["done", "Done", "OB and CS are both approved."],
    ["", "Waiting", "Filed, waiting for approval."],
    ["due", "Due today", "Today isn't fully filed yet."],
    ["act", "Needs action", "Missing, disapproved, or the times don't match the shift."],
  ];
</script>

<main class="wrap">
  <section class="card panel" aria-live="polite">
    {#if step === 0}
      <Buddy size={96} />
      <h1 class="display">Hi, I'm NBSBuddy.</h1>
      <p>
        I file your work-from-home <b>Official Business (OB)</b> and <b>Change Shift (CS)</b> in NBS in a
        couple of clicks, and show which days still need action before payroll cutoff.
      </p>
    {:else if step === 1}
      <h1 class="display">Connect ZeroTier first.</h1>
      <p>NBS only answers on the company ZeroTier network. Keep ZeroTier connected whenever you use NBSBuddy.</p>
      <div class="row">
        {#if reach === "checking"}
          <span class="sticker">Looking for NBS…</span>
        {:else if reach === "ok"}
          <span class="sticker success">NBS is reachable. You're set.</span>
        {:else}
          <span class="sticker danger">Can't reach NBS yet. Connect ZeroTier.</span>
          <button class="btn" onclick={check}>Check again</button>
        {/if}
      </div>
    {:else}
      <h1 class="display">Read the board.</h1>
      <p>Every workday of the payroll period gets one sticker:</p>
      <dl class="legend">
        {#each legend as [kind, name, meaning]}
          <dt><span class="pill {kind}">{name}</span></dt>
          <dd>{meaning}</dd>
        {/each}
      </dl>
      <ul>
        <li>Double-click a day, or select it and press Enter, to file it.</li>
        <li>Filed by mistake? Open that day and delete it while it's still <i>For Approval</i>, then file again.</li>
      </ul>
    {/if}

    <footer>
      <div class="dots" aria-label="Step {step + 1} of 3">
        {#each [0, 1, 2] as i}<span class:on={i === step}></span>{/each}
      </div>
      {#if step > 0}<button class="link" onclick={() => step--}>Back</button>{/if}
      {#if step < 2}
        <button class="btn primary" onclick={() => step++}>Next</button>
      {:else}
        <button class="btn primary" onclick={ondone}>Sign in</button>
      {/if}
    </footer>
  </section>
</main>

<style>
  .wrap {
    height: 100%;
    display: grid;
    place-items: center;
    padding: 24px;
  }
  .panel {
    width: min(580px, 100%);
    min-height: 420px;
    padding: 32px 32px 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  h1 {
    font-size: 36px;
  }
  p,
  li {
    font-size: 15px;
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
