use anyhow::{anyhow, Result};
use gpui::{AppContext, Task};
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(&str, &AppContext) -> Task<Result<String>>>>,
    pub definitions: Vec<Value>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            definitions: Vec::new(),
        }
    }

    pub fn register<T: 'static + LanguageModelTool>(&mut self, tool: T) -> Result<()> {
        self.definitions.push(tool.definition());
        let name = tool.name();
        let previous = self.tools.insert(
            name.clone(),
            Box::new(move |args: &str, cx: &AppContext| {
                let result = match serde_json::from_str::<T::Input>(&args) {
                    Ok(input) => tool.execute(input, cx),
                    Err(error) => return Task::ready(Err(anyhow!(error))),
                };

                cx.spawn(|_cx| async move {
                    let result: T::Output = result.await?;
                    Ok(serde_json::to_string(&result)?)
                })
            }),
        );

        if previous.is_some() {
            return Err(anyhow!("already registered a tool with name {}", name));
        }

        Ok(())
    }

    pub fn call(&self, name: &str, input: &str, cx: &AppContext) -> Task<Result<String>> {
        let tool = match self.tools.get(name) {
            Some(tool) => tool,
            None => {
                return Task::ready(Err(anyhow!("no tool registered with name {}", name)));
            }
        };

        tool(input, cx)
    }
}

pub trait LanguageModelTool {
    /// The input type that will be passed in to `execute` when the tool is called
    /// by the language model.
    type Input: for<'de> Deserialize<'de> + JsonSchema;

    /// The output returned by executing the tool.
    type Output: Serialize;

    /// The name of the tool is exposed to the language model to allow
    /// the model to pick which tools to use. As this name is used to
    /// identify the tool within a tool registry, it should be unique.
    fn name(&self) -> String;

    /// A description of the tool that can be used to _prompt_ the model
    /// as to what the tool does.
    fn description(&self) -> String;

    fn input_schema(&self) -> serde_json::Value {
        let schema: RootSchema = schema_for!(Self::Input);
        serde_json::to_value(schema).unwrap()
    }

    /// The OpenAI Function definition for the tool, for direct use with OpenAI's API.
    fn definition(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": self.description(),
                "schema": self.input_schema()
            }
        })
    }

    /// Execute the tool
    fn execute(&self, input: Self::Input, cx: &AppContext) -> Task<Result<Self::Output>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::TestAppContext;

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

        fn execute(&self, input: WeatherQuery, _cx: &AppContext) -> Task<Result<Self::Output>> {
            let _location = input.location.clone();
            let _unit = input.unit.clone();

            let weather = self.current_weather.clone();

            Task::ready(Ok(weather))
        }
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
        assert_eq!(
            tools,
            vec![json!({
                "type": "function",
                "function": {
                    "name": "get_current_weather",
                    "description": "Fetches the current weather for a given location.",
                    "schema": {
                        // TODO: Check if OpenAI can ignore this field
                        "$schema": "http://json-schema.org/draft-07/schema#",
                        // TODO: Check if OpenAI can ignore this field
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
                    }
                }
            })]
        );

        let args = json!({
            "location": "San Francisco",
            "unit": "Celsius"
        });

        let query: WeatherQuery = serde_json::from_value(args).unwrap();

        let result = cx.update(|cx| tool.execute(query, cx)).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, tool.current_weather);
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

        let result = cx
            .update(|cx| {
                registry.call(
                    "get_current_weather",
                    r#"{ "location": "San Francisco", "unit": "Celsius" }"#,
                    cx,
                )
            })
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();

        let expected = r#"{"location":"San Francisco","temperature":21.0,"unit":"Celsius"}"#;

        assert_eq!(result, expected);
    }
}
