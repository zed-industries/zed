pub(crate) trait GenericParamExt {
    fn to_generic_argument(&self) -> syn::GenericArgument;
}

impl GenericParamExt for syn::GenericParam {
    fn to_generic_argument(&self) -> syn::GenericArgument {
        match self {
            Self::Lifetime(param) => syn::GenericArgument::Lifetime(param.lifetime.clone()),
            Self::Type(param) => {
                let ident = &param.ident;
                syn::GenericArgument::Type(syn::parse_quote!(#ident))
            }
            Self::Const(param) => {
                let ident = &param.ident;
                syn::GenericArgument::Const(syn::parse_quote!(#ident))
            }
        }
    }
}
