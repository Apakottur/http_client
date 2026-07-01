<script lang="ts">
  import { get } from "svelte/store";
  import { open } from "@tauri-apps/plugin-dialog";
  import { collection, selectedId, configPath } from "../store";
  import { newRequest } from "../types";
  import type { Request } from "../types";
  import { saveCollection, setConfigPath, getConfigPath } from "../api";
  import { resolve as resolveTheme } from "../theme";
  import { ENVS, envOf, topLevelTags } from "../tags";

  let search = "";
  let mainFilter = "";
  let envFilter = "";
  let roleFilter = "";
  let collapsed = new Set<string>();

  $: isDark = resolveTheme($collection.settings.theme) === "dark";
  $: configName = $configPath ? $configPath.split("/").pop() : "(default)";
  $: topTags = topLevelTags($collection.requests);

  $: matches = $collection.requests.filter(
    (r) =>
      r.name.toLowerCase().includes(search.toLowerCase()) &&
      (!mainFilter || r.topLevelTag === mainFilter) &&
      (!envFilter || r.tags.includes(envFilter)) &&
      (!roleFilter || r.tags.includes(roleFilter)),
  );
  $: groups = groupByTop(matches);

  function groupByTop(reqs: Request[]): [string, Request[]][] {
    const map = new Map<string, Request[]>();
    for (const r of reqs) {
      const k = r.topLevelTag || "(untagged)";
      if (!map.has(k)) map.set(k, []);
      map.get(k)!.push(r);
    }
    for (const list of map.values()) {
      list.sort((a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: "base", numeric: true }));
    }
    return [...map.entries()].sort((a, b) => a[0].localeCompare(b[0]));
  }

  function toggleGroup(tag: string) {
    if (collapsed.has(tag)) collapsed.delete(tag);
    else collapsed.add(tag);
    collapsed = new Set(collapsed);
  }

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
    const current = get(configPath);
    const selected = await open({
      multiple: false,
      defaultPath: current || undefined,
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
  <div class="filter-bar">
    <select bind:value={mainFilter} title="Filter by top-level tag">
      <option value="">all tags</option>
      {#each topTags as t}<option value={t}>{t}</option>{/each}
    </select>
    <select bind:value={envFilter} title="Filter by environment">
      <option value="">all envs</option>
      {#each ENVS as e}<option value={e}>{e}</option>{/each}
    </select>
    <select bind:value={roleFilter} title="Filter by role">
      <option value="">all</option>
      <option value="admin">admin</option>
      <option value="user">user</option>
    </select>
  </div>

  <div class="req-list">
    {#each groups as [tag, reqs] (tag)}
      <div class="group">
        <button class="group-head" on:click={() => toggleGroup(tag)}>
          <span class="caret">{collapsed.has(tag) ? "▸" : "▾"}</span>
          <span class="group-name">{tag}</span>
          <span class="group-count">{reqs.length}</span>
        </button>
        {#if !collapsed.has(tag)}
          {#each reqs as r (r.id)}
            <div
              class="req-row"
              class:active={$selectedId === r.id}
              role="button"
              tabindex="0"
              on:click={() => selectedId.set(r.id)}
              on:keydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); selectedId.set(r.id); } }}
            >
              <span class={"badge m-" + r.method.toLowerCase()}>{r.method}</span>
              {#if envOf(r.tags)}<span class={"tagb env-" + envOf(r.tags)}>{envOf(r.tags)}</span>{/if}
              {#if r.tags.includes("admin")}<span class="tagb role-admin">admin</span>{/if}
              {#if r.tags.includes("user")}<span class="tagb role-user">user</span>{/if}
              <span class="name">{r.name}</span>
              <button class="del" on:click|stopPropagation={() => del(r.id)}>✕</button>
            </div>
          {/each}
        {/if}
      </div>
    {/each}
  </div>

  <div class="side-foot">
    <button class="icon-btn" title="Toggle light/dark" on:click={toggleTheme}>{isDark ? "☀️" : "🌙"}</button>
    <button class="config-btn" title={$configPath} on:click={pickConfig}>📄 {configName}</button>
  </div>
</div>
