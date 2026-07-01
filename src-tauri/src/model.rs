use serde::{Deserialize, Serialize};

fn default_true() -> bool { true }
fn default_theme() -> String { "system".into() }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub version: u32,
    #[serde(default)]
    pub settings: Settings,
    pub requests: Vec<Request>,
}

/// App-level, non-request settings persisted alongside the requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// "system" | "light" | "dark"
    #[serde(default = "default_theme")]
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { theme: default_theme() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub query_params: Vec<KeyValue>,
    #[serde(default)]
    pub headers: Vec<KeyValue>,
    #[serde(default)]
    pub auth: Auth,
    #[serde(default)]
    pub body: Body,
    #[serde(default)]
    pub sort_key: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    pub name: String,
    pub value: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    #[serde(rename = "type")]
    pub auth_type: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub token: String,
}

impl Default for Auth {
    fn default() -> Self {
        Auth { auth_type: "none".into(), username: String::new(), password: String::new(), token: String::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    #[serde(rename = "type")]
    pub body_type: String,
    #[serde(default)]
    pub json: String,
    #[serde(default)]
    pub form: Vec<KeyValue>,
    #[serde(default)]
    pub multipart: Vec<MultipartField>,
}

impl Default for Body {
    fn default() -> Self {
        Body { body_type: "none".into(), json: String::new(), form: Vec::new(), multipart: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultipartField {
    pub name: String,
    pub value: String,
    pub kind: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_template_shape_and_roundtrips() {
        let json = r#"{
          "version": 1,
          "requests": [{
            "id": "x", "name": "n", "method": "POST", "url": "http://h/p",
            "queryParams": [{"name":"q","value":"1","enabled":true}],
            "headers": [{"name":"H","value":"v","enabled":true}],
            "auth": {"type":"bearer","username":"","password":"","token":"tok"},
            "body": {"type":"json","json":"{}","form":[],"multipart":[]},
            "sortKey": 42
          }]
        }"#;
        let coll: Collection = serde_json::from_str(json).unwrap();
        assert_eq!(coll.requests[0].sort_key, 42);
        assert_eq!(coll.requests[0].auth.token, "tok");
        // round-trips back to camelCase keys
        let out = serde_json::to_string(&coll).unwrap();
        assert!(out.contains("\"queryParams\""));
        assert!(out.contains("\"sortKey\""));
    }

    #[test]
    fn missing_optional_fields_default() {
        let json = r#"{"version":1,"requests":[{"id":"x","name":"n","method":"GET","url":"http://h"}]}"#;
        let coll: Collection = serde_json::from_str(json).unwrap();
        let r = &coll.requests[0];
        assert_eq!(r.auth.auth_type, "none");
        assert_eq!(r.body.body_type, "none");
        assert!(r.headers.is_empty());
    }
}
