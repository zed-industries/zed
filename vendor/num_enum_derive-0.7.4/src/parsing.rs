use crate::enum_attributes::ErrorTypeAttribute;
use crate::utils::die;
use crate::variant_attributes::{NumEnumVariantAttributeItem, NumEnumVariantAttributes};
use proc_macro2::Span;
use quote::{format_ident, ToTokens};
use std::collections::BTreeSet;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Attribute, Data, DeriveInput, Expr, ExprLit, ExprUnary, Fields, Ident, Lit,
    LitInt, Meta, Path, Result, UnOp,
};

pub(crate) struct EnumInfo {
    pub(crate) name: Ident,
    pub(crate) repr: Ident,
    pub(crate) variants: Vec<VariantInfo>,
    pub(crate) error_type_info: ErrorType,
}

impl EnumInfo {
    /// Returns whether the number of variants (ignoring defaults, catch-alls, etc) is the same as
    /// the capacity of the repr.
    pub(crate) fn is_naturally_exhaustive(&self) -> Result<bool> {
        let repr_str = self.repr.to_string();
        if !repr_str.is_empty() {
            let suffix = repr_str
                .strip_prefix('i')
                .or_else(|| repr_str.strip_prefix('u'));
            if let Some(suffix) = suffix {
                if suffix == "size" {
                    return Ok(false);
                } else if let Ok(bits) = suffix.parse::<u32>() {
                    let variants = 1usize.checked_shl(bits);
                    return Ok(variants.map_or(false, |v| {
                        v == self
                            .variants
                            .iter()
                            .map(|v| v.alternative_values.len() + 1)
                            .sum()
                    }));
                }
            }
        }
        die!(self.repr.clone() => "Failed to parse repr into bit size");
    }

    pub(crate) fn default(&self) -> Option<&Ident> {
        self.variants
            .iter()
            .find(|info| info.is_default)
            .map(|info| &info.ident)
    }

    pub(crate) fn catch_all(&self) -> Option<&Ident> {
        self.variants
            .iter()
            .find(|info| info.is_catch_all)
            .map(|info| &info.ident)
    }

    pub(crate) fn variant_idents(&self) -> Vec<Ident> {
        self.variants
            .iter()
            .filter(|variant| !variant.is_catch_all)
            .map(|variant| variant.ident.clone())
            .collect()
    }

    pub(crate) fn expression_idents(&self) -> Vec<Vec<Ident>> {
        self.variants
            .iter()
            .filter(|variant| !variant.is_catch_all)
            .map(|info| {
                let indices = 0..(info.alternative_values.len() + 1);
                indices
                    .map(|index| format_ident!("{}__num_enum_{}__", info.ident, index))
                    .collect()
            })
            .collect()
    }

    pub(crate) fn variant_expressions(&self) -> Vec<Vec<Expr>> {
        self.variants
            .iter()
            .filter(|variant| !variant.is_catch_all)
            .map(|variant| variant.all_values().cloned().collect())
            .collect()
    }

    fn parse_attrs<Attrs: Iterator<Item = Attribute>>(
        attrs: Attrs,
    ) -> Result<(Ident, Option<ErrorType>)> {
        let mut maybe_repr = None;
        let mut maybe_error_type = None;
        for attr in attrs {
            if let Meta::List(meta_list) = &attr.meta {
                if let Some(ident) = meta_list.path.get_ident() {
                    if ident == "repr" {
                        let mut nested = meta_list.tokens.clone().into_iter();
                        let repr_tree = match (nested.next(), nested.next()) {
                            (Some(repr_tree), None) => repr_tree,
                            _ => die!(attr =>
                                "Expected exactly one `repr` argument"
                            ),
                        };
                        let repr_ident: Ident = parse_quote! {
                            #repr_tree
                        };
                        if repr_ident == "C" {
                            die!(repr_ident =>
                                "repr(C) doesn't have a well defined size"
                            );
                        } else {
                            maybe_repr = Some(repr_ident);
                        }
                    } else if ident == "num_enum" {
                        let attributes =
                            attr.parse_args_with(crate::enum_attributes::Attributes::parse)?;
                        if let Some(error_type) = attributes.error_type {
                            if maybe_error_type.is_some() {
                                die!(attr => "At most one num_enum error_type attribute may be specified");
                            }
                            maybe_error_type = Some(error_type.into());
                        }
                    }
                }
            }
        }
        if maybe_repr.is_none() {
            die!("Missing `#[repr({Integer})]` attribute");
        }
        Ok((maybe_repr.unwrap(), maybe_error_type))
    }
}

impl Parse for EnumInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok({
            let input: DeriveInput = input.parse()?;
            let name = input.ident;
            let data = match input.data {
                Data::Enum(data) => data,
                Data::Union(data) => die!(data.union_token => "Expected enum but found union"),
                Data::Struct(data) => die!(data.struct_token => "Expected enum but found struct"),
            };

            let (repr, maybe_error_type) = Self::parse_attrs(input.attrs.into_iter())?;

            let mut variants: Vec<VariantInfo> = vec![];
            let mut has_default_variant: bool = false;
            let mut has_catch_all_variant: bool = false;

            // Vec to keep track of the used discriminants and alt values.
            let mut discriminant_int_val_set = BTreeSet::new();

            let mut next_discriminant = literal(0);
            for variant in data.variants.into_iter() {
                let ident = variant.ident.clone();

                let discriminant = match &variant.discriminant {
                    Some(d) => d.1.clone(),
                    None => next_discriminant.clone(),
                };

                let mut raw_alternative_values: Vec<Expr> = vec![];
                // Keep the attribute around for better error reporting.
                let mut alt_attr_ref: Vec<&Attribute> = vec![];

                // `#[num_enum(default)]` is required by `#[derive(FromPrimitive)]`
                // and forbidden by `#[derive(UnsafeFromPrimitive)]`, so we need to
                // keep track of whether we encountered such an attribute:
                let mut is_default: bool = false;
                let mut is_catch_all: bool = false;

                for attribute in &variant.attrs {
                    if attribute.path().is_ident("default") {
                        if has_default_variant {
                            die!(attribute =>
                                "Multiple variants marked `#[default]` or `#[num_enum(default)]` found"
                            );
                        } else if has_catch_all_variant {
                            die!(attribute =>
                                "Attribute `default` is mutually exclusive with `catch_all`"
                            );
                        }
                        is_default = true;
                        has_default_variant = true;
                    }

                    if attribute.path().is_ident("num_enum") {
                        match attribute.parse_args_with(NumEnumVariantAttributes::parse) {
                            Ok(variant_attributes) => {
                                for variant_attribute in variant_attributes.items {
                                    match variant_attribute {
                                        NumEnumVariantAttributeItem::Default(default) => {
                                            if has_default_variant {
                                                die!(default.keyword =>
                                                    "Multiple variants marked `#[default]` or `#[num_enum(default)]` found"
                                                );
                                            } else if has_catch_all_variant {
                                                die!(default.keyword =>
                                                    "Attribute `default` is mutually exclusive with `catch_all`"
                                                );
                                            }
                                            is_default = true;
                                            has_default_variant = true;
                                        }
                                        NumEnumVariantAttributeItem::CatchAll(catch_all) => {
                                            if has_catch_all_variant {
                                                die!(catch_all.keyword =>
                                                    "Multiple variants marked with `#[num_enum(catch_all)]`"
                                                );
                                            } else if has_default_variant {
                                                die!(catch_all.keyword =>
                                                    "Attribute `catch_all` is mutually exclusive with `default`"
                                                );
                                            }

                                            match variant
                                                .fields
                                                .iter()
                                                .collect::<Vec<_>>()
                                                .as_slice()
                                            {
                                                [syn::Field {
                                                    ty: syn::Type::Path(syn::TypePath { path, .. }),
                                                    ..
                                                }] if path.is_ident(&repr) => {
                                                    is_catch_all = true;
                                                    has_catch_all_variant = true;
                                                }
                                                _ => {
                                                    die!(catch_all.keyword =>
                                                        "Variant with `catch_all` must be a tuple with exactly 1 field matching the repr type"
                                                    );
                                                }
                                            }
                                        }
                                        NumEnumVariantAttributeItem::Alternatives(alternatives) => {
                                            raw_alternative_values.extend(alternatives.expressions);
                                            alt_attr_ref.push(attribute);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                if cfg!(not(feature = "complex-expressions")) {
                                    let tokens = attribute.meta.to_token_stream();

                                    let attribute_str = format!("{}", tokens);
                                    if attribute_str.contains("alternatives")
                                        && attribute_str.contains("..")
                                    {
                                        // Give a nice error message suggesting how to fix the problem.
                                        die!(attribute => "Ranges are only supported as num_enum alternate values if the `complex-expressions` feature of the crate `num_enum` is enabled".to_string())
                                    }
                                }
                                die!(attribute =>
                                    format!("Invalid attribute: {}", err)
                                );
                            }
                        }
                    }
                }

                if !is_catch_all {
                    match &variant.fields {
                        Fields::Named(_) | Fields::Unnamed(_) => {
                            die!(variant => format!("`{}` only supports unit variants (with no associated data), but `{}::{}` was not a unit variant.", get_crate_name(), name, ident));
                        }
                        Fields::Unit => {}
                    }
                }

                let discriminant_value = parse_discriminant(&discriminant)?;

                // Check for collision.
                // We can't do const evaluation, or even compare arbitrary Exprs,
                // so unfortunately we can't check for duplicates.
                // That's not the end of the world, just we'll end up with compile errors for
                // matches with duplicate branches in generated code instead of nice friendly error messages.
                if let DiscriminantValue::Literal(canonical_value_int) = discriminant_value {
                    if discriminant_int_val_set.contains(&canonical_value_int) {
                        die!(ident => format!("The discriminant '{}' collides with a value attributed to a previous variant", canonical_value_int))
                    }
                }

                // Deal with the alternative values.
                let mut flattened_alternative_values = Vec::new();
                let mut flattened_raw_alternative_values = Vec::new();
                for raw_alternative_value in raw_alternative_values {
                    let expanded_values = parse_alternative_values(&raw_alternative_value)?;
                    for expanded_value in expanded_values {
                        flattened_alternative_values.push(expanded_value);
                        flattened_raw_alternative_values.push(raw_alternative_value.clone())
                    }
                }

                if !flattened_alternative_values.is_empty() {
                    let alternate_int_values = flattened_alternative_values
                        .into_iter()
                        .map(|v| {
                            match v {
                                DiscriminantValue::Literal(value) => Ok(value),
                                DiscriminantValue::Expr(expr) => {
                                    if let Expr::Range(_) = expr {
                                        if cfg!(not(feature = "complex-expressions")) {
                                            // Give a nice error message suggesting how to fix the problem.
                                            die!(expr => "Ranges are only supported as num_enum alternate values if the `complex-expressions` feature of the crate `num_enum` is enabled".to_string())
                                        }
                                    }
                                    // We can't do uniqueness checking on non-literals, so we don't allow them as alternate values.
                                    // We could probably allow them, but there doesn't seem to be much of a use-case,
                                    // and it's easier to give good error messages about duplicate values this way,
                                    // rather than rustc errors on conflicting match branches.
                                    die!(expr => "Only literals are allowed as num_enum alternate values".to_string())
                                },
                            }
                        })
                        .collect::<Result<Vec<i128>>>()?;
                    let mut sorted_alternate_int_values = alternate_int_values.clone();
                    sorted_alternate_int_values.sort_unstable();
                    let sorted_alternate_int_values = sorted_alternate_int_values;

                    // Check if the current discriminant is not in the alternative values.
                    if let DiscriminantValue::Literal(canonical_value_int) = discriminant_value {
                        if let Some(index) = alternate_int_values
                            .iter()
                            .position(|&x| x == canonical_value_int)
                        {
                            die!(&flattened_raw_alternative_values[index] => format!("'{}' in the alternative values is already attributed as the discriminant of this variant", canonical_value_int));
                        }
                    }

                    // Search for duplicates, the vec is sorted. Warn about them.
                    if (1..sorted_alternate_int_values.len()).any(|i| {
                        sorted_alternate_int_values[i] == sorted_alternate_int_values[i - 1]
                    }) {
                        let attr = *alt_attr_ref.last().unwrap();
                        die!(attr => "There is duplication in the alternative values");
                    }
                    // Search if those discriminant_int_val_set where already attributed.
                    // (discriminant_int_val_set is BTreeSet, and iter().next_back() is the is the maximum in the set.)
                    if let Some(last_upper_val) = discriminant_int_val_set.iter().next_back() {
                        if sorted_alternate_int_values.first().unwrap() <= last_upper_val {
                            for (index, val) in alternate_int_values.iter().enumerate() {
                                if discriminant_int_val_set.contains(val) {
                                    die!(&flattened_raw_alternative_values[index] => format!("'{}' in the alternative values is already attributed to a previous variant", val));
                                }
                            }
                        }
                    }

                    // Reconstruct the alternative_values vec of Expr but sorted.
                    flattened_raw_alternative_values = sorted_alternate_int_values
                        .iter()
                        .map(|val| literal(val.to_owned()))
                        .collect();

                    // Add the alternative values to the the set to keep track.
                    discriminant_int_val_set.extend(sorted_alternate_int_values);
                }

                // Add the current discriminant to the the set to keep track.
                if let DiscriminantValue::Literal(canonical_value_int) = discriminant_value {
                    discriminant_int_val_set.insert(canonical_value_int);
                }

                variants.push(VariantInfo {
                    ident,
                    is_default,
                    is_catch_all,
                    canonical_value: discriminant,
                    alternative_values: flattened_raw_alternative_values,
                });

                // Get the next value for the discriminant.
                next_discriminant = match discriminant_value {
                    DiscriminantValue::Literal(int_value) => literal(int_value.wrapping_add(1)),
                    DiscriminantValue::Expr(expr) => {
                        parse_quote! {
                            #repr::wrapping_add(#expr, 1)
                        }
                    }
                }
            }

            let error_type_info = maybe_error_type.unwrap_or_else(|| {
                let crate_name = Ident::new(&get_crate_name(), Span::call_site());
                ErrorType {
                    name: parse_quote! {
                        ::#crate_name::TryFromPrimitiveError<Self>
                    },
                    constructor: parse_quote! {
                        ::#crate_name::TryFromPrimitiveError::<Self>::new
                    },
                }
            });

            EnumInfo {
                name,
                repr,
                variants,
                error_type_info,
            }
        })
    }
}

fn literal(i: i128) -> Expr {
    Expr::Lit(ExprLit {
        lit: Lit::Int(LitInt::new(&i.to_string(), Span::call_site())),
        attrs: vec![],
    })
}

enum DiscriminantValue {
    Literal(i128),
    Expr(Expr),
}

fn parse_discriminant(val_exp: &Expr) -> Result<DiscriminantValue> {
    let mut sign = 1;
    let mut unsigned_expr = val_exp;
    if let Expr::Unary(ExprUnary {
        op: UnOp::Neg(..),
        expr,
        ..
    }) = val_exp
    {
        unsigned_expr = expr;
        sign = -1;
    }
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(ref lit_int),
        ..
    }) = unsigned_expr
    {
        Ok(DiscriminantValue::Literal(
            sign * lit_int.base10_parse::<i128>()?,
        ))
    } else {
        Ok(DiscriminantValue::Expr(val_exp.clone()))
    }
}

#[cfg(feature = "complex-expressions")]
fn parse_alternative_values(val_expr: &Expr) -> Result<Vec<DiscriminantValue>> {
    fn range_expr_value_to_number(
        parent_range_expr: &Expr,
        range_bound_value: &Option<Box<Expr>>,
    ) -> Result<i128> {
        // Avoid needing to calculate what the lower and upper bound would be - these are type dependent,
        // and also may not be obvious in context (e.g. an omitted bound could reasonably mean "from the last discriminant" or "from the lower bound of the type").
        if let Some(range_bound_value) = range_bound_value {
            let range_bound_value = parse_discriminant(range_bound_value.as_ref())?;
            // If non-literals are used, we can't expand to the mapped values, so can't write a nice match statement or do exhaustiveness checking.
            // Require literals instead.
            if let DiscriminantValue::Literal(value) = range_bound_value {
                return Ok(value);
            }
        }
        die!(parent_range_expr => "When ranges are used for alternate values, both bounds most be explicitly specified numeric literals")
    }

    if let Expr::Range(syn::ExprRange {
        start, end, limits, ..
    }) = val_expr
    {
        let lower = range_expr_value_to_number(val_expr, start)?;
        let upper = range_expr_value_to_number(val_expr, end)?;
        // While this is technically allowed in Rust, and results in an empty range, it's almost certainly a mistake in this context.
        if lower > upper {
            die!(val_expr => "When using ranges for alternate values, upper bound must not be less than lower bound");
        }
        let mut values = Vec::with_capacity((upper - lower) as usize);
        let mut next = lower;
        loop {
            match limits {
                syn::RangeLimits::HalfOpen(..) => {
                    if next == upper {
                        break;
                    }
                }
                syn::RangeLimits::Closed(..) => {
                    if next > upper {
                        break;
                    }
                }
            }
            values.push(DiscriminantValue::Literal(next));
            next += 1;
        }
        return Ok(values);
    }
    parse_discriminant(val_expr).map(|v| vec![v])
}

#[cfg(not(feature = "complex-expressions"))]
fn parse_alternative_values(val_expr: &Expr) -> Result<Vec<DiscriminantValue>> {
    parse_discriminant(val_expr).map(|v| vec![v])
}

pub(crate) struct VariantInfo {
    ident: Ident,
    is_default: bool,
    is_catch_all: bool,
    canonical_value: Expr,
    alternative_values: Vec<Expr>,
}

impl VariantInfo {
    fn all_values(&self) -> impl Iterator<Item = &Expr> {
        ::core::iter::once(&self.canonical_value).chain(self.alternative_values.iter())
    }
}

pub(crate) struct ErrorType {
    pub(crate) name: Path,
    pub(crate) constructor: Path,
}

impl From<ErrorTypeAttribute> for ErrorType {
    fn from(attribute: ErrorTypeAttribute) -> Self {
        Self {
            name: attribute.name.path,
            constructor: attribute.constructor.path,
        }
    }
}

#[cfg(feature = "proc-macro-crate")]
pub(crate) fn get_crate_name() -> String {
    let found_crate = proc_macro_crate::crate_name("num_enum").unwrap_or_else(|err| {
        eprintln!("Warning: {}\n    => defaulting to `num_enum`", err,);
        proc_macro_crate::FoundCrate::Itself
    });

    match found_crate {
        proc_macro_crate::FoundCrate::Itself => String::from("num_enum"),
        proc_macro_crate::FoundCrate::Name(name) => name,
    }
}

// Don't depend on proc-macro-crate in no_std environments because it causes an awkward dependency
// on serde with std.
//
// no_std dependees on num_enum cannot rename the num_enum crate when they depend on it. Sorry.
//
// See https://github.com/illicitonion/num_enum/issues/18
#[cfg(not(feature = "proc-macro-crate"))]
pub(crate) fn get_crate_name() -> String {
    String::from("num_enum")
}
