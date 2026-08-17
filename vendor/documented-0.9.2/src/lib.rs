#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

pub use documented_macros::{
    docs_const, Documented, DocumentedFields, DocumentedFieldsOpt, DocumentedOpt,
    DocumentedVariants, DocumentedVariantsOpt,
};

#[doc(hidden)]
pub use phf as _private_phf_reexport_for_macro;

/// Adds an associated constant [`DOCS`](Self::DOCS) on your type containing its
/// documentation, allowing you to access its documentation at runtime.
///
/// The associated derive macro of this trait will error if the type does not
/// have any doc comments. Use [`DocumentedOpt`] if this is undesirable.
///
/// For how to use the derive macro, see [`macro@Documented`].
pub trait Documented {
    /// The static doc comments on this type.
    const DOCS: &'static str;
}

/// The optional variant of [`Documented`].
pub trait DocumentedOpt {
    /// The static doc comments on this type.
    const DOCS: Option<&'static str>;
}

/// Adds an associated constant [`FIELD_DOCS`](Self::FIELD_DOCS) on your type
/// containing the documentation of its fields, allowing you to access their
/// documentation at runtime.
///
/// The associated derive macro of this trait will error if any field or variant
/// does not have any doc comments.
/// Use [`DocumentedFieldsOpt`] if this is undesirable.
///
/// This trait and associated derive macro works on structs, enums, and unions.
/// For enums, you may find [`DocumentedVariants`] more ergonomic to use.
///
/// For how to use the derive macro, see [`macro@DocumentedFields`].
pub trait DocumentedFields {
    /// The static doc comments on each field or variant of this type, indexed
    /// by field/variant order.
    const FIELD_DOCS: &'static [&'static str];
    /// Field names, as accepted by [`Self::get_field_docs`].
    ///
    /// Note that anonymous fields (i.e. fields in tuple structs), unless they
    /// have [a custom name set](macro@DocumentedFields#2-set-a-custom-name-for-a-specific-field-for-get_field_docs-like-so),
    /// will be omitted from `FIELD_NAMES`. This means that in such cases, the
    /// indices of `FIELD_NAMES` will be misaligned with that of `FIELD_DOCS`.
    ///
    /// It is therefore recommended to use [`Self::get_field_docs`] rather than
    /// the index to lookup the corresponding documentation.
    const FIELD_NAMES: &'static [&'static str];

    /// Method internally used by `documented`.
    #[doc(hidden)]
    fn __documented_get_index<T: AsRef<str>>(field_name: T) -> Option<usize>;

    /// Get a field's documentation using its name.
    ///
    /// Note that for structs with anonymous fields (i.e. tuple structs), this
    /// method will always return [`Error::NoSuchField`] by default. For this
    /// case, you can either:
    ///
    /// 1. use [`FIELD_DOCS`](Self::FIELD_DOCS) directly instead;
    /// 2. [set a custom name](macro@DocumentedFields#2-set-a-custom-name-for-a-specific-field-for-get_field_docs-like-so) for the anonymous field.
    fn get_field_docs<T: AsRef<str>>(field_name: T) -> Result<&'static str, Error> {
        let field_name = field_name.as_ref();
        let index = Self::__documented_get_index(field_name)
            .ok_or_else(|| Error::NoSuchField(field_name.into()))?;
        Ok(Self::FIELD_DOCS[index])
    }
}

/// The optional variant of [`DocumentedFields`].
pub trait DocumentedFieldsOpt {
    /// The static doc comments on each field or variant of this type, indexed
    /// by field/variant order.
    const FIELD_DOCS: &'static [Option<&'static str>];
    /// Field names, as accepted by [`Self::get_field_docs`].
    ///
    /// Note that anonymous fields (i.e. fields in tuple structs), unless they
    /// have [a custom name set](macro@DocumentedFields#2-set-a-custom-name-for-a-specific-field-for-get_field_docs-like-so),
    /// will be omitted from `FIELD_NAMES`. This means that in such cases, the
    /// indices of `FIELD_NAMES` will be misaligned with that of `FIELD_DOCS`.
    ///
    /// It is therefore recommended to use [`Self::get_field_docs`] rather than
    /// the index to lookup the corresponding documentation.
    const FIELD_NAMES: &'static [&'static str];

    /// Method internally used by `documented`.
    #[doc(hidden)]
    fn __documented_get_index<T: AsRef<str>>(field_name: T) -> Option<usize>;

    /// Get a field's documentation using its name.
    ///
    /// Note that for structs with anonymous fields (i.e. tuple structs), this
    /// method will always return [`Error::NoSuchField`] by default. For this
    /// case, you can either:
    ///
    /// 1. use [`FIELD_DOCS`](Self::FIELD_DOCS) directly instead;
    /// 2. [set a custom name](macro@DocumentedFields#2-set-a-custom-name-for-a-specific-field-for-get_field_docs-like-so) for the anonymous field.
    fn get_field_docs<T: AsRef<str>>(field_name: T) -> Result<&'static str, Error> {
        let field_name = field_name.as_ref();
        let index = Self::__documented_get_index(field_name)
            .ok_or_else(|| Error::NoSuchField(field_name.into()))?;
        Self::FIELD_DOCS[index].ok_or_else(|| Error::NoDocComments(field_name.into()))
    }
}

/// Adds an associated function [`get_variant_docs`](Self::get_variant_docs) to
/// access the documentation on an enum variant.
///
/// The associated derive macro of this trait will error if any variant does not
/// have any doc comments. Use [`DocumentedVariantsOpt`] if this is undesirable.
///
/// This trait and associated derive macro works on enums only. For structs and
/// unions, use [`DocumentedFields`] instead.
///
/// For how to use the derive macro, see [`macro@DocumentedVariants`].
pub trait DocumentedVariants {
    /// Get the documentation on this enum variant.
    fn get_variant_docs(&self) -> &'static str;
}

/// The optional variant of [`DocumentedVariants`].
pub trait DocumentedVariantsOpt {
    /// Get the documentation on this enum variant.
    fn get_variant_docs(&self) -> Option<&'static str>;
}

/// Errors of `documented`.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    /// The requested field does not have doc comments.
    #[error(r#"The field "{0}" has no doc comments"#)]
    NoDocComments(String),
    /// The requested field does not exist.
    #[error(r#"No field named "{0}" exists"#)]
    NoSuchField(String),
}
