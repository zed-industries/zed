pub(crate) trait ItemExt {
    fn attrs_mut(&mut self) -> Option<&mut Vec<syn::Attribute>>;
}

impl ItemExt for syn::Item {
    fn attrs_mut(&mut self) -> Option<&mut Vec<syn::Attribute>> {
        let attrs = match self {
            Self::Const(item) => &mut item.attrs,
            Self::Enum(item) => &mut item.attrs,
            Self::ExternCrate(item) => &mut item.attrs,
            Self::Fn(item) => &mut item.attrs,
            Self::ForeignMod(item) => &mut item.attrs,
            Self::Impl(item) => &mut item.attrs,
            Self::Macro(item) => &mut item.attrs,
            Self::Mod(item) => &mut item.attrs,
            Self::Static(item) => &mut item.attrs,
            Self::Struct(item) => &mut item.attrs,
            Self::Trait(item) => &mut item.attrs,
            Self::TraitAlias(item) => &mut item.attrs,
            Self::Type(item) => &mut item.attrs,
            Self::Union(item) => &mut item.attrs,
            Self::Use(item) => &mut item.attrs,
            _ => return None,
        };

        Some(attrs)
    }
}
