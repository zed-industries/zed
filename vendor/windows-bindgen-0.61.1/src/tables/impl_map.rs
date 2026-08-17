use super::*;

impl std::fmt::Debug for ImplMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplMap").field(&self.import_name()).finish()
    }
}

impl ImplMap {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.usize(0))
    }

    pub fn scope(&self) -> ModuleRef {
        ModuleRef(self.row(3))
    }

    pub fn import_name(&self) -> &'static str {
        self.str(2)
    }
}
