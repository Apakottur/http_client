<script lang="ts">
  import type { Request, HttpResponse } from "../types";
  import { sendRequest } from "../api";
  import { collection } from "../store";
  import KeyValueEditor from "./KeyValueEditor.svelte";
  import AuthTab from "./AuthTab.svelte";
  import BodyTab from "./BodyTab.svelte";
  import ResponseView from "./ResponseView.svelte";
  export let request: Request;

  // The editor mutates `request` in place (it's the same object held in the
  // collection store), so edits persist on save. But in-place mutation doesn't
  // notify store subscribers, so the sidebar's name/method badge would go stale.
  // Nudge the store after editing the fields the sidebar displays.
  function touch() { collection.update((c) => c); }

  const methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "HEAD"];
  let tab: "params" | "headers" | "auth" | "body" = "params";
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

<div class="editor">
  <div class="topbar">
    <select bind:value={request.method} on:change={touch}>
      {#each methods as m}<option value={m}>{m}</option>{/each}
    </select>
    <input class="url" placeholder="https://…" bind:value={request.url} />
    <button class="send" on:click={send} disabled={loading}>Send</button>
  </div>
  <input class="req-name" bind:value={request.name} on:input={touch} />

  <div class="tabs">
    <button class:active={tab === "params"} on:click={() => (tab = "params")}>Params</button>
    <button class:active={tab === "headers"} on:click={() => (tab = "headers")}>Headers</button>
    <button class:active={tab === "auth"} on:click={() => (tab = "auth")}>Auth</button>
    <button class:active={tab === "body"} on:click={() => (tab = "body")}>Body</button>
  </div>

  <div class="tab-body">
    {#if tab === "params"}<KeyValueEditor bind:rows={request.queryParams} placeholder="param" />
    {:else if tab === "headers"}<KeyValueEditor bind:rows={request.headers} placeholder="header" />
    {:else if tab === "auth"}<AuthTab bind:auth={request.auth} />
    {:else}<BodyTab bind:body={request.body} />{/if}
  </div>

  <div class="response"><ResponseView {response} {error} {loading} /></div>
</div>
