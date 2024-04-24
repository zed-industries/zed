use anyhow::Result;
use gpui::{
    AnyElement, AnyView, AppContext, IntoElement as _, Render, Task, View, ViewContext,
    WindowContext,
};
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
    Finished(Box<dyn AnyToolView>),
}

impl ToolFunctionCallResult {
    pub fn format(&self, name: &String, cx: &mut WindowContext) -> String {
        match self {
            ToolFunctionCallResult::NoSuchTool => format!("No tool for {name}"),
            ToolFunctionCallResult::ParsingFailed => {
                format!("Unable to parse arguments for {name}")
            }
            ToolFunctionCallResult::Finished(view) => view.format(cx),
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
            ToolFunctionCallResult::Finished(view) => {
                let view = view.to_view();
                view.into_any_element()
            }
        }
    }
}

pub trait ToolView: Render {
    fn format(&mut self, cx: &mut ViewContext<Self>) -> String;
}

pub trait AnyToolView {
    fn format(&self, cx: &mut WindowContext) -> String;
    fn to_view(&self) -> AnyView;
}

impl<V: ToolView> AnyToolView for View<V> {
    fn format(&self, cx: &mut WindowContext) -> String {
        self.update(cx, |this, cx| this.format(cx))
    }

    fn to_view(&self) -> AnyView {
        self.clone().into()
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

    type View: ToolView;

    /// The name of the tool is exposed to the language model to allow
    /// the model to pick which tools to use. As this name is used to
    /// identify the tool within a tool registry, it should be unique.
    fn name(&self) -> String;

    /// A description of the tool that can be used to _prompt_ the model
    /// as to what the tool does.
    fn description(&self) -> String;

    /// The OpenAI Function definition for the tool, for direct use with OpenAI's API.
    fn definition(&self) -> ToolFunctionDefinition {
        let root_schema = schema_for!(Self::Input);

        ToolFunctionDefinition {
            name: self.name(),
            description: self.description(),
            parameters: root_schema,
        }
    }

    /// Execute the tool
    fn execute(&self, input: &Self::Input, cx: &AppContext) -> Task<Result<Self::Output>>;

    fn new_view(
        tool_call_id: String,
        input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> View<Self::View>;
}
