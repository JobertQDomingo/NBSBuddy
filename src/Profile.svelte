<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Me, type Profile } from "./lib/api";
  import Modal from "./Modal.svelte";

  let {
    me,
    onclose,
    onsignedout,
    onerror,
  }: { me: Me; onclose: () => void; onsignedout: (message?: string) => void; onerror: (e: unknown) => string } = $props();

  let profile = $state<Profile | null>(null);
  let error = $state("");
  let busy = $state(false);

  onMount(async () => {
    try {
      profile = await api.profile();
    } catch (e) {
      error = onerror(e);
    }
  });

  const fields = $derived(
    profile
      ? ([
          ["Employee code", profile.employee_code],
          ["Position", profile.position],
          ["Department", profile.department],
          ["Division", profile.division],
          ["Team", profile.team],
          ["Branch", profile.branch],
          ["Work location", profile.work_location],
          ["Pay group", profile.pay_group],
          ["Employment", profile.employment_type],
          ["Date hired", profile.date_hired.slice(0, 10)],
          ["Tenure", profile.employment_tenure],
          ["Immediate superior", profile.immediate_superior_employee],
          ["Work email", profile.work_email_address],
        ].filter(([, v]) => v && v.trim()) as [string, string][])
      : [],
  );

  const name = $derived(
    profile ? [profile.first_name, profile.last_name].filter(Boolean).join(" ") || profile.employee_name : me.display_name,
  );
  const initials = $derived(
    name
      .split(/\s+/)
      .filter(Boolean)
      .map((w) => w[0])
      .filter((_, i, all) => i === 0 || i === all.length - 1)
      .join("")
      .toUpperCase(),
  );

  async function logout() {
    busy = true;
    try {
      await api.logout();
    } finally {
      onsignedout();
    }
  }
</script>

<Modal label="Your profile" width={520} {onclose}>
  <div class="body">
    <header>
      <span class="avatar display">{initials}</span>
      <div>
        <h2 class="display">{name}</h2>
        <p class="muted">{[profile?.position ?? me.position, profile?.department ?? me.department].filter(Boolean).join(" · ")}</p>
      </div>
    </header>

    {#if error}
      <p class="sticker danger" role="alert">{error}</p>
    {:else if !profile}
      <p class="muted">Loading your NBS profile…</p>
    {:else}
      <dl class="card">
        {#each fields as [label, value]}
          <dt>{label}</dt>
          <dd>{value}</dd>
        {/each}
      </dl>
      <p class="muted small">From your NBS profile. Change it in NBS.</p>
    {/if}

    <footer>
      <button class="btn" onclick={logout} disabled={busy}>Log out</button>
      <span class="hint muted">Also forgets the saved login on this PC.</span>
      <span class="grow"></span>
      <!-- svelte-ignore a11y_autofocus -->
      <button class="btn primary" onclick={onclose} autofocus>Close</button>
    </footer>
  </div>
</Modal>

<style>
  .body {
    padding: 22px;
    display: grid;
    gap: 16px;
  }
  header {
    display: flex;
    gap: 16px;
    align-items: center;
  }
  .avatar {
    display: grid;
    place-items: center;
    flex: none;
    width: 64px;
    height: 64px;
    border: var(--line);
    border-radius: var(--r-container);
    background: var(--mint);
    box-shadow: var(--shadow-2);
    font-size: 24px;
  }
  h2 {
    font-size: 24px;
  }
  dl {
    margin: 0;
    padding: 12px 16px;
    box-shadow: var(--shadow-3);
    display: grid;
    grid-template-columns: max-content minmax(0, 1fr);
    gap: 8px 18px;
  }
  dt {
    font-weight: 700;
    color: var(--text-muted);
  }
  dd {
    margin: 0;
    font-weight: 600;
    overflow-wrap: anywhere;
  }
  .small,
  .hint {
    font-size: 12px;
  }
  footer {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .grow {
    flex: 1;
  }
</style>
