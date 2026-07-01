<script lang="ts">
  import { get } from "svelte/store";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { collection, selectedId, configPath, drafts } from "../store";
  import { newRequest } from "../types";
  import type { Request } from "../types";
  import { setConfigPath, getConfigPath } from "../api";
  import { resolve as resolveTheme } from "../theme";
  import { ENVS, envOf, topLevelTags } from "../tags";
  import { persist, guardLeaveDraft, trySelect } from "../persist";

  let search = "";
  let mainFilter = "";
  let envFilter = "";
  let roleFilter = "";
  let collapsed = new Set<string>();

  // Row action menu (⋮)
  let menuId: string | null = null;
  let menuX = 0;
  let menuY = 0;

  $: isDark = resolveTheme($collection.settings.theme) === "dark";
  $: configName = $configPath ? $configPath.split("/").pop() : "(default)";
  $: topTags = topLevelTags($collection.requests);
  $: menuReq = menuId ? $collection.requests.find((r) => r.id === menuId) ?? null : null;

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

  // Create a new draft request (in memory, unsaved until first manual save).
  async function newDraft(req: Request) {
    if (!(await guardLeaveDraft())) return;
    collection.update((c) => ({ ...c, requests: [...c.requests, req] }));
    drafts.update((s) => new Set(s).add(req.id));
    selectedId.set(req.id);
  }
  function add() { newDraft(newRequest()); }
  function duplicate(orig: Request) {
    const copy: Request = { ...JSON.parse(JSON.stringify(orig)), id: crypto.randomUUID(), name: orig.name + " Copy" };
    newDraft(copy);
  }

  async function del(id: string) {
    const wasDraft = get(drafts).has(id);
    collection.update((c) => ({ ...c, requests: c.requests.filter((r) => r.id !== id) }));
    drafts.update((s) => { const n = new Set(s); n.delete(id); return n; });
    if (get(selectedId) === id) selectedId.set(null);
    if (!wasDraft) { try { await persist(); } catch (e) { console.error(e); } }
  }

  function openMenu(id: string, e: MouseEvent) {
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    menuX = rect.right;
    menuY = rect.bottom + 2;
    menuId = menuId === id ? null : id;
  }

  async function toggleTheme() {
    const next = isDark ? "light" : "dark";
    collection.update((c) => ({ ...c, settings: { ...c.settings, theme: next } }));
    try { await persist(); } catch (e) { console.error(e); }
  }

  async function switchTo(path: string) {
    try {
      collection.set(await setConfigPath(path));
      drafts.set(new Set());
      configPath.set(await getConfigPath());
      selectedId.set(null);
    } catch (e) { console.error(e); }
  }

  async function pickConfig() {
    if (!(await guardLeaveDraft())) return;
    const current = get(configPath);
    const selected = await open({
      multiple: false,
      defaultPath: current || undefined,
      filters: [{ name: "Config", extensions: ["json"] }],
    });
    if (typeof selected === "string") await switchTo(selected);
  }

  // Create a new project = a new config file. set_config_path seeds it from the
  // template when the chosen path doesn't exist yet.
  async function newProject() {
    if (!(await guardLeaveDraft())) return;
    const current = get(configPath);
    let path = await save({
      defaultPath: current || undefined,
      filters: [{ name: "Config", extensions: ["json"] }],
    });
    if (!path) return;
    if (!path.endsWith(".json")) path += ".json";
    await switchTo(path);
  }
</script>

<svelte:window on:click={() => (menuId = null)} />

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
              class:draft={$drafts.has(r.id)}
              role="button"
              tabindex="0"
              on:click={() => trySelect(r.id)}
              on:keydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); trySelect(r.id); } }}
            >
              <span class={"badge m-" + r.method.toLowerCase()}>{r.method}</span>
              {#if envOf(r.tags)}<span class={"tagb env-" + envOf(r.tags)}>{envOf(r.tags)}</span>{/if}
              {#if r.tags.includes("admin")}<span class="tagb role-admin">admin</span>{/if}
              {#if r.tags.includes("user")}<span class="tagb role-user">user</span>{/if}
              <span class="name">{r.name}</span>
              {#if $drafts.has(r.id)}<span class="draft-dot" title="Unsaved">●</span>{/if}
              <button class="kebab" title="Actions" on:click|stopPropagation={(e) => openMenu(r.id, e)}>⋮</button>
            </div>
          {/each}
        {/if}
      </div>
    {/each}
  </div>

  <div class="side-foot">
    <button class="icon-btn" title="Toggle light/dark" on:click={toggleTheme}>{isDark ? "☀️" : "🌙"}</button>
    <button class="config-btn" title={$configPath} on:click={pickConfig}>📄 {configName}</button>
    <button class="icon-btn" title="New project (new config file)" on:click={newProject}>＋</button>
  </div>
</div>

{#if menuReq}
  <div class="row-menu" style="left: {menuX}px; top: {menuY}px">
    <button class="menu-item" on:click={() => { duplicate(menuReq); menuId = null; }}>Duplicate</button>
    <button class="menu-item danger" on:click={() => { del(menuReq.id); menuId = null; }}>Delete</button>
  </div>
{/if}
