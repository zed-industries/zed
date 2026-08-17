#[derive(Debug, Clone)]
pub(crate) struct DirOptionsExt {
    pub(super) mode: u32,
}

impl DirOptionsExt {
    pub(crate) const fn new() -> Self {
        Self {
            // The default value; see
            // <https://doc.rust-lang.org/std/os/unix/fs/trait.DirBuilderExt.html#tymethod.mode>
            mode: 0o777,
        }
    }
}

impl crate::fs::DirBuilderExt for DirOptionsExt {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.mode = mode;
        self
    }
}
