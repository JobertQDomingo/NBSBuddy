<script lang="ts">
  import type { Snippet } from "svelte";
  import Modal from "./Modal.svelte";

  let {
    title,
    confirm,
    cancel = "Go back",
    danger = false,
    busy = false,
    disabled = false,
    onconfirm,
    oncancel,
    children,
  }: {
    title: string;
    confirm: string;
    cancel?: string;
    danger?: boolean;
    busy?: boolean;
    disabled?: boolean;
    onconfirm: () => void;
    oncancel: () => void;
    children: Snippet;
  } = $props();
</script>

<Modal label={title} width={440} onclose={() => !busy && oncancel()}>
  <div class="body">
    <h2 class="display">{title}</h2>
    {@render children()}
    <footer>
      <button class="btn {danger ? 'danger' : 'primary'}" onclick={onconfirm} disabled={busy || disabled}>{confirm}</button>
      <!-- svelte-ignore a11y_autofocus -->
      <button class="btn" onclick={oncancel} disabled={busy} autofocus>{cancel}</button>
    </footer>
  </div>
</Modal>

<style>
  .body {
    padding: 20px;
    display: grid;
    gap: 14px;
  }
  h2 {
    font-size: 22px;
  }
  footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 4px;
  }
</style>
