use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug, Clone)]
pub(crate) enum BonCratePath {
    Default,
    Explicit(syn::Path),
}

impl Default for BonCratePath {
    fn default() -> Self {
        Self::Default
    }
}

impl FromMeta for BonCratePath {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let path = super::parse_path_mod_style(meta)?;

        let prefix = &path
            .segments
            .first()
            .ok_or_else(|| err!(&path, "path must have at least one segment"))?
            .ident;

        let is_absolute = path.leading_colon.is_some() || prefix == "crate" || prefix == "$crate";

        if is_absolute {
            return Ok(Self::Explicit(path));
        }

        if prefix == "super" || prefix == "self" {
            bail!(
                &path,
                "path must not be relative; specify the path that starts with `crate::` \
                instead; if you want to refer to a reexport from an external crate then \
                use leading colons like `::crate_name::reexport::path::bon`"
            )
        }

        let path_str = darling::util::path_to_string(&path);

        bail!(
            &path,
            "path must be absolute; if you want to refer to a reexport from an external \
            crate then add leading colons like `::{path_str}`; if the path leads to a module \
            in the current crate, then specify the absolute path with `crate` like \
            `crate::reexport::path::bon` or `$crate::reexport::path::bon` (if within a macro)"
        );
    }
}

impl ToTokens for BonCratePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Default => tokens.extend(quote!(::bon)),
            Self::Explicit(path) => path.to_tokens(tokens),
        }
    }
}
