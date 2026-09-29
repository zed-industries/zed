use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context as _, Result, anyhow, bail};
use base64::Engine as _;
use fs::Fs;
use futures::AsyncReadExt as _;
use gpui::{BackgroundExecutor, FutureExt as _};
use http_client::{AsyncBody, HttpClient, StatusCode};
use percent_encoding::percent_decode_str;

pub const MAX_ICON_SIZE_BYTES: usize = 64 * 1024;
pub const ICON_FETCH_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentIconSource {
    HttpsUrl(String),
    InlineBytes(Vec<u8>),
}

pub fn sanitize_icon_filename(agent_id: &str) -> String {
    let sanitized = agent_id
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' => character,
            _ => '-',
        })
        .collect::<String>();

    if sanitized.is_empty() {
        "unknown.svg".to_string()
    } else {
        format!("{sanitized}.svg")
    }
}

pub fn external_agents_icons_dir() -> PathBuf {
    paths::external_agents_dir().join("icons")
}

pub fn validate_svg(bytes: &[u8]) -> Result<()> {
    if bytes.len() > MAX_ICON_SIZE_BYTES {
        bail!(
            "SVG exceeds maximum size of {MAX_ICON_SIZE_BYTES} bytes (was {} bytes)",
            bytes.len()
        );
    }

    let text = std::str::from_utf8(bytes).context("SVG content is not valid UTF-8")?;
    let lower = text.trim().to_ascii_lowercase();

    if !lower.contains("<svg") {
        bail!("Content does not contain an <svg> element");
    }

    if !lower.contains("</svg>") && !lower.contains("/>") {
        bail!("Content does not appear to be a closed SVG document");
    }

    if lower.contains("<script") {
        bail!("SVG contains forbidden <script> element");
    }

    Ok(())
}

pub fn parse_agent_icon_meta(value: &serde_json::Value) -> Result<AgentIconSource> {
    if let Some(url_str) = value.get("url").and_then(|v| v.as_str()) {
        return parse_icon_string(url_str);
    }
    if let Some(data_str) = value.get("data").and_then(|v| v.as_str()) {
        return parse_icon_string(data_str);
    }
    if let Some(direct_str) = value.as_str() {
        return parse_icon_string(direct_str);
    }

    bail!("Missing or invalid icon specification in metadata (expected 'url', 'data', or string)")
}

fn parse_icon_string(input: &str) -> Result<AgentIconSource> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        bail!("Icon string is empty");
    }

    if let Some(b64) = trimmed.strip_prefix("data:image/svg+xml;base64,") {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(b64.trim())
            .context("Failed to decode base64 SVG data URI")?;
        validate_svg(&bytes)?;
        return Ok(AgentIconSource::InlineBytes(bytes));
    }

    if let Some(raw) = trimmed
        .strip_prefix("data:image/svg+xml;utf8,")
        .or_else(|| trimmed.strip_prefix("data:image/svg+xml,"))
    {
        let bytes = if raw.contains('%') {
            percent_decode_str(raw).collect::<Vec<u8>>()
        } else {
            raw.as_bytes().to_vec()
        };
        validate_svg(&bytes)?;
        return Ok(AgentIconSource::InlineBytes(bytes));
    }

    if trimmed.starts_with("<svg") || trimmed.starts_with("<?xml") {
        let bytes = trimmed.as_bytes().to_vec();
        validate_svg(&bytes)?;
        return Ok(AgentIconSource::InlineBytes(bytes));
    }

    if trimmed.starts_with("https://") {
        return Ok(AgentIconSource::HttpsUrl(trimmed.to_string()));
    }

    if trimmed.starts_with("http://") {
        bail!("Insecure HTTP icon URLs are not permitted; must use HTTPS");
    }

    bail!("Unsupported agent icon format or URI scheme (must be https:// or data:image/svg+xml;...)")
}

pub async fn fetch_url_body(
    http_client: Arc<dyn HttpClient>,
    url: &str,
    timeout: Duration,
    executor: &BackgroundExecutor,
) -> Result<(StatusCode, Vec<u8>)> {
    async {
        let mut response = http_client
            .get(url, AsyncBody::default(), true)
            .await
            .with_context(|| format!("requesting {url}"))?;

        let status = response.status();
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .with_context(|| format!("reading response from {url}"))?;

        Ok((status, body))
    }
    .with_timeout(timeout, executor)
    .await
    .map_err(|_| {
        anyhow!(
            "timed out after {}s while fetching {url}",
            timeout.as_secs()
        )
    })?
}

pub async fn fetch_and_validate_icon(
    http_client: Arc<dyn HttpClient>,
    url: &str,
    timeout: Duration,
    executor: &BackgroundExecutor,
) -> Result<Vec<u8>> {
    if !url.starts_with("https://") {
        bail!("Only HTTPS icon URLs are permitted");
    }

    let (status, body) = fetch_url_body(http_client, url, timeout, executor).await?;
    if !status.is_success() {
        bail!("Icon request failed with status {status}");
    }
    validate_svg(&body)?;
    Ok(body)
}

pub async fn resolve_and_cache_agent_icon(
    agent_id: &str,
    meta_value: &serde_json::Value,
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    executor: &BackgroundExecutor,
) -> Result<PathBuf> {
    let source = parse_agent_icon_meta(meta_value)?;
    let icons_dir = external_agents_icons_dir();
    let file_name = sanitize_icon_filename(agent_id);
    let target_path = icons_dir.join(file_name);

    let bytes = match source {
        AgentIconSource::HttpsUrl(url) => {
            fetch_and_validate_icon(http_client, &url, ICON_FETCH_TIMEOUT, executor).await?
        }
        AgentIconSource::InlineBytes(bytes) => {
            validate_svg(&bytes)?;
            bytes
        }
    };

    if !fs.is_dir(&icons_dir).await {
        fs.create_dir(&icons_dir).await?;
    }
    fs.write(&target_path, &bytes).await?;
    Ok(target_path)
}

pub async fn copy_and_cache_custom_icon(
    agent_id: &str,
    source_path: &std::path::Path,
    fs: Arc<dyn Fs>,
) -> Result<PathBuf> {
    let bytes = fs.load(source_path).await?;
    validate_svg(bytes.as_bytes())?;

    let icons_dir = external_agents_icons_dir();
    let file_name = sanitize_icon_filename(agent_id);
    let target_path = icons_dir.join(file_name);

    if source_path == target_path {
        return Ok(target_path);
    }

    if !fs.is_dir(&icons_dir).await {
        fs.create_dir(&icons_dir).await?;
    }
    fs.write(&target_path, bytes.as_bytes()).await?;
    Ok(target_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::FakeHttpClient;
    use serde_json::json;

    #[test]
    fn test_validate_svg() {
        assert!(validate_svg(b"<svg viewBox=\"0 0 16 16\"><circle cx=\"8\" cy=\"8\" r=\"8\"/></svg>").is_ok());
        assert!(validate_svg(b"<svg viewBox=\"0 0 16 16\"/>").is_ok());

        assert!(validate_svg(b"not an svg").is_err());
        assert!(validate_svg(b"<svg><script>alert(1)</script></svg>").is_err());

        let oversized = vec![b'a'; MAX_ICON_SIZE_BYTES + 1];
        assert!(validate_svg(&oversized).is_err());
    }

    #[test]
    fn test_parse_agent_icon_meta() {
        let https_url = json!({
            "url": "https://example.com/icon.svg"
        });
        assert_eq!(
            parse_agent_icon_meta(&https_url).unwrap(),
            AgentIconSource::HttpsUrl("https://example.com/icon.svg".to_string())
        );

        let http_url = json!({
            "url": "http://insecure.com/icon.svg"
        });
        assert!(parse_agent_icon_meta(&http_url).is_err());

        let inline_data = json!({
            "data": "<svg viewBox=\"0 0 16 16\"></svg>"
        });
        assert_eq!(
            parse_agent_icon_meta(&inline_data).unwrap(),
            AgentIconSource::InlineBytes(b"<svg viewBox=\"0 0 16 16\"></svg>".to_vec())
        );

        let b64_svg = base64::engine::general_purpose::STANDARD.encode(b"<svg></svg>");
        let data_uri = json!({
            "url": format!("data:image/svg+xml;base64,{b64_svg}")
        });
        assert_eq!(
            parse_agent_icon_meta(&data_uri).unwrap(),
            AgentIconSource::InlineBytes(b"<svg></svg>".to_vec())
        );

        let direct_string = json!("https://example.com/direct.svg");
        assert_eq!(
            parse_agent_icon_meta(&direct_string).unwrap(),
            AgentIconSource::HttpsUrl("https://example.com/direct.svg".to_string())
        );
    }

    #[gpui::test]
    async fn test_resolve_and_cache_agent_icon(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let fake_svg = b"<svg width=\"16\" height=\"16\"></svg>";

        let http_client = FakeHttpClient::create({
            let fake_svg = fake_svg.to_vec();
            move |_| {
                let fake_svg = fake_svg.clone();
                async move {
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body(http_client::AsyncBody::from(fake_svg))?)
                }
            }
        });

        let url_meta = json!({
            "url": "https://example.com/agent-icon.svg"
        });

        let cached_path = resolve_and_cache_agent_icon(
            "my-custom-agent",
            &url_meta,
            fs.clone(),
            http_client.clone(),
            &cx.executor(),
        )
        .await
        .unwrap();

        assert_eq!(cached_path, external_agents_icons_dir().join("my-custom-agent.svg"));
        assert_eq!(fs.load(&cached_path).await.unwrap().as_bytes(), fake_svg);

        let inline_meta = json!({
            "data": "<svg viewBox=\"0 0 16 16\"><path d=\"M0 0\"/></svg>"
        });

        let inline_path = resolve_and_cache_agent_icon(
            "inline-agent",
            &inline_meta,
            fs.clone(),
            http_client,
            &cx.executor(),
        )
        .await
        .unwrap();

        assert_eq!(inline_path, external_agents_icons_dir().join("inline-agent.svg"));
        assert_eq!(
            fs.load(&inline_path).await.unwrap(),
            "<svg viewBox=\"0 0 16 16\"><path d=\"M0 0\"/></svg>"
        );
    }

    #[gpui::test]
    async fn test_copy_and_cache_custom_icon(cx: &mut gpui::TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let source_path = PathBuf::from("/Users/alice/Downloads/custom.svg");
        let valid_svg = b"<svg width=\"16\" height=\"16\"></svg>";
        fs.create_dir(source_path.parent().unwrap()).await.unwrap();
        fs.write(&source_path, valid_svg).await.unwrap();

        let cached_path = copy_and_cache_custom_icon("my-agent", &source_path, fs.clone())
            .await
            .unwrap();

        assert_eq!(cached_path, external_agents_icons_dir().join("my-agent.svg"));
        assert_eq!(fs.load(&cached_path).await.unwrap().as_bytes(), valid_svg);

        let cached_again = copy_and_cache_custom_icon("my-agent", &cached_path, fs.clone())
            .await
            .unwrap();
        assert_eq!(cached_again, cached_path);
    }
}
