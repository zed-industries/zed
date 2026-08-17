use crate::ir::comp::{CompInfo, CompKind, Field, FieldMethods};
use crate::ir::context::BindgenContext;
use crate::ir::item::{IsOpaque, Item};
use crate::ir::ty::{TypeKind, RUST_DERIVE_IN_ARRAY_LIMIT};

/// Generate a manual implementation of `PartialEq` trait for the
/// specified compound type.
pub(crate) fn gen_partialeq_impl(
    ctx: &BindgenContext,
    comp_info: &CompInfo,
    item: &Item,
    ty_for_impl: &proc_macro2::TokenStream,
) -> Option<proc_macro2::TokenStream> {
    let mut tokens = vec![];

    if item.is_opaque(ctx, &()) {
        tokens.push(quote! {
            &self._bindgen_opaque_blob[..] == &other._bindgen_opaque_blob[..]
        });
    } else if comp_info.kind() == CompKind::Union {
        assert!(!ctx.options().untagged_union);
        tokens.push(quote! {
            &self.bindgen_union_field[..] == &other.bindgen_union_field[..]
        });
    } else {
        for base in comp_info.base_members() {
            if !base.requires_storage(ctx) {
                continue;
            }

            let ty_item = ctx.resolve_item(base.ty);
            let field_name = &base.field_name;

            if ty_item.is_opaque(ctx, &()) {
                let field_name = ctx.rust_ident(field_name);
                tokens.push(quote! {
                    &self. #field_name [..] == &other. #field_name [..]
                });
            } else {
                tokens.push(gen_field(ctx, ty_item, field_name));
            }
        }

        for field in comp_info.fields() {
            match *field {
                Field::DataMember(ref fd) => {
                    let ty_item = ctx.resolve_item(fd.ty());
                    let name = fd.name().unwrap();
                    tokens.push(gen_field(ctx, ty_item, name));
                }
                Field::Bitfields(ref bu) => {
                    for bitfield in bu.bitfields() {
                        if bitfield.name().is_some() {
                            let getter_name = bitfield.getter_name();
                            let name_ident = ctx.rust_ident_raw(getter_name);
                            tokens.push(quote! {
                                self.#name_ident () == other.#name_ident ()
                            });
                        }
                    }
                }
            }
        }
    }

    Some(quote! {
        fn eq(&self, other: & #ty_for_impl) -> bool {
            #( #tokens )&&*
        }
    })
}

fn gen_field(
    ctx: &BindgenContext,
    ty_item: &Item,
    name: &str,
) -> proc_macro2::TokenStream {
    fn quote_equals(
        name_ident: proc_macro2::Ident,
    ) -> proc_macro2::TokenStream {
        quote! { self.#name_ident == other.#name_ident }
    }

    let name_ident = ctx.rust_ident(name);
    let ty = ty_item.expect_type();

    match *ty.kind() {
        TypeKind::Void |
        TypeKind::NullPtr |
        TypeKind::Int(..) |
        TypeKind::Complex(..) |
        TypeKind::Float(..) |
        TypeKind::Enum(..) |
        TypeKind::TypeParam |
        TypeKind::UnresolvedTypeRef(..) |
        TypeKind::Reference(..) |
        TypeKind::ObjCInterface(..) |
        TypeKind::ObjCId |
        TypeKind::ObjCSel |
        TypeKind::Comp(..) |
        TypeKind::Pointer(_) |
        TypeKind::Function(..) |
        TypeKind::Opaque => quote_equals(name_ident),

        TypeKind::TemplateInstantiation(ref inst) => {
            if inst.is_opaque(ctx, ty_item) {
                quote! {
                    &self. #name_ident [..] == &other. #name_ident [..]
                }
            } else {
                quote_equals(name_ident)
            }
        }

        TypeKind::Array(_, len) => {
            if len <= RUST_DERIVE_IN_ARRAY_LIMIT ||
                ctx.options().rust_features().larger_arrays
            {
                quote_equals(name_ident)
            } else {
                quote! {
                    &self. #name_ident [..] == &other. #name_ident [..]
                }
            }
        }
        TypeKind::Vector(_, len) => {
            let self_ids = 0..len;
            let other_ids = 0..len;
            quote! {
                #(self.#self_ids == other.#other_ids &&)* true
            }
        }

        TypeKind::ResolvedTypeRef(t) |
        TypeKind::TemplateAlias(t, _) |
        TypeKind::Alias(t) |
        TypeKind::BlockPointer(t) => {
            let inner_item = ctx.resolve_item(t);
            gen_field(ctx, inner_item, name)
        }
    }
}
