use super::*;

impl std::fmt::Debug for InterfaceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InterfaceImpl").field(&self.0).finish()
    }
}

impl InterfaceImpl {
    pub fn ty(&self, generics: &[Type]) -> Type {
        Type::from_ref(self.decode(1), None, generics)
    }
}
