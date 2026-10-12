use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result, bail};
use fs::Fs;

pub const MAX_ICON_SIZE_BYTES: usize = 64 * 1024;

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
    use std::path::PathBuf;

    #[test]
    fn test_validate_svg() {
        assert!(validate_svg(b"<svg viewBox=\"0 0 16 16\"><circle cx=\"8\" cy=\"8\" r=\"8\"/></svg>").is_ok());
        assert!(validate_svg(b"<svg viewBox=\"0 0 16 16\"/>").is_ok());

        assert!(validate_svg(b"not an svg").is_err());
        assert!(validate_svg(b"<svg><script>alert(1)</script></svg>").is_err());

        let oversized = vec![b'a'; MAX_ICON_SIZE_BYTES + 1];
        assert!(validate_svg(&oversized).is_err());
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
