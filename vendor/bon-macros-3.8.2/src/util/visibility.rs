use crate::util::prelude::*;

pub(crate) trait VisibilityExt {
    /// Returns [`syn::Visibility`] that is equivalent to the current visibility
    /// but for an item that is inside of the child module. This basically does
    /// the following conversions.
    ///
    /// - `pub` -> `pub` (unchanged)
    /// - `pub(crate)` -> `pub(crate)` (unchanged)
    /// - `pub(self)` or ` ` (default private visibility) -> `pub(super)`
    /// - `pub(super)` -> `pub(in super::super)`
    /// - `pub(in relative::path)` -> `pub(in super::relative::path)`
    /// - `pub(in ::absolute::path)` -> `pub(in ::absolute::path)` (unchanged)
    /// - `pub(in crate::path)` -> `pub(in crate::path)` (unchanged)
    /// - `pub(in self::path)` -> `pub(in super::path)`
    /// - `pub(in super::path)` -> `pub(in super::super::path)`
    ///
    /// Note that absolute paths in `pub(in ...)` are not supported with Rust 2018+,
    /// according to the [Rust reference]:
    ///
    /// > Edition Differences: Starting with the 2018 edition, paths for pub(in path)
    /// > must start with crate, self, or super. The 2015 edition may also use paths
    /// > starting with :: or modules from the crate root.
    ///
    /// # Errors
    ///
    /// This function may return an error if it encounters some unexpected syntax.
    /// For example, some syntax that isn't known to the latest version of Rust
    /// this code was written for.
    ///
    /// [Rust reference]: https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself
    fn into_equivalent_in_child_module(self) -> Result<syn::Visibility>;
}

impl VisibilityExt for syn::Visibility {
    fn into_equivalent_in_child_module(mut self) -> Result<syn::Visibility> {
        match &mut self {
            Self::Public(_) => Ok(self),
            Self::Inherited => Ok(syn::parse_quote!(pub(super))),
            Self::Restricted(syn::VisRestricted {
                path,
                in_token: None,
                ..
            }) => {
                if path.is_ident("crate") {
                    return Ok(self);
                }

                if path.is_ident("super") {
                    return Ok(syn::parse_quote!(pub(in super::#path)));
                }

                if path.is_ident("self") {
                    return Ok(syn::parse_quote!(pub(super)));
                }

                bail!(
                    &self,
                    "Expected either `crate` or `super` or `in some::path` inside of \
                    `pub(...)` but got something else. This may be because a new \
                    syntax for visibility was released in a newer Rust version, \
                    but this crate doesn't support it."
                );
            }
            Self::Restricted(syn::VisRestricted {
                path,
                in_token: Some(_),
                ..
            }) => {
                if path.leading_colon.is_some() {
                    return Ok(self);
                }

                if path.starts_with_segment("crate") {
                    return Ok(self);
                }

                if let Some(first_segment) = path.segments.first_mut() {
                    if first_segment.ident == "self" {
                        let span = first_segment.ident.span();
                        *first_segment = syn::parse_quote_spanned!(span=>super);
                        return Ok(self);
                    }
                }

                path.segments.insert(0, syn::parse_quote!(super));

                Ok(self)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote as pq;

    #[test]
    fn all_tests() {
        #[track_caller]
        // One less `&` character to type in assertions
        #[allow(clippy::needless_pass_by_value)]
        fn test(vis: syn::Visibility, expected: syn::Visibility) {
            let actual = vis.into_equivalent_in_child_module().unwrap();
            assert!(
                actual == expected,
                "got:\nactual: {}\nexpected: {}",
                actual.to_token_stream(),
                expected.to_token_stream()
            );
        }

        test(pq!(pub), pq!(pub));
        test(pq!(pub(crate)), pq!(pub(crate)));
        test(pq!(pub(self)), pq!(pub(super)));
        test(pq!(), pq!(pub(super)));
        test(pq!(pub(super)), pq!(pub(in super::super)));

        test(
            pq!(pub(in relative::path)),
            pq!(pub(in super::relative::path)),
        );
        test(pq!(pub(in crate::path)), pq!(pub(in crate::path)));
        test(pq!(pub(in self::path)), pq!(pub(in super::path)));
        test(pq!(pub(in super::path)), pq!(pub(in super::super::path)));

        test(pq!(pub(in ::absolute::path)), pq!(pub(in ::absolute::path)));
    }
}
