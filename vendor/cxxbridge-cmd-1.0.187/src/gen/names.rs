use crate::syntax::Pair;

impl Pair {
    pub(crate) fn to_fully_qualified(&self) -> String {
        let mut fully_qualified = String::new();
        for segment in &self.namespace {
            fully_qualified += "::";
            fully_qualified += &segment.to_string();
        }
        fully_qualified += "::";
        fully_qualified += &self.cxx.to_string();
        fully_qualified
    }
}
