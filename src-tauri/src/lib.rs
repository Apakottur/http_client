mod model;
mod insomnia;
mod storage;
mod http;

use model::{Collection, Request};
use http::HttpResponse;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::State;

// Anchor the default config file to the repo root regardless of the process's working
// directory (Tauri may launch the binary from src-tauri/). CARGO_MANIFEST_DIR is
// <repo>/src-tauri at compile time, so its parent is the repo root.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR always has a parent")
        .to_path_buf()
}
fn default_config_path() -> PathBuf { repo_root().join("http_client_config.json") }
fn template_path() -> PathBuf { repo_root().join("http_client_config_template.json") }

// ~/.config/http_client (honours XDG_CONFIG_HOME) — holds only the pointer to the
// active config file, so switching config files survives restarts.
fn state_dir() -> Option<PathBuf> {
    if let Some(x) = std::env::var_os("XDG_CONFIG_HOME") {
        if !x.is_empty() {
            return Some(PathBuf::from(x).join("http_client"));
        }
    }
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config").join("http_client"))
}
fn state_file() -> Option<PathBuf> {
    state_dir().map(|d| d.join("state.json"))
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StateFile {
    config_path: Option<String>,
}

fn remembered_path() -> Option<PathBuf> {
    let f = state_file()?;
    let text = std::fs::read_to_string(&f).ok()?;
    let parsed: StateFile = serde_json::from_str(&text).ok()?;
    parsed.config_path.map(PathBuf::from)
}

fn remember_path(path: &Path) -> Result<(), String> {
    let dir = state_dir().ok_or("cannot determine config directory")?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("create {dir:?} failed: {e}"))?;
    let f = dir.join("state.json");
    let sf = StateFile { config_path: Some(path.to_string_lossy().into_owned()) };
    let text = serde_json::to_string_pretty(&sf).map_err(|e| format!("serialize state failed: {e}"))?;
    std::fs::write(&f, text).map_err(|e| format!("write {f:?} failed: {e}"))
}

/// Resolution order: remembered path (set via the in-app file selector, cached in
/// state.json) > repo-root default. Pick a config file once via the UI and it sticks.
fn resolve_config_path() -> PathBuf {
    remembered_path().unwrap_or_else(default_config_path)
}

struct AppState {
    config_path: Mutex<PathBuf>,
}

impl AppState {
    fn path(&self) -> PathBuf {
        self.config_path.lock().expect("config_path mutex poisoned").clone()
    }
}

#[tauri::command]
fn load_collection(state: State<'_, AppState>) -> Result<Collection, String> {
    storage::load(&state.path(), &template_path())
}

#[tauri::command]
fn save_collection(state: State<'_, AppState>, collection: Collection) -> Result<(), String> {
    storage::save(&state.path(), &collection)
}

#[tauri::command]
async fn send_request(request: Request) -> Result<HttpResponse, String> {
    http::send(&request).await
}

#[tauri::command]
fn import_insomnia(state: State<'_, AppState>, path: String) -> Result<Collection, String> {
    let yaml = std::fs::read_to_string(&path).map_err(|e| format!("read {path}: {e}"))?;
    let coll = insomnia::import_from_str(&yaml)?;
    storage::save(&state.path(), &coll)?;
    Ok(coll)
}

/// Absolute path of the config file currently in use (for display in the UI).
#[tauri::command]
fn get_config_path(state: State<'_, AppState>) -> String {
    state.path().to_string_lossy().into_owned()
}

/// Point the app at a different config file, remember it for next launch, and load it.
#[tauri::command]
fn set_config_path(state: State<'_, AppState>, path: String) -> Result<Collection, String> {
    let new_path = PathBuf::from(&path);
    let coll = storage::load(&new_path, &template_path())?;
    remember_path(&new_path)?;
    *state.config_path.lock().expect("config_path mutex poisoned") = new_path;
    Ok(coll)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .manage(AppState { config_path: Mutex::new(resolve_config_path()) })
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        load_collection,
        save_collection,
        send_request,
        import_insomnia,
        get_config_path,
        set_config_path
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
