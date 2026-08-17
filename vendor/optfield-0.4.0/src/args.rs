use proc_macro2::{Group, Span};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::token::{Comma, Eq, Pub};
use syn::{parse2, Ident, LitStr, Meta, Visibility};

mod kw {
    // NOTE: when adding new keywords update ArgList::next_is_kw
    syn::custom_keyword!(doc);
    syn::custom_keyword!(merge_fn);
    syn::custom_keyword!(rewrap);
    syn::custom_keyword!(attrs);
    syn::custom_keyword!(field_doc);
    syn::custom_keyword!(field_attrs);
    syn::custom_keyword!(from);

    pub mod attrs_sub {
        syn::custom_keyword!(add);
    }
}

#[cfg_attr(test, derive(PartialEq))]
pub struct Args {
    pub item: GenItem,
    pub merge: Option<MergeFn>,
    pub rewrap: bool,
    pub doc: Option<Doc>,
    pub attrs: Option<Attrs>,
    pub field_doc: bool,
    pub field_attrs: Option<Attrs>,
    pub from: bool,
}

enum Arg {
    Merge(MergeFn),
    Doc(Doc),
    Rewrap(bool),
    Attrs(Attrs),
    FieldDocs(bool),
    FieldAttrs(Attrs),
    From(bool),
}

#[cfg_attr(test, derive(PartialEq))]
pub struct GenItem {
    pub name: Ident,
    pub visibility: Option<Visibility>,
}

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub struct MergeFn {
    pub visibility: Visibility,
    pub name: MergeFnName,
}

#[cfg_attr(test, derive(Clone, Debug, PartialEq))]
pub enum MergeFnName {
    Default,
    Custom(Ident),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Doc {
    Same,
    Custom(String),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Attrs {
    /// Keep same attributes.
    Keep,
    /// Replace with given attributes.
    Replace(Vec<Meta>),
    /// Keep original attributes and add the given ones.
    Add(Vec<Meta>),
}

#[derive(Debug)]
pub struct AttrList(Vec<Meta>);

/// Parser for unordered args.
struct ArgList {
    item: GenItem,
    merge: Option<Span>,
    doc: Option<Span>,
    rewrap: Option<Span>,
    attrs: Option<Span>,
    field_doc: Option<Span>,
    field_attrs: Option<Span>,
    from: Option<Span>,
    list: Vec<Arg>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg_list: ArgList = input.parse()?;

        Ok(arg_list.into())
    }
}

impl Parse for ArgList {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(input.error("expected opt struct name"));
        }

        if ArgList::next_is_kw(input) {
            return Err(input.error("first argument must be opt struct name"));
        }

        let item = input.parse()?;
        let mut arg_list = ArgList::new(item);

        while !input.is_empty() {
            input.parse::<Comma>()?;

            // allow trailing commas
            if input.is_empty() {
                break;
            }

            let lookahead = input.lookahead1();

            if lookahead.peek(kw::doc) {
                arg_list.parse_doc(input)?;
            } else if lookahead.peek(kw::merge_fn) {
                arg_list.parse_merge(input)?;
            } else if lookahead.peek(kw::rewrap) {
                arg_list.parse_rewrap(input)?;
            } else if lookahead.peek(kw::attrs) {
                arg_list.parse_attrs(input)?;
            } else if lookahead.peek(kw::field_doc) {
                arg_list.parse_field_doc(input)?;
            } else if lookahead.peek(kw::field_attrs) {
                arg_list.parse_field_attrs(input)?;
            } else if lookahead.peek(kw::from) {
                arg_list.parse_from(input)?;
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(arg_list)
    }
}

impl Args {
    fn new(item: GenItem) -> Self {
        Self {
            item,
            merge: None,
            rewrap: false,
            doc: None,
            attrs: None,
            field_doc: false,
            field_attrs: None,
            from: false,
        }
    }
}

impl ArgList {
    fn new(item: GenItem) -> Self {
        Self {
            item,
            merge: None,
            doc: None,
            rewrap: None,
            attrs: None,
            field_doc: None,
            field_attrs: None,
            from: None,
            list: Vec::with_capacity(6),
        }
    }

    fn next_is_kw(input: ParseStream) -> bool {
        input.peek(kw::doc)
            || input.peek(kw::merge_fn)
            || input.peek(kw::rewrap)
            || input.peek(kw::field_doc)
            || input.peek(kw::field_attrs)
            || input.peek(kw::attrs)
            || input.peek(kw::from)
    }

    fn parse_doc(&mut self, input: ParseStream) -> Result<()> {
        if let Some(doc_span) = self.doc {
            return ArgList::already_defined_error(input, "doc", doc_span);
        }

        let span = input.span();
        let doc: Doc = input.parse()?;

        self.doc = Some(span);
        self.list.push(Arg::Doc(doc));

        Ok(())
    }

    fn parse_merge(&mut self, input: ParseStream) -> Result<()> {
        if let Some(merge_span) = self.merge {
            return ArgList::already_defined_error(input, "merge_fn", merge_span);
        }

        let span = input.span();
        let merge: MergeFn = input.parse()?;

        self.merge = Some(span);
        self.list.push(Arg::Merge(merge));

        Ok(())
    }

    fn parse_rewrap(&mut self, input: ParseStream) -> Result<()> {
        if let Some(rewrap_span) = self.rewrap {
            return ArgList::already_defined_error(input, "rewrap", rewrap_span);
        }

        let span = input.span();
        input.parse::<kw::rewrap>()?;

        self.rewrap = Some(span);
        self.list.push(Arg::Rewrap(true));

        Ok(())
    }

    fn parse_attrs(&mut self, input: ParseStream) -> Result<()> {
        if let Some(attrs_span) = self.attrs {
            return ArgList::already_defined_error(input, "attrs", attrs_span);
        }

        let span = input.span();

        input.parse::<kw::attrs>()?;
        let attrs: Attrs = input.parse()?;

        self.attrs = Some(span);
        self.list.push(Arg::Attrs(attrs));

        Ok(())
    }

    fn parse_field_doc(&mut self, input: ParseStream) -> Result<()> {
        if let Some(field_doc_span) = self.field_doc {
            return ArgList::already_defined_error(input, "field_doc", field_doc_span);
        }

        let span = input.span();
        input.parse::<kw::field_doc>()?;

        self.field_doc = Some(span);
        self.list.push(Arg::FieldDocs(true));

        Ok(())
    }

    fn parse_field_attrs(&mut self, input: ParseStream) -> Result<()> {
        if let Some(field_attrs_apan) = self.field_attrs {
            return ArgList::already_defined_error(input, "field_attrs", field_attrs_apan);
        }

        let span = input.span();

        input.parse::<kw::field_attrs>()?;
        let field_attrs: Attrs = input.parse()?;

        self.field_attrs = Some(span);
        self.list.push(Arg::FieldAttrs(field_attrs));

        Ok(())
    }

    fn parse_from(&mut self, input: ParseStream) -> Result<()> {
        if let Some(from_span) = self.from {
            return ArgList::already_defined_error(input, "from", from_span);
        }

        let span = input.span();
        input.parse::<kw::from>()?;

        self.from = Some(span);
        self.list.push(Arg::From(true));

        Ok(())
    }

    fn already_defined_error(
        input: ParseStream,
        arg_name: &'static str,
        prev_span: Span,
    ) -> Result<()> {
        let mut e = input.error(format!("{} already defined", arg_name));
        e.combine(Error::new(prev_span, format!("{} defined here", arg_name)));
        Err(e)
    }
}

impl GenItem {
    pub fn final_visibility(&self) -> Visibility {
        match &self.visibility {
            None => Visibility::Inherited,
            Some(v) => v.clone(),
        }
    }
}

impl Parse for GenItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = if input.peek(Pub) {
            Some(input.parse()?)
        } else {
            None
        };

        let name = input.parse()?;

        Ok(Self { name, visibility })
    }
}

impl Parse for Doc {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::doc>()?;

        if input.peek(Eq) {
            input.parse::<Eq>()?;

            let doc_text: LitStr = input.parse()?;

            Ok(Doc::Custom(doc_text.value()))
        } else {
            Ok(Doc::Same)
        }
    }
}

impl Parse for MergeFn {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::merge_fn>()?;

        if input.peek(Eq) {
            input.parse::<Eq>()?;

            let visibility = if input.peek(Pub) {
                input.parse()?
            } else {
                Visibility::Inherited
            };

            let name = if input.peek(Ident) {
                MergeFnName::Custom(input.parse()?)
            } else {
                MergeFnName::Default
            };

            Ok(MergeFn { visibility, name })
        } else {
            Ok(MergeFn::default())
        }
    }
}

impl Default for MergeFn {
    fn default() -> MergeFn {
        MergeFn {
            visibility: Visibility::Inherited,
            name: MergeFnName::Default,
        }
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        use Attrs::*;

        if input.peek(Eq) {
            input.parse::<Eq>()?;

            if input.peek(kw::attrs_sub::add) {
                input.parse::<kw::attrs_sub::add>()?;

                Ok(Add(Attrs::parse_attr_list(input)?))
            } else {
                Ok(Replace(Attrs::parse_attr_list(input)?))
            }
        } else {
            Ok(Keep)
        }
    }
}

impl Attrs {
    fn parse_attr_list(input: ParseStream) -> Result<Vec<Meta>> {
        let group: Group = input.parse()?;

        let attrs: AttrList = parse2(group.stream())?;

        Ok(attrs.0)
    }
}

impl Parse for AttrList {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Vec::new();

        while !input.is_empty() {
            attrs.push(input.parse()?);

            if input.peek(Comma) {
                input.parse::<Comma>()?;
            }
        }

        Ok(Self(attrs))
    }
}

impl From<ArgList> for Args {
    fn from(arg_list: ArgList) -> Args {
        use Arg::*;

        let mut args = Args::new(arg_list.item);

        for arg in arg_list.list {
            match arg {
                Merge(merge) => args.merge = Some(merge),
                Doc(doc) => args.doc = Some(doc),
                Rewrap(rewrap) => args.rewrap = rewrap,
                Attrs(attrs) => args.attrs = Some(attrs),
                FieldDocs(field_doc) => args.field_doc = field_doc,
                FieldAttrs(field_attrs) => args.field_attrs = Some(field_attrs),
                From(from) => args.from = from,
            }
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse::Parser;

    use crate::test_util::*;

    macro_rules! duplicate_arg_panics_test {
        ($attr:meta, $expected:literal) => {
            duplicate_arg_panics_test!($attr, $attr, $expected);
        };

        ($attr:meta, $dup:meta, $expected:literal) => {
            paste::item! {
                #[test]
                #[should_panic(expected = $expected)]
                fn [<duplicate_ $attr _panics>]() {
                    parse_args(quote! {
                        Opt,
                        $attr,
                        $dup
                    });
                }
            }
        };
    }

    duplicate_arg_panics_test!(doc, doc = "custom", "doc already defined");
    duplicate_arg_panics_test!(merge_fn, "merge_fn already defined");
    duplicate_arg_panics_test!(rewrap, "rewrap already defined");
    duplicate_arg_panics_test!(attrs, "attrs already defined");
    duplicate_arg_panics_test!(field_doc, "field_doc already defined");
    duplicate_arg_panics_test!(field_attrs, "field_attrs already defined");
    duplicate_arg_panics_test!(from, "from already defined");

    macro_rules! struct_name_not_first_panics {
        ($attr:meta) => {
            paste::item! {
                #[test]
                #[should_panic(expected = "first argument must be opt struct name")]
                fn [<$attr _first_panics>]() {
                    parse_args(quote! {
                        $attr,
                        Opt
                    });
                }
            }
        };
    }

    struct_name_not_first_panics!(doc);
    struct_name_not_first_panics!(merge_fn);
    struct_name_not_first_panics!(rewrap);
    struct_name_not_first_panics!(attrs);
    struct_name_not_first_panics!(field_doc);
    struct_name_not_first_panics!(field_attrs);
    struct_name_not_first_panics!(from);

    #[test]
    #[should_panic(expected = "expected opt struct name")]
    fn empty_args_panics() {
        parse_args(TokenStream::new());
    }

    #[test]
    fn parse_name() {
        let cases = vec![
            quote! {
                OptionalFields
            },
            quote! {
                pub OptionalFields
            },
            quote! {
                pub(crate) OptionalFields
            },
            quote! {
                pub(in test::path) OptionalFields
            },
        ];

        for case in cases {
            let args = parse_args(case);

            assert_eq!(args.item.name, "OptionalFields");
        }
    }

    #[test]
    fn parse_no_optional_args() {
        let args = parse_args(quote! {
            Opt
        });

        assert_eq!(args.item.visibility, None);
        assert_eq!(args.item.final_visibility(), Visibility::Inherited);
        assert_eq!(args.merge, None);
        assert_eq!(args.rewrap, false);
        assert_eq!(args.doc, None);
        assert_eq!(args.attrs, None);
        assert_eq!(args.field_doc, false);
        assert_eq!(args.field_attrs, None);
        assert_eq!(args.from, false);
    }

    #[test]
    fn parse_visibility() {
        let cases = vec![
            (quote! {pub Opt}, quote!(pub)),
            (quote! {pub(crate) Opt}, quote!(pub(crate))),
            (quote! {pub(in test::path) Opt}, quote!(pub(in test::path))),
        ];

        for (args_tokens, vis_tokens) in cases {
            let args: Args = syn::parse2(args_tokens).unwrap();
            let vis: Visibility = syn::parse2(vis_tokens).unwrap();

            assert_eq!(args.item.visibility.as_ref(), Some(&vis));
            assert_eq!(args.item.final_visibility(), vis);
        }
    }

    #[test]
    fn parse_merge_fn() {
        let custom_fn_name = MergeFnName::Custom(syn::parse2(quote!(custom_fn)).unwrap());

        let cases = vec![
            (
                quote! {Opt, merge_fn},
                MergeFnName::Default,
                Visibility::Inherited,
            ),
            (
                quote! {Opt, merge_fn = custom_fn},
                custom_fn_name.clone(),
                Visibility::Inherited,
            ),
            (
                quote! {Opt, merge_fn = pub custom_fn},
                custom_fn_name.clone(),
                syn::parse2(quote!(pub)).unwrap(),
            ),
            (
                quote! {Opt, merge_fn = pub(crate) custom_fn},
                custom_fn_name.clone(),
                syn::parse2(quote!(pub(crate))).unwrap(),
            ),
            (
                quote! {Opt, merge_fn = pub(in test::path) custom_fn},
                custom_fn_name,
                syn::parse2(quote!(pub(in test::path))).unwrap(),
            ),
        ];

        for (args_tokens, fn_name, vis) in cases {
            let args = parse_args(args_tokens);

            assert_eq!(args.merge.clone().unwrap().name, fn_name);
            assert_eq!(args.merge.unwrap().visibility, vis);
        }
    }

    #[test]
    fn parse_rewrap() {
        let args = parse_args(quote! {
            Opt,
            rewrap
        });

        assert!(args.rewrap);
    }

    #[test]
    fn parse_doc() {
        let cases = vec![
            (quote! {Opt, doc}, Doc::Same),
            (
                quote! {Opt, doc = "custom docs"},
                Doc::Custom("custom docs".to_string()),
            ),
        ];

        for (args_tokens, doc) in cases {
            let args: Args = syn::parse2(args_tokens).unwrap();

            assert_eq!(args.doc, Some(doc));
        }
    }

    #[test]
    fn parse_attr_list() {
        let meta_tokens = quote! {
            (
                derive(Debug, Clone),
                serde(rename_all = "camelCase", default)
            )
        };

        let meta = Attrs::parse_attr_list.parse2(meta_tokens).unwrap();

        let meta_attrs = parse_attrs(quote! {
            #(#[#meta])*
        });

        let attrs = parse_attrs(quote! {
            #[derive(Debug, Clone)]
            #[serde(rename_all = "camelCase", default)]
        });

        assert_eq!(meta_attrs, attrs);
    }

    #[test]
    fn parse_attrs_test() {
        let parser = Attrs::parse_attr_list;

        let cases = vec![
            (quote! {Opt, attrs}, Attrs::Keep),
            (
                quote! {Opt, attrs = (derive(Debug), serde(rename_all = "camelCase"))},
                Attrs::Replace(
                    parser
                        .parse2(quote! {(derive(Debug), serde(rename_all = "camelCase"))})
                        .unwrap(),
                ),
            ),
            (
                quote! {Opt, attrs = add(derive(Clone), serde(default))},
                Attrs::Add(
                    parser
                        .parse2(quote! {(derive(Clone), serde(default))})
                        .unwrap(),
                ),
            ),
        ];

        for (args_tokens, attrs) in cases {
            let args = parse_args(args_tokens);

            assert_eq!(args.attrs, Some(attrs));
        }
    }

    #[test]
    fn parse_field_doc() {
        let args = parse_args(quote! {
            Opt,
            field_doc
        });

        assert!(args.field_doc);
    }

    #[test]
    fn parse_field_attrs() {
        let parser = Attrs::parse_attr_list;

        let cases = vec![
            (quote! {Opt, field_attrs}, Attrs::Keep),
            (
                quote! {Opt, field_attrs = (derive(Debug), serde(transparent))},
                Attrs::Replace(
                    parser
                        .parse2(quote! {(derive(Debug), serde(transparent))})
                        .unwrap(),
                ),
            ),
            (
                quote! {Opt, field_attrs = add(derive(Clone), serde(deny_unknown_fields))},
                Attrs::Add(
                    parser
                        .parse2(quote! {(derive(Clone), serde(deny_unknown_fields))})
                        .unwrap(),
                ),
            ),
        ];

        for (args_tokens, attrs) in cases {
            let args = parse_args(args_tokens);

            assert_eq!(args.field_attrs, Some(attrs));
        }
    }

    #[test]
    fn parse_from() {
        let args = parse_args(quote! {
            Opt,
            from
        });

        assert!(args.from);
    }
}
