use syn::{
    parse::{Parse, ParseStream},
    Generics,
};

////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ImplHeader {
    pub(crate) generics: Generics,
    pub(crate) self_ty: syn::Path,
}

impl Parse for ImplHeader {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut generics = input.parse::<Generics>()?;

        let self_ty = input.parse()?;

        generics.where_clause = input.parse()?;

        Ok(Self { generics, self_ty })
    }
}
