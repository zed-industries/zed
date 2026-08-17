/// Controls the signature of a setter method,
/// more specifically how `self` is passed and returned.
///
/// It can also be generalized to methods with different parameter sets and
/// return types, e.g. the `build()` method.
#[derive(PartialEq, Eq, Debug, Clone, Copy, FromMeta)]
pub enum BuilderPattern {
    /// E.g. `fn bar(self, bar: Bar) -> Self`.
    Owned,
    /// E.g. `fn bar(&mut self, bar: Bar) -> &mut Self`.
    Mutable,
    /// E.g. `fn bar(&self, bar: Bar) -> Self`.
    ///
    /// Note:
    /// - Needs to `clone` in order to return an _updated_ instance of `Self`.
    /// - There is a great chance that the Rust compiler (LLVM) will
    ///   optimize chained `clone` calls away in release mode.
    ///   Therefore this turns out not to be as bad as it sounds.
    Immutable,
}

impl BuilderPattern {
    /// Returns true if this style of builder needs to be able to clone its
    /// fields during the `build` method.
    pub fn requires_clone(&self) -> bool {
        *self != Self::Owned
    }
}

/// Defaults to `Mutable`.
impl Default for BuilderPattern {
    fn default() -> Self {
        Self::Mutable
    }
}

#[derive(Debug, Clone, FromMeta)]
pub struct Each {
    pub name: syn::Ident,
    #[darling(default)]
    pub into: bool,
}

impl From<syn::Ident> for Each {
    fn from(name: syn::Ident) -> Self {
        Self { name, into: false }
    }
}
