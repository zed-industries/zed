use anyhow::Result;
use gpui::{div, AnyElement, AppContext, Element, Task, WindowContext};
use schemars::{schema::SchemaObject, schema_for, JsonSchema};
use serde::Deserialize;
use std::fmt::Debug;

pub trait ToolFunctionOutput {
    fn render(&self, cx: &mut WindowContext) -> AnyElement;
    fn format(&self) -> String;
    fn boxed_clone(&self) -> Box<dyn ToolFunctionOutput>;
}

impl Debug for dyn ToolFunctionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format().fmt(f)
    }
}

#[derive(Clone)]
pub struct DefaultToolFunctionOutput;

impl ToolFunctionOutput for DefaultToolFunctionOutput {
    fn render(&self, _cx: &mut WindowContext) -> AnyElement {
        div().into_any()
    }

    fn format(&self) -> String {
        "".to_string()
    }

    fn boxed_clone(&self) -> Box<dyn ToolFunctionOutput> {
        Box::new((*self).clone())
    }
}

#[derive(Default, Deserialize, Debug)]
pub struct ToolFunctionCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
    #[serde(skip)]
    pub result: Option<Box<dyn ToolFunctionOutput>>,
}

impl Clone for ToolFunctionCall {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            arguments: self.arguments.clone(),
            result: self.result.as_ref().map(|r| r.boxed_clone()),
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
    type Output: ToolFunctionOutput + 'static;

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
    fn execute(&self, input: Self::Input, cx: &AppContext) -> Task<Result<Self::Output>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::{ParentElement, TestAppContext};
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
            Box::new((*self).clone())
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
            parameters: schema_for!(WeatherQuery).schema,
        };

        assert_eq!(tools[0].name, expected.name);
        assert_eq!(tools[0].description, expected.description);

        let expected_schema = serde_json::to_value(&tools[0].parameters).unwrap();

        assert_eq!(
            expected_schema,
            json!({
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

        let result = cx.update(|cx| tool.execute(query, cx)).await;

        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(result, tool.current_weather);
    }
}
