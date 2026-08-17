#![recursion_limit = "128"] // https://github.com/rust-lang/rust/issues/62059

extern crate proc_macro;

use crate::parse::attributes_from_syn;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::{BTreeSet, VecDeque};
use std::fmt;

mod parse;
mod shared;

// The snafu crate re-exports this and adds useful documentation.
#[proc_macro_derive(Snafu, attributes(snafu))]
pub fn snafu_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Could not parse type to derive Error for");

    impl_snafu_macro(ast)
}

mod report;
#[proc_macro_attribute]
pub fn report(attr: TokenStream, item: TokenStream) -> TokenStream {
    report::body(attr, item)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

type MultiSynResult<T> = std::result::Result<T, Vec<syn::Error>>;

/// Some arbitrary tokens we treat as a black box
type UserInput = Box<dyn quote::ToTokens>;

enum ModuleName {
    Default,
    Custom(syn::Ident),
}

enum SnafuInfo {
    Enum(EnumInfo),
    NamedStruct(NamedStructInfo),
    TupleStruct(TupleStructInfo),
}

struct EnumInfo {
    crate_root: UserInput,
    name: syn::Ident,
    generics: syn::Generics,
    variants: Vec<FieldContainer>,
    default_visibility: Option<UserInput>,
    default_suffix: SuffixKind,
    module: Option<ModuleName>,
}

/// A struct or enum variant, with named fields.
struct FieldContainer {
    name: syn::Ident,
    backtrace_field: Option<Field>,
    implicit_fields: Vec<Field>,
    selector_kind: ContextSelectorKind,
    display_format: Option<Display>,
    doc_comment: Option<DocComment>,
    visibility: Option<UserInput>,
    module: Option<ModuleName>,
    provides: Vec<Provide>,
    is_transparent: bool,
}

impl FieldContainer {
    fn user_fields(&self) -> &[Field] {
        self.selector_kind.user_fields()
    }

    fn provides(&self) -> &[Provide] {
        &self.provides
    }
}

struct Provide {
    is_chain: bool,
    is_opt: bool,
    is_priority: bool,
    is_ref: bool,
    ty: syn::Type,
    expr: syn::Expr,
}

enum ContextSelectorName {
    Provided(syn::Ident),
    Suffixed(SuffixKind),
}

impl Default for ContextSelectorName {
    fn default() -> Self {
        ContextSelectorName::Suffixed(SuffixKind::Default)
    }
}

impl ContextSelectorName {
    fn resolve_name(&self, def: &SuffixKind, base_name: &syn::Ident) -> syn::Ident {
        match self {
            ContextSelectorName::Provided(ident) => ident.clone(),
            ContextSelectorName::Suffixed(suffix_kind) => suffix_kind.resolve_name(def, base_name),
        }
    }
}

enum SuffixKind {
    Default,
    None,
    Some(syn::Ident),
}

impl SuffixKind {
    const DEFAULT_SUFFIX: &'static str = "Snafu";

    fn resolve_name(&self, def: &Self, base_name: &syn::Ident) -> syn::Ident {
        let span = base_name.span();

        let base_name = base_name.to_string();
        let base_name = base_name.trim_end_matches("Error");

        let suffix = self
            .as_option()
            .or_else(|| def.as_option())
            .unwrap_or(&Self::DEFAULT_SUFFIX);

        quote::format_ident!("{}{}", base_name, suffix, span = span)
    }

    fn as_option(&self) -> Option<&dyn quote::IdentFragment> {
        match self {
            SuffixKind::Default => None,
            SuffixKind::None => Some(&""),
            SuffixKind::Some(s) => Some(s),
        }
    }
}

enum ContextSelectorKind {
    Context {
        selector_name: ContextSelectorName,
        source_field: Option<SourceField>,
        user_fields: Vec<Field>,
    },

    Whatever {
        source_field: Option<SourceField>,
        message_field: Field,
    },

    NoContext {
        source_field: SourceField,
    },
}

impl ContextSelectorKind {
    fn is_whatever(&self) -> bool {
        matches!(self, ContextSelectorKind::Whatever { .. })
    }

    fn user_fields(&self) -> &[Field] {
        match self {
            ContextSelectorKind::Context { user_fields, .. } => user_fields,
            ContextSelectorKind::Whatever { .. } => &[],
            ContextSelectorKind::NoContext { .. } => &[],
        }
    }

    fn source_field(&self) -> Option<&SourceField> {
        match self {
            ContextSelectorKind::Context { source_field, .. } => source_field.as_ref(),
            ContextSelectorKind::Whatever { source_field, .. } => source_field.as_ref(),
            ContextSelectorKind::NoContext { source_field } => Some(source_field),
        }
    }

    fn message_field(&self) -> Option<&Field> {
        match self {
            ContextSelectorKind::Context { .. } => None,
            ContextSelectorKind::Whatever { message_field, .. } => Some(message_field),
            ContextSelectorKind::NoContext { .. } => None,
        }
    }

    fn resolve_name(&self, def: &SuffixKind, base_name: &syn::Ident) -> syn::Ident {
        let selector_name_default;

        let selector_name = match self {
            ContextSelectorKind::Context { selector_name, .. } => selector_name,
            _ => {
                selector_name_default = ContextSelectorName::default();
                &selector_name_default
            }
        };

        selector_name.resolve_name(def, base_name)
    }
}

struct NamedStructInfo {
    crate_root: UserInput,
    field_container: FieldContainer,
    generics: syn::Generics,
}

struct TupleStructInfo {
    crate_root: UserInput,
    name: syn::Ident,
    generics: syn::Generics,
    transformation: Transformation,
    provides: Vec<Provide>,
}

#[derive(Clone)]
pub(crate) struct Field {
    name: syn::Ident,
    ty: syn::Type,
    provide: bool,
    original: syn::Field,
}

impl Field {
    fn name(&self) -> &syn::Ident {
        &self.name
    }
}

struct SourceField {
    name: syn::Ident,
    transformation: Transformation,
    backtrace_delegate: bool,
    provide: bool,
}

impl SourceField {
    fn name(&self) -> &syn::Ident {
        &self.name
    }
}

enum Transformation {
    None {
        ty: syn::Type,
    },
    Transform {
        source_ty: syn::Type,
        target_ty: syn::Type,
        expr: syn::Expr,
    },
}

impl Transformation {
    fn source_ty(&self) -> &syn::Type {
        match self {
            Transformation::None { ty } => ty,
            Transformation::Transform { source_ty, .. } => source_ty,
        }
    }

    fn target_ty(&self) -> &syn::Type {
        match self {
            Transformation::None { ty } => ty,
            Transformation::Transform { target_ty, .. } => target_ty,
        }
    }

    fn transformation(&self) -> proc_macro2::TokenStream {
        match self {
            Transformation::None { .. } => quote! { |v| v },
            Transformation::Transform { expr, .. } => quote! { #expr },
        }
    }
}

enum ProvideKind {
    Flag(bool),
    Expression(Provide),
}

/// SyntaxErrors is a convenience wrapper for a list of syntax errors discovered while parsing
/// something that derives Snafu.  It makes it easier for developers to add and return syntax
/// errors while walking through the parse tree.
#[derive(Debug, Default)]
struct SyntaxErrors {
    inner: Vec<syn::Error>,
}

impl SyntaxErrors {
    /// Start a set of errors that all share the same location
    fn scoped(&mut self, scope: ErrorLocation) -> SyntaxErrorsScoped<'_> {
        SyntaxErrorsScoped {
            errors: self,
            scope,
        }
    }

    /// Adds a new syntax error. The description will be used in the
    /// compile error pointing to the tokens.
    fn add(&mut self, tokens: impl quote::ToTokens, description: impl fmt::Display) {
        self.inner
            .push(syn::Error::new_spanned(tokens, description));
    }

    /// Adds the given list of errors.
    fn extend(&mut self, errors: impl IntoIterator<Item = syn::Error>) {
        self.inner.extend(errors);
    }

    #[allow(dead_code)]
    /// Returns the number of errors that have been added.
    fn len(&self) -> usize {
        self.inner.len()
    }

    /// Consume the SyntaxErrors, returning Ok if there were no syntax errors added, or Err(list)
    /// if there were syntax errors.
    fn finish(self) -> MultiSynResult<()> {
        if self.inner.is_empty() {
            Ok(())
        } else {
            Err(self.inner)
        }
    }

    /// Consume the SyntaxErrors and a Result, returning the success
    /// value if neither have errors, otherwise combining the errors.
    fn absorb<T>(mut self, res: MultiSynResult<T>) -> MultiSynResult<T> {
        match res {
            Ok(v) => self.finish().map(|()| v),
            Err(e) => {
                self.inner.extend(e);
                Err(self.inner)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ErrorLocation {
    OnEnum,
    OnVariant,
    InVariant,
    OnField,
    OnNamedStruct,
    InNamedStruct,
    OnTupleStruct,
    OnTupleStructField,
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::ErrorLocation::*;

        match self {
            OnEnum => "on an enum".fmt(f),
            OnVariant => "on an enum variant".fmt(f),
            InVariant => "within an enum variant".fmt(f),
            OnField => "on a field".fmt(f),
            OnNamedStruct => "on a named struct".fmt(f),
            InNamedStruct => "within a named struct".fmt(f),
            OnTupleStruct => "on a tuple struct".fmt(f),
            OnTupleStructField => "on a tuple struct field".fmt(f),
        }
    }
}

trait ErrorForLocation {
    fn for_location(&self, location: ErrorLocation) -> String;
}

struct SyntaxErrorsScoped<'a> {
    errors: &'a mut SyntaxErrors,
    scope: ErrorLocation,
}

impl SyntaxErrorsScoped<'_> {
    /// Adds a new syntax error. The description will be used in the
    /// compile error pointing to the tokens.
    fn add(&mut self, tokens: impl quote::ToTokens, description: impl ErrorForLocation) {
        let description = description.for_location(self.scope);
        self.errors.add(tokens, description)
    }
}

/// Helper structure to handle cases where an attribute is
/// syntactically valid but semantically invalid.
#[derive(Debug)]
struct DoesNothing {
    /// The name of the attribute that was misused.
    attribute: &'static str,
}

impl ErrorForLocation for DoesNothing {
    fn for_location(&self, _location: ErrorLocation) -> String {
        format!("`{}` attribute has no effect", self.attribute)
    }
}

/// Helper structure to handle cases where an attribute was used on an
/// element where it's not valid.
#[derive(Debug)]
struct OnlyValidOn {
    /// The name of the attribute that was misused.
    attribute: &'static str,
    /// A description of where that attribute is valid.
    valid_on: &'static str,
}

impl ErrorForLocation for OnlyValidOn {
    fn for_location(&self, location: ErrorLocation) -> String {
        format!(
            "`{}` attribute is only valid on {}, not {}",
            self.attribute, self.valid_on, location,
        )
    }
}

/// Helper structure to handle cases where a specific attribute value
/// was used on an field where it's not valid.
#[derive(Debug)]
struct WrongField {
    /// The name of the attribute that was misused.
    attribute: &'static str,
    /// The name of the field where that attribute is valid.
    valid_field: &'static str,
}

impl ErrorForLocation for WrongField {
    fn for_location(&self, _location: ErrorLocation) -> String {
        format!(
            r#"`{}` attribute is only valid on a field named "{}", not on other fields"#,
            self.attribute, self.valid_field,
        )
    }
}

/// Helper structure to handle cases where two incompatible attributes
/// were specified on the same element.
#[derive(Debug)]
struct IncompatibleAttributes(&'static [&'static str]);

impl ErrorForLocation for IncompatibleAttributes {
    fn for_location(&self, location: ErrorLocation) -> String {
        let attrs_string = self
            .0
            .iter()
            .map(|attr| format!("`{}`", attr))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "Incompatible attributes [{}] specified {}",
            attrs_string, location,
        )
    }
}

/// Helper structure to handle cases where an attribute was
/// incorrectly used multiple times on the same element.
#[derive(Debug)]
struct DuplicateAttribute {
    attribute: &'static str,
    location: ErrorLocation,
}

impl fmt::Display for DuplicateAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Multiple `{}` attributes are not supported {}",
            self.attribute, self.location,
        )
    }
}

/// AtMostOne is a helper to track attributes seen during parsing.  If more than one item is added,
/// it's added to a list of DuplicateAttribute errors, using the given `name` and `location` as
/// descriptors.
///
/// When done parsing a structure, call `finish` to get first attribute found, if any, and the list
/// of errors, or call `finish_with_location` to get the attribute and the token tree where it was
/// found, which can be useful for error reporting.
#[derive(Debug)]
struct AtMostOne<T, U>
where
    U: quote::ToTokens,
{
    name: &'static str,
    location: ErrorLocation,
    // We store all the values we've seen to allow for `iter`, which helps the `AtMostOne` be
    // useful for additional manual error checking.
    values: VecDeque<(T, U)>,
    errors: SyntaxErrors,
}

impl<T, U> AtMostOne<T, U>
where
    U: quote::ToTokens + Clone,
{
    /// Creates an AtMostOne to track an attribute with the given
    /// `name` on the given `location` (often referencing a parent
    /// element).
    fn new(name: &'static str, location: ErrorLocation) -> Self {
        Self {
            name,
            location,
            values: VecDeque::new(),
            errors: SyntaxErrors::default(),
        }
    }

    /// Add an occurence of the attribute found at the given token tree `tokens`.
    fn add(&mut self, item: T, tokens: U) {
        if !self.values.is_empty() {
            self.errors.add(
                tokens.clone(),
                DuplicateAttribute {
                    attribute: self.name,
                    location: self.location,
                },
            );
        }
        self.values.push_back((item, tokens));
    }

    #[allow(dead_code)]
    /// Returns the number of elements that have been added.
    fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if no elements have been added, otherwise false.
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an iterator over all values that have been added.
    ///
    /// This can help with additional manual error checks beyond the duplication checks that
    /// `AtMostOne` handles for you.
    fn iter(&self) -> std::collections::vec_deque::Iter<'_, (T, U)> {
        self.values.iter()
    }

    /// Consumes the AtMostOne, returning the first item added, if any, and the list of errors
    /// representing any items added beyond the first.
    fn finish(self) -> (Option<T>, Vec<syn::Error>) {
        let (value, errors) = self.finish_with_location();
        (value.map(|(val, _location)| val), errors)
    }

    /// Like `finish` but also returns the location of the first item added.  Useful when you have
    /// to do additional, manual error checking on the first item added, and you'd like to report
    /// an accurate location for it in case of errors.
    fn finish_with_location(mut self) -> (Option<(T, U)>, Vec<syn::Error>) {
        let errors = match self.errors.finish() {
            Ok(()) => Vec::new(),
            Err(vec) => vec,
        };
        (self.values.pop_front(), errors)
    }
}

fn impl_snafu_macro(ty: syn::DeriveInput) -> TokenStream {
    match parse_snafu_information(ty) {
        Ok(info) => info.into(),
        Err(e) => to_compile_errors(e).into(),
    }
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote! { #(#compile_errors)* }
}

fn parse_snafu_information(ty: syn::DeriveInput) -> MultiSynResult<SnafuInfo> {
    use syn::spanned::Spanned;
    use syn::Data;

    let span = ty.span();
    let syn::DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = ty;

    match data {
        Data::Enum(enum_) => parse_snafu_enum(enum_, ident, generics, attrs).map(SnafuInfo::Enum),
        Data::Struct(struct_) => parse_snafu_struct(struct_, ident, generics, attrs, span),
        _ => Err(vec![syn::Error::new(
            span,
            "Can only derive `Snafu` for an enum or a newtype",
        )]),
    }
}

const ATTR_DISPLAY: OnlyValidOn = OnlyValidOn {
    attribute: "display",
    valid_on: "enum variants or structs with named fields",
};

const ATTR_SOURCE: OnlyValidOn = OnlyValidOn {
    attribute: "source",
    valid_on: "enum variant or struct fields with a name",
};

const ATTR_SOURCE_BOOL: OnlyValidOn = OnlyValidOn {
    attribute: "source(bool)",
    valid_on: "enum variant or struct fields with a name",
};

const ATTR_SOURCE_FALSE: WrongField = WrongField {
    attribute: "source(false)",
    valid_field: "source",
};

const ATTR_SOURCE_FROM: OnlyValidOn = OnlyValidOn {
    attribute: "source(from)",
    valid_on: "enum variant or struct fields with a name",
};

const ATTR_BACKTRACE: OnlyValidOn = OnlyValidOn {
    attribute: "backtrace",
    valid_on: "enum variant or struct fields with a name",
};

const ATTR_BACKTRACE_FALSE: WrongField = WrongField {
    attribute: "backtrace(false)",
    valid_field: "backtrace",
};

const ATTR_IMPLICIT: OnlyValidOn = OnlyValidOn {
    attribute: "implicit",
    valid_on: "enum variant or struct fields with a name",
};

const ATTR_IMPLICIT_FALSE: DoesNothing = DoesNothing {
    attribute: "implicit(false)",
};

const ATTR_VISIBILITY: OnlyValidOn = OnlyValidOn {
    attribute: "visibility",
    valid_on: "an enum, enum variants, or a struct with named fields",
};

const ATTR_MODULE: OnlyValidOn = OnlyValidOn {
    attribute: "module",
    valid_on: "an enum or structs with named fields",
};

const ATTR_PROVIDE_FLAG: OnlyValidOn = OnlyValidOn {
    attribute: "provide",
    valid_on: "enum variant or struct fields with a name",
};

const ATTR_PROVIDE_FALSE: WrongField = WrongField {
    attribute: "provide(false)",
    valid_field: r#"source" or "backtrace"#,
};

const ATTR_PROVIDE_EXPRESSION: OnlyValidOn = OnlyValidOn {
    attribute: "provide(type => expression)",
    valid_on: "enum variants, structs with named fields, or tuple structs",
};

const ATTR_CONTEXT: OnlyValidOn = OnlyValidOn {
    attribute: "context",
    valid_on: "enum variants or structs with named fields",
};

const ATTR_CONTEXT_FLAG: OnlyValidOn = OnlyValidOn {
    attribute: "context(bool)",
    valid_on: "enum variants or structs with named fields",
};

const ATTR_CONTEXT_NAME: OnlyValidOn = OnlyValidOn {
    attribute: "context(name)",
    valid_on: "enum variants or structs with named fields",
};

const ATTR_WHATEVER: OnlyValidOn = OnlyValidOn {
    attribute: "whatever",
    valid_on: "enum variants or structs with named fields",
};

const ATTR_CRATE_ROOT: OnlyValidOn = OnlyValidOn {
    attribute: "crate_root",
    valid_on: "an enum or a struct",
};

const ATTR_TRANSPARENT: OnlyValidOn = OnlyValidOn {
    attribute: "transparent",
    valid_on: "enum variants or structs with named fields",
};

const ATTR_TRANSPARENT_FALSE: DoesNothing = DoesNothing {
    attribute: "transparent(false)",
};

const SOURCE_BOOL_FROM_INCOMPATIBLE: IncompatibleAttributes =
    IncompatibleAttributes(&["source(false)", "source(from)"]);

fn parse_snafu_enum(
    enum_: syn::DataEnum,
    name: syn::Ident,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
) -> MultiSynResult<EnumInfo> {
    use syn::spanned::Spanned;
    use syn::Fields;

    let mut errors = SyntaxErrors::default();

    let mut modules = AtMostOne::new("module", ErrorLocation::OnEnum);
    let mut default_visibilities = AtMostOne::new("visibility", ErrorLocation::OnEnum);
    let mut default_suffixes = AtMostOne::new("context(suffix)", ErrorLocation::OnEnum);
    let mut crate_roots = AtMostOne::new("crate_root", ErrorLocation::OnEnum);
    let mut enum_errors = errors.scoped(ErrorLocation::OnEnum);

    for attr in attributes_from_syn(attrs)? {
        use SnafuAttribute as Att;

        match attr {
            Att::Visibility(tokens, v) => default_visibilities.add(v, tokens),
            Att::Display(tokens, ..) => enum_errors.add(tokens, ATTR_DISPLAY),
            Att::Source(tokens, ss) => {
                for s in ss {
                    match s {
                        Source::Flag(..) => enum_errors.add(tokens.clone(), ATTR_SOURCE_BOOL),
                        Source::From(..) => enum_errors.add(tokens.clone(), ATTR_SOURCE_FROM),
                    }
                }
            }
            Att::CrateRoot(tokens, root) => crate_roots.add(root, tokens),
            Att::Context(tokens, c) => match c {
                Context::Suffix(s) => default_suffixes.add(s, tokens),
                Context::Flag(_) => enum_errors.add(tokens, ATTR_CONTEXT_FLAG),
                Context::Name(_) => enum_errors.add(tokens, ATTR_CONTEXT_NAME),
            },
            Att::Module(tokens, v) => modules.add(v, tokens),
            Att::Provide(tokens, ProvideKind::Flag(..)) => {
                enum_errors.add(tokens, ATTR_PROVIDE_FLAG)
            }
            Att::Provide(tokens, ProvideKind::Expression { .. }) => {
                enum_errors.add(tokens, ATTR_PROVIDE_EXPRESSION)
            }
            Att::Backtrace(tokens, ..) => enum_errors.add(tokens, ATTR_BACKTRACE),
            Att::Implicit(tokens, ..) => enum_errors.add(tokens, ATTR_IMPLICIT),
            Att::Transparent(tokens, ..) => enum_errors.add(tokens, ATTR_TRANSPARENT),
            Att::Whatever(tokens) => enum_errors.add(tokens, ATTR_WHATEVER),
            Att::DocComment(..) => { /* Just a regular doc comment. */ }
        }
    }

    let (module, errs) = modules.finish();
    errors.extend(errs);

    let (default_visibility, errs) = default_visibilities.finish();
    errors.extend(errs);

    let (maybe_default_suffix, errs) = default_suffixes.finish();
    let default_suffix = maybe_default_suffix.unwrap_or(SuffixKind::Default);
    errors.extend(errs);

    let (maybe_crate_root, errs) = crate_roots.finish();
    let crate_root = maybe_crate_root.unwrap_or_else(default_crate_root);
    errors.extend(errs);

    let variants: sponge::AllErrors<_, _> = enum_
        .variants
        .into_iter()
        .map(|variant| {
            let fields = match variant.fields {
                Fields::Named(f) => f.named.into_iter().collect(),
                Fields::Unnamed(_) => {
                    return Err(vec![syn::Error::new(
                        variant.fields.span(),
                        "Can only derive `Snafu` for enums with struct-like and unit enum variants",
                    )]);
                }
                Fields::Unit => vec![],
            };

            let name = variant.ident;
            let span = name.span();

            let attrs = attributes_from_syn(variant.attrs)?;

            field_container(
                name,
                span,
                attrs,
                fields,
                &mut errors,
                ErrorLocation::OnVariant,
                ErrorLocation::InVariant,
            )
        })
        .collect();

    let variants = errors.absorb(variants.into_result())?;

    Ok(EnumInfo {
        crate_root,
        name,
        generics,
        variants,
        default_visibility,
        default_suffix,
        module,
    })
}

fn field_container(
    name: syn::Ident,
    variant_span: proc_macro2::Span,
    attrs: Vec<SnafuAttribute>,
    fields: Vec<syn::Field>,
    errors: &mut SyntaxErrors,
    outer_error_location: ErrorLocation,
    inner_error_location: ErrorLocation,
) -> MultiSynResult<FieldContainer> {
    use quote::ToTokens;
    use syn::spanned::Spanned;

    let mut outer_errors = errors.scoped(outer_error_location);

    let mut modules = AtMostOne::new("module", outer_error_location);
    let mut display_formats = AtMostOne::new("display", outer_error_location);
    let mut visibilities = AtMostOne::new("visibility", outer_error_location);
    let mut provides = Vec::new();

    let mut contexts = AtMostOne::new("context", outer_error_location);
    let mut whatevers = AtMostOne::new("whatever", outer_error_location);
    let mut transparents = AtMostOne::new("transparent", outer_error_location);

    let mut doc_comment = DocComment::default();
    let mut reached_end_of_doc_comment = false;

    for attr in attrs {
        use SnafuAttribute as Att;

        match attr {
            Att::Module(tokens, n) => modules.add(n, tokens),
            Att::Display(tokens, d) => display_formats.add(d, tokens),
            Att::Visibility(tokens, v) => visibilities.add(v, tokens),
            Att::Context(tokens, c) => contexts.add(c, tokens),
            Att::Whatever(tokens) => whatevers.add((), tokens),
            Att::Transparent(tokens, t) => {
                if t {
                    transparents.add((), tokens)
                } else {
                    outer_errors.add(tokens, ATTR_TRANSPARENT_FALSE)
                }
            }
            Att::Source(tokens, ..) => outer_errors.add(tokens, ATTR_SOURCE),
            Att::Backtrace(tokens, ..) => outer_errors.add(tokens, ATTR_BACKTRACE),
            Att::Implicit(tokens, ..) => outer_errors.add(tokens, ATTR_IMPLICIT),
            Att::CrateRoot(tokens, ..) => outer_errors.add(tokens, ATTR_CRATE_ROOT),
            Att::Provide(tokens, ProvideKind::Flag(..)) => {
                outer_errors.add(tokens, ATTR_PROVIDE_FLAG)
            }
            Att::Provide(_tts, ProvideKind::Expression(provide)) => {
                // TODO: can we have improved error handling for obvious type duplicates?
                provides.push(provide);
            }
            Att::DocComment(_tts, doc_comment_line) => {
                // We join all the doc comment attributes with a space,
                // but end once the summary of the doc comment is
                // complete, which is indicated by an empty line.
                if !reached_end_of_doc_comment {
                    let trimmed = doc_comment_line.trim();
                    if trimmed.is_empty() {
                        reached_end_of_doc_comment = true;
                    } else {
                        doc_comment.push_str(trimmed);
                    }
                }
            }
        }
    }

    let mut user_fields = Vec::new();
    let mut source_fields = AtMostOne::new("source", inner_error_location);
    let mut backtrace_fields = AtMostOne::new("backtrace", inner_error_location);
    let mut implicit_fields = Vec::new();

    for syn_field in fields {
        let original = syn_field.clone();
        let span = syn_field.span();
        let name = syn_field
            .ident
            .as_ref()
            .ok_or_else(|| vec![syn::Error::new(span, "Must have a named field")])?;

        // Check whether we have multiple source/backtrace attributes on this field.
        // We can't just add to source_fields/backtrace_fields from inside the attribute
        // loop because source and backtrace are connected and require a bit of special
        // logic after the attribute loop.  For example, we need to know whether there's a
        // source transformation before we record a source field, but it might be on a
        // later attribute.  We use the data field of `source_attrs` to track any
        // transformations in case it was a `source(from(...))`, but for backtraces we
        // don't need any more data.
        let mut source_attrs = AtMostOne::new("source", ErrorLocation::OnField);
        let mut backtrace_attrs = AtMostOne::new("backtrace", ErrorLocation::OnField);
        let mut implicit_attrs = AtMostOne::new("implicit", ErrorLocation::OnField);
        let mut provide_attrs = AtMostOne::new("provide", ErrorLocation::OnField);

        // Keep track of the negative markers so we can check for inconsistencies and
        // exclude fields even if they have the "source" or "backtrace" name.
        let mut source_opt_out = false;
        let mut backtrace_opt_out = false;
        let mut provide_opt_out = false;

        let mut field_errors = errors.scoped(ErrorLocation::OnField);

        for attr in attributes_from_syn(syn_field.attrs.clone())? {
            use SnafuAttribute as Att;

            match attr {
                Att::Source(tokens, ss) => {
                    for s in ss {
                        match s {
                            Source::Flag(v) => {
                                // If we've seen a `source(from)` then there will be a
                                // `Some` value in `source_attrs`.
                                let seen_source_from = source_attrs
                                    .iter()
                                    .map(|(val, _location)| val)
                                    .any(Option::is_some);
                                if !v && seen_source_from {
                                    field_errors.add(tokens.clone(), SOURCE_BOOL_FROM_INCOMPATIBLE);
                                }
                                if v {
                                    source_attrs.add(None, tokens.clone());
                                } else if is_implicit_source(name) {
                                    source_opt_out = true;
                                } else {
                                    field_errors.add(tokens.clone(), ATTR_SOURCE_FALSE);
                                }
                            }
                            Source::From(t, e) => {
                                if source_opt_out {
                                    field_errors.add(tokens.clone(), SOURCE_BOOL_FROM_INCOMPATIBLE);
                                }
                                source_attrs.add(Some((t, e)), tokens.clone());
                            }
                        }
                    }
                }
                Att::Backtrace(tokens, v) => {
                    if v {
                        backtrace_attrs.add((), tokens);
                    } else if is_implicit_backtrace(name) {
                        backtrace_opt_out = true;
                    } else {
                        field_errors.add(tokens, ATTR_BACKTRACE_FALSE);
                    }
                }
                Att::Implicit(tokens, v) => {
                    if v {
                        implicit_attrs.add((), tokens);
                    } else {
                        field_errors.add(tokens, ATTR_IMPLICIT_FALSE);
                    }
                }
                Att::Module(tokens, ..) => field_errors.add(tokens, ATTR_MODULE),
                Att::Provide(tokens, ProvideKind::Flag(v)) => {
                    if v {
                        provide_attrs.add((), tokens);
                    } else if is_implicit_provide(name) {
                        provide_opt_out = true;
                    } else {
                        field_errors.add(tokens, ATTR_PROVIDE_FALSE)
                    }
                }
                Att::Provide(tokens, ProvideKind::Expression { .. }) => {
                    field_errors.add(tokens, ATTR_PROVIDE_EXPRESSION)
                }
                Att::Visibility(tokens, ..) => field_errors.add(tokens, ATTR_VISIBILITY),
                Att::Display(tokens, ..) => field_errors.add(tokens, ATTR_DISPLAY),
                Att::Context(tokens, ..) => field_errors.add(tokens, ATTR_CONTEXT),
                Att::Transparent(tokens, ..) => field_errors.add(tokens, ATTR_TRANSPARENT),
                Att::Whatever(tokens) => field_errors.add(tokens, ATTR_WHATEVER),
                Att::CrateRoot(tokens, ..) => field_errors.add(tokens, ATTR_CRATE_ROOT),
                Att::DocComment(..) => { /* Just a regular doc comment. */ }
            }
        }

        // Add errors for any duplicated attributes on this field.
        let (source_attr, errs) = source_attrs.finish_with_location();
        errors.extend(errs);
        let (backtrace_attr, errs) = backtrace_attrs.finish_with_location();
        errors.extend(errs);

        let (implicit_attr, errs) = implicit_attrs.finish();
        errors.extend(errs);

        let (provide_attr, errs) = provide_attrs.finish();
        errors.extend(errs);

        let field = Field {
            name: name.clone(),
            ty: syn_field.ty.clone(),
            provide: provide_attr.is_some() || (is_implicit_provide(name) && !provide_opt_out),
            original,
        };

        let source_attr = source_attr.or_else(|| {
            if is_implicit_source(&field.name) && !source_opt_out {
                Some((None, syn_field.clone().into_token_stream()))
            } else {
                None
            }
        });

        let backtrace_attr = backtrace_attr.or_else(|| {
            if is_implicit_backtrace(&field.name) && !backtrace_opt_out {
                Some(((), syn_field.clone().into_token_stream()))
            } else {
                None
            }
        });

        let implicit_attr = implicit_attr.is_some();

        if let Some((maybe_transformation, location)) = source_attr {
            let Field {
                name, ty, provide, ..
            } = field;
            let transformation = maybe_transformation
                .map(|(source_ty, expr)| Transformation::Transform {
                    source_ty,
                    target_ty: ty.clone(),
                    expr,
                })
                .unwrap_or_else(|| Transformation::None { ty });

            source_fields.add(
                SourceField {
                    name,
                    transformation,
                    // Specifying `backtrace` on a source field is how you request
                    // delegation of the backtrace to the source error type.
                    backtrace_delegate: backtrace_attr.is_some(),
                    provide,
                },
                location,
            );
        } else if let Some((_, location)) = backtrace_attr {
            backtrace_fields.add(field, location);
        } else if implicit_attr {
            implicit_fields.push(field);
        } else {
            user_fields.push(field);
        }
    }

    let (source, errs) = source_fields.finish_with_location();
    errors.extend(errs);

    let (backtrace, errs) = backtrace_fields.finish_with_location();
    errors.extend(errs);

    match (&source, &backtrace) {
        (Some(source), Some(backtrace)) if source.0.backtrace_delegate => {
            let source_location = source.1.clone();
            let backtrace_location = backtrace.1.clone();
            errors.add(
                source_location,
                "Cannot have `backtrace` field and `backtrace` attribute on a source field in the same variant",
            );
            errors.add(
                backtrace_location,
                "Cannot have `backtrace` field and `backtrace` attribute on a source field in the same variant",
            );
        }
        _ => {} // no conflict
    }

    let (module, errs) = modules.finish();
    errors.extend(errs);

    let (display_format, errs) = display_formats.finish_with_location();
    errors.extend(errs);

    let (visibility, errs) = visibilities.finish();
    errors.extend(errs);

    let (is_context, errs) = contexts.finish_with_location();
    let mut is_context = is_context.map(|(c, tt)| (c.into_enabled(), tt));
    let mut is_context_false_checks_source = "Context selectors without context";
    errors.extend(errs);

    let (is_whatever, errs) = whatevers.finish_with_location();
    errors.extend(errs);

    let (is_transparent, errs) = transparents.finish_with_location();
    errors.extend(errs);

    if let (Some((_, d_tt)), Some((_, t_tt))) = (&display_format, &is_transparent) {
        let txt = "`transparent` errors cannot have a display format because they delegate `Display` to their source";
        errors.extend([
            syn::Error::new_spanned(d_tt, txt),
            syn::Error::new_spanned(t_tt, txt),
        ]);
    }

    match (&is_context, &is_transparent) {
        (Some(((true, _), c_tt)), Some((_, t_tt))) => {
            let txt = "`transparent` errors cannot have context";
            errors.extend([
                syn::Error::new_spanned(c_tt, txt),
                syn::Error::new_spanned(t_tt, txt),
            ]);
        }
        (None, Some((_, t_tt))) => {
            // `transparent` implies `context(false)`. Treat `transparent` as if setting
            // `context(false)` as well.
            is_context = Some((
                (false, ContextSelectorName::Suffixed(SuffixKind::None)),
                t_tt.clone(),
            ));
            is_context_false_checks_source = "`transparent` errors";
        }
        (Some(((false, _), _)), Some(_)) | (_, None) => {}
    }

    let source_field = source.map(|(val, _tts)| val);

    let selector_kind = match (is_context, is_whatever) {
        (Some(((true, _), c_tt)), Some(((), o_tt))) => {
            let txt = "Cannot be both a `context` and `whatever` error";
            return Err(vec![
                syn::Error::new_spanned(c_tt, txt),
                syn::Error::new_spanned(o_tt, txt),
            ]);
        }

        (Some(((true, suffix), _)), None) => ContextSelectorKind::Context {
            selector_name: suffix,
            source_field,
            user_fields,
        },

        (None, None) => ContextSelectorKind::Context {
            selector_name: ContextSelectorName::Suffixed(SuffixKind::Default),
            source_field,
            user_fields,
        },

        (Some(((false, _), _)), Some(_)) | (None, Some(_)) => {
            let mut messages = AtMostOne::new("message", outer_error_location);

            for f in user_fields {
                if is_implicit_message(&f.name) {
                    let l = f.original.clone();
                    messages.add(f, l);
                } else {
                    errors.add(
                        f.original,
                        "Whatever selectors must not have context fields",
                    );
                    // todo: phrasing?
                }
            }

            let (message_field, errs) = messages.finish();
            errors.extend(errs);

            let message_field = message_field.ok_or_else(|| {
                vec![syn::Error::new(
                    variant_span,
                    "Whatever selectors must have a message field",
                )]
            })?;

            ContextSelectorKind::Whatever {
                source_field,
                message_field,
            }
        }

        (Some(((false, _), _)), None) => {
            errors.extend(user_fields.into_iter().map(|Field { original, .. }| {
                syn::Error::new_spanned(
                    original,
                    format_args!(
                        "{} must not have context fields",
                        is_context_false_checks_source
                    ),
                )
            }));

            let source_field = source_field.ok_or_else(|| {
                vec![syn::Error::new(
                    variant_span,
                    format_args!(
                        "{} must have a source field",
                        is_context_false_checks_source
                    ),
                )]
            })?;

            ContextSelectorKind::NoContext { source_field }
        }
    };

    Ok(FieldContainer {
        name,
        backtrace_field: backtrace.map(|(val, _tts)| val),
        implicit_fields,
        selector_kind,
        display_format: display_format.map(|(d, _)| d),
        doc_comment: doc_comment.finish(),
        visibility,
        module,
        provides,
        is_transparent: is_transparent.is_some(),
    })
}

const IMPLICIT_SOURCE_FIELD_NAME: &str = "source";
const IMPLICIT_BACKTRACE_FIELD_NAME: &str = "backtrace";
const IMPLICIT_MESSAGE_FIELD_NAME: &str = "message";

fn is_implicit_source(name: &proc_macro2::Ident) -> bool {
    name == IMPLICIT_SOURCE_FIELD_NAME
}

fn is_implicit_backtrace(name: &proc_macro2::Ident) -> bool {
    name == IMPLICIT_BACKTRACE_FIELD_NAME
}

fn is_implicit_message(name: &proc_macro2::Ident) -> bool {
    name == IMPLICIT_MESSAGE_FIELD_NAME
}

fn is_implicit_provide(name: &proc_macro2::Ident) -> bool {
    is_implicit_source(name) || is_implicit_backtrace(name)
}

fn parse_snafu_struct(
    struct_: syn::DataStruct,
    name: syn::Ident,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    span: proc_macro2::Span,
) -> MultiSynResult<SnafuInfo> {
    use syn::Fields;

    match struct_.fields {
        Fields::Named(f) => {
            let f = f.named.into_iter().collect();
            parse_snafu_named_struct(f, name, generics, attrs, span).map(SnafuInfo::NamedStruct)
        }
        Fields::Unnamed(f) => {
            parse_snafu_tuple_struct(f, name, generics, attrs, span).map(SnafuInfo::TupleStruct)
        }
        Fields::Unit => parse_snafu_named_struct(vec![], name, generics, attrs, span)
            .map(SnafuInfo::NamedStruct),
    }
}

fn parse_snafu_named_struct(
    fields: Vec<syn::Field>,
    name: syn::Ident,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    span: proc_macro2::Span,
) -> MultiSynResult<NamedStructInfo> {
    let mut errors = SyntaxErrors::default();

    let attrs = attributes_from_syn(attrs)?;

    let mut crate_roots = AtMostOne::new("crate_root", ErrorLocation::OnNamedStruct);

    let attrs = attrs
        .into_iter()
        .flat_map(|attr| match attr {
            SnafuAttribute::CrateRoot(tokens, root) => {
                crate_roots.add(root, tokens);
                None
            }
            other => Some(other),
        })
        .collect();

    let field_container = field_container(
        name,
        span,
        attrs,
        fields,
        &mut errors,
        ErrorLocation::OnNamedStruct,
        ErrorLocation::InNamedStruct,
    )?;

    let (maybe_crate_root, errs) = crate_roots.finish();
    let crate_root = maybe_crate_root.unwrap_or_else(default_crate_root);
    errors.extend(errs);

    errors.finish()?;

    Ok(NamedStructInfo {
        crate_root,
        field_container,
        generics,
    })
}

fn parse_snafu_tuple_struct(
    mut fields: syn::FieldsUnnamed,
    name: syn::Ident,
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    span: proc_macro2::Span,
) -> MultiSynResult<TupleStructInfo> {
    let mut transformations = AtMostOne::new("source(from)", ErrorLocation::OnTupleStruct);
    let mut crate_roots = AtMostOne::new("crate_root", ErrorLocation::OnTupleStruct);
    let mut provides = Vec::new();

    let mut errors = SyntaxErrors::default();
    let mut struct_errors = errors.scoped(ErrorLocation::OnTupleStruct);

    for attr in attributes_from_syn(attrs)? {
        use SnafuAttribute as Att;

        match attr {
            Att::Module(tokens, ..) => struct_errors.add(tokens, ATTR_MODULE),
            Att::Provide(tokens, ProvideKind::Flag(..)) => {
                struct_errors.add(tokens, ATTR_PROVIDE_FLAG)
            }
            Att::Provide(_tokens, ProvideKind::Expression(provide)) => {
                provides.push(provide);
            }
            Att::Display(tokens, ..) => struct_errors.add(tokens, ATTR_DISPLAY),
            Att::Visibility(tokens, ..) => struct_errors.add(tokens, ATTR_VISIBILITY),
            Att::Source(tokens, ss) => {
                for s in ss {
                    match s {
                        Source::Flag(..) => struct_errors.add(tokens.clone(), ATTR_SOURCE_BOOL),
                        Source::From(t, e) => transformations.add((t, e), tokens.clone()),
                    }
                }
            }
            Att::Backtrace(tokens, ..) => struct_errors.add(tokens, ATTR_BACKTRACE),
            Att::Implicit(tokens, ..) => struct_errors.add(tokens, ATTR_IMPLICIT),
            Att::Context(tokens, ..) => struct_errors.add(tokens, ATTR_CONTEXT),
            Att::Whatever(tokens) => struct_errors.add(tokens, ATTR_WHATEVER),
            Att::Transparent(tokens, ..) => struct_errors.add(tokens, ATTR_TRANSPARENT),
            Att::CrateRoot(tokens, root) => crate_roots.add(root, tokens),
            Att::DocComment(..) => { /* Just a regular doc comment. */ }
        }
    }

    fn one_field_error(span: proc_macro2::Span) -> syn::Error {
        syn::Error::new(
            span,
            "Can only derive `Snafu` for tuple structs with exactly one field",
        )
    }

    let inner = fields
        .unnamed
        .pop()
        .ok_or_else(|| vec![one_field_error(span)])?;
    if !fields.unnamed.is_empty() {
        return Err(vec![one_field_error(span)]);
    }

    let inner = inner.into_value();

    let mut field_errors = errors.scoped(ErrorLocation::OnTupleStructField);

    for attr in attributes_from_syn(inner.attrs)? {
        use SnafuAttribute as Att;

        match attr {
            Att::Backtrace(tokens, ..) => field_errors.add(tokens, ATTR_BACKTRACE),
            Att::Context(tokens, ..) => field_errors.add(tokens, ATTR_CONTEXT),
            Att::CrateRoot(tokens, ..) => field_errors.add(tokens, ATTR_CRATE_ROOT),
            Att::Display(tokens, ..) => field_errors.add(tokens, ATTR_DISPLAY),
            Att::Implicit(tokens, ..) => field_errors.add(tokens, ATTR_IMPLICIT),
            Att::Module(tokens, ..) => field_errors.add(tokens, ATTR_MODULE),
            Att::Provide(tokens, ..) => field_errors.add(tokens, ATTR_PROVIDE_FLAG),
            Att::Source(tokens, ..) => field_errors.add(tokens.clone(), ATTR_SOURCE),
            Att::Transparent(tokens, ..) => field_errors.add(tokens, ATTR_TRANSPARENT),
            Att::Visibility(tokens, ..) => field_errors.add(tokens, ATTR_VISIBILITY),
            Att::Whatever(tokens) => field_errors.add(tokens, ATTR_WHATEVER),
            Att::DocComment(..) => { /* Just a regular doc comment. */ }
        }
    }

    let ty = inner.ty;
    let (maybe_transformation, errs) = transformations.finish();
    let transformation = maybe_transformation
        .map(|(source_ty, expr)| Transformation::Transform {
            source_ty,
            target_ty: ty.clone(),
            expr,
        })
        .unwrap_or_else(|| Transformation::None { ty });
    errors.extend(errs);

    let (maybe_crate_root, errs) = crate_roots.finish();
    let crate_root = maybe_crate_root.unwrap_or_else(default_crate_root);
    errors.extend(errs);

    errors.finish()?;

    Ok(TupleStructInfo {
        crate_root,
        name,
        generics,
        transformation,
        provides,
    })
}

enum Context {
    Flag(bool),
    Name(syn::Ident),
    Suffix(SuffixKind),
}

impl Context {
    fn into_enabled(self) -> (bool, ContextSelectorName) {
        match self {
            Context::Flag(b) => (b, ContextSelectorName::Suffixed(SuffixKind::None)),
            Context::Name(name) => (true, ContextSelectorName::Provided(name)),
            Context::Suffix(suffix) => (true, ContextSelectorName::Suffixed(suffix)),
        }
    }
}

enum Source {
    Flag(bool),
    From(syn::Type, syn::Expr),
}

struct Display {
    exprs: Vec<syn::Expr>,
    shorthand_names: BTreeSet<syn::Ident>,
    assigned_names: BTreeSet<syn::Ident>,
}

#[derive(Default)]
struct DocComment {
    content: String,
    shorthand_names: BTreeSet<syn::Ident>,
}

impl DocComment {
    fn push_str(&mut self, s: &str) {
        if !self.content.is_empty() {
            self.content.push(' ');
        }
        self.content.push_str(s);
    }

    fn finish(mut self) -> Option<Self> {
        if self.content.is_empty() {
            None
        } else {
            self.shorthand_names.extend(
                crate::parse::extract_field_names(&self.content)
                    .map(|n| quote::format_ident!("{}", n)),
            );

            Some(self)
        }
    }
}

/// A SnafuAttribute represents one SNAFU-specific attribute inside of `#[snafu(...)]`.  For
/// example, in `#[snafu(visibility(pub), display("hi"))]`, `visibility(pub)` and `display("hi")`
/// are each a SnafuAttribute.
///
/// We store the location in the source where we found the attribute (as a `TokenStream`) along
/// with the data.  The location can be used to give accurate error messages in case there was a
/// problem with the use of the attribute.
enum SnafuAttribute {
    Backtrace(proc_macro2::TokenStream, bool),
    Context(proc_macro2::TokenStream, Context),
    CrateRoot(proc_macro2::TokenStream, UserInput),
    Display(proc_macro2::TokenStream, Display),
    DocComment(proc_macro2::TokenStream, String),
    Implicit(proc_macro2::TokenStream, bool),
    Module(proc_macro2::TokenStream, ModuleName),
    Provide(proc_macro2::TokenStream, ProvideKind),
    Source(proc_macro2::TokenStream, Vec<Source>),
    Transparent(proc_macro2::TokenStream, bool),
    Visibility(proc_macro2::TokenStream, UserInput),
    Whatever(proc_macro2::TokenStream),
}

fn default_crate_root() -> UserInput {
    Box::new(quote! { ::snafu })
}

fn private_visibility() -> UserInput {
    Box::new(quote! {})
}

// Private context selectors wouldn't be accessible outside the
// module, so we use `pub(super)`.
fn default_context_selector_visibility_in_module() -> proc_macro2::TokenStream {
    quote! { pub(super) }
}

impl From<SnafuInfo> for proc_macro::TokenStream {
    fn from(other: SnafuInfo) -> proc_macro::TokenStream {
        match other {
            SnafuInfo::Enum(e) => e.into(),
            SnafuInfo::NamedStruct(s) => s.into(),
            SnafuInfo::TupleStruct(s) => s.into(),
        }
    }
}

impl From<EnumInfo> for proc_macro::TokenStream {
    fn from(other: EnumInfo) -> proc_macro::TokenStream {
        other.generate_snafu().into()
    }
}

impl From<NamedStructInfo> for proc_macro::TokenStream {
    fn from(other: NamedStructInfo) -> proc_macro::TokenStream {
        other.generate_snafu().into()
    }
}

impl From<TupleStructInfo> for proc_macro::TokenStream {
    fn from(other: TupleStructInfo) -> proc_macro::TokenStream {
        other.generate_snafu().into()
    }
}

trait GenericAwareNames {
    fn name(&self) -> &syn::Ident;

    fn generics(&self) -> &syn::Generics;

    fn parameterized_name(&self) -> UserInput {
        let enum_name = self.name();
        let original_generics = self.provided_generic_names();

        Box::new(quote! { #enum_name<#(#original_generics,)*> })
    }

    fn provided_generic_types_without_defaults(&self) -> Vec<proc_macro2::TokenStream> {
        use syn::TypeParam;

        self.generics()
            .type_params()
            .map(|t| {
                let TypeParam {
                    attrs,
                    ident,
                    colon_token,
                    bounds,
                    ..
                } = t;
                quote! {
                    #(#attrs)*
                    #ident
                    #colon_token
                    #bounds
                }
            })
            .collect()
    }

    fn provided_generics_without_defaults(&self) -> Vec<proc_macro2::TokenStream> {
        self.provided_generic_lifetimes()
            .into_iter()
            .chain(self.provided_generic_types_without_defaults())
            .chain(self.provided_generic_consts_without_defaults())
            .collect()
    }

    fn provided_generic_lifetimes(&self) -> Vec<proc_macro2::TokenStream> {
        use syn::LifetimeParam;

        self.generics()
            .lifetimes()
            .map(|l| {
                let LifetimeParam {
                    attrs, lifetime, ..
                } = l;
                quote! {
                    #(#attrs)*
                    #lifetime
                }
            })
            .collect()
    }

    fn provided_generic_consts_without_defaults(&self) -> Vec<proc_macro2::TokenStream> {
        self.generics()
            .const_params()
            .map(|c| {
                let syn::ConstParam {
                    attrs,
                    const_token,
                    ident,
                    colon_token,
                    ty,
                    ..
                } = c;
                quote! {
                    #(#attrs)*
                    #const_token
                    #ident
                    #colon_token
                    #ty
                }
            })
            .collect()
    }

    fn provided_generic_names(&self) -> Vec<proc_macro2::TokenStream> {
        use syn::{ConstParam, GenericParam, LifetimeParam, TypeParam};

        self.generics()
            .params
            .iter()
            .map(|p| match p {
                GenericParam::Type(TypeParam { ident, .. }) => quote! { #ident },
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => quote! { #lifetime },
                GenericParam::Const(ConstParam { ident, .. }) => quote! { #ident },
            })
            .collect()
    }

    fn provided_where_clauses(&self) -> Vec<proc_macro2::TokenStream> {
        self.generics()
            .where_clause
            .iter()
            .flat_map(|c| c.predicates.iter().map(|p| quote! { #p }))
            .collect()
    }
}

impl EnumInfo {
    fn generate_snafu(self) -> proc_macro2::TokenStream {
        let context_selectors = ContextSelectors(&self);
        let display_impl = DisplayImpl(&self);
        let error_impl = ErrorImpl(&self);
        let error_compat_impl = ErrorCompatImpl(&self);

        let context = match &self.module {
            None => quote! { #context_selectors },
            Some(module_name) => {
                use crate::shared::ContextModule;

                let context_module = ContextModule {
                    container_name: self.name(),
                    body: &context_selectors,
                    visibility: Some(&self.default_visibility),
                    module_name,
                };

                quote! { #context_module }
            }
        };

        quote! {
            #context
            #display_impl
            #error_impl
            #error_compat_impl
        }
    }
}

impl GenericAwareNames for EnumInfo {
    fn name(&self) -> &syn::Ident {
        &self.name
    }

    fn generics(&self) -> &syn::Generics {
        &self.generics
    }
}

struct ContextSelectors<'a>(&'a EnumInfo);

impl<'a> quote::ToTokens for ContextSelectors<'a> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let context_selectors = self
            .0
            .variants
            .iter()
            .map(|variant| ContextSelector(self.0, variant));

        stream.extend({
            quote! {
                #(#context_selectors)*
            }
        })
    }
}

struct ContextSelector<'a>(&'a EnumInfo, &'a FieldContainer);

impl<'a> quote::ToTokens for ContextSelector<'a> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        use crate::shared::ContextSelector;

        let enum_name = &self.0.name;
        let default_suffix = &self.0.default_suffix;

        let FieldContainer {
            name: variant_name,
            selector_kind,
            ..
        } = self.1;

        let default_visibility;
        let selector_visibility = match (
            &self.1.visibility,
            &self.0.default_visibility,
            &self.0.module,
        ) {
            (Some(v), _, _) | (_, Some(v), _) => Some(&**v),
            (None, None, Some(_)) => {
                default_visibility = default_context_selector_visibility_in_module();
                Some(&default_visibility as _)
            }
            (None, None, None) => None,
        };

        let selector_doc_string = format!(
            "SNAFU context selector for the `{}::{}` variant",
            enum_name, variant_name,
        );

        let context_selector = ContextSelector {
            backtrace_field: self.1.backtrace_field.as_ref(),
            implicit_fields: &self.1.implicit_fields,
            crate_root: &self.0.crate_root,
            error_constructor_name: &quote! { #enum_name::#variant_name },
            original_generics_without_defaults: &self.0.provided_generics_without_defaults(),
            parameterized_error_name: &self.0.parameterized_name(),
            selector_doc_string: &selector_doc_string,
            selector_kind,
            selector_base_name: variant_name,
            user_fields: selector_kind.user_fields(),
            visibility: selector_visibility,
            where_clauses: &self.0.provided_where_clauses(),
            default_suffix,
        };

        stream.extend(quote! { #context_selector });
    }
}

struct DisplayImpl<'a>(&'a EnumInfo);

impl<'a> quote::ToTokens for DisplayImpl<'a> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        use self::shared::{Display, DisplayMatchArm};

        let enum_name = &self.0.name;

        let arms: Vec<_> = self
            .0
            .variants
            .iter()
            .map(|variant| {
                let FieldContainer {
                    display_format,
                    doc_comment,
                    name: variant_name,
                    selector_kind,
                    ..
                } = variant;

                let arm = DisplayMatchArm {
                    field_container: variant,
                    default_name: &variant_name,
                    display_format: display_format.as_ref(),
                    doc_comment: doc_comment.as_ref(),
                    pattern_ident: &quote! { #enum_name::#variant_name },
                    selector_kind,
                };

                quote! { #arm }
            })
            .collect();

        let display = Display {
            arms: &arms,
            original_generics: &self.0.provided_generics_without_defaults(),
            parameterized_error_name: &self.0.parameterized_name(),
            where_clauses: &self.0.provided_where_clauses(),
        };

        let display_impl = quote! { #display };

        stream.extend(display_impl)
    }
}

struct ErrorImpl<'a>(&'a EnumInfo);

impl<'a> quote::ToTokens for ErrorImpl<'a> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        use self::shared::{Error, ErrorProvideMatchArm, ErrorSourceMatchArm};

        let crate_root = &self.0.crate_root;

        let mut variants_to_description = Vec::with_capacity(self.0.variants.len());
        let mut variants_to_source = Vec::with_capacity(self.0.variants.len());
        let mut variants_to_provide = Vec::with_capacity(self.0.variants.len());

        for field_container in &self.0.variants {
            let enum_name = &self.0.name;
            let variant_name = &field_container.name;
            let pattern_ident = &quote! { #enum_name::#variant_name };

            let error_description_match_arm = quote! {
                #pattern_ident { .. } => stringify!(#pattern_ident),
            };

            let error_source_match_arm = ErrorSourceMatchArm {
                field_container,
                pattern_ident,
            };
            let error_source_match_arm = quote! { #error_source_match_arm };

            let error_provide_match_arm = ErrorProvideMatchArm {
                crate_root,
                field_container,
                pattern_ident,
            };
            let error_provide_match_arm = quote! { #error_provide_match_arm };

            variants_to_description.push(error_description_match_arm);
            variants_to_source.push(error_source_match_arm);
            variants_to_provide.push(error_provide_match_arm);
        }

        let error_impl = Error {
            crate_root,
            parameterized_error_name: &self.0.parameterized_name(),
            description_arms: &variants_to_description,
            source_arms: &variants_to_source,
            original_generics: &self.0.provided_generics_without_defaults(),
            where_clauses: &self.0.provided_where_clauses(),
            provide_arms: &variants_to_provide,
        };
        let error_impl = quote! { #error_impl };

        stream.extend(error_impl);
    }
}

struct ErrorCompatImpl<'a>(&'a EnumInfo);

impl<'a> quote::ToTokens for ErrorCompatImpl<'a> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        use self::shared::{ErrorCompat, ErrorCompatBacktraceMatchArm};

        let variants_to_backtrace: Vec<_> = self
            .0
            .variants
            .iter()
            .map(|field_container| {
                let crate_root = &self.0.crate_root;
                let enum_name = &self.0.name;
                let variant_name = &field_container.name;

                let match_arm = ErrorCompatBacktraceMatchArm {
                    field_container,
                    crate_root,
                    pattern_ident: &quote! { #enum_name::#variant_name },
                };

                quote! { #match_arm }
            })
            .collect();

        let error_compat_impl = ErrorCompat {
            crate_root: &self.0.crate_root,
            parameterized_error_name: &self.0.parameterized_name(),
            backtrace_arms: &variants_to_backtrace,
            original_generics: &self.0.provided_generics_without_defaults(),
            where_clauses: &self.0.provided_where_clauses(),
        };

        let error_compat_impl = quote! { #error_compat_impl };

        stream.extend(error_compat_impl);
    }
}

impl NamedStructInfo {
    fn generate_snafu(self) -> proc_macro2::TokenStream {
        let parameterized_struct_name = self.parameterized_name();
        let original_generics = self.provided_generics_without_defaults();
        let where_clauses = self.provided_where_clauses();

        let Self {
            crate_root,
            field_container:
                FieldContainer {
                    name,
                    selector_kind,
                    backtrace_field,
                    implicit_fields,
                    display_format,
                    doc_comment,
                    visibility,
                    module,
                    ..
                },
            ..
        } = &self;
        let field_container = &self.field_container;

        let user_fields = selector_kind.user_fields();

        use crate::shared::{Error, ErrorProvideMatchArm, ErrorSourceMatchArm};

        let pattern_ident = &quote! { Self };

        let error_description_match_arm = quote! {
            #pattern_ident { .. } => stringify!(#name),
        };

        let error_source_match_arm = ErrorSourceMatchArm {
            field_container,
            pattern_ident,
        };
        let error_source_match_arm = quote! { #error_source_match_arm };

        let error_provide_match_arm = ErrorProvideMatchArm {
            crate_root: &crate_root,
            field_container,
            pattern_ident,
        };
        let error_provide_match_arm = quote! { #error_provide_match_arm };

        let error_impl = Error {
            crate_root: &crate_root,
            description_arms: &[error_description_match_arm],
            original_generics: &original_generics,
            parameterized_error_name: &parameterized_struct_name,
            provide_arms: &[error_provide_match_arm],
            source_arms: &[error_source_match_arm],
            where_clauses: &where_clauses,
        };
        let error_impl = quote! { #error_impl };

        use self::shared::{ErrorCompat, ErrorCompatBacktraceMatchArm};

        let match_arm = ErrorCompatBacktraceMatchArm {
            field_container,
            crate_root: &crate_root,
            pattern_ident: &quote! { Self },
        };
        let match_arm = quote! { #match_arm };

        let error_compat_impl = ErrorCompat {
            crate_root: &crate_root,
            parameterized_error_name: &parameterized_struct_name,
            backtrace_arms: &[match_arm],
            original_generics: &original_generics,
            where_clauses: &where_clauses,
        };

        use crate::shared::{Display, DisplayMatchArm};

        let arm = DisplayMatchArm {
            field_container,
            default_name: &name,
            display_format: display_format.as_ref(),
            doc_comment: doc_comment.as_ref(),
            pattern_ident: &quote! { Self },
            selector_kind,
        };
        let arm = quote! { #arm };

        let display_impl = Display {
            arms: &[arm],
            original_generics: &original_generics,
            parameterized_error_name: &parameterized_struct_name,
            where_clauses: &where_clauses,
        };

        use crate::shared::ContextSelector;

        let selector_doc_string = format!("SNAFU context selector for the `{}` error", name);

        let default_visibility;
        let selector_visibility = match (visibility, module) {
            (Some(v), _) => Some(&**v),
            (None, Some(_)) => {
                default_visibility = default_context_selector_visibility_in_module();
                Some(&default_visibility as _)
            }
            (None, None) => None,
        };

        let context_selector = ContextSelector {
            backtrace_field: backtrace_field.as_ref(),
            implicit_fields,
            crate_root: &crate_root,
            error_constructor_name: &name,
            original_generics_without_defaults: &original_generics,
            parameterized_error_name: &parameterized_struct_name,
            selector_doc_string: &selector_doc_string,
            selector_kind,
            selector_base_name: &field_container.name,
            user_fields,
            visibility: selector_visibility,
            where_clauses: &where_clauses,
            default_suffix: &SuffixKind::Default,
        };

        let context = match module {
            None => quote! { #context_selector },
            Some(module_name) => {
                use crate::shared::ContextModule;

                let context_module = ContextModule {
                    container_name: self.name(),
                    body: &context_selector,
                    visibility: visibility.as_ref().map(|x| &**x),
                    module_name,
                };

                quote! { #context_module }
            }
        };

        quote! {
            #error_impl
            #error_compat_impl
            #display_impl
            #context
        }
    }
}

impl GenericAwareNames for NamedStructInfo {
    fn name(&self) -> &syn::Ident {
        &self.field_container.name
    }

    fn generics(&self) -> &syn::Generics {
        &self.generics
    }
}

impl TupleStructInfo {
    fn generate_snafu(self) -> proc_macro2::TokenStream {
        let parameterized_struct_name = self.parameterized_name();

        let TupleStructInfo {
            crate_root,
            generics,
            name,
            transformation,
            provides,
        } = self;

        let inner_type = transformation.source_ty();
        let transformation = transformation.transformation();

        let where_clauses: Vec<_> = generics
            .where_clause
            .iter()
            .flat_map(|c| c.predicates.iter().map(|p| quote! { #p }))
            .collect();

        let description_fn = quote! {
            fn description(&self) -> &str {
                #crate_root::Error::description(&self.0)
            }
        };

        let cause_fn = quote! {
            fn cause(&self) -> ::core::option::Option<&dyn #crate_root::Error> {
                #crate_root::Error::cause(&self.0)
            }
        };

        let source_fn = quote! {
            fn source(&self) -> ::core::option::Option<&(dyn #crate_root::Error + 'static)> {
                #crate_root::Error::source(&self.0)
            }
        };

        let backtrace_fn = quote! {
            fn backtrace(&self) -> ::core::option::Option<&#crate_root::Backtrace> {
                #crate_root::ErrorCompat::backtrace(&self.0)
            }
        };

        let provide_fn = if cfg!(feature = "unstable-provider-api") {
            use shared::error::PROVIDE_ARG;

            let provides = shared::error::enhance_provider_list(&provides);
            let cached_expressions = shared::error::quote_cached_expressions(&provides);
            let user_chained = shared::error::quote_chained(&crate_root, &provides);

            let (hi_explicit_calls, lo_explicit_calls) =
                shared::error::build_explicit_provide_calls(&provides);

            Some(quote! {
                fn provide<'a>(&'a self, #PROVIDE_ARG: &mut #crate_root::error::Request<'a>) {
                    match self {
                        Self(v) => {
                            #(#cached_expressions;)*
                            #(#hi_explicit_calls;)*
                            v.provide(#PROVIDE_ARG);
                            #(#user_chained;)*
                            #(#lo_explicit_calls;)*
                        }
                    };
                }
            })
        } else {
            None
        };

        let error_impl = quote! {
            #[allow(single_use_lifetimes)]
            impl#generics #crate_root::Error for #parameterized_struct_name
            where
                #(#where_clauses),*
            {
                #description_fn
                #cause_fn
                #source_fn
                #provide_fn
            }
        };

        let error_compat_impl = quote! {
            #[allow(single_use_lifetimes)]
            impl#generics #crate_root::ErrorCompat for #parameterized_struct_name
            where
                #(#where_clauses),*
            {
                #backtrace_fn
            }
        };

        let display_impl = quote! {
            #[allow(single_use_lifetimes)]
            impl#generics ::core::fmt::Display for #parameterized_struct_name
            where
                #(#where_clauses),*
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Display::fmt(&self.0, f)
                }
            }
        };

        let from_impl = quote! {
            impl#generics ::core::convert::From<#inner_type> for #parameterized_struct_name
            where
                #(#where_clauses),*
            {
                fn from(other: #inner_type) -> Self {
                    #name((#transformation)(other))
                }
            }
        };

        quote! {
            #error_impl
            #error_compat_impl
            #display_impl
            #from_impl
        }
    }
}

impl GenericAwareNames for TupleStructInfo {
    fn name(&self) -> &syn::Ident {
        &self.name
    }

    fn generics(&self) -> &syn::Generics {
        &self.generics
    }
}

mod sponge {
    use std::iter::FromIterator;

    pub struct AllErrors<T, E>(Result<T, Vec<E>>);

    impl<T, E> AllErrors<T, E> {
        pub fn into_result(self) -> Result<T, Vec<E>> {
            self.0
        }
    }

    impl<C, T, E> FromIterator<Result<C, E>> for AllErrors<T, E>
    where
        T: FromIterator<C>,
    {
        fn from_iter<I>(i: I) -> Self
        where
            I: IntoIterator<Item = Result<C, E>>,
        {
            let mut errors = Vec::new();

            let inner = i
                .into_iter()
                .flat_map(|v| match v {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        errors.push(e);
                        Err(())
                    }
                })
                .collect();

            if errors.is_empty() {
                AllErrors(Ok(inner))
            } else {
                AllErrors(Err(errors))
            }
        }
    }

    impl<C, T, E> FromIterator<Result<C, Vec<E>>> for AllErrors<T, E>
    where
        T: FromIterator<C>,
    {
        fn from_iter<I>(i: I) -> Self
        where
            I: IntoIterator<Item = Result<C, Vec<E>>>,
        {
            let mut errors = Vec::new();

            let inner = i
                .into_iter()
                .flat_map(|v| match v {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        errors.extend(e);
                        Err(())
                    }
                })
                .collect();

            if errors.is_empty() {
                AllErrors(Ok(inner))
            } else {
                AllErrors(Err(errors))
            }
        }
    }
}
