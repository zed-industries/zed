use anyhow::{anyhow, Result};
use gpui::{AnyElement, AppContext, Task, WindowContext};
use std::{any::Any, collections::HashMap};

use crate::tool::{
    LanguageModelTool, ToolFunctionCall, ToolFunctionCallResult, ToolFunctionDefinition,
};

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(&ToolFunctionCall, &AppContext) -> Task<ToolFunctionCall>>>,
    definitions: Vec<ToolFunctionDefinition>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            definitions: Vec::new(),
        }
    }

    pub fn definitions(&self) -> &[ToolFunctionDefinition] {
        &self.definitions
    }

    pub fn register<T: 'static + LanguageModelTool>(&mut self, tool: T) -> Result<()> {
        fn render<T: 'static + LanguageModelTool>(
            tool_call_id: &str,
            input: &Box<dyn Any>,
            output: &Box<dyn Any>,
            cx: &mut WindowContext,
        ) -> AnyElement {
            T::render(
                tool_call_id,
                input.as_ref().downcast_ref::<T::Input>().unwrap(),
                output.as_ref().downcast_ref::<T::Output>().unwrap(),
                cx,
            )
        }

        fn format<T: 'static + LanguageModelTool>(
            input: &Box<dyn Any>,
            output: &Box<dyn Any>,
        ) -> String {
            T::format(
                input.as_ref().downcast_ref::<T::Input>().unwrap(),
                output.as_ref().downcast_ref::<T::Output>().unwrap(),
            )
        }

        self.definitions.push(tool.definition());
        let name = tool.name();
        let previous = self.tools.insert(
            name.clone(),
            Box::new(move |tool_call: &ToolFunctionCall, cx: &AppContext| {
                let name = tool_call.name.clone();
                let arguments = tool_call.arguments.clone();
                let id = tool_call.id.clone();

                let Ok(input) = serde_json::from_str::<T::Input>(arguments.as_str()) else {
                    return Task::ready(ToolFunctionCall {
                        id,
                        name: name.clone(),
                        arguments,
                        result: Some(ToolFunctionCallResult::ParsingFailed),
                    });
                };

                let result = tool.execute(&input, cx);

                cx.spawn(move |_cx| async move {
                    match result.await {
                        Ok(result) => {
                            let result: T::Output = result;
                            ToolFunctionCall {
                                id,
                                name: name.clone(),
                                arguments,
                                result: Some(ToolFunctionCallResult::Finished {
                                    input: Box::new(input),
                                    output: Box::new(result),
                                    render_fn: render::<T>,
                                    format_fn: format::<T>,
                                }),
                            }
                        }
                        Err(_error) => ToolFunctionCall {
                            id,
                            name: name.clone(),
                            arguments,
                            result: Some(ToolFunctionCallResult::ExecutionFailed {
                                input: Box::new(input),
                            }),
                        },
                    }
                })
            }),
        );

        if previous.is_some() {
            return Err(anyhow!("already registered a tool with name {}", name));
        }

        Ok(())
    }

    pub fn call(&self, tool_call: &ToolFunctionCall, cx: &AppContext) -> Task<ToolFunctionCall> {
        let name = tool_call.name.clone();
        let arguments = tool_call.arguments.clone();
        let id = tool_call.id.clone();

        let tool = match self.tools.get(&name) {
            Some(tool) => tool,
            None => {
                let name = name.clone();
                return Task::ready(ToolFunctionCall {
                    id,
                    name: name.clone(),
                    arguments,
                    result: Some(ToolFunctionCallResult::NoSuchTool),
                });
            }
        };

        tool(tool_call, cx)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use schemars::schema_for;

    use gpui::{div, AnyElement, Element, ParentElement, TestAppContext, WindowContext};
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

    impl LanguageModelTool for WeatherTool {
        type Input = WeatherQuery;
        type Output = WeatherResult;

        fn name(&self) -> String {
            "get_current_weather".to_string()
        }

        fn description(&self) -> String {
            "Fetches the current weather for a given location.".to_string()
        }

        fn execute(&self, input: &WeatherQuery, _cx: &AppContext) -> Task<Result<Self::Output>> {
            let _location = input.location.clone();
            let _unit = input.unit.clone();

            let weather = self.current_weather.clone();

            Task::ready(Ok(weather))
        }

        fn render(
            _tool_call_id: &str,
            _input: &Self::Input,
            output: &Self::Output,
            _cx: &mut WindowContext,
        ) -> AnyElement {
            div()
                .child(format!(
                    "The current temperature in {} is {} {}",
                    output.location, output.temperature, output.unit
                ))
                .into_any()
        }

        fn format(_input: &Self::Input, output: &Self::Output) -> String {
            format!(
                "The current temperature in {} is {} {}",
                output.location, output.temperature, output.unit
            )
        }
    }

    #[gpui::test]
    async fn test_function_registry(cx: &mut TestAppContext) {
        cx.background_executor.run_until_parked();

        let mut registry = ToolRegistry::new();

        let tool = WeatherTool {
            current_weather: WeatherResult {
                location: "San Francisco".to_string(),
                temperature: 21.0,
                unit: "Celsius".to_string(),
            },
        };

        registry.register(tool).unwrap();

        let _result = cx
            .update(|cx| {
                registry.call(
                    &ToolFunctionCall {
                        name: "get_current_weather".to_string(),
                        arguments: r#"{ "location": "San Francisco", "unit": "Celsius" }"#
                            .to_string(),
                        id: "test-123".to_string(),
                        result: None,
                    },
                    cx,
                )
            })
            .await;

        // assert!(result.is_ok());
        // let result = result.unwrap();

        // let expected = r#"{"location":"San Francisco","temperature":21.0,"unit":"Celsius"}"#;

        // todo!(): Put this back in after the interface is stabilized
        // assert_eq!(result, expected);
    }

    #[gpui::test]
    async fn test_openai_weather_example(cx: &mut TestAppContext) {
        cx.background_executor.run_until_parked();

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
