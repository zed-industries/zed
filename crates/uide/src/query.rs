//! Universal query system for UIDE

use crate::{
    error::{Result, UideError},
    universal::{DataType, RecordId, UniversalRecord},
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal query that can combine multiple search strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalQuery {
    /// What to find
    pub target: QueryTarget,
    
    /// Filters to apply
    pub filters: Vec<Filter>,
    
    /// Vector similarity search
    pub similarity: Option<SimilarityQuery>,
    
    /// Text search
    pub text: Option<TextQuery>,
    
    /// Graph traversal
    pub graph: Option<GraphQuery>,
    
    /// Result preferences
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: Option<SortCriteria>,
}

impl UniversalQuery {
    /// Create a new query builder
    pub fn builder() -> UniversalQueryBuilder {
        UniversalQueryBuilder::new()
    }

    /// Simple query by ID
    pub fn by_id(id: RecordId) -> Self {
        Self {
            target: QueryTarget::ById(id),
            filters: Vec::new(),
            similarity: None,
            text: None,
            graph: None,
            limit: None,
            offset: None,
            sort: None,
        }
    }

    /// Simple query by data type
    pub fn by_type(data_type: DataType) -> Self {
        Self {
            target: QueryTarget::ByType(data_type),
            filters: Vec::new(),
            similarity: None,
            text: None,
            graph: None,
            limit: Some(100), // Default limit for type queries
            offset: None,
            sort: None,
        }
    }

    /// Simple text search
    pub fn text_search(query: impl ToString) -> Self {
        Self {
            target: QueryTarget::All,
            filters: Vec::new(),
            similarity: None,
            text: Some(TextQuery {
                query: query.to_string(),
                boost: 1.0,
                fuzzy: false,
                phrase: false,
            }),
            graph: None,
            limit: Some(50),
            offset: None,
            sort: Some(SortCriteria::Relevance),
        }
    }

    /// Simple vector similarity search
    pub fn vector_search(vector: Vec<f32>, threshold: f64) -> Self {
        Self {
            target: QueryTarget::All,
            filters: Vec::new(),
            similarity: Some(SimilarityQuery {
                vector,
                threshold,
                metric: SimilarityMetric::Cosine,
            }),
            text: None,
            graph: None,
            limit: Some(10),
            offset: None,
            sort: Some(SortCriteria::Similarity),
        }
    }

    /// Validate the query
    pub fn validate(&self) -> Result<()> {
        // Check if we have at least one search criteria
        let has_criteria = matches!(self.target, QueryTarget::ById(_) | QueryTarget::ByType(_) | QueryTarget::ByContent(_) | QueryTarget::ByVector(_) | QueryTarget::ByRelationship { .. })
            || self.similarity.is_some()
            || self.text.is_some()
            || self.graph.is_some()
            || !self.filters.is_empty();

        if !has_criteria {
            return Err(UideError::invalid_query(
                "Query must have at least one search criterion"
            ));
        }

        // Validate vector dimensions if present
        if let Some(ref sim) = self.similarity {
            if sim.vector.is_empty() {
                return Err(UideError::invalid_query(
                    "Similarity query vector cannot be empty"
                ));
            }
            if sim.threshold < 0.0 || sim.threshold > 1.0 {
                return Err(UideError::invalid_query(
                    "Similarity threshold must be between 0.0 and 1.0"
                ));
            }
        }

        // Validate text query
        if let Some(ref text) = self.text {
            if text.query.trim().is_empty() {
                return Err(UideError::invalid_query(
                    "Text query cannot be empty"
                ));
            }
        }

        Ok(())
    }
}

/// Query target specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryTarget {
    /// Find by specific ID
    ById(RecordId),
    /// Find by data type
    ByType(DataType),
    /// Find by content (text search)
    ByContent(String),
    /// Find by vector similarity
    ByVector(Vec<f32>),
    /// Find by relationship
    ByRelationship { from: RecordId, relation_type: String },
    /// Find all (with other criteria)
    All,
    /// Custom SQL-like query
    Custom(String),
}

/// Filter operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Filter {
    /// Equal to value
    Equals(String, FilterValue),
    /// Not equal to value
    NotEquals(String, FilterValue),
    /// Greater than value
    GreaterThan(String, FilterValue),
    /// Less than value
    LessThan(String, FilterValue),
    /// Contains text
    Contains(String, String),
    /// Within time range
    Within(String, Duration),
    /// Has tag
    HasTag(String),
    /// In list of values
    In(String, Vec<FilterValue>),
    /// Field exists
    Exists(String),
}

/// Values for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValue {
    String(String),
    Number(f64),
    Bool(bool),
    DateTime(DateTime<Utc>),
}

/// Vector similarity query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityQuery {
    pub vector: Vec<f32>,
    pub threshold: f64,
    pub metric: SimilarityMetric,
}

/// Similarity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityMetric {
    Cosine,
    Euclidean,
    DotProduct,
    Manhattan,
}

/// Text search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextQuery {
    pub query: String,
    pub boost: f64,
    pub fuzzy: bool,
    pub phrase: bool,
}

/// Graph traversal query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQuery {
    pub start_nodes: Vec<RecordId>,
    pub max_depth: usize,
    pub relation_types: Vec<String>,
    pub direction: GraphDirection,
}

/// Graph traversal direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphDirection {
    Outgoing,
    Incoming,
    Both,
}

/// Sort criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortCriteria {
    /// Sort by timestamp (newest first)
    Timestamp,
    /// Sort by relevance score
    Relevance,
    /// Sort by similarity score
    Similarity,
    /// Sort by custom field
    Field(String, SortOrder),
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults<T = UniversalRecord> {
    pub results: Vec<SearchResult<T>>,
    pub total_count: Option<usize>,
    pub query_time_ms: u64,
    pub strategies_used: Vec<String>,
}

/// Individual search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T = UniversalRecord> {
    pub record: T,
    pub score: f64,
    pub explanation: Option<String>,
    pub highlights: Vec<Highlight>,
}

/// Text highlight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub field: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// Query builder for fluent interface
pub struct UniversalQueryBuilder {
    query: UniversalQuery,
}

impl UniversalQueryBuilder {
    pub fn new() -> Self {
        Self {
            query: UniversalQuery {
                target: QueryTarget::All,
                filters: Vec::new(),
                similarity: None,
                text: None,
                graph: None,
                limit: None,
                offset: None,
                sort: None,
            },
        }
    }

    /// Set query target
    pub fn target(mut self, target: QueryTarget) -> Self {
        self.query.target = target;
        self
    }

    /// Add similarity search
    pub fn similarity(mut self, vector: Vec<f32>) -> Self {
        self.query.similarity = Some(SimilarityQuery {
            vector,
            threshold: 0.7,
            metric: SimilarityMetric::Cosine,
        });
        self
    }

    /// Add similarity search with custom threshold
    pub fn similarity_with_threshold(mut self, vector: Vec<f32>, threshold: f64) -> Self {
        self.query.similarity = Some(SimilarityQuery {
            vector,
            threshold,
            metric: SimilarityMetric::Cosine,
        });
        self
    }

    /// Add text search
    pub fn text(mut self, query: impl ToString) -> Self {
        self.query.text = Some(TextQuery {
            query: query.to_string(),
            boost: 1.0,
            fuzzy: false,
            phrase: false,
        });
        self
    }

    /// Add fuzzy text search
    pub fn fuzzy_text(mut self, query: impl ToString) -> Self {
        self.query.text = Some(TextQuery {
            query: query.to_string(),
            boost: 1.0,
            fuzzy: true,
            phrase: false,
        });
        self
    }

    /// Add graph traversal
    pub fn graph(mut self, start_nodes: Vec<RecordId>, relation_types: Vec<String>) -> Self {
        self.query.graph = Some(GraphQuery {
            start_nodes,
            max_depth: 3,
            relation_types,
            direction: GraphDirection::Both,
        });
        self
    }

    /// Add filter
    pub fn filter(mut self, filter: Filter) -> Self {
        self.query.filters.push(filter);
        self
    }

    /// Filter by data type
    pub fn filter_type(mut self, data_type: DataType) -> Self {
        self.query.target = QueryTarget::ByType(data_type);
        self
    }

    /// Filter by tag
    pub fn filter_tag(mut self, tag: impl ToString) -> Self {
        self.query.filters.push(Filter::HasTag(tag.to_string()));
        self
    }

    /// Filter by time range (records within duration from now)
    pub fn filter_recent(mut self, duration: Duration) -> Self {
        self.query.filters.push(Filter::Within("timestamp".to_string(), duration));
        self
    }

    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.query.limit = Some(limit);
        self
    }

    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.query.offset = Some(offset);
        self
    }

    /// Sort by timestamp
    pub fn sort_by_time(mut self) -> Self {
        self.query.sort = Some(SortCriteria::Timestamp);
        self
    }

    /// Sort by relevance
    pub fn sort_by_relevance(mut self) -> Self {
        self.query.sort = Some(SortCriteria::Relevance);
        self
    }

    /// Sort by similarity
    pub fn sort_by_similarity(mut self) -> Self {
        self.query.sort = Some(SortCriteria::Similarity);
        self
    }

    /// Build the query
    pub fn build(self) -> Result<UniversalQuery> {
        self.query.validate()?;
        Ok(self.query)
    }
}

impl Default for UniversalQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Query execution strategy
#[derive(Debug, Clone)]
pub enum SearchStrategy {
    /// Direct ID lookup
    DirectId,
    /// Vector similarity search
    Vector(VectorSearchParams),
    /// Full-text search
    FullText(TextSearchParams),
    /// Structured field search
    Structural(StructuralSearchParams),
    /// Graph traversal
    Graph(GraphSearchParams),
    /// Hybrid search combining multiple strategies
    Hybrid(Vec<SearchStrategy>),
}

/// Vector search parameters
#[derive(Debug, Clone)]
pub struct VectorSearchParams {
    pub metric: SimilarityMetric,
    pub threshold: f64,
    pub use_index: bool,
}

/// Text search parameters
#[derive(Debug, Clone)]
pub struct TextSearchParams {
    pub fuzzy: bool,
    pub phrase: bool,
    pub boost: f64,
}

/// Structural search parameters
#[derive(Debug, Clone, Default)]
pub struct StructuralSearchParams {
    pub field_boosts: HashMap<String, f64>,
    pub exact_match: bool,
}

/// Graph search parameters
#[derive(Debug, Clone)]
pub struct GraphSearchParams {
    pub max_depth: usize,
    pub direction: GraphDirection,
    pub weight_threshold: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let query = UniversalQuery::builder()
            .text("test query")
            .filter_tag("important")
            .limit(10)
            .sort_by_relevance()
            .build()
            .unwrap();

        assert!(query.text.is_some());
        assert_eq!(query.filters.len(), 1);
        assert_eq!(query.limit, Some(10));
        assert!(matches!(query.sort, Some(SortCriteria::Relevance)));
    }

    #[test]
    fn test_vector_query() {
        let vector = vec![1.0, 2.0, 3.0];
        let query = UniversalQuery::vector_search(vector.clone(), 0.8);

        assert!(query.similarity.is_some());
        let sim = query.similarity.unwrap();
        assert_eq!(sim.vector, vector);
        assert_eq!(sim.threshold, 0.8);
    }

    #[test]
    fn test_query_validation() {
        // Valid query
        let valid_query = UniversalQuery::text_search("test");
        assert!(valid_query.validate().is_ok());

        // Invalid query - no criteria
        let invalid_query = UniversalQuery {
            target: QueryTarget::All,
            filters: Vec::new(),
            similarity: None,
            text: None,
            graph: None,
            limit: None,
            offset: None,
            sort: None,
        };
        assert!(invalid_query.validate().is_err());
    }
} 