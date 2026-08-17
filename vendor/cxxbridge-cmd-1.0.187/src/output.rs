use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum Output {
    Stdout,
    File(PathBuf),
}

impl Output {
    pub(crate) fn ends_with(&self, suffix: &str) -> bool {
        match self {
            Output::Stdout => false,
            Output::File(path) => path.to_string_lossy().ends_with(suffix),
        }
    }
}
