use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use syn::{meta::ParseNestedMeta, Attribute, WherePredicate};

use self::bounds::BOUNDS_FIELD_PARSE_MAP;

use super::{
    get_one_attribute,
    parsing::{attr_get_by_symbol_keys, meta_get_by_symbol_keys, parse_lit_into},
    BoundType, Symbol, BORSH, BOUND, DESERIALIZE_WITH, SERIALIZE_WITH, SKIP,
};

#[cfg(feature = "schema")]
use {
    super::schema_keys::{PARAMS, SCHEMA, WITH_FUNCS},
    schema::SCHEMA_FIELD_PARSE_MAP,
};

pub mod bounds;
#[cfg(feature = "schema")]
pub mod schema;

enum Variants {
    Bounds(bounds::Bounds),
    SerializeWith(syn::ExprPath),
    DeserializeWith(syn::ExprPath),
    Skip(()),
    #[cfg(feature = "schema")]
    Schema(schema::Attributes),
}

type ParseFn = dyn Fn(Symbol, Symbol, &ParseNestedMeta) -> syn::Result<Variants> + Send + Sync;

static BORSH_FIELD_PARSE_MAP: Lazy<BTreeMap<Symbol, Box<ParseFn>>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    // assigning closure `let f = |args| {...};` and boxing closure `let f: Box<ParseFn> = Box::new(f);`
    // on 2 separate lines doesn't work
    let f_bounds: Box<ParseFn> = Box::new(|_attr_name, _meta_item_name, meta| {
        let map_result = meta_get_by_symbol_keys(BOUND, meta, &BOUNDS_FIELD_PARSE_MAP)?;
        let bounds_attributes: bounds::Bounds = map_result.into();
        Ok(Variants::Bounds(bounds_attributes))
    });

    let f_serialize_with: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into::<syn::ExprPath>(attr_name, meta_item_name, meta)
            .map(Variants::SerializeWith)
    });

    let f_deserialize_with: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into::<syn::ExprPath>(attr_name, meta_item_name, meta)
            .map(Variants::DeserializeWith)
    });

    #[cfg(feature = "schema")]
    let f_schema: Box<ParseFn> = Box::new(|_attr_name, _meta_item_name, meta| {
        let map_result = meta_get_by_symbol_keys(SCHEMA, meta, &SCHEMA_FIELD_PARSE_MAP)?;
        let schema_attributes: schema::Attributes = map_result.into();
        Ok(Variants::Schema(schema_attributes))
    });

    let f_skip: Box<ParseFn> =
        Box::new(|_attr_name, _meta_item_name, _meta| Ok(Variants::Skip(())));
    m.insert(BOUND, f_bounds);
    m.insert(SERIALIZE_WITH, f_serialize_with);
    m.insert(DESERIALIZE_WITH, f_deserialize_with);
    m.insert(SKIP, f_skip);
    #[cfg(feature = "schema")]
    m.insert(SCHEMA, f_schema);
    m
});

#[derive(Default, Clone)]
pub(crate) struct Attributes {
    pub bounds: Option<bounds::Bounds>,
    pub serialize_with: Option<syn::ExprPath>,
    pub deserialize_with: Option<syn::ExprPath>,
    pub skip: bool,
    #[cfg(feature = "schema")]
    pub schema: Option<schema::Attributes>,
}

impl From<BTreeMap<Symbol, Variants>> for Attributes {
    fn from(mut map: BTreeMap<Symbol, Variants>) -> Self {
        let bounds = map.remove(&BOUND);
        let serialize_with = map.remove(&SERIALIZE_WITH);
        let deserialize_with = map.remove(&DESERIALIZE_WITH);
        let skip = map.remove(&SKIP);
        let bounds = bounds.map(|variant| match variant {
            Variants::Bounds(bounds) => bounds,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });

        let serialize_with = serialize_with.map(|variant| match variant {
            Variants::SerializeWith(serialize_with) => serialize_with,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });

        let deserialize_with = deserialize_with.map(|variant| match variant {
            Variants::DeserializeWith(deserialize_with) => deserialize_with,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });

        let skip = skip.map(|variant| match variant {
            Variants::Skip(skip) => skip,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });

        #[cfg(feature = "schema")]
        let schema = {
            let schema = map.remove(&SCHEMA);
            schema.map(|variant| match variant {
                Variants::Schema(schema) => schema,
                _ => {
                    unreachable!("only one enum variant is expected to correspond to given map key")
                }
            })
        };
        Self {
            bounds,
            serialize_with,
            deserialize_with,
            skip: skip.is_some(),
            #[cfg(feature = "schema")]
            schema,
        }
    }
}

#[cfg(feature = "schema")]
pub(crate) fn filter_attrs(
    attrs: impl Iterator<Item = Attribute>,
) -> impl Iterator<Item = Attribute> {
    attrs.filter(|attr| attr.path() == BORSH)
}

impl Attributes {
    fn check(&self, attr: &Attribute) -> Result<(), syn::Error> {
        if self.skip && (self.serialize_with.is_some() || self.deserialize_with.is_some()) {
            return Err(syn::Error::new_spanned(
                attr,
                format!(
                    "`{}` cannot be used at the same time as `{}` or `{}`",
                    SKIP.0, SERIALIZE_WITH.0, DESERIALIZE_WITH.0
                ),
            ));
        }

        #[cfg(feature = "schema")]
        self.check_schema(attr)?;

        Ok(())
    }
    pub(crate) fn parse(attrs: &[Attribute]) -> Result<Self, syn::Error> {
        let borsh = get_one_attribute(attrs)?;

        let result: Self = if let Some(attr) = borsh {
            let result: Self = attr_get_by_symbol_keys(BORSH, attr, &BORSH_FIELD_PARSE_MAP)?.into();
            result.check(attr)?;
            result
        } else {
            BTreeMap::new().into()
        };

        Ok(result)
    }
    pub(crate) fn needs_bounds_derive(&self, ty: BoundType) -> bool {
        let predicates = self.get_bounds(ty);
        predicates.is_none()
    }

    fn get_bounds(&self, ty: BoundType) -> Option<Vec<WherePredicate>> {
        let bounds = self.bounds.as_ref();
        bounds.and_then(|bounds| match ty {
            BoundType::Serialize => bounds.serialize.clone(),
            BoundType::Deserialize => bounds.deserialize.clone(),
        })
    }
    pub(crate) fn collect_bounds(&self, ty: BoundType) -> Vec<WherePredicate> {
        let predicates = self.get_bounds(ty);
        predicates.unwrap_or_default()
    }
}

#[cfg(feature = "schema")]
impl Attributes {
    fn check_schema(&self, attr: &Attribute) -> Result<(), syn::Error> {
        if let Some(ref schema) = self.schema {
            if self.skip && schema.params.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    format!(
                        "`{}` cannot be used at the same time as `{}({})`",
                        SKIP.0, SCHEMA.0, PARAMS.1
                    ),
                ));
            }

            if self.skip && schema.with_funcs.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    format!(
                        "`{}` cannot be used at the same time as `{}({})`",
                        SKIP.0, SCHEMA.0, WITH_FUNCS.1
                    ),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn needs_schema_params_derive(&self) -> bool {
        if let Some(ref schema) = self.schema {
            if schema.params.is_some() {
                return false;
            }
        }
        true
    }

    pub(crate) fn schema_declaration(&self) -> Option<syn::ExprPath> {
        self.schema.as_ref().and_then(|schema| {
            schema
                .with_funcs
                .as_ref()
                .and_then(|with_funcs| with_funcs.declaration.clone())
        })
    }

    pub(crate) fn schema_definitions(&self) -> Option<syn::ExprPath> {
        self.schema.as_ref().and_then(|schema| {
            schema
                .with_funcs
                .as_ref()
                .and_then(|with_funcs| with_funcs.definitions.clone())
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{Attribute, ItemStruct};

    fn parse_bounds(attrs: &[Attribute]) -> Result<Option<bounds::Bounds>, syn::Error> {
        // #[borsh(bound(serialize = "...", deserialize = "..."))]
        let borsh_attrs = Attributes::parse(attrs)?;
        Ok(borsh_attrs.bounds)
    }

    use crate::internals::test_helpers::{
        debug_print_tokenizable, debug_print_vec_of_tokenizable, local_insta_assert_debug_snapshot,
        local_insta_assert_snapshot,
    };

    use super::{bounds, Attributes};

    #[test]
    fn test_reject_multiple_borsh_attrs() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(skip)]
                #[borsh(bound(deserialize = "K: Hash + Ord,
                     V: Eq + Ord",
                    serialize = "K: Hash + Eq + Ord,
                     V: Ord"
                ))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match Attributes::parse(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_bounds_parsing1() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = "K: Hash + Ord,
                     V: Eq + Ord",
                    serialize = "K: Hash + Eq + Ord,
                     V: Ord"
                ))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = parse_bounds(&first_field.attrs).unwrap().unwrap();
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(attrs.serialize.clone()));
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(attrs.deserialize));
    }

    #[test]
    fn test_bounds_parsing2() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = "K: Hash + Eq + borsh::de::BorshDeserialize,
                     V: borsh::de::BorshDeserialize",
                    serialize = "K: Hash + Eq + borsh::ser::BorshSerialize,
                     V: borsh::ser::BorshSerialize"
                ))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = parse_bounds(&first_field.attrs).unwrap().unwrap();
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(attrs.serialize.clone()));
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(attrs.deserialize));
    }

    #[test]
    fn test_bounds_parsing3() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = "K: Hash + Eq + borsh::de::BorshDeserialize,
                     V: borsh::de::BorshDeserialize",
                    serialize = ""
                ))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = parse_bounds(&first_field.attrs).unwrap().unwrap();
        assert_eq!(attrs.serialize.as_ref().unwrap().len(), 0);
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(attrs.deserialize));
    }

    #[test]
    fn test_bounds_parsing4() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = "K: Hash"))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = parse_bounds(&first_field.attrs).unwrap().unwrap();
        assert!(attrs.serialize.is_none());
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(attrs.deserialize));
    }

    #[test]
    fn test_bounds_parsing_error() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deser = "K: Hash"))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match parse_bounds(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_bounds_parsing_error2() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = "K Hash"))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match parse_bounds(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_bounds_parsing_error3() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = 42))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match parse_bounds(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_ser_de_with_parsing1() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(
                    serialize_with = "third_party_impl::serialize_third_party",
                    deserialize_with = "third_party_impl::deserialize_third_party",
                )]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = Attributes::parse(&first_field.attrs).unwrap();
        local_insta_assert_snapshot!(debug_print_tokenizable(attrs.serialize_with.as_ref()));
        local_insta_assert_snapshot!(debug_print_tokenizable(attrs.deserialize_with));
    }
    #[test]
    fn test_borsh_skip() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(skip)]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];

        let result = Attributes::parse(&first_field.attrs).unwrap();
        assert!(result.skip);
    }
    #[test]
    fn test_borsh_no_skip() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];

        let result = Attributes::parse(&first_field.attrs).unwrap();
        assert!(!result.skip);
    }
}

#[cfg(feature = "schema")]
#[cfg(test)]
mod tests_schema {
    use crate::internals::{
        attributes::field::Attributes,
        test_helpers::{
            debug_print_tokenizable, debug_print_vec_of_tokenizable,
            local_insta_assert_debug_snapshot, local_insta_assert_snapshot,
        },
    };

    use quote::quote;
    use syn::{Attribute, ItemStruct};

    use super::schema;
    fn parse_schema_attrs(attrs: &[Attribute]) -> Result<Option<schema::Attributes>, syn::Error> {
        // #[borsh(schema(params = "..."))]
        let borsh_attrs = Attributes::parse(attrs)?;
        Ok(borsh_attrs.schema)
    }

    #[test]
    fn test_root_bounds_and_params_combined() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(
                    serialize_with = "third_party_impl::serialize_third_party",
                    bound(deserialize = "K: Hash"),
                    schema(params = "T => <T as TraitName>::Associated, V => Vec<V>")
                )]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];

        let attrs = Attributes::parse(&first_field.attrs).unwrap();
        let bounds = attrs.bounds.clone().unwrap();
        assert!(bounds.serialize.is_none());
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(bounds.deserialize));
        assert!(attrs.deserialize_with.is_none());
        let schema = attrs.schema.clone().unwrap();
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(schema.params.clone()));
        local_insta_assert_snapshot!(debug_print_tokenizable(attrs.serialize_with));
    }

    #[test]
    fn test_schema_params_parsing1() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(params =
                    "T => <T as TraitName>::Associated"
               ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let schema_attrs = parse_schema_attrs(&first_field.attrs).unwrap();
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(schema_attrs.unwrap().params));
    }
    #[test]
    fn test_schema_params_parsing_error() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(params =
                    "T => <T as TraitName, W>::Associated"
               ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match parse_schema_attrs(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_schema_params_parsing_error2() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(paramsbum =
                    "T => <T as TraitName>::Associated"
               ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match parse_schema_attrs(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_schema_params_parsing2() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(params =
                    "T => <T as TraitName>::Associated, V => Vec<V>"
               ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let schema_attrs = parse_schema_attrs(&first_field.attrs).unwrap();
        local_insta_assert_snapshot!(debug_print_vec_of_tokenizable(schema_attrs.unwrap().params));
    }
    #[test]
    fn test_schema_params_parsing3() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(params = "" ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let schema_attrs = parse_schema_attrs(&first_field.attrs).unwrap();
        assert_eq!(schema_attrs.unwrap().params.unwrap().len(), 0);
    }

    #[test]
    fn test_schema_params_parsing4() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let schema_attrs = parse_schema_attrs(&first_field.attrs).unwrap();
        assert!(schema_attrs.is_none());
    }

    #[test]
    fn test_schema_with_funcs_parsing() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(schema(with_funcs(
                    declaration = "third_party_impl::declaration::<K, V>",
                    definitions = "third_party_impl::add_definitions_recursively::<K, V>"
                )))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = Attributes::parse(&first_field.attrs).unwrap();
        let schema = attrs.schema.unwrap();
        let with_funcs = schema.with_funcs.unwrap();

        local_insta_assert_snapshot!(debug_print_tokenizable(with_funcs.declaration.clone()));
        local_insta_assert_snapshot!(debug_print_tokenizable(with_funcs.definitions));
    }

    // both `declaration` and `definitions` have to be specified
    #[test]
    fn test_schema_with_funcs_parsing_error() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(schema(with_funcs(
                    declaration = "third_party_impl::declaration::<K, V>"
                )))]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let attrs = Attributes::parse(&first_field.attrs);

        let err = match attrs {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_root_error() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(boons)]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];
        let err = match Attributes::parse(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }

    #[test]
    fn test_root_bounds_and_wrong_key_combined() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                #[borsh(bound(deserialize = "K: Hash"),
                        schhema(params = "T => <T as TraitName>::Associated, V => Vec<V>")
                )]
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let first_field = &item_struct.fields.into_iter().collect::<Vec<_>>()[0];

        let err = match Attributes::parse(&first_field.attrs) {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }
}
