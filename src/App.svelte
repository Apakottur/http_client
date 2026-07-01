<script lang="ts">
  import { onMount } from "svelte";
  import { collection, selectedRequest } from "./lib/store";
  import { loadCollection, saveCollection } from "./lib/api";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import RequestEditor from "./lib/components/RequestEditor.svelte";

  onMount(async () => {
    try { collection.set(await loadCollection()); } catch (e) { console.error(e); }
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
