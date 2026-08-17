use super::models::FinishFnParams;
use super::top_level_config::TopLevelConfig;
use super::{
    AssocMethodCtxParams, BuilderGenCtx, FinishFnBody, Generics, Member, MemberOrigin, RawMember,
};
use crate::builder::builder_gen::models::{BuilderGenCtxParams, BuilderTypeParams, StartFnParams};
use crate::normalization::{GenericsNamespace, SyntaxVariant};
use crate::parsing::{ItemSigConfig, SpannedKey};
use crate::util::prelude::*;
use std::borrow::Cow;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;

fn parse_top_level_config(item_struct: &syn::ItemStruct) -> Result<TopLevelConfig> {
    let configs = item_struct
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("builder"))
        .map(|attr| {
            let meta = match &attr.meta {
                syn::Meta::List(meta) => meta,
                syn::Meta::Path(_) => bail!(
                    &attr.meta,
                    "this empty `#[builder]` attribute is redundant; remove it"
                ),
                syn::Meta::NameValue(_) => bail!(
                    &attr.meta,
                    "`#[builder = ...]` syntax is unsupported; use `#[builder(...)]` instead"
                ),
            };

            crate::parsing::require_non_empty_paren_meta_list_or_name_value(&attr.meta)?;

            Ok(meta.tokens.clone())
        })
        .collect::<Result<Vec<_>>>()?;

    TopLevelConfig::parse_for_struct(configs)
}

pub(crate) struct StructInputCtx {
    struct_item: SyntaxVariant<syn::ItemStruct>,
    config: TopLevelConfig,
    struct_ty: syn::Type,
}

impl StructInputCtx {
    pub(crate) fn new(orig_struct: syn::ItemStruct) -> Result<Self> {
        let params = parse_top_level_config(&orig_struct)?;

        let generic_args = orig_struct
            .generics
            .params
            .iter()
            .map(syn::GenericParam::to_generic_argument);
        let struct_ident = &orig_struct.ident;
        let struct_ty = syn::parse_quote!(#struct_ident<#(#generic_args),*>);

        let mut norm_struct = orig_struct.clone();

        // Structs are free to use `Self` inside of their trait bounds and any
        // internal type contexts. However, when copying these bounds to the
        // builder struct and its impl blocks we need to get rid of `Self`
        // references and replace them with the actual struct type.
        crate::normalization::NormalizeSelfTy {
            self_ty: &struct_ty,
        }
        .visit_item_struct_mut(&mut norm_struct);

        let struct_item = SyntaxVariant {
            orig: orig_struct,
            norm: norm_struct,
        };

        Ok(Self {
            struct_item,
            config: params,
            struct_ty,
        })
    }

    pub(crate) fn into_builder_gen_ctx(self) -> Result<BuilderGenCtx> {
        let fields = self
            .struct_item
            .apply_ref(|struct_item| match &struct_item.fields {
                syn::Fields::Named(fields) => Ok(fields),
                _ => {
                    bail!(&struct_item, "Only structs with named fields are supported")
                }
            });

        let norm_fields = fields.norm?;
        let orig_fields = fields.orig?;

        let members = norm_fields
            .named
            .iter()
            .zip(&orig_fields.named)
            .map(|(norm_field, orig_field)| {
                let ident = norm_field.ident.clone().ok_or_else(|| {
                    err!(norm_field, "only structs with named fields are supported")
                })?;

                let ty = SyntaxVariant {
                    norm: Box::new(norm_field.ty.clone()),
                    orig: Box::new(orig_field.ty.clone()),
                };

                Ok(RawMember {
                    attrs: &norm_field.attrs,
                    ident,
                    ty,
                    span: orig_field.ident.span(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let members = Member::from_raw(&self.config, MemberOrigin::StructField, members)?;

        let generics = Generics::new(
            self.struct_item
                .norm
                .generics
                .params
                .iter()
                .cloned()
                .collect(),
            self.struct_item.norm.generics.where_clause.clone(),
        );

        let finish_fn_body = StructLiteralBody {
            struct_ident: self.struct_item.norm.ident.clone(),
        };

        let ItemSigConfig {
            name: start_fn_ident,
            vis: start_fn_vis,
            docs: start_fn_docs,
        } = self.config.start_fn;

        let start_fn_ident = start_fn_ident
            .map(SpannedKey::into_value)
            .unwrap_or_else(|| syn::Ident::new("builder", self.struct_item.norm.ident.span()));

        let ItemSigConfig {
            name: finish_fn_ident,
            vis: finish_fn_vis,
            docs: finish_fn_docs,
        } = self.config.finish_fn;

        let finish_fn_ident = finish_fn_ident
            .map(SpannedKey::into_value)
            .unwrap_or_else(|| syn::Ident::new("build", start_fn_ident.span()));

        let struct_ty = &self.struct_ty;
        let finish_fn = FinishFnParams {
            ident: finish_fn_ident,
            vis: finish_fn_vis.map(SpannedKey::into_value),
            unsafety: None,
            asyncness: None,
            special_attrs: vec![syn::parse_quote! {
                #[must_use = "building a struct without using it is likely a bug"]
            }],
            body: Box::new(finish_fn_body),
            output: syn::parse_quote!(-> #struct_ty),
            attrs: finish_fn_docs
                .map(SpannedKey::into_value)
                .unwrap_or_else(|| {
                    vec![syn::parse_quote! {
                        /// Finish building and return the requested object
                    }]
                }),
        };

        let start_fn_docs = start_fn_docs
            .map(SpannedKey::into_value)
            .unwrap_or_else(|| {
                let docs = format!(
                    "Create an instance of [`{}`] using the builder syntax",
                    self.struct_item.norm.ident
                );

                vec![syn::parse_quote!(#[doc = #docs])]
            });

        let start_fn = StartFnParams {
            ident: start_fn_ident,
            vis: start_fn_vis.map(SpannedKey::into_value),
            docs: start_fn_docs,
            generics: None,
            span: None,
        };

        let assoc_method_ctx = Some(AssocMethodCtxParams {
            self_ty: self.struct_ty.into(),
            receiver: None,
        });

        let allow_attrs = self
            .struct_item
            .norm
            .attrs
            .iter()
            .filter_map(syn::Attribute::to_allow)
            .collect();

        let builder_type = {
            let ItemSigConfig { name, vis, docs } = self.config.builder_type;

            let builder_ident = name.map(SpannedKey::into_value).unwrap_or_else(|| {
                format_ident!("{}Builder", self.struct_item.norm.ident.raw_name())
            });

            BuilderTypeParams {
                derives: self.config.derive,
                ident: builder_ident,
                docs: docs.map(SpannedKey::into_value),
                vis: vis.map(SpannedKey::into_value),
            }
        };

        let mut namespace = GenericsNamespace::default();
        namespace.visit_item_struct(&self.struct_item.orig);

        BuilderGenCtx::new(BuilderGenCtxParams {
            bon: self.config.bon,
            namespace: Cow::Owned(namespace),
            members,

            allow_attrs,

            const_: self.config.const_,
            on: self.config.on,

            assoc_method_ctx,
            generics,
            orig_item_vis: self.struct_item.norm.vis,

            builder_type,
            state_mod: self.config.state_mod,
            start_fn,
            finish_fn,
        })
    }
}

struct StructLiteralBody {
    struct_ident: syn::Ident,
}

impl FinishFnBody for StructLiteralBody {
    fn generate(&self, ctx: &BuilderGenCtx) -> TokenStream {
        let Self { struct_ident } = self;

        // The variables with values of members are in scope for this expression.
        let member_vars = ctx.members.iter().map(Member::orig_ident);

        quote! {
            #struct_ident {
                #(#member_vars,)*
            }
        }
    }
}
