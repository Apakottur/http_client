<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";
  import { collection, selectedRequest, configPath } from "./lib/store";
  import { loadCollection, saveCollection, getConfigPath } from "./lib/api";
  import { apply as applyTheme, watchSystem } from "./lib/theme";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import RequestEditor from "./lib/components/RequestEditor.svelte";

  // Re-apply the theme whenever the setting changes (initial load, toggle, config switch).
  $: applyTheme($collection.settings.theme);

  // Sidebar width: track the persisted value, but during a drag use the live value.
  let dragging = false;
  let width = 320;
  $: if (!dragging) width = $collection.settings.sidebarWidth || 320;

  function startResize(e: PointerEvent) {
    e.preventDefault();
    dragging = true;
    const move = (ev: PointerEvent) => { width = Math.min(640, Math.max(200, ev.clientX)); };
    const up = async () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", up);
      dragging = false;
      const w = Math.round(width);
      collection.update((c) => ({ ...c, settings: { ...c.settings, sidebarWidth: w } }));
      try { await saveCollection(get(collection)); } catch (err) { console.error(err); }
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", up);
  }
  function nudge(e: KeyboardEvent) {
    const delta = e.key === "ArrowLeft" ? -16 : e.key === "ArrowRight" ? 16 : 0;
    if (!delta) return;
    e.preventDefault();
    const w = Math.min(640, Math.max(200, Math.round(width) + delta));
    collection.update((c) => ({ ...c, settings: { ...c.settings, sidebarWidth: w } }));
    save();
  }

  onMount(async () => {
    try {
      collection.set(await loadCollection());
      configPath.set(await getConfigPath());
    } catch (e) { console.error(e); }
    watchSystem(() => get(collection).settings.theme);
  });

  async function save() {
    let current: any;
    collection.subscribe((c) => (current = c))();
    try { await saveCollection(current); } catch (e) { console.error(e); }
  }
  function onKey(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === "s") { e.preventDefault(); save(); }
  }
</script>

<svelte:window on:keydown={onKey} />

<main class="app" style="grid-template-columns: {width}px 6px 1fr">
  <Sidebar />
  <!-- svelte-ignore a11y_no_noninteractive_tabindex a11y_no_noninteractive_element_interactions -->
  <div
    class="resizer"
    class:dragging
    role="separator"
    aria-orientation="vertical"
    aria-label="Resize sidebar"
    tabindex="0"
    on:pointerdown={startResize}
    on:keydown={nudge}
  ></div>
  {#if $selectedRequest}
    {#key $selectedRequest.id}
      <RequestEditor request={$selectedRequest} />
    {/key}
  {:else}
    <div class="empty">Select or create a request.</div>
  {/if}
</main>
