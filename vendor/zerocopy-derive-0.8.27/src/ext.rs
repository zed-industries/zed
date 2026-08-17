// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Data, DataEnum, DataStruct, DataUnion, Field, Ident, Index, Type, Visibility};

pub(crate) trait DataExt {
    /// Extracts the names and types of all fields. For enums, extracts the
    /// names and types of fields from each variant. For tuple structs, the
    /// names are the indices used to index into the struct (ie, `0`, `1`, etc).
    ///
    /// FIXME: Extracting field names for enums doesn't really make sense. Types
    /// makes sense because we don't care about where they live - we just care
    /// about transitive ownership. But for field names, we'd only use them when
    /// generating is_bit_valid, which cares about where they live.
    fn fields(&self) -> Vec<(&Visibility, TokenStream, &Type)>;

    fn variants(&self) -> Vec<Vec<(&Visibility, TokenStream, &Type)>>;

    fn tag(&self) -> Option<Ident>;
}

impl DataExt for Data {
    fn fields(&self) -> Vec<(&Visibility, TokenStream, &Type)> {
        match self {
            Data::Struct(strc) => strc.fields(),
            Data::Enum(enm) => enm.fields(),
            Data::Union(un) => un.fields(),
        }
    }

    fn variants(&self) -> Vec<Vec<(&Visibility, TokenStream, &Type)>> {
        match self {
            Data::Struct(strc) => strc.variants(),
            Data::Enum(enm) => enm.variants(),
            Data::Union(un) => un.variants(),
        }
    }

    fn tag(&self) -> Option<Ident> {
        match self {
            Data::Struct(strc) => strc.tag(),
            Data::Enum(enm) => enm.tag(),
            Data::Union(un) => un.tag(),
        }
    }
}

impl DataExt for DataStruct {
    fn fields(&self) -> Vec<(&Visibility, TokenStream, &Type)> {
        map_fields(&self.fields)
    }

    fn variants(&self) -> Vec<Vec<(&Visibility, TokenStream, &Type)>> {
        vec![self.fields()]
    }

    fn tag(&self) -> Option<Ident> {
        None
    }
}

impl DataExt for DataEnum {
    fn fields(&self) -> Vec<(&Visibility, TokenStream, &Type)> {
        map_fields(self.variants.iter().flat_map(|var| &var.fields))
    }

    fn variants(&self) -> Vec<Vec<(&Visibility, TokenStream, &Type)>> {
        self.variants.iter().map(|var| map_fields(&var.fields)).collect()
    }

    fn tag(&self) -> Option<Ident> {
        Some(Ident::new("___ZerocopyTag", Span::call_site()))
    }
}

impl DataExt for DataUnion {
    fn fields(&self) -> Vec<(&Visibility, TokenStream, &Type)> {
        map_fields(&self.fields.named)
    }

    fn variants(&self) -> Vec<Vec<(&Visibility, TokenStream, &Type)>> {
        vec![self.fields()]
    }

    fn tag(&self) -> Option<Ident> {
        None
    }
}

fn map_fields<'a>(
    fields: impl 'a + IntoIterator<Item = &'a Field>,
) -> Vec<(&'a Visibility, TokenStream, &'a Type)> {
    fields
        .into_iter()
        .enumerate()
        .map(|(idx, f)| {
            (
                &f.vis,
                f.ident
                    .as_ref()
                    .map(ToTokens::to_token_stream)
                    .unwrap_or_else(|| Index::from(idx).to_token_stream()),
                &f.ty,
            )
        })
        .collect()
}
