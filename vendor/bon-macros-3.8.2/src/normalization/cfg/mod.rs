mod parse;
mod visit;

use crate::util::prelude::*;
use darling::ast::NestedMeta;
use parse::CfgSyntax;
use std::collections::BTreeSet;
use syn::parse::Parser;

pub(crate) enum Expansion {
    Expanded(Expanded),
    Recurse(TokenStream),
}

pub(crate) struct Expanded {
    pub(crate) config: TokenStream,
    pub(crate) item: Box<syn::Item>,
}

pub(crate) struct ExpandCfg {
    pub(crate) current_macro: syn::Ident,
    pub(crate) config: TokenStream,
    pub(crate) item: Box<syn::Item>,
}

impl ExpandCfg {
    pub(crate) fn expand_cfg(mut self) -> Result<Expansion> {
        let predicates = self.collect_predicates()?;

        if predicates.is_empty() {
            return Ok(Expansion::Expanded(Expanded {
                config: self.config,
                item: self.item,
            }));
        }

        let predicate_results = match parse::parse_predicate_results(self.config.clone())? {
            Some(predicate_results) => predicate_results,
            None => return self.into_recursion(0, &predicates),
        };

        // Update the config to remove the `@cfgs(...)` prefix from them
        self.config = predicate_results.rest;

        let true_predicates: BTreeSet<_> = predicates
            .iter()
            .map(ToString::to_string)
            .zip(predicate_results.results)
            .filter(|(_, result)| *result)
            .map(|(predicate, _)| predicate)
            .collect();

        visit::visit_attrs(&mut self.item, |attrs| eval_cfgs(&true_predicates, attrs))?;

        // Collect predicates again after the cfgs were evaluated. This is needed
        // because cfgs may create new cfgs e.g.: `#[cfg_attr(foo, cfg_attr(bar, ...))]`.
        let predicates = self.collect_predicates()?;

        if predicates.is_empty() {
            return Ok(Expansion::Expanded(Expanded {
                config: self.config,
                item: self.item,
            }));
        }

        self.into_recursion(predicate_results.recursion_counter + 1, &predicates)
    }

    /// There is no mutation happening here, but we just reuse the same
    /// visitor implementation that works with mutable references.
    fn collect_predicates(&mut self) -> Result<Vec<TokenStream>> {
        let mut predicates = vec![];
        let mut visited = BTreeSet::new();

        visit::visit_attrs(&mut self.item, |attrs| {
            for attr in attrs {
                let cfg_syntax = match CfgSyntax::from_meta(&attr.meta)? {
                    Some(cfg_syntax) => cfg_syntax,
                    None => continue,
                };

                let predicate = match cfg_syntax {
                    CfgSyntax::Cfg(predicate) => predicate,
                    CfgSyntax::CfgAttr(cfg_attr) => cfg_attr.predicate.to_token_stream(),
                };

                if visited.insert(predicate.to_string()) {
                    predicates.push(predicate);
                }
            }

            Ok(true)
        })?;

        Ok(predicates)
    }

    fn into_recursion(
        self,
        recursion_counter: usize,
        predicates: &[TokenStream],
    ) -> Result<Expansion> {
        let Self {
            config,
            item,
            current_macro,
        } = self;

        let bon = NestedMeta::parse_meta_list(config.clone())?
            .iter()
            .find_map(|meta| match meta {
                NestedMeta::Meta(syn::Meta::NameValue(meta)) if meta.path.is_ident("crate") => {
                    let path = &meta.value;
                    Some(syn::Path::parse_mod_style.parse2(quote!(#path)))
                }
                _ => None,
            })
            .transpose()?
            .unwrap_or_else(|| syn::parse_quote!(::bon));

        let current_macro = syn::parse_quote!(#bon::#current_macro);

        let invocation_name = Self::unique_invocation_name(&item, &current_macro)?;

        let predicates = predicates.iter().enumerate().map(|(i, predicate)| {
            // We need to insert the recursion counter into the name so that
            // the name is unique on every recursive iteration of the cfg eval.
            let pred_id = format_ident!("{invocation_name}_{recursion_counter}_{i}");
            quote!(#pred_id: #predicate)
        });

        let expansion = quote! {
            #bon::__eval_cfg_callback! {
                {}
                #((#predicates))*
                #current_macro,
                #recursion_counter,
                ( #config )
                #item
            }
        };

        Ok(Expansion::Recurse(expansion))
    }

    /// The macro `__eval_cfg_callback` needs to generate a use statement for
    /// every `cfg` predicate. To do that it needs to assign a unique name for
    /// every `use` statement so they doesn't collide with other items in
    /// the same scope and with each other.
    ///
    /// But.. How in the world can we generate a unique name for every `use`
    /// if proc macros are supposed to be stateless and deterministic? 😳
    ///
    /// We could use a random number here, but that would make the output
    /// non-deterministic, which is not good for reproducible builds and
    /// generally may lead to some unexpected headaches 🤧.
    ///
    /// That's a silly problem, and here is a silly solution that doesn't
    /// work in 100% of the cases but it's probably good enough 😸.
    ///
    /// We just need to use some existing name as a source of uniqueness.
    /// The name of the item under the macro is a good candidate for that.
    /// If the item is a function, then we can use the function name as that
    /// reliable source of uniqueness.
    ///
    /// If the item is an `impl` block, then we have a bit of a problem because
    /// the `impl` block doesn't have a unique identifier attached to it, especially
    /// if the `self_ty` of the `impl` block isn't some simple syntax like a path.
    ///
    /// However, in most of the cases it will be a simple path, so its combination
    /// with the name of the first function in the `impl` block should be unique enough.
    fn unique_invocation_name(item: &syn::Item, current_macro: &syn::Path) -> Result<String> {
        let path_to_ident =
            |path: &syn::Path| path.segments.iter().map(|segment| &segment.ident).join("_");

        // Include the name of the proc macro in the unique name to avoid
        // collisions when different proc macros are placed on the same item
        // and they use this code to generate unique names.
        let macro_path_str = path_to_ident(current_macro);

        let item_name = match item {
            syn::Item::Fn(item) => item.sig.ident.to_string(),
            syn::Item::Impl(item) => {
                let self_ty = item
                    .self_ty
                    .as_path()
                    .map(|path| path_to_ident(&path.path))
                    .unwrap_or_default();

                let first_fn = item
                    .items
                    .iter()
                    .find_map(|item| match item {
                        syn::ImplItem::Fn(method) => Some(method.sig.ident.to_string()),
                        _ => None,
                    })
                    .unwrap_or_default();

                format!("impl_{self_ty}_fn_{first_fn}")
            }
            _ => bail!(&Span::call_site(), "Unsupported item type"),
        };

        Ok(format!("__eval_cfg_{macro_path_str}_{item_name}"))
    }
}

fn eval_cfgs(true_predicates: &BTreeSet<String>, attrs: &mut Vec<syn::Attribute>) -> Result<bool> {
    let mut cfg_attr_expansions = vec![];

    for (i, attr) in attrs.iter().enumerate() {
        let syntax = match CfgSyntax::from_meta(&attr.meta)? {
            Some(syntax) => syntax,
            _ => continue,
        };

        let expansion = match syntax {
            CfgSyntax::Cfg(predicate) => {
                if !true_predicates.contains(&predicate.to_string()) {
                    // The cfg predicate is false. No need to keep iterating
                    // because the entire syntax this attribute is attached to
                    // should be removed. Signal the caller to remove it via `false`.
                    return Ok(false);
                }

                // Just remove the attribute. It evaluated to `true`
                None
            }
            CfgSyntax::CfgAttr(cfg_attr) => {
                let predicate = cfg_attr.predicate.to_token_stream().to_string();

                // We can't both iterate over the attributes and mutate them,
                // so collect the planned actions in a separate vector, and
                // do the mutations after the iteration.

                true_predicates
                    .contains(&predicate)
                    .then(|| cfg_attr.then_branch)
            }
        };

        cfg_attr_expansions.push((i, expansion));
    }

    // It's important to iterate in reverse order to avoid index invalidation
    for (i, metas) in cfg_attr_expansions.iter().rev() {
        let metas = if let Some(metas) = metas {
            metas
        } else {
            attrs.remove(*i);
            continue;
        };

        let replacement = metas.iter().map(|meta| syn::parse_quote!(#[#meta]));

        attrs.splice(i..=i, replacement);
    }

    Ok(true)
}
