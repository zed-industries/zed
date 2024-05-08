use anyhow::{anyhow, Result};
use gpui::{
    div, AnyElement, AnyView, IntoElement, ParentElement, Render, Styled, Task, View, WindowContext,
};
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::Deserialize;
use serde_json::Value;
use std::{
    any::TypeId,
    collections::HashMap,
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

use crate::ProjectContext;

pub struct ToolRegistry {
    registered_tools: HashMap<String, RegisteredTool>,
}

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
    Finished {
        view: AnyView,
        generate_fn: fn(AnyView, &mut ProjectContext, &mut WindowContext) -> String,
    },
}

#[derive(Clone)]
pub struct ToolFunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: RootSchema,
}

pub trait LanguageModelTool {
    /// The input type that will be passed in to `execute` when the tool is called
    /// by the language model.
    type Input: for<'de> Deserialize<'de> + JsonSchema;

    /// The output returned by executing the tool.
    type Output: 'static;

    type View: Render + ToolOutput;

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

    /// A view of the output of running the tool, for displaying to the user.
    fn output_view(
        input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> View<Self::View>;

    fn render_running(_arguments: &Option<Value>, _cx: &mut WindowContext) -> impl IntoElement {
        tool_running_placeholder()
    }
}

pub fn tool_running_placeholder() -> AnyElement {
    ui::Label::new("Researching...").into_any_element()
}

pub trait ToolOutput: Sized {
    fn generate(&self, project: &mut ProjectContext, cx: &mut WindowContext) -> String;
}

struct RegisteredTool {
    enabled: AtomicBool,
    type_id: TypeId,
    call: Box<dyn Fn(&ToolFunctionCall, &mut WindowContext) -> Task<Result<ToolFunctionCall>>>,
    render_running: fn(&ToolFunctionCall, &mut WindowContext) -> gpui::AnyElement,
    definition: ToolFunctionDefinition,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            registered_tools: HashMap::new(),
        }
    }

    pub fn set_tool_enabled<T: 'static + LanguageModelTool>(&self, is_enabled: bool) {
        for tool in self.registered_tools.values() {
            if tool.type_id == TypeId::of::<T>() {
                tool.enabled.store(is_enabled, SeqCst);
                return;
            }
        }
    }

    pub fn is_tool_enabled<T: 'static + LanguageModelTool>(&self) -> bool {
        for tool in self.registered_tools.values() {
            if tool.type_id == TypeId::of::<T>() {
                return tool.enabled.load(SeqCst);
            }
        }
        false
    }

    pub fn definitions(&self) -> Vec<ToolFunctionDefinition> {
        self.registered_tools
            .values()
            .filter(|tool| tool.enabled.load(SeqCst))
            .map(|tool| tool.definition.clone())
            .collect()
    }

    pub fn render_tool_call(
        &self,
        tool_call: &ToolFunctionCall,
        cx: &mut WindowContext,
    ) -> AnyElement {
        match &tool_call.result {
            Some(result) => div()
                .p_2()
                .child(result.into_any_element(&tool_call.name))
                .into_any_element(),
            None => {
                let tool = self.registered_tools.get(&tool_call.name);

                if let Some(tool) = tool {
                    (tool.render_running)(&tool_call, cx)
                } else {
                    tool_running_placeholder()
                }
            }
        }
    }

    pub fn register<T: 'static + LanguageModelTool>(
        &mut self,
        tool: T,
        _cx: &mut WindowContext,
    ) -> Result<()> {
        let name = tool.name();
        let registered_tool = RegisteredTool {
            type_id: TypeId::of::<T>(),
            definition: tool.definition(),
            enabled: AtomicBool::new(true),
            call: Box::new(
                move |tool_call: &ToolFunctionCall, cx: &mut WindowContext| {
                    let name = tool_call.name.clone();
                    let arguments = tool_call.arguments.clone();
                    let id = tool_call.id.clone();

                    let Ok(input) = serde_json::from_str::<T::Input>(arguments.as_str()) else {
                        return Task::ready(Ok(ToolFunctionCall {
                            id,
                            name: name.clone(),
                            arguments,
                            result: Some(ToolFunctionCallResult::ParsingFailed),
                        }));
                    };

                    let result = tool.execute(&input, cx);

                    cx.spawn(move |mut cx| async move {
                        let result: Result<T::Output> = result.await;
                        let view = cx.update(|cx| T::output_view(input, result, cx))?;

                        Ok(ToolFunctionCall {
                            id,
                            name: name.clone(),
                            arguments,
                            result: Some(ToolFunctionCallResult::Finished {
                                view: view.into(),
                                generate_fn: generate::<T>,
                            }),
                        })
                    })
                },
            ),
            render_running: render_running::<T>,
        };

        let previous = self.registered_tools.insert(name.clone(), registered_tool);
        if previous.is_some() {
            return Err(anyhow!("already registered a tool with name {}", name));
        }

        return Ok(());

        fn render_running<T: LanguageModelTool>(
            tool_call: &ToolFunctionCall,
            cx: &mut WindowContext,
        ) -> AnyElement {
            // Attempt to parse the string arguments that are JSON as a JSON value
            let maybe_arguments = serde_json::to_value(tool_call.arguments.clone()).ok();

            T::render_running(&maybe_arguments, cx).into_any_element()
        }

        fn generate<T: LanguageModelTool>(
            view: AnyView,
            project: &mut ProjectContext,
            cx: &mut WindowContext,
        ) -> String {
            view.downcast::<T::View>()
                .unwrap()
                .update(cx, |view, cx| T::View::generate(view, project, cx))
        }
    }

    /// Task yields an error if the window for the given WindowContext is closed before the task completes.
    pub fn call(
        &self,
        tool_call: &ToolFunctionCall,
        cx: &mut WindowContext,
    ) -> Task<Result<ToolFunctionCall>> {
        let name = tool_call.name.clone();
        let arguments = tool_call.arguments.clone();
        let id = tool_call.id.clone();

        let tool = match self.registered_tools.get(&name) {
            Some(tool) => tool,
            None => {
                let name = name.clone();
                return Task::ready(Ok(ToolFunctionCall {
                    id,
                    name: name.clone(),
                    arguments,
                    result: Some(ToolFunctionCallResult::NoSuchTool),
                }));
            }
        };

        (tool.call)(tool_call, cx)
    }
}

impl ToolFunctionCallResult {
    pub fn generate(
        &self,
        name: &String,
        project: &mut ProjectContext,
        cx: &mut WindowContext,
    ) -> String {
        match self {
            ToolFunctionCallResult::NoSuchTool => format!("No tool for {name}"),
            ToolFunctionCallResult::ParsingFailed => {
                format!("Unable to parse arguments for {name}")
            }
            ToolFunctionCallResult::Finished { generate_fn, view } => {
                (generate_fn)(view.clone(), project, cx)
            }
        }
    }

    fn into_any_element(&self, name: &String) -> AnyElement {
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

impl Display for ToolFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let schema = serde_json::to_string(&self.parameters).ok();
        let schema = schema.unwrap_or("None".to_string());
        write!(f, "Name: {}:\n", self.name)?;
        write!(f, "Description: {}\n", self.description)?;
        write!(f, "Parameters: {}", schema)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gpui::{div, prelude::*, Render, TestAppContext};
    use gpui::{EmptyView, View};
    use schemars::schema_for;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Deserialize, Serialize, JsonSchema)]
    struct WeatherQuery {
        location: String,
        unit: String,
    }

    struct WeatherTool {
        current_weather: WeatherResult,
    }

    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    struct WeatherResult {
        location: String,
        temperature: f64,
        unit: String,
    }

    struct WeatherView {
        result: WeatherResult,
    }

    impl Render for WeatherView {
        fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
            div().child(format!("temperature: {}", self.result.temperature))
        }
    }

    impl ToolOutput for WeatherView {
        fn generate(&self, _output: &mut ProjectContext, _cx: &mut WindowContext) -> String {
            serde_json::to_string(&self.result).unwrap()
        }
    }

    impl LanguageModelTool for WeatherTool {
        type Input = WeatherQuery;
        type Output = WeatherResult;
        type View = WeatherView;

        fn name(&self) -> String {
            "get_current_weather".to_string()
        }

        fn description(&self) -> String {
            "Fetches the current weather for a given location.".to_string()
        }

        fn execute(
            &self,
            input: &Self::Input,
            _cx: &mut WindowContext,
        ) -> Task<Result<Self::Output>> {
            let _location = input.location.clone();
            let _unit = input.unit.clone();

            let weather = self.current_weather.clone();

            Task::ready(Ok(weather))
        }

        fn output_view(
            _input: Self::Input,
            result: Result<Self::Output>,
            cx: &mut WindowContext,
        ) -> View<Self::View> {
            cx.new_view(|_cx| {
                let result = result.unwrap();
                WeatherView { result }
            })
        }
    }

    #[gpui::test]
    async fn test_openai_weather_example(cx: &mut TestAppContext) {
        cx.background_executor.run_until_parked();
        let (_, cx) = cx.add_window_view(|_cx| EmptyView);

        let tool = WeatherTool {
            current_weather: WeatherResult {
                location: "San Francisco".to_string(),
                temperature: 21.0,
                unit: "Celsius".to_string(),
            },
        };

        let tools = vec![tool.definition()];
        assert_eq!(tools.len(), 1);

        let expected = ToolFunctionDefinition {
            name: "get_current_weather".to_string(),
            description: "Fetches the current weather for a given location.".to_string(),
            parameters: schema_for!(WeatherQuery),
        };

        assert_eq!(tools[0].name, expected.name);
        assert_eq!(tools[0].description, expected.description);

        let expected_schema = serde_json::to_value(&tools[0].parameters).unwrap();

        assert_eq!(
            expected_schema,
            json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "title": "WeatherQuery",
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string"
                    },
                    "unit": {
                        "type": "string"
                    }
                },
                "required": ["location", "unit"]
            })
        );

        let args = json!({
            "location": "San Francisco",
            "unit": "Celsius"
        });

        let query: WeatherQuery = serde_json::from_value(args).unwrap();

        let result = cx.update(|cx| tool.execute(&query, cx)).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, tool.current_weather);
    }
}
