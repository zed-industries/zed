use anyhow::Result;
use futures::{future::BoxFuture, Future};
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, pin::Pin};

pub struct FunctionRegistry {
    function_calls: HashMap<String, Box<dyn Fn(String) -> BoxFuture<'static, Result<String>>>>,
    tools: Vec<Value>,
}

impl FunctionRegistry {
    pub fn register<F: FunctionCall + 'static + Send>(&mut self) -> Result<()> {
        let name = F::name().to_string();
        self.function_calls.insert(
            name,
            Box::new(move |args: String| -> BoxFuture<'static, Result<String>> {
                Box::pin(async move {
                    let query: F = serde_json::from_str(&args)?;
                    let result = query.execute().await?;
                    Ok(serde_json::to_string(&result)?)
                })
            }),
        );

        self.tools.push(F::definition());

        Ok(())
    }
}

pub type FunctionCallTask<'a, T> = Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send + 'a>>;

pub trait FunctionCall: Serialize + for<'de> Deserialize<'de> + JsonSchema {
    type Output: Serialize;

    fn name() -> &'static str;
    fn description() -> &'static str;

    fn schema() -> serde_json::Value {
        let schema: RootSchema = schema_for!(Self).into();
        serde_json::to_value(schema).unwrap()
    }

    fn definition() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": Self::name(),
                "description": Self::description(),
                "schema": Self::schema()
            }
        })
    }

    fn execute(&self) -> FunctionCallTask<Self::Output>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_openai_weather_example(cx: &mut TestAppContext) {
        cx.background_executor.run_until_parked();

        #[derive(Deserialize, Serialize, JsonSchema)]
        struct WeatherQuery {
            location: String,
            unit: String,
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct WeatherResult {
            location: String,
            temperature: f64,
            unit: String,
        }

        impl FunctionCall for WeatherQuery {
            type Output = WeatherResult;

            fn name() -> &'static str {
                "get_current_weather"
            }

            fn description() -> &'static str {
                "Fetches the current weather for a given location."
            }

            fn execute(&self) -> FunctionCallTask<WeatherResult> {
                Box::pin(async move {
                    let location = self.location.clone();
                    let unit = self.unit.clone();

                    let result = WeatherResult {
                        location,
                        temperature: 21.0,
                        unit,
                    };
                    Ok(result)
                })
            }
        }

        let tools = vec![WeatherQuery::definition()];
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

        let result = query.execute().await.unwrap();

        assert_eq!(
            result,
            WeatherResult {
                location: "San Francisco".to_string(),
                temperature: 21.0,
                unit: "Celsius".to_string()
            }
        );
    }
}
