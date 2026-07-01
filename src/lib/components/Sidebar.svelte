<script lang="ts">
  import { collection, selectedId } from "../store";
  import { newRequest } from "../types";
  import { get } from "svelte/store";
  let search = "";
  $: sorted = [...$collection.requests].sort((a, b) => a.sortKey - b.sortKey);
  $: filtered = sorted.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()));

  function add() {
    const r = newRequest();
    collection.update((c) => ({ ...c, requests: [...c.requests, r] }));
    selectedId.set(r.id);
  }
  function del(id: string) {
    collection.update((c) => ({ ...c, requests: c.requests.filter((r) => r.id !== id) }));
    if (get(selectedId) === id) selectedId.set(null);
  }
</script>

<div class="sidebar">
  <div class="side-head">
    <input placeholder="Search…" bind:value={search} />
    <button on:click={add}>New</button>
  </div>
  <div class="req-list">
    {#each filtered as r}
      <div
        class="req-row"
        class:active={$selectedId === r.id}
        role="button"
        tabindex="0"
        on:click={() => selectedId.set(r.id)}
        on:keydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); selectedId.set(r.id); } }}
      >
        <span class={"badge m-" + r.method.toLowerCase()}>{r.method}</span>
        <span class="name">{r.name}</span>
        <button class="del" on:click|stopPropagation={() => del(r.id)}>✕</button>
      </div>
    {/each}
  </div>
</div>
