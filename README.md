# HTTP Client

A lightweight [Tauri](https://tauri.app) desktop HTTP client — a small, fast
replacement for Insomnia. Send requests; manage method, URL, query params,
headers, auth, and body; read the response. Saved requests live in a sidebar.

- **Backend:** Rust — `reqwest` (HTTP), `serde` (model), `serde_yaml` (Insomnia import).
- **Frontend:** Svelte 5 + Vite + TypeScript, in a native Tauri window.

## Prerequisites

You need **Rust**, **Bun**, and the Tauri Linux system libraries.

On Arch / CachyOS:

```sh
sudo pacman -S --needed rustup base-devel webkit2gtk-4.1 curl wget file openssl librsvg
rustup default stable
```

Install Bun (if you don't have it): https://bun.sh — `curl -fsSL https://bun.sh/install | bash`

## Run (development)

```sh
bun install          # first time only — installs frontend deps
bun run tauri dev    # compiles the Rust backend, starts Vite, opens the window
```

The first `tauri dev` compiles the whole Rust/Tauri toolchain and takes a couple
of minutes; subsequent runs are fast. The window opens with your saved requests
in the sidebar.

**Using it:** pick a request (or click **New**), set the method / URL / headers /
auth / body, and hit **Send**. The response (status, time, size, pretty-printed
body, headers) shows below. **Ctrl/⌘ + S** saves edits to your collection.

## Build a standalone binary (optional)

```sh
bun run tauri build
```

Produces a native executable under `src-tauri/target/release/` plus installers
(`.deb` / `.rpm` / `.AppImage`) under `src-tauri/target/release/bundle/`.

## Config file (settings + requests)

Everything — app settings (like the theme) and all your requests — lives in one
JSON file:

```json
{
  "version": 2,
  "settings": { "theme": "system" },
  "requests": [ /* … */ ]
}
```

- **`http_client_config.json`** — your live config, including any real
  tokens/passwords. **Gitignored — never committed.**
- **`http_client_config_template.json`** — a committed example. If no config file
  exists, it's seeded from this template.

**Which file the app uses**, in priority order:

1. **`--config <path>`** on the command line, e.g.
   `bun run tauri dev -- --config ~/work/api.json` (or `./http-client --config …`
   for a built binary). Highest priority.
2. **The last file you picked** via the config selector in the sidebar. This choice
   is remembered in `~/.config/http_client/state.json` (honours `XDG_CONFIG_HOME`)
   so it survives restarts.
3. **Default:** `http_client_config.json` at the repo root.

Switch files at runtime with the 📄 button in the sidebar header.

## Theme

A light/dark toggle (🌙/☀️) lives in the sidebar header. It defaults to your OS
setting (`system`); once you toggle, the explicit choice is saved to
`settings.theme` in your config file.

## Tests

```sh
cd src-tauri && cargo test
```

Covers the Insomnia → model conversion and the request-building helpers.

## Importing from Insomnia

The importer (`src-tauri/src/insomnia.rs`) maps an Insomnia v5 YAML export —
URL, method, headers, query params, basic/bearer auth, and JSON /
form-urlencoded / multipart bodies — into the model in `src-tauri/src/model.rs`.
Folder/group entries and the `environments` / `cookieJar` sections are ignored.

## Not included in v1

Environment / `{{variable}}` templating, folders, and cloud sync — deliberately
left out to keep it simple.
