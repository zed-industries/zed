use crate::{
    compiler::Context,
    error::ErrorIterator,
    evaluation::{format_schema_location, Annotations, EvaluationNode},
    keywords::{BoxedValidator, Keyword},
    paths::{LazyLocation, Location},
    thread::{Shared, SharedWeak},
    validator::{EvaluationResult, Validate, ValidationContext},
    ValidationError,
};
use referencing::Uri;
use serde_json::Value;
use std::{
    cell::OnceCell,
    fmt,
    sync::{Arc, OnceLock},
};

/// A node in the schema tree, returned by `compiler::compile`
#[derive(Clone, Debug)]
pub(crate) struct SchemaNode {
    validators: Shared<NodeValidators>,
    location: Location,
    absolute_path: Option<Arc<Uri<String>>>,
}

// Separate type used only during compilation for handling recursive references
#[derive(Clone, Debug)]
pub(crate) struct PendingSchemaNode {
    cell: Shared<OnceLock<PendingTarget>>,
}

#[derive(Debug)]
struct PendingTarget {
    validators: SharedWeak<NodeValidators>,
    location: Location,
    absolute_path: Option<Arc<Uri<String>>>,
}

enum NodeValidators {
    /// The result of compiling a boolean valued schema, e.g
    ///
    /// ```json
    /// {
    ///     "additionalProperties": false
    /// }
    /// ```
    ///
    /// Here the result of `compiler::compile` called with the `false` value will return a
    /// `SchemaNode` with a single `BooleanValidator` as it's `validators`.
    Boolean { validator: Option<BoxedValidator> },
    /// The result of compiling a schema which is composed of keywords (almost all schemas)
    Keyword(KeywordValidators),
    /// The result of compiling a schema which is "array valued", e.g the "dependencies" keyword of
    /// draft 7 which can take values which are an array of other property names
    Array {
        validators: Vec<ArrayValidatorEntry>,
    },
}

impl fmt::Debug for NodeValidators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean { .. } => f.debug_struct("Boolean").finish(),
            Self::Keyword(_) => f.debug_tuple("Keyword").finish(),
            Self::Array { .. } => f.debug_struct("Array").finish(),
        }
    }
}

struct KeywordValidators {
    /// The keywords on this node which were not recognized by any vocabularies. These are
    /// stored so we can later produce them as annotations
    unmatched_keywords: Option<Arc<Value>>,
    // We should probably use AHashMap here but it breaks a bunch of tests which assume
    // validators are in a particular order
    validators: Vec<KeywordValidatorEntry>,
}

struct KeywordValidatorEntry {
    validator: BoxedValidator,
    location: Location,
    absolute_location: Option<Arc<Uri<String>>>,
}

struct ArrayValidatorEntry {
    validator: BoxedValidator,
    location: Location,
    absolute_location: Option<Arc<Uri<String>>>,
}

impl PendingSchemaNode {
    pub(crate) fn new() -> Self {
        PendingSchemaNode {
            cell: Shared::new(OnceLock::new()),
        }
    }

    pub(crate) fn initialize(&self, node: &SchemaNode) {
        let target = PendingTarget {
            validators: Shared::downgrade(&node.validators),
            location: node.location.clone(),
            absolute_path: node.absolute_path.clone(),
        };
        self.cell
            .set(target)
            .expect("pending node initialized twice");
    }

    pub(crate) fn get(&self) -> Option<SchemaNode> {
        self.cell.get().map(PendingTarget::materialize)
    }

    fn with_node<F, R>(&self, f: F) -> R
    where
        F: FnOnce(SchemaNode) -> R,
    {
        let node = self
            .cell
            .get()
            .expect("pending node accessed before initialization")
            .materialize();
        f(node)
    }

    /// Get a unique identifier for this pending node.
    /// Uses the address of the inner cell as a stable identifier.
    #[inline]
    fn node_id(&self) -> usize {
        Shared::as_ptr(&self.cell) as usize
    }
}

impl PendingTarget {
    fn materialize(&self) -> SchemaNode {
        let validators = self
            .validators
            .upgrade()
            .expect("pending schema target dropped");
        SchemaNode {
            validators,
            location: self.location.clone(),
            absolute_path: self.absolute_path.clone(),
        }
    }
}

impl Validate for PendingSchemaNode {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        if ctx.enter(self.node_id(), instance) {
            // Cycle detected - this (node, instance) pair is already being validated.
            // A pure reference cycle is equivalent to `true` schema.
            return true;
        }
        let result = self.with_node(|node| node.is_valid(instance, ctx));
        ctx.exit(self.node_id(), instance);
        result
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if ctx.enter(self.node_id(), instance) {
            return Ok(());
        }
        let result = self.with_node(|node| node.validate(instance, location, ctx));
        ctx.exit(self.node_id(), instance);
        result
    }

    fn iter_errors<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> ErrorIterator<'i> {
        if ctx.enter(self.node_id(), instance) {
            return crate::error::no_error();
        }
        let result = self.with_node(|node| node.iter_errors(instance, location, ctx));
        ctx.exit(self.node_id(), instance);
        result
    }

    fn evaluate(
        &self,
        instance: &Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> EvaluationResult {
        if ctx.enter(self.node_id(), instance) {
            return EvaluationResult::valid_empty();
        }
        let result = self.with_node(|node| node.evaluate(instance, location, ctx));
        ctx.exit(self.node_id(), instance);
        result
    }
}

impl SchemaNode {
    pub(crate) fn from_boolean(ctx: &Context<'_>, validator: Option<BoxedValidator>) -> SchemaNode {
        SchemaNode {
            location: ctx.location().clone(),
            absolute_path: ctx.base_uri(),
            validators: Shared::new(NodeValidators::Boolean { validator }),
        }
    }

    pub(crate) fn from_keywords(
        ctx: &Context<'_>,
        validators: Vec<(Keyword, BoxedValidator)>,
        unmatched_keywords: Option<Arc<Value>>,
    ) -> SchemaNode {
        let absolute_path = ctx.base_uri();
        let validators = validators
            .into_iter()
            .map(|(keyword, validator)| {
                let location = ctx.location().join(&keyword);
                let absolute_location = ctx.absolute_location(&location);
                KeywordValidatorEntry {
                    validator,
                    location,
                    absolute_location,
                }
            })
            .collect();
        SchemaNode {
            location: ctx.location().clone(),
            absolute_path,
            validators: Shared::new(NodeValidators::Keyword(KeywordValidators {
                unmatched_keywords,
                validators,
            })),
        }
    }

    pub(crate) fn from_array(ctx: &Context<'_>, validators: Vec<BoxedValidator>) -> SchemaNode {
        let absolute_path = ctx.base_uri();
        let validators = validators
            .into_iter()
            .enumerate()
            .map(|(index, validator)| {
                let location = ctx.location().join(index);
                let absolute_location = ctx.absolute_location(&location);
                ArrayValidatorEntry {
                    validator,
                    location,
                    absolute_location,
                }
            })
            .collect();
        SchemaNode {
            location: ctx.location().clone(),
            absolute_path,
            validators: Shared::new(NodeValidators::Array { validators }),
        }
    }

    pub(crate) fn clone_with_location(
        &self,
        location: Location,
        absolute_path: Option<Arc<Uri<String>>>,
    ) -> SchemaNode {
        SchemaNode {
            validators: self.validators.clone(),
            location,
            absolute_path,
        }
    }

    pub(crate) fn validators(&self) -> impl ExactSizeIterator<Item = &BoxedValidator> {
        match self.validators.as_ref() {
            NodeValidators::Boolean { validator } => {
                if let Some(v) = validator {
                    NodeValidatorsIter::BooleanValidators(std::iter::once(v))
                } else {
                    NodeValidatorsIter::NoValidator
                }
            }
            NodeValidators::Keyword(kvals) => {
                NodeValidatorsIter::KeywordValidators(kvals.validators.iter())
            }
            NodeValidators::Array { validators } => {
                NodeValidatorsIter::ArrayValidators(validators.iter())
            }
        }
    }

    pub(crate) fn evaluate_instance(
        &self,
        instance: &Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> EvaluationNode {
        let instance_location: Location = location.into();
        let schema_location = format_schema_location(&self.location, self.absolute_path.as_ref());
        match self.evaluate(instance, location, ctx) {
            EvaluationResult::Valid {
                annotations,
                children,
            } => EvaluationNode::valid(
                self.location.clone(),
                self.absolute_path.clone(),
                schema_location,
                instance_location,
                annotations,
                children,
            ),
            EvaluationResult::Invalid {
                errors,
                children,
                annotations,
            } => EvaluationNode::invalid(
                self.location.clone(),
                self.absolute_path.clone(),
                schema_location,
                instance_location,
                annotations,
                errors,
                children,
            ),
        }
    }

    /// Helper function to evaluate subschemas which already know their locations.
    fn evaluate_subschemas<'a, I>(
        instance: &Value,
        location: &LazyLocation,
        subschemas: I,
        annotations: Option<Annotations>,
        ctx: &mut ValidationContext,
    ) -> EvaluationResult
    where
        I: Iterator<
                Item = (
                    &'a Location,
                    Option<&'a Arc<Uri<String>>>,
                    &'a BoxedValidator,
                ),
            > + 'a,
    {
        let (lower_bound, _) = subschemas.size_hint();
        let mut children: Vec<EvaluationNode> = Vec::with_capacity(lower_bound);
        let mut invalid = false;
        let instance_location: OnceCell<Location> = OnceCell::new();

        for (child_location, absolute_location, validator) in subschemas {
            let child_result = validator.evaluate(instance, location, ctx);

            let schema_location = child_location.clone();
            let absolute_location = absolute_location.cloned();
            let instance_loc = instance_location.get_or_init(|| location.into()).clone();
            let formatted_schema_location =
                format_schema_location(&schema_location, absolute_location.as_ref());

            let child_node = match child_result {
                EvaluationResult::Valid {
                    annotations,
                    children,
                } => EvaluationNode::valid(
                    schema_location,
                    absolute_location,
                    formatted_schema_location,
                    instance_loc,
                    annotations,
                    children,
                ),
                EvaluationResult::Invalid {
                    errors,
                    children,
                    annotations,
                } => {
                    invalid = true;
                    EvaluationNode::invalid(
                        schema_location,
                        absolute_location,
                        formatted_schema_location,
                        instance_loc,
                        annotations,
                        errors,
                        children,
                    )
                }
            };
            children.push(child_node);
        }
        if invalid {
            EvaluationResult::Invalid {
                errors: Vec::new(),
                children,
                annotations,
            }
        } else {
            EvaluationResult::Valid {
                annotations,
                children,
            }
        }
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}

impl Validate for SchemaNode {
    fn is_valid(&self, instance: &Value, ctx: &mut ValidationContext) -> bool {
        match self.validators.as_ref() {
            // If we only have one validator then calling it's `is_valid` directly does
            // actually save the 20 or so instructions required to call the `slice::Iter::all`
            // implementation. Validators at the leaf of a tree are all single node validators so
            // this optimization can have significant cumulative benefits
            NodeValidators::Keyword(kvs) if kvs.validators.len() == 1 => {
                kvs.validators[0].validator.is_valid(instance, ctx)
            }
            NodeValidators::Keyword(kvs) => {
                for entry in &kvs.validators {
                    if !entry.validator.is_valid(instance, ctx) {
                        return false;
                    }
                }
                true
            }
            NodeValidators::Array { validators } => validators
                .iter()
                .all(|entry| entry.validator.is_valid(instance, ctx)),
            NodeValidators::Boolean { validator: Some(_) } => false,
            NodeValidators::Boolean { validator: None } => true,
        }
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        match self.validators.as_ref() {
            NodeValidators::Keyword(kvs) => {
                for entry in &kvs.validators {
                    entry.validator.validate(instance, location, ctx)?;
                }
            }
            NodeValidators::Array { validators } => {
                for entry in validators {
                    entry.validator.validate(instance, location, ctx)?;
                }
            }
            NodeValidators::Boolean { validator: Some(_) } => {
                return Err(ValidationError::false_schema(
                    self.location.clone(),
                    location.into(),
                    instance,
                ));
            }
            NodeValidators::Boolean { validator: None } => return Ok(()),
        }
        Ok(())
    }

    fn iter_errors<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> ErrorIterator<'i> {
        match self.validators.as_ref() {
            NodeValidators::Keyword(kvs) if kvs.validators.len() == 1 => kvs.validators[0]
                .validator
                .iter_errors(instance, location, ctx),
            NodeValidators::Keyword(kvs) => ErrorIterator::from_iterator(
                kvs.validators
                    .iter()
                    .flat_map(|entry| entry.validator.iter_errors(instance, location, ctx))
                    .collect::<Vec<_>>()
                    .into_iter(),
            ),
            NodeValidators::Boolean {
                validator: Some(v), ..
            } => v.iter_errors(instance, location, ctx),
            NodeValidators::Boolean {
                validator: None, ..
            } => ErrorIterator::from_iterator(std::iter::empty()),
            NodeValidators::Array { validators } => ErrorIterator::from_iterator(
                validators
                    .iter()
                    .flat_map(move |entry| entry.validator.iter_errors(instance, location, ctx))
                    .collect::<Vec<_>>()
                    .into_iter(),
            ),
        }
    }

    fn evaluate(
        &self,
        instance: &Value,
        location: &LazyLocation,
        ctx: &mut ValidationContext,
    ) -> EvaluationResult {
        match self.validators.as_ref() {
            NodeValidators::Array { ref validators } => Self::evaluate_subschemas(
                instance,
                location,
                validators.iter().map(|entry| {
                    (
                        &entry.location,
                        entry.absolute_location.as_ref(),
                        &entry.validator,
                    )
                }),
                None,
                ctx,
            ),
            NodeValidators::Boolean { ref validator } => {
                if let Some(validator) = validator {
                    validator.evaluate(instance, location, ctx)
                } else {
                    EvaluationResult::Valid {
                        annotations: None,
                        children: Vec::new(),
                    }
                }
            }
            NodeValidators::Keyword(ref kvals) => {
                let KeywordValidators {
                    ref unmatched_keywords,
                    ref validators,
                } = *kvals;
                let annotations: Option<Annotations> = unmatched_keywords
                    .as_ref()
                    .map(|v| Annotations::from_arc(Arc::clone(v)));
                Self::evaluate_subschemas(
                    instance,
                    location,
                    validators.iter().map(|entry| {
                        (
                            &entry.location,
                            entry.absolute_location.as_ref(),
                            &entry.validator,
                        )
                    }),
                    annotations,
                    ctx,
                )
            }
        }
    }
}

enum NodeValidatorsIter<'a> {
    NoValidator,
    BooleanValidators(std::iter::Once<&'a BoxedValidator>),
    KeywordValidators(std::slice::Iter<'a, KeywordValidatorEntry>),
    ArrayValidators(std::slice::Iter<'a, ArrayValidatorEntry>),
}

impl<'a> Iterator for NodeValidatorsIter<'a> {
    type Item = &'a BoxedValidator;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::NoValidator => None,
            Self::BooleanValidators(i) => i.next(),
            Self::KeywordValidators(v) => v.next().map(|entry| &entry.validator),
            Self::ArrayValidators(v) => v.next().map(|entry| &entry.validator),
        }
    }

    fn all<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        match self {
            Self::NoValidator => true,
            Self::BooleanValidators(i) => i.all(f),
            Self::KeywordValidators(v) => v.all(|entry| f(&entry.validator)),
            Self::ArrayValidators(v) => v.all(|entry| f(&entry.validator)),
        }
    }
}

impl ExactSizeIterator for NodeValidatorsIter<'_> {
    fn len(&self) -> usize {
        match self {
            Self::NoValidator => 0,
            Self::BooleanValidators(..) => 1,
            Self::KeywordValidators(v) => v.len(),
            Self::ArrayValidators(v) => v.len(),
        }
    }
}
