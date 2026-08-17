//! # Error Handling
//!
//! ## Masking Sensitive Data
//!
//! When working with sensitive data, you might want to hide actual values from error messages.
//! The `ValidationError` type provides methods to mask instance values while preserving the error context:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"maxLength": 5});
//! let instance = json!("sensitive data");
//! let validator = jsonschema::validator_for(&schema)?;
//!
//! if let Err(error) = validator.validate(&instance) {
//!     // Use default masking (replaces values with "value")
//!     println!("Masked error: {}", error.masked());
//!     // Or provide custom placeholder
//!     println!("Custom masked: {}", error.masked_with("[REDACTED]"));
//! }
//! # Ok(())
//! # }
//! ```
//!
//! The masked error messages will replace instance values with placeholders while maintaining
//! schema-related information like property names, limits, and types.
//!
//! Original error:
//! ```text
//! "sensitive data" is longer than 5 characters
//! ```
//!
//! Masked error:
//! ```text
//! value is longer than 5 characters
//! ```
use crate::{
    paths::Location,
    thread::ThreadBound,
    types::{JsonType, JsonTypeSet},
};
use serde_json::{Map, Number, Value};
use std::{
    borrow::Cow,
    error,
    fmt::{self, Formatter, Write},
    iter::{empty, once},
    slice,
    string::FromUtf8Error,
    vec,
};

/// An error that can occur during validation.
#[derive(Debug)]
pub struct ValidationError<'a> {
    repr: Box<ValidationErrorRepr<'a>>,
}

#[derive(Debug)]
struct ValidationErrorRepr<'a> {
    instance: Cow<'a, Value>,
    kind: ValidationErrorKind,
    instance_path: Location,
    schema_path: Location,
}

/// An iterator over instances of [`ValidationError`] that represent validation error for the
/// input instance.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"maxLength": 5});
/// let instance = json!("foo");
/// if let Ok(validator) = jsonschema::validator_for(&schema) {
///     let errors = validator.iter_errors(&instance);
///     for error in errors {
///         println!("Validation error: {}", error)
///     }
/// }
/// ```
#[doc(hidden)]
pub trait ValidationErrorIterator<'a>: Iterator<Item = ValidationError<'a>> + ThreadBound {}

impl<'a, T> ValidationErrorIterator<'a> for T where
    T: Iterator<Item = ValidationError<'a>> + ThreadBound
{
}

/// A lazily-evaluated iterator over validation errors.
///
/// Use [`into_errors()`](Self::into_errors) to convert into [`ValidationErrors`],
/// which implements [`std::error::Error`] for integration with error handling libraries.
pub struct ErrorIterator<'a> {
    iter: Box<dyn ValidationErrorIterator<'a> + 'a>,
}

impl<'a> ErrorIterator<'a> {
    #[inline]
    pub(crate) fn from_iterator<T>(iterator: T) -> Self
    where
        T: ValidationErrorIterator<'a> + 'a,
    {
        Self {
            iter: Box::new(iterator),
        }
    }

    /// Collects all errors into [`ValidationErrors`], which implements [`std::error::Error`].
    #[inline]
    #[must_use]
    pub fn into_errors(self) -> ValidationErrors<'a> {
        ValidationErrors {
            errors: self.collect(),
        }
    }
}

/// An owned collection of validation errors that implements [`std::error::Error`].
///
/// Obtain this by calling [`ErrorIterator::into_errors()`].
pub struct ValidationErrors<'a> {
    errors: Vec<ValidationError<'a>>,
}

impl<'a> ValidationErrors<'a> {
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns the errors as a slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[ValidationError<'a>] {
        &self.errors
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, ValidationError<'a>> {
        self.errors.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, ValidationError<'a>> {
        self.errors.iter_mut()
    }
}

impl fmt::Display for ValidationErrors<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            f.write_str("Validation succeeded")
        } else {
            writeln!(f, "Validation errors:")?;
            for (idx, error) in self.errors.iter().enumerate() {
                writeln!(f, "{:02}: {error}", idx + 1)?;
            }
            Ok(())
        }
    }
}

impl fmt::Debug for ValidationErrors<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidationErrors")
            .field("errors", &self.errors)
            .finish()
    }
}

impl error::Error for ValidationErrors<'_> {}

impl<'a> Iterator for ErrorIterator<'a> {
    type Item = ValidationError<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.as_mut().next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> IntoIterator for ValidationErrors<'a> {
    type Item = ValidationError<'a>;
    type IntoIter = vec::IntoIter<ValidationError<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'b ValidationErrors<'a> {
    type Item = &'b ValidationError<'a>;
    type IntoIter = slice::Iter<'b, ValidationError<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

impl<'a, 'b> IntoIterator for &'b mut ValidationErrors<'a> {
    type Item = &'b mut ValidationError<'a>;
    type IntoIter = slice::IterMut<'b, ValidationError<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter_mut()
    }
}

// Empty iterator means no error happened
pub(crate) fn no_error<'a>() -> ErrorIterator<'a> {
    ErrorIterator::from_iterator(empty())
}
// A wrapper for one error
pub(crate) fn error(instance: ValidationError) -> ErrorIterator {
    ErrorIterator::from_iterator(once(instance))
}

/// Kinds of errors that may happen during validation
#[derive(Debug)]
#[allow(missing_docs)]
pub enum ValidationErrorKind {
    /// The input array contain more items than expected.
    AdditionalItems { limit: usize },
    /// Unexpected properties.
    AdditionalProperties { unexpected: Vec<String> },
    /// The input value is not valid under any of the schemas listed in the 'anyOf' keyword.
    AnyOf {
        context: Vec<Vec<ValidationError<'static>>>,
    },
    /// Results from a [`fancy_regex::RuntimeError::BacktrackLimitExceeded`] variant when matching
    BacktrackLimitExceeded { error: fancy_regex::Error },
    /// The input value doesn't match expected constant.
    Constant { expected_value: Value },
    /// The input array doesn't contain items conforming to the specified schema.
    Contains,
    /// The input value does not respect the defined contentEncoding
    ContentEncoding { content_encoding: String },
    /// The input value does not respect the defined contentMediaType
    ContentMediaType { content_media_type: String },
    /// Custom error message for user-defined validation.
    Custom { message: String },
    /// The input value doesn't match any of specified options.
    Enum { options: Value },
    /// Value is too large.
    ExclusiveMaximum { limit: Value },
    /// Value is too small.
    ExclusiveMinimum { limit: Value },
    /// Everything is invalid for `false` schema.
    FalseSchema,
    /// When the input doesn't match to the specified format.
    Format { format: String },
    /// May happen in `contentEncoding` validation if `base64` encoded data is invalid.
    FromUtf8 { error: FromUtf8Error },
    /// Too many items in an array.
    MaxItems { limit: u64 },
    /// Value is too large.
    Maximum { limit: Value },
    /// String is too long.
    MaxLength { limit: u64 },
    /// Too many properties in an object.
    MaxProperties { limit: u64 },
    /// Too few items in an array.
    MinItems { limit: u64 },
    /// Value is too small.
    Minimum { limit: Value },
    /// String is too short.
    MinLength { limit: u64 },
    /// Not enough properties in an object.
    MinProperties { limit: u64 },
    /// When some number is not a multiple of another number.
    MultipleOf {
        #[cfg(feature = "arbitrary-precision")]
        multiple_of: Value,
        #[cfg(not(feature = "arbitrary-precision"))]
        multiple_of: f64,
    },
    /// Negated schema failed validation.
    Not { schema: Value },
    /// The given schema is valid under more than one of the schemas listed in the 'oneOf' keyword.
    OneOfMultipleValid {
        context: Vec<Vec<ValidationError<'static>>>,
    },
    /// The given schema is not valid under any of the schemas listed in the 'oneOf' keyword.
    OneOfNotValid {
        context: Vec<Vec<ValidationError<'static>>>,
    },
    /// When the input doesn't match to a pattern.
    Pattern { pattern: String },
    /// Object property names are invalid.
    PropertyNames {
        error: Box<ValidationError<'static>>,
    },
    /// When a required property is missing.
    Required { property: Value },
    /// When the input value doesn't match one or multiple required types.
    Type { kind: TypeKind },
    /// Unexpected items.
    UnevaluatedItems { unexpected: Vec<String> },
    /// Unexpected properties.
    UnevaluatedProperties { unexpected: Vec<String> },
    /// When the input array has non-unique elements.
    UniqueItems,
    /// Error during schema ref resolution.
    Referencing(referencing::Error),
}

impl ValidationErrorKind {
    pub(crate) fn keyword(&self) -> &'static str {
        match self {
            ValidationErrorKind::AdditionalItems { .. } => "additionalItems",
            ValidationErrorKind::AdditionalProperties { .. } => "additionalProperties",
            ValidationErrorKind::AnyOf { .. } => "anyOf",
            ValidationErrorKind::BacktrackLimitExceeded { .. }
            | ValidationErrorKind::Pattern { .. } => "pattern",
            ValidationErrorKind::Constant { .. } => "const",
            ValidationErrorKind::Contains => "contains",
            ValidationErrorKind::ContentEncoding { .. } | ValidationErrorKind::FromUtf8 { .. } => {
                "contentEncoding"
            }
            ValidationErrorKind::ContentMediaType { .. } => "contentMediaType",
            ValidationErrorKind::Custom { .. } => "custom",
            ValidationErrorKind::Enum { .. } => "enum",
            ValidationErrorKind::ExclusiveMaximum { .. } => "exclusiveMaximum",
            ValidationErrorKind::ExclusiveMinimum { .. } => "exclusiveMinimum",
            ValidationErrorKind::FalseSchema => "falseSchema",
            ValidationErrorKind::Format { .. } => "format",
            ValidationErrorKind::MaxItems { .. } => "maxItems",
            ValidationErrorKind::Maximum { .. } => "maximum",
            ValidationErrorKind::MaxLength { .. } => "maxLength",
            ValidationErrorKind::MaxProperties { .. } => "maxProperties",
            ValidationErrorKind::MinItems { .. } => "minItems",
            ValidationErrorKind::Minimum { .. } => "minimum",
            ValidationErrorKind::MinLength { .. } => "minLength",
            ValidationErrorKind::MinProperties { .. } => "minProperties",
            ValidationErrorKind::MultipleOf { .. } => "multipleOf",
            ValidationErrorKind::Not { .. } => "not",
            ValidationErrorKind::OneOfMultipleValid { .. }
            | ValidationErrorKind::OneOfNotValid { .. } => "oneOf",
            ValidationErrorKind::PropertyNames { .. } => "propertyNames",
            ValidationErrorKind::Required { .. } => "required",
            ValidationErrorKind::Type { .. } => "type",
            ValidationErrorKind::UnevaluatedItems { .. } => "unevaluatedItems",
            ValidationErrorKind::UnevaluatedProperties { .. } => "unevaluatedProperties",
            ValidationErrorKind::UniqueItems => "uniqueItems",
            ValidationErrorKind::Referencing(_) => "$ref",
        }
    }
}

#[derive(Debug)]
#[allow(missing_docs)]
pub enum TypeKind {
    Single(JsonType),
    Multiple(JsonTypeSet),
}

/// Shortcuts for creation of specific error kinds.
impl<'a> ValidationError<'a> {
    /// Creates a new validation error from parts.
    #[inline]
    #[must_use]
    pub(crate) fn new(
        instance: Cow<'a, Value>,
        kind: ValidationErrorKind,
        instance_path: Location,
        schema_path: Location,
    ) -> Self {
        Self {
            repr: Box::new(ValidationErrorRepr {
                instance,
                kind,
                instance_path,
                schema_path,
            }),
        }
    }

    /// Returns a reference to the instance that failed validation.
    #[inline]
    #[must_use]
    pub fn instance(&self) -> &Cow<'a, Value> {
        &self.repr.instance
    }

    /// Returns the kind of validation error.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &ValidationErrorKind {
        &self.repr.kind
    }

    /// Returns the JSON Pointer to the instance location that failed validation.
    #[inline]
    #[must_use]
    pub fn instance_path(&self) -> &Location {
        &self.repr.instance_path
    }

    /// Returns the JSON Pointer to the schema keyword that failed validation.
    #[inline]
    #[must_use]
    pub fn schema_path(&self) -> &Location {
        &self.repr.schema_path
    }

    /// Decomposes the error into owned parts.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (Cow<'a, Value>, ValidationErrorKind, Location, Location) {
        let repr = *self.repr;
        (
            repr.instance,
            repr.kind,
            repr.instance_path,
            repr.schema_path,
        )
    }

    #[inline]
    fn borrowed(
        instance: &'a Value,
        kind: ValidationErrorKind,
        instance_path: Location,
        schema_path: Location,
    ) -> Self {
        Self::new(Cow::Borrowed(instance), kind, instance_path, schema_path)
    }

    /// Returns a wrapper that masks instance values in error messages.
    /// Uses "value" as a default placeholder.
    #[must_use]
    pub fn masked<'b>(&'b self) -> MaskedValidationError<'a, 'b, 'static> {
        self.masked_with("value")
    }

    /// Returns a wrapper that masks instance values in error messages with a custom placeholder.
    pub fn masked_with<'b, 'c>(
        &'b self,
        placeholder: impl Into<Cow<'c, str>>,
    ) -> MaskedValidationError<'a, 'b, 'c> {
        MaskedValidationError {
            error: self,
            placeholder: placeholder.into(),
        }
    }
    /// Converts the `ValidationError` into an owned version with `'static` lifetime.
    #[must_use]
    pub fn to_owned(self) -> ValidationError<'static> {
        let (instance, kind, instance_path, schema_path) = self.into_parts();
        ValidationError::new(
            Cow::Owned(instance.into_owned()),
            kind,
            instance_path,
            schema_path,
        )
    }

    pub(crate) fn additional_items(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: usize,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::AdditionalItems { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn additional_properties(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        unexpected: Vec<String>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::AdditionalProperties { unexpected },
            instance_path,
            location,
        )
    }
    pub(crate) fn any_of(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        context: Vec<Vec<ValidationError<'a>>>,
    ) -> ValidationError<'a> {
        let context = context
            .into_iter()
            .map(|errors| errors.into_iter().map(ValidationError::to_owned).collect())
            .collect::<Vec<_>>();

        Self::borrowed(
            instance,
            ValidationErrorKind::AnyOf { context },
            instance_path,
            location,
        )
    }
    pub(crate) fn backtrack_limit(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        error: fancy_regex::Error,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::BacktrackLimitExceeded { error },
            instance_path,
            location,
        )
    }
    pub(crate) fn constant_array(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        expected_value: &[Value],
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Constant {
                expected_value: Value::Array(expected_value.to_vec()),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn constant_boolean(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        expected_value: bool,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Constant {
                expected_value: Value::Bool(expected_value),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn constant_null(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Constant {
                expected_value: Value::Null,
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn constant_number(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        expected_value: &Number,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Constant {
                expected_value: Value::Number(expected_value.clone()),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn constant_object(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        expected_value: &Map<String, Value>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Constant {
                expected_value: Value::Object(expected_value.clone()),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn constant_string(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        expected_value: &str,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Constant {
                expected_value: Value::String(expected_value.to_string()),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn contains(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Contains,
            instance_path,
            location,
        )
    }
    pub(crate) fn content_encoding(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        encoding: &str,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::ContentEncoding {
                content_encoding: encoding.to_string(),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn content_media_type(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        media_type: &str,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::ContentMediaType {
                content_media_type: media_type.to_string(),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn enumeration(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        options: &Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Enum {
                options: options.clone(),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn exclusive_maximum(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::ExclusiveMaximum { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn exclusive_minimum(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::ExclusiveMinimum { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn false_schema(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::FalseSchema,
            instance_path,
            location,
        )
    }
    pub(crate) fn format(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        format: impl Into<String>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Format {
                format: format.into(),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn from_utf8(error: FromUtf8Error) -> ValidationError<'a> {
        ValidationError::new(
            Cow::Owned(Value::Null),
            ValidationErrorKind::FromUtf8 { error },
            Location::new(),
            Location::new(),
        )
    }
    pub(crate) fn max_items(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: u64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MaxItems { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn maximum(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Maximum { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn max_length(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: u64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MaxLength { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn max_properties(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: u64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MaxProperties { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn min_items(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: u64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MinItems { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn minimum(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Minimum { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn min_length(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: u64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MinLength { limit },
            instance_path,
            location,
        )
    }
    pub(crate) fn min_properties(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        limit: u64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MinProperties { limit },
            instance_path,
            location,
        )
    }
    #[cfg(feature = "arbitrary-precision")]
    pub(crate) fn multiple_of(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        multiple_of: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MultipleOf { multiple_of },
            instance_path,
            location,
        )
    }

    #[cfg(not(feature = "arbitrary-precision"))]
    pub(crate) fn multiple_of(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        multiple_of: f64,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::MultipleOf { multiple_of },
            instance_path,
            location,
        )
    }
    pub(crate) fn not(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        schema: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Not { schema },
            instance_path,
            location,
        )
    }
    pub(crate) fn one_of_multiple_valid(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        context: Vec<Vec<ValidationError<'a>>>,
    ) -> ValidationError<'a> {
        let context = context
            .into_iter()
            .map(|errors| errors.into_iter().map(ValidationError::to_owned).collect())
            .collect::<Vec<_>>();

        Self::borrowed(
            instance,
            ValidationErrorKind::OneOfMultipleValid { context },
            instance_path,
            location,
        )
    }
    pub(crate) fn one_of_not_valid(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        context: Vec<Vec<ValidationError<'a>>>,
    ) -> ValidationError<'a> {
        let context = context
            .into_iter()
            .map(|errors| errors.into_iter().map(ValidationError::to_owned).collect())
            .collect::<Vec<_>>();

        Self::borrowed(
            instance,
            ValidationErrorKind::OneOfNotValid { context },
            instance_path,
            location,
        )
    }
    pub(crate) fn pattern(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        pattern: String,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Pattern { pattern },
            instance_path,
            location,
        )
    }
    pub(crate) fn property_names(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        error: ValidationError<'a>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::PropertyNames {
                error: Box::new(error.to_owned()),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn required(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        property: Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Required { property },
            instance_path,
            location,
        )
    }

    pub(crate) fn single_type_error(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        type_name: JsonType,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Type {
                kind: TypeKind::Single(type_name),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn multiple_type_error(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        types: JsonTypeSet,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Type {
                kind: TypeKind::Multiple(types),
            },
            instance_path,
            location,
        )
    }
    pub(crate) fn unevaluated_items(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        unexpected: Vec<String>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::UnevaluatedItems { unexpected },
            instance_path,
            location,
        )
    }
    pub(crate) fn unevaluated_properties(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        unexpected: Vec<String>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::UnevaluatedProperties { unexpected },
            instance_path,
            location,
        )
    }
    pub(crate) fn unique_items(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::UniqueItems,
            instance_path,
            location,
        )
    }
    /// Create a new custom validation error.
    pub fn custom(
        location: Location,
        instance_path: Location,
        instance: &'a Value,
        message: impl Into<String>,
    ) -> ValidationError<'a> {
        Self::borrowed(
            instance,
            ValidationErrorKind::Custom {
                message: message.into(),
            },
            instance_path,
            location,
        )
    }
}

impl error::Error for ValidationError<'_> {}
impl From<referencing::Error> for ValidationError<'_> {
    #[inline]
    fn from(err: referencing::Error) -> Self {
        ValidationError::new(
            Cow::Owned(Value::Null),
            ValidationErrorKind::Referencing(err),
            Location::new(),
            Location::new(),
        )
    }
}
impl From<FromUtf8Error> for ValidationError<'_> {
    #[inline]
    fn from(err: FromUtf8Error) -> Self {
        ValidationError::from_utf8(err)
    }
}

fn write_quoted_list(f: &mut Formatter<'_>, items: &[impl fmt::Display]) -> fmt::Result {
    let mut iter = items.iter();
    if let Some(item) = iter.next() {
        f.write_char('\'')?;
        write!(f, "{item}")?;
        f.write_char('\'')?;
    }
    for item in iter {
        f.write_str(", ")?;
        f.write_char('\'')?;
        write!(f, "{item}")?;
        f.write_char('\'')?;
    }
    Ok(())
}

fn write_unexpected_suffix(f: &mut Formatter<'_>, len: usize) -> fmt::Result {
    f.write_str(if len == 1 {
        " was unexpected)"
    } else {
        " were unexpected)"
    })
}

const MAX_DISPLAYED_ENUM_VARIANTS: usize = 3;

fn write_enum_message(
    f: &mut Formatter<'_>,
    value: impl fmt::Display,
    options: &Value,
) -> fmt::Result {
    let array = options
        .as_array()
        .expect("Enum options must be a JSON array");

    write!(f, "{value} is not one of ")?;

    let total_count = array.len();

    if total_count <= MAX_DISPLAYED_ENUM_VARIANTS {
        // Show all options with proper "a, b or c" formatting
        for (i, option) in array.iter().enumerate() {
            if i == 0 {
                write!(f, "{option}")?;
            } else if i == total_count - 1 {
                write!(f, " or {option}")?;
            } else {
                write!(f, ", {option}")?;
            }
        }
    } else {
        // Show first few, then "or X other candidates"
        let show_count = MAX_DISPLAYED_ENUM_VARIANTS - 1;
        for (i, option) in array.iter().take(show_count).enumerate() {
            if i == 0 {
                write!(f, "{option}")?;
            } else {
                write!(f, ", {option}")?;
            }
        }
        let remaining = total_count - show_count;
        write!(f, " or {remaining} other candidates")?;
    }
    Ok(())
}

/// Textual representation of various validation errors.
impl fmt::Display for ValidationError<'_> {
    #[allow(clippy::too_many_lines)] // The function is long but it does formatting only
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind() {
            ValidationErrorKind::Referencing(error) => error.fmt(f),
            ValidationErrorKind::BacktrackLimitExceeded { error } => error.fmt(f),
            ValidationErrorKind::Format { format } => {
                write!(f, r#"{} is not a "{}""#, self.instance(), format)
            }
            ValidationErrorKind::AdditionalItems { limit } => {
                f.write_str("Additional items are not allowed (")?;
                let array = self.instance().as_array().expect("Always valid");
                let mut iter = array.iter().skip(*limit);

                if let Some(item) = iter.next() {
                    write!(f, "{item}")?;
                }
                for item in iter {
                    f.write_str(", ")?;
                    write!(f, "{item}")?;
                }

                write_unexpected_suffix(f, array.len() - limit)
            }
            ValidationErrorKind::AdditionalProperties { unexpected } => {
                f.write_str("Additional properties are not allowed (")?;
                write_quoted_list(f, unexpected)?;
                write_unexpected_suffix(f, unexpected.len())
            }
            ValidationErrorKind::AnyOf { context: _ } => write!(
                f,
                "{} is not valid under any of the schemas listed in the 'anyOf' keyword",
                self.instance()
            ),
            ValidationErrorKind::OneOfNotValid { context: _ } => write!(
                f,
                "{} is not valid under any of the schemas listed in the 'oneOf' keyword",
                self.instance()
            ),
            ValidationErrorKind::Contains => write!(
                f,
                "None of {} are valid under the given schema",
                self.instance()
            ),
            ValidationErrorKind::Constant { expected_value } => {
                write!(f, "{expected_value} was expected")
            }
            ValidationErrorKind::ContentEncoding { content_encoding } => {
                write!(
                    f,
                    r#"{} is not compliant with "{}" content encoding"#,
                    self.instance(),
                    content_encoding
                )
            }
            ValidationErrorKind::ContentMediaType { content_media_type } => {
                write!(
                    f,
                    r#"{} is not compliant with "{}" media type"#,
                    self.instance(),
                    content_media_type
                )
            }
            ValidationErrorKind::FromUtf8 { error } => error.fmt(f),
            ValidationErrorKind::Enum { options } => {
                write_enum_message(f, self.instance(), options)
            }
            ValidationErrorKind::ExclusiveMaximum { limit } => write!(
                f,
                "{} is greater than or equal to the maximum of {}",
                self.instance(),
                limit
            ),
            ValidationErrorKind::ExclusiveMinimum { limit } => write!(
                f,
                "{} is less than or equal to the minimum of {}",
                self.instance(),
                limit
            ),
            ValidationErrorKind::FalseSchema => {
                write!(f, "False schema does not allow {}", self.instance())
            }
            ValidationErrorKind::Maximum { limit } => write!(
                f,
                "{} is greater than the maximum of {}",
                self.instance(),
                limit
            ),
            ValidationErrorKind::Minimum { limit } => {
                write!(
                    f,
                    "{} is less than the minimum of {}",
                    self.instance(),
                    limit
                )
            }
            ValidationErrorKind::MaxLength { limit } => write!(
                f,
                "{} is longer than {} character{}",
                self.instance(),
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MinLength { limit } => write!(
                f,
                "{} is shorter than {} character{}",
                self.instance(),
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MaxItems { limit } => write!(
                f,
                "{} has more than {} item{}",
                self.instance(),
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MinItems { limit } => write!(
                f,
                "{} has less than {} item{}",
                self.instance(),
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MaxProperties { limit } => write!(
                f,
                "{} has more than {} propert{}",
                self.instance(),
                limit,
                if *limit == 1 { "y" } else { "ies" }
            ),
            ValidationErrorKind::MinProperties { limit } => write!(
                f,
                "{} has less than {} propert{}",
                self.instance(),
                limit,
                if *limit == 1 { "y" } else { "ies" }
            ),
            ValidationErrorKind::Not { schema } => {
                write!(f, "{} is not allowed for {}", schema, self.instance())
            }
            ValidationErrorKind::OneOfMultipleValid { .. } => write!(
                f,
                "{} is valid under more than one of the schemas listed in the 'oneOf' keyword",
                self.instance()
            ),
            ValidationErrorKind::Pattern { pattern } => {
                write!(f, r#"{} does not match "{}""#, self.instance(), pattern)
            }
            ValidationErrorKind::PropertyNames { error } => error.fmt(f),
            ValidationErrorKind::Required { property } => {
                write!(f, "{property} is a required property")
            }
            ValidationErrorKind::MultipleOf { multiple_of } => {
                write!(
                    f,
                    "{} is not a multiple of {}",
                    self.instance(),
                    multiple_of
                )
            }
            ValidationErrorKind::UnevaluatedItems { unexpected } => {
                f.write_str("Unevaluated items are not allowed (")?;
                write_quoted_list(f, unexpected)?;
                write_unexpected_suffix(f, unexpected.len())
            }
            ValidationErrorKind::UnevaluatedProperties { unexpected } => {
                f.write_str("Unevaluated properties are not allowed (")?;
                write_quoted_list(f, unexpected)?;
                write_unexpected_suffix(f, unexpected.len())
            }
            ValidationErrorKind::UniqueItems => {
                write!(f, "{} has non-unique elements", self.instance())
            }
            ValidationErrorKind::Type {
                kind: TypeKind::Single(type_),
            } => write!(f, r#"{} is not of type "{}""#, self.instance(), type_),
            ValidationErrorKind::Type {
                kind: TypeKind::Multiple(types),
            } => {
                write!(f, "{} is not of types ", self.instance())?;
                let mut iter = types.iter();
                if let Some(t) = iter.next() {
                    f.write_char('"')?;
                    write!(f, "{t}")?;
                    f.write_char('"')?;
                }
                for t in iter {
                    f.write_str(", ")?;
                    f.write_char('"')?;
                    write!(f, "{t}")?;
                    f.write_char('"')?;
                }
                Ok(())
            }
            ValidationErrorKind::Custom { message } => f.write_str(message),
        }
    }
}

/// A wrapper that provides a masked display of validation errors.
pub struct MaskedValidationError<'a, 'b, 'c> {
    error: &'b ValidationError<'a>,
    placeholder: Cow<'c, str>,
}

impl fmt::Display for MaskedValidationError<'_, '_, '_> {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.error.kind() {
            ValidationErrorKind::Referencing(error) => error.fmt(f),
            ValidationErrorKind::BacktrackLimitExceeded { error } => error.fmt(f),
            ValidationErrorKind::Format { format } => {
                write!(f, r#"{} is not a "{format}""#, self.placeholder)
            }
            ValidationErrorKind::AdditionalItems { limit } => {
                write!(f, "Additional items are not allowed ({limit} items)")
            }
            ValidationErrorKind::AdditionalProperties { unexpected } => {
                f.write_str("Additional properties are not allowed (")?;
                write_quoted_list(f, unexpected)?;
                write_unexpected_suffix(f, unexpected.len())
            }
            ValidationErrorKind::AnyOf { .. } => write!(
                f,
                "{} is not valid under any of the schemas listed in the 'anyOf' keyword",
                self.placeholder
            ),
            ValidationErrorKind::OneOfNotValid { context: _ } => write!(
                f,
                "{} is not valid under any of the schemas listed in the 'oneOf' keyword",
                self.placeholder
            ),
            ValidationErrorKind::Contains => write!(
                f,
                "None of {} are valid under the given schema",
                self.placeholder
            ),
            ValidationErrorKind::Constant { expected_value } => {
                write!(f, "{expected_value} was expected")
            }
            ValidationErrorKind::ContentEncoding { content_encoding } => {
                write!(
                    f,
                    r#"{} is not compliant with "{}" content encoding"#,
                    self.placeholder, content_encoding
                )
            }
            ValidationErrorKind::ContentMediaType { content_media_type } => {
                write!(
                    f,
                    r#"{} is not compliant with "{}" media type"#,
                    self.placeholder, content_media_type
                )
            }
            ValidationErrorKind::FromUtf8 { error } => error.fmt(f),
            ValidationErrorKind::Enum { options } => {
                write_enum_message(f, &self.placeholder, options)
            }
            ValidationErrorKind::ExclusiveMaximum { limit } => write!(
                f,
                "{} is greater than or equal to the maximum of {}",
                self.placeholder, limit
            ),
            ValidationErrorKind::ExclusiveMinimum { limit } => write!(
                f,
                "{} is less than or equal to the minimum of {}",
                self.placeholder, limit
            ),
            ValidationErrorKind::FalseSchema => {
                write!(f, "False schema does not allow {}", self.placeholder)
            }
            ValidationErrorKind::Maximum { limit } => write!(
                f,
                "{} is greater than the maximum of {}",
                self.placeholder, limit
            ),
            ValidationErrorKind::Minimum { limit } => {
                write!(
                    f,
                    "{} is less than the minimum of {}",
                    self.placeholder, limit
                )
            }
            ValidationErrorKind::MaxLength { limit } => write!(
                f,
                "{} is longer than {} character{}",
                self.placeholder,
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MinLength { limit } => write!(
                f,
                "{} is shorter than {} character{}",
                self.placeholder,
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MaxItems { limit } => write!(
                f,
                "{} has more than {} item{}",
                self.placeholder,
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MinItems { limit } => write!(
                f,
                "{} has less than {} item{}",
                self.placeholder,
                limit,
                if *limit == 1 { "" } else { "s" }
            ),
            ValidationErrorKind::MaxProperties { limit } => write!(
                f,
                "{} has more than {} propert{}",
                self.placeholder,
                limit,
                if *limit == 1 { "y" } else { "ies" }
            ),
            ValidationErrorKind::MinProperties { limit } => write!(
                f,
                "{} has less than {} propert{}",
                self.placeholder,
                limit,
                if *limit == 1 { "y" } else { "ies" }
            ),
            ValidationErrorKind::Not { schema } => {
                write!(f, "{} is not allowed for {}", schema, self.placeholder)
            }
            ValidationErrorKind::OneOfMultipleValid { .. } => write!(
                f,
                "{} is valid under more than one of the schemas listed in the 'oneOf' keyword",
                self.placeholder
            ),
            ValidationErrorKind::Pattern { pattern } => {
                write!(f, r#"{} does not match "{}""#, self.placeholder, pattern)
            }
            ValidationErrorKind::PropertyNames { error } => error.fmt(f),
            ValidationErrorKind::Required { property } => {
                write!(f, "{property} is a required property")
            }
            ValidationErrorKind::MultipleOf { multiple_of } => {
                write!(
                    f,
                    "{} is not a multiple of {}",
                    self.placeholder, multiple_of
                )
            }
            ValidationErrorKind::UnevaluatedItems { unexpected } => {
                write!(
                    f,
                    "Unevaluated items are not allowed ({} items)",
                    unexpected.len()
                )
            }
            ValidationErrorKind::UnevaluatedProperties { unexpected } => {
                f.write_str("Unevaluated properties are not allowed (")?;
                write_quoted_list(f, unexpected)?;
                write_unexpected_suffix(f, unexpected.len())
            }
            ValidationErrorKind::UniqueItems => {
                write!(f, "{} has non-unique elements", self.placeholder)
            }
            ValidationErrorKind::Type {
                kind: TypeKind::Single(type_),
            } => write!(f, r#"{} is not of type "{}""#, self.placeholder, type_),
            ValidationErrorKind::Type {
                kind: TypeKind::Multiple(types),
            } => {
                write!(f, "{} is not of types ", self.placeholder)?;
                let mut iter = types.iter();
                if let Some(t) = iter.next() {
                    f.write_char('"')?;
                    write!(f, "{t}")?;
                    f.write_char('"')?;
                }
                for t in iter {
                    f.write_str(", ")?;
                    f.write_char('"')?;
                    write!(f, "{t}")?;
                    f.write_char('"')?;
                }
                Ok(())
            }
            ValidationErrorKind::Custom { message } => f.write_str(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use test_case::test_case;

    fn owned_error(instance: Value, kind: ValidationErrorKind) -> ValidationError<'static> {
        ValidationError::new(Cow::Owned(instance), kind, Location::new(), Location::new())
    }

    #[test]
    fn error_iterator_into_errors_collects_all_errors() {
        let iterator = ErrorIterator::from_iterator(
            vec![
                owned_error(json!(1), ValidationErrorKind::Minimum { limit: json!(2) }),
                owned_error(json!(3), ValidationErrorKind::Maximum { limit: json!(2) }),
            ]
            .into_iter(),
        );
        let validation_errors = iterator.into_errors();
        let collected: Vec<_> = validation_errors.into_iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0].to_string(), "1 is less than the minimum of 2");
        assert_eq!(
            collected[1].to_string(),
            "3 is greater than the maximum of 2"
        );
    }

    #[test]
    fn validation_errors_display_reports_success() {
        let errors = ValidationErrors { errors: Vec::new() };
        assert_eq!(format!("{errors}"), "Validation succeeded");
    }

    #[test]
    fn validation_errors_display_lists_messages() {
        let errors = ValidationErrors {
            errors: vec![
                owned_error(json!(1), ValidationErrorKind::Minimum { limit: json!(2) }),
                owned_error(json!(3), ValidationErrorKind::Maximum { limit: json!(2) }),
            ],
        };
        let rendered = format!("{errors}");
        assert!(rendered.contains("Validation errors:"));
        assert!(rendered.contains("01: 1 is less than the minimum of 2"));
        assert!(rendered.contains("02: 3 is greater than the maximum of 2"));
    }

    #[test]
    fn validation_errors_len_and_is_empty() {
        let empty = ValidationErrors { errors: vec![] };
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());

        let errors = ValidationErrors {
            errors: vec![owned_error(
                json!(1),
                ValidationErrorKind::Minimum { limit: json!(2) },
            )],
        };
        assert_eq!(errors.len(), 1);
        assert!(!errors.is_empty());
    }

    #[test]
    fn validation_errors_as_slice() {
        let errors = ValidationErrors {
            errors: vec![
                owned_error(json!(1), ValidationErrorKind::Minimum { limit: json!(2) }),
                owned_error(json!(3), ValidationErrorKind::Maximum { limit: json!(2) }),
            ],
        };

        let slice = errors.as_slice();
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0].to_string(), "1 is less than the minimum of 2");
        assert_eq!(slice[1].to_string(), "3 is greater than the maximum of 2");
    }

    #[test]
    fn validation_errors_iter() {
        let errors = ValidationErrors {
            errors: vec![
                owned_error(json!(1), ValidationErrorKind::Minimum { limit: json!(2) }),
                owned_error(json!(3), ValidationErrorKind::Maximum { limit: json!(2) }),
            ],
        };

        let collected: Vec<_> = errors.iter().map(ValidationError::to_string).collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0], "1 is less than the minimum of 2");
        assert_eq!(collected[1], "3 is greater than the maximum of 2");
    }

    #[test]
    #[allow(clippy::explicit_iter_loop)]
    fn validation_errors_iter_mut() {
        let mut errors = ValidationErrors {
            errors: vec![owned_error(
                json!(1),
                ValidationErrorKind::Minimum { limit: json!(2) },
            )],
        };

        // Verify we can get mutable references via iter_mut()
        for error in errors.iter_mut() {
            let _ = error.to_string();
        }
    }

    #[test]
    fn validation_errors_into_iterator_by_ref() {
        let errors = ValidationErrors {
            errors: vec![owned_error(
                json!(1),
                ValidationErrorKind::Minimum { limit: json!(2) },
            )],
        };

        let collected: Vec<_> = (&errors).into_iter().collect();
        assert_eq!(collected.len(), 1);
        // Verify errors is still usable
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn validation_errors_into_iterator_by_mut_ref() {
        let mut errors = ValidationErrors {
            errors: vec![owned_error(
                json!(1),
                ValidationErrorKind::Minimum { limit: json!(2) },
            )],
        };

        let collected: Vec<_> = (&mut errors).into_iter().collect();
        assert_eq!(collected.len(), 1);
        // Verify errors is still usable
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn error_iterator_size_hint() {
        let vec = vec![
            owned_error(json!(1), ValidationErrorKind::Minimum { limit: json!(2) }),
            owned_error(json!(3), ValidationErrorKind::Maximum { limit: json!(2) }),
        ];
        let iterator = ErrorIterator::from_iterator(vec.into_iter());
        let (lower, upper) = iterator.size_hint();
        assert_eq!(lower, 2);
        assert_eq!(upper, Some(2));
    }

    #[test]
    fn validation_errors_debug() {
        let errors = ValidationErrors {
            errors: vec![owned_error(
                json!(1),
                ValidationErrorKind::Minimum { limit: json!(2) },
            )],
        };
        let debug_output = format!("{errors:?}");
        assert!(debug_output.contains("ValidationErrors"));
        assert!(debug_output.contains("errors"));
    }

    #[test]
    fn single_type_error() {
        let instance = json!(42);
        let err = ValidationError::single_type_error(
            Location::new(),
            Location::new(),
            &instance,
            JsonType::String,
        );
        assert_eq!(err.to_string(), r#"42 is not of type "string""#);
    }

    #[test]
    fn multiple_types_error() {
        let instance = json!(42);
        let types = JsonTypeSet::empty()
            .insert(JsonType::String)
            .insert(JsonType::Number);
        let err = ValidationError::multiple_type_error(
            Location::new(),
            Location::new(),
            &instance,
            types,
        );
        assert_eq!(err.to_string(), r#"42 is not of types "number", "string""#);
    }

    #[test_case(true, &json!({"foo": {"bar": 42}}), "/foo/bar")]
    #[test_case(true, &json!({"foo": "a"}), "/foo")]
    #[test_case(false, &json!({"foo": {"bar": 42}}), "/foo/bar")]
    #[test_case(false, &json!({"foo": "a"}), "/foo")]
    fn instance_path_properties(additional_properties: bool, instance: &Value, expected: &str) {
        let schema = json!(
            {
                "additionalProperties": additional_properties,
                "type":"object",
                "properties":{
                   "foo":{
                      "type":"object",
                      "properties":{
                         "bar":{
                            "type":"string"
                         }
                      }
                   }
                }
            }
        );
        let validator = crate::validator_for(&schema).unwrap();
        let mut result = validator.iter_errors(instance);
        let error = result.next().expect("validation error");

        assert!(result.next().is_none());
        assert_eq!(error.instance_path().as_str(), expected);
    }

    #[test_case(true, &json!([1, {"foo": ["42"]}]), "/0")]
    #[test_case(true, &json!(["a", {"foo": [42]}]), "/1/foo/0")]
    #[test_case(false, &json!([1, {"foo": ["42"]}]), "/0")]
    #[test_case(false, &json!(["a", {"foo": [42]}]), "/1/foo/0")]
    fn instance_path_properties_and_arrays(
        additional_items: bool,
        instance: &Value,
        expected: &str,
    ) {
        let schema = json!(
            {
                "items": additional_items,
                "type": "array",
                "prefixItems": [
                    {
                        "type": "string"
                    },
                    {
                        "type": "object",
                        "properties": {
                            "foo": {
                                "type": "array",
                                "prefixItems": [
                                    {
                                        "type": "string"
                                    }
                                ]
                            }
                        }
                    }
                ]
            }
        );
        let validator = crate::validator_for(&schema).unwrap();
        let mut result = validator.iter_errors(instance);
        let error = result.next().expect("validation error");

        assert!(result.next().is_none());
        assert_eq!(error.instance_path().as_str(), expected);
    }

    #[test_case(true, &json!([[1, 2, 3], [4, "5", 6], [7, 8, 9]]), "/1/1")]
    #[test_case(false, &json!([[1, 2, 3], [4, "5", 6], [7, 8, 9]]), "/1/1")]
    #[test_case(true, &json!([[1, 2, 3], [4, 5, 6], 42]), "/2")]
    #[test_case(false, &json!([[1, 2, 3], [4, 5, 6], 42]), "/2")]
    fn instance_path_nested_arrays(additional_items: bool, instance: &Value, expected: &str) {
        let schema = json!(
            {
                "additionalItems": additional_items,
                "type": "array",
                "items": {
                    "type": "array",
                    "items": {
                        "type": "integer"
                    }
                }
            }
        );
        let validator = crate::validator_for(&schema).unwrap();
        let mut result = validator.iter_errors(instance);
        let error = result.next().expect("validation error");

        assert!(result.next().is_none());
        assert_eq!(error.instance_path().as_str(), expected);
    }

    #[test_case(true, &json!([1, "a"]), "/1")]
    #[test_case(false, &json!([1, "a"]), "/1")]
    #[test_case(true, &json!(123), "")]
    #[test_case(false, &json!(123), "")]
    fn instance_path_arrays(additional_items: bool, instance: &Value, expected: &str) {
        let schema = json!(
            {
                "additionalItems": additional_items,
                "type": "array",
                "items": {
                    "type": "integer"
                }
            }
        );
        let validator = crate::validator_for(&schema).unwrap();
        let mut result = validator.iter_errors(instance);
        let error = result.next().expect("validation error");

        assert!(result.next().is_none());
        assert_eq!(error.instance_path().as_str(), expected);
    }

    #[test_case(
        json!("2023-13-45"), 
        ValidationErrorKind::Format {
            format: "date".to_string(),
        },
        "value is not a \"date\""
    )]
    #[test_case(
        json!("sensitive data"),
        ValidationErrorKind::MaxLength { limit: 5 },
        "value is longer than 5 characters"
    )]
    #[test_case(
        json!({"secret": "data", "key": "value"}),
        ValidationErrorKind::AdditionalProperties {
            unexpected: vec!["secret".to_string(), "key".to_string()] 
        },
        "Additional properties are not allowed ('secret', 'key' were unexpected)"
    )]
    #[test_case(
        json!(123),
        ValidationErrorKind::Minimum { limit: json!(456) },
        "value is less than the minimum of 456"
    )]
    #[test_case(
        json!("secret_key_123"),
        ValidationErrorKind::Pattern {
            pattern: "^[A-Z0-9]{32}$".to_string(),
        },
        "value does not match \"^[A-Z0-9]{32}$\""
    )]
    #[test_case(
        json!([1, 2, 2, 3]),
        ValidationErrorKind::UniqueItems,
        "value has non-unique elements"
    )]
    #[test_case(
        json!(123),
        ValidationErrorKind::Type { kind: TypeKind::Single(JsonType::String) },
        "value is not of type \"string\""
    )]
    fn test_masked_error_messages(instance: Value, kind: ValidationErrorKind, expected: &str) {
        let error =
            ValidationError::new(Cow::Owned(instance), kind, Location::new(), Location::new());
        assert_eq!(error.masked().to_string(), expected);
    }

    #[test_case(
        json!("sensitive data"),
        ValidationErrorKind::MaxLength { limit: 5 },
        "[REDACTED]",
        "[REDACTED] is longer than 5 characters"
    )]
    #[test_case(
        json!({"password": "secret123"}),
        ValidationErrorKind::Type {
            kind: TypeKind::Single(JsonType::String)
        },
        "***",
        "*** is not of type \"string\""
    )]
    fn test_custom_masked_error_messages(
        instance: Value,
        kind: ValidationErrorKind,
        placeholder: &str,
        expected: &str,
    ) {
        let error =
            ValidationError::new(Cow::Owned(instance), kind, Location::new(), Location::new());
        assert_eq!(error.masked_with(placeholder).to_string(), expected);
    }
}
