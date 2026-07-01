<script lang="ts">
  import type { Body, MultipartField } from "../types";
  import { open } from "@tauri-apps/plugin-dialog";
  import KeyValueEditor from "./KeyValueEditor.svelte";
  export let body: Body;

  function addMultipart(kind: "text" | "file") {
    const row: MultipartField = { name: "", value: "", kind, enabled: true };
    body.multipart = [...body.multipart, row];
  }
  async function pickFile(row: MultipartField) {
    const selected = await open({ multiple: false });
    if (typeof selected === "string") row.value = selected;
    body.multipart = body.multipart;
  }
</script>

<label>Body type
  <select bind:value={body.type}>
    <option value="none">None</option>
    <option value="json">JSON</option>
    <option value="form">Form URL-Encoded</option>
    <option value="multipart">Multipart</option>
  </select>
</label>

{#if body.type === "json"}
  <textarea class="json" bind:value={body.json} spellcheck="false"></textarea>
{:else if body.type === "form"}
  <KeyValueEditor bind:rows={body.form} />
{:else if body.type === "multipart"}
  {#each body.multipart as row, i}
    <div class="mp-row">
      <input type="checkbox" bind:checked={row.enabled} />
      <input placeholder="name" bind:value={row.name} />
      {#if row.kind === "file"}
        <button on:click={() => pickFile(row)}>{row.value || "Choose file…"}</button>
      {:else}
        <input placeholder="value" bind:value={row.value} />
      {/if}
      <button on:click={() => (body.multipart = body.multipart.filter((_, idx) => idx !== i))}>✕</button>
    </div>
  {/each}
  <button on:click={() => addMultipart("text")}>+ Text</button>
  <button on:click={() => addMultipart("file")}>+ File</button>
{/if}
