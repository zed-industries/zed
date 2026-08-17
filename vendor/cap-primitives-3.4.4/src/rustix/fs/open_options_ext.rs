#[derive(Debug, Clone)]
pub(crate) struct ImplOpenOptionsExt {
    pub(crate) mode: u32,
    pub(crate) custom_flags: i32,
}

impl ImplOpenOptionsExt {
    pub(crate) const fn new() -> Self {
        Self {
            mode: 0o666,
            custom_flags: 0,
        }
    }

    pub(crate) fn mode(&mut self, mode: u32) -> &mut Self {
        self.mode = mode;
        self
    }

    pub(crate) fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.custom_flags = flags;
        self
    }
}
