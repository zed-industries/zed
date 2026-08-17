use crate::{
    content_encoding::{ContentEncodingCheckType, ContentEncodingConverterType},
    content_media_type::ContentMediaTypeCheckType,
    ecma,
    keywords::{
        self,
        custom::{CustomKeyword, KeywordFactory},
        format::Format,
        unevaluated_items::PendingItemsValidators,
        unevaluated_properties::PendingPropertyValidators,
        BoxedValidator, BuiltinKeyword, Keyword,
    },
    node::{PendingSchemaNode, SchemaNode},
    options::{PatternEngineOptions, ValidationOptions},
    paths::{Location, LocationSegment},
    types::{JsonType, JsonTypeSet},
    validator::Validate,
    ValidationError, Validator,
};
use ahash::{AHashMap, AHashSet};
use referencing::{
    uri, Draft, List, Registry, Resolved, Resolver, Resource, ResourceRef, Uri, Vocabulary,
    VocabularySet,
};
use serde_json::{Map, Value};
use std::{borrow::Cow, cell::RefCell, iter::once, rc::Rc, sync::Arc};

const DEFAULT_SCHEME: &str = "json-schema";
pub(crate) const DEFAULT_BASE_URI: &str = "json-schema:///";

/// Type alias for shared cache maps in compiler state.
type SharedCache<K, V> = Rc<RefCell<AHashMap<K, V>>>;
/// Type alias for shared sets in compiler state.
type SharedSet<T> = Rc<RefCell<AHashSet<T>>>;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub(crate) struct LocationCacheKey {
    pub(crate) base_uri: Arc<Uri<String>>,
    location: Arc<str>,
    dynamic_scope: List<Uri<String>>,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct PropertyValidatorsPendingKey {
    schema_ptr: usize,
}

impl PropertyValidatorsPendingKey {
    fn new(schema: &Map<String, Value>) -> Self {
        Self {
            schema_ptr: std::ptr::from_ref(schema) as usize,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct ItemsValidatorsPendingKey {
    schema_ptr: usize,
}

impl ItemsValidatorsPendingKey {
    fn new(schema: &Map<String, Value>) -> Self {
        Self {
            schema_ptr: std::ptr::from_ref(schema) as usize,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub(crate) struct AliasCacheKey {
    uri: Arc<Uri<String>>,
    dynamic_scope: List<Uri<String>>,
}

/// Shared caches reused across every `Context` derived from a schema root.
#[derive(Debug, Clone)]
struct SharedContextState {
    seen: SharedSet<Arc<Uri<String>>>,
    location_nodes: SharedCache<LocationCacheKey, SchemaNode>,
    alias_nodes: SharedCache<AliasCacheKey, SchemaNode>,
    pending_nodes: SharedCache<LocationCacheKey, PendingSchemaNode>,
    alias_placeholders: SharedCache<Arc<Uri<String>>, PendingSchemaNode>,
    pending_property_validators: SharedCache<LocationCacheKey, PendingPropertyValidators>,
    pending_property_validators_by_schema:
        SharedCache<PropertyValidatorsPendingKey, PendingPropertyValidators>,
    pending_items_validators: SharedCache<LocationCacheKey, PendingItemsValidators>,
    pending_items_validators_by_schema:
        SharedCache<ItemsValidatorsPendingKey, PendingItemsValidators>,
    pattern_cache: SharedCache<Arc<str>, PatternCacheEntry>,
    uri_buffer: Rc<RefCell<String>>,
}

#[derive(Debug, Clone)]
struct PatternCacheEntry {
    translated: Arc<str>,
    fancy: Option<Arc<fancy_regex::Regex>>,
    standard: Option<Arc<regex::Regex>>,
}

impl SharedContextState {
    fn new() -> Self {
        Self {
            seen: Rc::new(RefCell::new(AHashSet::new())),
            location_nodes: Rc::new(RefCell::new(AHashMap::new())),
            alias_nodes: Rc::new(RefCell::new(AHashMap::new())),
            pending_nodes: Rc::new(RefCell::new(AHashMap::new())),
            alias_placeholders: Rc::new(RefCell::new(AHashMap::new())),
            pending_property_validators: Rc::new(RefCell::new(AHashMap::new())),
            pending_property_validators_by_schema: Rc::new(RefCell::new(AHashMap::new())),
            pending_items_validators: Rc::new(RefCell::new(AHashMap::new())),
            pending_items_validators_by_schema: Rc::new(RefCell::new(AHashMap::new())),
            pattern_cache: Rc::new(RefCell::new(AHashMap::new())),
            uri_buffer: Rc::new(RefCell::new(String::new())),
        }
    }
}

/// Per-location view used while compiling schemas into validators.
#[derive(Debug, Clone)]
pub(crate) struct Context<'a> {
    config: &'a ValidationOptions,
    pub(crate) registry: &'a Registry,
    resolver: Resolver<'a>,
    vocabularies: VocabularySet,
    location: Location,
    pub(crate) draft: Draft,
    shared: SharedContextState,
}

impl<'a> Context<'a> {
    pub(crate) fn new(
        config: &'a ValidationOptions,
        registry: &'a Registry,
        resolver: Resolver<'a>,
        vocabularies: VocabularySet,
        draft: Draft,
        location: Location,
    ) -> Self {
        Context {
            config,
            registry,
            resolver,
            location,
            vocabularies,
            draft,
            shared: SharedContextState::new(),
        }
    }
    pub(crate) fn draft(&self) -> Draft {
        self.draft
    }
    pub(crate) fn config(&self) -> &ValidationOptions {
        self.config
    }

    /// Create a context for this schema.
    pub(crate) fn in_subresource(
        &'a self,
        resource: ResourceRef<'_>,
    ) -> Result<Context<'a>, referencing::Error> {
        let resolver = self.resolver.in_subresource(resource)?;
        Ok(Context {
            config: self.config,
            registry: self.registry,
            resolver,
            vocabularies: self.vocabularies.clone(),
            draft: resource.draft(),
            location: self.location.clone(),
            shared: self.shared.clone(),
        })
    }
    pub(crate) fn as_resource_ref<'r>(&'a self, contents: &'r Value) -> ResourceRef<'r> {
        self.draft.detect(contents).create_resource_ref(contents)
    }

    #[inline]
    pub(crate) fn new_at_location(&'a self, chunk: impl Into<LocationSegment<'a>>) -> Self {
        let location = self.location.join(chunk);
        Context {
            config: self.config,
            registry: self.registry,
            resolver: self.resolver.clone(),
            vocabularies: self.vocabularies.clone(),
            location,
            draft: self.draft,
            shared: self.shared.clone(),
        }
    }
    pub(crate) fn lookup(&'a self, reference: &str) -> Result<Resolved<'a>, referencing::Error> {
        self.resolver.lookup(reference)
    }

    pub(crate) fn location_cache_key(&self) -> LocationCacheKey {
        LocationCacheKey {
            base_uri: self.resolver.base_uri(),
            location: self.location.as_arc(),
            dynamic_scope: self.resolver.dynamic_scope(),
        }
    }

    fn alias_cache_key(&self, alias: Arc<Uri<String>>) -> AliasCacheKey {
        AliasCacheKey {
            uri: alias,
            dynamic_scope: self.resolver.dynamic_scope(),
        }
    }

    pub(crate) fn base_uri(&self) -> Option<Arc<Uri<String>>> {
        let base_uri = self.resolver.base_uri();
        if base_uri.scheme().as_str() == DEFAULT_SCHEME {
            None
        } else {
            Some(base_uri)
        }
    }

    pub(crate) fn absolute_location(&self, location: &Location) -> Option<Arc<Uri<String>>> {
        let base = self.base_uri()?;
        let mut buffer = self.shared.uri_buffer.borrow_mut();
        buffer.clear();
        uri::encode_to(location.as_str(), &mut buffer);
        let resolved = base.with_fragment(Some(uri::EncodedString::new_or_panic(&buffer)));
        buffer.clear();
        Some(Arc::new(resolved))
    }

    fn translated_pattern(&self, pattern: &str) -> Result<Arc<str>, ()> {
        if let Some(entry) = self.shared.pattern_cache.borrow().get(pattern) {
            return Ok(Arc::clone(&entry.translated));
        }
        let translated = Arc::<str>::from(ecma::to_rust_regex(pattern)?);
        self.shared.pattern_cache.borrow_mut().insert(
            Arc::from(pattern),
            PatternCacheEntry {
                translated: Arc::clone(&translated),
                fancy: None,
                standard: None,
            },
        );
        Ok(translated)
    }

    fn is_known_keyword(&self, keyword: &str) -> bool {
        self.draft.is_known_keyword(keyword)
    }
    pub(crate) fn supports_adjacent_validation(&self) -> bool {
        !matches!(self.draft, Draft::Draft4 | Draft::Draft6 | Draft::Draft7)
    }
    pub(crate) fn supports_integer_valued_numbers(&self) -> bool {
        !matches!(self.draft, Draft::Draft4)
    }
    pub(crate) fn validates_formats_by_default(&self) -> bool {
        self.config.validate_formats().unwrap_or(matches!(
            self.draft,
            Draft::Draft4 | Draft::Draft6 | Draft::Draft7
        ))
    }
    pub(crate) fn are_unknown_formats_ignored(&self) -> bool {
        self.config.are_unknown_formats_ignored()
    }
    pub(crate) fn with_resolver_and_draft(
        &'a self,
        resolver: Resolver<'a>,
        draft: Draft,
        vocabularies: VocabularySet,
        location: Location,
    ) -> Context<'a> {
        Context {
            config: self.config,
            registry: self.registry,
            resolver,
            draft,
            vocabularies,
            location,
            shared: self.shared.clone(),
        }
    }
    pub(crate) fn get_content_media_type_check(
        &self,
        media_type: &str,
    ) -> Option<ContentMediaTypeCheckType> {
        self.config.get_content_media_type_check(media_type)
    }
    pub(crate) fn get_content_encoding_check(
        &self,
        content_encoding: &str,
    ) -> Option<ContentEncodingCheckType> {
        self.config.content_encoding_check(content_encoding)
    }

    pub(crate) fn get_content_encoding_convert(
        &self,
        content_encoding: &str,
    ) -> Option<ContentEncodingConverterType> {
        self.config.get_content_encoding_convert(content_encoding)
    }
    pub(crate) fn get_keyword_factory(&self, name: &str) -> Option<&Arc<dyn KeywordFactory>> {
        self.config.get_keyword_factory(name)
    }
    pub(crate) fn get_format(&self, format: &str) -> Option<(&String, &Arc<dyn Format>)> {
        self.config.get_format(format)
    }
    pub(crate) fn is_circular_reference(
        &self,
        reference: &str,
    ) -> Result<bool, referencing::Error> {
        let uri = self
            .resolver
            .resolve_against(&self.resolver.base_uri().borrow(), reference)?;
        Ok(self.shared.seen.borrow().contains(&*uri))
    }
    pub(crate) fn mark_seen(&self, reference: &str) -> Result<(), referencing::Error> {
        let uri = self
            .resolver
            .resolve_against(&self.resolver.base_uri().borrow(), reference)?;
        self.shared.seen.borrow_mut().insert(uri);
        Ok(())
    }

    pub(crate) fn lookup_recursive_reference(&self) -> Result<Resolved<'_>, referencing::Error> {
        self.resolver.lookup_recursive_ref()
    }
    pub(crate) fn absolute_location_uri(&self) -> Result<Arc<Uri<String>>, referencing::Error> {
        // Reuse the shared buffer to avoid allocations
        let mut buffer = self.shared.uri_buffer.borrow_mut();
        buffer.clear();
        buffer.push('#');
        if !self.location.as_str().is_empty() {
            uri::encode_to(self.location.as_str(), &mut buffer);
        }
        let result = self
            .resolver
            .resolve_against(&self.resolver.base_uri().borrow(), &buffer);
        buffer.clear();
        result
    }

    pub(crate) fn resolve_reference_uri(
        &self,
        reference: &str,
    ) -> Result<Arc<Uri<String>>, referencing::Error> {
        self.resolver
            .resolve_against(&self.resolver.base_uri().borrow(), reference)
    }

    pub(crate) fn cached_location_node(&self, key: &LocationCacheKey) -> Option<SchemaNode> {
        self.shared.location_nodes.borrow().get(key).cloned()
    }

    pub(crate) fn cache_location_node(&self, key: LocationCacheKey, node: SchemaNode) {
        self.shared.location_nodes.borrow_mut().insert(key, node);
    }

    pub(crate) fn cached_alias_node(&self, key: &AliasCacheKey) -> Option<SchemaNode> {
        self.shared.alias_nodes.borrow().get(key).cloned()
    }

    pub(crate) fn cache_alias_node(&self, key: AliasCacheKey, node: SchemaNode) {
        self.shared.alias_nodes.borrow_mut().insert(key, node);
    }

    pub(crate) fn cached_pending_location_node(
        &self,
        key: &LocationCacheKey,
    ) -> Option<PendingSchemaNode> {
        self.shared.pending_nodes.borrow().get(key).cloned()
    }

    pub(crate) fn cache_pending_location_node(
        &self,
        key: LocationCacheKey,
        node: PendingSchemaNode,
    ) {
        self.shared.pending_nodes.borrow_mut().insert(key, node);
    }

    pub(crate) fn remove_pending_location_node(&self, key: &LocationCacheKey) {
        self.shared.pending_nodes.borrow_mut().remove(key);
    }

    pub(crate) fn get_pending_property_validators(
        &self,
        key: &LocationCacheKey,
    ) -> Option<PendingPropertyValidators> {
        self.shared
            .pending_property_validators
            .borrow()
            .get(key)
            .cloned()
    }

    pub(crate) fn cache_pending_property_validators(
        &self,
        key: LocationCacheKey,
        pending: PendingPropertyValidators,
    ) {
        self.shared
            .pending_property_validators
            .borrow_mut()
            .insert(key, pending);
    }

    pub(crate) fn remove_pending_property_validators(&self, key: &LocationCacheKey) {
        self.shared
            .pending_property_validators
            .borrow_mut()
            .remove(key);
    }

    fn property_schema_key(schema: &Map<String, Value>) -> PropertyValidatorsPendingKey {
        PropertyValidatorsPendingKey::new(schema)
    }

    pub(crate) fn get_pending_property_validators_for_schema(
        &self,
        schema: &Map<String, Value>,
    ) -> Option<PendingPropertyValidators> {
        let key = Self::property_schema_key(schema);
        self.shared
            .pending_property_validators_by_schema
            .borrow()
            .get(&key)
            .cloned()
    }

    pub(crate) fn cache_pending_property_validators_for_schema(
        &self,
        schema: &Map<String, Value>,
        pending: PendingPropertyValidators,
    ) {
        let key = Self::property_schema_key(schema);
        self.shared
            .pending_property_validators_by_schema
            .borrow_mut()
            .insert(key, pending);
    }

    pub(crate) fn remove_pending_property_validators_for_schema(
        &self,
        schema: &Map<String, Value>,
    ) {
        let key = Self::property_schema_key(schema);
        self.shared
            .pending_property_validators_by_schema
            .borrow_mut()
            .remove(&key);
    }

    fn items_schema_key(schema: &Map<String, Value>) -> ItemsValidatorsPendingKey {
        ItemsValidatorsPendingKey::new(schema)
    }

    pub(crate) fn get_pending_items_validators_for_schema(
        &self,
        schema: &Map<String, Value>,
    ) -> Option<PendingItemsValidators> {
        let key = Self::items_schema_key(schema);
        self.shared
            .pending_items_validators_by_schema
            .borrow()
            .get(&key)
            .cloned()
    }

    pub(crate) fn get_pending_items_validators(
        &self,
        key: &LocationCacheKey,
    ) -> Option<PendingItemsValidators> {
        self.shared
            .pending_items_validators
            .borrow()
            .get(key)
            .cloned()
    }

    pub(crate) fn cached_alias_placeholder(
        &self,
        alias: &Arc<Uri<String>>,
    ) -> Option<PendingSchemaNode> {
        self.shared.alias_placeholders.borrow().get(alias).cloned()
    }

    pub(crate) fn set_alias_placeholder(&self, alias: Arc<Uri<String>>, node: PendingSchemaNode) {
        self.shared
            .alias_placeholders
            .borrow_mut()
            .insert(alias, node);
    }

    pub(crate) fn remove_alias_placeholder(&self, alias: &Arc<Uri<String>>) {
        self.shared.alias_placeholders.borrow_mut().remove(alias);
    }

    /// Get a cached compiled regex, or compile and cache it if not present.
    pub(crate) fn get_or_compile_regex(
        &self,
        pattern: &str,
    ) -> Result<Arc<fancy_regex::Regex>, ()> {
        let translated = self.translated_pattern(pattern)?;
        {
            let cache = self.shared.pattern_cache.borrow();
            if let Some(entry) = cache.get(pattern) {
                if let Some(regex) = &entry.fancy {
                    return Ok(Arc::clone(regex));
                }
            }
        }

        let (backtrack_limit, size_limit, dfa_size_limit) = match self.config.pattern_options() {
            PatternEngineOptions::FancyRegex {
                backtrack_limit,
                size_limit,
                dfa_size_limit,
            } => (backtrack_limit, size_limit, dfa_size_limit),
            PatternEngineOptions::Regex { .. } => (None, None, None),
        };

        let mut builder = fancy_regex::RegexBuilder::new(translated.as_ref());
        if let Some(limit) = backtrack_limit {
            builder.backtrack_limit(limit);
        }
        if let Some(limit) = size_limit {
            builder.delegate_size_limit(limit);
        }
        if let Some(limit) = dfa_size_limit {
            builder.delegate_dfa_size_limit(limit);
        }
        let regex = Arc::new(builder.build().map_err(|_| ())?);

        if let Some(entry) = self.shared.pattern_cache.borrow_mut().get_mut(pattern) {
            entry.fancy = Some(Arc::clone(&regex));
        }

        Ok(regex)
    }

    /// Get a cached compiled standard regex, or compile and cache it if not present.
    pub(crate) fn get_or_compile_standard_regex(
        &self,
        pattern: &str,
    ) -> Result<Arc<regex::Regex>, ()> {
        let translated = self.translated_pattern(pattern)?;
        {
            let cache = self.shared.pattern_cache.borrow();
            if let Some(entry) = cache.get(pattern) {
                if let Some(regex) = &entry.standard {
                    return Ok(Arc::clone(regex));
                }
            }
        }

        let (size_limit, dfa_size_limit) = match self.config.pattern_options() {
            PatternEngineOptions::Regex {
                size_limit,
                dfa_size_limit,
            } => (size_limit, dfa_size_limit),
            PatternEngineOptions::FancyRegex { .. } => (None, None),
        };

        let mut builder = regex::RegexBuilder::new(translated.as_ref());
        if let Some(limit) = size_limit {
            builder.size_limit(limit);
        }
        if let Some(limit) = dfa_size_limit {
            builder.dfa_size_limit(limit);
        }
        let regex = Arc::new(builder.build().map_err(|_| ())?);

        if let Some(entry) = self.shared.pattern_cache.borrow_mut().get_mut(pattern) {
            entry.standard = Some(Arc::clone(&regex));
        }

        Ok(regex)
    }

    /// Lookup a reference that is potentially recursive and return already
    /// compiled nodes when available.
    pub(crate) fn lookup_maybe_recursive(
        &self,
        reference: &str,
    ) -> Result<Option<Box<dyn Validate>>, ValidationError<'static>> {
        if self.is_circular_reference(reference)? {
            let uri = self
                .resolve_reference_uri(reference)
                .map_err(ValidationError::from)?;
            let key = self.alias_cache_key(Arc::clone(&uri));
            if let Some(node) = self.cached_alias_node(&key) {
                return Ok(Some(Box::new(node)));
            }
            if let Some(node) = self.cached_alias_placeholder(&uri) {
                return Ok(Some(Box::new(node)));
            }
        }
        Ok(None)
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }

    pub(crate) fn has_vocabulary(&self, vocabulary: &Vocabulary) -> bool {
        if self.draft() < Draft::Draft201909 || vocabulary == &Vocabulary::Core {
            true
        } else {
            self.vocabularies.contains(vocabulary)
        }
    }
}

pub(crate) fn build_validator(
    config: &ValidationOptions,
    schema: &Value,
) -> Result<Validator, ValidationError<'static>> {
    let draft = config.draft_for(schema)?;
    let resource_ref = draft.create_resource_ref(schema);
    let resource = draft.create_resource(schema.clone());
    let base_uri = if let Some(base_uri) = config.base_uri.as_ref() {
        uri::from_str(base_uri)?
    } else {
        uri::from_str(resource_ref.id().unwrap_or(DEFAULT_BASE_URI))?
    };

    // Build a registry & resolver needed for validator compilation
    // Clone resources to drain them without mutating the original config
    let pairs = collect_resource_pairs(base_uri.as_str(), resource, config.resources.clone());

    let registry = if let Some(ref registry) = config.registry {
        Arc::new(registry.clone().try_with_resources_and_retriever(
            pairs,
            &*config.retriever,
            draft,
        )?)
    } else {
        Arc::new(
            Registry::options()
                .draft(draft)
                .retriever(Arc::clone(&config.retriever))
                .build(pairs)?,
        )
    };
    let vocabularies = registry.find_vocabularies(draft, schema);
    let resolver = registry.resolver(base_uri);

    let ctx = Context::new(
        config,
        &registry,
        resolver,
        vocabularies,
        draft,
        Location::new(),
    );

    // Validate the schema itself
    if config.validate_schema {
        validate_schema(draft, schema)?;
    }

    // Finally, compile the validator
    let root = compile(&ctx, resource_ref).map_err(ValidationError::to_owned)?;
    let draft = config.draft();
    Ok(Validator { root, draft })
}

#[cfg(feature = "resolve-async")]
pub(crate) async fn build_validator_async(
    config: &ValidationOptions<Arc<dyn referencing::AsyncRetrieve>>,
    schema: &Value,
) -> Result<Validator, ValidationError<'static>> {
    let draft = config.draft_for(schema).await?;
    let resource_ref = draft.create_resource_ref(schema);
    let resource = draft.create_resource(schema.clone());
    let base_uri = if let Some(base_uri) = config.base_uri.as_ref() {
        uri::from_str(base_uri)?
    } else {
        uri::from_str(resource_ref.id().unwrap_or(DEFAULT_BASE_URI))?
    };

    // Clone resources to drain them without mutating the original config
    let pairs = collect_resource_pairs(base_uri.as_str(), resource, config.resources.clone());

    let registry = if let Some(ref registry) = config.registry {
        Arc::new(
            registry
                .clone()
                .try_with_resources_and_retriever_async(pairs, &*config.retriever, draft)
                .await?,
        )
    } else {
        Arc::new(
            Registry::options()
                .async_retriever(Arc::clone(&config.retriever))
                .draft(draft)
                .build(pairs)
                .await?,
        )
    };

    let vocabularies = registry.find_vocabularies(draft, schema);
    let resolver = registry.resolver(base_uri);
    // HACK: `ValidationOptions` struct has a default type parameter as `Arc<dyn Retrieve>` and to
    //       avoid propagating types everywhere in `Context`, it is easier to just replace the
    //       retriever to one that implements `Retrieve`, as it is not used anymore anyway.
    let config_with_blocking_retriever = config
        .clone()
        .with_blocking_retriever(crate::retriever::DefaultRetriever);
    let ctx = Context::new(
        &config_with_blocking_retriever,
        &registry,
        resolver,
        vocabularies,
        draft,
        Location::new(),
    );

    if config.validate_schema {
        validate_schema(draft, schema)?;
    }

    let root = compile(&ctx, resource_ref).map_err(ValidationError::to_owned)?;
    let draft = config.draft();
    Ok(Validator { root, draft })
}

fn annotations_to_value(annotations: AHashMap<String, Value>) -> Arc<Value> {
    let mut object = Map::with_capacity(annotations.len());
    for (key, value) in annotations {
        object.insert(key, value);
    }
    Arc::new(Value::Object(object))
}

fn collect_resource_pairs(
    base_uri: &str,
    resource: Resource,
    resources: AHashMap<String, Resource>,
) -> impl IntoIterator<Item = (Cow<'_, str>, Resource)> {
    once((Cow::Borrowed(base_uri), resource)).chain(
        resources
            .into_iter()
            .map(|(uri, resource)| (Cow::Owned(uri), resource)),
    )
}

fn validate_schema(draft: Draft, schema: &Value) -> Result<(), ValidationError<'static>> {
    // Boolean schemas are always valid per the spec, skip validation
    if schema.is_boolean() {
        return Ok(());
    }

    // For objects, we can skip validation if they're empty (always valid)
    if let Some(obj) = schema.as_object() {
        if obj.is_empty() {
            return Ok(());
        }
    }

    let validator = crate::meta::validator_for_draft(draft);
    if let Err(error) = validator.validate(schema) {
        return Err(error.to_owned());
    }
    Ok(())
}

/// Compile a JSON Schema instance to a tree of nodes.
pub(crate) fn compile<'a>(
    ctx: &Context,
    resource: ResourceRef<'a>,
) -> Result<SchemaNode, ValidationError<'a>> {
    let ctx = ctx.in_subresource(resource)?;
    compile_with_internal(&ctx, resource, None)
}

pub(crate) fn compile_with_alias<'a>(
    ctx: &Context,
    resource: ResourceRef<'a>,
    alias: Arc<Uri<String>>,
) -> Result<SchemaNode, ValidationError<'a>> {
    compile_with_internal(ctx, resource, Some(alias))
}

#[allow(clippy::needless_pass_by_value)]
fn compile_with_internal<'a>(
    ctx: &Context,
    resource: ResourceRef<'a>,
    alias: Option<Arc<Uri<String>>>,
) -> Result<SchemaNode, ValidationError<'a>> {
    // Check if this alias already has a cached node
    if let Some(alias_key) = alias.as_ref() {
        let scoped_key = ctx.alias_cache_key(Arc::clone(alias_key));
        if let Some(existing_alias) = ctx.cached_alias_node(&scoped_key) {
            return Ok(existing_alias);
        }
    }

    // Check location-based cache
    let key = ctx.location_cache_key();
    if let Some(existing) = ctx.cached_location_node(&key) {
        return Ok(existing);
    }

    // Check if there's a pending node (circular reference being compiled)
    if let Some(pending) = ctx.cached_pending_location_node(&key) {
        // If the node has already been initialized, reuse it. Otherwise, we rely on the
        // in-flight compilation to finish initialization and continue compiling here.
        if let Some(node) = pending.get() {
            return Ok(node);
        }
    }

    // Create placeholder for circular reference detection
    let placeholder = PendingSchemaNode::new();
    ctx.cache_pending_location_node(key.clone(), placeholder.clone());
    if let Some(alias_key) = alias.as_ref() {
        ctx.set_alias_placeholder(Arc::clone(alias_key), placeholder.clone());
    }

    // Compile the schema
    match compile_without_cache(ctx, resource) {
        Ok(node) => {
            // Initialize the placeholder with the compiled node
            placeholder.initialize(&node);

            // Remove from pending cache and add to final cache
            ctx.remove_pending_location_node(&key);
            ctx.cache_location_node(key.clone(), node.clone());

            if let Some(alias_key) = alias.as_ref() {
                ctx.remove_alias_placeholder(alias_key);
                let scoped_key = ctx.alias_cache_key(Arc::clone(alias_key));
                ctx.cache_alias_node(scoped_key, node.clone());
            }
            Ok(node)
        }
        Err(err) => Err(err),
    }
}

fn compile_without_cache<'a>(
    ctx: &Context,
    resource: ResourceRef<'a>,
) -> Result<SchemaNode, ValidationError<'a>> {
    match resource.contents() {
        Value::Bool(value) => match value {
            true => Ok(SchemaNode::from_boolean(ctx, None)),
            false => Ok(SchemaNode::from_boolean(
                ctx,
                Some(
                    keywords::boolean::FalseValidator::compile(ctx.location().clone())
                        .expect("Should always compile"),
                ),
            )),
        },
        Value::Object(schema) => {
            // A schema could contain validation keywords along with annotations and we need to
            // collect annotations separately
            if !ctx.supports_adjacent_validation() {
                // Older drafts ignore all other keywords if `$ref` is present
                if let Some(reference) = schema.get("$ref") {
                    // Treat all keywords other than `$ref` as annotations
                    let annotations: AHashMap<String, Value> = schema
                        .iter()
                        .filter_map(|(k, v)| {
                            if k.as_str() == "$ref" {
                                None
                            } else {
                                Some((k.clone(), v.clone()))
                            }
                        })
                        .collect();
                    let annotations = if annotations.is_empty() {
                        None
                    } else {
                        Some(annotations_to_value(annotations))
                    };
                    return if let Some(validator) =
                        keywords::ref_::compile_ref(ctx, schema, reference)
                    {
                        let validators = vec![(BuiltinKeyword::Ref.into(), validator?)];
                        Ok(SchemaNode::from_keywords(ctx, validators, annotations))
                    } else {
                        // Infinite reference to the same location
                        Ok(SchemaNode::from_boolean(ctx, None))
                    };
                }
            }

            let mut validators = Vec::with_capacity(schema.len());
            let mut annotations = AHashMap::new();
            for (keyword, value) in schema {
                // Check if this keyword is overridden, then check the standard definitions
                if let Some(factory) = ctx.get_keyword_factory(keyword) {
                    let path = ctx.location().join(keyword);
                    let validator = CustomKeyword::new(factory.init(schema, value, path)?);
                    let validator: BoxedValidator = Box::new(validator);
                    validators.push((Keyword::custom(keyword), validator));
                } else if let Some((keyword, validator)) = keywords::get_for_draft(ctx, keyword)
                    .and_then(|(keyword, f)| f(ctx, schema, value).map(|v| (keyword, v)))
                {
                    validators.push((keyword, validator.map_err(ValidationError::to_owned)?));
                } else if !ctx.is_known_keyword(keyword) {
                    // Treat all non-validation keywords as annotations
                    annotations.insert(keyword.clone(), value.clone());
                }
            }
            let annotations = if annotations.is_empty() {
                None
            } else {
                Some(annotations_to_value(annotations))
            };
            Ok(SchemaNode::from_keywords(ctx, validators, annotations))
        }
        _ => Err(ValidationError::multiple_type_error(
            Location::new(),
            ctx.location().clone(),
            resource.contents(),
            JsonTypeSet::empty()
                .insert(JsonType::Boolean)
                .insert(JsonType::Object),
        )),
    }
}
