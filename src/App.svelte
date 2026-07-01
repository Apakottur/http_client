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

<main class="app">
  <Sidebar />
  {#if $selectedRequest}
    {#key $selectedRequest.id}
      <RequestEditor request={$selectedRequest} />
    {/key}
  {:else}
    <div class="empty">Select or create a request.</div>
  {/if}
</main>
