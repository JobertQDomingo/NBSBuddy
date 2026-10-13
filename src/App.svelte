<script lang="ts">
  import { onMount } from "svelte";
  import { api, local, type Me } from "./lib/api";
  import Onboarding from "./Onboarding.svelte";
  import Login from "./Login.svelte";
  import Board from "./Board.svelte";
  import Buddy from "./Buddy.svelte";

  let screen = $state<"boot" | "onboarding" | "login" | "board">("boot");
  let me = $state<Me | null>(null);
  let notice = $state(""); // why we're back at sign-in (offline, session ended, ...)

  async function restore() {
    screen = "boot";
    try {
      me = await api.restoreSession();
      screen = me ? "board" : "login";
    } catch (e) {
      notice = (e as Error).message;
      screen = "login";
    }
  }

  onMount(() => {
    if (local.get("onboarded") === "1") restore();
    else screen = "onboarding";
  });

  function onboarded() {
    local.set("onboarded", "1");
    restore();
  }

  function signedIn(user: Me) {
    me = user;
    notice = "";
    screen = "board";
  }

  function signedOut(message = "") {
    me = null;
    notice = message;
    screen = "login";
  }
</script>

{#if screen === "boot"}
  <main class="boot">
    <Buddy size={72} />
    <p class="sticker">Signing you in…</p>
  </main>
{:else if screen === "onboarding"}
  <Onboarding ondone={onboarded} />
{:else if screen === "login"}
  <Login {notice} onsignedin={signedIn} onretry={restore} />
{:else if me}
  <Board {me} onsignedout={signedOut} />
{/if}

<style>
  .boot {
    height: 100%;
    display: grid;
    place-content: center;
    justify-items: center;
    gap: 18px;
  }
</style>
