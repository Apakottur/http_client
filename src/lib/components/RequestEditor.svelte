<script lang="ts">
  import type { Request, HttpResponse } from "../types";
  import { sendRequest } from "../api";
  import { collection, drafts } from "../store";
  import { saveSelected, scheduleAutosave } from "../persist";
  import { ENVS, envOf, topLevelTags } from "../tags";
  import KeyValueEditor from "./KeyValueEditor.svelte";
  import AuthTab from "./AuthTab.svelte";
  import BodyTab from "./BodyTab.svelte";
  import ResponseView from "./ResponseView.svelte";
  export let request: Request;

  // Drafts (new/duplicated, never saved) require a manual first save; saved
  // requests autosave on edit.
  $: isDraft = $drafts.has(request.id);

  // The editor mutates `request` in place (it's the same object held in the
  // collection store). Nudge the store so the sidebar reflects name/tag/method
  // changes live, and schedule an autosave (a no-op for drafts).
  function touch() { collection.update((c) => c); }
  function onEdit() { touch(); scheduleAutosave(); }

  const methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"];

  // Existing top-level tags to choose from, always including the current one.
  $: topTagOptions = [...new Set([...topLevelTags($collection.requests), request.topLevelTag].filter(Boolean))].sort((a, b) => a.localeCompare(b));
  $: env = envOf(request.tags);

  function setTopTag(e: Event) {
    const v = (e.currentTarget as HTMLSelectElement).value;
    if (v === "__new__") {
      const name = window.prompt("New top-level tag:")?.trim();
      if (name) request.topLevelTag = name;
    } else {
      request.topLevelTag = v;
    }
    touch();
  }
  function setEnv(e: Event) {
    const v = (e.currentTarget as HTMLSelectElement).value;
    request.tags = request.tags.filter((t) => !ENVS.includes(t as (typeof ENVS)[number]));
    if (v) request.tags = [...request.tags, v];
    touch();
  }
  function toggleTag(tag: string, on: boolean) {
    request.tags = on ? [...new Set([...request.tags, tag])] : request.tags.filter((t) => t !== tag);
    touch();
  }
  let tab: "params" | "headers" | "auth" | "body" = "body";
  let response: HttpResponse | null = null;
  let error: string | null = null;
  let loading = false;

  async function send() {
    loading = true; error = null; response = null;
    try { response = await sendRequest(request); }
    catch (e) { error = String(e); }
    finally { loading = false; }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="editor" on:input={onEdit} on:change={onEdit}>
  <div class="topbar">
    <select bind:value={request.method}>
      {#each methods as m}<option value={m}>{m}</option>{/each}
    </select>
    <input class="url" placeholder="https://…" bind:value={request.url} />
    <button class="send" on:click={send} disabled={loading}>Send</button>
    {#if isDraft}<button class="save-draft" on:click={saveSelected}>Save</button>{/if}
  </div>
  <div class="name-row">
    <input class="req-name" bind:value={request.name} />
    {#if isDraft}<span class="draft-tag">● unsaved — Save (or ⌘S) to keep</span>{/if}
  </div>

  <div class="tag-bar">
    <select
      class:missing={!request.topLevelTag}
      value={request.topLevelTag}
      on:change={setTopTag}
      title="Top-level tag (required)"
    >
      {#if !request.topLevelTag}<option value="" disabled selected>Tag…</option>{/if}
      {#each topTagOptions as t}<option value={t}>{t}</option>{/each}
      <option value="__new__">＋ New…</option>
    </select>

    <select value={env} on:change={setEnv} title="Environment">
      <option value="">env: any</option>
      {#each ENVS as e}<option value={e}>{e}</option>{/each}
    </select>

    <label class="chk"><input type="checkbox" checked={request.tags.includes("admin")} on:change={(e) => toggleTag("admin", e.currentTarget.checked)} /> admin</label>
    <label class="chk"><input type="checkbox" checked={request.tags.includes("user")} on:change={(e) => toggleTag("user", e.currentTarget.checked)} /> user</label>
  </div>

  <div class="tabs">
    <button class:active={tab === "body"} on:click={() => (tab = "body")}>Body</button>
    <button class:active={tab === "params"} on:click={() => (tab = "params")}>Params</button>
    <button class:active={tab === "auth"} on:click={() => (tab = "auth")}>Auth</button>
    <button class:active={tab === "headers"} on:click={() => (tab = "headers")}>Headers</button>
  </div>

  <div class="tab-body">
    {#if tab === "body"}<BodyTab bind:body={request.body} onChange={onEdit} />
    {:else if tab === "params"}<KeyValueEditor bind:rows={request.queryParams} placeholder="param" />
    {:else if tab === "auth"}<AuthTab bind:auth={request.auth} />
    {:else}<KeyValueEditor bind:rows={request.headers} placeholder="header" />{/if}
  </div>

  <div class="response"><ResponseView {response} {error} {loading} /></div>
</div>
