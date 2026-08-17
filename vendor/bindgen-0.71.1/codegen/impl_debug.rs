use crate::ir::comp::{BitfieldUnit, CompKind, Field, FieldData, FieldMethods};
use crate::ir::context::BindgenContext;
use crate::ir::item::{HasTypeParamInArray, IsOpaque, Item, ItemCanonicalName};
use crate::ir::ty::{TypeKind, RUST_DERIVE_IN_ARRAY_LIMIT};
use std::fmt::Write as _;

pub(crate) fn gen_debug_impl(
    ctx: &BindgenContext,
    fields: &[Field],
    item: &Item,
    kind: CompKind,
) -> proc_macro2::TokenStream {
    let struct_name = item.canonical_name(ctx);
    let mut format_string = format!("{struct_name} {{{{ ");
    let mut tokens = vec![];

    if item.is_opaque(ctx, &()) {
        format_string.push_str("opaque");
    } else {
        match kind {
            CompKind::Union => {
                format_string.push_str("union");
            }
            CompKind::Struct => {
                let processed_fields = fields.iter().filter_map(|f| match f {
                    Field::DataMember(ref fd) => fd.impl_debug(ctx, ()),
                    Field::Bitfields(ref bu) => bu.impl_debug(ctx, ()),
                });

                for (i, (fstring, toks)) in processed_fields.enumerate() {
                    if i > 0 {
                        format_string.push_str(", ");
                    }
                    tokens.extend(toks);
                    format_string.push_str(&fstring);
                }
            }
        }
    }

    format_string.push_str(" }}");
    tokens.insert(0, quote! { #format_string });

    let prefix = ctx.trait_prefix();

    quote! {
        fn fmt(&self, f: &mut ::#prefix::fmt::Formatter<'_>) -> ::#prefix ::fmt::Result {
            write!(f, #( #tokens ),*)
        }
    }
}

/// A trait for the things which we can codegen tokens that contribute towards a
/// generated `impl Debug`.
pub(crate) trait ImplDebug<'a> {
    /// Any extra parameter required by this a particular `ImplDebug` implementation.
    type Extra;

    /// Generate a format string snippet to be included in the larger `impl Debug`
    /// format string, and the code to get the format string's interpolation values.
    fn impl_debug(
        &self,
        ctx: &BindgenContext,
        extra: Self::Extra,
    ) -> Option<(String, Vec<proc_macro2::TokenStream>)>;
}

impl ImplDebug<'_> for FieldData {
    type Extra = ();

    fn impl_debug(
        &self,
        ctx: &BindgenContext,
        _: Self::Extra,
    ) -> Option<(String, Vec<proc_macro2::TokenStream>)> {
        if let Some(name) = self.name() {
            ctx.resolve_item(self.ty()).impl_debug(ctx, name)
        } else {
            None
        }
    }
}

impl ImplDebug<'_> for BitfieldUnit {
    type Extra = ();

    fn impl_debug(
        &self,
        ctx: &BindgenContext,
        _: Self::Extra,
    ) -> Option<(String, Vec<proc_macro2::TokenStream>)> {
        let mut format_string = String::new();
        let mut tokens = vec![];
        for (i, bitfield) in self.bitfields().iter().enumerate() {
            if i > 0 {
                format_string.push_str(", ");
            }

            if let Some(bitfield_name) = bitfield.name() {
                let _ = write!(format_string, "{bitfield_name} : {{:?}}");
                let getter_name = bitfield.getter_name();
                let name_ident = ctx.rust_ident_raw(getter_name);
                tokens.push(quote! {
                    self.#name_ident ()
                });
            }
        }

        Some((format_string, tokens))
    }
}

impl<'a> ImplDebug<'a> for Item {
    type Extra = &'a str;

    fn impl_debug(
        &self,
        ctx: &BindgenContext,
        name: &str,
    ) -> Option<(String, Vec<proc_macro2::TokenStream>)> {
        let name_ident = ctx.rust_ident(name);

        // We don't know if blocklisted items `impl Debug` or not, so we can't
        // add them to the format string we're building up.
        if !ctx.allowlisted_items().contains(&self.id()) {
            return None;
        }

        let ty = self.as_type()?;

        fn debug_print(
            name: &str,
            name_ident: proc_macro2::TokenStream,
        ) -> Option<(String, Vec<proc_macro2::TokenStream>)> {
            Some((
                format!("{name}: {{:?}}"),
                vec![quote! {
                    self.#name_ident
                }],
            ))
        }

        match *ty.kind() {
            // Handle the simple cases.
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Complex(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::UnresolvedTypeRef(..) |
            TypeKind::ObjCInterface(..) |
            TypeKind::ObjCId |
            TypeKind::Comp(..) |
            TypeKind::ObjCSel => debug_print(name, quote! { #name_ident }),

            TypeKind::TemplateInstantiation(ref inst) => {
                if inst.is_opaque(ctx, self) {
                    Some((format!("{name}: opaque"), vec![]))
                } else {
                    debug_print(name, quote! { #name_ident })
                }
            }

            // The generic is not required to implement Debug, so we can not debug print that type
            TypeKind::TypeParam => {
                Some((format!("{name}: Non-debuggable generic"), vec![]))
            }

            TypeKind::Array(_, len) => {
                // Generics are not required to implement Debug
                if self.has_type_param_in_array(ctx) {
                    Some((format!("{name}: Array with length {len}"), vec![]))
                } else if len < RUST_DERIVE_IN_ARRAY_LIMIT ||
                    ctx.options().rust_features().larger_arrays
                {
                    // The simple case
                    debug_print(name, quote! { #name_ident })
                } else if ctx.options().use_core {
                    // There is no String in core; reducing field visibility to avoid breaking
                    // no_std setups.
                    Some((format!("{name}: [...]"), vec![]))
                } else {
                    // Let's implement our own print function
                    Some((
                        format!("{name}: [{{}}]"),
                        vec![quote! {{
                            use std::fmt::Write as _;
                            let mut output = String::new();
                            let mut iter = self.#name_ident.iter();
                            if let Some(value) = iter.next() {
                                let _ = write!(output, "{value:?}");
                                for value in iter {
                                    let _ = write!(output, ", {value:?}");
                                }
                            }
                            output
                        }}],
                    ))
                }
            }
            TypeKind::Vector(_, len) => {
                if ctx.options().use_core {
                    // There is no format! in core; reducing field visibility to avoid breaking
                    // no_std setups.
                    Some((format!("{name}(...)"), vec![]))
                } else {
                    let self_ids = 0..len;
                    Some((
                        format!("{name}({{}})"),
                        vec![quote! {
                            #(format!("{:?}", self.#self_ids)),*
                        }],
                    ))
                }
            }

            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) |
            TypeKind::BlockPointer(t) => {
                // We follow the aliases
                ctx.resolve_item(t).impl_debug(ctx, name)
            }

            TypeKind::Pointer(inner) => {
                let inner_type = ctx.resolve_type(inner).canonical_type(ctx);
                match *inner_type.kind() {
                    TypeKind::Function(ref sig)
                        if !sig.function_pointers_can_derive() =>
                    {
                        Some((format!("{name}: FunctionPointer"), vec![]))
                    }
                    _ => debug_print(name, quote! { #name_ident }),
                }
            }

            TypeKind::Opaque => None,
        }
    }
}
