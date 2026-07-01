<script lang="ts">
  import type { KeyValue } from "../types";
  export let rows: KeyValue[];
  export let placeholder = "name";
  function add() { rows = [...rows, { name: "", value: "", enabled: true }]; }
  function remove(i: number) { rows = rows.filter((_, idx) => idx !== i); }
</script>

<table class="kv">
  <tbody>
    {#each rows as row, i}
      <tr>
        <td><input type="checkbox" bind:checked={row.enabled} /></td>
        <td><input {placeholder} bind:value={row.name} /></td>
        <td><input placeholder="value" bind:value={row.value} /></td>
        <td><button on:click={() => remove(i)}>✕</button></td>
      </tr>
    {/each}
  </tbody>
</table>
<button class="add" on:click={add}>+ Add</button>
