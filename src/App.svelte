<script lang="ts">
  import { onMount } from "svelte";
  import { api, autoUpdate, local, type Me } from "./lib/api";
  import Onboarding from "./Onboarding.svelte";
  import Login from "./Login.svelte";
  import Board from "./Board.svelte";
  import Buddy from "./Buddy.svelte";

  let screen = $state<"boot" | "onboarding" | "login" | "board">("boot");
  let me = $state<Me | null>(null);
  let notice = $state(""); // why we're back at sign-in (offline, session ended, ...)
  let bootText = $state("Signing you in…");

  /** Automatic updates (the default): install a newer NBSBuddy before signing in. */
  async function updateFirst() {
    bootText = "Checking for updates…";
    try {
      const available = await api.checkUpdate();
      if (available) {
        bootText = `Updating NBSBuddy to ${available}…`;
        await api.installUpdate(); // NBSBuddy closes and reopens on the new version
      }
    } catch {
      // offline, GitHub blocked or a bad download: carry on and sign in
    }
    bootText = "Signing you in…";
  }

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

  onMount(async () => {
    if (autoUpdate.get() && !import.meta.env.DEV) await updateFirst(); // never from `tauri dev`
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
    <p class="sticker">{bootText}</p>
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
