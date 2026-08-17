use super::*;

impl std::fmt::Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeRef({})", self.type_name())
    }
}

impl TypeRef {
    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), self.name())
    }

    pub fn name(&self) -> &'static str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &'static str {
        self.str(2)
    }
}
