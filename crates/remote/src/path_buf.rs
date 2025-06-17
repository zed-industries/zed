use std::path::PathBuf;

use rpc::proto::ToProto;

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

    pub fn to_string(&self) -> String {
        match self.style {
            PathStyle::Posix => self.inner.to_string_lossy().replace('\\', "/").into(),
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
