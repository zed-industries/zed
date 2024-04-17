use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

use serde_json::{json, Value};

pub trait FunctionCall {
    type Input: Serialize + for<'de> Deserialize<'de> + JsonSchema;

    type Output: Serialize;

    fn name() -> &'static str;
    fn description() -> &'static str;

    fn schema() -> RootSchema;

    fn extract(args: &Value) -> Result<Self::Input, serde_json::Error> {
        serde_json::from_value(args.clone())
    }

    fn execute(args: Self::Input) -> Result<String, serde_json::Error>;

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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_weather_example() {
        #[derive(Serialize, Deserialize, JsonSchema)]
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

        struct GetWeather;

        impl FunctionCall for GetWeather {
            type Input = WeatherQuery;
            type Output = WeatherResult;

            fn name() -> &'static str {
                "get_current_weather"
            }

            fn description() -> &'static str {
                "Fetches the current weather for a given location."
            }

            // Could/should this be created at compile time?
            fn schema() -> RootSchema {
                schema_for!(WeatherQuery).into()
            }

            fn execute(args: WeatherQuery) -> Result<String, serde_json::Error> {
                let result = WeatherResult {
                    location: args.location,
                    temperature: 21.0,
                    unit: args.unit,
                };
                serde_json::to_string(&result)
            }
        }
    }

    #[test]
    fn test_function_tool() {
        #[derive(Deserialize, Serialize, JsonSchema)]
        struct CodebaseQuery {
            query: String,
        }
    }
}
