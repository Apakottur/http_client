<script lang="ts">
  import { get } from "svelte/store";
  import { open } from "@tauri-apps/plugin-dialog";
  import { collection, selectedId, configPath } from "../store";
  import { newRequest } from "../types";
  import { saveCollection, setConfigPath, getConfigPath } from "../api";
  import { resolve as resolveTheme } from "../theme";

  let search = "";
  $: sorted = [...$collection.requests].sort((a, b) => a.sortKey - b.sortKey);
  $: filtered = sorted.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()));
  $: isDark = resolveTheme($collection.settings.theme) === "dark";
  $: configName = $configPath ? $configPath.split("/").pop() : "(default)";

  function add() {
    const r = newRequest();
    collection.update((c) => ({ ...c, requests: [...c.requests, r] }));
    selectedId.set(r.id);
  }
  function del(id: string) {
    collection.update((c) => ({ ...c, requests: c.requests.filter((r) => r.id !== id) }));
    if (get(selectedId) === id) selectedId.set(null);
  }

  async function toggleTheme() {
    const next = isDark ? "light" : "dark";
    collection.update((c) => ({ ...c, settings: { ...c.settings, theme: next } }));
    try { await saveCollection(get(collection)); } catch (e) { console.error(e); }
  }

  async function pickConfig() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Config", extensions: ["json"] }],
    });
    if (typeof selected === "string") {
      try {
        collection.set(await setConfigPath(selected));
        configPath.set(await getConfigPath());
        selectedId.set(null);
      } catch (e) { console.error(e); }
    }
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
  <div class="side-foot">
    <button class="icon-btn" title="Toggle light/dark" on:click={toggleTheme}>{isDark ? "☀️" : "🌙"}</button>
    <button class="config-btn" title={$configPath} on:click={pickConfig}>📄 {configName}</button>
  </div>
</div>
