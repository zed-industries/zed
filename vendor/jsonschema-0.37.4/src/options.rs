use crate::{
    compiler,
    content_encoding::{
        ContentEncodingCheckType, ContentEncodingConverterType,
        DEFAULT_CONTENT_ENCODING_CHECKS_AND_CONVERTERS,
    },
    content_media_type::{ContentMediaTypeCheckType, DEFAULT_CONTENT_MEDIA_TYPE_CHECKS},
    keywords::{custom::KeywordFactory, format::Format},
    paths::Location,
    retriever::DefaultRetriever,
    thread::ThreadBound,
    Keyword, ValidationError, Validator,
};
use ahash::AHashMap;
use referencing::{Draft, Resource, Retrieve};
use serde_json::Value;
use std::{fmt, marker::PhantomData, sync::Arc};

/// Configuration options for JSON Schema validation.
#[derive(Clone)]
pub struct ValidationOptions<R = Arc<dyn Retrieve>> {
    pub(crate) draft: Option<Draft>,
    content_media_type_checks: AHashMap<&'static str, Option<ContentMediaTypeCheckType>>,
    content_encoding_checks_and_converters:
        AHashMap<&'static str, Option<(ContentEncodingCheckType, ContentEncodingConverterType)>>,
    pub(crate) base_uri: Option<String>,
    /// Retriever for external resources
    pub(crate) retriever: R,
    /// Additional resources that should be addressable during validation.
    pub(crate) resources: AHashMap<String, Resource>,
    pub(crate) registry: Option<referencing::Registry>,
    formats: AHashMap<String, Arc<dyn Format>>,
    validate_formats: Option<bool>,
    pub(crate) validate_schema: bool,
    ignore_unknown_formats: bool,
    keywords: AHashMap<String, Arc<dyn KeywordFactory>>,
    pattern_options: PatternEngineOptions,
}

impl Default for ValidationOptions<Arc<dyn Retrieve>> {
    fn default() -> Self {
        ValidationOptions {
            draft: None,
            content_media_type_checks: AHashMap::default(),
            content_encoding_checks_and_converters: AHashMap::default(),
            base_uri: None,
            retriever: Arc::new(DefaultRetriever),
            resources: AHashMap::default(),
            registry: None,
            formats: AHashMap::default(),
            validate_formats: None,
            validate_schema: true,
            ignore_unknown_formats: true,
            keywords: AHashMap::default(),
            pattern_options: PatternEngineOptions::default(),
        }
    }
}

#[cfg(feature = "resolve-async")]
impl Default for ValidationOptions<Arc<dyn referencing::AsyncRetrieve>> {
    fn default() -> Self {
        ValidationOptions {
            draft: None,
            content_media_type_checks: AHashMap::default(),
            content_encoding_checks_and_converters: AHashMap::default(),
            base_uri: None,
            retriever: Arc::new(DefaultRetriever),
            resources: AHashMap::default(),
            registry: None,
            formats: AHashMap::default(),
            validate_formats: None,
            validate_schema: true,
            ignore_unknown_formats: true,
            keywords: AHashMap::default(),
            pattern_options: PatternEngineOptions::default(),
        }
    }
}

impl<R> ValidationOptions<R> {
    /// Return the draft version, or the default if not set.
    pub(crate) fn draft(&self) -> Draft {
        self.draft.unwrap_or_default()
    }
    /// Sets the JSON Schema draft version.
    ///
    /// ```rust
    /// use jsonschema::Draft;
    ///
    /// let options = jsonschema::options()
    ///     .with_draft(Draft::Draft4);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `Draft::Unknown` is provided. `Draft::Unknown` is internal-only
    /// and represents custom meta-schemas that are resolved automatically from
    /// the registry.
    #[inline]
    #[must_use]
    pub fn with_draft(mut self, draft: Draft) -> Self {
        assert!(
            draft != Draft::Unknown,
            "Draft::Unknown is internal-only and cannot be explicitly set. \
             Custom meta-schemas are resolved automatically when registered in the Registry."
        );
        self.draft = Some(draft);
        self
    }

    pub(crate) fn get_content_media_type_check(
        &self,
        media_type: &str,
    ) -> Option<ContentMediaTypeCheckType> {
        if let Some(value) = self.content_media_type_checks.get(media_type) {
            *value
        } else {
            DEFAULT_CONTENT_MEDIA_TYPE_CHECKS.get(media_type).copied()
        }
    }
    /// Add support for a custom content media type validation.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn check_custom_media_type(instance_string: &str) -> bool {
    ///     instance_string.starts_with("custom:")
    /// }
    ///
    /// let options = jsonschema::options()
    ///     .with_content_media_type("application/custom", check_custom_media_type);
    /// ```
    #[must_use]
    pub fn with_content_media_type(
        mut self,
        media_type: &'static str,
        media_type_check: ContentMediaTypeCheckType,
    ) -> Self {
        self.content_media_type_checks
            .insert(media_type, Some(media_type_check));
        self
    }
    /// Remove support for a specific content media type validation.
    #[must_use]
    pub fn without_content_media_type_support(mut self, media_type: &'static str) -> Self {
        self.content_media_type_checks.insert(media_type, None);
        self
    }

    #[inline]
    fn content_encoding_check_and_converter(
        &self,
        content_encoding: &str,
    ) -> Option<(ContentEncodingCheckType, ContentEncodingConverterType)> {
        if let Some(value) = self
            .content_encoding_checks_and_converters
            .get(content_encoding)
        {
            *value
        } else {
            DEFAULT_CONTENT_ENCODING_CHECKS_AND_CONVERTERS
                .get(content_encoding)
                .copied()
        }
    }

    pub(crate) fn content_encoding_check(
        &self,
        content_encoding: &str,
    ) -> Option<ContentEncodingCheckType> {
        if let Some((check, _)) = self.content_encoding_check_and_converter(content_encoding) {
            Some(check)
        } else {
            None
        }
    }

    pub(crate) fn get_content_encoding_convert(
        &self,
        content_encoding: &str,
    ) -> Option<ContentEncodingConverterType> {
        if let Some((_, converter)) = self.content_encoding_check_and_converter(content_encoding) {
            Some(converter)
        } else {
            None
        }
    }
    /// Add support for a custom content encoding.
    ///
    /// # Arguments
    ///
    /// * `encoding`: Name of the content encoding (e.g., "base64")
    /// * `check`: Validates the input string (return `true` if valid)
    /// * `converter`: Converts the input string, returning:
    ///   - `Err(ValidationError)`: For supported errors
    ///   - `Ok(None)`: If input is invalid
    ///   - `Ok(Some(content))`: If valid, with decoded content
    ///
    /// # Example
    ///
    /// ```rust
    /// use jsonschema::ValidationError;
    ///
    /// fn check(s: &str) -> bool {
    ///     s.starts_with("valid:")
    /// }
    ///
    /// fn convert(s: &str) -> Result<Option<String>, ValidationError<'static>> {
    ///     if s.starts_with("valid:") {
    ///         Ok(Some(s[6..].to_string()))
    ///     } else {
    ///         Ok(None)
    ///     }
    /// }
    ///
    /// let options = jsonschema::options()
    ///     .with_content_encoding("custom", check, convert);
    /// ```
    #[must_use]
    pub fn with_content_encoding(
        mut self,
        encoding: &'static str,
        check: ContentEncodingCheckType,
        converter: ContentEncodingConverterType,
    ) -> Self {
        self.content_encoding_checks_and_converters
            .insert(encoding, Some((check, converter)));
        self
    }
    /// Remove support for a specific content encoding.
    ///
    /// # Example
    ///
    /// ```rust
    /// let options = jsonschema::options()
    ///     .without_content_encoding_support("base64");
    /// ```
    #[must_use]
    pub fn without_content_encoding_support(mut self, content_encoding: &'static str) -> Self {
        self.content_encoding_checks_and_converters
            .insert(content_encoding, None);
        self
    }
    /// Establish an anchor for resolving relative schema references during validation.
    ///
    /// Relative URIs found within the schema will be interpreted against this base.
    /// This is especially useful when validating schemas loaded from sources without an inherent base URL.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let validator = jsonschema::options()
    /// // Define a base URI for resolving relative references.
    ///     .with_base_uri("https://example.com/schemas/")
    ///     .build(&json!({
    ///         "$id": "relative-schema.json",
    ///         "type": "object"
    ///     }))?;
    ///
    /// // Relative URIs in the schema will now resolve against "https://example.com/schemas/".
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn with_base_uri(mut self, base_uri: impl Into<String>) -> Self {
        self.base_uri = Some(base_uri.into());
        self
    }
    /// Add a custom schema, allowing it to be referenced by the specified URI during validation.
    ///
    /// This enables the use of additional in-memory schemas alongside the main schema being validated.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use jsonschema::Resource;
    ///
    /// let extra = Resource::from_contents(json!({"minimum": 5}));
    ///
    /// let validator = jsonschema::options()
    ///     .with_resource("urn:minimum-schema", extra)
    ///     .build(&json!({"$ref": "urn:minimum-schema"}))?;
    /// assert!(validator.is_valid(&json!(5)));
    /// assert!(!validator.is_valid(&json!(4)));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_resource(mut self, uri: impl Into<String>, resource: Resource) -> Self {
        self.resources.insert(uri.into(), resource);
        self
    }
    /// Add custom schemas, allowing them to be referenced by the specified URI during validation.
    ///
    /// This enables the use of additional in-memory schemas alongside the main schema being validated.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use jsonschema::Resource;
    ///
    /// let validator = jsonschema::options()
    ///     .with_resources([
    ///         (
    ///             "urn:minimum-schema",
    ///             Resource::from_contents(json!({"minimum": 5})),
    ///         ),
    ///         (
    ///             "urn:maximum-schema",
    ///             Resource::from_contents(json!({"maximum": 10})),
    ///         ),
    ///       ].into_iter())
    ///     .build(&json!({"$ref": "urn:minimum-schema"}))?;
    /// assert!(validator.is_valid(&json!(5)));
    /// assert!(!validator.is_valid(&json!(4)));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_resources(
        mut self,
        pairs: impl Iterator<Item = (impl Into<String>, Resource)>,
    ) -> Self {
        for (uri, resource) in pairs {
            self.resources.insert(uri.into(), resource);
        }
        self
    }
    /// Use external schema resources from the registry, making them accessible via references
    /// during validation.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use jsonschema::{Registry, Resource};
    ///
    /// let registry = Registry::try_new(
    ///     "urn:name-schema",
    ///     Resource::from_contents(json!({"type": "string"}))
    /// )?;
    /// let schema = json!({
    ///     "properties": {
    ///         "name": { "$ref": "urn:name-schema" }
    ///     }
    /// });
    /// let validator = jsonschema::options()
    ///     .with_registry(registry)
    ///     .build(&schema)?;
    /// assert!(validator.is_valid(&json!({ "name": "Valid String" })));
    /// assert!(!validator.is_valid(&json!({ "name": 123 })));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_registry(mut self, registry: referencing::Registry) -> Self {
        self.registry = Some(registry);
        self
    }
    /// Register a custom format validator.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use serde_json::json;
    /// fn my_format(s: &str) -> bool {
    ///    // Your awesome format check!
    ///    s.ends_with("42!")
    /// }
    /// # fn foo() {
    /// let schema = json!({"type": "string", "format": "custom"});
    /// let validator = jsonschema::options()
    ///     .with_format("custom", my_format)
    ///     .build(&schema)
    ///     .expect("Valid schema");
    ///
    /// assert!(!validator.is_valid(&json!("foo")));
    /// assert!(validator.is_valid(&json!("foo42!")));
    /// # }
    /// ```
    #[must_use]
    pub fn with_format<N, F>(mut self, name: N, format: F) -> Self
    where
        N: Into<String>,
        F: Fn(&str) -> bool + ThreadBound + 'static,
    {
        self.formats.insert(name.into(), Arc::new(format));
        self
    }
    pub(crate) fn get_format(&self, format: &str) -> Option<(&String, &Arc<dyn Format>)> {
        self.formats.get_key_value(format)
    }
    /// Disable schema validation during compilation.
    ///
    /// Used internally to prevent infinite recursion when validating meta-schemas.
    /// **Note**: Manually-crafted `ValidationError`s may still occur during compilation.
    #[inline]
    #[must_use]
    pub(crate) fn without_schema_validation(mut self) -> Self {
        self.validate_schema = false;
        self
    }
    /// Set whether to validate formats.
    ///
    /// Default behavior depends on the draft version. This method overrides
    /// the default, enabling or disabling format validation regardless of draft.
    #[inline]
    #[must_use]
    pub fn should_validate_formats(mut self, yes: bool) -> Self {
        self.validate_formats = Some(yes);
        self
    }
    pub(crate) fn validate_formats(&self) -> Option<bool> {
        self.validate_formats
    }
    /// Set whether to ignore unknown formats.
    ///
    /// By default, unknown formats are silently ignored. Set to `false` to report
    /// unrecognized formats as validation errors.
    #[must_use]
    pub fn should_ignore_unknown_formats(mut self, yes: bool) -> Self {
        self.ignore_unknown_formats = yes;
        self
    }

    pub(crate) const fn are_unknown_formats_ignored(&self) -> bool {
        self.ignore_unknown_formats
    }
    /// Register a custom keyword validator.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use jsonschema::{
    /// #    paths::{LazyLocation, Location},
    /// #    ErrorIterator, Keyword, ValidationError,
    /// # };
    /// # use serde_json::{json, Map, Value};
    /// # use std::iter::once;
    ///
    /// struct MyCustomValidator;
    ///
    /// impl Keyword for MyCustomValidator {
    ///     fn validate<'i>(
    ///         &self,
    ///         instance: &'i Value,
    ///         location: &LazyLocation,
    ///     ) -> Result<(), ValidationError<'i>> {
    ///         // ... validate instance ...
    ///         if !instance.is_object() {
    ///             return Err(ValidationError::custom(
    ///                 Location::new(),
    ///                 location.into(),
    ///                 instance,
    ///                 "Boom!",
    ///             ));
    ///         } else {
    ///             Ok(())
    ///         }
    ///     }
    ///     fn is_valid(&self, instance: &Value) -> bool {
    ///         // ... determine if instance is valid ...
    ///         true
    ///     }
    /// }
    ///
    /// // You can create a factory function, or use a closure to create new validator instances.
    /// fn custom_validator_factory<'a>(
    ///     parent: &'a Map<String, Value>,
    ///     value: &'a Value,
    ///     path: Location,
    /// ) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
    ///     Ok(Box::new(MyCustomValidator))
    /// }
    ///
    /// let validator = jsonschema::options()
    ///     .with_keyword("my-type", custom_validator_factory)
    ///     .with_keyword("my-type-with-closure", |_, _, _| Ok(Box::new(MyCustomValidator)))
    ///     .build(&json!({ "my-type": "my-schema"}))
    ///     .expect("A valid schema");
    ///
    /// assert!(validator.is_valid(&json!({ "a": "b"})));
    /// ```
    #[must_use]
    pub fn with_keyword<N, F>(mut self, name: N, factory: F) -> Self
    where
        N: Into<String>,
        F: for<'a> Fn(
                &'a serde_json::Map<String, Value>,
                &'a Value,
                Location,
            ) -> Result<Box<dyn Keyword>, ValidationError<'a>>
            + ThreadBound
            + 'static,
    {
        self.keywords.insert(name.into(), Arc::new(factory));
        self
    }

    pub(crate) fn get_keyword_factory(&self, name: &str) -> Option<&Arc<dyn KeywordFactory>> {
        self.keywords.get(name)
    }
}

impl ValidationOptions<Arc<dyn referencing::Retrieve>> {
    /// Build a JSON Schema validator using the current options.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "string"});
    /// let validator = jsonschema::options()
    ///     .build(&schema)
    ///     .expect("A valid schema");
    ///
    /// assert!(validator.is_valid(&json!("Hello")));
    /// assert!(!validator.is_valid(&json!(42)));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if `schema` is invalid for the selected draft or if referenced resources
    /// cannot be retrieved or resolved.
    pub fn build(&self, schema: &Value) -> Result<Validator, ValidationError<'static>> {
        compiler::build_validator(self, schema)
    }
    pub(crate) fn draft_for(&self, contents: &Value) -> Result<Draft, ValidationError<'static>> {
        // Preference:
        //  - Explicitly set
        //  - Autodetected (with registry resolution for custom meta-schemas)
        //  - Default
        if let Some(draft) = self.draft {
            Ok(draft)
        } else {
            let default = Draft::default();
            let detected = default.detect(contents);

            // If detected draft is Unknown (custom meta-schema), try to resolve it
            if detected == Draft::Unknown {
                if let Some(registry) = &self.registry {
                    if let Some(meta_schema_uri) = contents
                        .as_object()
                        .and_then(|obj| obj.get("$schema"))
                        .and_then(|s| s.as_str())
                    {
                        // Walk the meta-schema chain to find the underlying draft
                        return Self::resolve_draft_from_registry(meta_schema_uri, registry);
                    }
                }
            }

            Ok(detected)
        }
    }

    fn resolve_draft_from_registry(
        uri: &str,
        registry: &referencing::Registry,
    ) -> Result<Draft, ValidationError<'static>> {
        let uri = uri.trim_end_matches('#');
        crate::meta::walk_meta_schema_chain(uri, |current_uri| {
            let resolver = registry.try_resolver(current_uri)?;
            let resolved = resolver.lookup("")?;
            Ok(resolved.contents().clone())
        })
    }
    /// Set a retriever to fetch external resources.
    #[must_use]
    pub fn with_retriever(mut self, retriever: impl Retrieve + 'static) -> Self {
        self.retriever = Arc::new(retriever);
        self
    }
    /// Configure the regular expression engine used during validation for keywords like `pattern`
    /// or `patternProperties`.
    ///
    /// The default engine is [fancy-regex](https://docs.rs/fancy-regex), which supports advanced
    /// features (e.g., backreferences, look-around). Be aware that using these may lead to exponential
    /// runtime due to backtracking. For simpler regexes without these features, [regex](https://docs.rs/regex)
    /// provides linear time performance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_json::json;
    /// use jsonschema::PatternOptions;
    ///
    /// let schema = json!({"type": "string"});
    ///
    /// // Set backtracking limit to 20000.
    /// let validator = jsonschema::options()
    ///     .with_pattern_options(
    ///         PatternOptions::fancy_regex()
    ///             .backtrack_limit(20000)
    ///     )
    ///     .build(&schema)
    ///     .expect("A valid schema");
    /// ```
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_pattern_options<E>(mut self, options: PatternOptions<E>) -> Self {
        self.pattern_options = options.inner;
        self
    }
    pub(crate) fn pattern_options(&self) -> PatternEngineOptions {
        self.pattern_options
    }
}

#[cfg(feature = "resolve-async")]
impl ValidationOptions<Arc<dyn referencing::AsyncRetrieve>> {
    /// Build a JSON Schema validator using the current async options.
    ///
    /// # Errors
    ///
    /// Returns an error if `schema` is invalid for the selected draft or if referenced resources
    /// cannot be retrieved or resolved.
    pub async fn build(&self, schema: &Value) -> Result<Validator, ValidationError<'static>> {
        compiler::build_validator_async(self, schema).await
    }
    #[must_use]
    pub fn with_retriever(
        self,
        retriever: impl referencing::AsyncRetrieve + 'static,
    ) -> ValidationOptions<Arc<dyn referencing::AsyncRetrieve>> {
        ValidationOptions {
            draft: self.draft,
            retriever: Arc::new(retriever),
            content_media_type_checks: self.content_media_type_checks,
            content_encoding_checks_and_converters: self.content_encoding_checks_and_converters,
            base_uri: None,
            resources: self.resources,
            registry: self.registry,
            formats: self.formats,
            validate_formats: self.validate_formats,
            validate_schema: self.validate_schema,
            ignore_unknown_formats: self.ignore_unknown_formats,
            keywords: self.keywords,
            pattern_options: self.pattern_options,
        }
    }
    #[allow(clippy::unused_async)]
    pub(crate) async fn draft_for(
        &self,
        contents: &Value,
    ) -> Result<Draft, ValidationError<'static>> {
        // Preference:
        //  - Explicitly set
        //  - Autodetected
        //  - Default
        if let Some(draft) = self.draft {
            Ok(draft)
        } else {
            let default = Draft::default();
            Ok(default.detect(contents))
        }
    }
    /// Set a retriever to fetch external resources.
    pub(crate) fn with_blocking_retriever(
        self,
        retriever: impl Retrieve + 'static,
    ) -> ValidationOptions<Arc<dyn Retrieve>> {
        ValidationOptions {
            draft: self.draft,
            retriever: Arc::new(retriever),
            content_media_type_checks: self.content_media_type_checks,
            content_encoding_checks_and_converters: self.content_encoding_checks_and_converters,
            base_uri: None,
            resources: self.resources,
            registry: self.registry,
            formats: self.formats,
            validate_formats: self.validate_formats,
            validate_schema: self.validate_schema,
            ignore_unknown_formats: self.ignore_unknown_formats,
            keywords: self.keywords,
            pattern_options: self.pattern_options,
        }
    }
}

impl fmt::Debug for ValidationOptions {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("CompilationConfig")
            .field("draft", &self.draft)
            .field("content_media_type", &self.content_media_type_checks.keys())
            .field(
                "content_encoding",
                &self.content_encoding_checks_and_converters.keys(),
            )
            .finish()
    }
}

/// Configuration for how regular expressions are handled in schema keywords like `pattern` and `patternProperties`.
#[derive(Debug, Clone)]
pub struct PatternOptions<E> {
    inner: PatternEngineOptions,
    _marker: PhantomData<E>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum PatternEngineOptions {
    FancyRegex {
        backtrack_limit: Option<usize>,
        size_limit: Option<usize>,
        dfa_size_limit: Option<usize>,
    },
    Regex {
        size_limit: Option<usize>,
        dfa_size_limit: Option<usize>,
    },
}

/// Marker for using the `fancy-regex` engine that includes advanced features like lookarounds.
pub struct FancyRegex;
/// Marker for using the `regex` engine, that has fewer features than `fancy-regex` but guarantees
/// linear time performance.
pub struct Regex;

impl PatternOptions<FancyRegex> {
    /// Create a pattern configuration based on the `fancy-regex` engine.
    #[must_use]
    pub fn fancy_regex() -> PatternOptions<FancyRegex> {
        PatternOptions {
            inner: PatternEngineOptions::FancyRegex {
                backtrack_limit: None,
                size_limit: None,
                dfa_size_limit: None,
            },
            _marker: PhantomData,
        }
    }
    /// Limit for how many times backtracking should be attempted for fancy regexes (where
    /// backtracking is used). If this limit is exceeded, execution returns an error.
    /// This is for preventing a regex with catastrophic backtracking to run for too long.
    ///
    /// Default is `1_000_000` (1 million).
    #[must_use]
    pub fn backtrack_limit(mut self, limit: usize) -> Self {
        if let PatternEngineOptions::FancyRegex {
            ref mut backtrack_limit,
            ..
        } = self.inner
        {
            *backtrack_limit = Some(limit);
        }
        self
    }
    /// Set the approximate size limit, in bytes, of the compiled regex.
    #[must_use]
    pub fn size_limit(mut self, limit: usize) -> Self {
        if let PatternEngineOptions::FancyRegex {
            ref mut size_limit, ..
        } = self.inner
        {
            *size_limit = Some(limit);
        }
        self
    }
    /// Set the approximate capacity, in bytes, of the cache of transitions used by the lazy DFA.
    #[must_use]
    pub fn dfa_size_limit(mut self, limit: usize) -> Self {
        if let PatternEngineOptions::FancyRegex {
            ref mut dfa_size_limit,
            ..
        } = self.inner
        {
            *dfa_size_limit = Some(limit);
        }
        self
    }
}

impl PatternOptions<Regex> {
    /// Create a pattern configuration based on the `regex` engine.
    #[must_use]
    pub fn regex() -> PatternOptions<Regex> {
        PatternOptions {
            inner: PatternEngineOptions::Regex {
                size_limit: None,
                dfa_size_limit: None,
            },
            _marker: PhantomData,
        }
    }
    /// Set the approximate size limit, in bytes, of the compiled regex.
    #[must_use]
    pub fn size_limit(mut self, limit: usize) -> Self {
        if let PatternEngineOptions::Regex {
            ref mut size_limit, ..
        } = self.inner
        {
            *size_limit = Some(limit);
        }
        self
    }
    /// Set the approximate capacity, in bytes, of the cache of transitions used by the lazy DFA.
    #[must_use]
    pub fn dfa_size_limit(mut self, limit: usize) -> Self {
        if let PatternEngineOptions::Regex {
            ref mut dfa_size_limit,
            ..
        } = self.inner
        {
            *dfa_size_limit = Some(limit);
        }
        self
    }
}

impl Default for PatternEngineOptions {
    fn default() -> Self {
        PatternEngineOptions::FancyRegex {
            backtrack_limit: None,
            size_limit: None,
            dfa_size_limit: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use referencing::{Registry, Resource};
    use serde_json::json;

    fn custom(s: &str) -> bool {
        s.ends_with("42!")
    }

    #[test]
    fn custom_format() {
        let schema = json!({"type": "string", "format": "custom"});
        let validator = crate::options()
            .with_format("custom", custom)
            .should_validate_formats(true)
            .build(&schema)
            .expect("Valid schema");
        assert!(!validator.is_valid(&json!("foo")));
        assert!(validator.is_valid(&json!("foo42!")));
    }

    #[test]
    fn with_registry() {
        let registry = Registry::try_new(
            "urn:name-schema",
            Resource::from_contents(json!({"type": "string"})),
        )
        .expect("Invalid URI");
        let schema = json!({
            "properties": {
                "name": { "$ref": "urn:name-schema" }
            }
        });
        let validator = crate::options()
            .with_registry(registry)
            .build(&schema)
            .expect("Invalid schema");
        assert!(validator.is_valid(&json!({ "name": "Valid String" })));
        assert!(!validator.is_valid(&json!({ "name": 123 })));
    }

    #[test]
    fn test_fancy_regex_options_builder() {
        let options = PatternOptions::fancy_regex()
            .backtrack_limit(1_000_000)
            .size_limit(10_000)
            .dfa_size_limit(5000);

        if let PatternEngineOptions::FancyRegex {
            backtrack_limit,
            size_limit,
            dfa_size_limit,
        } = options.inner
        {
            assert_eq!(backtrack_limit, Some(1_000_000));
            assert_eq!(size_limit, Some(10_000));
            assert_eq!(dfa_size_limit, Some(5000));
        } else {
            panic!("Expected FancyRegex variant");
        }
    }

    #[test]
    #[should_panic(expected = "Draft::Unknown is internal-only and cannot be explicitly set")]
    fn with_draft_rejects_unknown() {
        let _options = crate::options().with_draft(Draft::Unknown);
    }

    #[test]
    fn custom_meta_schema_allowed_when_draft_overridden() {
        let schema = json!({
            "$schema": "json-schema:///custom/meta",
            "type": "string"
        });

        crate::options()
            .with_draft(Draft::Draft7)
            .build(&schema)
            .expect("Explicit draft override should bypass custom meta-schema registry checks");
    }

    #[test]
    fn test_regex_options_builder() {
        let options = PatternOptions::regex()
            .size_limit(20_000)
            .dfa_size_limit(8000);

        if let PatternEngineOptions::Regex {
            size_limit,
            dfa_size_limit,
        } = options.inner
        {
            assert_eq!(size_limit, Some(20_000));
            assert_eq!(dfa_size_limit, Some(8000));
        } else {
            panic!("Expected Regex variant");
        }
    }

    #[test]
    fn test_validate_against_definition() {
        // Root schema with multiple definitions
        let root_schema = json!({
            "$id": "https://example.com/root",
            "definitions": {
                "User": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "integer", "minimum": 0}
                    },
                    "required": ["name"]
                },
                "Product": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "integer"},
                        "title": {"type": "string"}
                    },
                    "required": ["id", "title"]
                }
            }
        });

        // Create a schema that references the specific definition
        let user_schema = json!({"$ref": "https://example.com/root#/definitions/User"});

        // Build validator with the root schema registered as a resource
        let validator = crate::options()
            .with_resource(
                "https://example.com/root",
                Resource::from_contents(root_schema),
            )
            .build(&user_schema)
            .expect("Valid schema");

        // Valid user
        assert!(validator.is_valid(&json!({"name": "Alice", "age": 30})));
        assert!(validator.is_valid(&json!({"name": "Bob"})));

        // Invalid: missing required field
        assert!(!validator.is_valid(&json!({"age": 25})));

        // Invalid: wrong type for age
        assert!(!validator.is_valid(&json!({"name": "Charlie", "age": "thirty"})));
    }
}
