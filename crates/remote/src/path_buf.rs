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
pub struct RemotePathBuf {
    inner: PathBuf,
    style: PathStyle,
}

impl RemotePathBuf {
    pub fn new(path: PathBuf, style: PathStyle) -> Self {
        Self { inner: path, style }
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

#[cfg(target_os = "windows")]
impl RemotePathBuf {
    pub fn to_string(&self) -> String {
        match self.style {
            PathStyle::Posix => self.inner.to_string_lossy().replace('\\', "/"),
            PathStyle::Windows => self.inner.to_string_lossy().into(),
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl RemotePathBuf {
    pub fn to_string(&self) -> String {
        match self.style {
            PathStyle::Posix => self.inner.to_string_lossy().to_string(),
            PathStyle::Windows => self.inner.to_string_lossy().replace('/', "\\"),
        }
    }
}

impl ToProto for RemotePathBuf {
    #[cfg(target_os = "windows")]
    fn to_proto(self) -> String {
        match self.style {
            PathStyle::Posix => self.to_string(),
            PathStyle::Windows => self.inner.to_proto(),
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn to_proto(self) -> String {
        match self.style {
            PathStyle::Posix => self.inner.to_proto(),
            PathStyle::Windows => self.to_string(),
        }
    }
}
