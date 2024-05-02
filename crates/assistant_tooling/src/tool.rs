use anyhow::Result;
use gpui::{AnyElement, AnyView, IntoElement as _, Render, Task, View, WindowContext};
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::Deserialize;
use std::fmt::Display;

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
    Finished { for_model: String, view: AnyView },
}

impl ToolFunctionCallResult {
    pub fn format(&self, name: &String) -> String {
        match self {
            ToolFunctionCallResult::NoSuchTool => format!("No tool for {name}"),
            ToolFunctionCallResult::ParsingFailed => {
                format!("Unable to parse arguments for {name}")
            }
            ToolFunctionCallResult::Finished { for_model, .. } => for_model.clone(),
        }
    }

    pub fn into_any_element(&self, name: &String) -> AnyElement {
        match self {
            ToolFunctionCallResult::NoSuchTool => {
                format!("Language Model attempted to call {name}").into_any_element()
            }
            ToolFunctionCallResult::ParsingFailed => {
                format!("Language Model called {name} with bad arguments").into_any_element()
            }
            ToolFunctionCallResult::Finished { view, .. } => view.clone().into_any_element(),
        }
    }
}

#[derive(Clone)]
pub struct ToolFunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: RootSchema,
}

impl Display for ToolFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema = serde_json::to_string(&self.parameters).ok();
        let schema = schema.unwrap_or("None".to_string());
        write!(f, "Name: {}:\n", self.name)?;
        write!(f, "Description: {}\n", self.description)?;
        write!(f, "Parameters: {}", schema)
    }
}

pub trait LanguageModelTool {
    /// The input type that will be passed in to `execute` when the tool is called
    /// by the language model.
    type Input: for<'de> Deserialize<'de> + JsonSchema;

    /// The output returned by executing the tool.
    type Output: 'static;

    type View: Render;

    /// Returns the name of the tool.
    ///
    /// This name is exposed to the language model to allow the model to pick
    /// which tools to use. As this name is used to identify the tool within a
    /// tool registry, it should be unique.
    fn name(&self) -> String;

    /// Returns the description of the tool.
    ///
    /// This can be used to _prompt_ the model as to what the tool does.
    fn description(&self) -> String;

    /// Returns the OpenAI Function definition for the tool, for direct use with OpenAI's API.
    fn definition(&self) -> ToolFunctionDefinition {
        let root_schema = schema_for!(Self::Input);

        ToolFunctionDefinition {
            name: self.name(),
            description: self.description(),
            parameters: root_schema,
        }
    }

    /// Executes the tool with the given input.
    fn execute(&self, input: &Self::Input, cx: &mut WindowContext) -> Task<Result<Self::Output>>;

    fn format(input: &Self::Input, output: &Result<Self::Output>) -> String;

    fn output_view(
        tool_call_id: String,
        input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> View<Self::View>;
}
