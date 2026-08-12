use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyncPath(Vec<String>);

impl SyncPath {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn from_segments<I, S>(segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self(segments.into_iter().map(Into::into).collect())
    }

    pub fn parse(pointer: &str) -> Option<Self> {
        let pointer = pointer.trim();
        if pointer.is_empty() || pointer == "/" {
            return None;
        }
        let pointer = pointer.strip_prefix('/').unwrap_or(pointer);
        let segments = pointer
            .split('/')
            .map(|segment| segment.replace("~1", "/").replace("~0", "~"))
            .collect::<Vec<_>>();
        if segments.iter().any(String::is_empty) {
            return None;
        }
        Some(Self(segments))
    }

    pub fn join(&self, segment: &str) -> Self {
        let mut segments = self.0.clone();
        segments.push(segment.to_owned());
        Self(segments)
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn segments(&self) -> &[String] {
        &self.0
    }

    pub fn first(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    pub fn starts_with(&self, prefix: &Self) -> bool {
        self.0.len() >= prefix.0.len() && self.0[..prefix.0.len()] == prefix.0[..]
    }
}

impl fmt::Display for SyncPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "/");
        }
        for segment in &self.0 {
            let escaped = segment.replace('~', "~0").replace('/', "~1");
            write!(f, "/{escaped}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_display() {
        assert_eq!(
            SyncPath::parse("/buffer_font_size"),
            Some(SyncPath::from_segments(["buffer_font_size"]))
        );
        assert_eq!(
            SyncPath::parse("/languages/Rust/tab_size"),
            Some(SyncPath::from_segments(["languages", "Rust", "tab_size"]))
        );
        assert_eq!(SyncPath::parse(""), None);
        assert_eq!(SyncPath::parse("/"), None);
        assert_eq!(SyncPath::parse("//x"), None);
        assert_eq!(
            SyncPath::parse("/file_types/C++/~0~1"),
            Some(SyncPath::from_segments(["file_types", "C++", "~/"]))
        );
        let path = SyncPath::from_segments(["a/b", "c~d"]);
        assert_eq!(path.to_string(), "/a~1b/c~0d");
        assert_eq!(SyncPath::parse(&path.to_string()), Some(path));
    }

    #[test]
    fn test_starts_with() {
        let prefix = SyncPath::from_segments(["macos"]);
        let path = SyncPath::from_segments(["macos", "buffer_font_size"]);
        assert_eq!(path.starts_with(&prefix), true);
        assert_eq!(prefix.starts_with(&path), false);
        assert_eq!(path.starts_with(&path), true);
        let other = SyncPath::from_segments(["macos_other"]);
        assert_eq!(other.starts_with(&prefix), false);
    }
}
