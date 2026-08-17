use super::*;

impl std::fmt::Debug for ModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ModuleRef").field(&self.name()).finish()
    }
}

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.str(0)
    }
}
