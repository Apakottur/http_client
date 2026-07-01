import { writable, derived } from "svelte/store";
import type { Collection, Request } from "./types";

export const collection = writable<Collection>({ version: 2, settings: { theme: "system", sidebarWidth: 320 }, requests: [] });
export const selectedId = writable<string | null>(null);
// Absolute path of the active config file, for display in the sidebar.
export const configPath = writable<string>("");
// IDs of requests created (new/duplicate) but not yet saved to disk. These are
// kept in memory only and excluded from persistence until the first manual save.
export const drafts = writable<Set<string>>(new Set());

export const selectedRequest = derived(
  [collection, selectedId],
  ([$c, $id]) => $c.requests.find((r) => r.id === $id) ?? null
);
