import { get } from "svelte/store";
import { confirm } from "@tauri-apps/plugin-dialog";
import { collection, selectedId, drafts } from "./store";
import { saveCollection } from "./api";

/** Write the collection to disk, EXCLUDING draft (unsaved) requests. */
export async function persist(): Promise<void> {
  const c = get(collection);
  const d = get(drafts);
  await saveCollection({ ...c, requests: c.requests.filter((r) => !d.has(r.id)) });
}

/** Promote the currently selected request out of draft (if it is one), then persist. */
export async function saveSelected(): Promise<void> {
  const id = get(selectedId);
  if (id && get(drafts).has(id)) {
    drafts.update((s) => {
      const n = new Set(s);
      n.delete(id);
      return n;
    });
  }
  await persist();
}

/** Debounced autosave — skips drafts (they require a manual first save). */
let timer: ReturnType<typeof setTimeout> | undefined;
export function scheduleAutosave(): void {
  const id = get(selectedId);
  if (id && get(drafts).has(id)) return;
  clearTimeout(timer);
  timer = setTimeout(() => { persist().catch((e) => console.error(e)); }, 400);
}

/** Remove a draft from memory (never written to disk). */
export function discardDraft(id: string): void {
  collection.update((c) => ({ ...c, requests: c.requests.filter((r) => r.id !== id) }));
  drafts.update((s) => {
    const n = new Set(s);
    n.delete(id);
    return n;
  });
}

/**
 * If the current selection is an unsaved draft we're about to leave, warn.
 * Returns true if it's OK to proceed (discarding the draft), false to stay.
 */
export async function guardLeaveDraft(nextId: string | null = null): Promise<boolean> {
  const cur = get(selectedId);
  const d = get(drafts);
  if (cur && d.has(cur) && cur !== nextId) {
    const r = get(collection).requests.find((x) => x.id === cur);
    const ok = await confirm(`"${r?.name || "New request"}" hasn't been saved yet. Discard it?`, {
      title: "Discard unsaved request?",
      kind: "warning",
    });
    if (!ok) return false;
    discardDraft(cur);
  }
  return true;
}

/** Select a request, guarding against leaving an unsaved draft. */
export async function trySelect(id: string | null): Promise<void> {
  if (await guardLeaveDraft(id)) selectedId.set(id);
}
