use crate::ProjectContext;
use anyhow::{anyhow, Result};
use gpui::{AnyElement, AnyView, IntoElement, Render, Task, View, WindowContext};
use repair_json::repair;
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{
    any::TypeId,
    collections::HashMap,
    fmt::Display,
    mem,
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};
use ui::ViewContext;

pub struct ToolRegistry {
    registered_tools: HashMap<String, RegisteredTool>,
}

#[derive(Default)]
pub struct ToolFunctionCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
    state: ToolFunctionCallState,
}

#[derive(Default)]
enum ToolFunctionCallState {
    #[default]
    Initializing,
    NoSuchTool,
    KnownTool(Box<dyn InternalToolView>),
    ExecutedTool(Box<dyn InternalToolView>),
}

trait InternalToolView {
    fn view(&self) -> AnyView;
    fn generate(&self, project: &mut ProjectContext, cx: &mut WindowContext) -> String;
    fn try_set_input(&self, input: &str, cx: &mut WindowContext);
    fn execute(&self, cx: &mut WindowContext) -> Task<Result<()>>;
    fn serialize_output(&self, cx: &mut WindowContext) -> Result<Box<RawValue>>;
    fn deserialize_output(&self, raw_value: &RawValue, cx: &mut WindowContext) -> Result<()>;
}

#[derive(Default, Serialize, Deserialize)]
pub struct SavedToolFunctionCall {
    id: String,
    name: String,
    arguments: String,
    state: SavedToolFunctionCallState,
}

#[derive(Default, Serialize, Deserialize)]
enum SavedToolFunctionCallState {
    #[default]
    Initializing,
    NoSuchTool,
    KnownTool,
    ExecutedTool(Box<RawValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToolFunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: RootSchema,
}

pub trait LanguageModelTool {
    type View: ToolView;

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
        let root_schema = schema_for!(<Self::View as ToolView>::Input);

        ToolFunctionDefinition {
            name: self.name(),
            description: self.description(),
            parameters: root_schema,
        }
    }

    /// A view of the output of running the tool, for displaying to the user.
    fn view(&self, cx: &mut WindowContext) -> View<Self::View>;
}

pub trait ToolView: Render {
    /// The input type that will be passed in to `execute` when the tool is called
    /// by the language model.
    type Input: DeserializeOwned + JsonSchema;

    /// The output returned by executing the tool.
    type SerializedState: DeserializeOwned + Serialize;

    fn generate(&self, project: &mut ProjectContext, cx: &mut ViewContext<Self>) -> String;
    fn set_input(&mut self, input: Self::Input, cx: &mut ViewContext<Self>);
    fn execute(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>>;

    fn serialize(&self, cx: &mut ViewContext<Self>) -> Self::SerializedState;
    fn deserialize(
        &mut self,
        output: Self::SerializedState,
        cx: &mut ViewContext<Self>,
    ) -> Result<()>;
}

struct RegisteredTool {
    enabled: AtomicBool,
    type_id: TypeId,
    build_view: Box<dyn Fn(&mut WindowContext) -> Box<dyn InternalToolView>>,
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

    pub fn update_tool_call(
        &self,
        call: &mut ToolFunctionCall,
        name: Option<&str>,
        arguments: Option<&str>,
        cx: &mut WindowContext,
    ) {
        if let Some(name) = name {
            call.name.push_str(name);
        }
        if let Some(arguments) = arguments {
            if call.arguments.is_empty() {
                if let Some(tool) = self.registered_tools.get(&call.name) {
                    let view = (tool.build_view)(cx);
                    call.state = ToolFunctionCallState::KnownTool(view);
                } else {
                    call.state = ToolFunctionCallState::NoSuchTool;
                }
            }
            call.arguments.push_str(arguments);

            if let ToolFunctionCallState::KnownTool(view) = &call.state {
                if let Ok(repaired_arguments) = repair(call.arguments.clone()) {
                    view.try_set_input(&repaired_arguments, cx)
                }
            }
        }
    }

    pub fn execute_tool_call(
        &self,
        tool_call: &mut ToolFunctionCall,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<()>>> {
        if let ToolFunctionCallState::KnownTool(view) = mem::take(&mut tool_call.state) {
            let task = view.execute(cx);
            tool_call.state = ToolFunctionCallState::ExecutedTool(view);
            Some(task)
        } else {
            None
        }
    }

    pub fn render_tool_call(
        &self,
        tool_call: &ToolFunctionCall,
        _cx: &mut WindowContext,
    ) -> Option<AnyElement> {
        match &tool_call.state {
            ToolFunctionCallState::NoSuchTool => {
                Some(ui::Label::new("No such tool").into_any_element())
            }
            ToolFunctionCallState::Initializing => None,
            ToolFunctionCallState::KnownTool(view) | ToolFunctionCallState::ExecutedTool(view) => {
                Some(view.view().into_any_element())
            }
        }
    }

    pub fn content_for_tool_call(
        &self,
        tool_call: &ToolFunctionCall,
        project_context: &mut ProjectContext,
        cx: &mut WindowContext,
    ) -> String {
        match &tool_call.state {
            ToolFunctionCallState::Initializing => String::new(),
            ToolFunctionCallState::NoSuchTool => {
                format!("No such tool: {}", tool_call.name)
            }
            ToolFunctionCallState::KnownTool(view) | ToolFunctionCallState::ExecutedTool(view) => {
                view.generate(project_context, cx)
            }
        }
    }

    pub fn serialize_tool_call(
        &self,
        call: &ToolFunctionCall,
        cx: &mut WindowContext,
    ) -> Result<SavedToolFunctionCall> {
        Ok(SavedToolFunctionCall {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
            state: match &call.state {
                ToolFunctionCallState::Initializing => SavedToolFunctionCallState::Initializing,
                ToolFunctionCallState::NoSuchTool => SavedToolFunctionCallState::NoSuchTool,
                ToolFunctionCallState::KnownTool(_) => SavedToolFunctionCallState::KnownTool,
                ToolFunctionCallState::ExecutedTool(view) => {
                    SavedToolFunctionCallState::ExecutedTool(view.serialize_output(cx)?)
                }
            },
        })
    }

    pub fn deserialize_tool_call(
        &self,
        call: &SavedToolFunctionCall,
        cx: &mut WindowContext,
    ) -> Result<ToolFunctionCall> {
        let Some(tool) = self.registered_tools.get(&call.name) else {
            return Err(anyhow!("no such tool {}", call.name));
        };

        Ok(ToolFunctionCall {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
            state: match &call.state {
                SavedToolFunctionCallState::Initializing => ToolFunctionCallState::Initializing,
                SavedToolFunctionCallState::NoSuchTool => ToolFunctionCallState::NoSuchTool,
                SavedToolFunctionCallState::KnownTool => {
                    log::error!("Deserialized tool that had not executed");
                    let view = (tool.build_view)(cx);
                    view.try_set_input(&call.arguments, cx);
                    ToolFunctionCallState::KnownTool(view)
                }
                SavedToolFunctionCallState::ExecutedTool(output) => {
                    let view = (tool.build_view)(cx);
                    view.try_set_input(&call.arguments, cx);
                    view.deserialize_output(output, cx)?;
                    ToolFunctionCallState::ExecutedTool(view)
                }
            },
        })
    }

    pub fn register<T: 'static + LanguageModelTool>(&mut self, tool: T) -> Result<()> {
        let name = tool.name();
        let registered_tool = RegisteredTool {
            type_id: TypeId::of::<T>(),
            definition: tool.definition(),
            enabled: AtomicBool::new(true),
            build_view: Box::new(move |cx: &mut WindowContext| Box::new(tool.view(cx))),
        };

        let previous = self.registered_tools.insert(name.clone(), registered_tool);
        if previous.is_some() {
            return Err(anyhow!("already registered a tool with name {}", name));
        }

        return Ok(());
    }
}

impl<T: ToolView> InternalToolView for View<T> {
    fn view(&self) -> AnyView {
        self.clone().into()
    }

    fn generate(&self, project: &mut ProjectContext, cx: &mut WindowContext) -> String {
        self.update(cx, |view, cx| view.generate(project, cx))
    }

    fn try_set_input(&self, input: &str, cx: &mut WindowContext) {
        if let Ok(input) = serde_json::from_str::<T::Input>(input) {
            self.update(cx, |view, cx| {
                view.set_input(input, cx);
                cx.notify();
            });
        }
    }

    fn execute(&self, cx: &mut WindowContext) -> Task<Result<()>> {
        self.update(cx, |view, cx| view.execute(cx))
    }

    fn serialize_output(&self, cx: &mut WindowContext) -> Result<Box<RawValue>> {
        let output = self.update(cx, |view, cx| view.serialize(cx));
        Ok(RawValue::from_string(serde_json::to_string(&output)?)?)
    }

    fn deserialize_output(&self, output: &RawValue, cx: &mut WindowContext) -> Result<()> {
        let state = serde_json::from_str::<T::SerializedState>(output.get())?;
        self.update(cx, |view, cx| view.deserialize(state, cx))?;
        Ok(())
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
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Deserialize, Serialize, JsonSchema)]
    struct WeatherQuery {
        location: String,
        unit: String,
    }

    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    struct WeatherResult {
        location: String,
        temperature: f64,
        unit: String,
    }

    struct WeatherView {
        input: Option<WeatherQuery>,
        result: Option<WeatherResult>,

        // Fake API call
        current_weather: WeatherResult,
    }

    #[derive(Clone, Serialize)]
    struct WeatherTool {
        current_weather: WeatherResult,
    }

    impl WeatherView {
        fn new(current_weather: WeatherResult) -> Self {
            Self {
                input: None,
                result: None,
                current_weather,
            }
        }
    }

    impl Render for WeatherView {
        fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
            match self.result {
                Some(ref result) => div()
                    .child(format!("temperature: {}", result.temperature))
                    .into_any_element(),
                None => div().child("Calculating weather...").into_any_element(),
            }
        }
    }

    impl ToolView for WeatherView {
        type Input = WeatherQuery;

        type SerializedState = WeatherResult;

        fn generate(&self, _output: &mut ProjectContext, _cx: &mut ViewContext<Self>) -> String {
            serde_json::to_string(&self.result).unwrap()
        }

        fn set_input(&mut self, input: Self::Input, cx: &mut ViewContext<Self>) {
            self.input = Some(input);
            cx.notify();
        }

        fn execute(&mut self, _cx: &mut ViewContext<Self>) -> Task<Result<()>> {
            let input = self.input.as_ref().unwrap();

            let _location = input.location.clone();
            let _unit = input.unit.clone();

            let weather = self.current_weather.clone();

            self.result = Some(weather);

            Task::ready(Ok(()))
        }

        fn serialize(&self, _cx: &mut ViewContext<Self>) -> Self::SerializedState {
            self.current_weather.clone()
        }

        fn deserialize(
            &mut self,
            output: Self::SerializedState,
            _cx: &mut ViewContext<Self>,
        ) -> Result<()> {
            self.current_weather = output;
            Ok(())
        }
    }

    impl LanguageModelTool for WeatherTool {
        type View = WeatherView;

        fn name(&self) -> String {
            "get_current_weather".to_string()
        }

        fn description(&self) -> String {
            "Fetches the current weather for a given location.".to_string()
        }

        fn view(&self, cx: &mut WindowContext) -> View<Self::View> {
            cx.new_view(|_cx| WeatherView::new(self.current_weather.clone()))
        }
    }

    #[gpui::test]
    async fn test_openai_weather_example(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_cx| EmptyView);

        let mut registry = ToolRegistry::new();
        registry
            .register(WeatherTool {
                current_weather: WeatherResult {
                    location: "San Francisco".to_string(),
                    temperature: 21.0,
                    unit: "Celsius".to_string(),
                },
            })
            .unwrap();

        let definitions = registry.definitions();
        assert_eq!(
            definitions,
            [ToolFunctionDefinition {
                name: "get_current_weather".to_string(),
                description: "Fetches the current weather for a given location.".to_string(),
                parameters: serde_json::from_value(json!({
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
                }))
                .unwrap(),
            }]
        );

        let mut call = ToolFunctionCall {
            id: "the-id".to_string(),
            name: "get_cur".to_string(),
            ..Default::default()
        };

        let task = cx.update(|cx| {
            registry.update_tool_call(
                &mut call,
                Some("rent_weather"),
                Some(r#"{"location": "San Francisco","#),
                cx,
            );
            registry.update_tool_call(&mut call, None, Some(r#" "unit": "Celsius"}"#), cx);
            registry.execute_tool_call(&mut call, cx).unwrap()
        });
        task.await.unwrap();

        match &call.state {
            ToolFunctionCallState::ExecutedTool(_view) => {}
            _ => panic!(),
        }
    }
}
