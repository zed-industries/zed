use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{Attribute, Error, Ident, Path};

use super::attributes::item;

pub(crate) const BORSH: &str = "borsh";

pub(crate) fn get(attrs: &[Attribute]) -> Result<Path, Error> {
    let path = item::get_crate(attrs)?;
    match path {
        Some(path) => Ok(path),
        None => {
            let ident = get_from_cargo();
            Ok(ident.into())
        }
    }
}

pub(crate) fn get_from_cargo() -> Ident {
    let name = &crate_name(BORSH)
        .unwrap_or_else(|err| panic!("`proc_macro_crate::crate_name` call error: {:#?}", err));
    let name = match name {
        FoundCrate::Itself => BORSH,
        FoundCrate::Name(name) => name.as_str(),
    };
    Ident::new(name, Span::call_site())
}
