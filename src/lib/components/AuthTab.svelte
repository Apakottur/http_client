<script lang="ts">
  import type { Auth } from "../types";
  export let auth: Auth;
</script>

<label>Type
  <select bind:value={auth.type}>
    <option value="none">None</option>
    <option value="basic">Basic</option>
    <option value="bearer">Bearer</option>
  </select>
</label>

{#if auth.type === "basic"}
  <label>Username <input bind:value={auth.username} /></label>
  <label>Password <input type="password" bind:value={auth.password} /></label>
{:else if auth.type === "bearer"}
  <label>Token <input bind:value={auth.token} /></label>
{/if}
