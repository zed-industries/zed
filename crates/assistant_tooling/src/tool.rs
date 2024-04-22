use anyhow::Result;
use gpui::{div, AnyElement, AppContext, Element, ParentElement as _, Task, WindowContext};
use schemars::{schema::SchemaObject, schema_for, JsonSchema};
use serde::Deserialize;
use std::{any::Any, fmt::Debug};

#[derive(Default, Deserialize)]
pub struct ToolFunctionCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
    #[serde(skip)]
    pub result: Option<ToolFunctionCallResult>,
}

pub enum ToolFunctionCallResult {
    NoSuchTool,
    ParsingFailed,
    ExecutionFailed {
        input: Box<dyn Any>,
    },
    Finished {
        input: Box<dyn Any>,
        output: Box<dyn Any>,
        render_fn: fn(
            // tool_call_id
            &str,
            // LanguageModelTool::Input
            &Box<dyn Any>,
            // LanguageModelTool::Output
            &Box<dyn Any>,
            &mut WindowContext,
        ) -> AnyElement,
        format_fn: fn(
            // LanguageModelTool::Input
            &Box<dyn Any>,
            // LanguageModelTool::Output
            &Box<dyn Any>,
        ) -> String,
    },
}

impl ToolFunctionCallResult {
    pub fn render(
        &self,
        tool_name: &str,
        tool_call_id: &str,
        cx: &mut WindowContext,
    ) -> AnyElement {
        match self {
            ToolFunctionCallResult::NoSuchTool => {
                div().child(format!("no such tool {tool_name}")).into_any()
            }
            ToolFunctionCallResult::ParsingFailed => div()
                .child(format!("failed to parse input for tool {tool_name}"))
                .into_any(),
            ToolFunctionCallResult::ExecutionFailed { .. } => div()
                .child(format!("failed to execute tool {tool_name}"))
                .into_any(),
            ToolFunctionCallResult::Finished {
                input,
                output,
                render_fn,
                ..
            } => render_fn(tool_call_id, input, output, cx),
        }
    }

    pub fn format(&self, tool: &str) -> String {
        match self {
            ToolFunctionCallResult::NoSuchTool => format!("no such tool {tool}"),
            ToolFunctionCallResult::ParsingFailed => {
                format!("failed to parse input for tool {tool}")
            }
            ToolFunctionCallResult::ExecutionFailed { input: _input } => {
                format!("failed to execute tool {tool}")
            }
            ToolFunctionCallResult::Finished {
                input,
                output,
                format_fn,
                ..
            } => format_fn(input, output),
        }
    }
}

#[derive(Clone)]
pub struct ToolFunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: SchemaObject,
}

impl Debug for ToolFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema = serde_json::to_string(&self.parameters).ok();
        let schema = schema.unwrap_or("None".to_string());

        f.debug_struct("ToolFunctionDefinition")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("parameters", &schema)
            .finish()
    }
}

pub trait LanguageModelTool {
    /// The input type that will be passed in to `execute` when the tool is called
    /// by the language model.
    type Input: for<'de> Deserialize<'de> + JsonSchema;

    /// The output returned by executing the tool.
    type Output: 'static;

    /// The name of the tool is exposed to the language model to allow
    /// the model to pick which tools to use. As this name is used to
    /// identify the tool within a tool registry, it should be unique.
    fn name(&self) -> String;

    /// A description of the tool that can be used to _prompt_ the model
    /// as to what the tool does.
    fn description(&self) -> String;

    /// The OpenAI Function definition for the tool, for direct use with OpenAI's API.
    fn definition(&self) -> ToolFunctionDefinition {
        ToolFunctionDefinition {
            name: self.name(),
            description: self.description(),
            parameters: schema_for!(Self::Input).schema,
        }
    }

    /// Execute the tool
    fn execute(&self, input: &Self::Input, cx: &AppContext) -> Task<Result<Self::Output>>;

    fn render(
        tool_call_id: &str,
        input: &Self::Input,
        output: &Self::Output,
        cx: &mut WindowContext,
    ) -> AnyElement;

    fn format(input: &Self::Input, output: &Self::Output) -> String;
}
