<script lang="ts">
  // A native modal <dialog>: focus trap, Esc and top layer come from the browser.
  // Render it with {#if}; it opens on mount and asks the parent to close on Esc.
  import { onMount, type Snippet } from "svelte";

  let {
    label,
    width = 520,
    full = false,
    onclose,
    children,
  }: { label: string; width?: number; /** cover the whole window */ full?: boolean; onclose: () => void; children: Snippet } = $props();
  let el: HTMLDialogElement;

  onMount(() => {
    const opener = document.activeElement as HTMLElement | null;
    el.showModal();
    // Ctrl+Enter submits the dialog's form from any field, even a multi-line reason.
    el.addEventListener("keydown", (e) => {
      const form = (e.target as HTMLElement).closest("form");
      if (e.key !== "Enter" || !e.ctrlKey || !form) return;
      e.preventDefault();
      form.requestSubmit();
    });
    // {#if} removes the dialog without close(), which skips the browser's own focus return.
    return () => {
      el.close();
      opener?.focus();
    };
  });
</script>

<dialog
  bind:this={el}
  aria-label={label}
  class:full
  style:width={full ? undefined : `min(${width}px, calc(100vw - 32px))`}
  oncancel={(e) => {
    e.preventDefault();
    onclose();
  }}
>
  {@render children()}
</dialog>

<style>
  dialog {
    max-height: calc(100vh - 32px);
    overflow: auto;
  }
  .full {
    width: 100vw;
    height: 100vh;
    max-width: none;
    max-height: none;
    border: 0;
    border-radius: 0;
    box-shadow: none;
  }
</style>
