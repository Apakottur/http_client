use crate::model::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Export {
    #[serde(default)]
    collection: Vec<serde_yaml::Value>,
}

#[derive(Deserialize)]
struct RawParam {
    #[serde(default)] name: String,
    #[serde(default)] value: String,
    #[serde(default, rename = "type")] kind: Option<String>,
    #[serde(default)] disabled: bool,
}

pub fn import_from_str(yaml: &str) -> Result<Collection, String> {
    let export: Export = serde_yaml::from_str(yaml).map_err(|e| format!("YAML parse error: {e}"))?;
    let mut requests = Vec::new();
    for item in export.collection {
        // skip folder/group entries that have no method+url
        let method = item.get("method").and_then(|v| v.as_str());
        let url = item.get("url").and_then(|v| v.as_str());
        let (method, url) = match (method, url) {
            (Some(m), Some(u)) => (m.to_string(), u.to_string()),
            _ => continue,
        };
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let sort_key = item
            .get("meta").and_then(|m| m.get("sortKey")).and_then(|v| v.as_i64())
            .unwrap_or(0);

        let headers = kvs(item.get("headers"));
        let query_params = kvs(item.get("parameters"));
        let auth = parse_auth(item.get("authentication"));
        let body = parse_body(item.get("body"));

        requests.push(Request {
            id: uuid::Uuid::new_v4().to_string(),
            name, method, url, query_params, headers, auth, body, sort_key,
        });
    }
    Ok(Collection { version: 1, settings: Settings::default(), requests })
}

fn kvs(v: Option<&serde_yaml::Value>) -> Vec<KeyValue> {
    let Some(seq) = v.and_then(|v| v.as_sequence()) else { return Vec::new() };
    seq.iter().filter_map(|e| {
        let p: RawParam = serde_yaml::from_value(e.clone()).ok()?;
        Some(KeyValue { name: p.name, value: p.value, enabled: !p.disabled })
    }).collect()
}

fn parse_auth(v: Option<&serde_yaml::Value>) -> Auth {
    let Some(a) = v else { return Auth::default() };
    match a.get("type").and_then(|t| t.as_str()) {
        Some("bearer") => Auth {
            auth_type: "bearer".into(),
            token: a.get("token").and_then(|t| t.as_str()).unwrap_or("").to_string(),
            ..Auth::default()
        },
        Some("basic") => Auth {
            auth_type: "basic".into(),
            username: a.get("username").and_then(|t| t.as_str()).unwrap_or("").to_string(),
            password: a.get("password").and_then(|t| t.as_str()).unwrap_or("").to_string(),
            ..Auth::default()
        },
        _ => Auth::default(),
    }
}

fn parse_body(v: Option<&serde_yaml::Value>) -> Body {
    let Some(b) = v else { return Body::default() };
    match b.get("mimeType").and_then(|m| m.as_str()) {
        Some("application/json") => Body {
            body_type: "json".into(),
            json: b.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string(),
            ..Body::default()
        },
        Some("application/x-www-form-urlencoded") => Body {
            body_type: "form".into(),
            form: kvs(b.get("params")),
            ..Body::default()
        },
        Some("multipart/form-data") => {
            let seq = b.get("params").and_then(|p| p.as_sequence()).cloned().unwrap_or_default();
            let multipart = seq.iter().filter_map(|e| {
                let p: RawParam = serde_yaml::from_value(e.clone()).ok()?;
                let kind = if p.kind.as_deref() == Some("file") { "file" } else { "text" };
                Some(MultipartField { name: p.name, value: p.value, kind: kind.into(), enabled: !p.disabled })
            }).collect();
            Body { body_type: "multipart".into(), multipart, ..Body::default() }
        }
        _ => Body::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Collection {
        let yaml = include_str!("../tests/fixtures/insomnia_sample.yaml");
        import_from_str(yaml).unwrap()
    }

    #[test]
    fn skips_folder_groups_without_method_or_url() {
        assert_eq!(fixture().requests.len(), 4);
    }

    #[test]
    fn maps_json_body_and_bearer_auth() {
        let r = &fixture().requests[0];
        assert_eq!(r.method, "POST");
        assert_eq!(r.body.body_type, "json");
        assert_eq!(r.body.json, "{\"a\":1}");
        assert_eq!(r.auth.auth_type, "bearer");
        assert_eq!(r.auth.token, "FAKE_TOKEN");
        assert_eq!(r.headers[0].name, "x-api-key");
        assert_eq!(r.sort_key, 3800);
    }

    #[test]
    fn maps_form_body_and_basic_auth() {
        let r = &fixture().requests[1];
        assert_eq!(r.body.body_type, "form");
        assert_eq!(r.body.form.len(), 2);
        assert_eq!(r.auth.auth_type, "basic");
        assert_eq!(r.auth.username, "admin");
    }

    #[test]
    fn maps_multipart_files_query_params_and_disabled() {
        let r = &fixture().requests[2];
        assert_eq!(r.query_params[0].name, "q");
        assert_eq!(r.body.body_type, "multipart");
        assert_eq!(r.body.multipart.len(), 3);
        let file = r.body.multipart.iter().find(|m| m.kind == "file").unwrap();
        assert_eq!(file.value, "/tmp/a.csv");
        let disabled = r.body.multipart.iter().find(|m| m.name == "skipme").unwrap();
        assert!(!disabled.enabled);
    }

    #[test]
    fn no_body_request_defaults_to_none() {
        let r = &fixture().requests[3];
        assert_eq!(r.body.body_type, "none");
        assert_eq!(r.method, "GET");
    }
}
