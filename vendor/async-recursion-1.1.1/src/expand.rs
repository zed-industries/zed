use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, visit_mut::VisitMut, Block, Lifetime, Receiver,
    ReturnType, Signature, TypeReference, WhereClause,
};

use crate::parse::{AsyncItem, RecursionArgs};

impl ToTokens for AsyncItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub fn expand(item: &mut AsyncItem, args: &RecursionArgs) {
    item.0.attrs.push(parse_quote!(#[must_use]));
    transform_sig(&mut item.0.sig, args);
    transform_block(&mut item.0.block);
}

fn transform_block(block: &mut Block) {
    let brace = block.brace_token;
    *block = parse_quote!({
        Box::pin(async move #block)
    });
    block.brace_token = brace;
}

enum ArgLifetime {
    New(Lifetime),
    Existing(Lifetime),
}

impl ArgLifetime {
    pub fn lifetime(self) -> Lifetime {
        match self {
            ArgLifetime::New(lt) | ArgLifetime::Existing(lt) => lt,
        }
    }
}

#[derive(Default)]
struct ReferenceVisitor {
    counter: usize,
    lifetimes: Vec<ArgLifetime>,
    self_receiver: bool,
    self_receiver_new_lifetime: bool,
    self_lifetime: Option<Lifetime>,
}

impl VisitMut for ReferenceVisitor {
    fn visit_receiver_mut(&mut self, receiver: &mut Receiver) {
        self.self_lifetime = Some(if let Some((_, lt)) = &mut receiver.reference {
            self.self_receiver = true;

            if let Some(lt) = lt {
                lt.clone()
            } else {
                // Use 'life_self to avoid collisions with 'life<count> lifetimes.
                let new_lifetime: Lifetime = parse_quote!('life_self);
                lt.replace(new_lifetime.clone());

                self.self_receiver_new_lifetime = true;

                new_lifetime
            }
        } else {
            return;
        });
    }

    fn visit_type_reference_mut(&mut self, argument: &mut TypeReference) {
        if argument.lifetime.is_none() {
            // If this reference doesn't have a lifetime (e.g. &T), then give it one.
            let lt = Lifetime::new(&format!("'life{}", self.counter), Span::call_site());
            self.lifetimes.push(ArgLifetime::New(parse_quote!(#lt)));
            argument.lifetime = Some(lt);
            self.counter += 1;
        } else {
            // If it does (e.g. &'life T), then keep track of it.
            let lt = argument.lifetime.as_ref().cloned().unwrap();

            // Check that this lifetime isn't already in our vector
            let ident_matches = |x: &ArgLifetime| {
                if let ArgLifetime::Existing(elt) = x {
                    elt.ident == lt.ident
                } else {
                    false
                }
            };

            if !self.lifetimes.iter().any(ident_matches) {
                self.lifetimes.push(ArgLifetime::Existing(lt));
            }
        }
    }
}

// Input:
//     async fn f<S, T>(x : S, y : &T) -> Ret;
//
// Output:
//     fn f<S, T>(x : S, y : &T) -> Pin<Box<dyn Future<Output = Ret> + Send>
fn transform_sig(sig: &mut Signature, args: &RecursionArgs) {
    // Determine the original return type
    let ret = match &sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ret) => quote!(#ret),
    };

    // Remove the asyncness of this function
    sig.asyncness = None;

    // Find and update any references in the input arguments
    let mut v = ReferenceVisitor::default();
    for input in &mut sig.inputs {
        v.visit_fn_arg_mut(input);
    }

    // Does this expansion require `async_recursion to be added to the output?
    let mut requires_lifetime = false;
    let mut where_clause_lifetimes = vec![];
    let mut where_clause_generics = vec![];

    // 'async_recursion lifetime
    let asr: Lifetime = parse_quote!('async_recursion);

    // Add an S : 'async_recursion bound to any generic parameter
    for param in sig.generics.type_params() {
        let ident = param.ident.clone();
        where_clause_generics.push(ident);
        requires_lifetime = true;
    }

    // Add an 'a : 'async_recursion bound to any lifetimes 'a appearing in the function
    if !v.lifetimes.is_empty() {
        requires_lifetime = true;
        for alt in v.lifetimes {
            if let ArgLifetime::New(lt) = &alt {
                // If this is a new argument,
                sig.generics.params.push(parse_quote!(#lt));
            }

            // Add a bound to the where clause
            let lt = alt.lifetime();
            where_clause_lifetimes.push(lt);
        }
    }

    // If our function accepts &self, then we modify this to the explicit lifetime &'life_self,
    // and add the bound &'life_self : 'async_recursion
    if v.self_receiver {
        if v.self_receiver_new_lifetime {
            sig.generics.params.push(parse_quote!('life_self));
        }
        where_clause_lifetimes.extend(v.self_lifetime);
        requires_lifetime = true;
    }

    let box_lifetime: TokenStream = if requires_lifetime {
        // Add 'async_recursion to our generic parameters
        sig.generics.params.push(parse_quote!('async_recursion));

        quote!(+ #asr)
    } else {
        quote!()
    };

    let send_bound: TokenStream = if args.send_bound {
        quote!(+ ::core::marker::Send)
    } else {
        quote!()
    };

    let sync_bound: TokenStream = if args.sync_bound {
        quote!(+ ::core::marker::Sync)
    } else {
        quote!()
    };

    let where_clause = sig
        .generics
        .where_clause
        .get_or_insert_with(|| WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::new(),
        });

    // Add our S : 'async_recursion bounds
    for generic_ident in where_clause_generics {
        where_clause
            .predicates
            .push(parse_quote!(#generic_ident : #asr));
    }

    // Add our 'a : 'async_recursion bounds
    for lifetime in where_clause_lifetimes {
        where_clause.predicates.push(parse_quote!(#lifetime : #asr));
    }

    // Modify the return type
    sig.output = parse_quote! {
        -> ::core::pin::Pin<Box<
            dyn ::core::future::Future<Output = #ret> #box_lifetime #send_bound #sync_bound>>
    };
}
