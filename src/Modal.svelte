<script lang="ts">
  // A native modal <dialog>: focus trap, Esc and top layer come from the browser.
  // Render it with {#if}; it opens on mount and asks the parent to close on Esc.
  import { onMount, type Snippet } from "svelte";

  let { label, width = 520, onclose, children }: { label: string; width?: number; onclose: () => void; children: Snippet } =
    $props();
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
  style:width="min({width}px, calc(100vw - 32px))"
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
</style>
