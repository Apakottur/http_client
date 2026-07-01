<script lang="ts">
  import type { HttpResponse } from "../types";
  import JsonEditor from "./JsonEditor.svelte";
  export let response: HttpResponse | null;
  export let error: string | null;
  export let loading: boolean;
  let tab: "body" | "headers" = "body";

  function statusClass(s: number) {
    if (s >= 200 && s < 300) return "ok";
    if (s >= 300 && s < 400) return "redir";
    return "err";
  }
  function asJson(text: string): string | null {
    try { return JSON.stringify(JSON.parse(text), null, 2); } catch { return null; }
  }
  $: prettyJson = response ? asJson(response.body) : null;
</script>

{#if loading}
  <p>Sending…</p>
{:else if error}
  <pre class="error">{error}</pre>
{:else if response}
  <div class="resp-meta">
    <span class={"status " + statusClass(response.status)}>{response.status} {response.statusText}</span>
    <span>{response.timeMs} ms</span>
    <span>{response.size} B</span>
  </div>
  <div class="tabs">
    <button class:active={tab === "body"} on:click={() => (tab = "body")}>Body</button>
    <button class:active={tab === "headers"} on:click={() => (tab = "headers")}>Headers</button>
  </div>
  {#if tab === "body"}
    {#if prettyJson !== null}
      {#key prettyJson}<div class="resp-json"><JsonEditor value={prettyJson} readonly /></div>{/key}
    {:else}
      <pre class="resp-body">{response.body}</pre>
    {/if}
  {:else}
    <table class="kv">
      <tbody>
        {#each response.headers as h}
          <tr><td>{h.name}</td><td>{h.value}</td></tr>
        {/each}
      </tbody>
    </table>
  {/if}
{:else}
  <p class="hint">Send a request to see the response.</p>
{/if}
