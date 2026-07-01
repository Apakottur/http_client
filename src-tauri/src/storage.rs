use crate::model::Collection;
use std::fs;
use std::path::Path;

pub fn load(path: &Path, template: &Path) -> Result<Collection, String> {
    if !path.exists() {
        fs::copy(template, path).map_err(|e| format!("seed from template failed: {e}"))?;
    }
    let text = fs::read_to_string(path).map_err(|e| format!("read {path:?} failed: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("parse {path:?} failed: {e}"))
}

pub fn save(path: &Path, coll: &Collection) -> Result<(), String> {
    let text = serde_json::to_string_pretty(coll).map_err(|e| format!("serialize failed: {e}"))?;
    fs::write(path, text).map_err(|e| format!("write {path:?} failed: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn sample() -> Collection {
        Collection { version: 1, settings: Settings::default(), requests: vec![Request {
            id: "a".into(), name: "n".into(), method: "GET".into(), url: "http://h".into(),
            query_params: vec![], headers: vec![], auth: Auth::default(), body: Body::default(), sort_key: 1,
            top_level_tag: "Smitten".into(), tags: vec!["dev".into()],
        }]}
    }

    #[test]
    fn load_seeds_from_template_when_missing() {
        let dir = std::env::temp_dir().join(format!("scratch_test_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let tmpl = dir.join("template.json");
        let live = dir.join("collection.json");
        let _ = fs::remove_file(&live);
        save(&tmpl, &sample()).unwrap();

        let loaded = load(&live, &tmpl).unwrap();
        assert_eq!(loaded, sample());
        assert!(live.exists(), "live file should have been seeded");
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = std::env::temp_dir().join(format!("scratch_test_rt_{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let live = dir.join("c.json");
        let tmpl = dir.join("t.json");
        save(&tmpl, &sample()).unwrap();
        save(&live, &sample()).unwrap();
        assert_eq!(load(&live, &tmpl).unwrap(), sample());
    }
}
