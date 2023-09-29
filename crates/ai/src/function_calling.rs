use crate::{OpenAIUsage, RequestMessage, Role};
use anyhow::anyhow;
use erased_serde::serialize_trait_object;
use futures::AsyncReadExt;
use isahc::{http::StatusCode, Request, RequestExt};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

const OPENAI_API_URL: &'static str = "https://api.openai.com/v1";

pub trait OpenAIFunction: erased_serde::Serialize {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn system_prompt(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
    fn complete(&self, arguments: serde_json::Value) -> anyhow::Result<String>;
}
serialize_trait_object!(OpenAIFunction);

#[derive(Serialize)]
struct OpenAIFunctionCallingRequest {
    model: String,
    messages: Vec<RequestMessage>,
    functions: Vec<Box<dyn OpenAIFunction>>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

impl FunctionCall {
    fn arguments(&self) -> anyhow::Result<serde_json::Value> {
        serde_json::from_str(&self.arguments).map_err(|err| anyhow!(err))
    }
}

#[derive(Debug, Deserialize)]
pub struct FunctionCallingMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
    pub function_call: FunctionCall,
}

#[derive(Debug, Deserialize)]
pub struct FunctionCallingChoice {
    pub message: FunctionCallingMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIFunctionCallingResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<FunctionCallingChoice>,
    pub usage: OpenAIUsage,
}

impl OpenAIFunctionCallingResponse {
    fn get_details(&self) -> anyhow::Result<FunctionCallDetails> {
        if let Some(choice) = self.choices.first() {
            let name = choice.message.function_call.name.clone();
            let arguments = choice.message.function_call.arguments()?;

            Ok(FunctionCallDetails {
                name,
                arguments,
                message: choice.message.content.clone(),
            })
        } else {
            Err(anyhow!("no function call details available"))
        }
    }
}

#[derive(Debug)]
pub struct FunctionCallDetails {
    pub name: String,                 // name of function to call
    pub message: Option<String>,      // message if provided
    pub arguments: serde_json::Value, // json object respresenting provided arguments
}

#[derive(Clone)]
pub struct OpenAIFunctionCallingProvider {
    api_key: String,
}

impl OpenAIFunctionCallingProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    fn generate_system_message(
        &self,
        messages: &Vec<RequestMessage>,
        functions: &Vec<Box<dyn OpenAIFunction>>,
    ) -> RequestMessage {
        let mut system_message = messages
            .iter()
            .filter_map(|message| {
                if message.role == Role::System {
                    Some(message.content.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        let function_string = functions
            .iter()
            .map(|function| format!("'{}'", function.name()))
            .collect::<Vec<String>>()
            .join(",");

        writeln!(
            system_message,
            "You have access to the following functions: {function_string} you MUST return a function calling response using at one of the below functions."
        )
        .unwrap();

        for function in functions {
            writeln!(system_message, "\n{}", function.system_prompt()).unwrap();
        }

        RequestMessage {
            role: Role::System,
            content: system_message,
        }
    }

    pub async fn complete(
        &self,
        model: String,
        mut messages: Vec<RequestMessage>,
        functions: Vec<Box<dyn OpenAIFunction>>,
    ) -> anyhow::Result<FunctionCallDetails> {
        // TODO: Rename all this.
        let mut system_message = vec![self.generate_system_message(&messages, &functions)];
        messages.retain(|message| message.role != Role::System);

        system_message.extend(messages);
        // Lower temperature values, result in less randomness,
        // this is helping keep the function calling consistent
        let request = OpenAIFunctionCallingRequest {
            model,
            messages: system_message,
            functions,
            temperature: 0.0,
        };

        let json_data = serde_json::to_string(&request)?;
        println!("\nREQUEST: {:?}\n", &json_data);
        let mut response = Request::post(format!("{OPENAI_API_URL}/chat/completions"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(json_data)?
            .send_async()
            .await?;

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        println!("\nRESPONSE: {:?}\n", &body);
        match response.status() {
            StatusCode::OK => {
                let response_data: OpenAIFunctionCallingResponse = serde_json::from_str(&body)?;
                response_data.get_details()
            }
            _ => Err(anyhow!("open ai function calling failed: {:?}", body)),
        }
    }
}
