use crate::model::{KeyValue, Request};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub time_ms: u128,
    pub size: u64,
    pub headers: Vec<KeyValue>,
    pub body: String,
}

pub fn effective_url(url: &str, params: &[KeyValue]) -> String {
    let active: Vec<&KeyValue> = params.iter().filter(|p| p.enabled && !p.name.is_empty()).collect();
    if active.is_empty() {
        return url.to_string();
    }
    let qs = active.iter()
        .map(|p| format!("{}={}", urlencoding::encode(&p.name), urlencoding::encode(&p.value)))
        .collect::<Vec<_>>()
        .join("&");
    let sep = if url.contains('?') { "&" } else { "?" };
    format!("{url}{sep}{qs}")
}

pub fn form_body(fields: &[KeyValue]) -> String {
    fields.iter()
        .filter(|f| f.enabled && !f.name.is_empty())
        .map(|f| format!("{}={}", urlencoding::encode(&f.name), urlencoding::encode(&f.value)))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn basic_auth_header(user: &str, pass: &str) -> String {
    use base64::{Engine, engine::general_purpose::STANDARD};
    format!("Basic {}", STANDARD.encode(format!("{user}:{pass}")))
}

pub async fn send(req: &Request) -> Result<HttpResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("client build failed: {e}"))?;
    let method = reqwest::Method::from_bytes(req.method.as_bytes())
        .map_err(|e| format!("bad method: {e}"))?;
    let url = effective_url(&req.url, &req.query_params);
    let mut rb = client.request(method, &url);

    for h in req.headers.iter().filter(|h| h.enabled && !h.name.is_empty()) {
        rb = rb.header(&h.name, &h.value);
    }
    match req.auth.auth_type.as_str() {
        "bearer" => rb = rb.bearer_auth(&req.auth.token),
        "basic" => rb = rb.header("Authorization", basic_auth_header(&req.auth.username, &req.auth.password)),
        _ => {}
    }
    match req.body.body_type.as_str() {
        "json" => rb = rb.header("Content-Type", "application/json").body(req.body.json.clone()),
        "form" => rb = rb
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body(&req.body.form)),
        "multipart" => {
            let mut form = reqwest::multipart::Form::new();
            for f in req.body.multipart.iter().filter(|m| m.enabled && !m.name.is_empty()) {
                if f.kind == "file" {
                    let bytes = std::fs::read(&f.value)
                        .map_err(|e| format!("read file {}: {e}", f.value))?;
                    let filename = std::path::Path::new(&f.value)
                        .file_name().and_then(|n| n.to_str()).unwrap_or("file").to_string();
                    form = form.part(f.name.clone(), reqwest::multipart::Part::bytes(bytes).file_name(filename));
                } else {
                    form = form.text(f.name.clone(), f.value.clone());
                }
            }
            rb = rb.multipart(form);
        }
        _ => {}
    }

    let start = Instant::now();
    let resp = rb.send().await.map_err(|e| format!("request failed: {e}"))?;
    let status = resp.status();
    let headers: Vec<KeyValue> = resp.headers().iter()
        .map(|(k, v)| KeyValue { name: k.to_string(), value: v.to_str().unwrap_or("").to_string(), enabled: true })
        .collect();
    let body = resp.text().await.map_err(|e| format!("read body failed: {e}"))?;
    let time_ms = start.elapsed().as_millis();

    Ok(HttpResponse {
        status: status.as_u16(),
        status_text: status.canonical_reason().unwrap_or("").to_string(),
        time_ms,
        size: body.len() as u64,
        headers,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kv(n: &str, v: &str, enabled: bool) -> KeyValue {
        KeyValue { name: n.into(), value: v.into(), enabled }
    }

    #[test]
    fn effective_url_appends_enabled_params_and_encodes() {
        let url = effective_url("http://h/p", &[kv("a", "b c", true), kv("skip", "x", false), kv("", "y", true)]);
        assert_eq!(url, "http://h/p?a=b%20c");
    }

    #[test]
    fn effective_url_uses_ampersand_when_query_exists() {
        let url = effective_url("http://h/p?z=1", &[kv("a", "b", true)]);
        assert_eq!(url, "http://h/p?z=1&a=b");
    }

    #[test]
    fn form_body_encodes_enabled_fields() {
        assert_eq!(form_body(&[kv("u", "a&b", true), kv("p", "x", true)]), "u=a%26b&p=x");
    }

    #[test]
    fn basic_auth_header_is_base64() {
        assert_eq!(basic_auth_header("admin", "pw"), "Basic YWRtaW46cHc=");
    }
}
