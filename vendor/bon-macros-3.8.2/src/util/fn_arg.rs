pub(crate) trait FnArgExt {
    fn attrs_mut(&mut self) -> &mut Vec<syn::Attribute>;
    fn ty_mut(&mut self) -> &mut syn::Type;
    fn as_receiver(&self) -> Option<&syn::Receiver>;
    fn as_typed(&self) -> Option<&syn::PatType>;
}

impl FnArgExt for syn::FnArg {
    fn attrs_mut(&mut self) -> &mut Vec<syn::Attribute> {
        match self {
            Self::Receiver(arg) => &mut arg.attrs,
            Self::Typed(arg) => &mut arg.attrs,
        }
    }

    fn ty_mut(&mut self) -> &mut syn::Type {
        match self {
            Self::Receiver(arg) => &mut arg.ty,
            Self::Typed(arg) => &mut arg.ty,
        }
    }

    fn as_receiver(&self) -> Option<&syn::Receiver> {
        match self {
            Self::Typed(_) => None,
            Self::Receiver(arg) => Some(arg),
        }
    }

    fn as_typed(&self) -> Option<&syn::PatType> {
        match self {
            Self::Typed(arg) => Some(arg),
            Self::Receiver(_) => None,
        }
    }
}
