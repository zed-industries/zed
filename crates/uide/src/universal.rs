//! Universal data model that handles all data types in UIDE

use chrono::{DateTime, Duration, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for records
pub type RecordId = Uuid;

/// Universal record that can store any data type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRecord {
    pub id: RecordId,
    pub timestamp: DateTime<Utc>,
    pub data_type: DataType,
    pub content: UniversalContent,
    pub metadata: RecordMetadata,
    pub relationships: Vec<Relationship>,
}

impl UniversalRecord {
    pub fn new(data_type: DataType, content: UniversalContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            data_type,
            content,
            metadata: RecordMetadata::default(),
            relationships: Vec::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: RecordMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_relationships(mut self, relationships: Vec<Relationship>) -> Self {
        self.relationships = relationships;
        self
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    /// Check if this record matches a data type
    pub fn is_type(&self, data_type: &DataType) -> bool {
        &self.data_type == data_type
    }

    /// Get size estimate for storage optimization
    pub fn size_estimate(&self) -> usize {
        match &self.content {
            UniversalContent::Vector { values, .. } => values.len() * 4, // f32 = 4 bytes
            UniversalContent::Document { text, .. } => text.len(),
            UniversalContent::Binary { data, .. } => data.len(),
            UniversalContent::Structured { fields, .. } => {
                fields.iter().map(|(k, v)| k.len() + v.size_estimate()).sum()
            }
            UniversalContent::TimeSeries { values, .. } => values.len() * 16, // Rough estimate
            UniversalContent::Graph { properties, .. } => {
                properties.iter().map(|(k, v)| k.len() + v.size_estimate()).sum()
            }
        }
    }
}

/// Data type classification for optimization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    /// AI Knowledge (patterns, insights, preferences)
    Knowledge,
    /// Vector embeddings
    Vector,
    /// Text documents
    Document,
    /// Structured data (JSON-like)
    Structured,
    /// Binary data (models, files)
    Binary,
    /// Time series data
    TimeSeries,
    /// Graph nodes/edges
    Graph,
    /// Custom user-defined types
    Custom(String),
}

/// Universal content that can represent any data type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniversalContent {
    /// Vector data (embeddings, features)
    Vector {
        dimensions: usize,
        values: Vec<f32>,
        encoding: VectorEncoding,
    },

    /// Document/text data
    Document {
        text: String,
        tokens: Option<Vec<String>>,
        language: Option<String>,
    },

    /// Structured data (JSON-like)
    Structured {
        fields: IndexMap<String, Value>,
        schema: Option<Schema>,
    },

    /// Binary data (models, files)
    Binary {
        data: Vec<u8>,
        format: BinaryFormat,
        compression: CompressionType,
    },

    /// Time series data
    TimeSeries {
        values: Vec<TimeSeriesPoint>,
        interval: Duration,
        aggregation: AggregationType,
    },

    /// Graph node/edge
    Graph {
        node_type: Option<String>,
        edge_type: Option<String>,
        properties: HashMap<String, Value>,
    },
}

impl UniversalContent {
    /// Get the primary searchable text for this content
    pub fn searchable_text(&self) -> Option<String> {
        match self {
            UniversalContent::Document { text, .. } => Some(text.clone()),
            UniversalContent::Structured { fields, .. } => {
                let text_parts: Vec<String> = fields
                    .iter()
                    .filter_map(|(_, v)| v.as_text())
                    .collect();
                if text_parts.is_empty() { None } else { Some(text_parts.join(" ")) }
            }
            UniversalContent::Graph { properties, .. } => {
                let text_parts: Vec<String> = properties
                    .iter()
                    .filter_map(|(_, v)| v.as_text())
                    .collect();
                if text_parts.is_empty() { None } else { Some(text_parts.join(" ")) }
            }
            _ => None,
        }
    }

    /// Get vector representation if available
    pub fn vector(&self) -> Option<&Vec<f32>> {
        match self {
            UniversalContent::Vector { values, .. } => Some(values),
            _ => None,
        }
    }
}

/// Generic value type for structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
    Binary(Vec<u8>),
}

impl Value {
    pub fn as_text(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }

    pub fn size_estimate(&self) -> usize {
        match self {
            Value::Null => 0,
            Value::Bool(_) => 1,
            Value::Number(_) => 8,
            Value::String(s) => s.len(),
            Value::Array(arr) => arr.iter().map(|v| v.size_estimate()).sum(),
            Value::Object(obj) => obj.iter().map(|(k, v)| k.len() + v.size_estimate()).sum(),
            Value::Binary(data) => data.len(),
        }
    }
}

/// Vector encoding types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorEncoding {
    Float32,
    Float16,
    Int8,
    Binary,
}

/// Binary format types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryFormat {
    Raw,
    Safetensors,
    Onnx,
    Pytorch,
    TensorFlow,
    Custom(String),
}

/// Compression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    Gzip,
    Lz4,
    Zstd,
}

/// Time series point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub metadata: Option<HashMap<String, Value>>,
}

/// Time series aggregation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationType {
    Raw,
    Average,
    Sum,
    Min,
    Max,
    Count,
}

/// Schema for structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub version: String,
    pub fields: HashMap<String, FieldSchema>,
}

/// Field schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub field_type: String,
    pub required: bool,
    pub description: Option<String>,
}

/// Metadata associated with records
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordMetadata {
    pub tags: Vec<String>,
    pub source: Option<String>,
    pub confidence: Option<f64>,
    pub version: Option<String>,
    pub custom: HashMap<String, Value>,
}

impl RecordMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = Some(confidence);
        self
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn set_custom(&mut self, key: String, value: Value) {
        self.custom.insert(key, value);
    }
}

/// Relationship between records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: RecordId,
    pub relationship_type: String,
    pub strength: f64,
    pub bidirectional: bool,
    pub metadata: HashMap<String, Value>,
}

impl Relationship {
    pub fn new(
        target_id: RecordId,
        relationship_type: String,
        strength: f64,
    ) -> Self {
        Self {
            target_id,
            relationship_type,
            strength,
            bidirectional: false,
            metadata: HashMap::new(),
        }
    }

    pub fn bidirectional(mut self) -> Self {
        self.bidirectional = true;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, Value>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_metadata(&mut self, key: String, value: Value) {
        self.metadata.insert(key, value);
    }
}

/// Builder for structured content
pub struct StructuredBuilder {
    fields: IndexMap<String, Value>,
    schema: Option<Schema>,
}

impl StructuredBuilder {
    pub fn new() -> Self {
        Self {
            fields: IndexMap::new(),
            schema: None,
        }
    }

    pub fn field(mut self, key: impl ToString, value: Value) -> Self {
        self.fields.insert(key.to_string(), value);
        self
    }

    pub fn text_field(self, key: impl ToString, value: impl ToString) -> Self {
        self.field(key, Value::String(value.to_string()))
    }

    pub fn number_field(self, key: impl ToString, value: f64) -> Self {
        self.field(key, Value::Number(value))
    }

    pub fn bool_field(self, key: impl ToString, value: bool) -> Self {
        self.field(key, Value::Bool(value))
    }

    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn build(self) -> UniversalContent {
        UniversalContent::Structured {
            fields: self.fields,
            schema: self.schema,
        }
    }
}

impl Default for StructuredBuilder {
    fn default() -> Self {
        Self::new()
    }
} 