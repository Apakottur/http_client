import type { Request } from "./types";

/** Environment tags are mutually exclusive; one of these (or none). */
export const ENVS = ["dev", "staging", "prod"] as const;
export type Env = (typeof ENVS)[number];

/** The environment currently set on a request, or "" if none. */
export function envOf(tags: string[]): Env | "" {
  return (ENVS.find((e) => tags.includes(e)) ?? "") as Env | "";
}

/** Distinct top-level tags across all requests, sorted, for the selector. */
export function topLevelTags(requests: Request[]): string[] {
  return [...new Set(requests.map((r) => r.topLevelTag).filter(Boolean))].sort((a, b) =>
    a.localeCompare(b),
  );
}
