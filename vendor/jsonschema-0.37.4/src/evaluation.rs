use crate::{paths::Location, ValidationError};
use ahash::AHashMap;
use referencing::Uri;
use serde::{
    ser::{SerializeMap, SerializeSeq, SerializeStruct},
    Serialize,
};
use std::{fmt, sync::Arc};

/// Annotations associated with an output unit.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotations(Arc<serde_json::Value>);

impl Annotations {
    /// Create a new `Annotations` instance.
    #[must_use]
    pub(crate) fn new(v: serde_json::Value) -> Self {
        Annotations(Arc::new(v))
    }

    /// Create a new `Annotations` instance from an Arc.
    #[must_use]
    pub(crate) fn from_arc(v: Arc<serde_json::Value>) -> Self {
        Annotations(v)
    }

    /// Returns the inner [`serde_json::Value`] of the annotation.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> serde_json::Value {
        Arc::try_unwrap(self.0).unwrap_or_else(|arc| (*arc).clone())
    }

    /// The `serde_json::Value` of the annotation.
    #[must_use]
    pub fn value(&self) -> &serde_json::Value {
        &self.0
    }
}

impl serde::Serialize for Annotations {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// Description of a validation error used within evaluation outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDescription {
    keyword: &'static str,
    message: String,
}

impl ErrorDescription {
    #[inline]
    #[must_use]
    pub(crate) fn new(keyword: &'static str, message: String) -> Self {
        Self { keyword, message }
    }

    /// Create an `ErrorDescription` from a `ValidationError`.
    #[inline]
    #[must_use]
    pub(crate) fn from_validation_error(e: &ValidationError<'_>) -> Self {
        ErrorDescription {
            keyword: e.kind().keyword(),
            message: e.to_string(),
        }
    }

    /// Returns the keyword associated with this error.
    #[inline]
    #[must_use]
    pub fn keyword(&self) -> &'static str {
        self.keyword
    }

    /// Returns the inner [`String`] of the error description.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.message
    }

    /// Returns the message of the error description.
    #[inline]
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ErrorDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct EvaluationNode {
    pub(crate) keyword_location: Location,
    pub(crate) absolute_keyword_location: Option<Arc<Uri<String>>>,
    pub(crate) schema_location: Arc<str>,
    pub(crate) instance_location: Location,
    pub(crate) valid: bool,
    pub(crate) annotations: Option<Annotations>,
    pub(crate) dropped_annotations: Option<Annotations>,
    pub(crate) errors: Vec<ErrorDescription>,
    pub(crate) children: Vec<EvaluationNode>,
}

impl EvaluationNode {
    pub(crate) fn valid(
        keyword_location: Location,
        absolute_keyword_location: Option<Arc<Uri<String>>>,
        schema_location: impl Into<Arc<str>>,
        instance_location: Location,
        annotations: Option<Annotations>,
        children: Vec<EvaluationNode>,
    ) -> Self {
        let schema_location = schema_location.into();
        EvaluationNode {
            keyword_location,
            absolute_keyword_location,
            schema_location,
            instance_location,
            valid: true,
            annotations,
            dropped_annotations: None,
            errors: Vec::new(),
            children,
        }
    }

    pub(crate) fn invalid(
        keyword_location: Location,
        absolute_keyword_location: Option<Arc<Uri<String>>>,
        schema_location: impl Into<Arc<str>>,
        instance_location: Location,
        annotations: Option<Annotations>,
        errors: Vec<ErrorDescription>,
        children: Vec<EvaluationNode>,
    ) -> Self {
        let schema_location = schema_location.into();
        EvaluationNode {
            keyword_location,
            absolute_keyword_location,
            schema_location,
            instance_location,
            valid: false,
            annotations: None,
            dropped_annotations: annotations,
            errors,
            children,
        }
    }
}

/// Result of evaluating a JSON instance against a schema.
///
/// This type provides access to structured output formats as defined in the
/// [JSON Schema specification](https://json-schema.org/draft/2020-12/json-schema-core#name-output-structure).
///
/// # Output Formats
///
/// The evaluation result can be accessed in three standard formats:
///
/// - **Flag**: Simple boolean validity indicator via [`flag()`](Self::flag)
/// - **List**: Flat list of all evaluation units via [`list()`](Self::list)
/// - **Hierarchical**: Nested tree structure via [`hierarchical()`](Self::hierarchical)
///
/// All formats are serializable to JSON using `serde_json`.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({"type": "string", "minLength": 3});
/// let validator = jsonschema::validator_for(&schema)?;
///
/// // Evaluate an instance
/// let instance = json!("ab");
/// let evaluation = validator.evaluate(&instance);
///
/// // Check validity with flag format
/// let flag = evaluation.flag();
/// assert!(!flag.valid);
///
/// // Get structured output as JSON
/// let list_output = serde_json::to_value(evaluation.list())?;
/// println!("{}", serde_json::to_string_pretty(&list_output)?);
///
/// // Iterate over errors
/// for error in evaluation.iter_errors() {
///     println!("Error at {}: {}", error.instance_location, error.error);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Evaluation {
    root: EvaluationNode,
}

impl Evaluation {
    pub(crate) fn new(root: EvaluationNode) -> Self {
        Evaluation { root }
    }
    /// Returns the flag output format.
    ///
    /// This is the simplest output format, containing only a boolean indicating
    /// whether the instance is valid according to the schema.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({"type": "number"});
    /// let validator = jsonschema::validator_for(&schema)?;
    ///
    /// let evaluation = validator.evaluate(&json!(42));
    /// let flag = evaluation.flag();
    /// assert!(flag.valid);
    ///
    /// let evaluation = validator.evaluate(&json!("not a number"));
    /// let flag = evaluation.flag();
    /// assert!(!flag.valid);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn flag(&self) -> FlagOutput {
        FlagOutput {
            valid: self.root.valid,
        }
    }
    /// Returns the list output format.
    ///
    /// This format provides a flat list of all evaluation units, where each unit
    /// contains information about a specific validation step including its location,
    /// validity, annotations, and errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "array",
    ///     "prefixItems": [{"type": "string"}],
    ///     "items": {"type": "integer"}
    /// });
    /// let validator = jsonschema::validator_for(&schema)?;
    /// let evaluation = validator.evaluate(&json!(["hello", "oops"]));
    ///
    /// assert_eq!(
    ///     serde_json::to_value(evaluation.list())?,
    ///     json!({
    ///         "valid": false,
    ///         "details": [
    ///             {"evaluationPath": "", "instanceLocation": "", "schemaLocation": "", "valid": false},
    ///             {
    ///                 "valid": false,
    ///                 "evaluationPath": "/items",
    ///                 "instanceLocation": "",
    ///                 "schemaLocation": "/items",
    ///                 "droppedAnnotations": true
    ///             },
    ///             {
    ///                 "valid": false,
    ///                 "evaluationPath": "/items",
    ///                 "instanceLocation": "/1",
    ///                 "schemaLocation": "/items"
    ///             },
    ///             {
    ///                 "valid": false,
    ///                 "evaluationPath": "/items/type",
    ///                 "instanceLocation": "/1",
    ///                 "schemaLocation": "/items/type",
    ///                 "errors": {"type": "\"oops\" is not of type \"integer\""}
    ///             },
    ///             {
    ///                 "valid": true,
    ///                 "evaluationPath": "/prefixItems",
    ///                 "instanceLocation": "",
    ///                 "schemaLocation": "/prefixItems",
    ///                 "annotations": 0
    ///             },
    ///             {
    ///                 "valid": true,
    ///                 "evaluationPath": "/prefixItems/0",
    ///                 "instanceLocation": "/0",
    ///                 "schemaLocation": "/prefixItems/0"
    ///             },
    ///             {
    ///                 "valid": true,
    ///                 "evaluationPath": "/prefixItems/0/type",
    ///                 "instanceLocation": "/0",
    ///                 "schemaLocation": "/prefixItems/0/type"
    ///             },
    ///             {
    ///                 "valid": true,
    ///                 "evaluationPath": "/type",
    ///                 "instanceLocation": "",
    ///                 "schemaLocation": "/type"
    ///             }
    ///         ]
    ///     })
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn list(&self) -> ListOutput<'_> {
        ListOutput { root: &self.root }
    }
    /// Returns the hierarchical output format.
    ///
    /// This format represents the evaluation as a tree structure that mirrors the
    /// schema's logical structure. Each node contains its validation result along
    /// with nested child nodes representing sub-schema evaluations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "array",
    ///     "prefixItems": [{"type": "string"}],
    ///     "items": {"type": "integer"}
    /// });
    /// let validator = jsonschema::validator_for(&schema)?;
    /// let evaluation = validator.evaluate(&json!(["hello", "oops"]));
    ///
    /// assert_eq!(
    ///     serde_json::to_value(evaluation.hierarchical())?,
    ///     json!({
    ///         "valid": false,
    ///         "evaluationPath": "",
    ///         "schemaLocation": "",
    ///         "instanceLocation": "",
    ///         "details": [
    ///             {
    ///                 "valid": false,
    ///                 "evaluationPath": "/items",
    ///                 "instanceLocation": "",
    ///                 "schemaLocation": "/items",
    ///                 "droppedAnnotations": true,
    ///                 "details": [
    ///                     {
    ///                         "valid": false,
    ///                         "evaluationPath": "/items",
    ///                         "instanceLocation": "/1",
    ///                         "schemaLocation": "/items",
    ///                         "details": [
    ///                             {
    ///                                 "valid": false,
    ///                                 "evaluationPath": "/items/type",
    ///                                 "instanceLocation": "/1",
    ///                                 "schemaLocation": "/items/type",
    ///                                 "errors": {"type": "\"oops\" is not of type \"integer\""}
    ///                             }
    ///                         ]
    ///                     }
    ///                 ]
    ///             },
    ///             {
    ///                 "valid": true,
    ///                 "evaluationPath": "/prefixItems",
    ///                 "instanceLocation": "",
    ///                 "schemaLocation": "/prefixItems",
    ///                 "annotations": 0,
    ///                 "details": [
    ///                     {
    ///                         "valid": true,
    ///                         "evaluationPath": "/prefixItems/0",
    ///                         "instanceLocation": "/0",
    ///                         "schemaLocation": "/prefixItems/0",
    ///                         "details": [
    ///                             {
    ///                                 "valid": true,
    ///                                 "evaluationPath": "/prefixItems/0/type",
    ///                                 "instanceLocation": "/0",
    ///                                 "schemaLocation": "/prefixItems/0/type"
    ///                             }
    ///                         ]
    ///                     }
    ///                 ]
    ///             },
    ///             {
    ///                 "valid": true,
    ///                 "evaluationPath": "/type",
    ///                 "instanceLocation": "",
    ///                 "schemaLocation": "/type"
    ///             }
    ///         ]
    ///     })
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn hierarchical(&self) -> HierarchicalOutput<'_> {
        HierarchicalOutput { root: &self.root }
    }
    /// Returns an iterator over all annotations produced during evaluation.
    ///
    /// Annotations are metadata emitted by keywords during successful validation.
    /// They can be used to collect information about which parts of a schema
    /// matched the instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "object",
    ///     "properties": {"name": {"type": "string"}, "age": {"type": "number", "minimum": 0}},
    ///     "required": ["name"]
    /// });
    /// let validator = jsonschema::validator_for(&schema)?;
    /// let evaluation = validator.evaluate(&json!({"name": "Alice", "age": 30}));
    ///
    /// let entries: Vec<_> = evaluation.iter_annotations().collect();
    /// assert_eq!(entries.len(), 1);
    /// assert_eq!(entries[0].schema_location, "/properties");
    /// assert_eq!(entries[0].instance_location.as_str(), "");
    /// assert_eq!(entries[0].annotations.value(), &json!(["age", "name"]));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn iter_annotations(&self) -> AnnotationIter<'_> {
        AnnotationIter::new(&self.root)
    }
    /// Returns an iterator over all errors produced during evaluation.
    ///
    /// Each error entry contains information about a validation failure,
    /// including its location in both the schema and instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use serde_json::json;
    ///
    /// let schema = json!({
    ///     "type": "object",
    ///     "required": ["name"],
    ///     "properties": {
    ///         "age": {"type": "number"}
    ///     }
    /// });
    /// let validator = jsonschema::validator_for(&schema)?;
    /// let evaluation = validator.evaluate(&json!({"name": "Bob", "age": "oops"}));
    ///
    /// let errors: Vec<_> = evaluation.iter_errors().collect();
    /// assert_eq!(errors.len(), 1);
    /// assert_eq!(errors[0].schema_location, "/properties/age/type");
    /// assert_eq!(errors[0].instance_location.as_str(), "/age");
    /// assert_eq!(errors[0].error.to_string(), "\"oops\" is not of type \"number\"");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn iter_errors(&self) -> ErrorIter<'_> {
        ErrorIter::new(&self.root)
    }
}

/// Flag output format containing only a validity indicator.
///
/// This is the simplest output format defined in the JSON Schema specification.
/// It contains only a single boolean field indicating whether validation succeeded.
///
/// # JSON Structure
///
/// ```json
/// {
///   "valid": true
/// }
/// ```
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({"type": "string"});
/// let validator = jsonschema::validator_for(&schema)?;
/// let evaluation = validator.evaluate(&json!("hello"));
///
/// let flag = evaluation.flag();
/// assert_eq!(flag.valid, true);
///
/// let output = serde_json::to_value(flag)?;
/// assert_eq!(output, json!({"valid": true}));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, Serialize)]
pub struct FlagOutput {
    /// Whether the instance is valid according to the schema.
    pub valid: bool,
}

/// List output format providing a flat list of evaluation units.
///
/// This format represents the evaluation result as a flat sequence where each
/// entry corresponds to a validation step. Each unit includes its evaluation path,
/// schema location, instance location, validity, and any annotations or errors.
///
/// See [`Evaluation::list`] for an example JSON payload produced by this type.
#[derive(Debug)]
pub struct ListOutput<'a> {
    root: &'a EvaluationNode,
}

/// Hierarchical output format providing a tree structure of evaluation results.
///
/// This format represents the evaluation as a nested tree that mirrors the logical
/// structure of the schema. Each node contains validation results and child nodes
/// representing nested sub-schema evaluations.
///
/// See [`Evaluation::hierarchical`] for an example JSON payload produced by this type.
#[derive(Debug)]
pub struct HierarchicalOutput<'a> {
    root: &'a EvaluationNode,
}

impl Serialize for ListOutput<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_list(self.root, serializer)
    }
}

impl Serialize for HierarchicalOutput<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_hierarchical(self.root, serializer)
    }
}

fn serialize_list<S>(root: &EvaluationNode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut state = serializer.serialize_struct("ListOutput", 2)?;
    state.serialize_field("valid", &root.valid)?;
    let mut entries = Vec::new();
    collect_list_entries(root, &mut entries);
    state.serialize_field("details", &entries)?;
    state.end()
}

fn serialize_hierarchical<S>(root: &EvaluationNode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serialize_unit(root, serializer, true)
}

fn collect_list_entries<'a>(node: &'a EvaluationNode, out: &mut Vec<ListEntry<'a>>) {
    // Note: The spec says "Output units which do not contain errors or annotations SHOULD be
    // excluded" but the official test suite includes all nodes. We include all nodes to match
    // the reference implementation and test suite expectations.
    out.push(ListEntry::new(node));
    for child in &node.children {
        collect_list_entries(child, out);
    }
}

fn serialize_unit<S>(
    node: &EvaluationNode,
    serializer: S,
    include_children: bool,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut state = serializer.serialize_struct("OutputUnit", 7)?;
    state.serialize_field("valid", &node.valid)?;
    state.serialize_field("evaluationPath", node.keyword_location.as_str())?;
    state.serialize_field("schemaLocation", node.schema_location.as_ref())?;
    state.serialize_field("instanceLocation", node.instance_location.as_str())?;
    if let Some(annotations) = &node.annotations {
        state.serialize_field("annotations", annotations)?;
    }
    if let Some(annotations) = &node.dropped_annotations {
        state.serialize_field("droppedAnnotations", annotations)?;
    }
    if !node.errors.is_empty() {
        state.serialize_field("errors", &ErrorEntriesSerializer(&node.errors))?;
    }
    if include_children && !node.children.is_empty() {
        state.serialize_field(
            "details",
            &DetailsSerializer {
                children: &node.children,
            },
        )?;
    }
    state.end()
}

pub(crate) fn format_schema_location(
    location: &Location,
    absolute: Option<&Arc<Uri<String>>>,
) -> Arc<str> {
    if let Some(uri) = absolute {
        let base = uri.as_str();
        if base.contains('#') {
            Arc::from(base)
        } else if location.as_str().is_empty() {
            Arc::from(format!("{base}#"))
        } else {
            Arc::from(format!("{base}#{}", location.as_str()))
        }
    } else {
        location.as_arc()
    }
}

struct ListEntry<'a> {
    node: &'a EvaluationNode,
}

impl<'a> ListEntry<'a> {
    fn new(node: &'a EvaluationNode) -> Self {
        ListEntry { node }
    }
}

impl Serialize for ListEntry<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_unit(self.node, serializer, false)
    }
}

struct DetailsSerializer<'a> {
    children: &'a [EvaluationNode],
}

impl Serialize for DetailsSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.children.len()))?;
        for child in self.children {
            seq.serialize_element(&SeqEntry { node: child })?;
        }
        seq.end()
    }
}

struct SeqEntry<'a> {
    node: &'a EvaluationNode,
}

impl Serialize for SeqEntry<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_unit(self.node, serializer, true)
    }
}

/// Entry describing annotations emitted by a keyword during evaluation.
///
/// Annotations are metadata produced by keywords during successful validation.
/// They provide additional information about which schema keywords matched
/// and what values they produced.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "name": {"type": "string"},
///         "age": {"type": "number"}
///     }
/// });
/// let validator = jsonschema::validator_for(&schema)?;
/// let instance = json!({"name": "Alice", "age": 30});
/// let evaluation = validator.evaluate(&instance);
/// let entry = evaluation.iter_annotations().next().unwrap();
/// assert_eq!(entry.schema_location, "/properties");
/// assert_eq!(entry.instance_location.as_str(), "");
/// assert_eq!(entry.annotations.value(), &json!(["age", "name"]));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AnnotationEntry<'a> {
    /// The JSON Pointer to the schema keyword that produced the annotation.
    pub schema_location: &'a str,
    /// The absolute URI of the keyword location, if available.
    pub absolute_keyword_location: Option<&'a Uri<String>>,
    /// The JSON Pointer to the instance location being validated.
    pub instance_location: &'a Location,
    /// The annotations produced by the keyword.
    pub annotations: &'a Annotations,
}

/// Entry describing errors emitted by a keyword during evaluation.
///
/// Error entries contain information about validation failures, including
/// the locations in both the schema and instance where the error occurred.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({
///     "type": "object",
///     "required": ["name"],
///     "properties": {
///         "age": {"type": "number"}
///     }
/// });
/// let validator = jsonschema::validator_for(&schema)?;
/// let instance = json!({"age": "oops"});
/// let evaluation = validator.evaluate(&instance);
/// let entry = evaluation.iter_errors().next().unwrap();
/// assert_eq!(entry.schema_location, "/properties/age/type");
/// assert_eq!(entry.instance_location.as_str(), "/age");
/// assert_eq!(entry.error.to_string(), "\"oops\" is not of type \"number\"");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ErrorEntry<'a> {
    /// The JSON Pointer to the schema keyword that produced the error.
    pub schema_location: &'a str,
    /// The absolute URI of the keyword location, if available.
    pub absolute_keyword_location: Option<&'a Uri<String>>,
    /// The JSON Pointer to the instance location that failed validation.
    pub instance_location: &'a Location,
    /// The error description.
    pub error: &'a ErrorDescription,
}

struct NodeIter<'a> {
    stack: Vec<&'a EvaluationNode>,
}

impl<'a> NodeIter<'a> {
    fn new(root: &'a EvaluationNode) -> Self {
        NodeIter { stack: vec![root] }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a EvaluationNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        for child in node.children.iter().rev() {
            self.stack.push(child);
        }
        Some(node)
    }
}

/// Iterator over annotations produced during evaluation.
///
/// This iterator traverses the evaluation tree and yields [`AnnotationEntry`]
/// for each node that produced annotations during validation.
///
/// Annotations are only present for nodes where validation succeeded.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "name": {"type": "string"},
///         "age": {"type": "number"}
///     }
/// });
/// let validator = jsonschema::validator_for(&schema)?;
/// let evaluation = validator.evaluate(&json!({"name": "Alice", "age": 30}));
///
/// let annotations: Vec<_> = evaluation.iter_annotations().collect();
/// assert_eq!(annotations.len(), 1);
/// assert_eq!(annotations[0].instance_location.as_str(), "");
/// assert_eq!(annotations[0].annotations.value(), &json!(["age", "name"]));
/// # Ok(())
/// # }
/// ```
pub struct AnnotationIter<'a> {
    nodes: NodeIter<'a>,
}

impl<'a> AnnotationIter<'a> {
    fn new(root: &'a EvaluationNode) -> Self {
        AnnotationIter {
            nodes: NodeIter::new(root),
        }
    }
}

impl<'a> Iterator for AnnotationIter<'a> {
    type Item = AnnotationEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.nodes.by_ref() {
            if let Some(annotations) = node.annotations.as_ref() {
                return Some(AnnotationEntry {
                    schema_location: &node.schema_location,
                    absolute_keyword_location: node.absolute_keyword_location.as_deref(),
                    instance_location: &node.instance_location,
                    annotations,
                });
            }
        }
        None
    }
}

/// Iterator over errors produced during evaluation.
///
/// This iterator traverses the evaluation tree and yields [`ErrorEntry`]
/// for each error encountered during validation.
///
/// Nodes can have multiple errors, and this iterator will yield all of them
/// in depth-first order.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use serde_json::json;
///
/// let schema = json!({
///     "type": "object",
///     "required": ["name"],
///     "properties": {
///         "name": {"type": "string"},
///         "age": {"type": "number", "minimum": 0}
///     }
/// });
/// let validator = jsonschema::validator_for(&schema)?;
/// let evaluation = validator.evaluate(&json!({"age": -5}));
///
/// let errors: Vec<_> = evaluation.iter_errors().collect();
/// assert_eq!(errors.len(), 2);
/// assert_eq!(errors[0].instance_location.as_str(), "/age");
/// assert_eq!(errors[0].schema_location, "/properties/age/minimum");
/// assert_eq!(errors[1].schema_location, "/required");
/// # Ok(())
/// # }
/// ```
pub struct ErrorIter<'a> {
    nodes: NodeIter<'a>,
    current: Option<(&'a EvaluationNode, usize)>,
}

impl<'a> ErrorIter<'a> {
    fn new(root: &'a EvaluationNode) -> Self {
        ErrorIter {
            nodes: NodeIter::new(root),
            current: None,
        }
    }
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = ErrorEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((node, idx)) = self.current {
                if idx < node.errors.len() {
                    let entry = ErrorEntry {
                        schema_location: &node.schema_location,
                        absolute_keyword_location: node.absolute_keyword_location.as_deref(),
                        instance_location: &node.instance_location,
                        error: &node.errors[idx],
                    };
                    self.current = Some((node, idx + 1));
                    return Some(entry);
                }
                self.current = None;
            }

            match self.nodes.next() {
                Some(node) => {
                    if node.errors.is_empty() {
                        continue;
                    }
                    self.current = Some((node, 0));
                }
                None => return None,
            }
        }
    }
}

struct ErrorEntriesSerializer<'a>(&'a [ErrorDescription]);

impl Serialize for ErrorEntriesSerializer<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut grouped: Vec<(&'static str, Vec<&str>)> = Vec::new();
        let mut indexes: AHashMap<&'static str, usize> = AHashMap::new();

        for error in self.0 {
            let keyword = error.keyword();
            let msg = error.message();
            if let Some(&idx) = indexes.get(keyword) {
                grouped[idx].1.push(msg);
            } else {
                indexes.insert(keyword, grouped.len());
                grouped.push((keyword, vec![msg]));
            }
        }

        let mut map = serializer.serialize_map(Some(grouped.len()))?;
        for (keyword, messages) in grouped {
            if messages.len() == 1 {
                map.serialize_entry(keyword, messages[0])?;
            } else {
                map.serialize_entry(keyword, &messages)?;
            }
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Arc;

    fn loc() -> Location {
        Location::new()
    }

    fn annotation(value: serde_json::Value) -> Annotations {
        Annotations::new(value)
    }

    impl ErrorDescription {
        fn from_string(s: &str) -> Self {
            ErrorDescription {
                keyword: "error",
                message: s.to_string(),
            }
        }
    }

    fn leaf_with_annotation(schema: &str, ann: serde_json::Value) -> EvaluationNode {
        EvaluationNode::valid(
            loc(),
            None,
            schema.to_string(),
            loc(),
            Some(annotation(ann)),
            Vec::new(),
        )
    }

    fn leaf_with_error(schema: &str, msg: &str) -> EvaluationNode {
        EvaluationNode::invalid(
            loc(),
            None,
            schema.to_string(),
            loc(),
            None,
            vec![ErrorDescription::from_string(msg)],
            Vec::new(),
        )
    }

    #[test]
    fn iter_annotations_visits_all_nodes() {
        let child = leaf_with_annotation("/child", json!({"k": "v"}));
        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"root": true}))),
            vec![child],
        );
        let evaluation = Evaluation::new(root);
        let entries: Vec<_> = evaluation.iter_annotations().collect();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].schema_location, "/root");
        assert_eq!(entries[1].schema_location, "/child");
    }

    #[test]
    fn iter_errors_visits_all_nodes() {
        let child = leaf_with_error("/child", "boom");
        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            None,
            vec![ErrorDescription::from_string("root error")],
            vec![child],
        );
        let evaluation = Evaluation::new(root);
        let entries: Vec<_> = evaluation.iter_errors().collect();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].error.to_string(), "root error");
        assert_eq!(entries[1].error.to_string(), "boom");
    }

    #[test]
    fn flag_output_valid() {
        let root = EvaluationNode::valid(loc(), None, "/root".to_string(), loc(), None, Vec::new());
        let evaluation = Evaluation::new(root);
        let flag = evaluation.flag();
        assert!(flag.valid);
    }

    #[test]
    fn flag_output_invalid() {
        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            None,
            vec![ErrorDescription::from_string("error")],
            Vec::new(),
        );
        let evaluation = Evaluation::new(root);
        let flag = evaluation.flag();
        assert!(!flag.valid);
    }

    #[test]
    fn flag_output_serialization() {
        let root = EvaluationNode::valid(loc(), None, "/root".to_string(), loc(), None, Vec::new());
        let evaluation = Evaluation::new(root);
        let flag = evaluation.flag();
        let serialized = serde_json::to_value(flag).expect("serialization succeeds");
        assert_eq!(serialized, json!({"valid": true}));
    }

    #[test]
    fn list_output_serialization_valid() {
        let root = EvaluationNode::valid(loc(), None, "#".to_string(), loc(), None, Vec::new());
        let evaluation = Evaluation::new(root);
        let list = evaluation.list();
        let serialized = serde_json::to_value(list).expect("serialization succeeds");
        assert_eq!(
            serialized,
            json!({
                "valid": true,
                "details": [
                    {
                        "valid": true,
                        "evaluationPath": "",
                        "schemaLocation": "#",
                        "instanceLocation": ""
                    }
                ]
            })
        );
    }

    #[test]
    fn list_output_serialization_with_children() {
        let child1 = leaf_with_annotation("/child1", json!({"key": "value"}));
        let child2 = leaf_with_error("/child2", "child error");
        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"root": true}))),
            vec![child1, child2],
        );
        let evaluation = Evaluation::new(root);
        let list = evaluation.list();
        let serialized = serde_json::to_value(list).expect("serialization succeeds");
        assert_eq!(
            serialized,
            json!({
                "valid": true,
                "details": [
                    {
                        "valid": true,
                        "evaluationPath": "",
                        "schemaLocation": "/root",
                        "instanceLocation": "",
                        "annotations": {"root": true}
                    },
                    {
                        "valid": true,
                        "evaluationPath": "",
                        "schemaLocation": "/child1",
                        "instanceLocation": "",
                        "annotations": {"key": "value"}
                    },
                    {
                        "valid": false,
                        "evaluationPath": "",
                        "schemaLocation": "/child2",
                        "instanceLocation": "",
                        "errors": {"error": "child error"}
                    }
                ]
            })
        );
    }

    #[test]
    fn hierarchical_output_serialization() {
        let child = leaf_with_annotation("/child", json!({"nested": "data"}));
        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"root": "annotation"}))),
            vec![child],
        );
        let evaluation = Evaluation::new(root);
        let hierarchical = evaluation.hierarchical();
        let serialized = serde_json::to_value(hierarchical).expect("serialization succeeds");
        assert_eq!(
            serialized,
            json!({
                "valid": true,
                "evaluationPath": "",
                "schemaLocation": "/root",
                "instanceLocation": "",
                "annotations": {"root": "annotation"},
                "details": [
                    {
                        "valid": true,
                        "evaluationPath": "",
                        "schemaLocation": "/child",
                        "instanceLocation": "",
                        "annotations": {"nested": "data"}
                    }
                ]
            })
        );
    }

    #[test]
    fn outputs_include_errors_and_dropped_annotations() {
        let invalid_child = EvaluationNode::invalid(
            loc(),
            None,
            "/items/type".to_string(),
            Location::new().join(1usize),
            None,
            vec![ErrorDescription::from_string("child error")],
            Vec::new(),
        );
        let prefix_child = leaf_with_annotation("/prefix", json!(0));
        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"dropped": true}))),
            vec![ErrorDescription::from_string("root failure")],
            vec![invalid_child, prefix_child],
        );
        let evaluation = Evaluation::new(root);
        let list = serde_json::to_value(evaluation.list()).expect("serialization succeeds");
        assert_eq!(
            list,
            json!({
                "valid": false,
                "details": [
                    {
                        "valid": false,
                        "evaluationPath": "",
                        "schemaLocation": "/root",
                        "instanceLocation": "",
                        "droppedAnnotations": {"dropped": true},
                        "errors": {"error": "root failure"}
                    },
                    {
                        "valid": false,
                        "evaluationPath": "",
                        "schemaLocation": "/items/type",
                        "instanceLocation": "/1",
                        "errors": {"error": "child error"}
                    },
                    {
                        "valid": true,
                        "evaluationPath": "",
                        "schemaLocation": "/prefix",
                        "instanceLocation": "",
                        "annotations": 0
                    }
                ]
            })
        );
        let hierarchical =
            serde_json::to_value(evaluation.hierarchical()).expect("serialization succeeds");
        assert_eq!(
            hierarchical,
            json!({
                "valid": false,
                "evaluationPath": "",
                "schemaLocation": "/root",
                "instanceLocation": "",
                "droppedAnnotations": {"dropped": true},
                "errors": {"error": "root failure"},
                "details": [
                    {
                        "valid": false,
                        "evaluationPath": "",
                        "schemaLocation": "/items/type",
                        "instanceLocation": "/1",
                        "errors": {"error": "child error"}
                    },
                    {
                        "valid": true,
                        "evaluationPath": "",
                        "schemaLocation": "/prefix",
                        "instanceLocation": "",
                        "annotations": 0
                    }
                ]
            })
        );
    }

    #[test]
    fn empty_evaluation_tree() {
        let root = EvaluationNode::valid(loc(), None, "/root".to_string(), loc(), None, Vec::new());
        let evaluation = Evaluation::new(root);

        // No annotations
        assert_eq!(evaluation.iter_annotations().count(), 0);
        // No errors
        assert_eq!(evaluation.iter_errors().count(), 0);

        let flag = evaluation.flag();
        assert!(flag.valid);
    }

    #[test]
    fn deep_nesting() {
        // Create a deeply nested tree: root -> level1 -> level2 -> level3
        let level3 = leaf_with_annotation("/level3", json!({"level": 3}));
        let level2 = EvaluationNode::valid(
            loc(),
            None,
            "/level2".to_string(),
            loc(),
            Some(annotation(json!({"level": 2}))),
            vec![level3],
        );
        let level1 = EvaluationNode::valid(
            loc(),
            None,
            "/level1".to_string(),
            loc(),
            Some(annotation(json!({"level": 1}))),
            vec![level2],
        );
        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"level": 0}))),
            vec![level1],
        );

        let evaluation = Evaluation::new(root);
        let annotations: Vec<_> = evaluation.iter_annotations().collect();
        assert_eq!(annotations.len(), 4);

        // Check depth-first order
        assert_eq!(annotations[0].schema_location, "/root");
        assert_eq!(annotations[1].schema_location, "/level1");
        assert_eq!(annotations[2].schema_location, "/level2");
        assert_eq!(annotations[3].schema_location, "/level3");
    }

    #[test]
    fn wide_tree() {
        // Create a wide tree with many siblings
        let children: Vec<_> = (0..10)
            .map(|i| leaf_with_annotation(&format!("/child{i}"), json!({"index": i})))
            .collect();

        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"root": true}))),
            children,
        );

        let evaluation = Evaluation::new(root);
        let annotations: Vec<_> = evaluation.iter_annotations().collect();
        assert_eq!(annotations.len(), 11); // root + 10 children
    }

    #[test]
    fn multiple_errors_per_node() {
        let errors = vec![
            ErrorDescription::from_string("error 1"),
            ErrorDescription::from_string("error 2"),
            ErrorDescription::from_string("error 3"),
        ];
        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            None,
            errors,
            Vec::new(),
        );

        let evaluation = Evaluation::new(root);
        let error_entries: Vec<_> = evaluation.iter_errors().collect();
        assert_eq!(error_entries.len(), 3);
        assert_eq!(error_entries[0].error.to_string(), "error 1");
        assert_eq!(error_entries[1].error.to_string(), "error 2");
        assert_eq!(error_entries[2].error.to_string(), "error 3");
    }

    #[test]
    fn mixed_valid_and_invalid_nodes() {
        let valid_child = leaf_with_annotation("/valid", json!({"ok": true}));
        let invalid_child = leaf_with_error("/invalid", "failed");

        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            Some(annotation(json!({"attempted": true}))),
            vec![ErrorDescription::from_string("root failed")],
            vec![valid_child, invalid_child],
        );

        let evaluation = Evaluation::new(root);

        // Should have 1 annotation (from valid child only; root has dropped annotations)
        let annotations: Vec<_> = evaluation.iter_annotations().collect();
        assert_eq!(annotations.len(), 1);
        assert_eq!(annotations[0].schema_location, "/valid");

        // Should have 2 errors (root + invalid child)
        let errors: Vec<_> = evaluation.iter_errors().collect();
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn annotations_iterator_skips_nodes_without_annotations() {
        let no_annotation =
            EvaluationNode::valid(loc(), None, "/no_ann".to_string(), loc(), None, Vec::new());
        let with_annotation = leaf_with_annotation("/with_ann", json!({"present": true}));

        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            None,
            vec![no_annotation, with_annotation],
        );

        let evaluation = Evaluation::new(root);
        let annotations: Vec<_> = evaluation.iter_annotations().collect();
        assert_eq!(annotations.len(), 1);
        assert_eq!(annotations[0].schema_location, "/with_ann");
    }

    #[test]
    fn errors_iterator_skips_nodes_without_errors() {
        let no_error = EvaluationNode::valid(
            loc(),
            None,
            "/no_error".to_string(),
            loc(),
            Some(annotation(json!({"ok": true}))),
            Vec::new(),
        );
        let with_error = leaf_with_error("/with_error", "failed");

        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            None,
            vec![no_error, with_error],
        );

        let evaluation = Evaluation::new(root);
        let errors: Vec<_> = evaluation.iter_errors().collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].schema_location, "/with_error");
    }

    #[test]
    fn error_entries_serialization_empty() {
        let entries = ErrorEntriesSerializer(&[]);
        let serialized = serde_json::to_value(&entries).expect("serialization succeeds");
        assert!(serialized.is_object());
        assert_eq!(serialized.as_object().unwrap().len(), 0);
    }

    #[test]
    fn error_entries_serialization_single() {
        let errors = vec![ErrorDescription::from_string("test error")];
        let entries = ErrorEntriesSerializer(&errors);
        let serialized = serde_json::to_value(&entries).expect("serialization succeeds");
        assert!(serialized.is_object());
        assert_eq!(serialized.as_object().unwrap().len(), 1);
        assert!(serialized.get("error").is_some());
    }

    #[test]
    fn error_entries_serialization_multiple() {
        let errors = vec![
            ErrorDescription::new("alpha", "error 1".to_string()),
            ErrorDescription::new("beta", "error 2".to_string()),
            ErrorDescription::new("gamma", "error 3".to_string()),
        ];
        let entries = ErrorEntriesSerializer(&errors);
        let serialized = serde_json::to_value(&entries).expect("serialization succeeds");
        assert_eq!(serialized.as_object().unwrap().len(), 3);
        assert!(serialized.get("alpha").is_some());
        assert!(serialized.get("beta").is_some());
        assert!(serialized.get("gamma").is_some());
    }

    #[test]
    fn error_entries_serialization_preserves_duplicates() {
        let errors = vec![
            ErrorDescription::new("required", "\"foo\" is required".to_string()),
            ErrorDescription::new("required", "\"bar\" is required".to_string()),
        ];
        let entries = ErrorEntriesSerializer(&errors);
        let serialized = serde_json::to_value(&entries).expect("serialization succeeds");
        let value = serialized
            .get("required")
            .expect("required keyword present")
            .as_array()
            .expect("multiple errors serialized as array");
        assert_eq!(value.len(), 2);
        assert_eq!(value[0], "\"foo\" is required");
        assert_eq!(value[1], "\"bar\" is required");
    }

    #[test]
    fn list_output_preserves_multiple_errors_per_keyword() {
        let errors = vec![
            ErrorDescription::new("required", "\"foo\" is required".to_string()),
            ErrorDescription::new("required", "\"bar\" is required".to_string()),
        ];
        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/required".to_string(),
            loc(),
            None,
            errors,
            Vec::new(),
        );

        let evaluation = Evaluation::new(root);
        let list = serde_json::to_value(evaluation.list()).expect("serialization succeeds");
        let root_unit = list
            .get("details")
            .and_then(|value| value.as_array())
            .and_then(|details| details.first())
            .expect("list output contains root unit");
        let errors = root_unit
            .get("errors")
            .and_then(|errors| errors.get("required"))
            .and_then(|value| value.as_array())
            .expect("errors serialized as array");
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0], "\"foo\" is required");
        assert_eq!(errors[1], "\"bar\" is required");
    }

    #[test]
    fn format_schema_location_without_absolute() {
        let location = Location::new().join("properties").join("name");
        let formatted = format_schema_location(&location, None);
        assert_eq!(formatted.as_ref(), "/properties/name");
    }

    #[test]
    fn format_schema_location_with_absolute_no_fragment() {
        let location = Location::new().join("properties");
        let uri = Arc::new(
            Uri::parse("http://example.com/schema.json")
                .unwrap()
                .to_owned(),
        );
        let formatted = format_schema_location(&location, Some(&uri));
        assert_eq!(
            formatted.as_ref(),
            "http://example.com/schema.json#/properties"
        );
    }

    #[test]
    fn format_schema_location_with_absolute_empty_location() {
        let location = Location::new();
        let uri = Arc::new(
            Uri::parse("http://example.com/schema.json")
                .unwrap()
                .to_owned(),
        );
        let formatted = format_schema_location(&location, Some(&uri));
        assert_eq!(formatted.as_ref(), "http://example.com/schema.json#");
    }

    #[test]
    fn format_schema_location_with_absolute_existing_fragment() {
        let location = Location::new().join("properties");
        let uri = Arc::new(
            Uri::parse("http://example.com/schema.json#/defs/myDef")
                .unwrap()
                .to_owned(),
        );
        let formatted = format_schema_location(&location, Some(&uri));
        // When URI already contains a fragment, use it as-is
        assert_eq!(
            formatted.as_ref(),
            "http://example.com/schema.json#/defs/myDef"
        );
    }

    #[test]
    fn dropped_annotations_on_invalid_node() {
        let annotations = Some(annotation(json!({"dropped": true})));
        let root = EvaluationNode::invalid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            annotations.clone(),
            vec![ErrorDescription::from_string("failed")],
            Vec::new(),
        );

        assert!(!root.valid);
        assert!(root.annotations.is_none());
        assert!(root.dropped_annotations.is_some());
        assert_eq!(
            root.dropped_annotations.as_ref().unwrap(),
            annotations.as_ref().unwrap()
        );
    }

    #[test]
    fn valid_node_has_no_dropped_annotations() {
        let annotations = Some(annotation(json!({"kept": true})));
        let root = EvaluationNode::valid(
            loc(),
            None,
            "/root".to_string(),
            loc(),
            annotations.clone(),
            Vec::new(),
        );

        assert!(root.valid);
        assert!(root.annotations.is_some());
        assert!(root.dropped_annotations.is_none());
        assert_eq!(
            root.annotations.as_ref().unwrap(),
            annotations.as_ref().unwrap()
        );
    }

    #[test]
    fn absolute_keyword_location_populated_with_id() {
        use serde_json::json;

        // Schema with $id should populate absoluteKeywordLocation
        let schema = json!({
            "$id": "https://example.com/schema",
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        let validator = crate::validator_for(&schema).expect("schema compiles");
        let evaluation = validator.evaluate(&json!({"name": "test"}));

        // Verify that absoluteKeywordLocation is populated for nodes
        let annotations: Vec<_> = evaluation.iter_annotations().collect();
        assert!(!annotations.is_empty());

        // At least one annotation should have an absolute keyword location
        let with_absolute = annotations
            .iter()
            .filter(|a| a.absolute_keyword_location.is_some())
            .count();

        assert!(with_absolute > 0);

        // Verify the absolute locations start with the schema's $id
        for annotation in annotations
            .iter()
            .filter(|a| a.absolute_keyword_location.is_some())
        {
            let uri_str = annotation.absolute_keyword_location.unwrap().as_str();
            assert!(uri_str.starts_with("https://example.com/schema"));
        }
    }

    #[test]
    fn annotations_value_returns_reference() {
        let expected = json!({"key": "value"});
        let annotations = Annotations::new(expected.clone());

        // value() should return a reference to the inner value
        assert_eq!(annotations.value(), &expected);
    }

    #[test]
    fn annotations_into_inner_consumes_and_returns_value() {
        let expected = json!({"key": "value", "nested": {"array": [1, 2, 3]}});
        let annotations = Annotations::new(expected.clone());

        // into_inner() should consume self and return the owned value
        let inner = annotations.into_inner();
        assert_eq!(inner, expected);
    }

    #[test]
    fn error_description_into_inner_consumes_and_returns_message() {
        let expected_message = "test error message";
        let error = ErrorDescription::from_string(expected_message);

        // into_inner() should consume self and return the owned message
        let message = error.into_inner();
        assert_eq!(message, expected_message);
    }
}
