use std::{
    collections::{hash_map::Entry, VecDeque},
    num::NonZeroUsize,
    pin::Pin,
    sync::{Arc, LazyLock},
};

use ahash::{AHashMap, AHashSet};
use fluent_uri::{pct_enc::EStr, Uri};
use serde_json::Value;

use crate::{
    anchors::{AnchorKey, AnchorKeyRef},
    cache::{SharedUriCache, UriCache},
    meta::{self, metas_for_draft},
    resource::{unescape_segment, InnerResourcePtr, JsonSchemaResource},
    uri,
    vocabularies::{self, VocabularySet},
    Anchor, DefaultRetriever, Draft, Error, Resolver, Resource, ResourceRef, Retrieve,
};

/// An owned-or-refstatic wrapper for JSON `Value`.
#[derive(Debug)]
pub(crate) enum ValueWrapper {
    Owned(Value),
    StaticRef(&'static Value),
}

impl AsRef<Value> for ValueWrapper {
    fn as_ref(&self) -> &Value {
        match self {
            ValueWrapper::Owned(value) => value,
            ValueWrapper::StaticRef(value) => value,
        }
    }
}

// SAFETY: `Pin` guarantees stable memory locations for resource pointers,
// while `Arc` enables cheap sharing between multiple registries
type DocumentStore = AHashMap<Arc<Uri<String>>, Pin<Arc<ValueWrapper>>>;
type ResourceMap = AHashMap<Arc<Uri<String>>, InnerResourcePtr>;

/// Pre-loaded registry containing all JSON Schema meta-schemas and their vocabularies
pub static SPECIFICATIONS: LazyLock<Registry> =
    LazyLock::new(|| Registry::build_from_meta_schemas(meta::META_SCHEMAS_ALL.as_slice()));

/// A registry of JSON Schema resources, each identified by their canonical URIs.
///
/// Registries store a collection of in-memory resources and their anchors.
/// They eagerly process all added resources, including their subresources and anchors.
/// This means that subresources contained within any added resources are immediately
/// discoverable and retrievable via their own IDs.
///
/// # Resource Retrieval
///
/// Registry supports both blocking and non-blocking retrieval of external resources.
///
/// ## Blocking Retrieval
///
/// ```rust
/// use referencing::{Registry, Resource, Retrieve, Uri};
/// use serde_json::{json, Value};
///
/// struct ExampleRetriever;
///
/// impl Retrieve for ExampleRetriever {
///     fn retrieve(
///         &self,
///         uri: &Uri<String>
///     ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
///         // Always return the same value for brevity
///         Ok(json!({"type": "string"}))
///     }
/// }
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let registry = Registry::options()
///     .retriever(ExampleRetriever)
///     .build([
///         // Initial schema that might reference external schemas
///         (
///             "https://example.com/user.json",
///             Resource::from_contents(json!({
///                 "type": "object",
///                 "properties": {
///                     // Should be retrieved by `ExampleRetriever`
///                     "role": {"$ref": "https://example.com/role.json"}
///                 }
///             }))
///         )
///     ])?;
/// # Ok(())
/// # }
/// ```
///
/// ## Non-blocking Retrieval
///
/// ```rust
/// # #[cfg(feature = "retrieve-async")]
/// # mod example {
/// use referencing::{Registry, Resource, AsyncRetrieve, Uri};
/// use serde_json::{json, Value};
///
/// struct ExampleRetriever;
///
/// #[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
/// #[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
/// impl AsyncRetrieve for ExampleRetriever {
///     async fn retrieve(
///         &self,
///         uri: &Uri<String>
///     ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
///         // Always return the same value for brevity
///         Ok(json!({"type": "string"}))
///     }
/// }
///
///  # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let registry = Registry::options()
///     .async_retriever(ExampleRetriever)
///     .build([
///         (
///             "https://example.com/user.json",
///             Resource::from_contents(json!({
///                 // Should be retrieved by `ExampleRetriever`
///                 "$ref": "https://example.com/common/user.json"
///             }))
///         )
///     ])
///     .await?;
/// # Ok(())
/// # }
/// # }
/// ```
///
/// The registry will automatically:
///
/// - Resolve external references
/// - Cache retrieved schemas
/// - Handle nested references
/// - Process JSON Schema anchors
///
#[derive(Debug)]
pub struct Registry {
    documents: DocumentStore,
    pub(crate) resources: ResourceMap,
    anchors: AHashMap<AnchorKey, Anchor>,
    resolution_cache: SharedUriCache,
}

impl Clone for Registry {
    fn clone(&self) -> Self {
        Self {
            documents: self.documents.clone(),
            resources: self.resources.clone(),
            anchors: self.anchors.clone(),
            resolution_cache: self.resolution_cache.clone(),
        }
    }
}

/// Configuration options for creating a [`Registry`].
pub struct RegistryOptions<R> {
    retriever: R,
    draft: Draft,
}

impl<R> RegistryOptions<R> {
    /// Set specification version under which the resources should be interpreted under.
    #[must_use]
    pub fn draft(mut self, draft: Draft) -> Self {
        self.draft = draft;
        self
    }
}

impl RegistryOptions<Arc<dyn Retrieve>> {
    /// Create a new [`RegistryOptions`] with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            retriever: Arc::new(DefaultRetriever),
            draft: Draft::default(),
        }
    }
    /// Set a custom retriever for the [`Registry`].
    #[must_use]
    pub fn retriever(mut self, retriever: impl IntoRetriever) -> Self {
        self.retriever = retriever.into_retriever();
        self
    }
    /// Set a custom async retriever for the [`Registry`].
    #[cfg(feature = "retrieve-async")]
    #[must_use]
    pub fn async_retriever(
        self,
        retriever: impl IntoAsyncRetriever,
    ) -> RegistryOptions<Arc<dyn crate::AsyncRetrieve>> {
        RegistryOptions {
            retriever: retriever.into_retriever(),
            draft: self.draft,
        }
    }
    /// Create a [`Registry`] from multiple resources using these options.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any URI is invalid
    /// - Any referenced resources cannot be retrieved
    pub fn build(
        self,
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
    ) -> Result<Registry, Error> {
        Registry::try_from_resources_impl(pairs, &*self.retriever, self.draft)
    }
}

#[cfg(feature = "retrieve-async")]
impl RegistryOptions<Arc<dyn crate::AsyncRetrieve>> {
    /// Create a [`Registry`] from multiple resources using these options with async retrieval.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any URI is invalid
    /// - Any referenced resources cannot be retrieved
    pub async fn build(
        self,
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
    ) -> Result<Registry, Error> {
        Registry::try_from_resources_async_impl(pairs, &*self.retriever, self.draft).await
    }
}

pub trait IntoRetriever {
    fn into_retriever(self) -> Arc<dyn Retrieve>;
}

impl<T: Retrieve + 'static> IntoRetriever for T {
    fn into_retriever(self) -> Arc<dyn Retrieve> {
        Arc::new(self)
    }
}

impl IntoRetriever for Arc<dyn Retrieve> {
    fn into_retriever(self) -> Arc<dyn Retrieve> {
        self
    }
}

#[cfg(feature = "retrieve-async")]
pub trait IntoAsyncRetriever {
    fn into_retriever(self) -> Arc<dyn crate::AsyncRetrieve>;
}

#[cfg(feature = "retrieve-async")]
impl<T: crate::AsyncRetrieve + 'static> IntoAsyncRetriever for T {
    fn into_retriever(self) -> Arc<dyn crate::AsyncRetrieve> {
        Arc::new(self)
    }
}

#[cfg(feature = "retrieve-async")]
impl IntoAsyncRetriever for Arc<dyn crate::AsyncRetrieve> {
    fn into_retriever(self) -> Arc<dyn crate::AsyncRetrieve> {
        self
    }
}

impl Default for RegistryOptions<Arc<dyn Retrieve>> {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    /// Get [`RegistryOptions`] for configuring a new [`Registry`].
    #[must_use]
    pub fn options() -> RegistryOptions<Arc<dyn Retrieve>> {
        RegistryOptions::new()
    }
    /// Create a new [`Registry`] with a single resource.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the resource.
    /// * `resource` - The resource to add.
    ///
    /// # Errors
    ///
    /// Returns an error if the URI is invalid or if there's an issue processing the resource.
    pub fn try_new(uri: impl AsRef<str>, resource: Resource) -> Result<Self, Error> {
        Self::try_new_impl(uri, resource, &DefaultRetriever, Draft::default())
    }
    /// Create a new [`Registry`] from an iterator of (URI, Resource) pairs.
    ///
    /// # Arguments
    ///
    /// * `pairs` - An iterator of (URI, Resource) pairs.
    ///
    /// # Errors
    ///
    /// Returns an error if any URI is invalid or if there's an issue processing the resources.
    pub fn try_from_resources(
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
    ) -> Result<Self, Error> {
        Self::try_from_resources_impl(pairs, &DefaultRetriever, Draft::default())
    }
    fn try_new_impl(
        uri: impl AsRef<str>,
        resource: Resource,
        retriever: &dyn Retrieve,
        draft: Draft,
    ) -> Result<Self, Error> {
        Self::try_from_resources_impl([(uri, resource)], retriever, draft)
    }
    fn try_from_resources_impl(
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
        retriever: &dyn Retrieve,
        draft: Draft,
    ) -> Result<Self, Error> {
        let mut documents = AHashMap::new();
        let mut resources = ResourceMap::new();
        let mut anchors = AHashMap::new();
        let mut resolution_cache = UriCache::new();
        let custom_metaschemas = process_resources(
            pairs,
            retriever,
            &mut documents,
            &mut resources,
            &mut anchors,
            &mut resolution_cache,
            draft,
        )?;

        // Validate that all custom $schema references are registered
        validate_custom_metaschemas(&custom_metaschemas, &resources)?;

        Ok(Registry {
            documents,
            resources,
            anchors,
            resolution_cache: resolution_cache.into_shared(),
        })
    }
    /// Create a new [`Registry`] from an iterator of (URI, Resource) pairs using an async retriever.
    ///
    /// # Arguments
    ///
    /// * `pairs` - An iterator of (URI, Resource) pairs.
    ///
    /// # Errors
    ///
    /// Returns an error if any URI is invalid or if there's an issue processing the resources.
    #[cfg(feature = "retrieve-async")]
    async fn try_from_resources_async_impl(
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
        retriever: &dyn crate::AsyncRetrieve,
        draft: Draft,
    ) -> Result<Self, Error> {
        let mut documents = AHashMap::new();
        let mut resources = ResourceMap::new();
        let mut anchors = AHashMap::new();
        let mut resolution_cache = UriCache::new();

        let custom_metaschemas = process_resources_async(
            pairs,
            retriever,
            &mut documents,
            &mut resources,
            &mut anchors,
            &mut resolution_cache,
            draft,
        )
        .await?;

        // Validate that all custom $schema references are registered
        validate_custom_metaschemas(&custom_metaschemas, &resources)?;

        Ok(Registry {
            documents,
            resources,
            anchors,
            resolution_cache: resolution_cache.into_shared(),
        })
    }
    /// Create a new registry with a new resource.
    ///
    /// # Errors
    ///
    /// Returns an error if the URI is invalid or if there's an issue processing the resource.
    pub fn try_with_resource(
        self,
        uri: impl AsRef<str>,
        resource: Resource,
    ) -> Result<Registry, Error> {
        let draft = resource.draft();
        self.try_with_resources([(uri, resource)], draft)
    }
    /// Create a new registry with new resources.
    ///
    /// # Errors
    ///
    /// Returns an error if any URI is invalid or if there's an issue processing the resources.
    pub fn try_with_resources(
        self,
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
        draft: Draft,
    ) -> Result<Registry, Error> {
        self.try_with_resources_and_retriever(pairs, &DefaultRetriever, draft)
    }
    /// Create a new registry with new resources and using the given retriever.
    ///
    /// # Errors
    ///
    /// Returns an error if any URI is invalid or if there's an issue processing the resources.
    pub fn try_with_resources_and_retriever(
        self,
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
        retriever: &dyn Retrieve,
        draft: Draft,
    ) -> Result<Registry, Error> {
        let mut documents = self.documents;
        let mut resources = self.resources;
        let mut anchors = self.anchors;
        let mut resolution_cache = self.resolution_cache.into_local();
        let custom_metaschemas = process_resources(
            pairs,
            retriever,
            &mut documents,
            &mut resources,
            &mut anchors,
            &mut resolution_cache,
            draft,
        )?;
        validate_custom_metaschemas(&custom_metaschemas, &resources)?;
        Ok(Registry {
            documents,
            resources,
            anchors,
            resolution_cache: resolution_cache.into_shared(),
        })
    }
    /// Create a new registry with new resources and using the given non-blocking retriever.
    ///
    /// # Errors
    ///
    /// Returns an error if any URI is invalid or if there's an issue processing the resources.
    #[cfg(feature = "retrieve-async")]
    pub async fn try_with_resources_and_retriever_async(
        self,
        pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
        retriever: &dyn crate::AsyncRetrieve,
        draft: Draft,
    ) -> Result<Registry, Error> {
        let mut documents = self.documents;
        let mut resources = self.resources;
        let mut anchors = self.anchors;
        let mut resolution_cache = self.resolution_cache.into_local();
        let custom_metaschemas = process_resources_async(
            pairs,
            retriever,
            &mut documents,
            &mut resources,
            &mut anchors,
            &mut resolution_cache,
            draft,
        )
        .await?;
        validate_custom_metaschemas(&custom_metaschemas, &resources)?;
        Ok(Registry {
            documents,
            resources,
            anchors,
            resolution_cache: resolution_cache.into_shared(),
        })
    }
    /// Create a new [`Resolver`] for this registry with the given base URI.
    ///
    /// # Errors
    ///
    /// Returns an error if the base URI is invalid.
    pub fn try_resolver(&self, base_uri: &str) -> Result<Resolver<'_>, Error> {
        let base = uri::from_str(base_uri)?;
        Ok(self.resolver(base))
    }
    /// Create a new [`Resolver`] for this registry with a known valid base URI.
    #[must_use]
    pub fn resolver(&self, base_uri: Uri<String>) -> Resolver<'_> {
        Resolver::new(self, Arc::new(base_uri))
    }
    pub(crate) fn anchor<'a>(&self, uri: &'a Uri<String>, name: &'a str) -> Result<&Anchor, Error> {
        let key = AnchorKeyRef::new(uri, name);
        if let Some(value) = self.anchors.get(key.borrow_dyn()) {
            return Ok(value);
        }
        let resource = &self.resources[uri];
        if let Some(id) = resource.id() {
            let uri = uri::from_str(id)?;
            let key = AnchorKeyRef::new(&uri, name);
            if let Some(value) = self.anchors.get(key.borrow_dyn()) {
                return Ok(value);
            }
        }
        if name.contains('/') {
            Err(Error::invalid_anchor(name.to_string()))
        } else {
            Err(Error::no_such_anchor(name.to_string()))
        }
    }
    /// Resolves a reference URI against a base URI using registry's cache.
    ///
    /// # Errors
    ///
    /// Returns an error if base has not schema or there is a fragment.
    pub fn resolve_against(&self, base: &Uri<&str>, uri: &str) -> Result<Arc<Uri<String>>, Error> {
        self.resolution_cache.resolve_against(base, uri)
    }
    /// Returns vocabulary set configured for given draft and contents.
    ///
    /// For custom meta-schemas (`Draft::Unknown`), looks up the meta-schema in the registry
    /// and extracts its `$vocabulary` declaration. If the meta-schema is not registered,
    /// returns the default Draft 2020-12 vocabularies.
    #[must_use]
    pub fn find_vocabularies(&self, draft: Draft, contents: &Value) -> VocabularySet {
        match draft.detect(contents) {
            Draft::Unknown => {
                // Custom/unknown meta-schema - try to look it up in the registry
                if let Some(specification) = contents
                    .as_object()
                    .and_then(|obj| obj.get("$schema"))
                    .and_then(|s| s.as_str())
                {
                    if let Ok(mut uri) = uri::from_str(specification) {
                        // Remove fragment for lookup (e.g., "http://example.com/schema#" -> "http://example.com/schema")
                        // Resources are stored without fragments, so we must strip it to find the meta-schema
                        uri.set_fragment(None);
                        if let Some(resource) = self.resources.get(&uri) {
                            // Found the custom meta-schema - extract vocabularies
                            if let Ok(Some(vocabularies)) = vocabularies::find(resource.contents())
                            {
                                return vocabularies;
                            }
                        }
                        // Meta-schema not registered - this will be caught during compilation
                        // For now, return default vocabularies to allow resource creation
                    }
                }
                // Default to Draft 2020-12 vocabularies for unknown meta-schemas
                Draft::Unknown.default_vocabularies()
            }
            draft => draft.default_vocabularies(),
        }
    }

    /// Build a registry with all the given meta-schemas from specs.
    pub(crate) fn build_from_meta_schemas(schemas: &[(&'static str, &'static Value)]) -> Self {
        let schemas_count = schemas.len();
        let pairs = schemas
            .iter()
            .map(|(uri, schema)| (uri, ResourceRef::from_contents(schema)));

        let mut documents = DocumentStore::with_capacity(schemas_count);
        let mut resources = ResourceMap::with_capacity(schemas_count);

        // The actual number of anchors and cache-entries varies across
        // drafts. We overshoot here to avoid reallocations, using the sum
        // over all specifications.
        let mut anchors = AHashMap::with_capacity(8);
        let mut resolution_cache = UriCache::with_capacity(35);

        process_meta_schemas(
            pairs,
            &mut documents,
            &mut resources,
            &mut anchors,
            &mut resolution_cache,
        )
        .expect("Failed to process meta schemas");

        Self {
            documents,
            resources,
            anchors,
            resolution_cache: resolution_cache.into_shared(),
        }
    }
}

fn process_meta_schemas(
    pairs: impl IntoIterator<Item = (impl AsRef<str>, ResourceRef<'static>)>,
    documents: &mut DocumentStore,
    resources: &mut ResourceMap,
    anchors: &mut AHashMap<AnchorKey, Anchor>,
    resolution_cache: &mut UriCache,
) -> Result<(), Error> {
    let mut queue = VecDeque::with_capacity(32);

    for (uri, resource) in pairs {
        let uri = uri::from_str(uri.as_ref().trim_end_matches('#'))?;
        let key = Arc::new(uri);
        let contents: &'static Value = resource.contents();
        let wrapped_value = Arc::pin(ValueWrapper::StaticRef(contents));
        let resource = InnerResourcePtr::new((*wrapped_value).as_ref(), resource.draft());
        documents.insert(Arc::clone(&key), wrapped_value);
        resources.insert(Arc::clone(&key), resource.clone());
        queue.push_back((key, resource));
    }

    // Process current queue and collect references to external resources
    while let Some((mut base, resource)) = queue.pop_front() {
        if let Some(id) = resource.id() {
            base = resolution_cache.resolve_against(&base.borrow(), id)?;
            resources.insert(base.clone(), resource.clone());
        }

        // Look for anchors
        for anchor in resource.anchors() {
            anchors.insert(AnchorKey::new(base.clone(), anchor.name()), anchor);
        }

        // Process subresources
        for contents in resource.draft().subresources_of(resource.contents()) {
            let subresource = InnerResourcePtr::new(contents, resource.draft());
            queue.push_back((base.clone(), subresource));
        }
    }
    Ok(())
}

#[derive(Hash, Eq, PartialEq)]
struct ReferenceKey {
    base_ptr: NonZeroUsize,
    reference: String,
}

impl ReferenceKey {
    fn new(base: &Arc<Uri<String>>, reference: &str) -> Self {
        Self {
            base_ptr: NonZeroUsize::new(Arc::as_ptr(base) as usize)
                .expect("Arc pointer should never be null"),
            reference: reference.to_owned(),
        }
    }
}

type ReferenceTracker = AHashSet<ReferenceKey>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ReferenceKind {
    Ref,
    Schema,
}

/// An entry in the processing queue.
/// The optional third element is the document root URI, used when the resource
/// was extracted from a fragment of a larger document. Local `$ref`s need to be
/// resolved against the document root, not just the fragment content.
type QueueEntry = (Arc<Uri<String>>, InnerResourcePtr, Option<Arc<Uri<String>>>);

struct ProcessingState {
    queue: VecDeque<QueueEntry>,
    seen: ReferenceTracker,
    external: AHashSet<(String, Uri<String>, ReferenceKind)>,
    scratch: String,
    refers_metaschemas: bool,
    custom_metaschemas: Vec<Arc<Uri<String>>>,
    /// Tracks schema pointers we've visited during recursive external resource collection.
    /// This prevents infinite recursion when schemas reference each other.
    visited_schemas: AHashSet<usize>,
}

impl ProcessingState {
    fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(32),
            seen: ReferenceTracker::new(),
            external: AHashSet::new(),
            scratch: String::new(),
            refers_metaschemas: false,
            custom_metaschemas: Vec::new(),
            visited_schemas: AHashSet::new(),
        }
    }
}

fn process_input_resources(
    pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
    documents: &mut DocumentStore,
    resources: &mut ResourceMap,
    state: &mut ProcessingState,
) -> Result<(), Error> {
    for (uri, resource) in pairs {
        let uri = uri::from_str(uri.as_ref().trim_end_matches('#'))?;
        let key = Arc::new(uri);
        match documents.entry(Arc::clone(&key)) {
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => {
                let (draft, contents) = resource.into_inner();
                let wrapped_value = Arc::pin(ValueWrapper::Owned(contents));
                let resource = InnerResourcePtr::new((*wrapped_value).as_ref(), draft);
                resources.insert(Arc::clone(&key), resource.clone());

                // Track resources with custom meta-schemas for later validation
                if draft == Draft::Unknown {
                    state.custom_metaschemas.push(Arc::clone(&key));
                }

                state.queue.push_back((key, resource, None));
                entry.insert(wrapped_value);
            }
        }
    }
    Ok(())
}

fn process_queue(
    state: &mut ProcessingState,
    resources: &mut ResourceMap,
    anchors: &mut AHashMap<AnchorKey, Anchor>,
    resolution_cache: &mut UriCache,
) -> Result<(), Error> {
    while let Some((mut base, resource, document_root_uri)) = state.queue.pop_front() {
        if let Some(id) = resource.id() {
            base = resolve_id(&base, id, resolution_cache)?;
            resources.insert(base.clone(), resource.clone());
        }

        for anchor in resource.anchors() {
            anchors.insert(AnchorKey::new(base.clone(), anchor.name()), anchor);
        }

        // Determine the document root for resolving local $refs.
        // If document_root_uri is set (e.g., for fragment-extracted resources),
        // look up the full document. Otherwise, this resource IS the document root.
        let root = document_root_uri
            .as_ref()
            .and_then(|uri| resources.get(uri))
            .map_or_else(|| resource.contents(), InnerResourcePtr::contents);

        // Skip if already visited during local $ref resolution
        let contents_ptr = std::ptr::from_ref::<Value>(resource.contents()) as usize;
        if state.visited_schemas.insert(contents_ptr) {
            collect_external_resources(
                &base,
                root,
                resource.contents(),
                &mut state.external,
                &mut state.seen,
                resolution_cache,
                &mut state.scratch,
                &mut state.refers_metaschemas,
                resource.draft(),
                &mut state.visited_schemas,
            )?;
        }

        // Subresources inherit the document root URI, or use the current base if none set
        let subresource_root_uri = document_root_uri.or_else(|| Some(base.clone()));
        for contents in resource.draft().subresources_of(resource.contents()) {
            let subresource = InnerResourcePtr::new(contents, resource.draft());
            state
                .queue
                .push_back((base.clone(), subresource, subresource_root_uri.clone()));
        }
    }
    Ok(())
}

fn handle_fragment(
    uri: &Uri<String>,
    resource: &InnerResourcePtr,
    key: &Arc<Uri<String>>,
    default_draft: Draft,
    queue: &mut VecDeque<QueueEntry>,
    document_root_uri: Arc<Uri<String>>,
) {
    if let Some(fragment) = uri.fragment() {
        if let Some(resolved) = pointer(resource.contents(), fragment.as_str()) {
            let draft = default_draft.detect(resolved);
            let contents = std::ptr::addr_of!(*resolved);
            let resource = InnerResourcePtr::new(contents, draft);
            queue.push_back((Arc::clone(key), resource, Some(document_root_uri)));
        }
    }
}

fn handle_metaschemas(
    refers_metaschemas: bool,
    resources: &mut ResourceMap,
    anchors: &mut AHashMap<AnchorKey, Anchor>,
    draft_version: Draft,
) {
    if refers_metaschemas {
        let schemas = metas_for_draft(draft_version);
        let draft_registry = Registry::build_from_meta_schemas(schemas);
        resources.reserve(draft_registry.resources.len());
        for (key, resource) in draft_registry.resources {
            resources.insert(key, resource.clone());
        }
        anchors.reserve(draft_registry.anchors.len());
        for (key, anchor) in draft_registry.anchors {
            anchors.insert(key, anchor);
        }
    }
}

fn create_resource(
    retrieved: Value,
    fragmentless: Uri<String>,
    default_draft: Draft,
    documents: &mut DocumentStore,
    resources: &mut ResourceMap,
    custom_metaschemas: &mut Vec<Arc<Uri<String>>>,
) -> (Arc<Uri<String>>, InnerResourcePtr) {
    let draft = default_draft.detect(&retrieved);
    let wrapped_value = Arc::pin(ValueWrapper::Owned(retrieved));
    let resource = InnerResourcePtr::new((*wrapped_value).as_ref(), draft);
    let key = Arc::new(fragmentless);
    documents.insert(Arc::clone(&key), wrapped_value);
    resources.insert(Arc::clone(&key), resource.clone());

    // Track resources with custom meta-schemas for later validation
    if draft == Draft::Unknown {
        custom_metaschemas.push(Arc::clone(&key));
    }

    (key, resource)
}

fn process_resources(
    pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
    retriever: &dyn Retrieve,
    documents: &mut DocumentStore,
    resources: &mut ResourceMap,
    anchors: &mut AHashMap<AnchorKey, Anchor>,
    resolution_cache: &mut UriCache,
    default_draft: Draft,
) -> Result<Vec<Arc<Uri<String>>>, Error> {
    let mut state = ProcessingState::new();
    process_input_resources(pairs, documents, resources, &mut state)?;

    loop {
        if state.queue.is_empty() && state.external.is_empty() {
            break;
        }

        process_queue(&mut state, resources, anchors, resolution_cache)?;

        // Retrieve external resources
        for (original, uri, kind) in state.external.drain() {
            let mut fragmentless = uri.clone();
            fragmentless.set_fragment(None);
            if !resources.contains_key(&fragmentless) {
                let retrieved = match retriever.retrieve(&fragmentless) {
                    Ok(retrieved) => retrieved,
                    Err(error) => {
                        handle_retrieve_error(&uri, &original, &fragmentless, error, kind)?;
                        continue;
                    }
                };

                let (key, resource) = create_resource(
                    retrieved,
                    fragmentless,
                    default_draft,
                    documents,
                    resources,
                    &mut state.custom_metaschemas,
                );
                handle_fragment(
                    &uri,
                    &resource,
                    &key,
                    default_draft,
                    &mut state.queue,
                    Arc::clone(&key),
                );
                state.queue.push_back((key, resource, None));
            }
        }
    }

    handle_metaschemas(state.refers_metaschemas, resources, anchors, default_draft);

    Ok(state.custom_metaschemas)
}

#[cfg(feature = "retrieve-async")]
async fn process_resources_async(
    pairs: impl IntoIterator<Item = (impl AsRef<str>, Resource)>,
    retriever: &dyn crate::AsyncRetrieve,
    documents: &mut DocumentStore,
    resources: &mut ResourceMap,
    anchors: &mut AHashMap<AnchorKey, Anchor>,
    resolution_cache: &mut UriCache,
    default_draft: Draft,
) -> Result<Vec<Arc<Uri<String>>>, Error> {
    let mut state = ProcessingState::new();
    process_input_resources(pairs, documents, resources, &mut state)?;

    loop {
        if state.queue.is_empty() && state.external.is_empty() {
            break;
        }

        process_queue(&mut state, resources, anchors, resolution_cache)?;

        if !state.external.is_empty() {
            let data = state
                .external
                .drain()
                .filter_map(|(original, uri, kind)| {
                    let mut fragmentless = uri.clone();
                    fragmentless.set_fragment(None);
                    if resources.contains_key(&fragmentless) {
                        None
                    } else {
                        Some((original, uri, kind, fragmentless))
                    }
                })
                .collect::<Vec<_>>();

            let results = {
                let futures = data
                    .iter()
                    .map(|(_, _, _, fragmentless)| retriever.retrieve(fragmentless));
                futures::future::join_all(futures).await
            };

            for ((original, uri, kind, fragmentless), result) in data.iter().zip(results) {
                let retrieved = match result {
                    Ok(retrieved) => retrieved,
                    Err(error) => {
                        handle_retrieve_error(uri, original, fragmentless, error, *kind)?;
                        continue;
                    }
                };

                let (key, resource) = create_resource(
                    retrieved,
                    fragmentless.clone(),
                    default_draft,
                    documents,
                    resources,
                    &mut state.custom_metaschemas,
                );
                handle_fragment(
                    uri,
                    &resource,
                    &key,
                    default_draft,
                    &mut state.queue,
                    Arc::clone(&key),
                );
                state.queue.push_back((key, resource, None));
            }
        }
    }

    handle_metaschemas(state.refers_metaschemas, resources, anchors, default_draft);

    Ok(state.custom_metaschemas)
}

fn handle_retrieve_error(
    uri: &Uri<String>,
    original: &str,
    fragmentless: &Uri<String>,
    error: Box<dyn std::error::Error + Send + Sync>,
    kind: ReferenceKind,
) -> Result<(), Error> {
    match kind {
        ReferenceKind::Schema => {
            // $schema fetch failures are non-fatal during resource processing
            // Unregistered custom meta-schemas will be caught in validate_custom_metaschemas()
            Ok(())
        }
        ReferenceKind::Ref => {
            // $ref fetch failures are fatal - they're required for validation
            if uri.scheme().as_str() == "json-schema" {
                Err(Error::unretrievable(
                    original,
                    "No base URI is available".into(),
                ))
            } else {
                Err(Error::unretrievable(fragmentless.as_str(), error))
            }
        }
    }
}

fn validate_custom_metaschemas(
    custom_metaschemas: &[Arc<Uri<String>>],
    resources: &ResourceMap,
) -> Result<(), Error> {
    // Only validate resources with Draft::Unknown
    for uri in custom_metaschemas {
        if let Some(resource) = resources.get(uri) {
            // Extract the $schema value from this resource
            if let Some(schema_uri) = resource
                .contents()
                .as_object()
                .and_then(|obj| obj.get("$schema"))
                .and_then(|s| s.as_str())
            {
                // Check if this meta-schema is registered
                match uri::from_str(schema_uri) {
                    Ok(mut meta_uri) => {
                        // Remove fragment for lookup (e.g., "http://example.com/schema#" -> "http://example.com/schema")
                        meta_uri.set_fragment(None);
                        if !resources.contains_key(&meta_uri) {
                            return Err(Error::unknown_specification(schema_uri));
                        }
                    }
                    Err(_) => {
                        return Err(Error::unknown_specification(schema_uri));
                    }
                }
            }
        }
    }
    Ok(())
}

fn collect_external_resources(
    base: &Arc<Uri<String>>,
    root: &Value,
    contents: &Value,
    collected: &mut AHashSet<(String, Uri<String>, ReferenceKind)>,
    seen: &mut ReferenceTracker,
    resolution_cache: &mut UriCache,
    scratch: &mut String,
    refers_metaschemas: &mut bool,
    draft: Draft,
    visited: &mut AHashSet<usize>,
) -> Result<(), Error> {
    // URN schemes are not supported for external resolution
    if base.scheme().as_str() == "urn" {
        return Ok(());
    }

    macro_rules! on_reference {
        ($reference:expr, $key:literal) => {
            // Skip well-known schema references
            if $reference.starts_with("https://json-schema.org/draft/")
                || $reference.starts_with("http://json-schema.org/draft-")
                || base.as_str().starts_with("https://json-schema.org/draft/")
            {
                if $key == "$ref" {
                    *refers_metaschemas = true;
                }
            } else if $reference != "#" {
                if mark_reference(seen, base, $reference) {
                    // Handle local references separately as they may have nested references to external resources
                    if $reference.starts_with('#') {
                        // Use the root document for pointer resolution since local refs are always
                        // relative to the document root, not the current subschema.
                        // Also track $id changes along the path to get the correct base URI.
                        if let Some((referenced, resolved_base)) = pointer_with_base(
                            root,
                            $reference.trim_start_matches('#'),
                            base,
                            resolution_cache,
                            draft,
                        )? {
                            // Recursively collect from the referenced schema and all its subresources
                            collect_external_resources_recursive(
                                &resolved_base,
                                root,
                                referenced,
                                collected,
                                seen,
                                resolution_cache,
                                scratch,
                                refers_metaschemas,
                                draft,
                                visited,
                            )?;
                        }
                    } else {
                        let resolved = if base.has_fragment() {
                            let mut base_without_fragment = base.as_ref().clone();
                            base_without_fragment.set_fragment(None);

                            let (path, fragment) = match $reference.split_once('#') {
                                Some((path, fragment)) => (path, Some(fragment)),
                                None => ($reference, None),
                            };

                            let mut resolved = (*resolution_cache
                                .resolve_against(&base_without_fragment.borrow(), path)?)
                            .clone();
                            // Add the fragment back if present
                            if let Some(fragment) = fragment {
                                // It is cheaper to check if it is properly encoded than allocate given that
                                // the majority of inputs do not need to be additionally encoded
                                if let Some(encoded) = uri::EncodedString::new(fragment) {
                                    resolved = resolved.with_fragment(Some(encoded));
                                } else {
                                    uri::encode_to(fragment, scratch);
                                    resolved = resolved.with_fragment(Some(
                                        uri::EncodedString::new_or_panic(scratch),
                                    ));
                                    scratch.clear();
                                }
                            }
                            resolved
                        } else {
                            (*resolution_cache
                                .resolve_against(&base.borrow(), $reference)?)
                            .clone()
                        };

                        let kind = if $key == "$schema" {
                            ReferenceKind::Schema
                        } else {
                            ReferenceKind::Ref
                        };
                        collected.insert(($reference.to_string(), resolved, kind));
                    }
                }
            }
        };
    }

    if let Some(object) = contents.as_object() {
        if object.len() < 3 {
            for (key, value) in object {
                if key == "$ref" {
                    if let Some(reference) = value.as_str() {
                        on_reference!(reference, "$ref");
                    }
                } else if key == "$schema" {
                    if let Some(reference) = value.as_str() {
                        on_reference!(reference, "$schema");
                    }
                }
            }
        } else {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
                on_reference!(reference, "$ref");
            }
            if let Some(reference) = object.get("$schema").and_then(Value::as_str) {
                on_reference!(reference, "$schema");
            }
        }
    }
    Ok(())
}

/// Recursively collect external resources from a schema and all its subresources.
///
/// The `visited` set tracks schema pointers we've already processed to avoid infinite
/// recursion when schemas reference each other (directly or through subresources).
fn collect_external_resources_recursive(
    base: &Arc<Uri<String>>,
    root: &Value,
    contents: &Value,
    collected: &mut AHashSet<(String, Uri<String>, ReferenceKind)>,
    seen: &mut ReferenceTracker,
    resolution_cache: &mut UriCache,
    scratch: &mut String,
    refers_metaschemas: &mut bool,
    draft: Draft,
    visited: &mut AHashSet<usize>,
) -> Result<(), Error> {
    // Track by pointer address to avoid processing the same schema twice
    let ptr = std::ptr::from_ref::<Value>(contents) as usize;
    if !visited.insert(ptr) {
        return Ok(());
    }

    let current_base = match draft.id_of(contents) {
        Some(id) => resolve_id(base, id, resolution_cache)?,
        None => Arc::clone(base),
    };

    // First, collect from the current schema
    collect_external_resources(
        &current_base,
        root,
        contents,
        collected,
        seen,
        resolution_cache,
        scratch,
        refers_metaschemas,
        draft,
        visited,
    )?;

    // Then recursively process all subresources
    for subresource in draft.subresources_of(contents) {
        collect_external_resources_recursive(
            &current_base,
            root,
            subresource,
            collected,
            seen,
            resolution_cache,
            scratch,
            refers_metaschemas,
            draft,
            visited,
        )?;
    }
    Ok(())
}

fn mark_reference(seen: &mut ReferenceTracker, base: &Arc<Uri<String>>, reference: &str) -> bool {
    seen.insert(ReferenceKey::new(base, reference))
}

/// Resolve an `$id` against a base URI, handling anchor-style IDs and empty fragments.
///
/// Anchor-style `$id` values (starting with `#`) don't change the base URI.
/// Empty fragments are stripped from the resolved URI.
fn resolve_id(
    base: &Arc<Uri<String>>,
    id: &str,
    resolution_cache: &mut UriCache,
) -> Result<Arc<Uri<String>>, Error> {
    if id.starts_with('#') {
        return Ok(Arc::clone(base));
    }
    let mut resolved = (*resolution_cache.resolve_against(&base.borrow(), id)?).clone();
    if resolved.fragment().is_some_and(EStr::is_empty) {
        resolved.set_fragment(None);
    }
    Ok(Arc::new(resolved))
}

/// Look up a value by a JSON Pointer.
///
/// **NOTE**: A slightly faster version of pointer resolution based on `Value::pointer` from `serde_json`.
pub fn pointer<'a>(document: &'a Value, pointer: &str) -> Option<&'a Value> {
    if pointer.is_empty() {
        return Some(document);
    }
    if !pointer.starts_with('/') {
        return None;
    }
    pointer.split('/').skip(1).map(unescape_segment).try_fold(
        document,
        |target, token| match target {
            Value::Object(map) => map.get(&*token),
            Value::Array(list) => parse_index(&token).and_then(|x| list.get(x)),
            _ => None,
        },
    )
}

/// Look up a value by a JSON Pointer, tracking `$id` changes along the path.
///
/// Returns both the resolved value and the accumulated base URI after processing
/// any `$id` declarations encountered along the path. Note that anchor-style `$id`
/// values (starting with `#`) don't change the base URI.
#[allow(clippy::type_complexity)]
fn pointer_with_base<'a>(
    document: &'a Value,
    pointer: &str,
    base: &Arc<Uri<String>>,
    resolution_cache: &mut UriCache,
    draft: Draft,
) -> Result<Option<(&'a Value, Arc<Uri<String>>)>, Error> {
    if pointer.is_empty() {
        return Ok(Some((document, Arc::clone(base))));
    }
    if !pointer.starts_with('/') {
        return Ok(None);
    }

    let mut current = document;
    let mut current_base = Arc::clone(base);

    for token in pointer.split('/').skip(1).map(unescape_segment) {
        // Check for $id in the current value before traversing deeper
        if let Some(id) = draft.id_of(current) {
            current_base = resolve_id(&current_base, id, resolution_cache)?;
        }

        current = match current {
            Value::Object(map) => match map.get(&*token) {
                Some(v) => v,
                None => return Ok(None),
            },
            Value::Array(list) => match parse_index(&token).and_then(|x| list.get(x)) {
                Some(v) => v,
                None => return Ok(None),
            },
            _ => return Ok(None),
        };
    }

    // Note: We don't check $id in the final value here because
    // `collect_external_resources_recursive` will handle it
    Ok(Some((current, current_base)))
}

// Taken from `serde_json`.
#[must_use]
pub fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;

    use ahash::AHashMap;
    use fluent_uri::Uri;
    use serde_json::{json, Value};
    use test_case::test_case;

    use crate::{uri::from_str, Draft, Registry, Resource, Retrieve};

    use super::{pointer, RegistryOptions, SPECIFICATIONS};

    #[test]
    fn test_empty_pointer() {
        let document = json!({});
        assert_eq!(pointer(&document, ""), Some(&document));
    }

    #[test]
    fn test_invalid_uri_on_registry_creation() {
        let schema = Draft::Draft202012.create_resource(json!({}));
        let result = Registry::try_new(":/example.com", schema);
        let error = result.expect_err("Should fail");

        assert_eq!(
            error.to_string(),
            "Invalid URI reference ':/example.com': unexpected character at index 0"
        );
        let source_error = error.source().expect("Should have a source");
        let inner_source = source_error.source().expect("Should have a source");
        assert_eq!(inner_source.to_string(), "unexpected character at index 0");
    }

    #[test]
    fn test_lookup_unresolvable_url() {
        // Create a registry with a single resource
        let schema = Draft::Draft202012.create_resource(json!({
            "type": "object",
            "properties": {
                "foo": { "type": "string" }
            }
        }));
        let registry =
            Registry::try_new("http://example.com/schema1", schema).expect("Invalid resources");

        // Attempt to create a resolver for a URL not in the registry
        let resolver = registry
            .try_resolver("http://example.com/non_existent_schema")
            .expect("Invalid base URI");

        let result = resolver.lookup("");

        assert_eq!(
            result.unwrap_err().to_string(),
            "Resource 'http://example.com/non_existent_schema' is not present in a registry and retrieving it failed: Retrieving external resources is not supported once the registry is populated"
        );
    }

    #[test]
    fn test_relative_uri_without_base() {
        let schema = Draft::Draft202012.create_resource(json!({"$ref": "./virtualNetwork.json"}));
        let error = Registry::try_new("json-schema:///", schema).expect_err("Should fail");
        assert_eq!(error.to_string(), "Resource './virtualNetwork.json' is not present in a registry and retrieving it failed: No base URI is available");
    }

    #[test]
    fn test_try_with_resources_requires_registered_custom_meta_schema() {
        let base_registry = Registry::try_new(
            "http://example.com/root",
            Resource::from_contents(json!({"type": "object"})),
        )
        .expect("Base registry should be created");

        let custom_schema = Resource::from_contents(json!({
            "$id": "http://example.com/custom",
            "$schema": "http://example.com/meta/custom",
            "type": "string"
        }));

        let error = base_registry
            .try_with_resources(
                [("http://example.com/custom", custom_schema)],
                Draft::default(),
            )
            .expect_err("Extending registry must fail when the custom $schema is not registered");

        let error_msg = error.to_string();
        assert_eq!(
            error_msg,
            "Unknown meta-schema: 'http://example.com/meta/custom'. Custom meta-schemas must be registered in the registry before use"
        );
    }

    #[test]
    fn test_try_with_resources_accepts_registered_custom_meta_schema_fragment() {
        let meta_schema = Resource::from_contents(json!({
            "$id": "http://example.com/meta/custom#",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object"
        }));

        let registry = Registry::try_new("http://example.com/meta/custom#", meta_schema)
            .expect("Meta-schema should be registered successfully");

        let schema = Resource::from_contents(json!({
            "$id": "http://example.com/schemas/my-schema",
            "$schema": "http://example.com/meta/custom#",
            "type": "string"
        }));

        registry
            .clone()
            .try_with_resources(
                [("http://example.com/schemas/my-schema", schema)],
                Draft::default(),
            )
            .expect("Schema should accept registered meta-schema URI with trailing '#'");
    }

    #[test]
    fn test_chained_custom_meta_schemas() {
        // Meta-schema B (uses standard Draft 2020-12)
        let meta_schema_b = json!({
            "$id": "json-schema:///meta/level-b",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2020-12/vocab/core": true,
                "https://json-schema.org/draft/2020-12/vocab/validation": true,
            },
            "type": "object",
            "properties": {
                "customProperty": {"type": "string"}
            }
        });

        // Meta-schema A (uses Meta-schema B)
        let meta_schema_a = json!({
            "$id": "json-schema:///meta/level-a",
            "$schema": "json-schema:///meta/level-b",
            "customProperty": "level-a-meta",
            "type": "object"
        });

        // Schema (uses Meta-schema A)
        let schema = json!({
            "$id": "json-schema:///schemas/my-schema",
            "$schema": "json-schema:///meta/level-a",
            "customProperty": "my-schema",
            "type": "string"
        });

        // Register all meta-schemas and schema in a chained manner
        // All resources are provided upfront, so no external retrieval should occur
        Registry::try_from_resources([
            (
                "json-schema:///meta/level-b",
                Resource::from_contents(meta_schema_b),
            ),
            (
                "json-schema:///meta/level-a",
                Resource::from_contents(meta_schema_a),
            ),
            (
                "json-schema:///schemas/my-schema",
                Resource::from_contents(schema),
            ),
        ])
        .expect("Chained custom meta-schemas should be accepted when all are registered");
    }

    struct TestRetriever {
        schemas: AHashMap<String, Value>,
    }

    impl TestRetriever {
        fn new(schemas: AHashMap<String, Value>) -> Self {
            TestRetriever { schemas }
        }
    }

    impl Retrieve for TestRetriever {
        fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            if let Some(value) = self.schemas.get(uri.as_str()) {
                Ok(value.clone())
            } else {
                Err(format!("Failed to find {uri}").into())
            }
        }
    }

    fn create_test_retriever(schemas: &[(&str, Value)]) -> TestRetriever {
        TestRetriever::new(
            schemas
                .iter()
                .map(|&(k, ref v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }

    struct TestCase {
        input_resources: Vec<(&'static str, Value)>,
        remote_resources: Vec<(&'static str, Value)>,
        expected_resolved_uris: Vec<&'static str>,
    }

    #[test_case(
        TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({"$ref": "http://example.com/schema2"})),
            ],
            remote_resources: vec![
                ("http://example.com/schema2", json!({"type": "object"})),
            ],
            expected_resolved_uris: vec!["http://example.com/schema1", "http://example.com/schema2"],
        }
    ;"External ref at top")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({
                    "$defs": {
                        "subschema": {"type": "string"}
                    },
                    "$ref": "#/$defs/subschema"
                })),
            ],
            remote_resources: vec![],
            expected_resolved_uris: vec!["http://example.com/schema1"],
        }
    ;"Internal ref at top")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({"$ref": "http://example.com/schema2"})),
                ("http://example.com/schema2", json!({"type": "object"})),
            ],
            remote_resources: vec![],
            expected_resolved_uris: vec!["http://example.com/schema1", "http://example.com/schema2"],
        }
    ;"Ref to later resource")]
    #[test_case(
    TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({
                    "type": "object",
                    "properties": {
                        "prop1": {"$ref": "http://example.com/schema2"}
                    }
                })),
            ],
            remote_resources: vec![
                ("http://example.com/schema2", json!({"type": "string"})),
            ],
            expected_resolved_uris: vec!["http://example.com/schema1", "http://example.com/schema2"],
        }
    ;"External ref in subresource")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({
                    "type": "object",
                    "properties": {
                        "prop1": {"$ref": "#/$defs/subschema"}
                    },
                    "$defs": {
                        "subschema": {"type": "string"}
                    }
                })),
            ],
            remote_resources: vec![],
            expected_resolved_uris: vec!["http://example.com/schema1"],
        }
    ;"Internal ref in subresource")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("file:///schemas/main.json", json!({"$ref": "file:///schemas/external.json"})),
            ],
            remote_resources: vec![
                ("file:///schemas/external.json", json!({"type": "object"})),
            ],
            expected_resolved_uris: vec!["file:///schemas/main.json", "file:///schemas/external.json"],
        }
    ;"File scheme: external ref at top")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("file:///schemas/main.json", json!({"$ref": "subfolder/schema.json"})),
            ],
            remote_resources: vec![
                ("file:///schemas/subfolder/schema.json", json!({"type": "string"})),
            ],
            expected_resolved_uris: vec!["file:///schemas/main.json", "file:///schemas/subfolder/schema.json"],
        }
    ;"File scheme: relative path ref")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("file:///schemas/main.json", json!({
                    "type": "object",
                    "properties": {
                        "local": {"$ref": "local.json"},
                        "remote": {"$ref": "http://example.com/schema"}
                    }
                })),
            ],
            remote_resources: vec![
                ("file:///schemas/local.json", json!({"type": "string"})),
                ("http://example.com/schema", json!({"type": "number"})),
            ],
            expected_resolved_uris: vec![
                "file:///schemas/main.json",
                "file:///schemas/local.json",
                "http://example.com/schema"
            ],
        }
    ;"File scheme: mixing with http scheme")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("file:///C:/schemas/main.json", json!({"$ref": "/D:/other_schemas/schema.json"})),
            ],
            remote_resources: vec![
                ("file:///D:/other_schemas/schema.json", json!({"type": "boolean"})),
            ],
            expected_resolved_uris: vec![
                "file:///C:/schemas/main.json",
                "file:///D:/other_schemas/schema.json"
            ],
        }
    ;"File scheme: absolute path in Windows style")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({"$ref": "http://example.com/schema2"})),
            ],
            remote_resources: vec![
                ("http://example.com/schema2", json!({"$ref": "http://example.com/schema3"})),
                ("http://example.com/schema3", json!({"$ref": "http://example.com/schema4"})),
                ("http://example.com/schema4", json!({"$ref": "http://example.com/schema5"})),
                ("http://example.com/schema5", json!({"type": "object"})),
            ],
            expected_resolved_uris: vec![
                "http://example.com/schema1",
                "http://example.com/schema2",
                "http://example.com/schema3",
                "http://example.com/schema4",
                "http://example.com/schema5",
            ],
        }
    ;"Four levels of external references")]
    #[test_case(
        TestCase {
            input_resources: vec![
                ("http://example.com/schema1", json!({"$ref": "http://example.com/schema2"})),
            ],
            remote_resources: vec![
                ("http://example.com/schema2", json!({"$ref": "http://example.com/schema3"})),
                ("http://example.com/schema3", json!({"$ref": "http://example.com/schema4"})),
                ("http://example.com/schema4", json!({"$ref": "http://example.com/schema5"})),
                ("http://example.com/schema5", json!({"$ref": "http://example.com/schema6"})),
                ("http://example.com/schema6", json!({"$ref": "http://example.com/schema1"})),
            ],
            expected_resolved_uris: vec![
                "http://example.com/schema1",
                "http://example.com/schema2",
                "http://example.com/schema3",
                "http://example.com/schema4",
                "http://example.com/schema5",
                "http://example.com/schema6",
            ],
        }
    ;"Five levels of external references with circular reference")]
    fn test_references_processing(test_case: TestCase) {
        let retriever = create_test_retriever(&test_case.remote_resources);

        let input_pairs = test_case
            .input_resources
            .clone()
            .into_iter()
            .map(|(uri, value)| (uri, Resource::from_contents(value)));

        let registry = Registry::options()
            .retriever(retriever)
            .build(input_pairs)
            .expect("Invalid resources");
        // Verify that all expected URIs are resolved and present in resources
        for uri in test_case.expected_resolved_uris {
            let resolver = registry.try_resolver("").expect("Invalid base URI");
            assert!(resolver.lookup(uri).is_ok());
        }
    }

    #[test]
    fn test_default_retriever_with_remote_refs() {
        let result = Registry::try_from_resources([(
            "http://example.com/schema1",
            Resource::from_contents(json!({"$ref": "http://example.com/schema2"})),
        )]);
        let error = result.expect_err("Should fail");
        assert_eq!(error.to_string(), "Resource 'http://example.com/schema2' is not present in a registry and retrieving it failed: Default retriever does not fetch resources");
        assert!(error.source().is_some());
    }

    #[test]
    fn test_options() {
        let _registry = RegistryOptions::default()
            .build([("", Resource::from_contents(json!({})))])
            .expect("Invalid resources");
    }

    #[test]
    fn test_registry_with_duplicate_input_uris() {
        let input_resources = vec![
            (
                "http://example.com/schema",
                json!({
                    "type": "object",
                    "properties": {
                        "foo": { "type": "string" }
                    }
                }),
            ),
            (
                "http://example.com/schema",
                json!({
                    "type": "object",
                    "properties": {
                        "bar": { "type": "number" }
                    }
                }),
            ),
        ];

        let result = Registry::try_from_resources(
            input_resources
                .into_iter()
                .map(|(uri, value)| (uri, Draft::Draft202012.create_resource(value))),
        );

        assert!(
            result.is_ok(),
            "Failed to create registry with duplicate input URIs"
        );
        let registry = result.unwrap();

        let resource = registry
            .resources
            .get(&from_str("http://example.com/schema").expect("Invalid URI"))
            .unwrap();
        let properties = resource
            .contents()
            .get("properties")
            .and_then(|v| v.as_object())
            .unwrap();

        assert!(
            !properties.contains_key("bar"),
            "Registry should contain the earliest added schema"
        );
        assert!(
            properties.contains_key("foo"),
            "Registry should contain the overwritten schema"
        );
    }

    #[test]
    fn test_resolver_debug() {
        let registry = SPECIFICATIONS
            .clone()
            .try_with_resource("http://example.com", Resource::from_contents(json!({})))
            .expect("Invalid resource");
        let resolver = registry
            .try_resolver("http://127.0.0.1/schema")
            .expect("Invalid base URI");
        assert_eq!(
            format!("{resolver:?}"),
            "Resolver { base_uri: \"http://127.0.0.1/schema\", scopes: \"[]\" }"
        );
    }

    #[test]
    fn test_try_with_resource() {
        let registry = SPECIFICATIONS
            .clone()
            .try_with_resource("http://example.com", Resource::from_contents(json!({})))
            .expect("Invalid resource");
        let resolver = registry.try_resolver("").expect("Invalid base URI");
        let resolved = resolver
            .lookup("http://json-schema.org/draft-06/schema#/definitions/schemaArray")
            .expect("Lookup failed");
        assert_eq!(
            resolved.contents(),
            &json!({
                "type": "array",
                "minItems": 1,
                "items": { "$ref": "#" }
            })
        );
    }

    #[test]
    fn test_invalid_reference() {
        // Found via fuzzing
        let resource = Draft::Draft202012.create_resource(json!({"$schema": "$##"}));
        let _ = Registry::try_new("http://#/", resource);
    }
}

#[cfg(all(test, feature = "retrieve-async"))]
mod async_tests {
    use crate::{uri, DefaultRetriever, Draft, Registry, Resource, Uri};
    use ahash::AHashMap;
    use serde_json::{json, Value};
    use std::error::Error;

    struct TestAsyncRetriever {
        schemas: AHashMap<String, Value>,
    }

    impl TestAsyncRetriever {
        fn with_schema(uri: impl Into<String>, schema: Value) -> Self {
            TestAsyncRetriever {
                schemas: { AHashMap::from_iter([(uri.into(), schema)]) },
            }
        }
    }

    #[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
    impl crate::AsyncRetrieve for TestAsyncRetriever {
        async fn retrieve(
            &self,
            uri: &Uri<String>,
        ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
            self.schemas
                .get(uri.as_str())
                .cloned()
                .ok_or_else(|| "Schema not found".into())
        }
    }

    #[tokio::test]
    async fn test_default_async_retriever_with_remote_refs() {
        let result = Registry::options()
            .async_retriever(DefaultRetriever)
            .build([(
                "http://example.com/schema1",
                Resource::from_contents(json!({"$ref": "http://example.com/schema2"})),
            )])
            .await;

        let error = result.expect_err("Should fail");
        assert_eq!(error.to_string(), "Resource 'http://example.com/schema2' is not present in a registry and retrieving it failed: Default retriever does not fetch resources");
        assert!(error.source().is_some());
    }

    #[tokio::test]
    async fn test_async_options() {
        let _registry = Registry::options()
            .async_retriever(DefaultRetriever)
            .build([("", Draft::default().create_resource(json!({})))])
            .await
            .expect("Invalid resources");
    }

    #[tokio::test]
    async fn test_async_registry_with_duplicate_input_uris() {
        let input_resources = vec![
            (
                "http://example.com/schema",
                json!({
                    "type": "object",
                    "properties": {
                        "foo": { "type": "string" }
                    }
                }),
            ),
            (
                "http://example.com/schema",
                json!({
                    "type": "object",
                    "properties": {
                        "bar": { "type": "number" }
                    }
                }),
            ),
        ];

        let result = Registry::options()
            .async_retriever(DefaultRetriever)
            .build(
                input_resources
                    .into_iter()
                    .map(|(uri, value)| (uri, Draft::Draft202012.create_resource(value))),
            )
            .await;

        assert!(
            result.is_ok(),
            "Failed to create registry with duplicate input URIs"
        );
        let registry = result.unwrap();

        let resource = registry
            .resources
            .get(&uri::from_str("http://example.com/schema").expect("Invalid URI"))
            .unwrap();
        let properties = resource
            .contents()
            .get("properties")
            .and_then(|v| v.as_object())
            .unwrap();

        assert!(
            !properties.contains_key("bar"),
            "Registry should contain the earliest added schema"
        );
        assert!(
            properties.contains_key("foo"),
            "Registry should contain the overwritten schema"
        );
    }

    #[tokio::test]
    async fn test_async_try_with_resource() {
        let retriever = TestAsyncRetriever::with_schema(
            "http://example.com/schema2",
            json!({"type": "object"}),
        );

        let registry = Registry::options()
            .async_retriever(retriever)
            .build([(
                "http://example.com",
                Resource::from_contents(json!({"$ref": "http://example.com/schema2"})),
            )])
            .await
            .expect("Invalid resource");

        let resolver = registry.try_resolver("").expect("Invalid base URI");
        let resolved = resolver
            .lookup("http://example.com/schema2")
            .expect("Lookup failed");
        assert_eq!(resolved.contents(), &json!({"type": "object"}));
    }

    #[tokio::test]
    async fn test_async_registry_with_multiple_refs() {
        let retriever = TestAsyncRetriever {
            schemas: AHashMap::from_iter([
                (
                    "http://example.com/schema2".to_string(),
                    json!({"type": "object"}),
                ),
                (
                    "http://example.com/schema3".to_string(),
                    json!({"type": "string"}),
                ),
            ]),
        };

        let registry = Registry::options()
            .async_retriever(retriever)
            .build([(
                "http://example.com/schema1",
                Resource::from_contents(json!({
                    "type": "object",
                    "properties": {
                        "obj": {"$ref": "http://example.com/schema2"},
                        "str": {"$ref": "http://example.com/schema3"}
                    }
                })),
            )])
            .await
            .expect("Invalid resource");

        let resolver = registry.try_resolver("").expect("Invalid base URI");

        // Check both references are resolved correctly
        let resolved2 = resolver
            .lookup("http://example.com/schema2")
            .expect("Lookup failed");
        assert_eq!(resolved2.contents(), &json!({"type": "object"}));

        let resolved3 = resolver
            .lookup("http://example.com/schema3")
            .expect("Lookup failed");
        assert_eq!(resolved3.contents(), &json!({"type": "string"}));
    }

    #[tokio::test]
    async fn test_async_registry_with_nested_refs() {
        let retriever = TestAsyncRetriever {
            schemas: AHashMap::from_iter([
                (
                    "http://example.com/address".to_string(),
                    json!({
                        "type": "object",
                        "properties": {
                            "street": {"type": "string"},
                            "city": {"$ref": "http://example.com/city"}
                        }
                    }),
                ),
                (
                    "http://example.com/city".to_string(),
                    json!({
                        "type": "string",
                        "minLength": 1
                    }),
                ),
            ]),
        };

        let registry = Registry::options()
            .async_retriever(retriever)
            .build([(
                "http://example.com/person",
                Resource::from_contents(json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "address": {"$ref": "http://example.com/address"}
                    }
                })),
            )])
            .await
            .expect("Invalid resource");

        let resolver = registry.try_resolver("").expect("Invalid base URI");

        // Verify nested reference resolution
        let resolved = resolver
            .lookup("http://example.com/city")
            .expect("Lookup failed");
        assert_eq!(
            resolved.contents(),
            &json!({"type": "string", "minLength": 1})
        );
    }
}
