//! Building a JSON Schema validator.
//! The main idea is to create a tree from the input JSON Schema. This tree will contain
//! everything needed to perform such validation in runtime.
use crate::{
    error::{error, no_error, ErrorIterator},
    evaluation::{Annotations, ErrorDescription, Evaluation, EvaluationNode},
    node::SchemaNode,
    paths::LazyLocation,
    thread::ThreadBound,
    Draft, ValidationError, ValidationOptions,
};
use serde_json::Value;

/// Tracks `(node_id, instance_ptr)` pairs to detect cycles in circular `$ref` chains.
#[derive(Default)]
pub(crate) struct ValidationContext {
    validating: Vec<(usize, usize)>,
}

impl ValidationContext {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if cycle detected, `false` otherwise (and adds pair to stack).
    #[inline]
    pub(crate) fn enter(&mut self, node_id: usize, instance: &Value) -> bool {
        let key = (node_id, std::ptr::from_ref::<Value>(instance) as usize);
        if self.validating.contains(&key) {
            return true;
        }
        self.validating.push(key);
        false
    }

    #[inline]
    pub(crate) fn exit(&mut self, node_id: usize, instance: &Value) {
        let popped = self.validating.pop();
        debug_assert_eq!(
            popped,
            Some((node_id, std::ptr::from_ref::<Value>(instance) as usize)),
            "ValidationContext::exit called out of order"
        );
    }
}

/// The Validate trait represents a predicate over some JSON value. Some validators are very simple
/// predicates such as "a value which is a string", whereas others may be much more complex,
/// consisting of several other validators composed together in various ways.
///
/// Much of the time all an application cares about is whether the predicate returns true or false,
/// in that case the `is_valid` function is sufficient. Sometimes applications will want more
/// detail about why a schema has failed, in which case the `validate` method can be used to
/// iterate over the errors produced by this validator. Finally, applications may be interested in
/// annotations produced by schemas over valid results, in this case the `evaluate` method can be used
/// to obtain this information.
///
/// If you are implementing `Validate` it is often sufficient to implement `validate` and
/// `is_valid`. `evaluate` is only necessary for validators which compose other validators. See the
/// documentation for `evaluate` for more information.
///
/// All methods accept a `ValidationContext` parameter for cycle detection in circular `$ref`
/// chains. Simple validators that don't recurse into child schemas can ignore this parameter.
pub(crate) trait Validate: ThreadBound {
    fn iter_errors<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> ErrorIterator<'i> {
        match self.validate(instance, location, ctx) {
            Ok(()) => no_error(),
            Err(err) => error(err),
        }
    }

    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool;

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>>;

    fn evaluate(
        &self,
        instance: &Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> EvaluationResult {
        let errors: Vec<ErrorDescription> = self
            .iter_errors(instance, location, ctx)
            .map(|e| ErrorDescription::from_validation_error(&e))
            .collect();
        if errors.is_empty() {
            EvaluationResult::valid_empty()
        } else {
            EvaluationResult::invalid_empty(errors)
        }
    }
}

/// The result of evaluating a validator against an instance. This is a "partial" result because it does not include information about
/// where the error or annotation occurred.
#[derive(PartialEq)]
pub(crate) enum EvaluationResult {
    Valid {
        /// Annotations produced by this validator
        annotations: Option<Annotations>,
        /// Children evaluation nodes
        children: Vec<EvaluationNode>,
    },
    Invalid {
        /// Errors which caused this schema to be invalid
        errors: Vec<ErrorDescription>,
        /// Children evaluation nodes
        children: Vec<EvaluationNode>,
        /// Potential annotations that should be reported as dropped on failure
        annotations: Option<Annotations>,
    },
}

impl EvaluationResult {
    /// Create an empty `EvaluationResult` which is valid
    pub(crate) fn valid_empty() -> EvaluationResult {
        EvaluationResult::Valid {
            annotations: None,
            children: Vec::new(),
        }
    }

    /// Create an empty `EvaluationResult` which is invalid
    pub(crate) fn invalid_empty(errors: Vec<ErrorDescription>) -> EvaluationResult {
        EvaluationResult::Invalid {
            errors,
            children: Vec::new(),
            annotations: None,
        }
    }

    /// Set the annotation that will be returned for the current validator. If this
    /// `EvaluationResult` is invalid then this method does nothing
    pub(crate) fn annotate(&mut self, new_annotations: Annotations) {
        match self {
            Self::Valid { annotations, .. } | Self::Invalid { annotations, .. } => {
                *annotations = Some(new_annotations);
            }
        }
    }

    /// Set the error that will be returned for the current validator. If this
    /// `EvaluationResult` is valid then this method converts this application into
    /// `EvaluationResult::Invalid`
    pub(crate) fn mark_errored(&mut self, error: ErrorDescription) {
        match self {
            Self::Invalid { errors, .. } => errors.push(error),
            Self::Valid {
                annotations,
                children,
            } => {
                *self = Self::Invalid {
                    errors: vec![error],
                    children: std::mem::take(children),
                    annotations: annotations.take(),
                }
            }
        }
    }

    pub(crate) fn from_children(children: Vec<EvaluationNode>) -> EvaluationResult {
        if children.iter().any(|node| !node.valid) {
            EvaluationResult::Invalid {
                errors: Vec::new(),
                children,
                annotations: None,
            }
        } else {
            EvaluationResult::Valid {
                annotations: None,
                children,
            }
        }
    }
}

impl From<EvaluationNode> for EvaluationResult {
    fn from(node: EvaluationNode) -> Self {
        if node.valid {
            EvaluationResult::Valid {
                annotations: None,
                children: vec![node],
            }
        } else {
            EvaluationResult::Invalid {
                errors: Vec::new(),
                children: vec![node],
                annotations: None,
            }
        }
    }
}

/// A compiled JSON Schema validator.
///
/// This structure represents a JSON Schema that has been parsed and compiled into
/// an efficient internal representation for validation. It contains the root node
/// of the schema tree and the configuration options used during compilation.
#[derive(Clone, Debug)]
pub struct Validator {
    pub(crate) root: SchemaNode,
    pub(crate) draft: Draft,
}

impl Validator {
    /// Create a default [`ValidationOptions`] for configuring JSON Schema validation.
    ///
    /// Use this to set the draft version and other validation parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use jsonschema::Draft;
    /// # let schema = serde_json::json!({});
    /// let validator = jsonschema::options()
    ///     .with_draft(Draft::Draft7)
    ///     .build(&schema);
    /// ```
    #[must_use]
    pub fn options() -> ValidationOptions {
        ValidationOptions::default()
    }
    /// Create a default [`ValidationOptions`] configured for async validation.
    ///
    /// Use this to set the draft version and other validation parameters when working
    /// with schemas that require async reference resolution.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # use jsonschema::Draft;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let schema = json!({
    ///     "$ref": "https://example.com/schema.json"
    /// });
    ///
    /// let validator = jsonschema::async_options()
    ///     .with_draft(Draft::Draft202012)
    ///     .build(&schema)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// For sync validation, use [`options()`](crate::options()) instead.
    #[cfg(feature = "resolve-async")]
    #[must_use]
    pub fn async_options() -> ValidationOptions<std::sync::Arc<dyn referencing::AsyncRetrieve>> {
        ValidationOptions::default()
    }
    /// Create a validator using the default options.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied `schema` is invalid for the selected draft or references cannot be resolved.
    pub fn new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        Self::options().build(schema)
    }
    /// Create a validator using the default async options.
    #[cfg(feature = "resolve-async")]
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied `schema` is invalid for the selected draft or references cannot be resolved.
    pub async fn async_new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        Self::async_options().build(schema).await
    }
    /// Validate `instance` against `schema` and return the first error if any.
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] describing why `instance` does not satisfy the schema.
    #[inline]
    pub fn validate<'i>(&self, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        let mut ctx = ValidationContext::new();
        self.root.validate(instance, &LazyLocation::new(), &mut ctx)
    }
    /// Run validation against `instance` and return an iterator over [`ValidationError`] in the error case.
    #[inline]
    #[must_use]
    pub fn iter_errors<'i>(&'i self, instance: &'i Value) -> ErrorIterator<'i> {
        let mut ctx = ValidationContext::new();
        self.root
            .iter_errors(instance, &LazyLocation::new(), &mut ctx)
    }
    /// Run validation against `instance` but return a boolean result instead of an iterator.
    /// It is useful for cases, where it is important to only know the fact if the data is valid or not.
    /// This approach is much faster, than [`Validator::validate`].
    #[must_use]
    #[inline]
    pub fn is_valid(&self, instance: &Value) -> bool {
        let mut ctx = ValidationContext::new();
        self.root.is_valid(instance, &mut ctx)
    }
    /// Evaluate the schema and expose structured output formats.
    #[must_use]
    #[inline]
    pub fn evaluate(&self, instance: &Value) -> Evaluation {
        let mut ctx = ValidationContext::new();
        let root = self
            .root
            .evaluate_instance(instance, &LazyLocation::new(), &mut ctx);
        Evaluation::new(root)
    }
    /// The [`Draft`] which was used to build this validator.
    #[must_use]
    pub fn draft(&self) -> Draft {
        self.draft
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::ValidationError,
        keywords::custom::Keyword,
        paths::{LazyLocation, Location},
        thread::ThreadBound,
        types::JsonType,
        Validator,
    };
    use fancy_regex::Regex;
    use num_cmp::NumCmp;
    use serde_json::{json, Map, Value};
    use std::sync::LazyLock;

    #[cfg(not(target_arch = "wasm32"))]
    fn load(path: &str, idx: usize) -> Value {
        use std::{fs::File, io::Read, path::Path};
        let path = Path::new(path);
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).ok().unwrap();
        let data: Value = serde_json::from_str(&content).unwrap();
        let case = &data.as_array().unwrap()[idx];
        case.get("schema").unwrap().clone()
    }

    #[test]
    fn only_keyword() {
        // When only one keyword is specified
        let schema = json!({"type": "string"});
        let validator = crate::validator_for(&schema).unwrap();
        let value1 = json!("AB");
        let value2 = json!(1);
        // And only this validator
        assert_eq!(validator.root.validators().len(), 1);
        assert!(validator.validate(&value1).is_ok());
        assert!(validator.validate(&value2).is_err());
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn validate_ref() {
        let schema = load("tests/suite/tests/draft7/ref.json", 1);
        let value = json!({"bar": 3});
        let validator = crate::validator_for(&schema).unwrap();
        assert!(validator.validate(&value).is_ok());
        let value = json!({"bar": true});
        assert!(validator.validate(&value).is_err());
    }

    #[test]
    fn wrong_schema_type() {
        let schema = json!([1]);
        let validator = crate::validator_for(&schema);
        assert!(validator.is_err());
    }

    #[test]
    fn multiple_errors() {
        let schema = json!({"minProperties": 2, "propertyNames": {"minLength": 3}});
        let value = json!({"a": 3});
        let validator = crate::validator_for(&schema).unwrap();
        let errors: Vec<_> = validator.iter_errors(&value).collect();
        assert_eq!(errors.len(), 2);
        assert_eq!(
            errors[0].to_string(),
            r#"{"a":3} has less than 2 properties"#
        );
        assert_eq!(errors[1].to_string(), r#""a" is shorter than 3 characters"#);
    }

    #[test]
    fn custom_keyword_definition() {
        /// Define a custom validator that verifies the object's keys consist of
        /// only ASCII representable characters.
        /// NOTE: This could be done with `propertyNames` + `pattern` but will be slower due to
        /// regex usage.
        struct CustomObjectValidator;
        impl Keyword for CustomObjectValidator {
            fn validate<'i>(
                &self,
                instance: &'i Value,
                location: &LazyLocation,
            ) -> Result<(), ValidationError<'i>> {
                for key in instance.as_object().unwrap().keys() {
                    if !key.is_ascii() {
                        return Err(ValidationError::custom(
                            Location::new(),
                            location.into(),
                            instance,
                            "Key is not ASCII",
                        ));
                    }
                }
                Ok(())
            }

            fn is_valid(&self, instance: &Value) -> bool {
                for (key, _value) in instance.as_object().unwrap() {
                    if !key.is_ascii() {
                        return false;
                    }
                }
                true
            }
        }

        fn custom_object_type_factory<'a>(
            _: &'a Map<String, Value>,
            schema: &'a Value,
            path: Location,
        ) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
            const EXPECTED: &str = "ascii-keys";
            if schema.as_str() == Some(EXPECTED) {
                Ok(Box::new(CustomObjectValidator))
            } else {
                Err(ValidationError::constant_string(
                    Location::new(),
                    path,
                    schema,
                    EXPECTED,
                ))
            }
        }

        // Define a JSON schema that enforces the top level object has ASCII keys and has at least 1 property
        let schema =
            json!({ "custom-object-type": "ascii-keys", "type": "object", "minProperties": 1 });
        let validator = crate::options()
            .with_keyword("custom-object-type", custom_object_type_factory)
            .build(&schema)
            .unwrap();

        // Verify schema validation detects object with too few properties
        let instance = json!({});
        assert!(validator.validate(&instance).is_err());
        assert!(!validator.is_valid(&instance));

        // Verify validator succeeds on a valid custom-object-type
        let instance = json!({ "a" : 1 });
        assert!(validator.validate(&instance).is_ok());
        assert!(validator.is_valid(&instance));

        // Verify validator detects invalid custom-object-type
        let instance = json!({ "å" : 1 });
        let error = validator.validate(&instance).expect_err("Should fail");
        assert_eq!(error.to_string(), "Key is not ASCII");
        assert!(!validator.is_valid(&instance));
    }

    #[test]
    fn custom_format_and_override_keyword() {
        /// Check that a string has some number of digits followed by a dot followed by exactly 2 digits.
        fn currency_format_checker(s: &str) -> bool {
            static CURRENCY_RE: LazyLock<Regex> = LazyLock::new(|| {
                Regex::new("^(0|([1-9]+[0-9]*))(\\.[0-9]{2})$").expect("Invalid regex")
            });
            CURRENCY_RE.is_match(s).expect("Invalid regex")
        }
        /// A custom keyword validator that overrides "minimum"
        /// so that "minimum" may apply to "currency"-formatted strings as well.
        struct CustomMinimumValidator {
            limit: f64,
            limit_val: Value,
            with_currency_format: bool,
            location: Location,
        }

        impl Keyword for CustomMinimumValidator {
            fn validate<'i>(
                &self,
                instance: &'i Value,
                location: &LazyLocation,
            ) -> Result<(), ValidationError<'i>> {
                if self.is_valid(instance) {
                    Ok(())
                } else {
                    Err(ValidationError::minimum(
                        self.location.clone(),
                        location.into(),
                        instance,
                        self.limit_val.clone(),
                    ))
                }
            }

            fn is_valid(&self, instance: &Value) -> bool {
                match instance {
                    // Numeric comparison should happen just like original behavior
                    Value::Number(instance) => {
                        if let Some(item) = instance.as_u64() {
                            !NumCmp::num_lt(item, self.limit)
                        } else if let Some(item) = instance.as_i64() {
                            !NumCmp::num_lt(item, self.limit)
                        } else {
                            let item = instance.as_f64().expect("Always valid");
                            !NumCmp::num_lt(item, self.limit)
                        }
                    }
                    // String comparison should cast currency-formatted
                    Value::String(instance) => {
                        if self.with_currency_format && currency_format_checker(instance) {
                            // all preconditions for minimum applying are met
                            let value = instance
                                .parse::<f64>()
                                .expect("format validated by regex checker");
                            !NumCmp::num_lt(value, self.limit)
                        } else {
                            true
                        }
                    }
                    // In all other cases, the "minimum" keyword should not apply
                    _ => true,
                }
            }
        }

        /// Build a validator that overrides the standard `minimum` keyword
        fn custom_minimum_factory<'a>(
            parent: &'a Map<String, Value>,
            schema: &'a Value,
            location: Location,
        ) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
            let limit = if let Value::Number(limit) = schema {
                limit.as_f64().expect("Always valid")
            } else {
                return Err(ValidationError::single_type_error(
                    // There is no metaschema definition for a custom keyword, hence empty `schema` pointer
                    Location::new(),
                    location,
                    schema,
                    JsonType::Number,
                ));
            };
            let with_currency_format = parent
                .get("format")
                .is_some_and(|format| format == "currency");
            Ok(Box::new(CustomMinimumValidator {
                limit,
                limit_val: schema.clone(),
                with_currency_format,
                location,
            }))
        }

        // Schema includes both the custom format and the overridden keyword
        let schema = json!({ "minimum": 2, "type": "string", "format": "currency" });
        let validator = crate::options()
            .with_format("currency", currency_format_checker)
            .with_keyword("minimum", custom_minimum_factory)
            .with_keyword("minimum-2", custom_minimum_factory)
            .should_validate_formats(true)
            .build(&schema)
            .expect("Invalid schema");

        // Control: verify schema validation rejects non-string types
        let instance = json!(15);
        assert!(validator.validate(&instance).is_err());
        assert!(!validator.is_valid(&instance));

        // Control: verify validator rejects ill-formatted strings
        let instance = json!("not a currency");
        assert!(validator.validate(&instance).is_err());
        assert!(!validator.is_valid(&instance));

        // Verify validator allows properly formatted strings that conform to custom keyword
        let instance = json!("3.00");
        assert!(validator.validate(&instance).is_ok());
        assert!(validator.is_valid(&instance));

        // Verify validator rejects properly formatted strings that do not conform to custom keyword
        let instance = json!("1.99");
        assert!(validator.validate(&instance).is_err());
        assert!(!validator.is_valid(&instance));

        // Define another schema that applies "minimum" to an integer to ensure original behavior
        let schema = json!({ "minimum": 2, "type": "integer" });
        let validator = crate::options()
            .with_format("currency", currency_format_checker)
            .with_keyword("minimum", custom_minimum_factory)
            .build(&schema)
            .expect("Invalid schema");

        // Verify schema allows integers greater than 2
        let instance = json!(3);
        assert!(validator.validate(&instance).is_ok());
        assert!(validator.is_valid(&instance));

        // Verify schema rejects integers less than 2
        let instance = json!(1);
        assert!(validator.validate(&instance).is_err());
        assert!(!validator.is_valid(&instance));

        // Invalid `minimum` value
        let schema = json!({ "minimum": "foo" });
        let error = crate::options()
            .with_keyword("minimum", custom_minimum_factory)
            .build(&schema)
            .expect_err("Should fail");
        assert_eq!(error.to_string(), "\"foo\" is not of type \"number\"");
    }

    #[test]
    fn test_validator_is_send_and_sync() {
        fn assert_send_sync<T: ThreadBound>() {}
        assert_send_sync::<Validator>();
    }

    #[test]
    fn test_validator_clone() {
        let schema = json!({"type": "string", "minLength": 3});
        let validator = crate::validator_for(&schema).expect("Valid schema");

        // Clone the validator
        let cloned = validator.clone();

        // Both validators should work independently
        assert!(validator.is_valid(&json!("hello")));
        assert!(!validator.is_valid(&json!("hi")));

        assert!(cloned.is_valid(&json!("hello")));
        assert!(!cloned.is_valid(&json!("hi")));

        // Verify they validate the same way
        assert_eq!(
            validator.is_valid(&json!("test")),
            cloned.is_valid(&json!("test"))
        );
    }
}
