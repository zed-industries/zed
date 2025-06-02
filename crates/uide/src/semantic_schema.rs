//! Semantic Schema System for UIDE
//! 
//! Provides runtime schema definitions with voice awareness, AI descriptions,
//! and automatic UI generation capabilities.

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{universal::{Value, UniversalContent, UniversalRecord}, DataType};

pub type SchemaId = Uuid;

/// A semantic schema that defines entity structure with AI-friendly metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSchema {
    pub id: SchemaId,
    pub name: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Core schema definition
    pub fields: IndexMap<String, FieldDefinition>,
    pub display_config: DisplayConfig,
    
    // AI & Voice metadata
    pub semantic_metadata: SemanticMetadata,
    pub voice_commands: VoiceCommandConfig,
    
    // Relationships
    pub relationships: Vec<SchemaRelationship>,
}

/// Field definition with semantic and UI information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub default_value: Option<Value>,
    
    // Display configuration
    pub display_name: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub widget_config: WidgetConfig,
    
    // Validation
    pub validation: ValidationConfig,
    
    // Semantic metadata for AI understanding
    pub semantic_tags: Vec<String>,
    pub ai_description: Option<String>,
    pub voice_aliases: Vec<String>,
}

/// Field type system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    // Basic types
    Text { max_length: Option<usize> },
    Number { min: Option<f64>, max: Option<f64> },
    Boolean,
    Date,
    DateTime,
    
    // Complex types
    Array { item_type: Box<FieldType> },
    Object { schema: SchemaId },
    Reference { target_schema: SchemaId },
    
    // Specialized types
    Email,
    Url,
    Json,
    ApiKey,
    FilePath,
    
    // AI/ML types
    Vector { dimensions: Option<usize> },
    Embedding { model: Option<String> },
}

/// UI widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetConfig {
    TextInput { multiline: bool },
    NumberInput { step: Option<f64> },
    Checkbox,
    Select { options: Vec<SelectOption> },
    DatePicker,
    FileUpload,
    ColorPicker,
    Slider { min: f64, max: f64, step: f64 },
    Custom { widget_type: String, config: HashMap<String, Value> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: Value,
    pub label: String,
    pub description: Option<String>,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationConfig {
    pub pattern: Option<String>,
    pub custom_validator: Option<String>,
    pub error_message: Option<String>,
}

/// Display configuration for lists and forms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub list_view: ListViewConfig,
    pub form_view: FormViewConfig,
    pub detail_view: DetailViewConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListViewConfig {
    pub columns: Vec<ColumnConfig>,
    pub sort_by: Option<String>,
    pub sort_order: SortOrder,
    pub page_size: usize,
    pub search_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub field: String,
    pub width: Option<u32>,
    pub sortable: bool,
    pub filterable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormViewConfig {
    pub layout: FormLayout,
    pub sections: Vec<FormSection>,
    pub submit_button_text: String,
    pub cancel_button_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormLayout {
    Vertical,
    Horizontal,
    Grid { columns: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSection {
    pub title: String,
    pub fields: Vec<String>,
    pub collapsible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailViewConfig {
    pub sections: Vec<DetailSection>,
    pub actions: Vec<ActionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailSection {
    pub title: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub name: String,
    pub label: String,
    pub icon: Option<String>,
    pub style: ActionStyle,
    pub confirmation_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionStyle {
    Primary,
    Secondary,
    Danger,
    Success,
}

/// Semantic metadata for AI understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMetadata {
    pub purpose: String,
    pub domain: Vec<String>,
    pub ai_description: String,
    pub synonyms: Vec<String>,
    pub related_concepts: Vec<String>,
    pub usage_examples: Vec<String>,
}

/// Voice command configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommandConfig {
    pub entity_names: Vec<String>,
    pub action_mappings: HashMap<String, ActionMetadata>,
    pub context_patterns: Vec<ContextPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMetadata {
    pub voice_triggers: Vec<String>,
    pub required_parameters: Vec<ParameterMetadata>,
    pub optional_parameters: Vec<ParameterMetadata>,
    pub confirmation_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterMetadata {
    pub name: String,
    pub field_path: String,
    pub voice_names: Vec<String>,
    pub extraction_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPattern {
    pub pattern: String,
    pub intent: String,
    pub confidence_threshold: f64,
}

/// Schema relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaRelationship {
    pub name: String,
    pub target_schema: SchemaId,
    pub relationship_type: RelationshipType,
    pub cardinality: Cardinality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany,
    Composition,
    Aggregation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cardinality {
    Required,
    Optional,
    Multiple,
}

impl SemanticSchema {
    /// Create a new semantic schema
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            version: "1.0.0".to_string(),
            created_at: now,
            updated_at: now,
            fields: IndexMap::new(),
            display_config: DisplayConfig::default(),
            semantic_metadata: SemanticMetadata {
                purpose: purpose.into(),
                domain: Vec::new(),
                ai_description: String::new(),
                synonyms: Vec::new(),
                related_concepts: Vec::new(),
                usage_examples: Vec::new(),
            },
            voice_commands: VoiceCommandConfig {
                entity_names: Vec::new(),
                action_mappings: HashMap::new(),
                context_patterns: Vec::new(),
            },
            relationships: Vec::new(),
        }
    }

    /// Add a field to the schema
    pub fn add_field(mut self, field: FieldDefinition) -> Self {
        self.fields.insert(field.name.clone(), field);
        self.updated_at = Utc::now();
        self
    }

    /// Add semantic metadata
    pub fn with_ai_description(mut self, description: impl Into<String>) -> Self {
        self.semantic_metadata.ai_description = description.into();
        self
    }

    /// Add voice command names
    pub fn with_voice_names(mut self, names: Vec<String>) -> Self {
        self.voice_commands.entity_names = names;
        self
    }

    /// Convert data to this schema format
    pub fn create_record(&self, data: HashMap<String, Value>) -> crate::Result<UniversalRecord> {
        // Validate data against schema
        self.validate_data(&data)?;
        
        let content = UniversalContent::Structured {
            fields: data.into_iter().collect(),
            schema: Some(crate::universal::Schema {
                version: self.version.clone(),
                fields: self.fields.iter().map(|(name, def)| {
                    (name.clone(), crate::universal::FieldSchema {
                        field_type: def.field_type.to_string(),
                        required: def.required,
                        description: def.description.clone(),
                    })
                }).collect(),
            }),
        };

        Ok(UniversalRecord::new(DataType::Structured, content))
    }

    /// Validate data against this schema
    pub fn validate_data(&self, data: &HashMap<String, Value>) -> crate::Result<()> {
        for (field_name, field_def) in &self.fields {
            if field_def.required && !data.contains_key(field_name) {
                return Err(crate::error::UideError::invalid_query(
                    format!("Required field '{}' is missing", field_name)
                ));
            }
            
            if let Some(value) = data.get(field_name) {
                self.validate_field_value(field_def, value)?;
            }
        }
        Ok(())
    }

    fn validate_field_value(&self, field_def: &FieldDefinition, value: &Value) -> crate::Result<()> {
        // Basic type validation
        match (&field_def.field_type, value) {
            (FieldType::Text { max_length }, Value::String(s)) => {
                if let Some(max) = max_length {
                    if s.len() > *max {
                        return Err(crate::error::UideError::invalid_query(
                            format!("Text field '{}' exceeds maximum length of {}", field_def.name, max)
                        ));
                    }
                }
            }
            (FieldType::Number { min, max }, Value::Number(n)) => {
                if let Some(min_val) = min {
                    if *n < *min_val {
                        return Err(crate::error::UideError::invalid_query(
                            format!("Number field '{}' is below minimum value of {}", field_def.name, min_val)
                        ));
                    }
                }
                if let Some(max_val) = max {
                    if *n > *max_val {
                        return Err(crate::error::UideError::invalid_query(
                            format!("Number field '{}' exceeds maximum value of {}", field_def.name, max_val)
                        ));
                    }
                }
            }
            (FieldType::Boolean, Value::Bool(_)) => {} // Valid
            _ => {} // Additional validation can be added here
        }
        Ok(())
    }

    /// Get fields that are searchable by voice
    pub fn get_voice_searchable_fields(&self) -> Vec<&FieldDefinition> {
        self.fields.values()
            .filter(|field| !field.voice_aliases.is_empty() || field.semantic_tags.contains(&"searchable".to_string()))
            .collect()
    }
}

impl FieldType {
    fn to_string(&self) -> String {
        match self {
            FieldType::Text { .. } => "text".to_string(),
            FieldType::Number { .. } => "number".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Date => "date".to_string(),
            FieldType::DateTime => "datetime".to_string(),
            FieldType::Array { .. } => "array".to_string(),
            FieldType::Object { .. } => "object".to_string(),
            FieldType::Reference { .. } => "reference".to_string(),
            FieldType::Email => "email".to_string(),
            FieldType::Url => "url".to_string(),
            FieldType::Json => "json".to_string(),
            FieldType::ApiKey => "apikey".to_string(),
            FieldType::FilePath => "filepath".to_string(),
            FieldType::Vector { .. } => "vector".to_string(),
            FieldType::Embedding { .. } => "embedding".to_string(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            list_view: ListViewConfig {
                columns: Vec::new(),
                sort_by: None,
                sort_order: SortOrder::Ascending,
                page_size: 50,
                search_fields: Vec::new(),
            },
            form_view: FormViewConfig {
                layout: FormLayout::Vertical,
                sections: Vec::new(),
                submit_button_text: "Save".to_string(),
                cancel_button_text: "Cancel".to_string(),
            },
            detail_view: DetailViewConfig {
                sections: Vec::new(),
                actions: Vec::new(),
            },
        }
    }
}

impl Default for WidgetConfig {
    fn default() -> Self {
        WidgetConfig::TextInput { multiline: false }
    }
} 