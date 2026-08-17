//! Runtime-neutral MCP tool registration and dispatch.

use std::{collections::HashSet, sync::Arc};

use futures::future::BoxFuture;
use rustc_hash::FxHashMap;
use schemars::{JsonSchema, generate::SchemaSettings};
use serde_json::{Map, Value};

use crate::{Error, Role};

use super::{McpConnectionTo, McpTool};

/// JSON Schema object used to describe MCP tool inputs and outputs.
pub type McpToolSchema = Map<String, Value>;

/// Tracks which tools are enabled.
///
/// - `DenyList`: All tools enabled except those in the set (default)
/// - `AllowList`: Only tools in the set are enabled
#[derive(Clone, Debug)]
pub enum EnabledTools {
    /// All tools enabled except those in the deny set.
    DenyList(HashSet<String>),
    /// Only tools in the allow set are enabled.
    AllowList(HashSet<String>),
}

impl Default for EnabledTools {
    fn default() -> Self {
        EnabledTools::DenyList(HashSet::new())
    }
}

impl EnabledTools {
    /// Check if a tool is enabled.
    #[must_use]
    pub fn is_enabled(&self, name: &str) -> bool {
        match self {
            EnabledTools::DenyList(deny) => !deny.contains(name),
            EnabledTools::AllowList(allow) => allow.contains(name),
        }
    }
}

/// Runtime-neutral metadata for an MCP tool.
#[derive(Clone, Debug)]
pub struct McpToolMetadata {
    name: String,
    title: Option<String>,
    description: String,
    input_schema: Arc<McpToolSchema>,
    output_schema: Option<Arc<McpToolSchema>>,
}

impl McpToolMetadata {
    fn from_tool<R: Role, M: McpTool<R>>(tool: &M) -> Self {
        Self {
            name: tool.name(),
            title: tool.title(),
            description: tool.description(),
            input_schema: schema_for_type::<M::Input>(),
            output_schema: schema_for_output::<M::Output>(),
        }
    }

    /// The tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// A human-readable title for the tool.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// A description of what the tool does.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// JSON Schema object defining the expected parameters for the tool.
    #[must_use]
    pub fn input_schema(&self) -> &Arc<McpToolSchema> {
        &self.input_schema
    }

    /// Optional JSON Schema object defining the structure of the tool's output.
    #[must_use]
    pub fn output_schema(&self) -> Option<&Arc<McpToolSchema>> {
        self.output_schema.as_ref()
    }
}

/// A registered MCP tool that can be dispatched with erased JSON values.
pub struct RegisteredMcpTool<Counterpart: Role> {
    metadata: McpToolMetadata,
    tool: Arc<dyn ErasedMcpTool<Counterpart>>,
}

impl<Counterpart: Role + std::fmt::Debug> std::fmt::Debug for RegisteredMcpTool<Counterpart> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisteredMcpTool")
            .field("metadata", &self.metadata)
            .field("has_structured_output", &self.has_structured_output())
            .finish_non_exhaustive()
    }
}

impl<Counterpart: Role> RegisteredMcpTool<Counterpart> {
    fn new(tool: impl McpTool<Counterpart> + 'static) -> Self {
        let metadata = McpToolMetadata::from_tool(&tool);
        Self {
            metadata,
            tool: make_erased_mcp_tool(tool),
        }
    }

    /// Tool metadata.
    #[must_use]
    pub fn metadata(&self) -> &McpToolMetadata {
        &self.metadata
    }

    /// The tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.metadata.name()
    }

    /// Whether the tool returns structured output.
    #[must_use]
    pub fn has_structured_output(&self) -> bool {
        self.metadata.output_schema().is_some()
    }

    /// Invoke the registered tool using JSON input and output values.
    pub fn call_tool(
        &self,
        input: Value,
        connection: McpConnectionTo<Counterpart>,
    ) -> BoxFuture<'_, Result<Value, Error>> {
        self.tool.call_tool(input, connection)
    }
}

/// Runtime-neutral registry for MCP tools.
#[derive(Debug)]
pub struct McpToolRegistry<Counterpart: Role> {
    instructions: Option<String>,
    tool_indices: FxHashMap<String, usize>,
    tools: Vec<RegisteredMcpTool<Counterpart>>,
    enabled_tools: EnabledTools,
}

impl<Counterpart: Role> Default for McpToolRegistry<Counterpart> {
    fn default() -> Self {
        Self {
            instructions: None,
            tool_indices: FxHashMap::default(),
            tools: Vec::new(),
            enabled_tools: EnabledTools::default(),
        }
    }
}

impl<Counterpart: Role> McpToolRegistry<Counterpart> {
    /// Set the server instructions that are provided to the client.
    pub fn set_instructions(&mut self, instructions: impl ToString) {
        self.instructions = Some(instructions.to_string());
    }

    /// Server instructions provided to the client.
    #[must_use]
    pub fn instructions(&self) -> Option<&str> {
        self.instructions.as_deref()
    }

    /// Register a tool.
    pub fn register_tool(&mut self, tool: impl McpTool<Counterpart> + 'static) {
        let registered_tool = RegisteredMcpTool::new(tool);
        let name = registered_tool.name().to_string();

        if let Some(&index) = self.tool_indices.get(&name) {
            self.tools[index] = registered_tool;
        } else {
            self.tool_indices.insert(name, self.tools.len());
            self.tools.push(registered_tool);
        }
    }

    /// Return all registered tools in registration order.
    pub fn tools(&self) -> impl Iterator<Item = &RegisteredMcpTool<Counterpart>> {
        self.tools.iter()
    }

    /// Return enabled registered tools in registration order.
    pub fn enabled_tools(&self) -> impl Iterator<Item = &RegisteredMcpTool<Counterpart>> {
        self.tools
            .iter()
            .filter(|tool| self.enabled_tools.is_enabled(tool.name()))
    }

    /// Return a registered tool by name, even if it is disabled.
    #[must_use]
    pub fn tool(&self, name: &str) -> Option<&RegisteredMcpTool<Counterpart>> {
        self.tool_indices
            .get(name)
            .and_then(|&index| self.tools.get(index))
    }

    /// Return an enabled tool by name.
    #[must_use]
    pub fn enabled_tool(&self, name: &str) -> Option<&RegisteredMcpTool<Counterpart>> {
        self.tool(name)
            .filter(|tool| self.enabled_tools.is_enabled(tool.name()))
    }

    /// Check whether a tool is registered.
    #[must_use]
    pub fn contains_tool(&self, name: &str) -> bool {
        self.tool_indices.contains_key(name)
    }

    /// Disable all tools. After calling this, only tools explicitly enabled
    /// with [`enable_tool`](Self::enable_tool) will be available.
    pub fn disable_all_tools(&mut self) {
        self.enabled_tools = EnabledTools::AllowList(HashSet::new());
    }

    /// Enable all tools. After calling this, all tools will be available
    /// except those explicitly disabled with [`disable_tool`](Self::disable_tool).
    pub fn enable_all_tools(&mut self) {
        self.enabled_tools = EnabledTools::DenyList(HashSet::new());
    }

    /// Disable a specific tool by name.
    ///
    /// Returns an error if the tool is not registered.
    pub fn disable_tool(&mut self, name: &str) -> Result<(), Error> {
        if !self.contains_tool(name) {
            return Err(Error::invalid_request().data(format!("unknown tool: {name}")));
        }
        match &mut self.enabled_tools {
            EnabledTools::DenyList(deny) => {
                deny.insert(name.to_string());
            }
            EnabledTools::AllowList(allow) => {
                allow.remove(name);
            }
        }
        Ok(())
    }

    /// Enable a specific tool by name.
    ///
    /// Returns an error if the tool is not registered.
    pub fn enable_tool(&mut self, name: &str) -> Result<(), Error> {
        if !self.contains_tool(name) {
            return Err(Error::invalid_request().data(format!("unknown tool: {name}")));
        }
        match &mut self.enabled_tools {
            EnabledTools::DenyList(deny) => {
                deny.remove(name);
            }
            EnabledTools::AllowList(allow) => {
                allow.insert(name.to_string());
            }
        }
        Ok(())
    }
}

/// Erased version of the MCP tool trait that is dyn-compatible.
trait ErasedMcpTool<Counterpart: Role>: Send + Sync {
    fn call_tool(
        &self,
        input: Value,
        connection: McpConnectionTo<Counterpart>,
    ) -> BoxFuture<'_, Result<Value, Error>>;
}

fn make_erased_mcp_tool<R, M>(tool: M) -> Arc<dyn ErasedMcpTool<R>>
where
    R: Role,
    M: McpTool<R> + 'static,
{
    struct ErasedMcpToolImpl<M> {
        tool: M,
    }

    impl<R, M> ErasedMcpTool<R> for ErasedMcpToolImpl<M>
    where
        R: Role,
        M: McpTool<R>,
    {
        fn call_tool(
            &self,
            input: Value,
            context: McpConnectionTo<R>,
        ) -> BoxFuture<'_, Result<Value, Error>> {
            Box::pin(async move {
                let input = serde_json::from_value(input).map_err(crate::util::internal_error)?;
                serde_json::to_value(self.tool.call_tool(input, context).await?)
                    .map_err(crate::util::internal_error)
            })
        }
    }

    Arc::new(ErasedMcpToolImpl { tool })
}

fn schema_for_type<T: JsonSchema>() -> Arc<McpToolSchema> {
    let settings = SchemaSettings::draft2020_12();
    let generator = settings.into_generator();
    let schema = generator.into_root_schema_for::<T>();
    let object = serde_json::to_value(schema).expect("failed to serialize schema");
    let Value::Object(object) = object else {
        panic!("Schema serialization produced non-object value: expected JSON object");
    };
    Arc::new(object)
}

fn schema_for_output<T: JsonSchema>() -> Option<Arc<McpToolSchema>> {
    let schema = schema_for_type::<T>();
    match schema.get("type") {
        Some(Value::String(t)) if t == "object" => Some(schema),
        _ => None,
    }
}
