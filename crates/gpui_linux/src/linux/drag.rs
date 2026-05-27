use anyhow::{Result, anyhow};
use gpui::ExternalPaths;
use url::Url;

pub(crate) const FILE_LIST_MIME_TYPE: &str = "text/uri-list";

pub(crate) fn encode_paths_as_uri_list(paths: &ExternalPaths) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for path in paths.paths() {
        let url = Url::from_file_path(path)
            .map_err(|_| anyhow!("failed to encode file path as URL: {path:?}"))?;
        bytes.extend_from_slice(url.as_str().as_bytes());
        bytes.extend_from_slice(b"\r\n");
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn encode_paths_as_uri_list_preserves_spaces() {
        let paths = ExternalPaths(vec![PathBuf::from("/tmp/with space.txt")].into());
        let encoded = encode_paths_as_uri_list(&paths).unwrap();
        assert_eq!(
            String::from_utf8(encoded).unwrap(),
            "file:///tmp/with%20space.txt\r\n"
        );
    }

    #[test]
    fn encode_paths_as_uri_list_ends_with_crlf() {
        let paths = ExternalPaths(vec![PathBuf::from("/tmp/example.txt")].into());
        let encoded = encode_paths_as_uri_list(&paths).unwrap();
        assert!(encoded.ends_with(b"\r\n"));
    }
}
