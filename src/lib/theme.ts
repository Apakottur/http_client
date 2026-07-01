import type { Theme } from "./types";

const mql = window.matchMedia("(prefers-color-scheme: dark)");

/** Resolve a theme setting to the concrete scheme to render. */
export function resolve(theme: Theme): "light" | "dark" {
  return theme === "system" ? (mql.matches ? "dark" : "light") : theme;
}

/** Apply the resolved theme to the document (drives the CSS `[data-theme]` rules). */
export function apply(theme: Theme): void {
  document.documentElement.dataset.theme = resolve(theme);
}

/** Re-apply when the OS scheme changes, but only while the setting is "system". */
export function watchSystem(getTheme: () => Theme): void {
  mql.addEventListener("change", () => {
    if (getTheme() === "system") apply("system");
  });
}
