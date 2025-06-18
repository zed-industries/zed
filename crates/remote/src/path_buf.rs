use std::path::PathBuf;

use rpc::proto::ToProto;

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
pub struct TargetPathBuf {
    inner: PathBuf,
    style: PathStyle,
}

impl TargetPathBuf {
    pub fn new(path: PathBuf, style: PathStyle) -> Self {
        Self { inner: path, style }
    }

    pub fn path_style(&self) -> PathStyle {
        self.style
    }

    pub fn to_target(self) -> PathBuf {
        match self.style {
            PathStyle::Posix => self.inner.to_string_lossy().replace('\\', "/").into(),
            PathStyle::Windows => self.inner,
        }
    }

    pub fn to_string(&self) -> String {
        match self.style {
            PathStyle::Posix => self.inner.to_string_lossy().replace('\\', "/"),
            PathStyle::Windows => self.inner.to_string_lossy().into(),
        }
    }

    pub fn parent(&self) -> Option<TargetPathBuf> {
        self.inner
            .parent()
            .map(|p| TargetPathBuf::new(p.to_path_buf(), self.style))
    }
}

impl ToProto for TargetPathBuf {
    fn to_proto(self) -> String {
        match self.style {
            PathStyle::Posix => self.to_string(),
            PathStyle::Windows => self.inner.to_proto(),
        }
    }
}
