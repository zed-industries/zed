use std::collections::HashSet;

use syn::Ident;

/// A set of idents.
pub type IdentSet = HashSet<Ident>;

/// A set of references to idents.
pub type IdentRefSet<'a> = HashSet<&'a Ident>;
