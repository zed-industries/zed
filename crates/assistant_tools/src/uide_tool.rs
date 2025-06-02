use std::sync::Arc;

use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult, ToolSource};
use gpui::{AnyWindowHandle, App, Entity, Task, Global};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;
use uide::{UnifiedDataEngine, UniversalQuery, DataType, universal::{UniversalRecord, UniversalContent, Value, StructuredBuilder}};
use uide::query::{SearchResult, SearchResults};
use uide::semantic_schema::{SemanticSchema, FieldDefinition, FieldType, WidgetConfig, ValidationConfig};
use uuid;
use std::sync::RwLock;

// Global UIDE engine wrapper following GPUI patterns
#[derive(Default)]
struct GlobalUideEngine(Arc<RwLock<Option<Arc<UnifiedDataEngine>>>>);

impl Global for GlobalUideEngine {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum UideToolInput {
    /// Create a new semantic schema for organizing data
    CreateSchema {
        /// Schema name
        name: String,
        /// What this schema is for
        purpose: String,
        /// Field definitions
        fields: Vec<FieldInput>,
    },
    /// List all available semantic schemas
    ListSchemas,
    /// Search for schemas using natural language
    SearchSchemas {
        /// Search query for schemas
        query: String,
        /// Maximum number of results (default: 10)
        limit: Option<usize>,
    },
    /// Create a new entity using a semantic schema
    CreateEntity {
        /// Name of schema to use
        schema_name: String,
        /// Entity data
        data: serde_json::Value,
    },
    /// Search for entities using natural language
    SearchEntities {
        /// Search query
        query: String,
        /// Maximum number of results (default: 10)
        limit: Option<usize>,
    },
    /// Delete an entity by ID
    DeleteEntity {
        /// Entity ID to delete
        entity_id: String,
    },
    /// Delete a schema and optionally its entities
    DeleteSchema {
        /// Name of schema to delete
        schema_name: String,
        /// Whether to also delete all entities using this schema (default: false)
        cascade: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FieldInput {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Whether field is required
    pub required: bool,
    /// Field description
    pub description: Option<String>,
}

pub struct UideTool;

impl Tool for UideTool {
    fn name(&self) -> String {
        "uide".into()
    }

    fn description(&self) -> String {
        "Interact with UIDE (Unified Intelligent Data Engine) to manage semantic schemas and entities. Can create schemas, manage entities, and search data using natural language.".into()
    }

    fn icon(&self) -> IconName {
        IconName::Brain
    }

    fn source(&self) -> ToolSource {
        ToolSource::Native
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn input_schema(&self, _format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        // Create a simpler, more explicit schema for better LLM understanding
        let schema = serde_json::json!({
            "type": "object",
            "title": "UIDE Tool - Multilingual Data Engine",
            "description": "UIDE (Unified Intelligent Data Engine) - manages semantic schemas and data entities.\n\n⚠️ CRITICAL: When creating entities, you MUST use the EXACT field names and types from the schema.\n\n🌍 MULTILINGUAL SUPPORT: This tool works with queries in ANY language. Common translations:\n• English: users, people, customers, products → Russian: пользователи, люди, клиенты, товары\n• Spanish: usuarios, personas, clientes, productos → French: utilisateurs, personnes, clients, produits\n• German: Benutzer, Personen, Kunden, Produkte → Chinese: 用户, 人员, 客户, 产品",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create_schema", "list_schemas", "search_schemas", "create_entity", "search_entities", "delete_entity", "delete_schema"],
                    "description": "REQUIRED: Choose the action to perform\n🌍 WORKS WITH ANY LANGUAGE: 'search_schemas' and 'search_entities' understand queries in any language"
                },
                "name": {
                    "type": "string",
                    "description": "REQUIRED for create_schema: The schema name (e.g. 'UserProfile', 'ProductCatalog')\n⚠️ Use meaningful, descriptive names"
                },
                "purpose": {
                    "type": "string", 
                    "description": "REQUIRED for create_schema: What this schema is for (e.g. 'Store user profile information')\n⚠️ Be specific about the schema's purpose"
                },
                "fields": {
                    "type": "array",
                    "description": "REQUIRED for create_schema: Array of field definitions\n⚠️ Think carefully about what fields you need and their types",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description": "Field name (e.g. 'email', 'age', 'title')\n⚠️ Use clear, descriptive field names"
                            },
                            "field_type": {
                                "type": "string",
                                "enum": ["string", "text", "number", "boolean", "date", "datetime", "email", "url", "json"],
                                "description": "Field data type\n• string: Short text (names, titles)\n• text: Long text (descriptions, content)\n• number: Integers or decimals\n• boolean: true/false values\n• date: YYYY-MM-DD format\n• datetime: ISO 8601 format (2023-10-15T14:48:00Z)\n• email: Email addresses\n• url: Web URLs\n• json: Complex nested data"
                            },
                            "required": {
                                "type": "boolean",
                                "description": "Whether this field is required\n⚠️ Required fields MUST be provided when creating entities"
                            },
                            "description": {
                                "type": "string",
                                "description": "Human-readable description of the field\n⚠️ Be descriptive to help with future searches"
                            }
                        },
                        "required": ["name", "field_type", "required"]
                    }
                },
                "schema_name": {
                    "type": "string",
                    "description": "REQUIRED for create_entity OR delete_schema: Name of existing schema to use/delete\n⚠️ CRITICAL: Must exactly match an existing schema name. Use 'search_schemas' or 'list_schemas' to see available schemas first!"
                },
                "data": {
                    "type": "object",
                    "description": "REQUIRED for create_entity: The entity data as key-value pairs\n⚠️ CRITICAL RULES:\n1. Field names MUST exactly match the schema field names\n2. All required fields MUST be included\n3. Field values MUST match the expected types\n4. Do NOT include fields that aren't in the schema\n\nExample: If schema has fields 'email' (required) and 'age' (optional), your data must use exactly 'email' and optionally 'age'"
                },
                "query": {
                    "type": "string",
                    "description": "REQUIRED for search_entities OR search_schemas: Search query in natural language\n🌍 MULTILINGUAL EXAMPLES:\n• English: 'users with age over 25', 'find john@example.com', 'user schemas'\n• Russian: 'пользователи старше 25', 'найти john@example.com', 'схемы пользователей'\n• Spanish: 'usuarios mayores de 25', 'buscar john@example.com', 'esquemas de usuario'\n• German: 'Benutzer über 25', 'finden john@example.com', 'Benutzerschemas'\n• French: 'utilisateurs de plus de 25 ans', 'trouver john@example.com', 'schémas utilisateur'"
                },
                "limit": {
                    "type": "integer",
                    "description": "OPTIONAL for search_entities OR search_schemas: Max results (default: 10)",
                    "minimum": 1,
                    "maximum": 100
                },
                "entity_id": {
                    "type": "string",
                    "description": "REQUIRED for delete_entity: UUID of entity to delete"
                },
                "cascade": {
                    "type": "boolean",
                    "description": "OPTIONAL for delete_schema: Whether to also delete all entities using this schema (default: false)\n⚠️ WARNING: Setting to true will permanently delete ALL entities that use this schema!"
                }
            },
            "required": ["action"],
            "examples": [
                {
                    "title": "🌍 MULTILINGUAL: Answer 'What users are there?' in ANY language",
                    "description": "When user asks about users/пользователи/usuarios/utilisateurs/Benutzer, search for user entities or schemas",
                    "value": {
                        "action": "search_entities",
                        "query": "users",
                        "limit": 10
                    }
                },
                {
                    "title": "🌍 MULTILINGUAL: Search for user schemas in ANY language", 
                    "description": "If no entities found, search for user-related schemas first",
                    "value": {
                        "action": "search_schemas",
                        "query": "user profile schema",
                        "limit": 5
                    }
                },
                {
                    "title": "STEP 1: Create a Schema First",
                    "description": "Always start by creating a schema that defines your data structure",
                    "value": {
                        "action": "create_schema",
                        "name": "UserProfile",
                        "purpose": "Store user profile information", 
                        "fields": [
                            {
                                "name": "email",
                                "field_type": "email",
                                "required": true,
                                "description": "User's email address"
                            },
                            {
                                "name": "username", 
                                "field_type": "string",
                                "required": true,
                                "description": "User's unique username"
                            },
                            {
                                "name": "age",
                                "field_type": "number",
                                "required": false,
                                "description": "User's age in years"
                            }
                        ]
                    }
                },
                {
                    "title": "STEP 2: List Schemas to See What's Available",
                    "description": "Check what schemas exist and their exact field names",
                    "value": {
                        "action": "list_schemas"
                    }
                },
                {
                    "title": "STEP 2b: Search for Specific Schemas",
                    "description": "Find schemas by purpose, field names, or topic",
                    "value": {
                        "action": "search_schemas",
                        "query": "user profile schemas",
                        "limit": 5
                    }
                },
                {
                    "title": "STEP 3: Create Entity with EXACT Schema Fields",
                    "description": "⚠️ CRITICAL: Use the EXACT field names from the schema. If schema has 'username' and 'email', don't use 'name' or 'user_email'",
                    "value": {
                        "action": "create_entity",
                        "schema_name": "UserProfile",
                        "data": {
                            "email": "john@example.com",
                            "username": "john_doe",
                            "age": 30
                        }
                    }
                },
                {
                    "title": "STEP 4: Search for Entities",
                    "description": "Search using natural language",
                    "value": {
                        "action": "search_entities",
                        "query": "users with email john@example.com",
                        "limit": 5
                    }
                },
                {
                    "title": "🌍 Russian Example: 'какие есть пользователи?'",
                    "description": "When asked in Russian about users, search for user entities",
                    "value": {
                        "action": "search_entities",
                        "query": "пользователи",
                        "limit": 10
                    }
                },
                {
                    "title": "🌍 Spanish Example: '¿qué usuarios hay?'",
                    "description": "When asked in Spanish about users, search for user entities",
                    "value": {
                        "action": "search_entities",
                        "query": "usuarios", 
                        "limit": 10
                    }
                },
                {
                    "title": "🌍 German Example: 'Welche Benutzer gibt es?'",
                    "description": "When asked in German about users, search for user entities",
                    "value": {
                        "action": "search_entities",
                        "query": "Benutzer",
                        "limit": 10
                    }
                },
                {
                    "title": "❌ COMMON MISTAKE - Wrong Field Names",
                    "description": "If schema has 'username' field, don't use 'name' or 'user_name'",
                    "value": {
                        "action": "create_entity",
                        "schema_name": "UserProfile", 
                        "data": {
                            "name": "Wrong! Should be 'username'",
                            "user_email": "Wrong! Should be 'email'"
                        }
                    }
                },
                {
                    "title": "✅ CORRECT - Exact Field Names",
                    "description": "Use the exact field names from the schema definition",
                    "value": {
                        "action": "create_entity",
                        "schema_name": "UserProfile",
                        "data": {
                            "username": "Correct field name",
                            "email": "Correct field name"
                        }
                    }
                },
                {
                    "title": "Delete Schema (Safe)",
                    "description": "Delete a schema without affecting existing entities that use it",
                    "value": {
                        "action": "delete_schema",
                        "schema_name": "UserProfile",
                        "cascade": false
                    }
                },
                {
                    "title": "Delete Schema and All Entities (Dangerous)",
                    "description": "⚠️ DANGEROUS: Delete a schema AND all entities that use it",
                    "value": {
                        "action": "delete_schema",
                        "schema_name": "UserProfile",
                        "cascade": true
                    }
                }
            ],
            "additionalProperties": false
        });
        
        Ok(schema)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<UideToolInput>(input.clone()) {
            Ok(UideToolInput::CreateSchema { name, .. }) => {
                format!("Create UIDE schema '{}'", name)
            }
            Ok(UideToolInput::ListSchemas) => {
                "List UIDE schemas".to_string()
            }
            Ok(UideToolInput::SearchSchemas { query, .. }) => {
                format!("Search UIDE schemas: '{}'", query)
            }
            Ok(UideToolInput::CreateEntity { schema_name, .. }) => {
                format!("Create entity in schema '{}'", schema_name)
            }
            Ok(UideToolInput::SearchEntities { query, .. }) => {
                format!("Search UIDE entities: '{}'", query)
            }
            Ok(UideToolInput::DeleteEntity { entity_id }) => {
                format!("Delete UIDE entity '{}'", entity_id)
            }
            Ok(UideToolInput::DeleteSchema { schema_name, .. }) => {
                format!("Delete UIDE schema '{}'", schema_name)
            }
            Err(_) => "UIDE operation".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: UideToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        let task = cx.spawn(async move |cx| {
            match input {
                UideToolInput::CreateSchema { name, purpose, fields } => {
                    handle_create_schema(name, purpose, fields, cx).await
                }
                UideToolInput::ListSchemas => {
                    handle_list_schemas(cx).await
                }
                UideToolInput::SearchSchemas { query, limit } => {
                    handle_search_schemas(query, limit.unwrap_or(10), cx).await
                }
                UideToolInput::CreateEntity { schema_name, data } => {
                    handle_create_entity(schema_name, data, cx).await
                }
                UideToolInput::SearchEntities { query, limit } => {
                    handle_search_entities(query, limit.unwrap_or(10), cx).await
                }
                UideToolInput::DeleteEntity { entity_id } => {
                    handle_delete_entity(entity_id, cx).await
                }
                UideToolInput::DeleteSchema { schema_name, cascade } => {
                    handle_delete_schema(schema_name, cascade, cx).await
                }
            }
        });

        task.into()
    }
}

async fn handle_create_schema(
    name: String,
    purpose: String,
    fields: Vec<FieldInput>,
    cx: &gpui::AsyncApp,
) -> Result<assistant_tool::ToolResultOutput> {
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    log::debug!("UIDE Debug: Creating schema '{}' with {} fields", name, fields.len());
    
    // Create semantic schema
    let mut schema = SemanticSchema::new(&name, &purpose);
    
    // Add fields to schema
    for field in &fields {
        let field_type = match field.field_type.as_str() {
            "text" | "string" => FieldType::Text { max_length: None },
            "number" | "float" => FieldType::Number { min: None, max: None },
            "boolean" | "bool" => FieldType::Boolean,
            "date" => FieldType::Date,
            "datetime" => FieldType::DateTime,
            "email" => FieldType::Email,
            "url" => FieldType::Url,
            "json" => FieldType::Json,
            _ => FieldType::Text { max_length: None }, // Default to text
        };
        
        let field_def = FieldDefinition {
            name: field.name.clone(),
            field_type,
            required: field.required,
            default_value: None,
            display_name: field.name.clone(),
            description: field.description.clone(),
            placeholder: None,
            widget_config: WidgetConfig::default(),
            validation: ValidationConfig::default(),
            semantic_tags: vec![],
            ai_description: field.description.clone(),
            voice_aliases: vec![],
        };
        
        schema = schema.add_field(field_def);
    }
    
    log::debug!("UIDE Debug: Created semantic schema with ID: {}", schema.id);
    
    // Store schema using flexible metadata approach
    let schema_content = {
        let mut builder = StructuredBuilder::new();
        builder = builder.field("_meta_category", Value::String("schema".to_string()));
        builder = builder.field("_meta_purpose", Value::String("data_structure_definition".to_string()));
        builder = builder.field("name", Value::String(schema.name.clone()));
        builder = builder.field("purpose", Value::String(purpose.clone()));
        builder = builder.field("definition", serde_json::to_value(&schema).map(json_to_uide_value)?);
        // Add searchable field names for better discovery
        let field_names: Vec<Value> = schema.fields.keys().map(|k| Value::String(k.clone())).collect();
        builder = builder.field("field_names", Value::Array(field_names));
        builder.build()
    };
    
    let schema_record = UniversalRecord::new(DataType::Structured, schema_content);
    
    log::debug!("UIDE Debug: Storing schema record in UIDE engine");
    let record_id = engine.store_record(schema_record).await?;
    log::debug!("UIDE Debug: Successfully stored schema with record ID: {}", record_id);
    
    // Verify the schema was stored by searching for it
    let verify_query = UniversalQuery::builder()
        .filter_type(DataType::Structured)
        .text(&format!("name:{}", name))
        .build()?;
    
    let verify_results = engine.search(verify_query).await?;
    log::debug!("UIDE Debug: Verification search found {} records for name '{}'", verify_results.results.len(), name);
    
    // Create example entity creation command for the LLM to copy
    let required_fields: Vec<_> = fields.iter().filter(|f| f.required).collect();
    let optional_fields: Vec<_> = fields.iter().filter(|f| !f.required).collect();
    
    let example_data = {
        let mut data = serde_json::Map::new();
        for field in &required_fields {
            let example_value = match field.field_type.as_str() {
                "email" => serde_json::Value::String("example@example.com".to_string()),
                "string" | "text" => serde_json::Value::String(format!("example_{}", field.name)),
                "number" => serde_json::Value::Number(serde_json::Number::from(42)),
                "boolean" => serde_json::Value::Bool(true),
                "date" => serde_json::Value::String("2023-01-01".to_string()),
                "datetime" => serde_json::Value::String("2023-01-01T12:00:00Z".to_string()),
                "url" => serde_json::Value::String("https://example.com".to_string()),
                _ => serde_json::Value::String(format!("example_{}", field.name)),
            };
            data.insert(field.name.clone(), example_value);
        }
        serde_json::Value::Object(data)
    };
    
    let example_entity_command = serde_json::json!({
        "action": "create_entity",
        "schema_name": name,
        "data": example_data
    });
    
    let result = format!(
        "✅ Created UIDE semantic schema '{}' with {} fields:\n{}\n\nSchema ID: {}\nRecord ID: {}\n\n🔍 Verification: Found {} records when searching for this schema\n\n📋 SCHEMA SUMMARY FOR ENTITY CREATION:\nSchema Name: '{}'\nRequired Fields: {}\nOptional Fields: {}\n\n💡 ⚠️ CRITICAL: Use the EXACT field names shown above. Required fields MUST be included.\n\n```json\n{}\n```",
        name,
        fields.len(),
        fields
            .iter()
            .map(|f| format!("  • {}: {} ({})", f.name, f.field_type, if f.required { "required" } else { "optional" }))
            .collect::<Vec<_>>()
            .join("\n"),
        schema.id,
        record_id,
        verify_results.results.len(),
        name,
        if required_fields.is_empty() { 
            "None".to_string() 
        } else { 
            required_fields.iter().map(|f| format!("'{}'", f.name)).collect::<Vec<_>>().join(", ") 
        },
        if optional_fields.is_empty() { 
            "None".to_string() 
        } else { 
            optional_fields.iter().map(|f| format!("'{}'", f.name)).collect::<Vec<_>>().join(", ") 
        },
        serde_json::to_string_pretty(&example_entity_command).unwrap_or_else(|_| "Error generating example".to_string())
    );
    Ok(result.into())
}

async fn handle_list_schemas(cx: &gpui::AsyncApp) -> Result<assistant_tool::ToolResultOutput> {
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    // Search for schema-like content using flexible detection
    let schema_results = find_schema_like_content(&engine).await?;
    
    log::debug!("UIDE Debug: Found {} schema-like records", schema_results.len());
    
    if schema_results.is_empty() {
        let result = "📋 No schemas found.\n\nUse the create_schema action to create your first semantic schema!".to_string();
        return Ok(result.into());
    }
    
    let mut schema_list = vec!["📋 Available Schemas:\n".to_string()];
    
    for (i, result) in schema_results.iter().enumerate().take(20) {
        if let UniversalContent::Structured { fields, .. } = &result.record.content {
            if let Some(Value::String(name)) = fields.get("name") {
                let schema_info = if let Some(definition_value) = fields.get("definition") {
                    // Try to parse the schema to get detailed field information
                    match uide_value_to_json(definition_value) {
                        Ok(json_value) => {
                            if let Ok(schema) = serde_json::from_value::<SemanticSchema>(json_value) {
                                let required_fields: Vec<_> = schema.fields.iter()
                                    .filter(|(_, field)| field.required)
                                    .map(|(name, field)| format!("'{}' ({})", name, field_type_to_string(&field.field_type)))
                                    .collect();
                                
                                let optional_fields: Vec<_> = schema.fields.iter()
                                    .filter(|(_, field)| !field.required)
                                    .map(|(name, field)| format!("'{}' ({})", name, field_type_to_string(&field.field_type)))
                                    .collect();
                                
                                // Create example entity creation command
                                let example_data = create_example_entity_data(&schema);
                                let example_command = serde_json::json!({
                                    "action": "create_entity",
                                    "schema_name": name,
                                    "data": example_data
                                });
                                
                                format!(
                                    "{}. 📋 Schema: '{}' ({} fields)\n   Purpose: {}\n   Required Fields: {}\n   Optional Fields: {}\n   \n   💡 To create entity, use:\n   ```json\n   {}\n   ```",
                                    i + 1, 
                                    name, 
                                    schema.fields.len(),
                                    schema.semantic_metadata.purpose,
                                    if required_fields.is_empty() { "None".to_string() } else { required_fields.join(", ") },
                                    if optional_fields.is_empty() { "None".to_string() } else { optional_fields.join(", ") },
                                    serde_json::to_string_pretty(&example_command).unwrap_or_else(|_| "Error".to_string())
                                )
                            } else {
                                format!("{}. {} (schema parse error)", i + 1, name)
                            }
                        }
                        Err(_) => format!("{}. {} (data error)", i + 1, name)
                    }
                } else {
                    format!("{}. {}", i + 1, name)
                };
                
                schema_list.push(schema_info);
                schema_list.push("".to_string()); // Empty line for readability
            }
        }
    }
    
    schema_list.push(format!("Total: {} schema(s) found", schema_results.len()));
    schema_list.push("⚠️ CRITICAL: When creating entities, use the EXACT field names shown above!".to_string());
    
    let result = schema_list.join("\n");
    Ok(result.into())
}

async fn handle_create_entity(
    schema_name: String,
    data: serde_json::Value,
    cx: &gpui::AsyncApp,
) -> Result<assistant_tool::ToolResultOutput> {
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    // Find schema using flexible search
    let schema_results = find_schema_by_name(&engine, &schema_name).await?;
    
    if schema_results.is_empty() {
        return Ok(format!("❌ Schema '{}' not found. Use list_schemas to see available schemas.", schema_name).into());
    }
    
    // Get the first matching schema
    let schema_record = &schema_results[0].record;
    let schema = if let UniversalContent::Structured { fields, .. } = &schema_record.content {
        if let Some(definition_value) = fields.get("definition") {
            match uide_value_to_json(definition_value) {
                Ok(json_value) => {
                    match serde_json::from_value::<SemanticSchema>(json_value) {
                        Ok(schema) => schema,
                        Err(e) => return Ok(format!("❌ Failed to parse schema '{}': {}", schema_name, e).into()),
                    }
                }
                Err(e) => return Ok(format!("❌ Failed to convert schema data: {}", e).into()),
            }
        } else {
            return Ok(format!("❌ Schema '{}' has no definition field", schema_name).into());
        }
    } else {
        return Ok(format!("❌ Schema '{}' is not a structured record", schema_name).into());
    };
    
    // Convert JSON data to UIDE Value format
    let uide_data = json_to_uide_value(data.clone());
    let data_map = if let Value::Object(map) = uide_data {
        map.into_iter().collect::<std::collections::HashMap<_, _>>()
    } else {
        return Ok("❌ Entity data must be a JSON object".to_string().into());
    };
    
    // Validate data against schema
    if let Err(e) = schema.validate_data(&data_map) {
        return Ok(format!("❌ Validation failed: {}", e).into());
    }
    
    // Create entity record using flexible metadata approach
    let entity_content = StructuredBuilder::new()
        .field("_meta_category", Value::String("entity".to_string()))
        .field("_meta_purpose", Value::String("structured_data".to_string()))
        .field("schema_name", Value::String(schema_name.clone()))
        .field("schema_id", Value::String(schema.id.to_string()))
        .field("data", Value::Object(data_map.into_iter().collect()))
        .build();
    
    let entity_record = UniversalRecord::new(DataType::Structured, entity_content);
    let entity_id = engine.store_record(entity_record).await?;
    
    let result = format!(
        "✅ Created entity in schema '{}'\nEntity ID: {}\nData: {}",
        schema_name,
        entity_id,
        serde_json::to_string_pretty(&data).unwrap_or_else(|_| "invalid JSON".to_string())
    );
    Ok(result.into())
}

async fn handle_search_entities(
    query: String,
    limit: usize,
    cx: &gpui::AsyncApp,
) -> Result<assistant_tool::ToolResultOutput> {
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    log::debug!("UIDE Debug: Searching entities with query '{}', limit {}", query, limit);
    
    // Find entity-like content using flexible detection
    let entity_results = match find_entity_like_content(&engine).await {
        Ok(results) => results,
        Err(e) => {
            log::error!("UIDE Debug: Error finding entity content: {}", e);
            return Ok(format!("🔍 No entities found for query: '{}'\n\nError: {}\nTry creating some entities first using create_entity.", query, e).into());
        }
    };
    
    log::debug!("UIDE Debug: Found {} entity-like records", entity_results.len());
    
    if entity_results.is_empty() {
        let result = format!("🔍 No entities found for query: '{}'\n\nTry creating some entities first using create_entity.", query);
        return Ok(result.into());
    }
    
    // Extract key search terms from the natural language query
    let search_terms = extract_search_terms(&query);
    log::debug!("UIDE Debug: Extracted search terms: {:?}", search_terms);
    
    let search_results: Vec<SearchResult> = entity_results.into_iter()
        .filter_map(|result| {
            if let UniversalContent::Structured { fields, .. } = &result.record.content {
                // Check if entity data matches any of the search terms - try multiple field names
                let data_value = fields.get("data")
                    .or_else(|| fields.get("entity_data"))
                    .or_else(|| fields.get("content"));
                
                if let Some(entity_data) = data_value {
                    let matches = if search_terms.is_empty() {
                        true // Empty query matches all
                    } else {
                        search_in_entity_data(entity_data, &search_terms)
                    };
                    log::debug!("UIDE Debug: Entity {} matches: {}", result.record.id, matches);
                    if matches {
                        Some(result)
                    } else {
                        None
                    }
                } else {
                    // If no data field, still include in results if query is empty or very broad
                    if search_terms.is_empty() || query.trim().is_empty() {
                        log::debug!("UIDE Debug: Entity {} has no data field but included due to empty query", result.record.id);
                        Some(result)
                    } else {
                        log::debug!("UIDE Debug: Entity {} has no data field, excluded from specific search", result.record.id);
                        None
                    }
                }
            } else { None }
        })
        .collect();
    
    if search_results.is_empty() {
        let result = format!("🔍 No entities found matching query: '{}'", query);
        return Ok(result.into());
    }
    
    let mut entity_list = vec![format!("🔍 Found {} entities for query: '{}'\n", search_results.len(), query)];
    
    for (i, result) in search_results.iter().enumerate().take(limit) {
        if let UniversalContent::Structured { fields, .. } = &result.record.content {
            let schema_name = fields.get("schema_name")
                .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None })
                .unwrap_or_else(|| "unknown".to_string());
            
            // Try multiple field names for entity data
            let entity_data = {
                let data_value = fields.get("data")
                    .or_else(|| fields.get("entity_data"))
                    .or_else(|| fields.get("content"));
                
                match data_value {
                    Some(v) => {
                        match uide_value_to_json(v) {
                            Ok(json) => serde_json::to_string_pretty(&json).unwrap_or_else(|_| "invalid data".to_string()),
                            Err(e) => {
                                log::debug!("UIDE Debug: Failed to convert entity data to JSON: {}", e);
                                format!("conversion error: {}", e)
                            }
                        }
                    }
                    None => {
                        // Debug: Show what fields are available
                        let available_fields: Vec<String> = fields.keys().cloned().collect();
                        log::debug!("UIDE Debug: Entity {} has fields: {:?}", result.record.id, available_fields);
                        format!("no data field found (available fields: {})", available_fields.join(", "))
                    }
                }
            };
            
            entity_list.push(format!("{}. Entity in schema '{}' (Score: {:.2})", i + 1, schema_name, result.score));
            entity_list.push(format!("   ID: {}", result.record.id));
            entity_list.push(format!("   Data: {}", entity_data));
            entity_list.push("".to_string()); // Empty line for readability
        }
    }
    
    entity_list.push("Query completed".to_string());
    
    let result = entity_list.join("\n");
    Ok(result.into())
}

async fn handle_delete_entity(
    entity_id: String,
    cx: &gpui::AsyncApp,
) -> Result<assistant_tool::ToolResultOutput> {
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    // Parse the entity ID
    let record_id = match uuid::Uuid::parse_str(&entity_id) {
        Ok(id) => id,
        Err(_) => return Ok(format!("❌ Invalid entity ID format: '{}'", entity_id).into()),
    };
    
    // Try to delete the entity
    match engine.delete(record_id).await {
        Ok(true) => {
            let result = format!("✅ Successfully deleted entity '{}'", entity_id);
            Ok(result.into())
        }
        Ok(false) => {
            let result = format!("❌ Entity '{}' not found", entity_id);
            Ok(result.into())
        }
        Err(e) => {
            let result = format!("❌ Failed to delete entity '{}': {}", entity_id, e);
            Ok(result.into())
        }
    }
}

async fn handle_delete_schema(
    schema_name: String,
    cascade: Option<bool>,
    cx: &gpui::AsyncApp,
) -> Result<assistant_tool::ToolResultOutput> {
    #[allow(unused_variables)]
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    log::debug!("UIDE Debug: Deleting schema '{}' with cascade={:?}", schema_name, cascade);
    
    // Find schema using flexible search
    let schema_results = find_schema_by_name(&engine, &schema_name).await?;
    
    if schema_results.is_empty() {
        return Ok(format!("❌ Schema '{}' not found.\n\nUse list_schemas to see available schemas.", schema_name).into());
    }
    
    // Get the first matching schema
    let schema_record = &schema_results[0].record;
    
    // Check for existing entities that use this schema
    let entity_search_query = UniversalQuery::builder()
        .filter_type(DataType::Structured)
        .text(&format!("schema_name {}", schema_name))
        .build()?;
    
    let search_results = engine.search(entity_search_query).await
        .unwrap_or_else(|_| SearchResults { 
            results: Vec::new(), 
            total_count: Some(0),
            query_time_ms: 0,
            strategies_used: Vec::new(),
        });
        
    let existing_entities: Vec<SearchResult> = search_results.results
        .into_iter()
        .filter(|result| {
            if let UniversalContent::Structured { fields, .. } = &result.record.content {
                fields.get("schema_name")
                    .and_then(|v| if let Value::String(s) = v { Some(s.as_str()) } else { None })
                    .map_or(false, |s| s == schema_name)
            } else {
                false
            }
        })
        .collect();
    
    let entity_count = existing_entities.len();
    let cascade = cascade.unwrap_or(false);
    
    // Check if entities exist and handle cascade option
    if entity_count > 0 && !cascade {
        return Ok(format!(
            "❌ Cannot delete schema '{}' because {} entities are using it.\n\
            Options:\n\
            1. Use {{ \"action\": \"delete_schema\", \"schema_name\": \"{}\", \"cascade\": true }} to delete schema AND all entities\n\
            2. Delete the entities first using delete_entity action\n\
            3. Keep the schema if you need it\n\n\
            ⚠️ WARNING: Cascade deletion will permanently delete {} entities!",
            schema_name, entity_count, schema_name, entity_count
        ).into());
    }
    
    // Perform deletion
    let mut deletion_summary = Vec::new();
    
    // Delete entities first if cascade is enabled
    if cascade && entity_count > 0 {
        let mut deleted_entities = 0;
        
        for entity_result in existing_entities {
            match engine.delete(entity_result.record.id).await {
                Ok(true) => {
                    deleted_entities += 1;
                    log::debug!("UIDE Debug: Deleted entity {}", entity_result.record.id);
                }
                Ok(false) => {
                    log::warn!("UIDE Debug: Entity {} was already deleted", entity_result.record.id);
                }
                Err(e) => {
                    log::error!("UIDE Debug: Failed to delete entity {}: {}", entity_result.record.id, e);
                }
            }
        }
        
        deletion_summary.push(format!("🗑️ Deleted {} entities that used this schema", deleted_entities));
    }
    
    // Delete the schema record itself
    match engine.delete(schema_record.id).await {
        Ok(true) => {
            deletion_summary.push(format!("✅ Deleted schema '{}'", schema_name));
            log::debug!("UIDE Debug: Successfully deleted schema '{}'", schema_name);
        }
        Ok(false) => {
            return Ok(format!("❌ Schema '{}' was already deleted", schema_name).into());
        }
        Err(e) => {
            return Ok(format!("❌ Failed to delete schema '{}': {}", schema_name, e).into());
        }
    }
    
    let result = if deletion_summary.is_empty() {
        format!("❌ No deletion performed for schema '{}'", schema_name)
    } else {
        format!("🎯 Schema Deletion Complete:\n\n{}", deletion_summary.join("\n"))
    };
    
    Ok(result.into())
}

// Helper functions for UIDE integration

/// Get or create a UIDE engine instance using GPUI global pattern
async fn get_uide_engine(cx: &gpui::AsyncApp) -> Result<Arc<UnifiedDataEngine>> {
    // Try to read existing engine, or create default global if none exists
    let existing_engine = cx.update(|cx| {
        // Use try_global to avoid panic if global doesn't exist
        match cx.try_global::<GlobalUideEngine>() {
            Some(global) => global.0.read().unwrap().clone(),
            None => {
                // Initialize the global if it doesn't exist
                cx.set_global(GlobalUideEngine::default());
                None
            }
        }
    })?;
    
    if let Some(engine) = existing_engine {
        return Ok(engine);
    }
    
    // Create new engine if none exists
    let uide_path = std::env::var("UIDE_DATA_PATH")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/Library/Application Support/Zed/uide", home)
        });
    
    log::info!("UIDE Debug: Initializing global UIDE engine at path: {}", uide_path);
    
    let engine = Arc::new(UnifiedDataEngine::new(uide_path).await
        .map_err(|e| anyhow!("Failed to create UIDE engine: {}", e))?);
        
    // Store in global state
    cx.update(|cx| {
        // Ensure global exists before writing to it
        if cx.try_global::<GlobalUideEngine>().is_none() {
            cx.set_global(GlobalUideEngine::default());
        }
        if let Some(global) = cx.try_global::<GlobalUideEngine>() {
            *global.0.write().unwrap() = Some(engine.clone());
        }
    })?;
    
    Ok(engine)
}

/// Convert JSON value to UIDE Value
fn json_to_uide_value(json_value: serde_json::Value) -> Value {
    match json_value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            // Try to preserve integer types when possible
            if let Some(i) = n.as_i64() {
                Value::Number(i as f64)
            } else if let Some(u) = n.as_u64() {
                Value::Number(u as f64)
            } else if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(json_to_uide_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let mut uide_obj = indexmap::IndexMap::new();
            for (k, v) in obj {
                uide_obj.insert(k, json_to_uide_value(v));
            }
            Value::Object(uide_obj)
        }
    }
}

/// Convert UIDE Value to JSON value
fn uide_value_to_json(value: &Value) -> Result<serde_json::Value> {
    match value {
        Value::Null => Ok(serde_json::Value::Null),
        Value::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Number(f) => {
            // Try to preserve integer values when they don't have fractional parts
            if f.fract() == 0.0 && *f >= 0.0 && *f <= u64::MAX as f64 {
                // It's a whole number, convert to integer in JSON
                let as_u64 = *f as u64;
                Ok(serde_json::Value::Number(serde_json::Number::from(as_u64)))
            } else {
                serde_json::Number::from_f64(*f)
                    .map(serde_json::Value::Number)
                    .ok_or_else(|| anyhow!("Invalid float value: {}", f))
            }
        }
        Value::String(s) => Ok(serde_json::Value::String(s.clone())),
        Value::Array(arr) => {
            let json_arr: Result<Vec<_>> = arr.iter()
                .map(uide_value_to_json)
                .collect();
            Ok(serde_json::Value::Array(json_arr?))
        }
        Value::Object(obj) => {
            let mut json_obj = serde_json::Map::new();
            for (k, v) in obj {
                json_obj.insert(k.clone(), uide_value_to_json(v)?);
            }
            Ok(serde_json::Value::Object(json_obj))
        }
        Value::Binary(_) => Ok(serde_json::Value::String("<binary data>".to_string())),
    }
}

async fn handle_search_schemas(
    query: String,
    limit: usize,
    cx: &gpui::AsyncApp,
) -> Result<assistant_tool::ToolResultOutput> {
    // Get or create UIDE engine
    let engine = get_uide_engine(cx).await?;
    
    log::debug!("UIDE Debug: Searching schemas with query '{}', limit {}", query, limit);
    
    // Find schema-like content using flexible detection
    let schema_results = find_schema_like_content(&engine).await?;
    
    log::debug!("UIDE Debug: Found {} total schemas", schema_results.len());
    
    if schema_results.is_empty() {
        let result = "🔍 No schemas found. Use create_schema to create your first schema!".to_string();
        return Ok(result.into());
    }
    
    let total_schemas_count = schema_results.len();
    
    // Extract search terms from the query
    let search_terms = extract_search_terms(&query);
    log::debug!("UIDE Debug: Extracted search terms: {:?}", search_terms);
    
    // Search through schemas based on query
    let matching_schemas: Vec<_> = schema_results.into_iter()
        .filter_map(|result| {
            if let UniversalContent::Structured { fields, .. } = &result.record.content {
                if let (Some(Value::String(name)), Some(definition_value)) = (fields.get("name"), fields.get("definition")) {
                    // Parse the schema to get searchable content
                    if let Ok(json_value) = uide_value_to_json(definition_value) {
                        if let Ok(schema) = serde_json::from_value::<SemanticSchema>(json_value) {
                            let searchable_text = create_schema_searchable_text(name, &schema);
                            
                            log::debug!("UIDE Debug: Schema '{}' searchable text: '{}'", name, searchable_text);
                            
                            // Check if any search term matches
                            let matches = if search_terms.is_empty() {
                                true // No search terms means show all
                            } else {
                                search_terms.iter().any(|term| {
                                    let term_matches = searchable_text.contains(&term.to_lowercase());
                                    log::debug!("UIDE Debug: Term '{}' matches schema '{}': {}", term, name, term_matches);
                                    term_matches
                                })
                            };
                            
                            if matches {
                                return Some((result, schema));
                            }
                        }
                    }
                }
            }
            None
        })
        .take(limit)
        .collect();
    
    if matching_schemas.is_empty() {
        let result = format!(
            "🔍 No schemas found for query: '{}'\n\nTotal schemas available: {}\n\nTry a broader search or use 'list_schemas' to see all available schemas.",
            query,
            total_schemas_count
        );
        return Ok(result.into());
    }
    
    let mut schema_list = vec![format!("🔍 Found {} schemas for query: '{}'\n", matching_schemas.len(), query)];
    
    for (i, (result, schema)) in matching_schemas.iter().enumerate() {
        let required_fields: Vec<_> = schema.fields.iter()
            .filter(|(_, field)| field.required)
            .map(|(name, field)| format!("'{}' ({})", name, field_type_to_string(&field.field_type)))
            .collect();
        
        let optional_fields: Vec<_> = schema.fields.iter()
            .filter(|(_, field)| !field.required)
            .map(|(name, field)| format!("'{}' ({})", name, field_type_to_string(&field.field_type)))
            .collect();
        
        // Create example entity creation command
        let example_data = create_example_entity_data(&schema);
        let example_command = serde_json::json!({
            "action": "create_entity",
            "schema_name": schema.name,
            "data": example_data
        });
        
        schema_list.push(format!(
            "{}. 📋 Schema: '{}' ({} fields) [Score: {:.2}]\n   Purpose: {}\n   Required Fields: {}\n   Optional Fields: {}\n   \n   💡 To create entity, use:\n   ```json\n   {}\n   ```",
            i + 1, 
            schema.name, 
            schema.fields.len(),
            result.score,
            schema.semantic_metadata.purpose,
            if required_fields.is_empty() { "None".to_string() } else { required_fields.join(", ") },
            if optional_fields.is_empty() { "None".to_string() } else { optional_fields.join(", ") },
            serde_json::to_string_pretty(&example_command).unwrap_or_else(|_| "Error".to_string())
        ));
        schema_list.push("".to_string()); // Empty line for readability
    }
    
    schema_list.push(format!("Query completed. Found {} matching schemas out of {} total.", matching_schemas.len(), total_schemas_count));
    schema_list.push("⚠️ CRITICAL: When creating entities, use the EXACT field names shown above!".to_string());
    
    let result = schema_list.join("\n");
    Ok(result.into())
}

// Helper functions for flexible content detection

/// Find schema-like content using flexible detection instead of hardcoded types
async fn find_schema_like_content(engine: &Arc<UnifiedDataEngine>) -> Result<Vec<SearchResult>> {
    // Try multiple strategies to find schemas
    let strategies = vec![
        // Strategy 1: Look for records with schema indicators
        ("schema definition purpose", "schema purpose definition field"),
        ("semantic_metadata fields", "semantic_metadata"),
        ("schema name purpose", "name purpose field_names"),
        // Strategy 2: Look for meta category schemas
        ("meta schema", "_meta_category:schema"),
    ];
    
    for (strategy_name, query_text) in strategies {
        log::debug!("UIDE Debug: Trying schema detection strategy: {}", strategy_name);
        
        let query = UniversalQuery::builder()
            .filter_type(DataType::Structured)
            .text(query_text)
            .limit(100) // Limit results to prevent excessive processing
            .build()?;
        
        let results = engine.search(query).await?;
        
        // Filter results to only include likely schemas
        let schema_candidates: Vec<_> = results.results.into_iter()
            .filter(|result| is_likely_schema(&result.record))
            .collect();
        
        if !schema_candidates.is_empty() {
            log::debug!("UIDE Debug: Strategy '{}' found {} schema candidates", strategy_name, schema_candidates.len());
            return Ok(schema_candidates);
        }
    }
    
    // Early return if no data found - don't do expensive fallback search
    log::debug!("UIDE Debug: No schemas found with specific strategies, returning empty result");
    Ok(Vec::new())
}

/// Find entity-like content using flexible detection instead of hardcoded types
async fn find_entity_like_content(engine: &Arc<UnifiedDataEngine>) -> Result<Vec<SearchResult>> {
    // Try multiple strategies to find entities
    let strategies = vec![
        // Strategy 1: Look for records with entity indicators
        ("entity data schema", "data schema_name schema_id"),
        ("structured data entity", "structured_data entity"),
        // Strategy 2: Look for common entity patterns
        ("meta entity", "_meta_category:entity"),
    ];
    
    for (strategy_name, query_text) in strategies {
        log::debug!("UIDE Debug: Trying entity detection strategy: {}", strategy_name);
        
        let query = UniversalQuery::builder()
            .filter_type(DataType::Structured)
            .text(query_text)
            .limit(100) // Limit results to prevent excessive processing
            .build()?;
        
        let results = engine.search(query).await?;
        
        // Filter results to only include likely entities
        let entity_candidates: Vec<_> = results.results.into_iter()
            .filter(|result| is_likely_entity(&result.record))
            .collect();
        
        if !entity_candidates.is_empty() {
            log::debug!("UIDE Debug: Strategy '{}' found {} entity candidates", strategy_name, entity_candidates.len());
            return Ok(entity_candidates);
        }
    }
    
    // Early return if no data found - don't do expensive fallback search
    log::debug!("UIDE Debug: No entities found with specific strategies, returning empty result");
    Ok(Vec::new())
}

/// Find schema by name using flexible search
async fn find_schema_by_name(engine: &Arc<UnifiedDataEngine>, schema_name: &str) -> Result<Vec<SearchResult>> {
    // First try direct name search
    let query = UniversalQuery::builder()
        .filter_type(DataType::Structured)
        .text(&format!("name:{}", schema_name))
        .build()?;
    
    let results = engine.search(query).await?;
    
    // Filter to only include schemas with matching name
    let matching_schemas: Vec<_> = results.results.into_iter()
        .filter(|result| {
            if is_likely_schema(&result.record) {
                if let UniversalContent::Structured { fields, .. } = &result.record.content {
                    if let Some(Value::String(name)) = fields.get("name") {
                        return name == schema_name;
                    }
                }
            }
            false
        })
        .collect();
    
    if !matching_schemas.is_empty() {
        return Ok(matching_schemas);
    }
    
    // Fallback: search all schemas and filter by name
    let all_schemas = find_schema_like_content(engine).await?;
    let matching_schemas: Vec<_> = all_schemas.into_iter()
        .filter(|result| {
            if let UniversalContent::Structured { fields, .. } = &result.record.content {
                if let Some(Value::String(name)) = fields.get("name") {
                    return name == schema_name;
                }
            }
            false
        })
        .collect();
    
    Ok(matching_schemas)
}

/// Determine if a record is likely a schema based on its structure and content
fn is_likely_schema(record: &UniversalRecord) -> bool {
    if let UniversalContent::Structured { fields, .. } = &record.content {
        // Look for schema-like indicators
        let has_schema_category = fields.get("_meta_category")
            .map(|v| matches!(v, Value::String(s) if s == "schema"))
            .unwrap_or(false);
        
        let has_definition = fields.contains_key("definition");
        let has_field_names = fields.contains_key("field_names");
        let has_purpose = fields.contains_key("purpose");
        
        // Legacy support for old format
        let has_legacy_schema_type = fields.get("schema_type")
            .map(|v| matches!(v, Value::String(s) if s == "semantic_schema"))
            .unwrap_or(false);
        
        has_schema_category || (has_definition && (has_field_names || has_purpose)) || has_legacy_schema_type
    } else {
        false
    }
}

/// Determine if a record is likely an entity based on its structure and content
fn is_likely_entity(record: &UniversalRecord) -> bool {
    if let UniversalContent::Structured { fields, .. } = &record.content {
        // Look for entity-like indicators
        let has_entity_category = fields.get("_meta_category")
            .map(|v| matches!(v, Value::String(s) if s == "entity"))
            .unwrap_or(false);
        
        let has_data = fields.contains_key("data");
        let has_schema_name = fields.contains_key("schema_name");
        
        // Legacy support for old format
        let has_legacy_entity_type = fields.get("entity_type")
            .map(|v| matches!(v, Value::String(s) if s == "schema_entity"))
            .unwrap_or(false);
        
        has_entity_category || (has_data && has_schema_name) || has_legacy_entity_type
    } else {
        false
    }
}

/// Create searchable text for a schema
fn create_schema_searchable_text(name: &str, schema: &SemanticSchema) -> String {
    format!(
        "{} {} {} {}",
        name,
        schema.semantic_metadata.purpose,
        schema.fields.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(" "),
        schema.fields.values()
            .filter_map(|f| f.description.as_ref())
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    ).to_lowercase()
}

/// Create example entity data for a schema
fn create_example_entity_data(schema: &SemanticSchema) -> serde_json::Value {
    let mut data = serde_json::Map::new();
    for (field_name, field_def) in &schema.fields {
        if field_def.required {
            let example_value = match field_def.field_type {
                FieldType::Email => serde_json::Value::String("example@example.com".to_string()),
                FieldType::Text { .. } => serde_json::Value::String(format!("example_{}", field_name)),
                FieldType::Number { .. } => serde_json::Value::Number(serde_json::Number::from(42)),
                FieldType::Boolean => serde_json::Value::Bool(true),
                FieldType::Date => serde_json::Value::String("2023-01-01".to_string()),
                FieldType::DateTime => serde_json::Value::String("2023-01-01T12:00:00Z".to_string()),
                FieldType::Url => serde_json::Value::String("https://example.com".to_string()),
                _ => serde_json::Value::String(format!("example_{}", field_name)),
            };
            data.insert(field_name.clone(), example_value);
        }
    }
    serde_json::Value::Object(data)
}

// Helper function to convert FieldType to string
fn field_type_to_string(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Text { .. } => "string".to_string(),
        FieldType::Number { .. } => "number".to_string(),
        FieldType::Boolean => "boolean".to_string(),
        FieldType::Date => "date".to_string(),
        FieldType::DateTime => "datetime".to_string(),
        FieldType::Email => "email".to_string(),
        FieldType::Url => "url".to_string(),
        FieldType::Json => "json".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Extract search terms from a natural language query
fn extract_search_terms(query: &str) -> Vec<String> {
    // Common stop words to ignore in multiple languages
    let stop_words = [
        // English
        "find", "search", "get", "show", "with", "that", "have", "has", "are", "is", "the", "a", "an", "and", "or", "but", "what", "which", "where", "there", "who", "all",
        // Russian
        "найти", "найдите", "поиск", "покажи", "покажите", "есть", "которые", "который", "где", "что", "какие", "какой", "с", "и", "или", "но",
        // Spanish  
        "buscar", "encontrar", "mostrar", "que", "cual", "donde", "hay", "con", "y", "o", "pero", "qué", "cuál", "dónde",
        // German
        "finden", "suchen", "zeigen", "mit", "die", "der", "das", "und", "oder", "aber", "welche", "welcher", "wo", "was", "gibt",
        // French
        "trouver", "chercher", "montrer", "avec", "que", "qui", "où", "il", "y", "a", "et", "ou", "mais", "quel", "quelle"
    ];
    
    // Extract meaningful terms from the query
    let terms: Vec<String> = query.to_lowercase()
        .split_whitespace()
        .filter(|term| !stop_words.contains(term) && term.len() > 1)
        .map(|term| term.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|term| !term.is_empty())
        .collect();
    
    log::debug!("UIDE Debug: Original query: '{}', extracted terms: {:?}", query, terms);
    terms
}

/// Search for terms within entity data
fn search_in_entity_data(entity_data: &Value, search_terms: &[String]) -> bool {
    if search_terms.is_empty() {
        return true; // If no search terms, match everything
    }
    
    // Convert entity data to searchable text
    let searchable_text = value_to_searchable_text(entity_data).to_lowercase();
    log::debug!("UIDE Debug: Searchable text: '{}'", searchable_text);
    
    // Check if any search term matches
    search_terms.iter().any(|term| {
        let matches = searchable_text.contains(&term.to_lowercase());
        log::debug!("UIDE Debug: Term '{}' matches: {}", term, matches);
        matches
    })
}

/// Convert a UIDE Value to searchable text
fn value_to_searchable_text(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            arr.iter()
                .map(value_to_searchable_text)
                .collect::<Vec<_>>()
                .join(" ")
        }
        Value::Object(obj) => {
            obj.iter()
                .map(|(k, v)| format!("{} {}", k, value_to_searchable_text(v)))
                .collect::<Vec<_>>()
                .join(" ")
        }
        Value::Binary(_) => String::new(),
    }
} 