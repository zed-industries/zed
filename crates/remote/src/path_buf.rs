use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum PathStyle {
    Posix,
    Windows,
}

#[derive(Debug, Clone)]
pub struct TargetPathBuf {
    inner: PathBuf,
    style: PathStyle,
}

impl TargetPathBuf {
    pub fn new(path: PathBuf, style: PathStyle) -> Self {
        Self { inner: path, style }
    }

    pub fn to_target(self) -> PathBuf {
        match self.style {
            PathStyle::Posix => self.inner.to_string_lossy().replace('\\', "/").into(),
            PathStyle::Windows => self.inner,
        }
    }
}
