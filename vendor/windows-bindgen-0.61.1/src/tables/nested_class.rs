use super::*;

impl std::fmt::Debug for NestedClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedClass")
            .field("inner", &self.inner())
            .field("outer", &self.outer())
            .finish()
    }
}

impl NestedClass {
    pub fn inner(&self) -> TypeDef {
        TypeDef(self.row(0))
    }

    pub fn outer(&self) -> TypeDef {
        TypeDef(self.row(1))
    }
}
