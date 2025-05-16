use anyhow::{anyhow, Result};
use context_server::protocol::{Request, Response, ServerCapability, ToolCallResult, ToolInput, ToolMetadata};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use vector_store::{VectorStoreRegistry};

pub struct VectorStoreServer {
    db_path: PathBuf,
    registry: Arc<Mutex<Option<VectorStoreRegistry>>>,
}

impl VectorStoreServer {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            registry: Arc::new(Mutex::new(None)),
        }
    }

    pub fn capabilities(&self) -> Vec<ServerCapability> {
        vec![ServerCapability::Tools, ServerCapability::SlashCommands]
    }

    // Handle a slash command from the Zed UI
    fn handle_slash_command(&self, command: &str, args: &[String]) -> Result<String> {
        match command {
            "vector-info" => self.handle_info(),
            "vector-search" => self.handle_search(args),
            "vector-create" => self.handle_create(args),
            "vector-add" => self.handle_add(args),
            _ => Err(anyhow!("Unknown command: {}", command)),
        }
    }

    // Handle a tool call from the LLM
    fn handle_tool_call(&self, tool: &str, inputs: &ToolInput) -> Result<ToolCallResult> {
        match tool {
            "vector_search" => {
                let query = inputs.get("query").ok_or_else(|| anyhow!("Missing query parameter"))?;
                let store_name = inputs.get("store_name").unwrap_or("default");
                let limit = inputs.get("limit").unwrap_or("5").parse::<usize>().unwrap_or(5);
                
                let results = self.search_vectors(store_name, query, limit)?;
                Ok(ToolCallResult::Success(serde_json::json!({
                    "results": results
                })))
            },
            "vector_create" => {
                let store_name = inputs.get("name").ok_or_else(|| anyhow!("Missing name parameter"))?;
                self.create_store(store_name)?;
                Ok(ToolCallResult::Success(serde_json::json!({
                    "success": true,
                    "message": format!("Created vector store '{}'", store_name)
                })))
            },
            "vector_add" => {
                let store_name = inputs.get("store_name").unwrap_or("default");
                let content = inputs.get("content").ok_or_else(|| anyhow!("Missing content parameter"))?;
                let metadata = inputs.get("metadata").unwrap_or("{}");
                
                self.add_to_store(store_name, content, metadata)?;
                Ok(ToolCallResult::Success(serde_json::json!({
                    "success": true,
                    "message": format!("Added content to store '{}'", store_name)
                })))
            },
            _ => Err(anyhow!("Unknown tool: {}", tool)),
        }
    }

    // Get available tools for the LLM
    fn get_tools(&self) -> Vec<ToolMetadata> {
        vec![
            ToolMetadata {
                name: "vector_search".to_string(),
                description: "Search for similar vectors in a vector store".to_string(),
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The text query to search for"
                        },
                        "store_name": {
                            "type": "string",
                            "description": "The name of the vector store to search in",
                            "default": "default"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of results to return",
                            "default": 5
                        }
                    },
                    "required": ["query"]
                }),
            },
            ToolMetadata {
                name: "vector_create".to_string(),
                description: "Create a new vector store".to_string(),
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "The name of the vector store to create"
                        }
                    },
                    "required": ["name"]
                }),
            },
            ToolMetadata {
                name: "vector_add".to_string(),
                description: "Add content to a vector store".to_string(),
                schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "store_name": {
                            "type": "string",
                            "description": "The name of the vector store to add to",
                            "default": "default"
                        },
                        "content": {
                            "type": "string",
                            "description": "The text content to store"
                        },
                        "metadata": {
                            "type": "string",
                            "description": "JSON metadata to associate with the content",
                            "default": "{}"
                        }
                    },
                    "required": ["content"]
                }),
            },
        ]
    }

    // Handle a request from the context server protocol
    pub fn handle_request(&self, request: Request) -> Result<Response> {
        match request {
            Request::ToolCall { tool, inputs } => {
                let result = self.handle_tool_call(&tool, &inputs)?;
                Ok(Response::ToolCallResult(result))
            }
            Request::SlashCommand { name, args } => {
                let result = self.handle_slash_command(&name, &args)?;
                Ok(Response::SlashCommandResult(result))
            }
            Request::GetTools => {
                Ok(Response::Tools(self.get_tools()))
            }
            _ => Err(anyhow!("Unsupported request type")),
        }
    }

    // Handler implementations

    fn handle_info(&self) -> Result<String> {
        let registry = self.get_registry()?;
        let stores = registry.list_stores();
        
        let mut result = String::from("# Vector Stores\n\n");
        if stores.is_empty() {
            result.push_str("No vector stores found.");
        } else {
            for store in stores {
                let count = registry.store(&store)
                    .map(|s| s.count())
                    .unwrap_or(0);
                    
                result.push_str(&format!("- **{}**: {} vectors\n", store, count));
            }
        }
        
        Ok(result)
    }

    fn handle_search(&self, args: &[String]) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow!("Usage: /vector-search <query> [store_name] [limit]"));
        }

        let query = &args[0];
        let store_name = args.get(1).map(|s| s.as_str()).unwrap_or("default");
        let limit = args.get(2)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(5);

        let results = self.search_vectors(store_name, query, limit)?;
        
        let mut response = format!("# Search Results for '{}'\n\n", query);
        if results.is_empty() {
            response.push_str("No results found.");
        } else {
            for (i, (content, score)) in results.iter().enumerate() {
                response.push_str(&format!("## Result {}\n", i + 1));
                response.push_str(&format!("**Similarity Score**: {:.2}\n\n", score));
                response.push_str(&format!("```\n{}\n```\n\n", content));
            }
        }
        
        Ok(response)
    }

    fn handle_create(&self, args: &[String]) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow!("Usage: /vector-create <store_name>"));
        }

        let store_name = &args[0];
        self.create_store(store_name)?;
        
        Ok(format!("Created vector store '{}'", store_name))
    }

    fn handle_add(&self, args: &[String]) -> Result<String> {
        if args.len() < 2 {
            return Err(anyhow!("Usage: /vector-add <store_name> <content> [metadata_json]"));
        }

        let store_name = &args[0];
        let content = &args[1];
        let metadata = args.get(2).map(|s| s.as_str()).unwrap_or("{}");

        self.add_to_store(store_name, content, metadata)?;
        
        Ok(format!("Added content to store '{}'", store_name))
    }

    fn get_registry(&self) -> Result<VectorStoreRegistry> {
        let mut registry_guard = self.registry.lock().map_err(|_| anyhow!("Failed to acquire registry lock"))?;
        
        if registry_guard.is_none() {
            *registry_guard = Some(VectorStoreRegistry::new(&self.db_path)?);
        }
        
        Ok(registry_guard.as_ref().unwrap().clone())
    }

    fn create_store(&self, name: &str) -> Result<()> {
        let registry = self.get_registry()?;
        registry.create_store(name)
    }

    fn search_vectors(&self, store_name: &str, query: &str, limit: usize) -> Result<Vec<(String, f32)>> {
        let registry = self.get_registry()?;
        let store = registry.store(store_name).ok_or_else(|| anyhow!("Store not found: {}", store_name))?;
        
        let results = store.search(query, limit)?;
        
        Ok(results.into_iter()
            .map(|(content, score)| (content, score))
            .collect())
    }

    fn add_to_store(&self, store_name: &str, content: &str, metadata_json: &str) -> Result<()> {
        let registry = self.get_registry()?;
        
        // Create the store if it doesn't exist
        if registry.store(store_name).is_none() {
            registry.create_store(store_name)?;
        }
        
        let store = registry.store(store_name).ok_or_else(|| anyhow!("Failed to get store: {}", store_name))?;
        
        // Parse metadata if provided
        let metadata = if metadata_json.trim().is_empty() || metadata_json == "{}" {
            serde_json::Value::Object(serde_json::Map::new())
        } else {
            serde_json::from_str(metadata_json)?
        };
        
        store.add(content, metadata)?;
        
        Ok(())
    }
} 