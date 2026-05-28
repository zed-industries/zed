//! Implements `#[derive_inspector_reflection]` macro to provide runtime access to trait methods
//! that have the shape `fn method(self) -> Self`. This code was generated using Zed Agent with Claude Opus 4.

use heck::ToSnakeCase as _;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    Attribute, Expr, FnArg, Ident, Item, ItemTrait, Lit, Meta, Path, ReturnType, TraitItem, Type,
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
};

pub fn derive_inspector_reflection(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);

    // First, expand any macros in the trait
    match &mut item {
        Item::Trait(trait_item) => {
            let mut expander = MacroExpander;
            expander.visit_item_trait_mut(trait_item);
        }
        _ => {
            return syn::Error::new_spanned(
                quote!(#item),
                "#[derive_inspector_reflection] can only be applied to traits",
            )
            .to_compile_error()
            .into();
        }
    }

    // Now process the expanded trait
    match item {
        Item::Trait(trait_item) => generate_reflected_trait(trait_item),
        _ => unreachable!(),
    }
}

fn generate_reflected_trait(trait_item: ItemTrait) -> TokenStream {
    let trait_name = &trait_item.ident;
    let vis = &trait_item.vis;

    // Determine if we're being called from within the gpui crate
    let call_site = Span::call_site();
    let inspector_reflection_path = if is_called_from_gpui_crate(call_site) {
        quote! { crate::inspector_reflection }
    } else {
        quote! { ::gpui::inspector_reflection }
    };

    // Collect method information for methods of form fn name(self) -> Self or fn name(mut self) -> Self
    let mut method_infos = Vec::new();

    for item in &trait_item.items {
        if let TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;

            // Check if method has self or mut self receiver
            let has_valid_self_receiver = method
                .sig
                .inputs
                .iter()
                .any(|arg| matches!(arg, FnArg::Receiver(r) if r.reference.is_none()));

            // Check if method returns Self
            let returns_self = match &method.sig.output {
                ReturnType::Type(_, ty) => {
                    matches!(**ty, Type::Path(ref path) if path.path.is_ident("Self"))
                }
                ReturnType::Default => false,
            };

            // Check if method has exactly one parameter (self or mut self)
            let param_count = method.sig.inputs.len();

            // Include methods of form fn name(self) -> Self or fn name(mut self) -> Self
            // This includes methods with default implementations
            if has_valid_self_receiver && returns_self && param_count == 1 {
                // Extract documentation and cfg attributes
                let doc = extract_doc_comment(&method.attrs);
                let cfg_attrs = extract_cfg_attributes(&method.attrs);
                method_infos.push((method_name.clone(), doc, cfg_attrs));
            }
        }
    }

    // Generate the reflection module name
    let reflection_mod_name = Ident::new(
        &format!("{}_reflection", trait_name.to_string().to_snake_case()),
        trait_name.span(),
    );

    // Generate wrapper functions for each method
    // These wrappers use type erasure to allow runtime invocation
    let wrapper_functions = method_infos.iter().map(|(method_name, _doc, cfg_attrs)| {
        let wrapper_name = Ident::new(
            &format!("__wrapper_{}", method_name),
            method_name.span(),
        );
        quote! {
            #(#cfg_attrs)*
            fn #wrapper_name<T: #trait_name + 'static>(value: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
                if let Ok(concrete) = value.downcast::<T>() {
                    Box::new(concrete.#method_name())
                } else {
                    panic!("Type mismatch in reflection wrapper");
                }
            }
        }
    });

    // Generate method info entries
    let method_info_entries = method_infos.iter().map(|(method_name, doc, cfg_attrs)| {
        let method_name_str = method_name.to_string();
        let wrapper_name = Ident::new(&format!("__wrapper_{}", method_name), method_name.span());
        let doc_expr = match doc {
            Some(doc_str) => quote! { Some(#doc_str) },
            None => quote! { None },
        };
        quote! {
            #(#cfg_attrs)*
            #inspector_reflection_path::FunctionReflection {
                name: #method_name_str,
                function: #wrapper_name::<T>,
                documentation: #doc_expr,
                _type: ::std::marker::PhantomData,
            }
        }
    });

    // Generate the complete output
    let output = quote! {
        #trait_item

        /// Implements function reflection
        #vis mod #reflection_mod_name {
            use super::*;

            #(#wrapper_functions)*

            /// Get all reflectable methods for a concrete type implementing the trait
            pub fn methods<T: #trait_name + 'static>() -> Vec<#inspector_reflection_path::FunctionReflection<T>> {
                vec![
                    #(#method_info_entries),*
                ]
            }

            /// Find a method by name for a concrete type implementing the trait
            pub fn find_method<T: #trait_name + 'static>(name: &str) -> Option<#inspector_reflection_path::FunctionReflection<T>> {
                methods::<T>().into_iter().find(|m| m.name == name)
            }
        }
    };

    TokenStream::from(output)
}

fn extract_doc_comment(attrs: &[Attribute]) -> Option<String> {
    let mut doc_lines = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc")
            && let Meta::NameValue(meta) = &attr.meta
            && let Expr::Lit(expr_lit) = &meta.value
            && let Lit::Str(lit_str) = &expr_lit.lit
        {
            let line = lit_str.value();
            let line = line.strip_prefix(' ').unwrap_or(&line);
            doc_lines.push(line.to_string());
        }
    }

    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join("\n"))
    }
}

fn extract_cfg_attributes(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg"))
        .cloned()
        .collect()
}

fn is_called_from_gpui_crate(_span: Span) -> bool {
    // Check if we're being called from within the gpui crate by examining the call site
    // This is a heuristic approach - we check if the current crate name is "gpui"
    std::env::var("CARGO_PKG_NAME").is_ok_and(|name| name == "gpui")
}

struct MacroExpander;

impl VisitMut for MacroExpander {
    fn visit_item_trait_mut(&mut self, trait_item: &mut ItemTrait) {
        let mut expanded_items = Vec::new();
        let mut items_to_keep = Vec::new();

        for item in trait_item.items.drain(..) {
            match item {
                TraitItem::Macro(macro_item) => {
                    // Try to expand known macros
                    if let Some(expanded) = try_expand_macro(&macro_item) {
                        expanded_items.extend(expanded);
                    } else {
                        // Keep unknown macros as-is
                        items_to_keep.push(TraitItem::Macro(macro_item));
                    }
                }
                other => {
                    items_to_keep.push(other);
                }
            }
        }

        // Rebuild the items list with expanded content first, then original items
        trait_item.items = expanded_items;
        trait_item.items.extend(items_to_keep);

        // Continue visiting
        visit_mut::visit_item_trait_mut(self, trait_item);
    }
}

fn try_expand_macro(macro_item: &syn::TraitItemMacro) -> Option<Vec<TraitItem>> {
    let path = &macro_item.mac.path;

    // Check if this is one of our known style macros
    let macro_name = path_to_string(path);

    // Handle the known macros by calling their implementations
    match macro_name.as_str() {
        "gpui_macros::style_helpers" | "style_helpers" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::style_helpers(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::visibility_style_methods" | "visibility_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::visibility_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::margin_style_methods" | "margin_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::margin_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::padding_style_methods" | "padding_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::padding_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::position_style_methods" | "position_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::position_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::overflow_style_methods" | "overflow_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::overflow_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::cursor_style_methods" | "cursor_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::cursor_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::border_style_methods" | "border_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::border_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        "gpui_macros::box_shadow_style_methods" | "box_shadow_style_methods" => {
            let tokens = macro_item.mac.tokens.clone();
            let expanded = crate::styles::box_shadow_style_methods(TokenStream::from(tokens));
            parse_expanded_items(expanded)
        }
        _ => None,
    }
}

fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn parse_expanded_items(expanded: TokenStream) -> Option<Vec<TraitItem>> {
    let tokens = TokenStream2::from(expanded);

    // Try to parse the expanded tokens as trait items
    // We need to wrap them in a dummy trait to parse properly
    let dummy_trait: ItemTrait = parse_quote! {
        trait Dummy {
            #tokens
        }
    };

    Some(dummy_trait.items)
}
