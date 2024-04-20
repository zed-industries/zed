use anyhow::{anyhow, Result};
use gpui::{AppContext, Task};
use std::collections::HashMap;

use crate::tool::{LanguageModelTool, ToolFunctionDefinition};

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(&str, &AppContext) -> Task<Result<String>>>>,
    pub definitions: Vec<ToolFunctionDefinition>,
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

#[cfg(test)]
mod test {
    use super::*;

    use gpui::TestAppContext;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

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
