mod model;
mod insomnia;
mod storage;
mod http;

use model::{Collection, Request};
use http::HttpResponse;
use std::path::PathBuf;

// Anchor the collection files to the repo root regardless of the process's
// working directory (Tauri may launch the binary from src-tauri/). CARGO_MANIFEST_DIR
// is <repo>/src-tauri at compile time, so its parent is the repo root. This matches
// the v1 "run from the repo" design; a packaged binary would use the app-data dir.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR always has a parent")
        .to_path_buf()
}
fn collection_path() -> PathBuf { repo_root().join("collection.json") }
fn template_path() -> PathBuf { repo_root().join("collection.template.json") }

#[tauri::command]
fn load_collection() -> Result<Collection, String> {
    storage::load(&collection_path(), &template_path())
}

#[tauri::command]
fn save_collection(collection: Collection) -> Result<(), String> {
    storage::save(&collection_path(), &collection)
}

#[tauri::command]
async fn send_request(request: Request) -> Result<HttpResponse, String> {
    http::send(&request).await
}

#[tauri::command]
fn import_insomnia(path: String) -> Result<Collection, String> {
    let yaml = std::fs::read_to_string(&path).map_err(|e| format!("read {path}: {e}"))?;
    let coll = insomnia::import_from_str(&yaml)?;
    storage::save(&collection_path(), &coll)?;
    Ok(coll)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
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
        import_insomnia
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
