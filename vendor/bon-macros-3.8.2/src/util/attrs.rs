use syn::spanned::Spanned;

pub(crate) trait AttributeExt {
    fn is_doc_expr(&self) -> bool;
    fn as_doc_expr(&self) -> Option<&syn::Expr>;
    fn to_allow(&self) -> Option<syn::Attribute>;
}

impl AttributeExt for syn::Attribute {
    fn is_doc_expr(&self) -> bool {
        self.as_doc_expr().is_some()
    }

    fn as_doc_expr(&self) -> Option<&syn::Expr> {
        let attr = match &self.meta {
            syn::Meta::NameValue(attr) => attr,
            _ => return None,
        };

        if !attr.path.is_ident("doc") {
            return None;
        }

        Some(&attr.value)
    }

    /// Returns `Some` if this is an `#[allow(...)]` or `#[expect(...)]` attribute.
    /// Turns an `#[expect(...)]` into `#[allow(...)]`, which is useful to make sure
    /// that macro doesn't trigger another warning that there is actually no
    /// instance of a lint warning under the `#[expect(...)]`.
    fn to_allow(&self) -> Option<syn::Attribute> {
        if self.path().is_ident("allow") {
            return Some(self.clone());
        }

        if !self.path().is_ident("expect") {
            return None;
        }

        // Turn an `expect` into allow
        let mut attr = self.clone();
        let path = match &mut attr.meta {
            syn::Meta::Path(path) => path,
            syn::Meta::List(meta) => &mut meta.path,
            syn::Meta::NameValue(meta) => &mut meta.path,
        };

        *path = syn::parse_quote_spanned!(path.span()=> allow);

        Some(attr)
    }
}
