// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types and helpers to parse the input of the derive macro.

use syn::{Attribute, DeriveInput, Expr, ExprLit, Generics, Ident, Lit, Meta};

#[derive(Debug, PartialEq)]
pub(crate) enum GrammarSource {
    File(String),
    Inline(String),
}

/// Parsed information of the derive and the attributes.
pub struct ParsedDerive {
    /// The identifier of the deriving struct, union, or enum.
    pub name: Ident,
    /// The generics of the deriving struct, union, or enum.
    pub generics: Generics,
    /// Indicates whether the 'non_exhaustive' attribute is added to the 'Rule' enum.
    pub non_exhaustive: bool,
}

pub(crate) fn parse_derive(ast: DeriveInput) -> (ParsedDerive, Vec<GrammarSource>) {
    let name = ast.ident;
    let generics = ast.generics;

    let grammar: Vec<&Attribute> = ast
        .attrs
        .iter()
        .filter(|attr| {
            let path = attr.meta.path();
            path.is_ident("grammar") || path.is_ident("grammar_inline")
        })
        .collect();

    if grammar.is_empty() {
        panic!("a grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute");
    }

    let mut grammar_sources = Vec::with_capacity(grammar.len());
    for attr in grammar {
        grammar_sources.push(get_attribute(attr))
    }

    let non_exhaustive = ast
        .attrs
        .iter()
        .any(|attr| attr.meta.path().is_ident("non_exhaustive"));

    (
        ParsedDerive {
            name,
            generics,
            non_exhaustive,
        },
        grammar_sources,
    )
}

fn get_attribute(attr: &Attribute) -> GrammarSource {
    match &attr.meta {
        Meta::NameValue(name_value) => match &name_value.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(string),
                ..
            }) => {
                if name_value.path.is_ident("grammar") {
                    GrammarSource::File(string.value())
                } else {
                    GrammarSource::Inline(string.value())
                }
            }
            _ => panic!("grammar attribute must be a string"),
        },
        _ => panic!("grammar attribute must be of the form `grammar = \"...\"`"),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_derive;
    use super::GrammarSource;

    #[test]
    fn derive_inline_file() {
        let definition = "
            #[other_attr]
            #[grammar_inline = \"GRAMMAR\"]
            pub struct MyParser<'a, T>;
        ";
        let ast = syn::parse_str(definition).unwrap();
        let (_, filenames) = parse_derive(ast);
        assert_eq!(filenames, [GrammarSource::Inline("GRAMMAR".to_string())]);
    }

    #[test]
    fn derive_ok() {
        let definition = "
            #[other_attr]
            #[grammar = \"myfile.pest\"]
            pub struct MyParser<'a, T>;
        ";
        let ast = syn::parse_str(definition).unwrap();
        let (parsed_derive, filenames) = parse_derive(ast);
        assert_eq!(filenames, [GrammarSource::File("myfile.pest".to_string())]);
        assert!(!parsed_derive.non_exhaustive);
    }

    #[test]
    fn derive_multiple_grammars() {
        let definition = "
            #[other_attr]
            #[grammar = \"myfile1.pest\"]
            #[grammar = \"myfile2.pest\"]
            pub struct MyParser<'a, T>;
        ";
        let ast = syn::parse_str(definition).unwrap();
        let (_, filenames) = parse_derive(ast);
        assert_eq!(
            filenames,
            [
                GrammarSource::File("myfile1.pest".to_string()),
                GrammarSource::File("myfile2.pest".to_string())
            ]
        );
    }

    #[test]
    fn derive_nonexhaustive() {
        let definition = "
            #[non_exhaustive]
            #[grammar = \"myfile.pest\"]
            pub struct MyParser<'a, T>;
        ";
        let ast = syn::parse_str(definition).unwrap();
        let (parsed_derive, filenames) = parse_derive(ast);
        assert_eq!(filenames, [GrammarSource::File("myfile.pest".to_string())]);
        assert!(parsed_derive.non_exhaustive);
    }

    #[test]
    #[should_panic(expected = "grammar attribute must be a string")]
    fn derive_wrong_arg() {
        let definition = "
            #[other_attr]
            #[grammar = 1]
            pub struct MyParser<'a, T>;
        ";
        let ast = syn::parse_str(definition).unwrap();
        parse_derive(ast);
    }

    #[test]
    #[should_panic(
        expected = "a grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute"
    )]
    fn derive_no_grammar() {
        let definition = "
            #[other_attr]
            pub struct MyParser<'a, T>;
        ";
        let ast = syn::parse_str(definition).unwrap();
        parse_derive(ast);
    }
}
