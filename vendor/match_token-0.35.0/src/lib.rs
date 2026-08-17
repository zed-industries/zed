extern crate proc_macro;

use quote::quote;
use syn::{braced, Token};

use std::collections::HashSet;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};

/// Implements the `match_token!()` macro for use by the HTML tree builder
/// in `src/tree_builder/rules.rs`.
///
/// ## Example
///
/// ```rust,ignore
/// match_token!(token {
///     CommentToken(text) => 1,
///     tag @ <base> <link> <meta> => 2,
///     </head> => 3,
///     </body> </html> </br> => else,
///     tag @ </_> => 4,
///     token => 5,
/// })
/// ```
///
/// ## Syntax
/// Because of the simplistic parser, the macro invocation must
/// start with exactly `match_token!(token {` (with whitespace as specified)
/// and end with exactly `})`.
/// The left-hand side of each match arm is an optional `name @` binding, followed by
///   - an ordinary Rust pattern that starts with an identifier or an underscore, or
///   - a sequence of HTML tag names as identifiers, each inside "<...>" or "</...>"
///     to match an open or close tag respectively, or
///   - a "wildcard tag" "<_>" or "</_>" to match all open tags or all close tags
///     respectively.
///
/// The right-hand side is either an expression or the keyword `else`.
/// Note that this syntax does not support guards or pattern alternation like
/// `Foo | Bar`.  This is not a fundamental limitation; it's done for implementation
/// simplicity.
/// ## Semantics
/// Ordinary Rust patterns match as usual.  If present, the `name @` binding has
/// the usual meaning.
/// A sequence of named tags matches any of those tags.  A single sequence can
/// contain both open and close tags.  If present, the `name @` binding binds (by
/// move) the `Tag` struct, not the outer `Token`.  That is, a match arm like
/// ```rust,ignore
/// tag @ <html> <head> => ...
/// ```
/// expands to something like
/// ```rust,ignore
/// TagToken(tag @ Tag { name: local_name!("html"), kind: StartTag })
/// | TagToken(tag @ Tag { name: local_name!("head"), kind: StartTag }) => ...
/// ```
/// A wildcard tag matches any tag of the appropriate kind, *unless* it was
/// previously matched with an `else` right-hand side (more on this below).
/// The expansion of this macro reorders code somewhat, to satisfy various
/// restrictions arising from moves.  However it provides the semantics of in-order
/// matching, by enforcing the following restrictions on its input:
///   - The last pattern must be a variable or the wildcard "_".  In other words
///     it must match everything.
///   - Otherwise, ordinary Rust patterns and specific-tag patterns cannot appear
///     after wildcard tag patterns.
///   - No tag name may appear more than once.
///   - A wildcard tag pattern may not occur in the same arm as any other tag.
///     "<_> <html> => ..." and "<_> </_> => ..." are both forbidden.
///   - The right-hand side "else" may only appear with specific-tag patterns.
///     It means that these specific tags should be handled by the last,
///     catch-all case arm, rather than by any wildcard tag arm.  This situation
///     is common in the HTML5 syntax.
#[proc_macro]
pub fn match_token(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let match_token = syn::parse2::<MatchToken>(input).expect("Parsing match_token! input failed");
    let output = expand_match_token_macro(match_token);

    proc_macro::TokenStream::from(output)
}

struct MatchToken {
    ident: syn::Ident,
    arms: Vec<MatchTokenArm>,
}

struct MatchTokenArm {
    binding: Option<syn::Ident>,
    lhs: Lhs,
    rhs: Rhs,
}

enum Lhs {
    Tags(Vec<Tag>),
    Pattern(syn::Pat),
}

enum Rhs {
    Expression(syn::Expr),
    Else,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum TagKind {
    StartTag,
    EndTag,
}

// Option is None if wildcard
#[derive(PartialEq, Eq, Hash, Clone)]
struct Tag {
    kind: TagKind,
    name: Option<syn::Ident>,
}

impl Parse for Tag {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let closing: Option<Token![/]> = input.parse()?;
        let name = match input.call(syn::Ident::parse_any)? {
            ref wildcard if wildcard == "_" => None,
            other => Some(other),
        };
        input.parse::<Token![>]>()?;
        Ok(Tag {
            kind: if closing.is_some() {
                TagKind::EndTag
            } else {
                TagKind::StartTag
            },
            name,
        })
    }
}

impl Parse for Lhs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) {
            let mut tags = Vec::new();
            while !input.peek(Token![=>]) {
                tags.push(input.parse()?);
            }
            Ok(Lhs::Tags(tags))
        } else {
            let p = input.call(syn::Pat::parse_single)?;
            Ok(Lhs::Pattern(p))
        }
    }
}

impl Parse for MatchTokenArm {
    fn parse(input: ParseStream) -> Result<Self> {
        let binding = if input.peek2(Token![@]) {
            let binding = input.parse::<syn::Ident>()?;
            input.parse::<Token![@]>()?;
            Some(binding)
        } else {
            None
        };
        let lhs = input.parse::<Lhs>()?;
        input.parse::<Token![=>]>()?;
        let rhs = if input.peek(syn::token::Brace) {
            let block = input.parse::<syn::Block>().unwrap();
            let block = syn::ExprBlock {
                attrs: vec![],
                label: None,
                block,
            };
            input.parse::<Option<Token![,]>>()?;
            Rhs::Expression(syn::Expr::Block(block))
        } else if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            input.parse::<Token![,]>()?;
            Rhs::Else
        } else {
            let expr = input.parse::<syn::Expr>().unwrap();
            input.parse::<Option<Token![,]>>()?;
            Rhs::Expression(expr)
        };

        Ok(MatchTokenArm { binding, lhs, rhs })
    }
}

impl Parse for MatchToken {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let content;
        braced!(content in input);
        let mut arms = vec![];
        while !content.is_empty() {
            arms.push(content.parse()?);
        }
        Ok(MatchToken { ident, arms })
    }
}

fn expand_match_token_macro(match_token: MatchToken) -> proc_macro2::TokenStream {
    let mut arms = match_token.arms;
    let to_be_matched = match_token.ident;
    // Handle the last arm specially at the end.
    let last_arm = arms.pop().unwrap();

    // Tags we've seen, used for detecting duplicates.
    let mut seen_tags: HashSet<Tag> = HashSet::new();

    // Case arms for wildcard matching.  We collect these and
    // emit them later.
    let mut wildcards_patterns: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut wildcards_expressions: Vec<syn::Expr> = Vec::new();

    // Tags excluded (by an 'else' RHS) from wildcard matching.
    let mut wild_excluded_patterns: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut arms_code = Vec::new();

    for MatchTokenArm { binding, lhs, rhs } in arms {
        // Build Rust syntax for the `name @` binding, if any.
        let binding = match binding {
            Some(ident) => quote!(#ident @),
            None => quote!(),
        };

        match (lhs, rhs) {
            (Lhs::Pattern(_), Rhs::Else) => {
                panic!("'else' may not appear with an ordinary pattern")
            },

            // ordinary pattern => expression
            (Lhs::Pattern(pat), Rhs::Expression(expr)) => {
                if !wildcards_patterns.is_empty() {
                    panic!("ordinary patterns may not appear after wildcard tags");
                }
                arms_code.push(quote!(#binding #pat => #expr,))
            },

            // <tag> <tag> ... => else
            (Lhs::Tags(tags), Rhs::Else) => {
                for tag in tags {
                    if !seen_tags.insert(tag.clone()) {
                        panic!("duplicate tag");
                    }
                    if tag.name.is_none() {
                        panic!("'else' may not appear with a wildcard tag");
                    }
                    wild_excluded_patterns
                        .push(make_tag_pattern(&proc_macro2::TokenStream::new(), tag));
                }
            },

            // <_> => expression
            // <tag> <tag> ... => expression
            (Lhs::Tags(tags), Rhs::Expression(expr)) => {
                // Is this arm a tag wildcard?
                // `None` if we haven't processed the first tag yet.
                let mut wildcard = None;
                for tag in tags {
                    if !seen_tags.insert(tag.clone()) {
                        panic!("duplicate tag");
                    }

                    match tag.name {
                        // <tag>
                        Some(_) => {
                            if !wildcards_patterns.is_empty() {
                                panic!("specific tags may not appear after wildcard tags");
                            }

                            if wildcard == Some(true) {
                                panic!("wildcard tags must appear alone");
                            }

                            if wildcard.is_some() {
                                // Push the delimiter `|` if it's not the first tag.
                                arms_code.push(quote!( | ))
                            }
                            arms_code.push(make_tag_pattern(&binding, tag));

                            wildcard = Some(false);
                        },

                        // <_>
                        None => {
                            if wildcard.is_some() {
                                panic!("wildcard tags must appear alone");
                            }
                            wildcard = Some(true);
                            wildcards_patterns.push(make_tag_pattern(&binding, tag));
                            wildcards_expressions.push(expr.clone());
                        },
                    }
                }

                match wildcard {
                    None => panic!("[internal macro error] tag arm with no tags"),
                    Some(false) => arms_code.push(quote!( => #expr,)),
                    Some(true) => {}, // codegen for wildcards is deferred
                }
            },
        }
    }

    // Time to process the last, catch-all arm.  We will generate something like
    //
    //     last_arm_token => {
    //         let enable_wildcards = match last_arm_token {
    //             TagToken(Tag { kind: EndTag, name: local_name!("body"), .. }) => false,
    //             TagToken(Tag { kind: EndTag, name: local_name!("html"), .. }) => false,
    //             // ...
    //             _ => true,
    //         };
    //
    //         match (enable_wildcards, last_arm_token) {
    //             (true, TagToken(name @ Tag { kind: StartTag, .. }))
    //                 => ...,  // wildcard action for start tags
    //
    //             (true, TagToken(name @ Tag { kind: EndTag, .. }))
    //                 => ...,  // wildcard action for end tags
    //
    //             (_, token) => ...  // using the pattern from that last arm
    //         }
    //     }

    let MatchTokenArm { binding, lhs, rhs } = last_arm;

    let (last_pat, last_expr) = match (binding, lhs, rhs) {
        (Some(_), _, _) => panic!("the last arm cannot have an @-binding"),
        (None, Lhs::Tags(_), _) => panic!("the last arm cannot have tag patterns"),
        (None, _, Rhs::Else) => panic!("the last arm cannot use 'else'"),
        (None, Lhs::Pattern(p), Rhs::Expression(e)) => (p, e),
    };

    quote! {
        match #to_be_matched {
            #(
                #arms_code
            )*
            last_arm_token => {
                let enable_wildcards = match last_arm_token {
                    #(
                        #wild_excluded_patterns => false,
                    )*
                    _ => true,
                };
                match (enable_wildcards, last_arm_token) {
                    #(
                        (true, #wildcards_patterns) => #wildcards_expressions,
                    )*
                    (_, #last_pat) => #last_expr,
                }
            }
        }
    }
}

fn make_tag_pattern(binding: &proc_macro2::TokenStream, tag: Tag) -> proc_macro2::TokenStream {
    let kind = match tag.kind {
        TagKind::StartTag => quote!(crate::tokenizer::StartTag),
        TagKind::EndTag => quote!(crate::tokenizer::EndTag),
    };
    let name_field = if let Some(name) = tag.name {
        let name = name.to_string();
        quote!(name: local_name!(#name),)
    } else {
        quote!()
    };
    quote! {
        crate::tree_builder::types::Token::Tag(#binding crate::tokenizer::Tag { kind: #kind, #name_field .. })
    }
}
