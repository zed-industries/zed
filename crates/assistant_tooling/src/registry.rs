use anyhow::{anyhow, Result};
use gpui::{Task, WindowContext};
use std::{
    any::TypeId,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

use crate::tool::{
    LanguageModelTool, ToolFunctionCall, ToolFunctionCallResult, ToolFunctionDefinition,
};

// Internal Tool representation for the registry
pub struct Tool {
    enabled: AtomicBool,
    type_id: TypeId,
    call: Box<dyn Fn(&ToolFunctionCall, &mut WindowContext) -> Task<Result<ToolFunctionCall>>>,
    definition: ToolFunctionDefinition,
}

impl Tool {
    fn new(
        type_id: TypeId,
        call: Box<dyn Fn(&ToolFunctionCall, &mut WindowContext) -> Task<Result<ToolFunctionCall>>>,
        definition: ToolFunctionDefinition,
    ) -> Self {
        Self {
            enabled: AtomicBool::new(true),
            type_id,
            call,
            definition,
        }
    }
}

pub struct ToolRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn set_tool_enabled<T: 'static + LanguageModelTool>(&self, is_enabled: bool) {
        for tool in self.tools.values() {
            if tool.type_id == TypeId::of::<T>() {
                tool.enabled.store(is_enabled, SeqCst);
                return;
            }
        }
    }

    pub fn is_tool_enabled<T: 'static + LanguageModelTool>(&self) -> bool {
        for tool in self.tools.values() {
            if tool.type_id == TypeId::of::<T>() {
                return tool.enabled.load(SeqCst);
            }
        }
        false
    }

    pub fn definitions(&self) -> Vec<ToolFunctionDefinition> {
        self.tools
            .values()
            .filter(|tool| tool.enabled.load(SeqCst))
            .map(|tool| tool.definition.clone())
            .collect()
    }

    pub fn register<T: 'static + LanguageModelTool>(
        &mut self,
        tool: T,
        _cx: &mut WindowContext,
    ) -> Result<()> {
        let definition = tool.definition();

        let name = tool.name();

        let registered_tool = Tool::new(
            TypeId::of::<T>(),
            Box::new(
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
                        let for_model = T::format(&input, &result);
                        let view = cx.update(|cx| T::output_view(id.clone(), input, result, cx))?;

                        Ok(ToolFunctionCall {
                            id,
                            name: name.clone(),
                            arguments,
                            result: Some(ToolFunctionCallResult::Finished {
                                view: view.into(),
                                for_model,
                            }),
                        })
                    })
                },
            ),
            definition,
        );

        let previous = self.tools.insert(name.clone(), registered_tool);

        if previous.is_some() {
            return Err(anyhow!("already registered a tool with name {}", name));
        }

        Ok(())
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

        let tool = match self.tools.get(&name) {
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
            _tool_call_id: String,
            _input: Self::Input,
            result: Result<Self::Output>,
            cx: &mut WindowContext,
        ) -> View<Self::View> {
            cx.new_view(|_cx| {
                let result = result.unwrap();
                WeatherView { result }
            })
        }

        fn format(_: &Self::Input, output: &Result<Self::Output>) -> String {
            serde_json::to_string(&output.as_ref().unwrap()).unwrap()
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
