//! Main UIDE engine that combines all components

use crate::{
    error::{Result, UideError},
    query::{SearchResults, SearchResult, SearchStrategy, UniversalQuery, QueryTarget, Filter, SortCriteria},
    storage::{StorageEngine, StorageConfig, StorageStats},
    universal::{DataType, RecordId, UniversalRecord, UniversalContent, Value},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Main UIDE engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub storage: StorageConfig,
    pub enable_indexing: bool,
    pub max_cache_size: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            storage: StorageConfig::default(),
            enable_indexing: true,
            max_cache_size: 1000,
        }
    }
}

/// The unified data engine
pub struct UnifiedDataEngine {
    storage: Arc<StorageEngine>,
    basic_index: Arc<RwLock<BasicIndex>>,
    config: EngineConfig,
}

impl UnifiedDataEngine {
    /// Create a new engine with default configuration
    pub async fn new(data_path: impl ToString) -> Result<Self> {
        let config = EngineConfig {
            storage: StorageConfig {
                path: data_path.to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        Self::with_config(config).await
    }

    /// Create a new engine with custom configuration
    pub async fn with_config(config: EngineConfig) -> Result<Self> {
        let storage = Arc::new(StorageEngine::new(config.storage.clone()).await?);
        let basic_index = Arc::new(RwLock::new(BasicIndex::new()));

        info!("UIDE engine initialized");

        Ok(Self {
            storage,
            basic_index,
            config,
        })
    }

    /// Store any serializable data
    pub async fn store<T: Serialize>(&self, data: T) -> Result<RecordId> {
        // Convert to universal record
        let record = self.to_universal_record(data).await?;
        let id = record.id;

        // Store in storage engine
        self.storage.store_record(&record).await?;

        // Update indexes if enabled
        if self.config.enable_indexing {
            self.update_index(&record).await?;
        }

        debug!("Stored record with ID: {}", id);
        Ok(id)
    }

    /// Store a pre-built universal record
    pub async fn store_record(&self, record: UniversalRecord) -> Result<RecordId> {
        let id = record.id;
        
        // Store in storage engine
        self.storage.store_record(&record).await?;

        // Update indexes if enabled
        if self.config.enable_indexing {
            self.update_index(&record).await?;
        }

        debug!("Stored universal record with ID: {}", id);
        Ok(id)
    }

    /// Retrieve data by ID and convert to desired type
    pub async fn retrieve<T: DeserializeOwned>(&self, id: RecordId) -> Result<Option<T>> {
        match self.storage.get_record(id).await? {
            Some(record) => {
                let data = self.from_universal_record(record).await?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    /// Retrieve raw universal record by ID
    pub async fn get_record(&self, id: RecordId) -> Result<Option<UniversalRecord>> {
        self.storage.get_record(id).await
    }

    /// Delete a record
    pub async fn delete(&self, id: RecordId) -> Result<bool> {
        let deleted = self.storage.delete_record(id).await?;
        
        if deleted && self.config.enable_indexing {
            self.remove_from_index(id).await?;
        }

        Ok(deleted)
    }

    /// Universal search interface
    pub async fn search(&self, query: UniversalQuery) -> Result<SearchResults> {
        let start_time = Instant::now();
        
        // Validate query
        query.validate()?;

        // Determine search strategy
        let strategy = self.determine_search_strategy(&query).await?;
        let strategy_name = format!("{:?}", strategy);
        
        // Execute search based on strategy
        let mut results = match strategy {
            SearchStrategy::DirectId => self.search_by_id(&query).await?,
            SearchStrategy::Vector(_) => self.vector_search(&query).await?,
            SearchStrategy::FullText(_) => self.text_search(&query).await?,
            SearchStrategy::Structural(_) => self.structural_search(&query).await?,
            SearchStrategy::Graph(_) => self.graph_search(&query).await?,
            SearchStrategy::Hybrid(ref strategies) => self.hybrid_search(&query, strategies.clone()).await?,
        };

        // Apply filters
        results = self.apply_filters(results, &query.filters).await?;

        // Sort results
        results = self.sort_results(results, &query.sort).await?;

        // Apply pagination
        results = self.paginate_results(results, query.offset, query.limit).await?;

        let query_time = start_time.elapsed().as_millis() as u64;
        
        Ok(SearchResults {
            results,
            total_count: None, // TODO: Implement total count
            query_time_ms: query_time,
            strategies_used: vec![strategy_name],
        })
    }

    /// Get engine statistics
    pub async fn stats(&self) -> Result<EngineStats> {
        let storage_stats = self.storage.get_stats().await?;
        let index_stats = self.basic_index.read().await.stats();

        Ok(EngineStats {
            storage: storage_stats,
            index: index_stats,
        })
    }

    // Private helper methods...
    async fn to_universal_record<T: Serialize>(&self, data: T) -> Result<UniversalRecord> {
        // For now, serialize as JSON and store as structured data
        let json_value = serde_json::to_value(&data)?;
        let fields = self.json_to_fields(json_value)?;

        let content = UniversalContent::Structured {
            fields,
            schema: None,
        };

        Ok(UniversalRecord::new(DataType::Structured, content))
    }

    async fn from_universal_record<T: DeserializeOwned>(&self, record: UniversalRecord) -> Result<T> {
        match record.content {
            UniversalContent::Structured { fields, .. } => {
                let json_value = self.fields_to_json(fields)?;
                let data = serde_json::from_value(json_value)?;
                Ok(data)
            }
            _ => Err(UideError::type_conversion(
                "Cannot convert non-structured content to arbitrary type"
            )),
        }
    }

    async fn determine_search_strategy(&self, query: &UniversalQuery) -> Result<SearchStrategy> {
        match &query.target {
            QueryTarget::ById(_) => Ok(SearchStrategy::DirectId),
            _ => {
                let has_vector = query.similarity.is_some();
                let has_text = query.text.is_some();
                let has_graph = query.graph.is_some();

                match (has_vector, has_text, has_graph) {
                    (true, false, false) => Ok(SearchStrategy::Vector(Default::default())),
                    (false, true, false) => Ok(SearchStrategy::FullText(Default::default())),
                    (false, false, true) => Ok(SearchStrategy::Graph(Default::default())),
                    (false, false, false) => Ok(SearchStrategy::Structural(Default::default())),
                    _ => {
                        let mut strategies = Vec::new();
                        if has_vector { strategies.push(SearchStrategy::Vector(Default::default())); }
                        if has_text { strategies.push(SearchStrategy::FullText(Default::default())); }
                        if has_graph { strategies.push(SearchStrategy::Graph(Default::default())); }
                        Ok(SearchStrategy::Hybrid(strategies))
                    }
                }
            }
        }
    }

    async fn search_by_id(&self, query: &UniversalQuery) -> Result<Vec<SearchResult>> {
        if let QueryTarget::ById(id) = &query.target {
            match self.storage.get_record(*id).await? {
                Some(record) => Ok(vec![SearchResult {
                    record,
                    score: 1.0,
                    explanation: Some("Direct ID match".to_string()),
                    highlights: Vec::new(),
                }]),
                None => Ok(Vec::new()),
            }
        } else {
            Ok(Vec::new())
        }
    }

    async fn vector_search(&self, query: &UniversalQuery) -> Result<Vec<SearchResult>> {
        let similarity_query = match &query.similarity {
            Some(sim) => sim,
            None => return Ok(Vec::new()),
        };

        let all_records = self.storage.list_records(None).await?;
        let mut scored_results = Vec::new();

        for record in all_records {
            if let Some(record_vector) = record.content.vector() {
                let similarity = self.compute_similarity(&similarity_query.vector, record_vector)?;
                
                if similarity >= similarity_query.threshold {
                    scored_results.push(SearchResult {
                        record,
                        score: similarity,
                        explanation: Some(format!("Vector similarity: {:.3}", similarity)),
                        highlights: Vec::new(),
                    });
                }
            }
        }

        scored_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scored_results)
    }

    async fn text_search(&self, query: &UniversalQuery) -> Result<Vec<SearchResult>> {
        let text_query = match &query.text {
            Some(text) => text,
            None => return Ok(Vec::new()),
        };

        let search_terms: Vec<&str> = text_query.query.split_whitespace().collect();
        let all_records = self.storage.list_records(None).await?;
        let mut scored_results = Vec::new();

        for record in all_records {
            if let Some(text) = record.content.searchable_text() {
                let text_lower = text.to_lowercase();
                let mut score = 0.0;
                let mut highlights = Vec::new();

                for term in &search_terms {
                    let term_lower = term.to_lowercase();
                    if text_lower.contains(&term_lower) {
                        score += 1.0 / search_terms.len() as f64;
                        
                        if let Some(pos) = text_lower.find(&term_lower) {
                            highlights.push(crate::query::Highlight {
                                field: "text".to_string(),
                                text: term.to_string(),
                                start: pos,
                                end: pos + term.len(),
                            });
                        }
                    }
                }

                if score > 0.0 {
                    scored_results.push(SearchResult {
                        record,
                        score,
                        explanation: Some(format!("Text match score: {:.3}", score)),
                        highlights,
                    });
                }
            }
        }

        scored_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scored_results)
    }

    async fn structural_search(&self, query: &UniversalQuery) -> Result<Vec<SearchResult>> {
        let all_records = self.storage.list_records(None).await?;
        let mut results = Vec::new();

        for record in all_records {
            if let QueryTarget::ByType(ref data_type) = query.target {
                if &record.data_type != data_type {
                    continue;
                }
            }

            results.push(SearchResult {
                record,
                score: 1.0,
                explanation: Some("Structural match".to_string()),
                highlights: Vec::new(),
            });
        }

        Ok(results)
    }

    async fn graph_search(&self, _query: &UniversalQuery) -> Result<Vec<SearchResult>> {
        warn!("Graph search not yet implemented");
        Ok(Vec::new())
    }

    async fn hybrid_search(&self, query: &UniversalQuery, strategies: Vec<SearchStrategy>) -> Result<Vec<SearchResult>> {
        let mut all_results: HashMap<RecordId, SearchResult> = HashMap::new();

        for strategy in strategies {
            let strategy_results = match strategy {
                SearchStrategy::Vector(_) => self.vector_search(query).await?,
                SearchStrategy::FullText(_) => self.text_search(query).await?,
                SearchStrategy::Structural(_) => self.structural_search(query).await?,
                SearchStrategy::Graph(_) => self.graph_search(query).await?,
                _ => continue,
            };

            for result in strategy_results {
                let id = result.record.id;
                if let Some(existing) = all_results.get_mut(&id) {
                    existing.score = (existing.score + result.score) / 2.0;
                } else {
                    all_results.insert(id, result);
                }
            }
        }

        let mut final_results: Vec<SearchResult> = all_results.into_values().collect();
        final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        Ok(final_results)
    }

    async fn apply_filters(&self, results: Vec<SearchResult>, filters: &[Filter]) -> Result<Vec<SearchResult>> {
        if filters.is_empty() {
            return Ok(results);
        }

        let mut filtered_results = Vec::new();
        for result in results {
            let mut include = true;
            for filter in filters {
                if !self.record_matches_filter(&result.record, filter)? {
                    include = false;
                    break;
                }
            }
            if include {
                filtered_results.push(result);
            }
        }

        Ok(filtered_results)
    }

    fn record_matches_filter(&self, record: &UniversalRecord, filter: &Filter) -> Result<bool> {
        match filter {
            Filter::HasTag(tag) => Ok(record.metadata.tags.contains(tag)),
            Filter::Within(field, duration) => {
                if field == "timestamp" {
                    let now = chrono::Utc::now();
                    let cutoff = now - *duration;
                    Ok(record.timestamp > cutoff)
                } else {
                    Ok(true) // For now, pass through
                }
            }
            _ => Ok(true), // TODO: Implement other filter types
        }
    }

    async fn sort_results(&self, mut results: Vec<SearchResult>, sort: &Option<SortCriteria>) -> Result<Vec<SearchResult>> {
        if let Some(criteria) = sort {
            match criteria {
                SortCriteria::Similarity | SortCriteria::Relevance => {
                    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
                }
                SortCriteria::Timestamp => {
                    results.sort_by(|a, b| b.record.timestamp.cmp(&a.record.timestamp));
                }
                _ => {} // TODO: Implement field-based sorting
            }
        }
        Ok(results)
    }

    async fn paginate_results(&self, results: Vec<SearchResult>, offset: Option<usize>, limit: Option<usize>) -> Result<Vec<SearchResult>> {
        let start = offset.unwrap_or(0);
        let end = match limit {
            Some(limit) => std::cmp::min(start + limit, results.len()),
            None => results.len(),
        };

        if start >= results.len() {
            Ok(Vec::new())
        } else {
            Ok(results[start..end].to_vec())
        }
    }

    fn compute_similarity(&self, vec1: &[f32], vec2: &[f32]) -> Result<f64> {
        if vec1.len() != vec2.len() {
            return Err(UideError::invalid_query("Vector dimensions don't match"));
        }

        let dot_product: f32 = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|a| a * a).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|a| a * a).sum::<f32>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            Ok(0.0)
        } else {
            Ok((dot_product / (norm1 * norm2)) as f64)
        }
    }

    async fn update_index(&self, record: &UniversalRecord) -> Result<()> {
        let mut index = self.basic_index.write().await;
        index.add_record(record);
        Ok(())
    }

    async fn remove_from_index(&self, id: RecordId) -> Result<()> {
        let mut index = self.basic_index.write().await;
        index.remove_record(id);
        Ok(())
    }

    fn json_to_fields(&self, value: serde_json::Value) -> Result<indexmap::IndexMap<String, Value>> {
        let mut fields = indexmap::IndexMap::new();
        
        if let serde_json::Value::Object(obj) = value {
            for (key, val) in obj {
                fields.insert(key, self.json_value_to_value(val)?);
            }
        }

        Ok(fields)
    }

    fn fields_to_json(&self, fields: indexmap::IndexMap<String, Value>) -> Result<serde_json::Value> {
        let mut obj = serde_json::Map::new();
        
        for (key, value) in fields {
            obj.insert(key, self.value_to_json_value(value)?);
        }

        Ok(serde_json::Value::Object(obj))
    }

    fn value_to_json_value(&self, value: Value) -> Result<serde_json::Value> {
        match value {
            Value::Null => Ok(serde_json::Value::Null),
            Value::Bool(b) => Ok(serde_json::Value::Bool(b)),
            Value::Number(n) => Ok(serde_json::Number::from_f64(n).map_or(serde_json::Value::Null, serde_json::Value::Number)),
            Value::String(s) => Ok(serde_json::Value::String(s)),
            Value::Array(arr) => {
                let json_arr: Result<Vec<_>> = arr.into_iter().map(|v| self.value_to_json_value(v)).collect();
                Ok(serde_json::Value::Array(json_arr?))
            }
            Value::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in obj {
                    json_obj.insert(k, self.value_to_json_value(v)?);
                }
                Ok(serde_json::Value::Object(json_obj))
            }
            Value::Binary(_) => Ok(serde_json::Value::Null),
        }
    }

    fn json_value_to_value(&self, value: serde_json::Value) -> Result<Value> {
        match value {
            serde_json::Value::Null => Ok(Value::Null),
            serde_json::Value::Bool(b) => Ok(Value::Bool(b)),
            serde_json::Value::Number(n) => Ok(Value::Number(n.as_f64().unwrap_or(0.0))),
            serde_json::Value::String(s) => Ok(Value::String(s)),
            serde_json::Value::Array(arr) => {
                let values: Result<Vec<_>> = arr.into_iter().map(|v| self.json_value_to_value(v)).collect();
                Ok(Value::Array(values?))
            }
            serde_json::Value::Object(obj) => {
                let mut value_obj = indexmap::IndexMap::new();
                for (k, v) in obj {
                    value_obj.insert(k, self.json_value_to_value(v)?);
                }
                Ok(Value::Object(value_obj))
            }
        }
    }
}

#[derive(Debug)]
struct BasicIndex {
    by_type: HashMap<DataType, Vec<RecordId>>,
    by_tag: HashMap<String, Vec<RecordId>>,
    text_index: HashMap<String, Vec<RecordId>>,
}

impl BasicIndex {
    fn new() -> Self {
        Self {
            by_type: HashMap::new(),
            by_tag: HashMap::new(),
            text_index: HashMap::new(),
        }
    }

    fn add_record(&mut self, record: &UniversalRecord) {
        self.by_type.entry(record.data_type.clone()).or_default().push(record.id);

        for tag in &record.metadata.tags {
            self.by_tag.entry(tag.clone()).or_default().push(record.id);
        }

        if let Some(text) = record.content.searchable_text() {
            for word in text.split_whitespace() {
                let word = word.to_lowercase();
                self.text_index.entry(word).or_default().push(record.id);
            }
        }
    }

    fn remove_record(&mut self, id: RecordId) {
        for ids in self.by_type.values_mut() {
            ids.retain(|&x| x != id);
        }
        for ids in self.by_tag.values_mut() {
            ids.retain(|&x| x != id);
        }
        for ids in self.text_index.values_mut() {
            ids.retain(|&x| x != id);
        }
    }

    fn stats(&self) -> IndexStats {
        IndexStats {
            types_count: self.by_type.len(),
            tags_count: self.by_tag.len(),
            text_terms_count: self.text_index.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStats {
    pub storage: StorageStats,
    pub index: IndexStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub types_count: usize,
    pub tags_count: usize,
    pub text_terms_count: usize,
}

// Default implementations for search parameters
impl Default for crate::query::VectorSearchParams {
    fn default() -> Self {
        Self {
            metric: crate::query::SimilarityMetric::Cosine,
            threshold: 0.7,
            use_index: true,
        }
    }
}

impl Default for crate::query::TextSearchParams {
    fn default() -> Self {
        Self {
            fuzzy: false,
            phrase: false,
            boost: 1.0,
        }
    }
}

impl Default for crate::query::StructuralSearchParams {
    fn default() -> Self {
        Self {
            field_boosts: HashMap::new(),
            exact_match: false,
        }
    }
}

impl Default for crate::query::GraphSearchParams {
    fn default() -> Self {
        Self {
            max_depth: 3,
            direction: crate::query::GraphDirection::Both,
            weight_threshold: 0.1,
        }
    }
} 