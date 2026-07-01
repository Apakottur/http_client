import { writable, derived } from "svelte/store";
import type { Collection, Request } from "./types";

export const collection = writable<Collection>({ version: 1, requests: [] });
export const selectedId = writable<string | null>(null);

export const selectedRequest = derived(
  [collection, selectedId],
  ([$c, $id]) => $c.requests.find((r) => r.id === $id) ?? null
);
