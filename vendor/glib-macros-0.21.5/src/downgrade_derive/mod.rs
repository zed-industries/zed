// Take a look at the license at the top of the repository in the LICENSE file.

mod enums;
mod fields;
mod structs;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

pub fn impl_downgrade(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(data_struct) => {
            structs::derive_downgrade_for_struct(input.ident, input.generics, data_struct)
        }
        Data::Enum(data_enum) => {
            enums::derive_downgrade_for_enum(input.ident, input.generics, data_enum)
        }
        Data::Union(..) => {
            panic!("#[derive(Downgrade)] is not available for unions.");
        }
    }
}
