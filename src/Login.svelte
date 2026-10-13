<script lang="ts">
  import { api, local, type Me } from "./lib/api";
  import Buddy from "./Buddy.svelte";

  let {
    notice = "",
    onsignedin,
    onretry,
  }: { notice?: string; onsignedin: (me: Me) => void; onretry: () => void } = $props();

  let username = $state(local.get("username"));
  let password = $state("");
  let remember = $state(true);
  let busy = $state(false);
  let error = $state("");

  async function submit(e: SubmitEvent) {
    e.preventDefault();
    busy = true;
    error = "";
    try {
      const me = await api.login(username, password, remember);
      local.set("username", username.trim());
      onsignedin(me);
    } catch (err) {
      error = (err as Error).message;
    } finally {
      busy = false;
    }
  }
</script>

<main class="wrap">
  <form class="card panel" onsubmit={submit}>
    <header>
      <Buddy size={56} />
      <div>
        <h1 class="display">NBSBuddy</h1>
        <p class="muted">Sign in with your NBS account.</p>
      </div>
    </header>

    {#if notice && !error}
      <p class="sticker attention" role="status">
        {notice} <button type="button" class="link" onclick={onretry}>Try again</button>
      </p>
    {/if}

    <label>
      <span>Username</span>
      <!-- svelte-ignore a11y_autofocus -->
      <input bind:value={username} autocomplete="username" autofocus={!username} required />
    </label>
    <label>
      <span>Password</span>
      <!-- svelte-ignore a11y_autofocus -->
      <input type="password" bind:value={password} autocomplete="current-password" autofocus={!!username} required />
    </label>
    <label class="check">
      <input type="checkbox" bind:checked={remember} />
      <span>Remember me on this PC <span class="muted">(skip sign-in next time)</span></span>
    </label>

    {#if error}<p class="sticker danger" role="alert">{error}</p>{/if}

    <button class="btn primary" type="submit" disabled={busy}>{busy ? "Signing in…" : "Sign in"}</button>
    <p class="hint muted">Needs ZeroTier. Your password is only sent to NBS{remember ? " and kept in Windows Credential Manager" : ""}.</p>
  </form>
</main>

<style>
  .wrap {
    height: 100%;
    display: grid;
    place-items: center;
    padding: 24px;
  }
  .panel {
    width: min(420px, 100%);
    padding: 28px;
    display: grid;
    gap: 14px;
  }
  header {
    display: flex;
    gap: 14px;
    align-items: center;
    margin-bottom: 4px;
  }
  h1 {
    font-size: 32px;
  }
  label {
    display: grid;
    gap: 5px;
    font-weight: 700;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 9px;
    font-weight: 600;
  }
  .hint {
    font-size: 12px;
  }
</style>
