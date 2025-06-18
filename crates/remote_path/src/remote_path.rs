use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum PathStyle {
    Posix,
    Windows,
}

impl PathStyle {
    #[cfg(target_os = "windows")]
    pub const fn current() -> Self {
        PathStyle::Windows
    }

    #[cfg(not(target_os = "windows"))]
    pub const fn current() -> Self {
        PathStyle::Posix
    }
}

#[derive(Debug, Clone)]
pub struct RemotePathBuf {
    inner: PathBuf,
    style: PathStyle,
    string: String, // Cached string representation
}

impl RemotePathBuf {
    pub fn new(path: PathBuf, style: PathStyle) -> Self {
        #[cfg(target_os = "windows")]
        let string = match style {
            PathStyle::Posix => path.to_string_lossy().replace('\\', "/"),
            PathStyle::Windows => path.to_string_lossy().into(),
        };
        #[cfg(not(target_os = "windows"))]
        let string = match style {
            PathStyle::Posix => path.to_string_lossy().to_string(),
            PathStyle::Windows => path.to_string_lossy().replace('/', "\\"),
        };
        Self {
            inner: path,
            style,
            string,
        }
    }

    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    pub fn path_style(&self) -> PathStyle {
        self.style
    }

    pub fn parent(&self) -> Option<RemotePathBuf> {
        self.inner
            .parent()
            .map(|p| RemotePathBuf::new(p.to_path_buf(), self.style))
    }
}

impl RemotePathBuf {
    pub fn to_string(&self) -> String {
        self.string.clone()
    }
}
