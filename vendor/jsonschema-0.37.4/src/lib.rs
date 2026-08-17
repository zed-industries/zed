#![allow(clippy::unnecessary_wraps)]
//! A high-performance JSON Schema validator for Rust.
//!
//! - 📚 Support for popular JSON Schema drafts
//! - 🔧 Custom keywords and format validators
//! - 🌐 Blocking & non-blocking remote reference fetching (network/file)
//! - 🎨 Structured Output v1 reports (flag/list/hierarchical)
//! - ✨ Meta-schema validation for schema documents
//! - 🚀 WebAssembly support
//!
//! ## Supported drafts
//!
//! Compliance levels vary across drafts, with newer versions having some unimplemented keywords.
//!
//! - ![Draft 2020-12](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft2020-12.json)
//! - ![Draft 2019-09](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft2019-09.json)
//! - ![Draft 7](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft7.json)
//! - ![Draft 6](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft6.json)
//! - ![Draft 4](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft4.json)
//!
//! # Validation
//!
//! The `jsonschema` crate offers two main approaches to validation: one-off validation and reusable validators.
//! When external references are involved, the validator can be constructed using either blocking or non-blocking I/O.
//!
//!
//! For simple use cases where you need to validate an instance against a schema once, use [`is_valid`] or [`validate`] functions:
//!
//! ```rust
//! use serde_json::json;
//!
//! let schema = json!({"type": "string"});
//! let instance = json!("Hello, world!");
//!
//! assert!(jsonschema::is_valid(&schema, &instance));
//! assert!(jsonschema::validate(&schema, &instance).is_ok());
//! ```
//!
//! For better performance, especially when validating multiple instances against the same schema, build a validator once and reuse it:
//! If your schema contains external references, you can choose between blocking and non-blocking construction:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"type": "string"});
//! // Blocking construction - will fetch external references synchronously
//! let validator = jsonschema::validator_for(&schema)?;
//! // Non-blocking construction - will fetch external references asynchronously
//! # #[cfg(feature = "resolve-async")]
//! # async fn async_example() -> Result<(), Box<dyn std::error::Error>> {
//! # let schema = json!({"type": "string"});
//! let validator = jsonschema::async_validator_for(&schema).await?;
//! # Ok(())
//! # }
//!
//!  // Once constructed, validation is always synchronous as it works with in-memory data
//! assert!(validator.is_valid(&json!("Hello, world!")));
//! assert!(!validator.is_valid(&json!(42)));
//! assert!(validator.validate(&json!(42)).is_err());
//!
//! // Iterate over all errors
//! let instance = json!(42);
//! for error in validator.iter_errors(&instance) {
//!     eprintln!("Error: {}", error);
//!     eprintln!("Location: {}", error.instance_path());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Note on `format` keyword
//!
//! By default, format validation is draft‑dependent. To opt in for format checks, you can configure your validator like this:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use serde_json::json;
//! #
//! # let schema = json!({"type": "string"});
//! let validator = jsonschema::draft202012::options()
//!     .should_validate_formats(true)
//!     .build(&schema)?;
//! # Ok(())
//! # }
//! ```
//!
//! Once built, any `format` keywords in your schema will be actively validated according to the chosen draft.
//!
//! # Structured Output
//!
//! The `evaluate()` method provides access to structured validation output formats defined by
//! [JSON Schema Output v1](https://github.com/json-schema-org/json-schema-spec/blob/main/specs/output/jsonschema-validation-output-machines.md).
//! This is useful when you need detailed information about the validation process beyond simple pass/fail results.
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({
//!     "type": "object",
//!     "properties": {
//!         "name": {"type": "string"},
//!         "age": {"type": "number", "minimum": 0}
//!     },
//!     "required": ["name"]
//! });
//!
//! let validator = jsonschema::validator_for(&schema)?;
//! let instance = json!({"name": "Alice", "age": 30});
//!
//! // Evaluate the instance
//! let evaluation = validator.evaluate(&instance);
//!
//! // Flag format: Simple boolean validity
//! let flag = evaluation.flag();
//! assert!(flag.valid);
//!
//! // List format: Flat list of all evaluation steps
//! let list_output = serde_json::to_value(evaluation.list())?;
//! println!("List output: {}", serde_json::to_string_pretty(&list_output)?);
//!
//! // Hierarchical format: Nested tree structure
//! let hierarchical_output = serde_json::to_value(evaluation.hierarchical())?;
//! println!(
//!     "Hierarchical output: {}",
//!     serde_json::to_string_pretty(&hierarchical_output)?
//! );
//!
//! // Iterate over annotations collected during validation
//! for annotation in evaluation.iter_annotations() {
//!     println!("Annotation at {}: {:?}",
//!         annotation.instance_location,
//!         annotation.annotations
//!     );
//! }
//!
//! // Iterate over errors (if any)
//! for error in evaluation.iter_errors() {
//!     println!("Error: {}", error.error);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! The structured output formats are particularly useful for:
//! - **Debugging**: Understanding exactly which schema keywords matched or failed
//! - **User feedback**: Providing detailed, actionable error messages
//! - **Annotations**: Collecting metadata produced by successful validation
//! - **Tooling**: Building development tools that work with JSON Schema
//!
//! For example, validating `["hello", "oops"]` against a schema with both `prefixItems` and
//! `items` produces list output similar to:
//!
//! ```json
//! {
//!   "valid": false,
//!   "details": [
//!     {"valid": false, "evaluationPath": "", "schemaLocation": "", "instanceLocation": ""},
//!     {
//!       "valid": false,
//!       "evaluationPath": "/items",
//!       "schemaLocation": "/items",
//!       "instanceLocation": "",
//!       "droppedAnnotations": true
//!     },
//!     {
//!       "valid": false,
//!       "evaluationPath": "/items",
//!       "schemaLocation": "/items",
//!       "instanceLocation": "/1"
//!     },
//!     {
//!       "valid": false,
//!       "evaluationPath": "/items/type",
//!       "schemaLocation": "/items/type",
//!       "instanceLocation": "/1",
//!       "errors": {"type": "\"oops\" is not of type \"integer\""}
//!     },
//!     {"valid": true, "evaluationPath": "/prefixItems", "schemaLocation": "/prefixItems", "instanceLocation": "", "annotations": 0}
//!   ]
//! }
//! ```
//!
//! ## Output Formats
//!
//! ### Flag Format
//!
//! The simplest format, containing only a boolean validity indicator:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use serde_json::json;
//! # let schema = json!({"type": "string"});
//! # let validator = jsonschema::validator_for(&schema)?;
//! let evaluation = validator.evaluate(&json!("hello"));
//! let flag = evaluation.flag();
//!
//! let output = serde_json::to_value(flag)?;
//! // Output: {"valid": true}
//! # Ok(())
//! # }
//! ```
//!
//! ### List Format
//!
//! A flat list of all evaluation units, where each unit describes a validation step:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use serde_json::json;
//! let schema = json!({
//!     "allOf": [
//!         {"type": "number"},
//!         {"minimum": 0}
//!     ]
//! });
//! let validator = jsonschema::validator_for(&schema)?;
//! let evaluation = validator.evaluate(&json!(42));
//!
//! let list = evaluation.list();
//! let output = serde_json::to_value(list)?;
//! // Output includes all evaluation steps in a flat array
//! # Ok(())
//! # }
//! ```
//!
//! ### Hierarchical Format
//!
//! A nested tree structure that mirrors the schema's logical structure:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use serde_json::json;
//! let schema = json!({
//!     "allOf": [
//!         {"type": "number"},
//!         {"minimum": 0}
//!     ]
//! });
//! let validator = jsonschema::validator_for(&schema)?;
//! let evaluation = validator.evaluate(&json!(42));
//!
//! let hierarchical = evaluation.hierarchical();
//! let output = serde_json::to_value(hierarchical)?;
//! // Output has nested "details" arrays for sub-schema evaluations
//! # Ok(())
//! # }
//! ```
//!
//! # Meta-Schema Validation
//!
//! The crate provides functionality to validate JSON Schema documents themselves against their meta-schemas.
//! This ensures your schema documents are valid according to the JSON Schema specification.
//!
//! ```rust
//! use serde_json::json;
//!
//! let schema = json!({
//!     "type": "object",
//!     "properties": {
//!         "name": {"type": "string"},
//!         "age": {"type": "integer", "minimum": 0}
//!     }
//! });
//!
//! // Validate schema with automatic draft detection
//! assert!(jsonschema::meta::is_valid(&schema));
//! assert!(jsonschema::meta::validate(&schema).is_ok());
//!
//! // Invalid schema example
//! let invalid_schema = json!({
//!     "type": "invalid_type",  // must be one of the valid JSON Schema types
//!     "minimum": "not_a_number"
//! });
//! assert!(!jsonschema::meta::is_valid(&invalid_schema));
//! assert!(jsonschema::meta::validate(&invalid_schema).is_err());
//! ```
//!
//! # Configuration
//!
//! `jsonschema` provides several ways to configure and use JSON Schema validation.
//!
//! ## Draft-specific Modules
//!
//! The library offers modules for specific JSON Schema draft versions:
//!
//! - [`draft4`]
//! - [`draft6`]
//! - [`draft7`]
//! - [`draft201909`]
//! - [`draft202012`]
//!
//! Each module provides:
//! - A `new` function to create a validator
//! - An `is_valid` function for validation with a boolean result
//! - An `validate` function for getting the first validation error
//! - An `options` function to create a draft-specific configuration builder
//! - A `meta` module for draft-specific meta-schema validation
//!
//! Here's how you can explicitly use a specific draft version:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"type": "string"});
//!
//! // Instance validation
//! let validator = jsonschema::draft7::new(&schema)?;
//! assert!(validator.is_valid(&json!("Hello")));
//!
//! // Meta-schema validation
//! assert!(jsonschema::draft7::meta::is_valid(&schema));
//! # Ok(())
//! # }
//! ```
//!
//! You can also use the convenience [`is_valid`] and [`validate`] functions:
//!
//! ```rust
//! use serde_json::json;
//!
//! let schema = json!({"type": "number", "minimum": 0});
//! let instance = json!(42);
//!
//! assert!(jsonschema::draft202012::is_valid(&schema, &instance));
//! assert!(jsonschema::draft202012::validate(&schema, &instance).is_ok());
//! ```
//!
//! For more advanced configuration, you can use the draft-specific `options` function:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"type": "string", "format": "ends-with-42"});
//! let validator = jsonschema::draft202012::options()
//!     .with_format("ends-with-42", |s| s.ends_with("42"))
//!     .should_validate_formats(true)
//!     .build(&schema)?;
//!
//! assert!(validator.is_valid(&json!("Hello 42")));
//! assert!(!validator.is_valid(&json!("No!")));
//! # Ok(())
//! # }
//! ```
//!
//! ## General Configuration
//!
//! For configuration options that are not draft-specific, `jsonschema` provides a builder via `jsonschema::options()`.
//!
//! Here's an example:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"type": "string"});
//! let validator = jsonschema::options()
//!     // Add configuration options here
//!     .build(&schema)?;
//!
//! assert!(validator.is_valid(&json!("Hello")));
//! # Ok(())
//! # }
//! ```
//!
//! For a complete list of configuration options and their usage, please refer to the [`ValidationOptions`] struct.
//!
//! ## Automatic Draft Detection
//!
//! If you don't need to specify a particular draft version, you can use `jsonschema::validator_for`
//! which automatically detects the appropriate draft:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"$schema": "http://json-schema.org/draft-07/schema#", "type": "string"});
//! let validator = jsonschema::validator_for(&schema)?;
//!
//! assert!(validator.is_valid(&json!("Hello")));
//! # Ok(())
//! # }
//! ```
//!
//! # External References
//!
//! By default, `jsonschema` resolves HTTP references using `reqwest` and file references from the local file system.
//! Both blocking and non-blocking retrieval is supported during validator construction. Note that the validation
//! itself is always synchronous as it operates on in-memory data only.
//!
//! ```rust
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//!
//! let schema = json!({"$schema": "http://json-schema.org/draft-07/schema#", "type": "string"});
//!
//! // Building a validator with blocking retrieval (default)
//! let validator = jsonschema::validator_for(&schema)?;
//!
//! // Building a validator with non-blocking retrieval (requires `resolve-async` feature)
//! # #[cfg(feature = "resolve-async")]
//! let validator = jsonschema::async_validator_for(&schema).await?;
//!
//! // Validation is always synchronous
//! assert!(validator.is_valid(&json!("Hello")));
//! # Ok(())
//! # }
//! ```
//!
//! To enable HTTPS support, add the `rustls-tls` feature to `reqwest` in your `Cargo.toml`:
//!
//! ```toml
//! reqwest = { version = "*", features = ["rustls-tls"] }
//! ```
//!
//! You can disable the default behavior using crate features:
//!
//! - Disable HTTP resolving: `default-features = false, features = ["resolve-file"]`
//! - Disable file resolving: `default-features = false, features = ["resolve-http"]`
//! - Enable async resolution: `features = ["resolve-async"]`
//! - Disable all resolving: `default-features = false`
//!
//! ## Custom retrievers
//!
//! You can implement custom retrievers for both blocking and non-blocking retrieval:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::{collections::HashMap, sync::Arc};
//! use jsonschema::{Retrieve, Uri};
//! use serde_json::{json, Value};
//!
//! struct InMemoryRetriever {
//!     schemas: HashMap<String, Value>,
//! }
//!
//! impl Retrieve for InMemoryRetriever {
//!
//!    fn retrieve(
//!        &self,
//!        uri: &Uri<String>,
//!    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
//!         self.schemas
//!             .get(uri.as_str())
//!             .cloned()
//!             .ok_or_else(|| format!("Schema not found: {uri}").into())
//!     }
//! }
//!
//! let mut schemas = HashMap::new();
//! schemas.insert(
//!     "https://example.com/person.json".to_string(),
//!     json!({
//!         "type": "object",
//!         "properties": {
//!             "name": { "type": "string" },
//!             "age": { "type": "integer" }
//!         },
//!         "required": ["name", "age"]
//!     }),
//! );
//!
//! let retriever = InMemoryRetriever { schemas };
//!
//! let schema = json!({
//!     "$ref": "https://example.com/person.json"
//! });
//!
//! let validator = jsonschema::options()
//!     .with_retriever(retriever)
//!     .build(&schema)?;
//!
//! assert!(validator.is_valid(&json!({
//!     "name": "Alice",
//!     "age": 30
//! })));
//!
//! assert!(!validator.is_valid(&json!({
//!     "name": "Bob"
//! })));
//! #    Ok(())
//! # }
//! ```
//!
//! And non-blocking version with the `resolve-async` feature enabled:
//!
//! ```rust
//! # #[cfg(feature = "resolve-async")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use jsonschema::{AsyncRetrieve, Registry, Resource, Uri};
//! use serde_json::{Value, json};
//!
//! struct HttpRetriever;
//!
//! #[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
//! #[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
//! impl AsyncRetrieve for HttpRetriever {
//!     async fn retrieve(
//!         &self,
//!         uri: &Uri<String>,
//!     ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
//!         reqwest::get(uri.as_str())
//!             .await?
//!             .json()
//!             .await
//!             .map_err(Into::into)
//!     }
//! }
//!
//! // Then use it to build a validator
//! let validator = jsonschema::async_options()
//!     .with_retriever(HttpRetriever)
//!     .build(&json!({"$ref": "https://example.com/user.json"}))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! On `wasm32` targets, use `async_trait::async_trait(?Send)` so your retriever can rely on `Rc`, `JsFuture`, or other non-thread-safe types.
//!
//! ## Validating against schema definitions
//!
//! When working with large schemas containing multiple definitions (e.g., Open API schemas, DAP schemas),
//! you may want to validate data against a specific definition rather than the entire schema. This can be
//! achieved by registering the root schema as a resource and creating a wrapper schema that references
//! the target definition:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use serde_json::json;
//! use jsonschema::Resource;
//!
//! // Root schema with multiple definitions
//! let root_schema = json!({
//!     "$id": "https://example.com/root",
//!     "definitions": {
//!         "User": {
//!             "type": "object",
//!             "properties": {
//!                 "name": {"type": "string"},
//!                 "age": {"type": "integer", "minimum": 0}
//!             },
//!             "required": ["name"]
//!         },
//!         "Product": {
//!             "type": "object",
//!             "properties": {
//!                 "id": {"type": "integer"},
//!                 "title": {"type": "string"}
//!             },
//!             "required": ["id", "title"]
//!         }
//!     }
//! });
//!
//! // Create a schema that references the specific definition you want to validate against
//! let user_schema = json!({"$ref": "https://example.com/root#/definitions/User"});
//!
//! // Register the root schema and build validator for the specific definition
//! let validator = jsonschema::options()
//!     .with_resource("https://example.com/root", Resource::from_contents(root_schema))
//!     .build(&user_schema)?;
//!
//! // Now validate data against just the User definition
//! assert!(validator.is_valid(&json!({"name": "Alice", "age": 30})));
//! assert!(!validator.is_valid(&json!({"age": 25})));  // Missing required "name"
//! # Ok(())
//! # }
//! ```
//!
//! This pattern is particularly useful when:
//! - Working with API schemas that define multiple request/response types
//! - Validating configuration snippets against specific sections of a larger schema
//! - Testing individual schema components in isolation
//!
//! # Regular Expression Configuration
//!
//! The `jsonschema` crate allows configuring the regular expression engine used for validating
//! keywords like `pattern` or `patternProperties`.
//!
//! By default, the crate uses [`fancy-regex`](https://docs.rs/fancy-regex), which supports advanced
//! regular expression features such as lookaround and backreferences.
//!
//! The primary motivation for switching to the `regex` engine is security and performance:
//! it guarantees linear-time matching, preventing potential Denial of Service attacks from malicious patterns
//! in user-provided schemas while offering better performance with a smaller feature set.
//!
//! You can configure the engine at **runtime** using the [`PatternOptions`] API:
//!
//! ### Example: Configure `fancy-regex` with Backtracking Limit
//!
//! ```rust
//! use serde_json::json;
//! use jsonschema::PatternOptions;
//!
//! let schema = json!({
//!     "type": "string",
//!     "pattern": "^(a+)+$"
//! });
//!
//! let validator = jsonschema::options()
//!     .with_pattern_options(
//!         PatternOptions::fancy_regex()
//!             .backtrack_limit(10_000)
//!     )
//!     .build(&schema)
//!     .expect("A valid schema");
//! ```
//!
//! ### Example: Use the `regex` Engine Instead
//!
//! ```rust
//! use serde_json::json;
//! use jsonschema::PatternOptions;
//!
//! let schema = json!({
//!     "type": "string",
//!     "pattern": "^a+$"
//! });
//!
//! let validator = jsonschema::options()
//!     .with_pattern_options(PatternOptions::regex())
//!     .build(&schema)
//!     .expect("A valid schema");
//! ```
//!
//! ### Notes
//!
//! - If neither engine is explicitly set, `fancy-regex` is used by default.
//! - Regular expressions that rely on advanced features like `(?<=...)` (lookbehind) or backreferences (`\1`) will fail with the `regex` engine.
//!
//! # Custom Keywords
//!
//! `jsonschema` allows you to extend its functionality by implementing custom validation logic through custom keywords.
//! This feature is particularly useful when you need to validate against domain-specific rules that aren't covered by the standard JSON Schema keywords.
//!
//! To implement a custom keyword, you need to:
//! 1. Create a struct that implements the [`Keyword`] trait
//! 2. Create a factory function or closure that produces instances of your custom keyword
//! 3. Register the custom keyword with the [`Validator`] instance using the [`ValidationOptions::with_keyword`] method
//!
//! Here's a complete example:
//!
//! ```rust
//! use jsonschema::{
//!     paths::{LazyLocation, Location},
//!     Keyword, ValidationError,
//! };
//! use serde_json::{json, Map, Value};
//! use std::iter::once;
//!
//! // Step 1: Implement the Keyword trait
//! struct EvenNumberValidator;
//!
//! impl Keyword for EvenNumberValidator {
//!     fn validate<'i>(
//!         &self,
//!         instance: &'i Value,
//!         location: &LazyLocation,
//!     ) -> Result<(), ValidationError<'i>> {
//!         if let Value::Number(n) = instance {
//!             if n.as_u64().map_or(false, |n| n % 2 == 0) {
//!                 Ok(())
//!             } else {
//!                 return Err(ValidationError::custom(
//!                     Location::new(),
//!                     location.into(),
//!                     instance,
//!                     "Number must be even",
//!                 ));
//!             }
//!         } else {
//!             Err(ValidationError::custom(
//!                 Location::new(),
//!                 location.into(),
//!                 instance,
//!                 "Value must be a number",
//!             ))
//!         }
//!     }
//!
//!     fn is_valid(&self, instance: &Value) -> bool {
//!         instance.as_u64().map_or(false, |n| n % 2 == 0)
//!     }
//! }
//!
//! // Step 2: Create a factory function
//! fn even_number_validator_factory<'a>(
//!     _parent: &'a Map<String, Value>,
//!     value: &'a Value,
//!     path: Location,
//! ) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
//!     // You can use the `value` parameter to configure your validator if needed
//!     if value.as_bool() == Some(true) {
//!         Ok(Box::new(EvenNumberValidator))
//!     } else {
//!         Err(ValidationError::custom(
//!             Location::new(),
//!             path,
//!             value,
//!             "The 'even-number' keyword must be set to true",
//!         ))
//!     }
//! }
//!
//! // Step 3: Use the custom keyword
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let schema = json!({"even-number": true, "type": "integer"});
//!     let validator = jsonschema::options()
//!         .with_keyword("even-number", even_number_validator_factory)
//!         .build(&schema)?;
//!
//!     assert!(validator.is_valid(&json!(2)));
//!     assert!(!validator.is_valid(&json!(3)));
//!     assert!(!validator.is_valid(&json!("not a number")));
//!
//!     Ok(())
//! }
//! ```
//!
//! In this example, we've created a custom `even-number` keyword that validates whether a number is even.
//! The `EvenNumberValidator` implements the actual validation logic, while the `even_number_validator_factory`
//! creates instances of the validator and allows for additional configuration based on the keyword's value in the schema.
//!
//! You can also use a closure instead of a factory function for simpler cases:
//!
//! ```rust
//! # use jsonschema::{
//! #     paths::LazyLocation,
//! #     Keyword, ValidationError,
//! # };
//! # use serde_json::{json, Map, Value};
//! # use std::iter::once;
//! #
//! # struct EvenNumberValidator;
//! #
//! # impl Keyword for EvenNumberValidator {
//! #     fn validate<'i>(
//! #         &self,
//! #         instance: &'i Value,
//! #         location: &LazyLocation,
//! #     ) -> Result<(), ValidationError<'i>> {
//! #         Ok(())
//! #     }
//! #
//! #     fn is_valid(&self, instance: &Value) -> bool {
//! #         true
//! #     }
//! # }
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let schema = json!({"even-number": true, "type": "integer"});
//! let validator = jsonschema::options()
//!     .with_keyword("even-number", |_, _, _| {
//!         Ok(Box::new(EvenNumberValidator))
//!     })
//!     .build(&schema)?;
//! # Ok(())
//! # }
//! ```
//!
//! # Custom Formats
//!
//! JSON Schema allows for format validation through the `format` keyword. While `jsonschema`
//! provides built-in validators for standard formats, you can also define custom format validators
//! for domain-specific string formats.
//!
//! To implement a custom format validator:
//!
//! 1. Define a function or a closure that takes a `&str` and returns a `bool`.
//! 2. Register the function with `jsonschema::options().with_format()`.
//!
//! ```rust
//! use serde_json::json;
//!
//! // Step 1: Define the custom format validator function
//! fn ends_with_42(s: &str) -> bool {
//!     s.ends_with("42!")
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Step 2: Create a schema using the custom format
//! let schema = json!({
//!     "type": "string",
//!     "format": "ends-with-42"
//! });
//!
//! // Step 3: Build the validator with the custom format
//! let validator = jsonschema::options()
//!     .with_format("ends-with-42", ends_with_42)
//!     .with_format("ends-with-43", |s| s.ends_with("43!"))
//!     .should_validate_formats(true)
//!     .build(&schema)?;
//!
//! // Step 4: Validate instances
//! assert!(validator.is_valid(&json!("Hello42!")));
//! assert!(!validator.is_valid(&json!("Hello43!")));
//! assert!(!validator.is_valid(&json!(42))); // Not a string
//! #    Ok(())
//! # }
//! ```
//!
//! ### Notes on Custom Format Validators
//!
//! - Custom format validators are only called for string instances.
//! - In newer drafts, `format` is purely an annotation and won’t do any checking unless you
//!   opt in by calling `.should_validate_formats(true)` on your options builder. If you omit
//!   it, all `format` keywords are ignored at validation time.
//!
//! # Arbitrary Precision Numbers
//!
//! Enable the `arbitrary-precision` feature for exact validation of numbers beyond standard numeric ranges:
//!
//! ```toml
//! jsonschema = { version = "x.y.z", features = ["arbitrary-precision"] }
//! ```
//!
//! This provides:
//! - Arbitrarily large integers (e.g., `18446744073709551616`)
//! - Exact decimal precision without `f64` rounding (e.g., `0.1`, `0.3`)
//!
//! **Important**: Precision is only preserved when parsing JSON from strings. Using Rust literals
//! or the `json!()` macro converts numbers to `f64`, losing precision.
//!
//! ```rust
//! # use jsonschema::Validator;
//! // Precision preserved - parsed from JSON string
//! let schema = serde_json::from_str(r#"{"minimum": 0.1}"#)?;
//! let instance = serde_json::from_str("0.3")?;
//! let validator = Validator::new(&schema)?;
//! assert!(validator.is_valid(&instance));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # WebAssembly support
//!
//! `jsonschema` supports WebAssembly with different capabilities based on the target platform:
//!
//! ## Browser/JavaScript (`wasm32-unknown-unknown`)
//!
//! When targeting browser or JavaScript environments, external reference resolution is not
//! supported by default due to platform limitations:
//!    - No filesystem access (`resolve-file` feature is not available)
//!    - No synchronous HTTP requests (`resolve-http` feature is not available)
//!
//! To use `jsonschema` in these environments, disable default features:
//!
//! ```toml
//! jsonschema = { version = "x.y.z", default-features = false }
//! ```
//!
//! Note: Attempting to compile with `resolve-http` or `resolve-file` features on
//! `wasm32-unknown-unknown` will result in a compile error.
//!
//! For external references in browser environments, implement a custom retriever that uses
//! browser APIs (like `fetch`). See the [External References](#external-references) section.
//!
//! ## WASI (`wasm32-wasip1` / `wasm32-wasip2`)
//!
//! WASI environments (preview 1 and preview 2) can compile schemas and run validators, but the bundled
//! HTTP retriever depends on `reqwest`’s blocking client, which isn't available on these targets. Use
//! file access and custom retrievers instead.
//!
//! **Supported:**
//! - Blocking file resolution (`resolve-file` feature)
//! - Custom blocking retrievers (including wrapping async operations)
//! - Custom async retrievers via the `resolve-async` feature (for example, `jsonschema::async_options`
//!   together with your own async runtime)
//!
//! **Not Supported:**
//! - The bundled HTTP retriever (depends on `reqwest`’s blocking client)
//!
//! ```toml
//! jsonschema = { version = "x.y.z", default-features = false, features = ["resolve-file"] }
//! ```
//!
//! **Workaround for HTTP:** Implement a custom blocking or async [`Retrieve`] that uses your preferred
//! HTTP client, and enable `resolve-async` if you want to build validators through `async_options()`
//! on WASI.

#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    any(feature = "resolve-file", feature = "resolve-http")
))]
compile_error!(
    "Features 'resolve-http' and 'resolve-file' are not supported on wasm32-unknown-unknown"
);

pub(crate) mod compiler;
mod content_encoding;
mod content_media_type;
mod ecma;
pub mod error;
mod evaluation;
#[doc(hidden)]
pub mod ext;
mod keywords;
mod node;
mod options;
pub mod output;
pub mod paths;
pub(crate) mod properties;
pub(crate) mod regex;
mod retriever;
pub(crate) mod thread;
pub mod types;
mod validator;

pub use error::{ErrorIterator, MaskedValidationError, ValidationError, ValidationErrors};
pub use evaluation::{
    AnnotationEntry, ErrorEntry, Evaluation, FlagOutput, HierarchicalOutput, ListOutput,
};
pub use keywords::custom::Keyword;
pub use options::{FancyRegex, PatternOptions, Regex, ValidationOptions};
pub use referencing::{
    Draft, Error as ReferencingError, Registry, RegistryOptions, Resource, Retrieve, Uri,
};
pub use types::{JsonType, JsonTypeSet, JsonTypeSetIterator};
pub use validator::Validator;

#[cfg(feature = "resolve-async")]
pub use referencing::AsyncRetrieve;

use serde_json::Value;

/// Validate `instance` against `schema` and get a `true` if the instance is valid and `false`
/// otherwise. Draft is detected automatically.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"maxLength": 5});
/// let instance = json!("foo");
/// assert!(jsonschema::is_valid(&schema, &instance));
/// ```
///
/// # Panics
///
/// This function panics if an invalid schema is passed.
#[must_use]
#[inline]
pub fn is_valid(schema: &Value, instance: &Value) -> bool {
    validator_for(schema)
        .expect("Invalid schema")
        .is_valid(instance)
}

/// Validate `instance` against `schema` and return the first error if any. Draft is detected automatically.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"maxLength": 5});
/// let instance = json!("foo");
/// assert!(jsonschema::validate(&schema, &instance).is_ok());
/// ```
///
/// # Errors
///
/// Returns the first [`ValidationError`] encountered when `instance` violates `schema`.
///
/// # Panics
///
/// This function panics if an invalid schema is passed.
#[inline]
pub fn validate<'i>(schema: &Value, instance: &'i Value) -> Result<(), ValidationError<'i>> {
    validator_for(schema)
        .expect("Invalid schema")
        .validate(instance)
}

/// Evaluate `instance` against `schema` and return structured validation output. Draft is detected automatically.
///
/// Returns an [`Evaluation`] containing detailed validation results in JSON Schema Output v1 format,
/// including annotations and errors across the entire validation tree.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"type": "string", "minLength": 3});
/// let instance = json!("foo");
/// let evaluation = jsonschema::evaluate(&schema, &instance);
/// assert!(evaluation.flag().valid);
/// ```
///
/// # Panics
///
/// This function panics if an invalid schema is passed.
#[must_use]
#[inline]
pub fn evaluate(schema: &Value, instance: &Value) -> Evaluation {
    validator_for(schema)
        .expect("Invalid schema")
        .evaluate(instance)
}

/// Create a validator for the input schema with automatic draft detection and default options.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({"minimum": 5});
/// let instance = json!(42);
///
/// let validator = jsonschema::validator_for(&schema)?;
/// assert!(validator.is_valid(&instance));
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if the schema is invalid or external references cannot be resolved.
pub fn validator_for(schema: &Value) -> Result<Validator, ValidationError<'static>> {
    Validator::new(schema)
}

/// Create a validator for the input schema with automatic draft detection and default options,
/// using non-blocking retrieval for external references.
///
/// This is the async counterpart to [`validator_for`]. Note that only the construction is
/// asynchronous - validation itself is always synchronous.
///
/// # Examples
///
/// ```rust
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "user": { "$ref": "https://example.com/user.json" }
///     }
/// });
///
/// let validator = jsonschema::async_validator_for(&schema).await?;
/// assert!(validator.is_valid(&json!({"user": {"name": "Alice"}})));
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if the schema is invalid or external references cannot be resolved.
#[cfg(feature = "resolve-async")]
pub async fn async_validator_for(schema: &Value) -> Result<Validator, ValidationError<'static>> {
    Validator::async_new(schema).await
}

/// Create a builder for configuring JSON Schema validation options.
///
/// This function returns a [`ValidationOptions`] struct, which allows you to set various
/// options for JSON Schema validation. You can use this builder to specify
/// the draft version, set custom formats, and more.
///
/// # Examples
///
/// Basic usage with draft specification:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
/// use jsonschema::Draft;
///
/// let schema = json!({"type": "string"});
/// let validator = jsonschema::options()
///     .with_draft(Draft::Draft7)
///     .build(&schema)?;
///
/// assert!(validator.is_valid(&json!("Hello")));
/// # Ok(())
/// # }
/// ```
///
/// Advanced configuration:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({"type": "string", "format": "custom"});
/// let validator = jsonschema::options()
///     .with_format("custom", |value| value.len() == 3)
///     .should_validate_formats(true)
///     .build(&schema)?;
///
/// assert!(validator.is_valid(&json!("abc")));
/// assert!(!validator.is_valid(&json!("abcd")));
/// # Ok(())
/// # }
/// ```
///
/// See [`ValidationOptions`] for all available configuration options.
#[must_use]
pub fn options() -> ValidationOptions {
    Validator::options()
}

/// Create a builder for configuring JSON Schema validation options.
///
/// This function returns a [`ValidationOptions`] struct which allows you to set various options for JSON Schema validation.
/// External references will be retrieved using non-blocking I/O.
///
/// # Examples
///
/// Basic usage with external references:
///
/// ```rust
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({
///     "$ref": "https://example.com/user.json"
/// });
///
/// let validator = jsonschema::async_options()
///     .build(&schema)
///     .await?;
///
/// assert!(validator.is_valid(&json!({"name": "Alice"})));
/// # Ok(())
/// # }
/// ```
///
/// Advanced configuration:
///
/// ```rust
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::{Value, json};
/// use jsonschema::{Draft, AsyncRetrieve, Uri};
///
/// // Custom async retriever
/// struct MyRetriever;
///
/// #[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
/// #[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
/// impl AsyncRetrieve for MyRetriever {
///     async fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
///         // Custom retrieval logic
///         Ok(json!({}))
///     }
/// }
///
/// let schema = json!({
///     "$ref": "https://example.com/user.json"
/// });
/// let validator = jsonschema::async_options()
///     .with_draft(Draft::Draft202012)
///     .with_retriever(MyRetriever)
///     .build(&schema)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// On `wasm32` targets, annotate your implementation with `async_trait::async_trait(?Send)` to drop the `Send + Sync` requirement.
///
/// See [`ValidationOptions`] for all available configuration options.
#[cfg(feature = "resolve-async")]
#[must_use]
pub fn async_options() -> ValidationOptions<std::sync::Arc<dyn AsyncRetrieve>> {
    Validator::async_options()
}

/// Functionality for validating JSON Schema documents against their meta-schemas.
pub mod meta {
    use crate::{error::ValidationError, Draft};
    use ahash::AHashSet;
    use referencing::{Registry, Retrieve};
    use serde_json::Value;

    pub use validator_handle::MetaValidator;

    /// Create a meta-validation options builder.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    /// use jsonschema::{Registry, Resource};
    ///
    /// let custom_meta = Resource::from_contents(json!({
    ///     "$schema": "https://json-schema.org/draft/2020-12/schema",
    ///     "type": "object"
    /// }));
    ///
    /// let registry = Registry::try_new(
    ///     "http://example.com/meta",
    ///     custom_meta
    /// ).unwrap();
    ///
    /// let schema = json!({
    ///     "$schema": "http://example.com/meta",
    ///     "type": "string"
    /// });
    ///
    /// assert!(jsonschema::meta::options()
    ///     .with_registry(registry)
    ///     .is_valid(&schema));
    /// ```
    #[must_use]
    pub fn options() -> MetaSchemaOptions {
        MetaSchemaOptions::default()
    }

    /// Options for meta-schema validation.
    #[derive(Clone, Default)]
    pub struct MetaSchemaOptions {
        registry: Option<Registry>,
    }

    impl MetaSchemaOptions {
        /// Use a registry for resolving custom meta-schemas.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        /// use jsonschema::{Registry, Resource};
        ///
        /// let custom_meta = Resource::from_contents(json!({
        ///     "$schema": "https://json-schema.org/draft/2020-12/schema",
        ///     "type": "object"
        /// }));
        ///
        /// let registry = Registry::try_new(
        ///     "http://example.com/meta",
        ///     custom_meta
        /// ).unwrap();
        ///
        /// let options = jsonschema::meta::options()
        ///     .with_registry(registry);
        /// ```
        #[must_use]
        pub fn with_registry(mut self, registry: Registry) -> Self {
            self.registry = Some(registry);
            self
        }

        /// Check if a schema is valid according to its meta-schema.
        ///
        /// # Panics
        ///
        /// Panics if the meta-schema cannot be resolved.
        #[must_use]
        pub fn is_valid(&self, schema: &Value) -> bool {
            match try_meta_validator_for(schema, self.registry.as_ref()) {
                Ok(validator) => validator.as_ref().is_valid(schema),
                Err(e) => panic!("Failed to resolve meta-schema: {e}"),
            }
        }

        /// Validate a schema according to its meta-schema.
        ///
        /// # Errors
        ///
        /// Returns [`ValidationError`] if the schema is invalid or if the meta-schema cannot be resolved.
        pub fn validate<'a>(&self, schema: &'a Value) -> Result<(), ValidationError<'a>> {
            let validator = try_meta_validator_for(schema, self.registry.as_ref())?;
            validator.as_ref().validate(schema)
        }
    }

    mod validator_handle {
        use crate::Validator;
        use std::{marker::PhantomData, ops::Deref};

        /// Handle to a draft-specific meta-schema [`Validator`]. Borrows cached validators on native
        /// targets and owns validators on `wasm32`.
        pub struct MetaValidator<'a>(MetaValidatorInner<'a>);

        // Native builds can hand out references to cached validators or own dynamic ones,
        // while wasm targets need owned instances because the validator type does not implement `Sync` there.
        enum MetaValidatorInner<'a> {
            #[cfg(not(target_family = "wasm"))]
            Borrowed(&'a Validator),
            Owned(Box<Validator>, PhantomData<&'a Validator>),
        }

        #[cfg_attr(target_family = "wasm", allow(clippy::elidable_lifetime_names))]
        impl<'a> MetaValidator<'a> {
            #[cfg(not(target_family = "wasm"))]
            pub(crate) fn borrowed(validator: &'a Validator) -> Self {
                Self(MetaValidatorInner::Borrowed(validator))
            }

            pub(crate) fn owned(validator: Validator) -> Self {
                Self(MetaValidatorInner::Owned(Box::new(validator), PhantomData))
            }
        }

        impl AsRef<Validator> for MetaValidator<'_> {
            fn as_ref(&self) -> &Validator {
                match &self.0 {
                    #[cfg(not(target_family = "wasm"))]
                    MetaValidatorInner::Borrowed(validator) => validator,
                    MetaValidatorInner::Owned(validator, _) => validator,
                }
            }
        }

        impl Deref for MetaValidator<'_> {
            type Target = Validator;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }
    }

    pub(crate) mod validators {
        use crate::Validator;
        #[cfg(not(target_family = "wasm"))]
        use std::sync::LazyLock;

        fn build_validator(schema: &serde_json::Value) -> Validator {
            crate::options()
                .without_schema_validation()
                .build(schema)
                .expect("Meta-schema should be valid")
        }

        #[cfg(not(target_family = "wasm"))]
        pub(crate) static DRAFT4_META_VALIDATOR: LazyLock<Validator> =
            LazyLock::new(|| build_validator(&referencing::meta::DRAFT4));
        #[cfg(target_family = "wasm")]
        pub(crate) fn draft4_meta_validator() -> Validator {
            build_validator(&referencing::meta::DRAFT4)
        }

        #[cfg(not(target_family = "wasm"))]
        pub(crate) static DRAFT6_META_VALIDATOR: LazyLock<Validator> =
            LazyLock::new(|| build_validator(&referencing::meta::DRAFT6));
        #[cfg(target_family = "wasm")]
        pub(crate) fn draft6_meta_validator() -> Validator {
            build_validator(&referencing::meta::DRAFT6)
        }

        #[cfg(not(target_family = "wasm"))]
        pub(crate) static DRAFT7_META_VALIDATOR: LazyLock<Validator> =
            LazyLock::new(|| build_validator(&referencing::meta::DRAFT7));
        #[cfg(target_family = "wasm")]
        pub(crate) fn draft7_meta_validator() -> Validator {
            build_validator(&referencing::meta::DRAFT7)
        }

        #[cfg(not(target_family = "wasm"))]
        pub(crate) static DRAFT201909_META_VALIDATOR: LazyLock<Validator> =
            LazyLock::new(|| build_validator(&referencing::meta::DRAFT201909));
        #[cfg(target_family = "wasm")]
        pub(crate) fn draft201909_meta_validator() -> Validator {
            build_validator(&referencing::meta::DRAFT201909)
        }

        #[cfg(not(target_family = "wasm"))]
        pub(crate) static DRAFT202012_META_VALIDATOR: LazyLock<Validator> =
            LazyLock::new(|| build_validator(&referencing::meta::DRAFT202012));
        #[cfg(target_family = "wasm")]
        pub(crate) fn draft202012_meta_validator() -> Validator {
            build_validator(&referencing::meta::DRAFT202012)
        }
    }

    pub(crate) fn validator_for_draft(draft: Draft) -> MetaValidator<'static> {
        #[cfg(not(target_family = "wasm"))]
        {
            match draft {
                Draft::Draft4 => MetaValidator::borrowed(&validators::DRAFT4_META_VALIDATOR),
                Draft::Draft6 => MetaValidator::borrowed(&validators::DRAFT6_META_VALIDATOR),
                Draft::Draft7 => MetaValidator::borrowed(&validators::DRAFT7_META_VALIDATOR),
                Draft::Draft201909 => {
                    MetaValidator::borrowed(&validators::DRAFT201909_META_VALIDATOR)
                }
                // Draft202012, Unknown, or any future draft variants
                _ => MetaValidator::borrowed(&validators::DRAFT202012_META_VALIDATOR),
            }
        }
        #[cfg(target_family = "wasm")]
        {
            let validator = match draft {
                Draft::Draft4 => validators::draft4_meta_validator(),
                Draft::Draft6 => validators::draft6_meta_validator(),
                Draft::Draft7 => validators::draft7_meta_validator(),
                Draft::Draft201909 => validators::draft201909_meta_validator(),
                // Draft202012, Unknown, or any future draft variants
                _ => validators::draft202012_meta_validator(),
            };
            MetaValidator::owned(validator)
        }
    }

    /// Validate a JSON Schema document against its meta-schema and get a `true` if the schema is valid
    /// and `false` otherwise. Draft version is detected automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "string",
    ///     "maxLength": 5
    /// });
    /// assert!(jsonschema::meta::is_valid(&schema));
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if the meta-schema can't be detected.
    ///
    /// # Note
    ///
    /// This helper only works with the built-in JSON Schema drafts. For schemas that declare a
    /// custom `$schema`, construct a registry that contains your meta-schema and use
    /// [`meta::options().with_registry(...)`](crate::meta::options) to validate it.
    #[must_use]
    pub fn is_valid(schema: &Value) -> bool {
        match try_meta_validator_for(schema, None) {
            Ok(validator) => validator.as_ref().is_valid(schema),
            Err(error) => panic!("Failed to resolve meta-schema: {error}"),
        }
    }
    /// Validate a JSON Schema document against its meta-schema and return the first error if any.
    /// Draft version is detected automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "string",
    ///     "maxLength": 5
    /// });
    /// assert!(jsonschema::meta::validate(&schema).is_ok());
    ///
    /// // Invalid schema
    /// let invalid_schema = json!({
    ///     "type": "invalid_type"
    /// });
    /// assert!(jsonschema::meta::validate(&invalid_schema).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] describing why the schema violates the detected meta-schema.
    ///
    /// # Panics
    ///
    /// This function panics if the meta-schema can't be detected.
    ///
    /// # Note
    ///
    /// Like [`is_valid`], this helper only handles the bundled JSON
    /// Schema drafts. For custom meta-schemas, use [`meta::options().with_registry(...)`](crate::meta::options)
    /// so the registry can supply the meta-schema.
    pub fn validate(schema: &Value) -> Result<(), ValidationError<'_>> {
        let validator = try_meta_validator_for(schema, None)?;
        validator.as_ref().validate(schema)
    }

    /// Build a validator for a JSON Schema's meta-schema.
    /// Draft version is detected automatically.
    ///
    /// Returns a [`MetaValidator`] that can be used to validate the schema or access
    /// structured validation output via the evaluate API.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "string",
    ///     "maxLength": 5
    /// });
    ///
    /// let validator = jsonschema::meta::validator_for(&schema)
    ///     .expect("Valid meta-schema");
    ///
    /// // Use evaluate API for structured output
    /// let evaluation = validator.evaluate(&schema);
    /// assert!(evaluation.flag().valid);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ValidationError`] if the meta-schema cannot be resolved or built.
    ///
    /// # Panics
    ///
    /// This function panics if the meta-schema can't be detected.
    ///
    /// # Note
    ///
    /// This helper only handles the bundled JSON Schema drafts. For custom meta-schemas,
    /// use [`meta::options().with_registry(...)`](crate::meta::options).
    pub fn validator_for(
        schema: &Value,
    ) -> Result<MetaValidator<'static>, ValidationError<'static>> {
        try_meta_validator_for(schema, None)
    }

    fn try_meta_validator_for<'a>(
        schema: &Value,
        registry: Option<&Registry>,
    ) -> Result<MetaValidator<'a>, ValidationError<'static>> {
        let draft = Draft::default().detect(schema);

        // For custom meta-schemas (Draft::Unknown), attempt to resolve the meta-schema
        if draft == Draft::Unknown {
            if let Some(meta_schema_uri) = schema
                .as_object()
                .and_then(|obj| obj.get("$schema"))
                .and_then(|s| s.as_str())
            {
                // Try registry first if available
                if let Some(registry) = registry {
                    let (custom_meta_schema, resolved_draft) =
                        resolve_meta_schema_with_registry(meta_schema_uri, registry)?;
                    let validator = crate::options()
                        .with_draft(resolved_draft)
                        .with_registry(registry.clone())
                        .without_schema_validation()
                        .build(&custom_meta_schema)?;
                    return Ok(MetaValidator::owned(validator));
                }

                // Use default retriever
                let (custom_meta_schema, resolved_draft) =
                    resolve_meta_schema_chain(meta_schema_uri)?;
                let validator = crate::options()
                    .with_draft(resolved_draft)
                    .without_schema_validation()
                    .build(&custom_meta_schema)?;
                return Ok(MetaValidator::owned(validator));
            }
        }

        Ok(validator_for_draft(draft))
    }

    fn resolve_meta_schema_with_registry(
        uri: &str,
        registry: &Registry,
    ) -> Result<(Value, Draft), ValidationError<'static>> {
        let resolver = registry.try_resolver(uri)?;
        let first_resolved = resolver.lookup("")?;
        let first_meta_schema = first_resolved.contents().clone();

        let draft = walk_meta_schema_chain(uri, |current_uri| {
            let resolver = registry.try_resolver(current_uri)?;
            let resolved = resolver.lookup("")?;
            Ok(resolved.contents().clone())
        })?;

        Ok((first_meta_schema, draft))
    }

    fn resolve_meta_schema_chain(uri: &str) -> Result<(Value, Draft), ValidationError<'static>> {
        let retriever = crate::retriever::DefaultRetriever;
        let first_meta_uri = referencing::uri::from_str(uri)?;
        let first_meta_schema = retriever
            .retrieve(&first_meta_uri)
            .map_err(|e| referencing::Error::unretrievable(uri, e))?;

        let draft = walk_meta_schema_chain(uri, |current_uri| {
            let meta_uri = referencing::uri::from_str(current_uri)?;
            Ok(retriever
                .retrieve(&meta_uri)
                .map_err(|e| referencing::Error::unretrievable(current_uri, e))?)
        })?;

        Ok((first_meta_schema, draft))
    }

    pub(crate) fn walk_meta_schema_chain(
        start_uri: &str,
        mut fetch: impl FnMut(&str) -> Result<Value, ValidationError<'static>>,
    ) -> Result<Draft, ValidationError<'static>> {
        let mut visited = AHashSet::new();
        let mut current_uri = start_uri.to_string();

        loop {
            if !visited.insert(current_uri.clone()) {
                return Err(referencing::Error::circular_metaschema(current_uri).into());
            }

            let meta_schema = fetch(&current_uri)?;
            let draft = Draft::default().detect(&meta_schema);

            if draft != Draft::Unknown {
                return Ok(draft);
            }

            current_uri = meta_schema
                .get("$schema")
                .and_then(|s| s.as_str())
                .expect("`$schema` must exist when draft is Unknown")
                .to_string();
        }
    }
}

/// Functionality specific to JSON Schema Draft 4.
///
/// [![Draft 4](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft4.json)](https://bowtie.report/#/implementations/rust-jsonschema)
///
/// This module provides functions for creating validators and performing validation
/// according to the JSON Schema Draft 4 specification.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"type": "number", "multipleOf": 2});
/// let instance = json!(4);
///
/// assert!(jsonschema::draft4::is_valid(&schema, &instance));
/// ```
pub mod draft4 {
    use super::{Draft, ValidationError, ValidationOptions, Validator, Value};

    /// Create a new JSON Schema validator using Draft 4 specifications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let instance = json!(42);
    ///
    /// let validator = jsonschema::draft4::new(&schema)?;
    /// assert!(validator.is_valid(&instance));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not a valid Draft 4 document or if referenced resources
    /// cannot be resolved.
    pub fn new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        options().build(schema)
    }
    /// Validate an instance against a schema using Draft 4 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft4::is_valid(&schema, &valid));
    /// assert!(!jsonschema::draft4::is_valid(&schema, &invalid));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 4 validator.
    #[must_use]
    pub fn is_valid(schema: &Value, instance: &Value) -> bool {
        new(schema).expect("Invalid schema").is_valid(instance)
    }
    /// Validate an instance against a schema using Draft 4 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft4::validate(&schema, &valid).is_ok());
    /// assert!(jsonschema::draft4::validate(&schema, &invalid).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] when `instance` violates the schema.
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 4 validator.
    pub fn validate<'i>(schema: &Value, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        new(schema).expect("Invalid schema").validate(instance)
    }
    /// Creates a [`ValidationOptions`] builder pre-configured for JSON Schema Draft 4.
    ///
    /// This function provides a shorthand for `jsonschema::options().with_draft(Draft::Draft4)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "string", "format": "ends-with-42"});
    /// let validator = jsonschema::draft4::options()
    ///     .with_format("ends-with-42", |s| s.ends_with("42"))
    ///     .should_validate_formats(true)
    ///     .build(&schema)?;
    ///
    /// assert!(validator.is_valid(&json!("Hello 42")));
    /// assert!(!validator.is_valid(&json!("No!")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`ValidationOptions`] for all available configuration options.
    #[must_use]
    pub fn options() -> ValidationOptions {
        crate::options().with_draft(Draft::Draft4)
    }

    /// Functionality for validating JSON Schema Draft 4 documents.
    pub mod meta {
        use crate::{meta::MetaValidator, ValidationError};
        use serde_json::Value;

        /// Returns a handle to the Draft 4 meta-schema validator. Native targets borrow cached
        /// statics while `wasm32` builds an owned validator.
        #[must_use]
        pub fn validator() -> MetaValidator<'static> {
            crate::meta::validator_for_draft(super::Draft::Draft4)
        }

        /// Validate a JSON Schema document against Draft 4 meta-schema and get a `true` if the schema is valid
        /// and `false` otherwise.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft4::meta::is_valid(&schema));
        /// ```
        #[must_use]
        #[inline]
        pub fn is_valid(schema: &Value) -> bool {
            validator().as_ref().is_valid(schema)
        }

        /// Validate a JSON Schema document against Draft 4 meta-schema and return the first error if any.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft4::meta::validate(&schema).is_ok());
        ///
        /// // Invalid schema
        /// let invalid_schema = json!({
        ///     "type": "invalid_type"
        /// });
        /// assert!(jsonschema::draft4::meta::validate(&invalid_schema).is_err());
        /// ```
        ///
        /// # Errors
        ///
        /// Returns the first [`ValidationError`] describing why the schema violates the Draft 4 meta-schema.
        #[inline]
        pub fn validate(schema: &Value) -> Result<(), ValidationError<'_>> {
            validator().as_ref().validate(schema)
        }
    }
}

/// Functionality specific to JSON Schema Draft 6.
///
/// [![Draft 6](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft6.json)](https://bowtie.report/#/implementations/rust-jsonschema)
///
/// This module provides functions for creating validators and performing validation
/// according to the JSON Schema Draft 6 specification.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"type": "string", "format": "uri"});
/// let instance = json!("https://www.example.com");
///
/// assert!(jsonschema::draft6::is_valid(&schema, &instance));
/// ```
pub mod draft6 {
    use super::{Draft, ValidationError, ValidationOptions, Validator, Value};

    /// Create a new JSON Schema validator using Draft 6 specifications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let instance = json!(42);
    ///
    /// let validator = jsonschema::draft6::new(&schema)?;
    /// assert!(validator.is_valid(&instance));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not a valid Draft 6 document or if referenced resources
    /// cannot be resolved.
    pub fn new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        options().build(schema)
    }
    /// Validate an instance against a schema using Draft 6 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft6::is_valid(&schema, &valid));
    /// assert!(!jsonschema::draft6::is_valid(&schema, &invalid));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 6 validator.
    #[must_use]
    pub fn is_valid(schema: &Value, instance: &Value) -> bool {
        new(schema).expect("Invalid schema").is_valid(instance)
    }
    /// Validate an instance against a schema using Draft 6 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft6::validate(&schema, &valid).is_ok());
    /// assert!(jsonschema::draft6::validate(&schema, &invalid).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] when `instance` violates the schema.
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 6 validator.
    pub fn validate<'i>(schema: &Value, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        new(schema).expect("Invalid schema").validate(instance)
    }
    /// Creates a [`ValidationOptions`] builder pre-configured for JSON Schema Draft 6.
    ///
    /// This function provides a shorthand for `jsonschema::options().with_draft(Draft::Draft6)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "string", "format": "ends-with-42"});
    /// let validator = jsonschema::draft6::options()
    ///     .with_format("ends-with-42", |s| s.ends_with("42"))
    ///     .should_validate_formats(true)
    ///     .build(&schema)?;
    ///
    /// assert!(validator.is_valid(&json!("Hello 42")));
    /// assert!(!validator.is_valid(&json!("No!")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`ValidationOptions`] for all available configuration options.
    #[must_use]
    pub fn options() -> ValidationOptions {
        crate::options().with_draft(Draft::Draft6)
    }

    /// Functionality for validating JSON Schema Draft 6 documents.
    pub mod meta {
        use crate::{meta::MetaValidator, ValidationError};
        use serde_json::Value;

        /// Returns a handle to the Draft 6 meta-schema validator. Native targets borrow cached
        /// statics while `wasm32` builds an owned validator.
        #[must_use]
        pub fn validator() -> MetaValidator<'static> {
            crate::meta::validator_for_draft(super::Draft::Draft6)
        }

        /// Validate a JSON Schema document against Draft 6 meta-schema and get a `true` if the schema is valid
        /// and `false` otherwise.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft6::meta::is_valid(&schema));
        /// ```
        #[must_use]
        #[inline]
        pub fn is_valid(schema: &Value) -> bool {
            validator().as_ref().is_valid(schema)
        }

        /// Validate a JSON Schema document against Draft 6 meta-schema and return the first error if any.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft6::meta::validate(&schema).is_ok());
        ///
        /// // Invalid schema
        /// let invalid_schema = json!({
        ///     "type": "invalid_type"
        /// });
        /// assert!(jsonschema::draft6::meta::validate(&invalid_schema).is_err());
        /// ```
        ///
        /// # Errors
        ///
        /// Returns the first [`ValidationError`] describing why the schema violates the Draft 6 meta-schema.
        #[inline]
        pub fn validate(schema: &Value) -> Result<(), ValidationError<'_>> {
            validator().as_ref().validate(schema)
        }
    }
}

/// Functionality specific to JSON Schema Draft 7.
///
/// [![Draft 7](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft7.json)](https://bowtie.report/#/implementations/rust-jsonschema)
///
/// This module provides functions for creating validators and performing validation
/// according to the JSON Schema Draft 7 specification.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"type": "string", "pattern": "^[a-zA-Z0-9]+$"});
/// let instance = json!("abc123");
///
/// assert!(jsonschema::draft7::is_valid(&schema, &instance));
/// ```
pub mod draft7 {
    use super::{Draft, ValidationError, ValidationOptions, Validator, Value};

    /// Create a new JSON Schema validator using Draft 7 specifications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let instance = json!(42);
    ///
    /// let validator = jsonschema::draft7::new(&schema)?;
    /// assert!(validator.is_valid(&instance));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not a valid Draft 7 document or if referenced resources
    /// cannot be resolved.
    pub fn new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        options().build(schema)
    }
    /// Validate an instance against a schema using Draft 7 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft7::is_valid(&schema, &valid));
    /// assert!(!jsonschema::draft7::is_valid(&schema, &invalid));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 7 validator.
    #[must_use]
    pub fn is_valid(schema: &Value, instance: &Value) -> bool {
        new(schema).expect("Invalid schema").is_valid(instance)
    }
    /// Validate an instance against a schema using Draft 7 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft7::validate(&schema, &valid).is_ok());
    /// assert!(jsonschema::draft7::validate(&schema, &invalid).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] when `instance` violates the schema.
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 7 validator.
    pub fn validate<'i>(schema: &Value, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        new(schema).expect("Invalid schema").validate(instance)
    }
    /// Creates a [`ValidationOptions`] builder pre-configured for JSON Schema Draft 7.
    ///
    /// This function provides a shorthand for `jsonschema::options().with_draft(Draft::Draft7)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "string", "format": "ends-with-42"});
    /// let validator = jsonschema::draft7::options()
    ///     .with_format("ends-with-42", |s| s.ends_with("42"))
    ///     .should_validate_formats(true)
    ///     .build(&schema)?;
    ///
    /// assert!(validator.is_valid(&json!("Hello 42")));
    /// assert!(!validator.is_valid(&json!("No!")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`ValidationOptions`] for all available configuration options.
    #[must_use]
    pub fn options() -> ValidationOptions {
        crate::options().with_draft(Draft::Draft7)
    }

    /// Functionality for validating JSON Schema Draft 7 documents.
    pub mod meta {
        use crate::{meta::MetaValidator, ValidationError};
        use serde_json::Value;

        /// Returns a handle to the Draft 7 meta-schema validator. Native targets borrow cached
        /// statics while `wasm32` builds an owned validator.
        #[must_use]
        pub fn validator() -> MetaValidator<'static> {
            crate::meta::validator_for_draft(super::Draft::Draft7)
        }

        /// Validate a JSON Schema document against Draft 7 meta-schema and get a `true` if the schema is valid
        /// and `false` otherwise.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft7::meta::is_valid(&schema));
        /// ```
        #[must_use]
        #[inline]
        pub fn is_valid(schema: &Value) -> bool {
            validator().as_ref().is_valid(schema)
        }

        /// Validate a JSON Schema document against Draft 7 meta-schema and return the first error if any.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft7::meta::validate(&schema).is_ok());
        ///
        /// // Invalid schema
        /// let invalid_schema = json!({
        ///     "type": "invalid_type"
        /// });
        /// assert!(jsonschema::draft7::meta::validate(&invalid_schema).is_err());
        /// ```
        ///
        /// # Errors
        ///
        /// Returns the first [`ValidationError`] describing why the schema violates the Draft 7 meta-schema.
        #[inline]
        pub fn validate(schema: &Value) -> Result<(), ValidationError<'_>> {
            validator().as_ref().validate(schema)
        }
    }
}

/// Functionality specific to JSON Schema Draft 2019-09.
///
/// [![Draft 2019-09](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft2019-09.json)](https://bowtie.report/#/implementations/rust-jsonschema)
///
/// This module provides functions for creating validators and performing validation
/// according to the JSON Schema Draft 2019-09 specification.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
///
/// let schema = json!({"type": "array", "minItems": 2, "uniqueItems": true});
/// let instance = json!([1, 2]);
///
/// assert!(jsonschema::draft201909::is_valid(&schema, &instance));
/// ```
pub mod draft201909 {
    use super::{Draft, ValidationError, ValidationOptions, Validator, Value};

    /// Create a new JSON Schema validator using Draft 2019-09 specifications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let instance = json!(42);
    ///
    /// let validator = jsonschema::draft201909::new(&schema)?;
    /// assert!(validator.is_valid(&instance));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not a valid Draft 2019-09 document or if referenced resources
    /// cannot be resolved.
    pub fn new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        options().build(schema)
    }
    /// Validate an instance against a schema using Draft 2019-09 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft201909::is_valid(&schema, &valid));
    /// assert!(!jsonschema::draft201909::is_valid(&schema, &invalid));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 2019-09 validator.
    #[must_use]
    pub fn is_valid(schema: &Value, instance: &Value) -> bool {
        new(schema).expect("Invalid schema").is_valid(instance)
    }
    /// Validate an instance against a schema using Draft 2019-09 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft201909::validate(&schema, &valid).is_ok());
    /// assert!(jsonschema::draft201909::validate(&schema, &invalid).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] when `instance` violates the schema.
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 2019-09 validator.
    pub fn validate<'i>(schema: &Value, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        new(schema).expect("Invalid schema").validate(instance)
    }
    /// Creates a [`ValidationOptions`] builder pre-configured for JSON Schema Draft 2019-09.
    ///
    /// This function provides a shorthand for `jsonschema::options().with_draft(Draft::Draft201909)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "string", "format": "ends-with-42"});
    /// let validator = jsonschema::draft201909::options()
    ///     .with_format("ends-with-42", |s| s.ends_with("42"))
    ///     .should_validate_formats(true)
    ///     .build(&schema)?;
    ///
    /// assert!(validator.is_valid(&json!("Hello 42")));
    /// assert!(!validator.is_valid(&json!("No!")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`ValidationOptions`] for all available configuration options.
    #[must_use]
    pub fn options() -> ValidationOptions {
        crate::options().with_draft(Draft::Draft201909)
    }

    /// Functionality for validating JSON Schema Draft 2019-09 documents.
    pub mod meta {
        use crate::{meta::MetaValidator, ValidationError};
        use serde_json::Value;

        /// Returns a handle to the Draft 2019-09 meta-schema validator. Native targets borrow cached
        /// statics while `wasm32` builds an owned validator.
        #[must_use]
        pub fn validator() -> MetaValidator<'static> {
            crate::meta::validator_for_draft(super::Draft::Draft201909)
        }
        /// Validate a JSON Schema document against Draft 2019-09 meta-schema and get a `true` if the schema is valid
        /// and `false` otherwise.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft201909::meta::is_valid(&schema));
        /// ```
        #[must_use]
        #[inline]
        pub fn is_valid(schema: &Value) -> bool {
            validator().as_ref().is_valid(schema)
        }

        /// Validate a JSON Schema document against Draft 2019-09 meta-schema and return the first error if any.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft201909::meta::validate(&schema).is_ok());
        ///
        /// // Invalid schema
        /// let invalid_schema = json!({
        ///     "type": "invalid_type"
        /// });
        /// assert!(jsonschema::draft201909::meta::validate(&invalid_schema).is_err());
        /// ```
        ///
        /// # Errors
        ///
        /// Returns the first [`ValidationError`] describing why the schema violates the Draft 2019-09 meta-schema.
        #[inline]
        pub fn validate(schema: &Value) -> Result<(), ValidationError<'_>> {
            validator().as_ref().validate(schema)
        }
    }
}

/// Functionality specific to JSON Schema Draft 2020-12.
///
/// [![Draft 2020-12](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft2020-12.json)](https://bowtie.report/#/implementations/rust-jsonschema)
///
/// This module provides functions for creating validators and performing validation
/// according to the JSON Schema Draft 2020-12 specification.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({"type": "object", "properties": {"name": {"type": "string"}}, "required": ["name"]});
/// let instance = json!({"name": "John Doe"});
///
/// assert!(jsonschema::draft202012::is_valid(&schema, &instance));
/// # Ok(())
/// # }
/// ```
pub mod draft202012 {
    use super::{Draft, ValidationError, ValidationOptions, Validator, Value};

    /// Create a new JSON Schema validator using Draft 2020-12 specifications.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let instance = json!(42);
    ///
    /// let validator = jsonschema::draft202012::new(&schema)?;
    /// assert!(validator.is_valid(&instance));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the schema is not a valid Draft 2020-12 document or if referenced resources
    /// cannot be resolved.
    pub fn new(schema: &Value) -> Result<Validator, ValidationError<'static>> {
        options().build(schema)
    }
    /// Validate an instance against a schema using Draft 2020-12 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft202012::is_valid(&schema, &valid));
    /// assert!(!jsonschema::draft202012::is_valid(&schema, &invalid));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 2020-12 validator.
    #[must_use]
    pub fn is_valid(schema: &Value, instance: &Value) -> bool {
        new(schema).expect("Invalid schema").is_valid(instance)
    }
    /// Validate an instance against a schema using Draft 2020-12 specifications without creating a validator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_json::json;
    ///
    /// let schema = json!({"minimum": 5});
    /// let valid = json!(42);
    /// let invalid = json!(3);
    ///
    /// assert!(jsonschema::draft202012::validate(&schema, &valid).is_ok());
    /// assert!(jsonschema::draft202012::validate(&schema, &invalid).is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the first [`ValidationError`] when `instance` violates the schema.
    ///
    /// # Panics
    ///
    /// Panics if `schema` cannot be compiled into a Draft 2020-12 validator.
    pub fn validate<'i>(schema: &Value, instance: &'i Value) -> Result<(), ValidationError<'i>> {
        new(schema).expect("Invalid schema").validate(instance)
    }
    /// Creates a [`ValidationOptions`] builder pre-configured for JSON Schema Draft 2020-12.
    ///
    /// This function provides a shorthand for `jsonschema::options().with_draft(Draft::Draft202012)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "string", "format": "ends-with-42"});
    /// let validator = jsonschema::draft202012::options()
    ///     .with_format("ends-with-42", |s| s.ends_with("42"))
    ///     .should_validate_formats(true)
    ///     .build(&schema)?;
    ///
    /// assert!(validator.is_valid(&json!("Hello 42")));
    /// assert!(!validator.is_valid(&json!("No!")));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [`ValidationOptions`] for all available configuration options.
    #[must_use]
    pub fn options() -> ValidationOptions {
        crate::options().with_draft(Draft::Draft202012)
    }

    /// Functionality for validating JSON Schema Draft 2020-12 documents.
    pub mod meta {
        use crate::{meta::MetaValidator, ValidationError};
        use serde_json::Value;

        /// Returns a handle to the Draft 2020-12 meta-schema validator. Native targets borrow
        /// cached statics while `wasm32` builds an owned validator.
        #[must_use]
        pub fn validator() -> MetaValidator<'static> {
            crate::meta::validator_for_draft(super::Draft::Draft202012)
        }

        /// Validate a JSON Schema document against Draft 2020-12 meta-schema and get a `true` if the schema is valid
        /// and `false` otherwise.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft202012::meta::is_valid(&schema));
        /// ```
        #[must_use]
        #[inline]
        pub fn is_valid(schema: &Value) -> bool {
            validator().as_ref().is_valid(schema)
        }

        /// Validate a JSON Schema document against Draft 2020-12 meta-schema and return the first error if any.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use serde_json::json;
        ///
        /// let schema = json!({
        ///     "type": "string",
        ///     "maxLength": 5
        /// });
        /// assert!(jsonschema::draft202012::meta::validate(&schema).is_ok());
        ///
        /// // Invalid schema
        /// let invalid_schema = json!({
        ///     "type": "invalid_type"
        /// });
        /// assert!(jsonschema::draft202012::meta::validate(&invalid_schema).is_err());
        /// ```
        ///
        /// # Errors
        ///
        /// Returns the first [`ValidationError`] describing why the schema violates the Draft 2020-12 meta-schema.
        #[inline]
        pub fn validate(schema: &Value) -> Result<(), ValidationError<'_>> {
            validator().as_ref().validate(schema)
        }
    }
}

#[cfg(test)]
pub(crate) mod tests_util {
    use super::Validator;
    use crate::ValidationError;
    use serde_json::Value;

    #[track_caller]
    pub(crate) fn is_not_valid_with(validator: &Validator, instance: &Value) {
        assert!(
            !validator.is_valid(instance),
            "{instance} should not be valid (via is_valid)",
        );
        assert!(
            validator.validate(instance).is_err(),
            "{instance} should not be valid (via validate)",
        );
        assert!(
            validator.iter_errors(instance).next().is_some(),
            "{instance} should not be valid (via validate)",
        );
        let evaluation = validator.evaluate(instance);
        assert!(
            !evaluation.flag().valid,
            "{instance} should not be valid (via evaluate)",
        );
    }

    #[track_caller]
    pub(crate) fn is_not_valid(schema: &Value, instance: &Value) {
        let validator = crate::options()
            .should_validate_formats(true)
            .build(schema)
            .expect("Invalid schema");
        is_not_valid_with(&validator, instance);
    }

    pub(crate) fn expect_errors(schema: &Value, instance: &Value, errors: &[&str]) {
        assert_eq!(
            crate::validator_for(schema)
                .expect("Should be a valid schema")
                .iter_errors(instance)
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            errors
        );
    }

    #[track_caller]
    pub(crate) fn is_valid_with(validator: &Validator, instance: &Value) {
        if let Some(first) = validator.iter_errors(instance).next() {
            panic!(
                "{} should be valid (via validate). Error: {} at {}",
                instance,
                first,
                first.instance_path()
            );
        }
        assert!(
            validator.is_valid(instance),
            "{instance} should be valid (via is_valid)",
        );
        assert!(
            validator.validate(instance).is_ok(),
            "{instance} should be valid (via is_valid)",
        );
        let evaluation = validator.evaluate(instance);
        assert!(
            evaluation.flag().valid,
            "{instance} should be valid (via evaluate)",
        );
    }

    #[track_caller]
    pub(crate) fn is_valid(schema: &Value, instance: &Value) {
        let validator = crate::options()
            .should_validate_formats(true)
            .build(schema)
            .expect("Invalid schema");
        is_valid_with(&validator, instance);
    }

    #[track_caller]
    pub(crate) fn validate(schema: &Value, instance: &Value) -> ValidationError<'static> {
        let validator = crate::options()
            .should_validate_formats(true)
            .build(schema)
            .expect("Invalid schema");
        let err = validator
            .validate(instance)
            .expect_err("Should be an error")
            .to_owned();
        err
    }

    #[track_caller]
    pub(crate) fn assert_schema_location(schema: &Value, instance: &Value, expected: &str) {
        let error = validate(schema, instance);
        assert_eq!(error.schema_path().as_str(), expected);
    }

    #[track_caller]
    pub(crate) fn assert_locations(schema: &Value, instance: &Value, expected: &[&str]) {
        let validator = crate::validator_for(schema).unwrap();
        let errors = validator.iter_errors(instance);
        for (error, location) in errors.into_iter().zip(expected) {
            assert_eq!(error.schema_path().as_str(), *location);
        }
    }

    #[track_caller]
    pub(crate) fn assert_keyword_location(
        validator: &Validator,
        instance: &Value,
        instance_pointer: &str,
        keyword_pointer: &str,
    ) {
        fn pointer_from_schema_location(location: &str) -> &str {
            location
                .split_once('#')
                .map_or(location, |(_, fragment)| fragment)
        }

        let evaluation = validator.evaluate(instance);
        let serialized =
            serde_json::to_value(evaluation.list()).expect("List output should be serializable");
        let details = serialized
            .get("details")
            .and_then(|value| value.as_array())
            .expect("List output must contain details");
        let mut available = Vec::new();
        for entry in details {
            let Some(instance_location) = entry
                .get("instanceLocation")
                .and_then(|value| value.as_str())
            else {
                continue;
            };
            if instance_location != instance_pointer {
                continue;
            }
            let schema_location = entry
                .get("schemaLocation")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let pointer = pointer_from_schema_location(schema_location);
            if pointer == keyword_pointer {
                return;
            }
            available.push(pointer.to_string());
        }

        panic!(
            "No annotation for instance pointer `{instance_pointer}` with keyword location `{keyword_pointer}`. Available keyword locations for pointer: {available:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{validator_for, ValidationError};
    use referencing::{Registry, Resource};

    use super::Draft;
    use serde_json::json;
    use test_case::test_case;

    #[test_case(crate::is_valid ; "autodetect")]
    #[test_case(crate::draft4::is_valid ; "draft4")]
    #[test_case(crate::draft6::is_valid ; "draft6")]
    #[test_case(crate::draft7::is_valid ; "draft7")]
    #[test_case(crate::draft201909::is_valid ; "draft201909")]
    #[test_case(crate::draft202012::is_valid ; "draft202012")]
    fn test_is_valid(is_valid_fn: fn(&serde_json::Value, &serde_json::Value) -> bool) {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["name"]
        });

        let valid_instance = json!({
            "name": "John Doe",
            "age": 30
        });

        let invalid_instance = json!({
            "age": -5
        });

        assert!(is_valid_fn(&schema, &valid_instance));
        assert!(!is_valid_fn(&schema, &invalid_instance));
    }

    #[test_case(crate::validate ; "autodetect")]
    #[test_case(crate::draft4::validate ; "draft4")]
    #[test_case(crate::draft6::validate ; "draft6")]
    #[test_case(crate::draft7::validate ; "draft7")]
    #[test_case(crate::draft201909::validate ; "draft201909")]
    #[test_case(crate::draft202012::validate ; "draft202012")]
    fn test_validate(
        validate_fn: for<'i> fn(
            &serde_json::Value,
            &'i serde_json::Value,
        ) -> Result<(), ValidationError<'i>>,
    ) {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["name"]
        });

        let valid_instance = json!({
            "name": "John Doe",
            "age": 30
        });

        let invalid_instance = json!({
            "age": -5
        });

        assert!(validate_fn(&schema, &valid_instance).is_ok());
        assert!(validate_fn(&schema, &invalid_instance).is_err());
    }

    #[test]
    fn test_evaluate() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["name"]
        });

        let valid_instance = json!({
            "name": "John Doe",
            "age": 30
        });

        let invalid_instance = json!({
            "age": -5
        });

        let valid_eval = crate::evaluate(&schema, &valid_instance);
        assert!(valid_eval.flag().valid);

        let invalid_eval = crate::evaluate(&schema, &invalid_instance);
        assert!(!invalid_eval.flag().valid);
        let errors: Vec<_> = invalid_eval.iter_errors().collect();
        assert!(!errors.is_empty());
    }

    #[test_case(crate::meta::validate, crate::meta::is_valid ; "autodetect")]
    #[test_case(crate::draft4::meta::validate, crate::draft4::meta::is_valid ; "draft4")]
    #[test_case(crate::draft6::meta::validate, crate::draft6::meta::is_valid ; "draft6")]
    #[test_case(crate::draft7::meta::validate, crate::draft7::meta::is_valid ; "draft7")]
    #[test_case(crate::draft201909::meta::validate, crate::draft201909::meta::is_valid ; "draft201909")]
    #[test_case(crate::draft202012::meta::validate, crate::draft202012::meta::is_valid ; "draft202012")]
    fn test_meta_validation(
        validate_fn: fn(&serde_json::Value) -> Result<(), ValidationError>,
        is_valid_fn: fn(&serde_json::Value) -> bool,
    ) {
        let valid = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["name"]
        });

        let invalid = json!({
            "type": "invalid_type",
            "minimum": "not_a_number",
            "required": true  // should be an array
        });

        assert!(validate_fn(&valid).is_ok());
        assert!(validate_fn(&invalid).is_err());
        assert!(is_valid_fn(&valid));
        assert!(!is_valid_fn(&invalid));
    }

    #[test]
    fn test_exclusive_minimum_across_drafts() {
        // In Draft 4, exclusiveMinimum is a boolean modifier for minimum
        let draft4_schema = json!({
            "$schema": "http://json-schema.org/draft-04/schema#",
            "minimum": 5,
            "exclusiveMinimum": true
        });
        assert!(crate::meta::is_valid(&draft4_schema));
        assert!(crate::meta::validate(&draft4_schema).is_ok());

        // This is invalid in Draft 4 (exclusiveMinimum must be boolean)
        let invalid_draft4 = json!({
            "$schema": "http://json-schema.org/draft-04/schema#",
            "exclusiveMinimum": 5
        });
        assert!(!crate::meta::is_valid(&invalid_draft4));
        assert!(crate::meta::validate(&invalid_draft4).is_err());

        // In Draft 6 and later, exclusiveMinimum is a numeric value
        let drafts = [
            "http://json-schema.org/draft-06/schema#",
            "http://json-schema.org/draft-07/schema#",
            "https://json-schema.org/draft/2019-09/schema",
            "https://json-schema.org/draft/2020-12/schema",
        ];

        for uri in drafts {
            // Valid in Draft 6+ (numeric exclusiveMinimum)
            let valid_schema = json!({
                "$schema": uri,
                "exclusiveMinimum": 5
            });
            assert!(
                crate::meta::is_valid(&valid_schema),
                "Schema should be valid for {uri}"
            );
            assert!(
                crate::meta::validate(&valid_schema).is_ok(),
                "Schema validation should succeed for {uri}",
            );

            // Invalid in Draft 6+ (can't use boolean with minimum)
            let invalid_schema = json!({
                "$schema": uri,
                "minimum": 5,
                "exclusiveMinimum": true
            });
            assert!(
                !crate::meta::is_valid(&invalid_schema),
                "Schema should be invalid for {uri}",
            );
            assert!(
                crate::meta::validate(&invalid_schema).is_err(),
                "Schema validation should fail for {uri}",
            );
        }
    }

    #[test_case(
        "http://json-schema.org/draft-04/schema#",
        true,
        5,
        true ; "draft4 valid"
    )]
    #[test_case(
        "http://json-schema.org/draft-04/schema#",
        5,
        true,
        false ; "draft4 invalid"
    )]
    #[test_case(
        "http://json-schema.org/draft-06/schema#",
        5,
        true,
        false ; "draft6 invalid"
    )]
    #[test_case(
        "http://json-schema.org/draft-07/schema#",
        5,
        true,
        false ; "draft7 invalid"
    )]
    #[test_case(
        "https://json-schema.org/draft/2019-09/schema",
        5,
        true,
        false ; "draft2019-09 invalid"
    )]
    #[test_case(
        "https://json-schema.org/draft/2020-12/schema",
        5,
        true,
        false ; "draft2020-12 invalid"
    )]
    fn test_exclusive_minimum_detection(
        schema_uri: &str,
        exclusive_minimum: impl Into<serde_json::Value>,
        minimum: impl Into<serde_json::Value>,
        expected: bool,
    ) {
        let schema = json!({
            "$schema": schema_uri,
            "minimum": minimum.into(),
            "exclusiveMinimum": exclusive_minimum.into()
        });

        let is_valid_result = crate::meta::is_valid(&schema);
        assert_eq!(is_valid_result, expected);

        let validate_result = crate::meta::validate(&schema);
        assert_eq!(validate_result.is_ok(), expected);
    }

    #[test]
    fn test_invalid_schema_uri() {
        let schema = json!({
            "$schema": "invalid-uri",
            "type": "string"
        });

        let result = crate::options().without_schema_validation().build(&schema);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Unknown meta-schema"));
        assert!(error.to_string().contains("invalid-uri"));
    }

    #[test]
    fn test_invalid_schema_keyword() {
        let schema = json!({
            // Note `htt`, not `http`
            "$schema": "htt://json-schema.org/draft-07/schema",
            "type": "string"
        });

        // Without registering the meta-schema, this should fail
        let result = crate::options().without_schema_validation().build(&schema);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Unknown meta-schema"));
        assert!(error
            .to_string()
            .contains("htt://json-schema.org/draft-07/schema"));
    }

    #[test_case(Draft::Draft4)]
    #[test_case(Draft::Draft6)]
    #[test_case(Draft::Draft7)]
    fn meta_schemas(draft: Draft) {
        // See GH-258
        for schema in [json!({"enum": [0, 0.0]}), json!({"enum": []})] {
            assert!(crate::options().with_draft(draft).build(&schema).is_ok());
        }
    }

    #[test]
    fn incomplete_escape_in_pattern() {
        // See GH-253
        let schema = json!({"pattern": "\\u"});
        assert!(crate::validator_for(&schema).is_err());
    }

    #[test]
    fn validation_error_propagation() {
        fn foo() -> Result<(), Box<dyn std::error::Error>> {
            let schema = json!({});
            let validator = validator_for(&schema)?;
            let _ = validator.is_valid(&json!({}));
            Ok(())
        }
        let _ = foo();
    }

    #[test]
    fn test_meta_validation_with_unknown_schema() {
        let schema = json!({
            "$schema": "json-schema:///custom",
            "type": "string"
        });

        // Meta-validation now errors when the meta-schema is unknown/unregistered
        assert!(crate::meta::validate(&schema).is_err());

        // Building a validator also fails without registration
        let result = crate::validator_for(&schema);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(all(not(target_arch = "wasm32"), feature = "resolve-file"))]
    fn test_meta_validation_respects_metaschema_draft() {
        use std::io::Write;

        let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let meta_schema_draft7 = json!({
            "$id": "http://example.com/meta/draft7",
            "$schema": "http://json-schema.org/draft-07/schema",
            "type": ["object", "boolean"],
            "properties": {
                "$schema": { "type": "string" },
                "type": {},
                "properties": { "type": "object" }
            },
            "additionalProperties": false
        });
        write!(temp_file, "{meta_schema_draft7}").expect("Failed to write to temp file");

        let uri = crate::retriever::path_to_uri(temp_file.path());

        let schema_using_draft7_meta = json!({
            "$schema": uri,
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "unevaluatedProperties": false
        });

        let schema_valid_for_draft7_meta = json!({
            "$schema": uri,
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            }
        });

        assert!(crate::meta::is_valid(&meta_schema_draft7));
        assert!(!crate::meta::is_valid(&schema_using_draft7_meta));
        assert!(crate::meta::is_valid(&schema_valid_for_draft7_meta));
    }

    #[test]
    #[cfg(all(not(target_arch = "wasm32"), feature = "resolve-file"))]
    fn test_meta_schema_chain_resolution() {
        use std::io::Write;

        // Create intermediate meta-schema pointing to Draft 2020-12
        let mut intermediate_file =
            tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let intermediate_meta = json!({
            "$id": "http://example.com/meta/intermediate",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object"
        });
        write!(intermediate_file, "{intermediate_meta}").expect("Failed to write to temp file");
        let intermediate_uri = crate::retriever::path_to_uri(intermediate_file.path());

        // Create custom meta-schema with unknown draft that points to intermediate
        // This triggers the chain resolution code path in resolve_meta_schema_chain
        let mut custom_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let custom_meta = json!({
            "$id": "http://example.com/meta/custom",
            "$schema": intermediate_uri,
            "type": "object"
        });
        write!(custom_file, "{custom_meta}").expect("Failed to write to temp file");
        let custom_uri = crate::retriever::path_to_uri(custom_file.path());

        let schema = json!({
            "$schema": custom_uri,
            "type": "string"
        });

        // Should successfully resolve through the chain and detect Draft 2020-12
        assert!(crate::meta::is_valid(&schema));
    }

    #[test]
    #[cfg(all(not(target_arch = "wasm32"), feature = "resolve-file"))]
    fn test_circular_meta_schema_reference() {
        use std::io::Write;

        // Create meta-schema A pointing to meta-schema B
        let mut meta_a_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let meta_a_uri = crate::retriever::path_to_uri(meta_a_file.path());

        // Create meta-schema B pointing back to meta-schema A
        let mut meta_b_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let meta_b_uri = crate::retriever::path_to_uri(meta_b_file.path());

        let meta_a = json!({
            "$id": "http://example.com/meta/a",
            "$schema": &meta_b_uri,
            "type": "object"
        });
        write!(meta_a_file, "{meta_a}").expect("Failed to write to temp file");

        let meta_b = json!({
            "$id": "http://example.com/meta/b",
            "$schema": &meta_a_uri,
            "type": "object"
        });
        write!(meta_b_file, "{meta_b}").expect("Failed to write to temp file");

        let schema = json!({
            "$schema": meta_a_uri.clone(),
            "type": "string"
        });

        // Should return a circular meta-schema error
        let result = crate::meta::options().validate(&schema);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Circular meta-schema reference"));
    }

    #[test]
    fn simple_schema_with_unknown_draft() {
        // Define a custom meta-schema
        let meta_schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "http://custom.example.com/schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2020-12/vocab/core": true,
                "https://json-schema.org/draft/2020-12/vocab/applicator": true,
                "https://json-schema.org/draft/2020-12/vocab/validation": true,
            }
        });

        // Schema using the custom meta-schema
        let schema = json!({
            "$schema": "http://custom.example.com/schema",
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            }
        });

        // Register the custom meta-schema as a resource
        let resources = vec![(
            "http://custom.example.com/schema".to_string(),
            Resource::from_contents(meta_schema),
        )];

        let validator = crate::options()
            .without_schema_validation()
            .with_resources(resources.into_iter())
            .build(&schema)
            .expect("Should build validator");

        // Valid instance
        assert!(validator.is_valid(&json!({"name": "test"})));

        // Invalid instance - name should be string, not number
        assert!(!validator.is_valid(&json!({"name": 123})));

        // Also verify type validation works
        assert!(!validator.is_valid(&json!("not an object")));
    }

    #[test]
    fn custom_meta_schema_support() {
        // Define a custom meta-schema that extends Draft 2020-12
        let meta_schema = json!({
            "$id": "http://example.com/meta/schema",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Core schema definition",
            "type": "object",
            "allOf": [
                {
                    "$ref": "#/$defs/editable"
                },
                {
                    "$ref": "#/$defs/core"
                }
            ],
            "properties": {
                "properties": {
                    "type": "object",
                    "patternProperties": {
                        ".*": {
                            "type": "object",
                            "properties": {
                                "type": {
                                    "type": "string",
                                    "enum": [
                                        "array",
                                        "boolean",
                                        "integer",
                                        "number",
                                        "object",
                                        "string",
                                        "null"
                                    ]
                                }
                            }
                        }
                    },
                    "propertyNames": {
                        "type": "string",
                        "pattern": "^[A-Za-z_][A-Za-z0-9_]*$"
                    }
                }
            },
            "unevaluatedProperties": false,
            "required": [
                "properties"
            ],
            "$defs": {
                "core": {
                    "type": "object",
                    "properties": {
                        "$id": {
                            "type": "string"
                        },
                        "$schema": {
                            "type": "string"
                        },
                        "type": {
                            "const": "object"
                        },
                        "title": {
                            "type": "string"
                        },
                        "description": {
                            "type": "string"
                        },
                        "additionalProperties": {
                            "type": "boolean",
                            "const": false
                        }
                    },
                    "required": [
                        "$id",
                        "$schema",
                        "type"
                    ]
                },
                "editable": {
                    "type": "object",
                    "properties": {
                        "creationDate": {
                            "type": "string",
                            "format": "date-time"
                        },
                        "updateDate": {
                            "type": "string",
                            "format": "date-time"
                        }
                    },
                    "required": [
                        "creationDate"
                    ]
                }
            }
        });

        // A schema that uses the custom meta-schema
        let element_schema = json!({
            "$schema": "http://example.com/meta/schema",
            "$id": "http://example.com/schemas/element",
            "title": "Element",
            "description": "An element",
            "creationDate": "2024-12-31T12:31:53+01:00",
            "properties": {
                "value": {
                    "type": "string"
                }
            },
            "type": "object"
        });

        // Build the validator with both the meta-schema and the element schema as resources
        let resources = vec![
            (
                "http://example.com/meta/schema".to_string(),
                referencing::Resource::from_contents(meta_schema),
            ),
            (
                "http://example.com/schemas/element".to_string(),
                referencing::Resource::from_contents(element_schema.clone()),
            ),
        ];

        let validator = crate::options()
            .without_schema_validation()
            .with_resources(resources.into_iter())
            .build(&element_schema)
            .expect("Should successfully build validator with custom meta-schema");

        let valid_instance = json!({
            "value": "test string"
        });
        assert!(validator.is_valid(&valid_instance));

        let invalid_instance = json!({
            "value": 123
        });
        assert!(!validator.is_valid(&invalid_instance));
    }

    #[test]
    fn custom_meta_schema_with_fragment_finds_vocabularies() {
        // Custom meta-schema URIs with trailing # should be found in registry
        let custom_meta = json!({
            "$id": "http://example.com/custom-with-unevaluated",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2020-12/vocab/core": true,
                "https://json-schema.org/draft/2020-12/vocab/applicator": true,
                "https://json-schema.org/draft/2020-12/vocab/validation": true,
                "https://json-schema.org/draft/2020-12/vocab/unevaluated": true
            }
        });

        let registry = Registry::try_new(
            "http://example.com/custom-with-unevaluated",
            Resource::from_contents(custom_meta),
        )
        .expect("Should create registry");

        let schema = json!({
            "$schema": "http://example.com/custom-with-unevaluated#",
            "type": "object",
            "properties": {
                "foo": { "type": "string" }
            },
            "unevaluatedProperties": false
        });

        let validator = crate::options()
            .without_schema_validation()
            .with_registry(registry)
            .build(&schema)
            .expect("Should build validator");

        assert!(validator.is_valid(&json!({"foo": "bar"})));
        assert!(!validator.is_valid(&json!({"foo": "bar", "extra": "value"})));
    }

    #[test]
    fn strict_meta_schema_catches_typos() {
        // Issue #764: Use strict meta-schema with unevaluatedProperties: false
        // to catch typos in schema keywords

        let strict_meta = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://json-schema.org/draft/2020-12/strict",
            "$dynamicAnchor": "meta",
            "$ref": "https://json-schema.org/draft/2020-12/schema",
            "unevaluatedProperties": false
        });

        let registry = Registry::try_new(
            "https://json-schema.org/draft/2020-12/strict",
            Resource::from_contents(strict_meta),
        )
        .expect("Should create registry");

        // Valid schema - all keywords are recognized
        let valid_schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/strict",
            "type": "object",
            "properties": {
                "name": {"type": "string", "minLength": 1}
            }
        });

        assert!(crate::meta::options()
            .with_registry(registry.clone())
            .is_valid(&valid_schema));

        // Invalid schema - top-level typo "typ" instead of "type"
        let invalid_schema_top_level = json!({
            "$schema": "https://json-schema.org/draft/2020-12/strict",
            "typ": "string"  // Typo
        });

        assert!(!crate::meta::options()
            .with_registry(registry.clone())
            .is_valid(&invalid_schema_top_level));

        // Invalid schema - nested invalid keyword "minSize" (not a real JSON Schema keyword)
        let invalid_schema_nested = json!({
            "$schema": "https://json-schema.org/draft/2020-12/strict",
            "type": "object",
            "properties": {
                "name": {"type": "string", "minSize": 1}  // Invalid keyword in nested schema
            }
        });

        assert!(!crate::meta::options()
            .with_registry(registry)
            .is_valid(&invalid_schema_nested));
    }

    #[test]
    fn custom_meta_schema_preserves_underlying_draft_behavior() {
        // Regression test: Custom meta-schemas should preserve the draft-specific
        // behavior of their underlying draft, not default to Draft 2020-12
        // Draft 7 specific behavior: $ref siblings are ignored

        let custom_meta_draft7 = json!({
            "$id": "http://example.com/meta/draft7-custom",
            "$schema": "http://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "customKeyword": {"type": "string"}
            }
        });

        let registry = Registry::try_new(
            "http://example.com/meta/draft7-custom",
            Resource::from_contents(custom_meta_draft7),
        )
        .expect("Should create registry");

        let schema = json!({
            "$schema": "http://example.com/meta/draft7-custom",
            "$ref": "#/$defs/positiveNumber",
            "maximum": 5,
            "$defs": {
                "positiveNumber": {
                    "type": "number",
                    "minimum": 0
                }
            }
        });

        let validator = crate::options()
            .without_schema_validation()
            .with_registry(registry)
            .build(&schema)
            .expect("Should build validator");

        // In Draft 7: siblings of $ref are ignored, so maximum: 5 has no effect
        // In Draft 2020-12: siblings are evaluated, so maximum: 5 would apply
        assert!(validator.is_valid(&json!(10)));
    }

    mod meta_options_tests {
        use super::*;
        use crate::{Registry, Resource};

        #[test]
        fn test_meta_options_with_registry_valid_schema() {
            let custom_meta = Resource::from_contents(json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" },
                    "maxLength": { "type": "integer" }
                },
                "additionalProperties": false
            }));

            let registry = Registry::try_new("http://example.com/meta", custom_meta).unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta",
                "type": "string",
                "maxLength": 10
            });

            assert!(crate::meta::options()
                .with_registry(registry.clone())
                .is_valid(&schema));

            assert!(crate::meta::options()
                .with_registry(registry)
                .validate(&schema)
                .is_ok());
        }

        #[test]
        fn test_meta_options_with_registry_invalid_schema() {
            let custom_meta = Resource::from_contents(json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "type": { "type": "string" }
                },
                "additionalProperties": false
            }));

            let registry = Registry::try_new("http://example.com/meta", custom_meta).unwrap();

            // Schema has disallowed property
            let schema = json!({
                "$schema": "http://example.com/meta",
                "type": "string",
                "maxLength": 10  // Not allowed by custom meta-schema
            });

            assert!(!crate::meta::options()
                .with_registry(registry.clone())
                .is_valid(&schema));

            assert!(crate::meta::options()
                .with_registry(registry)
                .validate(&schema)
                .is_err());
        }

        #[test]
        fn test_meta_options_with_registry_chain() {
            // Create a chain: custom-meta -> draft2020-12
            let custom_meta = Resource::from_contents(json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object"
            }));

            let registry = Registry::try_new("http://example.com/custom", custom_meta).unwrap();

            let schema = json!({
                "$schema": "http://example.com/custom",
                "type": "string"
            });

            assert!(crate::meta::options()
                .with_registry(registry)
                .is_valid(&schema));
        }

        #[test]
        fn test_meta_options_with_registry_multi_level_chain() {
            // Create chain: schema -> meta-level-2 -> meta-level-1 -> draft2020-12
            let meta_level_1 = Resource::from_contents(json!({
                "$id": "http://example.com/meta/level1",
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "customProp": { "type": "boolean" }
                }
            }));

            let meta_level_2 = Resource::from_contents(json!({
                "$id": "http://example.com/meta/level2",
                "$schema": "http://example.com/meta/level1",
                "type": "object",
                "customProp": true
            }));

            let registry = Registry::try_from_resources([
                ("http://example.com/meta/level1", meta_level_1),
                ("http://example.com/meta/level2", meta_level_2),
            ])
            .unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta/level2",
                "type": "string",
                "customProp": true
            });

            assert!(crate::meta::options()
                .with_registry(registry)
                .is_valid(&schema));
        }

        #[test]
        fn test_meta_options_with_registry_multi_document_meta_schema() {
            let shared_constraints = Resource::from_contents(json!({
                "$id": "http://example.com/meta/shared",
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "maxLength": { "type": "integer", "minimum": 0 }
                }
            }));

            let root_meta = Resource::from_contents(json!({
                "$id": "http://example.com/meta/root",
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" }
                },
                "allOf": [
                    { "$ref": "http://example.com/meta/shared" }
                ]
            }));

            let registry = Registry::try_from_resources([
                ("http://example.com/meta/root", root_meta),
                ("http://example.com/meta/shared", shared_constraints),
            ])
            .unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta/root",
                "type": "string",
                "maxLength": 5
            });

            let result = crate::meta::options()
                .with_registry(registry.clone())
                .validate(&schema);

            assert!(
                result.is_ok(),
                "meta validation failed even though registry contains all meta-schemas: {}",
                result.unwrap_err()
            );

            assert!(crate::meta::options()
                .with_registry(registry)
                .is_valid(&schema));
        }

        #[test]
        fn test_meta_options_without_registry_unknown_metaschema() {
            let schema = json!({
                "$schema": "http://0.0.0.0/nonexistent",
                "type": "string"
            });

            // Without registry, should fail to resolve
            let result = crate::meta::options().validate(&schema);
            assert!(result.is_err());
        }

        #[test]
        #[should_panic(expected = "Failed to resolve meta-schema")]
        fn test_meta_options_is_valid_panics_on_missing_metaschema() {
            let schema = json!({
                "$schema": "http://0.0.0.0/nonexistent",
                "type": "string"
            });

            // is_valid() should panic if meta-schema cannot be resolved
            let _ = crate::meta::options().is_valid(&schema);
        }

        #[test]
        fn test_meta_options_with_registry_missing_metaschema() {
            let custom_meta = Resource::from_contents(json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object"
            }));

            let registry = Registry::try_new("http://example.com/meta1", custom_meta).unwrap();

            // Schema references a different meta-schema not in registry
            let schema = json!({
                "$schema": "http://example.com/meta2",
                "type": "string"
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_err());
        }

        #[test]
        fn test_meta_options_circular_reference_detection() {
            // Create a circular reference: meta1 -> meta2 -> meta1
            let meta1 = Resource::from_contents(json!({
                "$id": "http://example.com/meta1",
                "$schema": "http://example.com/meta2",
                "type": "object"
            }));

            let meta2 = Resource::from_contents(json!({
                "$id": "http://example.com/meta2",
                "$schema": "http://example.com/meta1",
                "type": "object"
            }));

            let registry = Registry::try_from_resources([
                ("http://example.com/meta1", meta1),
                ("http://example.com/meta2", meta2),
            ])
            .unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta1",
                "type": "string"
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_err());
            // Check it's specifically a circular error
            let err = result.unwrap_err();
            assert!(err.to_string().contains("Circular"));
        }

        #[test]
        fn test_meta_options_standard_drafts_without_registry() {
            // Standard drafts should work without registry
            let schemas = vec![
                json!({ "$schema": "http://json-schema.org/draft-04/schema#", "type": "string" }),
                json!({ "$schema": "http://json-schema.org/draft-06/schema#", "type": "string" }),
                json!({ "$schema": "http://json-schema.org/draft-07/schema#", "type": "string" }),
                json!({ "$schema": "https://json-schema.org/draft/2019-09/schema", "type": "string" }),
                json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string" }),
            ];

            for schema in schemas {
                assert!(
                    crate::meta::options().is_valid(&schema),
                    "Failed for schema: {schema}"
                );
            }
        }

        #[test]
        fn test_meta_options_validate_returns_specific_errors() {
            let custom_meta = Resource::from_contents(json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "required": ["type"]
            }));

            let registry = Registry::try_new("http://example.com/meta", custom_meta).unwrap();

            // Schema missing required property
            let schema = json!({
                "$schema": "http://example.com/meta",
                "properties": {
                    "name": { "type": "string" }
                }
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.to_string().contains("required") || err.to_string().contains("type"));
        }

        #[test]
        fn test_meta_options_builds_validator_with_resolved_draft() {
            let custom_meta = Resource::from_contents(json!({
                "$id": "http://example.com/meta/draft7-based",
                "$schema": "http://json-schema.org/draft-07/schema#",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" },
                    "minLength": { "type": "integer" }
                },
                "additionalProperties": false
            }));

            let registry =
                Registry::try_new("http://example.com/meta/draft7-based", custom_meta).unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta/draft7-based",
                "type": "string",
                "minLength": 5
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_ok());
        }

        #[test]
        fn test_meta_options_validator_uses_correct_draft() {
            let custom_meta_draft6 = Resource::from_contents(json!({
                "$id": "http://example.com/meta/draft6-based",
                "$schema": "http://json-schema.org/draft-06/schema#",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" },
                    "exclusiveMinimum": { "type": "number" }
                },
                "additionalProperties": false
            }));

            let registry =
                Registry::try_new("http://example.com/meta/draft6-based", custom_meta_draft6)
                    .unwrap();

            let schema_valid_for_draft6 = json!({
                "$schema": "http://example.com/meta/draft6-based",
                "type": "number",
                "exclusiveMinimum": 0
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema_valid_for_draft6);

            assert!(result.is_ok());
        }

        #[test]
        fn test_meta_options_without_schema_validation_in_built_validator() {
            let custom_meta = Resource::from_contents(json!({
                "$id": "http://example.com/meta/custom",
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" }
                },
                "additionalProperties": false
            }));

            let registry =
                Registry::try_new("http://example.com/meta/custom", custom_meta).unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta/custom",
                "type": "string"
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_ok());
        }

        #[test]
        fn test_meta_validation_uses_resolved_draft_from_chain() {
            // Chain: user-schema -> custom-meta -> Draft 4
            // Validator should use Draft 4 rules to validate the schema
            let custom_meta = Resource::from_contents(json!({
                "$id": "http://example.com/meta/draft4-based",
                "$schema": "http://json-schema.org/draft-04/schema#",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" },
                    "enum": { "type": "array" },
                    "const": { "type": "string" }
                },
                "additionalProperties": false
            }));

            let registry =
                Registry::try_new("http://example.com/meta/draft4-based", custom_meta).unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta/draft4-based",
                "type": "string",
                "const": "foo"
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_ok());
        }

        #[test]
        fn test_meta_validation_multi_level_chain_uses_resolved_draft() {
            // Multi-level chain: user-schema -> meta-2 -> meta-1 -> Draft 4
            let meta_level_1 = Resource::from_contents(json!({
                "$id": "http://example.com/meta/level1",
                "$schema": "http://json-schema.org/draft-04/schema#",
                "type": "object",
                "properties": {
                    "customKeyword": { "type": "boolean" }
                }
            }));

            let meta_level_2 = Resource::from_contents(json!({
                "$id": "http://example.com/meta/level2",
                "$schema": "http://example.com/meta/level1",
                "type": "object",
                "properties": {
                    "$schema": { "type": "string" },
                    "type": { "type": "string" },
                    "minimum": { "type": "number" },
                    "exclusiveMinimum": { "type": "boolean" }
                },
                "customKeyword": true,
                "additionalProperties": false
            }));

            let registry = Registry::try_from_resources([
                ("http://example.com/meta/level1", meta_level_1),
                ("http://example.com/meta/level2", meta_level_2),
            ])
            .unwrap();

            let schema = json!({
                "$schema": "http://example.com/meta/level2",
                "type": "number",
                "minimum": 5,
                "exclusiveMinimum": true
            });

            let result = crate::meta::options()
                .with_registry(registry)
                .validate(&schema);

            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_meta_validator_for_valid_schema() {
        let schema = json!({
            "type": "string",
            "maxLength": 5
        });

        let validator = crate::meta::validator_for(&schema).expect("Valid meta-schema");
        assert!(validator.is_valid(&schema));
    }

    #[test]
    fn test_meta_validator_for_invalid_schema() {
        let schema = json!({
            "type": "invalid_type"
        });

        let validator = crate::meta::validator_for(&schema).expect("Valid meta-schema");
        assert!(!validator.is_valid(&schema));
    }

    #[test]
    fn test_meta_validator_for_evaluate_api() {
        let schema = json!({
            "type": "string",
            "maxLength": 5
        });

        let validator = crate::meta::validator_for(&schema).expect("Valid meta-schema");
        let evaluation = validator.evaluate(&schema);

        let flag = evaluation.flag();
        assert!(flag.valid);
    }

    #[test]
    fn test_meta_validator_for_evaluate_api_invalid() {
        let schema = json!({
            "type": "invalid_type",
            "minimum": "not a number"
        });

        let validator = crate::meta::validator_for(&schema).expect("Valid meta-schema");
        let evaluation = validator.evaluate(&schema);

        let flag = evaluation.flag();
        assert!(!flag.valid);
    }

    #[test]
    fn test_meta_validator_for_all_drafts() {
        let schemas = vec![
            json!({ "$schema": "http://json-schema.org/draft-04/schema#", "type": "string" }),
            json!({ "$schema": "http://json-schema.org/draft-06/schema#", "type": "string" }),
            json!({ "$schema": "http://json-schema.org/draft-07/schema#", "type": "string" }),
            json!({ "$schema": "https://json-schema.org/draft/2019-09/schema", "type": "string" }),
            json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "type": "string" }),
        ];

        for schema in schemas {
            let validator = crate::meta::validator_for(&schema).unwrap();
            assert!(validator.is_valid(&schema));
        }
    }

    #[test]
    fn test_meta_validator_for_iter_errors() {
        let schema = json!({
            "type": "invalid_type",
            "minimum": "not a number"
        });

        let validator = crate::meta::validator_for(&schema).expect("Valid meta-schema");
        let errors: Vec<_> = validator.iter_errors(&schema).collect();
        assert!(!errors.is_empty());
    }
}

#[cfg(all(test, feature = "resolve-async", not(target_family = "wasm")))]
mod async_tests {
    use referencing::Resource;
    use std::{collections::HashMap, sync::Arc};

    use serde_json::json;

    use crate::{AsyncRetrieve, Draft, Uri};

    /// Mock async retriever for testing
    #[derive(Clone)]
    struct TestRetriever {
        schemas: HashMap<String, serde_json::Value>,
    }

    impl TestRetriever {
        fn new() -> Self {
            let mut schemas = HashMap::new();
            schemas.insert(
                "https://example.com/user.json".to_string(),
                json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "integer", "minimum": 0}
                    },
                    "required": ["name"]
                }),
            );
            Self { schemas }
        }
    }

    #[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
    impl AsyncRetrieve for TestRetriever {
        async fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
            self.schemas
                .get(uri.as_str())
                .cloned()
                .ok_or_else(|| "Schema not found".into())
        }
    }

    #[tokio::test]
    async fn test_async_validator_for() {
        let schema = json!({
            "$ref": "https://example.com/user.json"
        });

        let validator = crate::async_options()
            .with_retriever(TestRetriever::new())
            .build(&schema)
            .await
            .unwrap();

        // Valid instance
        assert!(validator.is_valid(&json!({
            "name": "John Doe",
            "age": 30
        })));

        // Invalid instances
        assert!(!validator.is_valid(&json!({
            "age": -5
        })));
        assert!(!validator.is_valid(&json!({
            "name": 123,
            "age": 30
        })));
    }

    #[tokio::test]
    async fn test_async_options_with_draft() {
        let schema = json!({
            "$ref": "https://example.com/user.json"
        });

        let validator = crate::async_options()
            .with_draft(Draft::Draft202012)
            .with_retriever(TestRetriever::new())
            .build(&schema)
            .await
            .unwrap();

        assert!(validator.is_valid(&json!({
            "name": "John Doe",
            "age": 30
        })));
    }

    #[tokio::test]
    async fn test_async_retrieval_failure() {
        let schema = json!({
            "$ref": "https://example.com/nonexistent.json"
        });

        let result = crate::async_options()
            .with_retriever(TestRetriever::new())
            .build(&schema)
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Schema not found"));
    }

    #[tokio::test]
    async fn test_async_nested_references() {
        let mut retriever = TestRetriever::new();
        retriever.schemas.insert(
            "https://example.com/nested.json".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "user": { "$ref": "https://example.com/user.json" }
                }
            }),
        );

        let schema = json!({
            "$ref": "https://example.com/nested.json"
        });

        let validator = crate::async_options()
            .with_retriever(retriever)
            .build(&schema)
            .await
            .unwrap();

        // Valid nested structure
        assert!(validator.is_valid(&json!({
            "user": {
                "name": "John Doe",
                "age": 30
            }
        })));

        // Invalid nested structure
        assert!(!validator.is_valid(&json!({
            "user": {
                "age": -5
            }
        })));
    }

    #[tokio::test]
    async fn test_async_with_registry() {
        use crate::Registry;

        // Create a registry with initial schemas
        let registry = Registry::options()
            .async_retriever(TestRetriever::new())
            .build([(
                "https://example.com/user.json",
                Resource::from_contents(json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "integer", "minimum": 0}
                    },
                    "required": ["name"]
                })),
            )])
            .await
            .unwrap();

        // Create a validator using the pre-populated registry
        let validator = crate::async_options()
            .with_registry(registry)
            .build(&json!({
                "$ref": "https://example.com/user.json"
            }))
            .await
            .unwrap();

        // Verify that validation works with the registry
        assert!(validator.is_valid(&json!({
            "name": "John Doe",
            "age": 30
        })));
        assert!(!validator.is_valid(&json!({
            "age": -5
        })));
    }

    #[tokio::test]
    async fn test_async_validator_for_basic() {
        let schema = json!({"type": "integer"});

        let validator = crate::async_validator_for(&schema).await.unwrap();

        assert!(validator.is_valid(&json!(42)));
        assert!(!validator.is_valid(&json!("abc")));
    }

    #[tokio::test]
    async fn test_async_build_future_is_send() {
        let schema = Arc::new(json!({
            "$ref": "https://example.com/user.json"
        }));
        let retriever = TestRetriever::new();

        let handle = tokio::spawn({
            let schema = Arc::clone(&schema);
            let retriever = retriever.clone();
            async move {
                crate::async_options()
                    .with_retriever(retriever)
                    .build(&schema)
                    .await
            }
        });

        let validator = handle.await.unwrap().unwrap();
        assert!(validator.is_valid(&json!({
            "name": "John Doe",
            "age": 30
        })));
    }
}
