use std::collections::BTreeSet;

use crate::{ModuleName, ProvideKind, SnafuAttribute};
use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token, Expr, Ident, Lit, LitBool, LitStr, Path, Type,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(backtrace);
    custom_keyword!(context);
    custom_keyword!(crate_root);
    custom_keyword!(display);
    custom_keyword!(implicit);
    custom_keyword!(module);
    custom_keyword!(provide);
    custom_keyword!(source);
    custom_keyword!(transparent);
    custom_keyword!(visibility);
    custom_keyword!(whatever);

    custom_keyword!(from);

    custom_keyword!(name);
    custom_keyword!(suffix);

    custom_keyword!(chain);
    custom_keyword!(opt);
    custom_keyword!(priority);
}

pub(crate) fn attributes_from_syn(
    attrs: Vec<syn::Attribute>,
) -> super::MultiSynResult<Vec<SnafuAttribute>> {
    let mut ours = Vec::new();
    let mut errs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("snafu") {
            let attr_list = Punctuated::<Attribute, token::Comma>::parse_terminated;

            match attr.parse_args_with(attr_list) {
                Ok(attrs) => {
                    ours.extend(attrs.into_iter().map(Into::into));
                }
                Err(e) => errs.push(e),
            }
        } else if attr.path().is_ident("doc") {
            // Ignore any errors that occur while parsing the doc
            // comment. This isn't our attribute so we shouldn't
            // assume that we know what values are acceptable.
            if let Ok(comment) = syn::parse2::<DocComment>(attr.meta.to_token_stream()) {
                ours.push(comment.into());
            }
        }
    }

    for attr in &ours {
        if let SnafuAttribute::Provide(tt, ProvideKind::Expression(p)) = attr {
            if p.is_chain && !p.is_ref {
                errs.push(syn::Error::new_spanned(
                    tt,
                    "May only chain to references; please add `ref` flag",
                ));
            }
        }
    }

    if errs.is_empty() {
        Ok(ours)
    } else {
        Err(errs)
    }
}

enum Attribute {
    Backtrace(Backtrace),
    Context(Context),
    CrateRoot(CrateRoot),
    Display(Display),
    Implicit(Implicit),
    Module(Module),
    Provide(Provide),
    Source(Source),
    Transparent(Transparent),
    Visibility(Visibility),
    Whatever(Whatever),
}

impl From<Attribute> for SnafuAttribute {
    fn from(other: Attribute) -> Self {
        use self::Attribute::*;

        match other {
            Backtrace(b) => SnafuAttribute::Backtrace(b.to_token_stream(), b.into_bool()),
            Context(c) => SnafuAttribute::Context(c.to_token_stream(), c.into_component()),
            CrateRoot(cr) => SnafuAttribute::CrateRoot(cr.to_token_stream(), cr.into_arbitrary()),
            Display(d) => SnafuAttribute::Display(d.to_token_stream(), d.into_display()),
            Implicit(d) => SnafuAttribute::Implicit(d.to_token_stream(), d.into_bool()),
            Module(v) => SnafuAttribute::Module(v.to_token_stream(), v.into_value()),
            Provide(v) => SnafuAttribute::Provide(v.to_token_stream(), v.into_value()),
            Source(s) => SnafuAttribute::Source(s.to_token_stream(), s.into_components()),
            Transparent(t) => SnafuAttribute::Transparent(t.to_token_stream(), t.into_bool()),
            Visibility(v) => SnafuAttribute::Visibility(v.to_token_stream(), v.into_arbitrary()),
            Whatever(o) => SnafuAttribute::Whatever(o.to_token_stream()),
        }
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::backtrace) {
            input.parse().map(Attribute::Backtrace)
        } else if lookahead.peek(kw::context) {
            input.parse().map(Attribute::Context)
        } else if lookahead.peek(kw::crate_root) {
            input.parse().map(Attribute::CrateRoot)
        } else if lookahead.peek(kw::display) {
            input.parse().map(Attribute::Display)
        } else if lookahead.peek(kw::implicit) {
            input.parse().map(Attribute::Implicit)
        } else if lookahead.peek(kw::module) {
            input.parse().map(Attribute::Module)
        } else if lookahead.peek(kw::provide) {
            input.parse().map(Attribute::Provide)
        } else if lookahead.peek(kw::source) {
            input.parse().map(Attribute::Source)
        } else if lookahead.peek(kw::transparent) {
            input.parse().map(Attribute::Transparent)
        } else if lookahead.peek(kw::visibility) {
            input.parse().map(Attribute::Visibility)
        } else if lookahead.peek(kw::whatever) {
            input.parse().map(Attribute::Whatever)
        } else {
            Err(lookahead.error())
        }
    }
}

struct Backtrace {
    backtrace_token: kw::backtrace,
    arg: MaybeArg<LitBool>,
}

impl Backtrace {
    fn into_bool(self) -> bool {
        self.arg.into_option().map_or(true, |a| a.value)
    }
}

impl Parse for Backtrace {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            backtrace_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl ToTokens for Backtrace {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.backtrace_token.to_tokens(tokens);
        self.arg.to_tokens(tokens);
    }
}

struct Context {
    context_token: kw::context,
    arg: MaybeArg<ContextArg>,
}

impl Context {
    fn into_component(self) -> super::Context {
        use super::{Context::*, SuffixKind};

        match self.arg.into_option() {
            None => Flag(true),
            Some(arg) => match arg {
                ContextArg::Flag { value } => Flag(value.value),
                ContextArg::Name { name, .. } => Name(name),
                ContextArg::Suffix {
                    suffix:
                        SuffixArg::Flag {
                            value: LitBool { value: true, .. },
                        },
                    ..
                } => Suffix(SuffixKind::Default),
                ContextArg::Suffix {
                    suffix:
                        SuffixArg::Flag {
                            value: LitBool { value: false, .. },
                        },
                    ..
                } => Suffix(SuffixKind::None),
                ContextArg::Suffix {
                    suffix: SuffixArg::Suffix { suffix, .. },
                    ..
                } => Suffix(SuffixKind::Some(suffix)),
            },
        }
    }
}

impl Parse for Context {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            context_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl ToTokens for Context {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.context_token.to_tokens(tokens);
        self.arg.to_tokens(tokens);
    }
}

enum ContextArg {
    Flag {
        value: LitBool,
    },
    Name {
        name_token: kw::name,
        paren_token: token::Paren,
        name: Ident,
    },
    Suffix {
        suffix_token: kw::suffix,
        paren_token: token::Paren,
        suffix: SuffixArg,
    },
}

impl Parse for ContextArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitBool) {
            Ok(ContextArg::Flag {
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::suffix) {
            let content;
            Ok(ContextArg::Suffix {
                suffix_token: input.parse()?,
                paren_token: parenthesized!(content in input),
                suffix: content.parse()?,
            })
        } else if lookahead.peek(kw::name) {
            let content;
            Ok(ContextArg::Name {
                name_token: input.parse()?,
                paren_token: parenthesized!(content in input),
                name: content.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for ContextArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ContextArg::Flag { value } => {
                value.to_tokens(tokens);
            }
            ContextArg::Name {
                name_token,
                paren_token,
                name,
            } => {
                name_token.to_tokens(tokens);
                paren_token.surround(tokens, |tokens| {
                    name.to_tokens(tokens);
                });
            }
            ContextArg::Suffix {
                suffix_token,
                paren_token,
                suffix,
            } => {
                suffix_token.to_tokens(tokens);
                paren_token.surround(tokens, |tokens| {
                    suffix.to_tokens(tokens);
                })
            }
        }
    }
}

enum SuffixArg {
    Flag { value: LitBool },
    Suffix { suffix: Ident },
}

impl Parse for SuffixArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitBool) {
            Ok(SuffixArg::Flag {
                value: input.parse()?,
            })
        } else if lookahead.peek(syn::Ident) {
            Ok(SuffixArg::Suffix {
                suffix: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for SuffixArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            SuffixArg::Flag { value } => {
                value.to_tokens(tokens);
            }
            SuffixArg::Suffix { suffix } => {
                suffix.to_tokens(tokens);
            }
        }
    }
}

struct CrateRoot {
    crate_root_token: kw::crate_root,
    paren_token: token::Paren,
    arg: Path,
}

impl CrateRoot {
    // TODO: Remove boxed trait object
    fn into_arbitrary(self) -> Box<dyn ToTokens> {
        Box::new(self.arg)
    }
}

impl Parse for CrateRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            crate_root_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            arg: content.parse()?,
        })
    }
}

impl ToTokens for CrateRoot {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.crate_root_token.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.arg.to_tokens(tokens);
        });
    }
}

struct Display {
    display_token: kw::display,
    paren_token: token::Paren,
    args: Punctuated<Expr, token::Comma>,
}

impl Display {
    fn into_display(self) -> crate::Display {
        let exprs: Vec<_> = self.args.into_iter().collect();
        let mut shorthand_names = BTreeSet::new();
        let mut assigned_names = BTreeSet::new();

        // Do a best-effort parsing here; if we fail, the compiler
        // will likely spit out something more useful when it tries to
        // parse it.
        if let Some((Expr::Lit(l), args)) = exprs.split_first() {
            if let Lit::Str(s) = &l.lit {
                let format_str = s.value();
                let names = extract_field_names(&format_str).map(|n| format_ident!("{}", n));
                shorthand_names.extend(names);
            }

            for arg in args {
                if let Expr::Assign(a) = arg {
                    if let Expr::Path(p) = &*a.left {
                        assigned_names.extend(p.path.get_ident().cloned());
                    }
                }
            }
        }

        crate::Display {
            exprs,
            shorthand_names,
            assigned_names,
        }
    }
}

pub(crate) fn extract_field_names(mut s: &str) -> impl Iterator<Item = &str> {
    std::iter::from_fn(move || loop {
        let open_curly = s.find('{')?;
        s = &s[open_curly + '{'.len_utf8()..];

        if s.starts_with('{') {
            s = &s['{'.len_utf8()..];
            continue;
        }

        let end_curly = s.find('}')?;
        let format_contents = &s[..end_curly];

        let name = match format_contents.find(':') {
            Some(idx) => &format_contents[..idx],
            None => format_contents,
        };

        if name.is_empty() {
            continue;
        }

        return Some(name);
    })
}

impl Parse for Display {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            display_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            args: Punctuated::parse_terminated(&content)?,
        })
    }
}

impl ToTokens for Display {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.display_token.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
    }
}

struct DocComment {
    doc_ident: Ident,
    eq_token: token::Eq,
    str: LitStr,
}

impl DocComment {
    fn into_value(self) -> String {
        self.str.value()
    }
}

impl From<DocComment> for SnafuAttribute {
    fn from(other: DocComment) -> Self {
        SnafuAttribute::DocComment(other.to_token_stream(), other.into_value())
    }
}

impl Parse for DocComment {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            doc_ident: input.parse()?,
            eq_token: input.parse()?,
            str: input.parse()?,
        })
    }
}

impl ToTokens for DocComment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.doc_ident.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.str.to_tokens(tokens);
    }
}

struct Implicit {
    implicit_token: kw::implicit,
    arg: MaybeArg<LitBool>,
}

impl Implicit {
    fn into_bool(self) -> bool {
        self.arg.into_option().map_or(true, |a| a.value)
    }
}

impl Parse for Implicit {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            implicit_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl ToTokens for Implicit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.implicit_token.to_tokens(tokens);
        self.arg.to_tokens(tokens);
    }
}

struct Module {
    module_token: kw::module,
    arg: MaybeArg<Ident>,
}

impl Module {
    fn into_value(self) -> ModuleName {
        match self.arg.into_option() {
            None => ModuleName::Default,
            Some(name) => ModuleName::Custom(name),
        }
    }
}

impl Parse for Module {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            module_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.module_token.to_tokens(tokens);
        self.arg.to_tokens(tokens);
    }
}

struct Provide {
    provide_token: kw::provide,
    arg: MaybeArg<ProvideArg>,
}

impl Provide {
    fn into_value(self) -> ProvideKind {
        match self.arg.into_option() {
            None => ProvideKind::Flag(true),
            Some(ProvideArg::Flag { value }) => ProvideKind::Flag(value.value),
            Some(ProvideArg::Expression {
                flags,
                ty,
                arrow: _,
                expr,
            }) => ProvideKind::Expression(crate::Provide {
                is_chain: flags.is_chain(),
                is_opt: flags.is_opt(),
                is_priority: flags.is_priority(),
                is_ref: flags.is_ref(),
                ty,
                expr,
            }),
        }
    }
}

impl Parse for Provide {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            provide_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl ToTokens for Provide {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.provide_token.to_tokens(tokens);
        self.arg.to_tokens(tokens);
    }
}

enum ProvideArg {
    Flag {
        value: LitBool,
    },
    Expression {
        flags: ProvideFlags,
        ty: Type,
        arrow: token::FatArrow,
        expr: Expr,
    },
}

impl Parse for ProvideArg {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitBool) {
            Ok(ProvideArg::Flag {
                value: input.parse()?,
            })
        } else {
            Ok(ProvideArg::Expression {
                flags: input.parse()?,
                ty: input.parse()?,
                arrow: input.parse()?,
                expr: input.parse()?,
            })
        }
    }
}

impl ToTokens for ProvideArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ProvideArg::Flag { value } => {
                value.to_tokens(tokens);
            }
            ProvideArg::Expression {
                flags,
                ty,
                arrow,
                expr,
            } => {
                flags.to_tokens(tokens);
                ty.to_tokens(tokens);
                arrow.to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}

struct ProvideFlags(Punctuated<ProvideFlag, token::Comma>);

impl ProvideFlags {
    fn is_chain(&self) -> bool {
        self.0.iter().any(ProvideFlag::is_chain)
    }

    fn is_opt(&self) -> bool {
        self.0.iter().any(ProvideFlag::is_opt)
    }

    fn is_priority(&self) -> bool {
        self.0.iter().any(ProvideFlag::is_priority)
    }

    fn is_ref(&self) -> bool {
        self.0.iter().any(ProvideFlag::is_ref)
    }
}

impl Parse for ProvideFlags {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut flags = Punctuated::new();

        while ProvideFlag::peek(input) {
            flags.push_value(input.parse()?);
            flags.push_punct(input.parse()?);
        }

        Ok(Self(flags))
    }
}

impl ToTokens for ProvideFlags {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

enum ProvideFlag {
    Chain(kw::chain),
    Opt(kw::opt),
    Priority(kw::priority),
    Ref(token::Ref),
}

impl ProvideFlag {
    fn peek(input: ParseStream) -> bool {
        input.peek(kw::chain)
            || input.peek(kw::opt)
            || input.peek(kw::priority)
            || input.peek(token::Ref)
    }

    fn is_chain(&self) -> bool {
        matches!(self, ProvideFlag::Chain(_))
    }

    fn is_opt(&self) -> bool {
        matches!(self, ProvideFlag::Opt(_))
    }

    fn is_priority(&self) -> bool {
        matches!(self, ProvideFlag::Priority(_))
    }

    fn is_ref(&self) -> bool {
        matches!(self, ProvideFlag::Ref(_))
    }
}

impl Parse for ProvideFlag {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::chain) {
            input.parse().map(ProvideFlag::Chain)
        } else if lookahead.peek(kw::opt) {
            input.parse().map(ProvideFlag::Opt)
        } else if lookahead.peek(kw::priority) {
            input.parse().map(ProvideFlag::Priority)
        } else if lookahead.peek(token::Ref) {
            input.parse().map(ProvideFlag::Ref)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for ProvideFlag {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ProvideFlag::Chain(v) => v.to_tokens(tokens),
            ProvideFlag::Opt(v) => v.to_tokens(tokens),
            ProvideFlag::Priority(v) => v.to_tokens(tokens),
            ProvideFlag::Ref(v) => v.to_tokens(tokens),
        }
    }
}

struct Source {
    source_token: kw::source,
    args: MaybeArg<Punctuated<SourceArg, token::Comma>>,
}

impl Source {
    fn into_components(self) -> Vec<super::Source> {
        match self.args.into_option() {
            None => vec![super::Source::Flag(true)],
            Some(args) => args
                .into_iter()
                .map(|sa| match sa {
                    SourceArg::Flag { value } => super::Source::Flag(value.value),
                    SourceArg::From { r#type, expr, .. } => super::Source::From(r#type, expr),
                })
                .collect(),
        }
    }
}

impl Parse for Source {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            source_token: input.parse()?,
            args: MaybeArg::parse_with(input, Punctuated::parse_terminated)?,
        })
    }
}

impl ToTokens for Source {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.source_token.to_tokens(tokens);
        self.args.to_tokens(tokens);
    }
}

enum SourceArg {
    Flag {
        value: LitBool,
    },
    From {
        from_token: kw::from,
        paren_token: token::Paren,
        r#type: Type,
        comma_token: token::Comma,
        expr: Expr,
    },
}

impl Parse for SourceArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitBool) {
            Ok(SourceArg::Flag {
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::from) {
            let content;
            Ok(SourceArg::From {
                from_token: input.parse()?,
                paren_token: parenthesized!(content in input),
                r#type: content.parse()?,
                comma_token: content.parse()?,
                expr: content.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for SourceArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            SourceArg::Flag { value } => {
                value.to_tokens(tokens);
            }
            SourceArg::From {
                from_token,
                paren_token,
                r#type,
                comma_token,
                expr,
            } => {
                from_token.to_tokens(tokens);
                paren_token.surround(tokens, |tokens| {
                    r#type.to_tokens(tokens);
                    comma_token.to_tokens(tokens);
                    expr.to_tokens(tokens);
                })
            }
        }
    }
}

struct Transparent {
    transparent_token: kw::transparent,
    arg: MaybeArg<LitBool>,
}

impl Transparent {
    fn into_bool(self) -> bool {
        self.arg.into_option().map_or(true, |a| a.value)
    }
}

impl Parse for Transparent {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            transparent_token: input.parse()?,
            arg: input.parse()?,
        })
    }
}

impl ToTokens for Transparent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.transparent_token.to_tokens(tokens);
        self.arg.to_tokens(tokens);
    }
}

struct Visibility {
    visibility_token: kw::visibility,
    visibility: MaybeArg<syn::Visibility>,
}

impl Visibility {
    // TODO: Remove boxed trait object
    fn into_arbitrary(self) -> Box<dyn ToTokens> {
        // TODO: Move this default value out of parsing
        self.visibility
            .into_option()
            .map_or_else(super::private_visibility, |v| Box::new(v))
    }
}

impl Parse for Visibility {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            visibility_token: input.parse()?,
            visibility: input.parse()?,
        })
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.visibility_token.to_tokens(tokens);
        self.visibility.to_tokens(tokens);
    }
}

struct Whatever {
    whatever_token: kw::whatever,
}

impl Parse for Whatever {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            whatever_token: input.parse()?,
        })
    }
}

impl ToTokens for Whatever {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.whatever_token.to_tokens(tokens);
    }
}

enum MaybeArg<T> {
    None,
    Some {
        paren_token: token::Paren,
        content: T,
    },
}

impl<T> MaybeArg<T> {
    fn into_option(self) -> Option<T> {
        match self {
            MaybeArg::None => None,
            MaybeArg::Some { content, .. } => Some(content),
        }
    }

    fn parse_with<F>(input: ParseStream<'_>, parser: F) -> Result<Self>
    where
        F: FnOnce(ParseStream<'_>) -> Result<T>,
    {
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Paren) {
            let content;
            Ok(MaybeArg::Some {
                paren_token: parenthesized!(content in input),
                content: parser(&content)?,
            })
        } else {
            Ok(MaybeArg::None)
        }
    }
}

impl<T: Parse> Parse for MaybeArg<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        Self::parse_with(input, Parse::parse)
    }
}

impl<T: ToTokens> ToTokens for MaybeArg<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let MaybeArg::Some {
            paren_token,
            content,
        } = self
        {
            paren_token.surround(tokens, |tokens| {
                content.to_tokens(tokens);
            });
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn names(s: &str) -> Vec<&str> {
        extract_field_names(s).collect::<Vec<_>>()
    }

    #[test]
    fn ignores_positional_arguments() {
        assert_eq!(names("{}"), [] as [&str; 0]);
    }

    #[test]
    fn finds_named_argument() {
        assert_eq!(names("{a}"), ["a"]);
    }

    #[test]
    fn finds_multiple_named_arguments() {
        assert_eq!(names("{a} {b}"), ["a", "b"]);
    }

    #[test]
    fn ignores_escaped_braces() {
        assert_eq!(names("{{a}}"), [] as [&str; 0]);
    }

    #[test]
    fn finds_named_arguments_around_escaped() {
        assert_eq!(names("{a} {{b}} {c}"), ["a", "c"]);
    }

    #[test]
    fn ignores_format_spec() {
        assert_eq!(names("{a:?}"), ["a"]);
    }
}
