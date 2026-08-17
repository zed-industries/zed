use std::{borrow::Cow, vec::IntoIter};

use crate::BuildMethod;

use darling::util::{Flag, PathList};
use darling::{self, Error, FromMeta};
use proc_macro2::{Span, TokenStream};
use syn::parse::{ParseStream, Parser};
use syn::Meta;
use syn::{self, spanned::Spanned, Attribute, Generics, Ident, Path};

use crate::{
    BlockContents, Builder, BuilderField, BuilderFieldType, BuilderPattern, DefaultExpression,
    DeprecationNotes, Each, FieldConversion, Initializer, Setter,
};

/// `derive_builder` uses separate sibling keywords to represent
/// mutually-exclusive visibility states. This trait requires implementers to
/// expose those property values and provides a method to compute any explicit visibility
/// bounds.
trait Visibility {
    fn public(&self) -> &Flag;
    fn private(&self) -> &Flag;
    fn explicit(&self) -> Option<&syn::Visibility>;

    /// Get the explicitly-expressed visibility preference from the attribute.
    /// This returns `None` if the input didn't include either keyword.
    ///
    /// # Panics
    /// This method panics if the input specifies both `public` and `private`.
    fn as_expressed_vis(&self) -> Option<Cow<syn::Visibility>> {
        let declares_public = self.public().is_present();
        let declares_private = self.private().is_present();
        let declares_explicit = self.explicit().is_some();

        if declares_private {
            assert!(!declares_public && !declares_explicit);
            Some(Cow::Owned(syn::Visibility::Inherited))
        } else if let Some(vis) = self.explicit() {
            assert!(!declares_public);
            Some(Cow::Borrowed(vis))
        } else if declares_public {
            Some(Cow::Owned(syn::parse_quote!(pub)))
        } else {
            None
        }
    }
}

fn no_visibility_conflict<T: Visibility>(v: &T) -> darling::Result<()> {
    let declares_public = v.public().is_present();
    let declares_private = v.private().is_present();
    if let Some(vis) = v.explicit() {
        if declares_public || declares_private {
            Err(
                Error::custom(r#"`vis="..."` cannot be used with `public` or `private`"#)
                    .with_span(vis),
            )
        } else {
            Ok(())
        }
    } else if declares_public && declares_private {
        Err(
            Error::custom(r#"`public` and `private` cannot be used together"#)
                .with_span(v.public()),
        )
    } else {
        Ok(())
    }
}

/// Options for the `build_fn` property in struct-level builder options.
/// There is no inheritance for these settings from struct-level to field-level,
/// so we don't bother using `Option` for values in this struct.
#[derive(Debug, Clone, FromMeta)]
#[darling(default)]
pub struct BuildFn {
    skip: bool,
    name: Ident,
    validate: Option<Path>,
    public: Flag,
    private: Flag,
    vis: Option<syn::Visibility>,
    /// The path to an existing error type that the build method should return.
    ///
    /// Setting this will prevent `derive_builder` from generating an error type for the build
    /// method.
    ///
    /// # Type Bounds
    /// This type's bounds depend on other settings of the builder.
    ///
    /// * If uninitialized fields cause `build()` to fail, then this type
    ///   must `impl From<UninitializedFieldError>`. Uninitialized fields do not cause errors
    ///   when default values are provided for every field or at the struct level.
    /// * If `validate` is specified, then this type must provide a conversion from the specified
    ///   function's error type.
    error: Option<Path>,
}

impl Default for BuildFn {
    fn default() -> Self {
        BuildFn {
            skip: false,
            name: Ident::new("build", Span::call_site()),
            validate: None,
            public: Default::default(),
            private: Default::default(),
            vis: None,
            error: None,
        }
    }
}

impl Visibility for BuildFn {
    fn public(&self) -> &Flag {
        &self.public
    }

    fn private(&self) -> &Flag {
        &self.private
    }

    fn explicit(&self) -> Option<&syn::Visibility> {
        self.vis.as_ref()
    }
}

/// Contents of the `field` meta in `builder` attributes at the struct level.
#[derive(Debug, Clone, Default, FromMeta)]
pub struct StructLevelFieldMeta {
    public: Flag,
    private: Flag,
    vis: Option<syn::Visibility>,
}

impl Visibility for StructLevelFieldMeta {
    fn public(&self) -> &Flag {
        &self.public
    }

    fn private(&self) -> &Flag {
        &self.private
    }

    fn explicit(&self) -> Option<&syn::Visibility> {
        self.vis.as_ref()
    }
}

/// Contents of the `field` meta in `builder` attributes at the field level.
//
// This is a superset of the attributes permitted in `field` at the struct level.
// Perhaps in the future we will be able to use `#[darling(flatten)]`, but
// that does not exist right now: https://github.com/TedDriggs/darling/issues/146
#[derive(Debug, Clone, Default, FromMeta)]
pub struct FieldLevelFieldMeta {
    public: Flag,
    private: Flag,
    vis: Option<syn::Visibility>,
    /// Custom builder field type
    #[darling(rename = "type")]
    builder_type: Option<syn::Type>,
    /// Custom builder field method, for making target struct field value
    build: Option<BlockContents>,
}

impl Visibility for FieldLevelFieldMeta {
    fn public(&self) -> &Flag {
        &self.public
    }

    fn private(&self) -> &Flag {
        &self.private
    }

    fn explicit(&self) -> Option<&syn::Visibility> {
        self.vis.as_ref()
    }
}

#[derive(Debug, Clone, Default, FromMeta)]
pub struct StructLevelSetter {
    prefix: Option<Ident>,
    into: Option<bool>,
    strip_option: Option<bool>,
    skip: Option<bool>,
}

impl StructLevelSetter {
    /// Check if setters are explicitly enabled or disabled at
    /// the struct level.
    pub fn enabled(&self) -> Option<bool> {
        self.skip.map(|x| !x)
    }
}

/// Create `Each` from an attribute's `Meta`.
///
/// Two formats are supported:
///
/// * `each = "..."`, which provides the name of the `each` setter and otherwise uses default values
/// * `each(name = "...")`, which allows setting additional options on the `each` setter
fn parse_each(meta: &Meta) -> darling::Result<Option<Each>> {
    if let Meta::NameValue(mnv) = meta {
        if let syn::Lit::Str(v) = &mnv.lit {
            v.parse::<Ident>()
                .map(Each::from)
                .map(Some)
                .map_err(|_| darling::Error::unknown_value(&v.value()).with_span(v))
        } else {
            Err(darling::Error::unexpected_lit_type(&mnv.lit))
        }
    } else {
        Each::from_meta(meta).map(Some)
    }
}

/// The `setter` meta item on fields in the input type.
/// Unlike the `setter` meta item at the struct level, this allows specific
/// name overrides.
#[derive(Debug, Clone, Default, FromMeta)]
pub struct FieldLevelSetter {
    prefix: Option<Ident>,
    name: Option<Ident>,
    into: Option<bool>,
    strip_option: Option<bool>,
    skip: Option<bool>,
    custom: Option<bool>,
    #[darling(with = "parse_each")]
    each: Option<Each>,
}

impl FieldLevelSetter {
    /// Get whether the setter should be emitted. The rules are the same as
    /// for `field_enabled`, except we only skip the setter if `setter(custom)` is present.
    pub fn setter_enabled(&self) -> Option<bool> {
        if self.custom.is_some() {
            return self.custom.map(|x| !x);
        }

        self.field_enabled()
    }

    /// Get whether or not this field-level setter indicates a setter and
    /// field should be emitted. The setter shorthand rules are that the
    /// presence of a `setter` with _any_ properties set forces the setter
    /// to be emitted.
    pub fn field_enabled(&self) -> Option<bool> {
        if self.skip.is_some() {
            return self.skip.map(|x| !x);
        }

        if self.prefix.is_some()
            || self.name.is_some()
            || self.into.is_some()
            || self.strip_option.is_some()
            || self.each.is_some()
        {
            return Some(true);
        }

        None
    }
}

/// `derive_builder` allows the calling code to use `setter` as a word to enable
/// setters when they've been disabled at the struct level.
fn field_setter(meta: &Meta) -> darling::Result<FieldLevelSetter> {
    // it doesn't matter what the path is; the fact that this function
    // has been called means that a valueless path is the shorthand case.
    if let Meta::Path(_) = meta {
        Ok(FieldLevelSetter {
            skip: Some(false),
            ..Default::default()
        })
    } else {
        FieldLevelSetter::from_meta(meta)
    }
}

/// Data extracted from the fields of the input struct.
#[derive(Debug, Clone, FromField)]
#[darling(
    attributes(builder),
    forward_attrs(doc, cfg, allow, builder_field_attr, builder_setter_attr),
    and_then = "Self::resolve"
)]
pub struct Field {
    ident: Option<Ident>,
    /// Raw input attributes, for consumption by Field::unnest_attrs.  Do not use elsewhere.
    attrs: Vec<syn::Attribute>,
    ty: syn::Type,
    /// Field-level override for builder pattern.
    /// Note that setting this may force the builder to derive `Clone`.
    pattern: Option<BuilderPattern>,
    public: Flag,
    private: Flag,
    /// Declared visibility for the field in the builder, e.g. `#[builder(vis = "...")]`.
    ///
    /// This cannot be named `vis` or `darling` would put the deriving field's visibility into the
    /// field instead.
    #[darling(rename = "vis")]
    visibility: Option<syn::Visibility>,
    // See the documentation for `FieldSetterMeta` to understand how `darling`
    // is interpreting this field.
    #[darling(default, with = "field_setter")]
    setter: FieldLevelSetter,
    /// The value for this field if the setter is never invoked.
    ///
    /// A field can get its default one of three ways:
    ///
    /// 1. An explicit `default = "..."` expression
    /// 2. An explicit `default` word, in which case the field type's `Default::default()`
    ///    value is used
    /// 3. Inherited from the field's value in the struct's `default` value.
    ///
    /// This property only captures the first two, the third is computed in `FieldWithDefaults`.
    default: Option<DefaultExpression>,
    try_setter: Flag,
    #[darling(default)]
    field: FieldLevelFieldMeta,
    #[darling(skip)]
    field_attrs: Vec<Attribute>,
    #[darling(skip)]
    setter_attrs: Vec<Attribute>,
}

impl Field {
    fn no_visibility_conflicts(&self) -> darling::Result<()> {
        let mut errors = Error::accumulator();
        errors.handle(no_visibility_conflict(&self.field));
        errors.handle(no_visibility_conflict(self));
        errors.finish()
    }

    /// Resolve and check (post-parsing) options which come from multiple darling options
    ///
    ///  * Check that we don't have a custom field type or builder *and* a default value
    ///  * Populate `self.field_attrs` and `self.setter_attrs` by draining `self.attrs`
    fn resolve(mut self) -> darling::Result<Self> {
        let mut errors = darling::Error::accumulator();

        // `default` can be preempted by properties in `field`. Silently ignoring a
        // `default` could cause the direct user of `derive_builder` to see unexpected
        // behavior from the builder, so instead we require that the deriving struct
        // not pass any ignored instructions.
        if let Field {
            default: Some(field_default),
            ..
        } = &self
        {
            // `field.build` is stronger than `default`, as it contains both instructions on how to
            // deal with a missing value and conversions to do on the value during target type
            // construction.
            if self.field.build.is_some() {
                errors.push(
                    darling::Error::custom(
                        r#"#[builder(default)] and #[builder(field(build="..."))] cannot be used together"#,
                    )
                    .with_span(field_default),
                );
            }

            // `field.type` being set means `default` will not be used, since we don't know how
            // to check a custom field type for the absence of a value and therefore we'll never
            // know that we should use the `default` value.
            if self.field.builder_type.is_some() {
                errors.push(
                    darling::Error::custom(
                        r#"#[builder(default)] and #[builder(field(type="..."))] cannot be used together"#,
                    )
                    .with_span(field_default)
                )
            }
        };

        errors.handle(distribute_and_unnest_attrs(
            &mut self.attrs,
            &mut [
                ("builder_field_attr", &mut self.field_attrs),
                ("builder_setter_attr", &mut self.setter_attrs),
            ],
        ));

        errors.finish_with(self)
    }
}

/// Divide a list of attributes into multiple partially-overlapping output lists.
///
/// Some attributes from the macro input will be added to the output in multiple places;
/// for example, a `cfg` attribute must be replicated to both the struct and its impl block or
/// the resulting code will not compile.
///
/// Other attributes are scoped to a specific output by their path, e.g. `builder_field_attr`.
/// These attributes will only appear in one output list, but need that outer path removed.
///
/// For performance reasons, we want to do this in one pass through the list instead of
/// first distributing and then iterating through each of the output lists.
///
/// Each item in `outputs` contains the attribute name unique to that output, and the `Vec` where all attributes for that output should be inserted.
/// Attributes whose path matches any value in `outputs` will be added only to the first matching one, and will be "unnested".
/// Other attributes are not unnested, and simply copied for each decoratee.
fn distribute_and_unnest_attrs(
    input: &mut Vec<Attribute>,
    outputs: &mut [(&'static str, &mut Vec<Attribute>)],
) -> darling::Result<()> {
    let mut errors = vec![];

    for (name, list) in &*outputs {
        assert!(list.is_empty(), "Output Vec for '{}' was not empty", name);
    }

    for attr in input.drain(..) {
        let destination = outputs
            .iter_mut()
            .find(|(ptattr, _)| attr.path.is_ident(ptattr));

        if let Some((_, destination)) = destination {
            match unnest_from_one_attribute(attr) {
                Ok(n) => destination.push(n),
                Err(e) => errors.push(e),
            }
        } else {
            for (_, output) in outputs.iter_mut() {
                output.push(attr.clone());
            }
        }
    }

    if !errors.is_empty() {
        return Err(darling::Error::multiple(errors));
    }

    Ok(())
}

fn unnest_from_one_attribute(attr: syn::Attribute) -> darling::Result<Attribute> {
    match &attr.style {
        syn::AttrStyle::Outer => (),
        syn::AttrStyle::Inner(bang) => {
            return Err(darling::Error::unsupported_format(&format!(
                "{} must be an outer attribute",
                attr.path
                    .get_ident()
                    .map(Ident::to_string)
                    .unwrap_or_else(|| "Attribute".to_string())
            ))
            .with_span(bang));
        }
    };

    #[derive(Debug)]
    struct ContainedAttribute(syn::Attribute);
    impl syn::parse::Parse for ContainedAttribute {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            // Strip parentheses, and save the span of the parenthesis token
            let content;
            let paren_token = parenthesized!(content in input);
            let wrap_span = paren_token.span;

            // Wrap up in #[ ] instead.
            let pound = Token![#](wrap_span); // We can't write a literal # inside quote
            let content: TokenStream = content.parse()?;
            let content = quote_spanned!(wrap_span=> #pound [ #content ]);

            let parser = syn::Attribute::parse_outer;
            let mut attrs = parser.parse2(content)?.into_iter();
            // TryFrom for Array not available in Rust 1.40
            // We think this error can never actually happen, since `#[...]` ought to make just one Attribute
            let attr = match (attrs.next(), attrs.next()) {
                (Some(attr), None) => attr,
                _ => return Err(input.error("expected exactly one attribute")),
            };
            Ok(Self(attr))
        }
    }

    let ContainedAttribute(attr) = syn::parse2(attr.tokens)?;
    Ok(attr)
}

impl Visibility for Field {
    fn public(&self) -> &Flag {
        &self.public
    }

    fn private(&self) -> &Flag {
        &self.private
    }

    fn explicit(&self) -> Option<&syn::Visibility> {
        self.visibility.as_ref()
    }
}

fn default_crate_root() -> Path {
    parse_quote!(::derive_builder)
}

fn default_create_empty() -> Ident {
    Ident::new("create_empty", Span::call_site())
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(
    attributes(builder),
    forward_attrs(cfg, allow, builder_struct_attr, builder_impl_attr),
    supports(struct_named),
    and_then = "Self::unnest_attrs"
)]
pub struct Options {
    ident: Ident,

    /// DO NOT USE.
    ///
    /// Initial receiver for forwarded attributes from the struct; these are split
    /// into `Options::struct_attrs` and `Options::impl_attrs` before `FromDeriveInput`
    /// returns.
    attrs: Vec<Attribute>,

    #[darling(skip)]
    struct_attrs: Vec<Attribute>,

    #[darling(skip)]
    impl_attrs: Vec<Attribute>,

    /// The visibility of the deriving struct. Do not confuse this with `#[builder(vis = "...")]`,
    /// which is received by `Options::visibility`.
    vis: syn::Visibility,

    generics: Generics,

    /// The name of the generated builder. Defaults to `#{ident}Builder`.
    name: Option<Ident>,

    /// The path to the root of the derive_builder crate used in generated
    /// code.
    #[darling(rename = "crate", default = "default_crate_root")]
    crate_root: Path,

    #[darling(default)]
    pattern: BuilderPattern,

    #[darling(default)]
    build_fn: BuildFn,

    /// Additional traits to derive on the builder.
    #[darling(default)]
    derive: PathList,

    custom_constructor: Flag,

    /// The ident of the inherent method which takes no arguments and returns
    /// an instance of the builder with all fields empty.
    #[darling(default = "default_create_empty")]
    create_empty: Ident,

    /// Setter options applied to all field setters in the struct.
    #[darling(default)]
    setter: StructLevelSetter,

    /// Struct-level value to use in place of any unfilled fields
    default: Option<DefaultExpression>,

    public: Flag,

    private: Flag,

    /// Desired visibility of the builder struct.
    ///
    /// Do not confuse this with `Options::vis`, which is the visibility of the deriving struct.
    #[darling(rename = "vis")]
    visibility: Option<syn::Visibility>,

    /// The parsed body of the derived struct.
    data: darling::ast::Data<darling::util::Ignored, Field>,

    no_std: Flag,

    /// When present, emit additional fallible setters alongside each regular
    /// setter.
    try_setter: Flag,

    #[darling(default)]
    field: StructLevelFieldMeta,

    #[darling(skip, default)]
    deprecation_notes: DeprecationNotes,
}

impl Visibility for Options {
    fn public(&self) -> &Flag {
        &self.public
    }

    fn private(&self) -> &Flag {
        &self.private
    }

    fn explicit(&self) -> Option<&syn::Visibility> {
        self.visibility.as_ref()
    }
}

impl Options {
    /// Populate `self.struct_attrs` and `self.impl_attrs` by draining `self.attrs`
    fn unnest_attrs(mut self) -> darling::Result<Self> {
        let mut errors = Error::accumulator();

        errors.handle(distribute_and_unnest_attrs(
            &mut self.attrs,
            &mut [
                ("builder_struct_attr", &mut self.struct_attrs),
                ("builder_impl_attr", &mut self.impl_attrs),
            ],
        ));

        // Check for conflicting visibility declarations. These cannot be pushed
        // down into `FieldMeta` et al because of the call to `no_visibility_conflict(&self)`,
        // as all sub-fields must be valid for this `Options` function to run.
        errors.handle(no_visibility_conflict(&self.field));
        errors.handle(no_visibility_conflict(&self.build_fn));
        self.data
            .as_ref()
            .map_struct_fields(|f| errors.handle(f.no_visibility_conflicts()));
        errors.handle(no_visibility_conflict(&self));

        errors.finish_with(self)
    }
}

/// Accessors for parsed properties.
impl Options {
    pub fn builder_ident(&self) -> Ident {
        if let Some(ref custom) = self.name {
            return custom.clone();
        }

        format_ident!("{}Builder", self.ident)
    }

    pub fn builder_error_ident(&self) -> Path {
        if let Some(existing) = self.build_fn.error.as_ref() {
            existing.clone()
        } else if let Some(ref custom) = self.name {
            format_ident!("{}Error", custom).into()
        } else {
            format_ident!("{}BuilderError", self.ident).into()
        }
    }

    /// The visibility of the builder struct.
    /// If a visibility was declared in attributes, that will be used;
    /// otherwise the struct's own visibility will be used.
    pub fn builder_vis(&self) -> Cow<syn::Visibility> {
        self.as_expressed_vis().unwrap_or(Cow::Borrowed(&self.vis))
    }

    /// Get the visibility of the emitted `build` method.
    /// This defaults to the visibility of the parent builder, but can be overridden.
    pub fn build_method_vis(&self) -> Cow<syn::Visibility> {
        self.build_fn
            .as_expressed_vis()
            .unwrap_or_else(|| self.builder_vis())
    }

    pub fn raw_fields(&self) -> Vec<&Field> {
        self.data
            .as_ref()
            .take_struct()
            .expect("Only structs supported")
            .fields
    }

    /// A builder requires `Clone` to be derived if its build method or any of its setters
    /// use the mutable or immutable pattern.
    pub fn requires_clone(&self) -> bool {
        self.pattern.requires_clone() || self.fields().any(|f| f.pattern().requires_clone())
    }

    /// Get an iterator over the input struct's fields which pulls fallback
    /// values from struct-level settings.
    pub fn fields(&self) -> FieldIter {
        FieldIter(self, self.raw_fields().into_iter())
    }

    pub fn field_count(&self) -> usize {
        self.raw_fields().len()
    }
}

/// Converters to codegen structs
impl Options {
    pub fn as_builder(&self) -> Builder {
        Builder {
            crate_root: &self.crate_root,
            enabled: true,
            ident: self.builder_ident(),
            pattern: self.pattern,
            derives: &self.derive,
            struct_attrs: &self.struct_attrs,
            impl_attrs: &self.impl_attrs,
            impl_default: !self.custom_constructor.is_present(),
            create_empty: self.create_empty.clone(),
            generics: Some(&self.generics),
            visibility: self.builder_vis(),
            fields: Vec::with_capacity(self.field_count()),
            field_initializers: Vec::with_capacity(self.field_count()),
            functions: Vec::with_capacity(self.field_count()),
            generate_error: self.build_fn.error.is_none(),
            must_derive_clone: self.requires_clone(),
            doc_comment: None,
            deprecation_notes: Default::default(),
            std: !self.no_std.is_present(),
        }
    }

    pub fn as_build_method(&self) -> BuildMethod {
        let (_, ty_generics, _) = self.generics.split_for_impl();
        BuildMethod {
            crate_root: &self.crate_root,
            enabled: !self.build_fn.skip,
            ident: &self.build_fn.name,
            visibility: self.build_method_vis(),
            pattern: self.pattern,
            target_ty: &self.ident,
            target_ty_generics: Some(ty_generics),
            error_ty: self.builder_error_ident(),
            initializers: Vec::with_capacity(self.field_count()),
            doc_comment: None,
            default_struct: self.default.as_ref(),
            validate_fn: self.build_fn.validate.as_ref(),
        }
    }
}

/// Accessor for field data which can pull through options from the parent
/// struct.
pub struct FieldWithDefaults<'a> {
    parent: &'a Options,
    field: &'a Field,
}

/// Accessors for parsed properties, with transparent pull-through from the
/// parent struct's configuration.
impl<'a> FieldWithDefaults<'a> {
    /// Check if this field should emit a setter.
    pub fn setter_enabled(&self) -> bool {
        self.field
            .setter
            .setter_enabled()
            .or_else(|| self.parent.setter.enabled())
            .unwrap_or(true)
    }

    pub fn field_enabled(&self) -> bool {
        self.field
            .setter
            .field_enabled()
            .or_else(|| self.parent.setter.enabled())
            .unwrap_or(true)
    }

    /// Check if this field should emit a fallible setter.
    /// This depends on the `TryFrom` trait, which hasn't yet stabilized.
    pub fn try_setter(&self) -> bool {
        self.field.try_setter.is_present() || self.parent.try_setter.is_present()
    }

    /// Get the prefix that should be applied to the field name to produce
    /// the setter ident, if any.
    pub fn setter_prefix(&self) -> Option<&Ident> {
        self.field
            .setter
            .prefix
            .as_ref()
            .or(self.parent.setter.prefix.as_ref())
    }

    /// Get the ident of the emitted setter method
    pub fn setter_ident(&self) -> syn::Ident {
        if let Some(ref custom) = self.field.setter.name {
            return custom.clone();
        }

        let ident = &self.field.ident;

        if let Some(ref prefix) = self.setter_prefix() {
            return format_ident!("{}_{}", prefix, ident.as_ref().unwrap());
        }

        ident.clone().unwrap()
    }

    /// Checks if the emitted setter should be generic over types that impl
    /// `Into<FieldType>`.
    pub fn setter_into(&self) -> bool {
        self.field
            .setter
            .into
            .or(self.parent.setter.into)
            .unwrap_or_default()
    }

    /// Checks if the emitted setter should strip the wrapper Option over types that impl
    /// `Option<FieldType>`.
    pub fn setter_strip_option(&self) -> bool {
        self.field
            .setter
            .strip_option
            .or(self.parent.setter.strip_option)
            .unwrap_or_default()
    }

    /// Get the visibility of the emitted setter, if there will be one.
    pub fn setter_vis(&self) -> Cow<syn::Visibility> {
        self.field
            .as_expressed_vis()
            .or_else(|| self.parent.as_expressed_vis())
            .unwrap_or_else(|| Cow::Owned(syn::parse_quote!(pub)))
    }

    /// Get the ident of the input field. This is also used as the ident of the
    /// emitted field.
    pub fn field_ident(&self) -> &syn::Ident {
        self.field
            .ident
            .as_ref()
            .expect("Tuple structs are not supported")
    }

    pub fn field_vis(&self) -> Cow<syn::Visibility> {
        self.field
            .field
            .as_expressed_vis()
            .or_else(
                // Disabled fields become a PhantomData in the builder.  We make that field
                // non-public, even if the rest of the builder is public, since this field is just
                // there to make sure the struct's generics are properly handled.
                || {
                    if self.field_enabled() {
                        None
                    } else {
                        Some(Cow::Owned(syn::Visibility::Inherited))
                    }
                },
            )
            .or_else(|| self.parent.field.as_expressed_vis())
            .unwrap_or(Cow::Owned(syn::Visibility::Inherited))
    }

    pub fn field_type(&'a self) -> BuilderFieldType<'a> {
        if !self.field_enabled() {
            BuilderFieldType::Phantom(&self.field.ty)
        } else if let Some(custom_ty) = self.field.field.builder_type.as_ref() {
            BuilderFieldType::Precise(custom_ty)
        } else {
            BuilderFieldType::Optional(&self.field.ty)
        }
    }

    pub fn conversion(&'a self) -> FieldConversion<'a> {
        match (&self.field.field.builder_type, &self.field.field.build) {
            (_, Some(block)) => FieldConversion::Block(block),
            (Some(_), None) => FieldConversion::Move,
            (None, None) => FieldConversion::OptionOrDefault,
        }
    }

    pub fn pattern(&self) -> BuilderPattern {
        self.field.pattern.unwrap_or(self.parent.pattern)
    }

    pub fn use_parent_default(&self) -> bool {
        self.field.default.is_none() && self.parent.default.is_some()
    }

    pub fn deprecation_notes(&self) -> &DeprecationNotes {
        &self.parent.deprecation_notes
    }
}

/// Converters to codegen structs
impl<'a> FieldWithDefaults<'a> {
    /// Returns a `Setter` according to the options.
    pub fn as_setter(&'a self) -> Setter<'a> {
        Setter {
            crate_root: &self.parent.crate_root,
            setter_enabled: self.setter_enabled(),
            try_setter: self.try_setter(),
            visibility: self.setter_vis(),
            pattern: self.pattern(),
            attrs: &self.field.setter_attrs,
            ident: self.setter_ident(),
            field_ident: self.field_ident(),
            field_type: self.field_type(),
            generic_into: self.setter_into(),
            strip_option: self.setter_strip_option(),
            deprecation_notes: self.deprecation_notes(),
            each: self.field.setter.each.as_ref(),
        }
    }

    /// Returns an `Initializer` according to the options.
    ///
    /// # Panics
    ///
    /// if `default_expression` can not be parsed as `Block`.
    pub fn as_initializer(&'a self) -> Initializer<'a> {
        Initializer {
            crate_root: &self.parent.crate_root,
            field_enabled: self.field_enabled(),
            field_ident: self.field_ident(),
            builder_pattern: self.pattern(),
            default_value: self.field.default.as_ref(),
            use_default_struct: self.use_parent_default(),
            conversion: self.conversion(),
            custom_error_type_span: self
                .parent
                .build_fn
                .error
                .as_ref()
                .map(|err_ty| err_ty.span()),
        }
    }

    pub fn as_builder_field(&'a self) -> BuilderField<'a> {
        BuilderField {
            crate_root: &self.parent.crate_root,
            field_ident: self.field_ident(),
            field_type: self.field_type(),
            field_visibility: self.field_vis(),
            attrs: &self.field.field_attrs,
        }
    }
}

pub struct FieldIter<'a>(&'a Options, IntoIter<&'a Field>);

impl<'a> Iterator for FieldIter<'a> {
    type Item = FieldWithDefaults<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|field| FieldWithDefaults {
            parent: self.0,
            field,
        })
    }
}
