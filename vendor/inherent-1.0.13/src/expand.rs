use crate::parse::TraitImpl;
use crate::verbatim::VerbatimFn;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Attribute, FnArg, Ident, ImplItem, Pat, Path, Signature, Visibility};

pub fn inherent(mut input: TraitImpl) -> TokenStream {
    let impl_token = &input.impl_token;
    let generics = &input.generics;
    let where_clause = &input.generics.where_clause;
    let trait_ = &input.trait_;
    let ty = &input.self_ty;

    let fwd_methods: Vec<_> = input
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Fn(method) => Some(fwd_method(
                trait_,
                &method.attrs,
                &method.vis,
                &method.sig,
                method.block.brace_token.span.join(),
            )),
            ImplItem::Verbatim(tokens) => {
                if let Ok(method) = syn::parse2::<VerbatimFn>(tokens.clone()) {
                    Some(fwd_method(
                        trait_,
                        &method.attrs,
                        &method.vis,
                        &method.sig,
                        method.semi_token.span,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    input.items = input
        .items
        .into_iter()
        .filter_map(|item| match item {
            ImplItem::Fn(mut method) => {
                method.vis = Visibility::Inherited;
                Some(ImplItem::Fn(method))
            }
            ImplItem::Verbatim(tokens) => {
                if syn::parse2::<VerbatimFn>(tokens.clone()).is_ok() {
                    None
                } else {
                    Some(ImplItem::Verbatim(tokens))
                }
            }
            item => Some(item),
        })
        .collect();

    let body = quote_spanned!(input.brace_token.span=> { #(#fwd_methods)* });

    quote! {
        #impl_token #generics #ty #where_clause #body

        #input
    }
}

fn fwd_method(
    trait_: &Path,
    attrs: &[Attribute],
    vis: &Visibility,
    sig: &Signature,
    body_span: Span,
) -> TokenStream {
    let constness = &sig.constness;
    let asyncness = &sig.asyncness;
    let unsafety = &sig.unsafety;
    let abi = &sig.abi;
    let fn_token = sig.fn_token;
    let ident = &sig.ident;
    let generics = &sig.generics;
    let output = &sig.output;
    let where_clause = &sig.generics.where_clause;

    let (arg_pat, arg_val): (Vec<_>, Vec<_>) = sig
        .inputs
        .pairs()
        .enumerate()
        .map(|(i, pair)| {
            let (input, comma_token) = pair.into_tuple();
            match input {
                FnArg::Receiver(receiver) => {
                    let self_token = receiver.self_token;
                    if receiver.reference.is_some() {
                        (quote!(#receiver #comma_token), quote!(#self_token))
                    } else {
                        (quote!(#self_token #comma_token), quote!(#self_token))
                    }
                }
                FnArg::Typed(arg) => {
                    let var = match arg.pat.as_ref() {
                        Pat::Ident(pat) => pat.ident.clone(),
                        _ => Ident::new(&format!("__arg{}", i), Span::call_site()),
                    };
                    let colon_token = arg.colon_token;
                    let ty = &arg.ty;
                    (quote!(#var #colon_token #ty #comma_token), quote!(#var))
                }
            }
        })
        .unzip();

    let types = generics.type_params().map(|param| &param.ident);
    let wait = asyncness.map(|_| quote!(.await));
    let body = quote!(<Self as #trait_>::#ident::<#(#types,)*>(#(#arg_val,)*) #wait);
    let block = quote_spanned!(body_span=> { #body });
    let args = quote_spanned!(sig.paren_token.span=> (#(#arg_pat)*));

    let has_doc = attrs.iter().any(|attr| attr.path().is_ident("doc"));
    let default_doc = if has_doc {
        None
    } else {
        let mut link = String::new();
        for segment in &trait_.segments {
            link += &segment.ident.to_string();
            link += "::";
        }
        let msg = format!("See [`{}{}`]", link, ident);
        Some(quote!(#[doc = #msg]))
    };

    quote! {
        #(#attrs)*
        #default_doc
        #vis #constness #asyncness #unsafety #abi #fn_token #ident #generics #args #output #where_clause #block
    }
}
