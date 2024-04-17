use futures::Future;
use schemars::{schema::RootSchema, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::pin::Pin;

pub trait FunctionCall: Serialize + for<'de> Deserialize<'de> + JsonSchema {
    type Output: Serialize;

    fn name() -> &'static str;
    fn description() -> &'static str;

    fn parameters() -> &'static serde_json::Value;
    fn schema() -> &'static RootSchema;

    fn definition() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": Self::name(),
                "description": Self::description(),
                "schema": Self::parameters()
            }
        })
    }

    fn extract(args: &Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(args.clone())
    }

    fn execute(
        args: Self,
    ) -> Pin<Box<dyn Future<Output = Result<String, serde_json::Error>> + Send>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use gpui::TestAppContext;

    use lazy_static::lazy_static;
    use schemars::schema_for;

    #[gpui::test]
    async fn test_openai_weather_example(cx: &mut TestAppContext) {
        cx.background_executor.run_until_parked();

        #[derive(Deserialize, Serialize, JsonSchema)]
        struct WeatherQuery {
            location: String,
            unit: String,
        }

        #[derive(Serialize)]
        struct WeatherResult {
            location: String,
            temperature: f64,
            unit: String,
        }

        lazy_static! {
            static ref SCHEMA: RootSchema = schema_for!(WeatherQuery).into();
            static ref PARAMETERS: serde_json::Value =
                serde_json::to_value(&*SCHEMA).expect("Schema serialization must not fail");
        }

        impl FunctionCall for WeatherQuery {
            type Output = WeatherResult;

            fn name() -> &'static str {
                "get_current_weather"
            }

            fn description() -> &'static str {
                "Fetches the current weather for a given location."
            }

            fn parameters() -> &'static serde_json::Value {
                &PARAMETERS
            }

            fn schema() -> &'static RootSchema {
                &SCHEMA
            }

            fn execute(
                args: Self,
            ) -> Pin<Box<dyn Future<Output = Result<String, serde_json::Error>> + Send>>
            {
                Box::pin(async move {
                    let result = WeatherResult {
                        location: args.location,
                        temperature: 21.0,
                        unit: args.unit,
                    };
                    Ok(serde_json::to_string(&result)?)
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

        // Now let's test having "the model" pass in args, get back a result, and then return the result
        let args = json!({
            "location": "San Francisco",
            "unit": "Celsius"
        });

        let input = WeatherQuery::extract(&args).unwrap();
        let result = WeatherQuery::execute(input).await.unwrap();
        println!("Result: {}", result);
    }
}
