use super::*;

impl std::fmt::Debug for TypeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TypeSpec").field(&self.0).finish()
    }
}
