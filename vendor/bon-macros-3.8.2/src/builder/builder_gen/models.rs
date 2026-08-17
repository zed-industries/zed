use super::member::Member;
use super::top_level_config::{DerivesConfig, OnConfig};
use crate::normalization::GenericsNamespace;
use crate::parsing::{BonCratePath, ItemSigConfig, SpannedKey};
use crate::util::prelude::*;
use std::borrow::Cow;

pub(super) trait FinishFnBody {
    /// Generate the `finish` function body from the ready-made variables.
    /// The generated function body may assume that there are variables
    /// named the same as the members in scope.
    fn generate(&self, ctx: &BuilderGenCtx) -> TokenStream;
}

pub(super) struct AssocMethodReceiverCtx {
    pub(super) with_self_keyword: syn::Receiver,
    pub(super) without_self_keyword: Box<syn::Type>,

    /// Name of the receiver field in the builder struct.
    pub(super) field_ident: syn::Ident,
}

pub(super) struct AssocMethodReceiverCtxParams {
    pub(super) with_self_keyword: syn::Receiver,
    pub(super) without_self_keyword: Box<syn::Type>,
}

pub(super) struct AssocMethodCtx {
    /// The `Self` type of the impl block. It doesn't contain any nested
    /// `Self` keywords in it. This is prohibited by Rust's syntax itself.
    pub(super) self_ty: Box<syn::Type>,

    /// Present only if the method has a receiver, i.e. `self` or `&self` or
    /// `&mut self` or `self: ExplicitType`.
    pub(super) receiver: Option<AssocMethodReceiverCtx>,
}

pub(super) struct AssocMethodCtxParams {
    /// The `Self` type of the impl block. It doesn't contain any nested
    /// `Self` keywords in it. This is prohibited by Rust's syntax itself.
    pub(super) self_ty: Box<syn::Type>,

    /// Present only if the method has a receiver, i.e. `self` or `&self` or
    /// `&mut self` or `self: ExplicitType`.
    pub(super) receiver: Option<AssocMethodReceiverCtxParams>,
}

pub(super) struct FinishFn {
    pub(super) ident: syn::Ident,

    /// Visibility override specified by the user
    pub(super) vis: syn::Visibility,

    /// Additional attributes to apply to the item
    pub(super) attrs: Vec<syn::Attribute>,

    pub(super) unsafety: Option<syn::Token![unsafe]>,
    pub(super) asyncness: Option<syn::Token![async]>,
    /// Special attributes to propagate, such as [`must_use`](<https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute>)
    /// and [`track_caller`](<https://doc.rust-lang.org/reference/attributes/codegen.html#r-attributes.codegen.track_caller>)
    pub(super) special_attrs: Vec<syn::Attribute>,
    pub(super) body: Box<dyn FinishFnBody>,
    pub(super) output: syn::ReturnType,
}

pub(super) struct FinishFnParams {
    pub(super) ident: syn::Ident,

    /// Visibility override specified by the user
    pub(super) vis: Option<syn::Visibility>,

    pub(super) attrs: Vec<syn::Attribute>,
    pub(super) unsafety: Option<syn::Token![unsafe]>,
    pub(super) asyncness: Option<syn::Token![async]>,
    pub(super) special_attrs: Vec<syn::Attribute>,
    pub(super) body: Box<dyn FinishFnBody>,
    pub(super) output: syn::ReturnType,
}

pub(super) struct StartFn {
    pub(super) ident: syn::Ident,
    pub(super) vis: syn::Visibility,

    pub(super) docs: Vec<syn::Attribute>,

    /// Overrides the default generics
    pub(super) generics: Option<Generics>,

    /// Preserve a span the documentation for `start_fn` should link to.
    pub(super) span: Span,
}

pub(super) struct StartFnParams {
    pub(super) ident: syn::Ident,

    /// If present overrides the default visibility derived from the builder's type.
    pub(super) vis: Option<syn::Visibility>,

    pub(super) docs: Vec<syn::Attribute>,

    /// Overrides the default generics
    pub(super) generics: Option<Generics>,
    /// Preserve a span the documentation for `start_fn` should link to.
    /// Defaults to macro callsite if not supplied.
    pub(super) span: Option<Span>,
}

pub(super) struct BuilderType {
    pub(super) ident: syn::Ident,

    /// Visibility of the builder module itself.
    pub(super) vis: syn::Visibility,

    pub(super) derives: DerivesConfig,
    pub(super) docs: Vec<syn::Attribute>,
}

pub(super) struct BuilderTypeParams {
    pub(super) ident: syn::Ident,
    pub(super) vis: Option<syn::Visibility>,
    pub(super) derives: DerivesConfig,
    pub(super) docs: Option<Vec<syn::Attribute>>,
}

pub(super) struct StateMod {
    pub(super) ident: syn::Ident,

    /// Visibility of the builder module itself.
    pub(super) vis: syn::Visibility,

    /// Visibility equivalent to the [`Self::vis`], but for items
    /// generated inside the builder child module.
    pub(super) vis_child: syn::Visibility,

    /// Visibility equivalent to the [`Self::vis_child`], but for items
    /// generated inside one more level of nesting in the builder child module.
    pub(super) vis_child_child: syn::Visibility,

    pub(super) docs: Vec<syn::Attribute>,
}

pub(super) struct Generics {
    pub(super) where_clause: Option<syn::WhereClause>,

    /// Original generics that may contain default values in them. This is only
    /// suitable for use in places where default values for generic parameters
    /// are allowed.
    pub(super) decl_with_defaults: Vec<syn::GenericParam>,

    /// Generic parameters without default values in them. This is suitable for
    /// use as generics in function signatures or impl blocks.
    pub(super) decl_without_defaults: Vec<syn::GenericParam>,

    /// Mirrors the `decl` representing how generic params should be represented
    /// when these parameters are passed through as arguments in a turbofish.
    pub(super) args: Vec<syn::GenericArgument>,
}

pub(crate) struct BuilderGenCtx {
    pub(super) bon: BonCratePath,

    /// Name of the generic variable that holds the builder's state.
    pub(super) state_var: syn::Ident,

    pub(super) members: Vec<Member>,

    /// Lint suppressions from the original item that will be inherited by all items
    /// generated by the macro. If the original syntax used `#[expect(...)]`,
    /// then it must be represented as `#[allow(...)]` here.
    pub(super) allow_attrs: Vec<syn::Attribute>,
    pub(super) const_: Option<syn::Token![const]>,
    pub(super) on: Vec<OnConfig>,

    pub(super) generics: Generics,

    pub(super) assoc_method_ctx: Option<AssocMethodCtx>,

    pub(super) builder_type: BuilderType,
    pub(super) state_mod: StateMod,
    pub(super) start_fn: StartFn,
    pub(super) finish_fn: FinishFn,
}

pub(super) struct BuilderGenCtxParams<'a> {
    pub(crate) bon: BonCratePath,
    pub(super) namespace: Cow<'a, GenericsNamespace>,
    pub(super) members: Vec<Member>,

    pub(super) allow_attrs: Vec<syn::Attribute>,
    pub(super) const_: Option<syn::Token![const]>,
    pub(super) on: Vec<OnConfig>,

    /// This is the visibility of the original item that the builder is generated for.
    /// For example, the `struct` or `fn` item visibility that the `#[builder]` or
    /// `#[derive(Builder)]` attribute is applied to.
    ///
    /// It is used as the default visibility for all the generated items unless
    /// explicitly overridden at a more specific level.
    pub(super) orig_item_vis: syn::Visibility,

    /// Generics to apply to the builder type.
    pub(super) generics: Generics,

    pub(super) assoc_method_ctx: Option<AssocMethodCtxParams>,

    pub(super) builder_type: BuilderTypeParams,
    pub(super) state_mod: ItemSigConfig,
    pub(super) start_fn: StartFnParams,
    pub(super) finish_fn: FinishFnParams,
}

impl BuilderGenCtx {
    pub(super) fn new(params: BuilderGenCtxParams<'_>) -> Result<Self> {
        let BuilderGenCtxParams {
            bon,
            namespace,
            members,
            allow_attrs,
            const_,
            on,
            generics,
            orig_item_vis,
            assoc_method_ctx,
            builder_type,
            state_mod,
            start_fn,
            finish_fn,
        } = params;

        let builder_type = BuilderType {
            ident: builder_type.ident,
            vis: builder_type.vis.unwrap_or(orig_item_vis),
            derives: builder_type.derives,
            docs: builder_type.docs.unwrap_or_else(|| {
                let doc = format!(
                    "Use builder syntax to set the inputs and finish with [`{0}()`](Self::{0}()).",
                    finish_fn.ident
                );

                vec![syn::parse_quote! {
                    #[doc = #doc]
                }]
            }),
        };

        let state_mod = {
            let is_ident_overridden = state_mod.name.is_some();
            let ident = state_mod
                .name
                .map(SpannedKey::into_value)
                .unwrap_or_else(|| builder_type.ident.pascal_to_snake_case());

            if builder_type.ident == ident {
                if is_ident_overridden {
                    bail!(
                        &ident,
                        "the builder module name must be different from the builder type name"
                    )
                }

                bail!(
                    &builder_type.ident,
                    "couldn't infer the builder module name that doesn't conflict with \
                    the builder type name; by default, the builder module name is set \
                    to a snake_case equivalent of the builder type name; the snake_case \
                    conversion doesn't produce a different name for this builder type \
                    name; consider using PascalCase for the builder type name or specify \
                    a separate name for the builder module explicitly via \
                    `#[builder(state_mod = {{new_name}})]`"
                );
            }

            // The builder module is private by default, meaning all symbols under
            // that module can't be accessed from outside the module where the builder
            // is defined. This makes the builder type signature unnamable from outside
            // the module where we output the builder. The users need to explicitly
            // opt-in to make the builder module public.
            let vis = state_mod
                .vis
                .map(SpannedKey::into_value)
                .unwrap_or_else(|| syn::Visibility::Inherited);

            // The visibility for child items is based on the visibility of the
            // builder type itself, because the types and traits from this module
            // are part of the builder's generic type state parameter signature.
            let vis_child = builder_type.vis.clone().into_equivalent_in_child_module()?;
            let vis_child_child = vis_child.clone().into_equivalent_in_child_module()?;

            StateMod {
                vis,
                vis_child,
                vis_child_child,

                ident,

                docs: state_mod
                    .docs
                    .map(SpannedKey::into_value)
                    .unwrap_or_else(|| {
                        let docs = format!(
                            "Tools for manipulating the type state of [`{}`].\n\
                            \n\
                            See the [detailed guide](https://bon-rs.com/guide/typestate-api) \
                            that describes how all the pieces here fit together.",
                            builder_type.ident
                        );

                        vec![syn::parse_quote!(#[doc = #docs])]
                    }),
            }
        };

        let start_fn = StartFn {
            ident: start_fn.ident,
            vis: start_fn.vis.unwrap_or_else(|| builder_type.vis.clone()),
            docs: start_fn.docs,
            generics: start_fn.generics,
            span: start_fn.span.unwrap_or_else(Span::call_site),
        };

        let finish_fn = FinishFn {
            ident: finish_fn.ident,
            vis: finish_fn.vis.unwrap_or_else(|| builder_type.vis.clone()),
            attrs: finish_fn.attrs,
            unsafety: finish_fn.unsafety,
            asyncness: finish_fn.asyncness,
            special_attrs: finish_fn.special_attrs,
            body: finish_fn.body,
            output: finish_fn.output,
        };

        let state_var = {
            let possible_names = ["S", "State", "BuilderState"];
            possible_names
                .iter()
                .find(|&&candidate| !namespace.idents.contains(candidate))
                .map(|&name| syn::Ident::new(name, Span::call_site()))
                .unwrap_or_else(|| namespace.unique_ident(format!("{}_", possible_names[0])))
        };

        let assoc_method_ctx = assoc_method_ctx.map(|ctx| {
            let receiver = ctx.receiver.map(|receiver| {
                let start_fn_arg_names = members
                    .iter()
                    .filter_map(Member::as_start_fn)
                    .map(|member| member.ident.to_string())
                    .collect();

                let field_ident = crate::normalization::unique_name(
                    &start_fn_arg_names,
                    "self_receiver".to_owned(),
                );

                AssocMethodReceiverCtx {
                    with_self_keyword: receiver.with_self_keyword,
                    without_self_keyword: receiver.without_self_keyword,
                    field_ident: syn::Ident::new(&field_ident, Span::call_site()),
                }
            });

            AssocMethodCtx {
                self_ty: ctx.self_ty,
                receiver,
            }
        });

        Ok(Self {
            bon,
            state_var,
            members,
            allow_attrs,
            const_,
            on,
            generics,
            assoc_method_ctx,
            builder_type,
            state_mod,
            start_fn,
            finish_fn,
        })
    }
}

impl Generics {
    pub(super) fn new(
        decl_with_defaults: Vec<syn::GenericParam>,
        where_clause: Option<syn::WhereClause>,
    ) -> Self {
        let decl_without_defaults = decl_with_defaults
            .iter()
            .cloned()
            .map(|mut param| {
                match &mut param {
                    syn::GenericParam::Type(param) => {
                        param.default = None;
                    }
                    syn::GenericParam::Const(param) => {
                        param.default = None;
                    }
                    syn::GenericParam::Lifetime(_) => {}
                }
                param
            })
            .collect();

        let args = decl_with_defaults
            .iter()
            .map(syn::GenericParam::to_generic_argument)
            .collect();

        Self {
            where_clause,
            decl_with_defaults,
            decl_without_defaults,
            args,
        }
    }

    pub(super) fn where_clause_predicates(&self) -> impl Iterator<Item = &syn::WherePredicate> {
        self.where_clause
            .as_ref()
            .into_iter()
            .flat_map(|clause| &clause.predicates)
    }
}
