use anyhow::{anyhow, Error, Result};
use gpui::{div, AnyElement, AppContext, Element, ParentElement, Task, WindowContext};
use std::{collections::HashMap, sync::Arc};

use crate::tool::{
    LanguageModelTool, ToolFunctionCall, ToolFunctionDefinition, ToolFunctionOutput,
};

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(&ToolFunctionCall, &AppContext) -> Task<ToolFunctionCall>>>,
    definitions: Vec<ToolFunctionDefinition>,
}

// Since we're centering on the tool registry always returning Tasks with associated Tool IDs,
// we will center on always returning a `ToolFunctionCall`. As a result we need to have different
// error outputs that implement `ToolFunctionOutput`.
#[derive(Clone)]
struct NoToolForName {
    name: String,
}

impl NoToolForName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl ToolFunctionOutput for NoToolForName {
    fn render(&self, _cx: &mut WindowContext) -> AnyElement {
        let name = self.name.clone();
        div().child(format!("No tool found for {name}")).into_any()
    }

    fn format(&self) -> String {
        let name = self.name.clone();
        format!("No tool found for {name}")
    }

    fn boxed_clone(&self) -> Box<dyn ToolFunctionOutput> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
struct FailedToParseArguments {
    name: String,
    error: String,
}

impl FailedToParseArguments {
    pub fn new(name: String, error: serde_json::Error) -> Self {
        Self {
            name,
            error: error.to_string(),
        }
    }
}

impl ToolFunctionOutput for FailedToParseArguments {
    fn render(&self, _cx: &mut WindowContext) -> AnyElement {
        let message = self.error.clone();
        let name = self.name.clone();

        div()
            .child(format!("Model sent bad input for {name}: {message}"))
            .into_any()
    }

    fn format(&self) -> String {
        self.error.clone()
    }

    fn boxed_clone(&self) -> Box<dyn ToolFunctionOutput> {
        Box::new((*self).clone())
    }
}

// Generic error output for when a tool fails to execute.
#[derive(Clone)]
struct ToolExecutionError {
    name: String,
    error: String,
}

impl ToolExecutionError {
    pub fn new(name: String, error: Error) -> Self {
        Self {
            name,
            error: error.to_string(),
        }
    }
}

impl ToolFunctionOutput for ToolExecutionError {
    fn render(&self, _cx: &mut WindowContext) -> AnyElement {
        let error = self.error.clone();
        let name = self.name.clone();

        div()
            .child(format!("Error executing tool {name}: {error}"))
            .into_any()
    }

    fn format(&self) -> String {
        self.error.clone()
    }

    fn boxed_clone(&self) -> Box<dyn ToolFunctionOutput> {
        Box::new((*self).clone())
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            definitions: Vec::new(),
        }
    }

    // pub fn definitions(&self) -> Arc<&[ToolFunctionDefinition]> {
    //     Arc::new(self.definitions.as_slice())
    // }

    pub fn definitions(&self) -> &[ToolFunctionDefinition] {
        &self.definitions
    }

    pub fn register<T: 'static + LanguageModelTool>(&mut self, tool: T) -> Result<()> {
        self.definitions.push(tool.definition());
        let name = tool.name();
        let previous = self.tools.insert(
            name.clone(),
            Box::new(move |tool_call: &ToolFunctionCall, cx: &AppContext| {
                let name = tool_call.name.clone();
                let arguments = tool_call.arguments.clone();
                let id = tool_call.id.clone();

                let result = match serde_json::from_str::<T::Input>(arguments.as_str()) {
                    Ok(input) => tool.execute(input, cx),
                    Err(error) => {
                        return Task::ready(ToolFunctionCall {
                            id,
                            name: name.clone(),
                            arguments,
                            result: Some(Box::new(FailedToParseArguments::new(name, error))),
                        })
                    }
                };

                cx.spawn(|_cx| async move {
                    let result = result.await;

                    match result {
                        Ok(result) => {
                            let result: T::Output = result;
                            ToolFunctionCall {
                                id,
                                name,
                                arguments,
                                result: Some(Box::new(result)),
                            }
                        }
                        Err(error) => ToolFunctionCall {
                            id,
                            name: name.clone(),
                            arguments,
                            result: Some(Box::new(ToolExecutionError::new(name.clone(), error))),
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
                    result: Some(Box::new(NoToolForName::new(name))),
                });
            }
        };

        tool(tool_call, cx)
    }
}

#[cfg(test)]
mod test {
    use crate::tool::ToolFunctionOutput;

    use super::*;

    use gpui::{div, AnyElement, Element, ParentElement, TestAppContext, WindowContext};
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

    impl ToolFunctionOutput for WeatherResult {
        fn render(&self, _cx: &mut WindowContext) -> AnyElement {
            div()
                .child(format!(
                    "The current temperature in {} is {} {}",
                    self.location, self.temperature, self.unit
                ))
                .into_any()
        }

        fn format(&self) -> String {
            format!(
                "The current temperature in {} is {} {}",
                self.location, self.temperature, self.unit
            )
        }

        fn boxed_clone(&self) -> Box<dyn ToolFunctionOutput> {
            Box::new(self.clone())
        }
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
}
