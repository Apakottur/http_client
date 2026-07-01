import { writable, derived } from "svelte/store";
import type { Collection, Request } from "./types";

export const collection = writable<Collection>({ version: 2, settings: { theme: "system" }, requests: [] });
export const selectedId = writable<string | null>(null);
// Absolute path of the active config file, for display in the sidebar.
export const configPath = writable<string>("");

export const selectedRequest = derived(
  [collection, selectedId],
  ([$c, $id]) => $c.requests.find((r) => r.id === $id) ?? null
);
