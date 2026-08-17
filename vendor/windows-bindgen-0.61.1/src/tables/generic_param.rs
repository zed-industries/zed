use super::*;

impl std::fmt::Debug for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericParam").field(&self.name()).finish()
    }
}

impl GenericParam {
    pub fn name(&self) -> &'static str {
        self.str(3)
    }
}
