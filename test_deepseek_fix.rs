use serde_json::json;
use std::collections::HashMap;

#[derive(Debug)]
enum ResponseStreamResult {
    Ok(ResponseStreamEvent),
    Err { error: ResponseStreamError },
}

#[derive(Debug)]
struct ResponseStreamEvent {
    choices: Vec<ChoiceDelta>,
    usage: Option<Usage>,
    additional_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
struct ResponseStreamError {
    message: String,
}

#[derive(Debug)]
struct ChoiceDelta {
    index: u32,
    delta: Option<ResponseMessageDelta>,
    finish_reason: Option<String>,
}

#[derive(Debug)]
struct ResponseMessageDelta {
    role: Option<String>,
    content: Option<String>,
    tool_calls: Option<Vec<ToolCallChunk>>,
}

#[derive(Debug)]
struct ToolCallChunk {
    index: usize,
    id: Option<String>,
    function: Option<FunctionChunk>,
}

#[derive(Debug)]
struct FunctionChunk {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Debug)]
struct Usage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
}

fn parse_response_stream_result(line: &str) -> Result<ResponseStreamResult, serde_json::Error> {
    // First try the standard parsing
    match serde_json::from_str(line) {
        Ok(result) => return Ok(result),
        Err(_) => {
            // If standard parsing fails, try a more lenient approach
            let value: serde_json::Value = serde_json::from_str(line)?;
            
            // Check if this looks like a success response (has choices)
            if let Some(choices) = value.get("choices") {
                // Try to build a ResponseStreamEvent manually
                let choices = serde_json::from_value::<Vec<ChoiceDelta>>(choices.clone())?;
                let usage = value.get("usage").and_then(|u| serde_json::from_value::<Usage>(u.clone()).ok());
                
                let mut additional_fields = HashMap::new();
                if let Some(obj) = value.as_object() {
                    for (key, val) in obj {
                        if key != "choices" && key != "usage" {
                            additional_fields.insert(key.clone(), val.clone());
                        }
                    }
                }
                
                let event = ResponseStreamEvent {
                    choices,
                    usage,
                    additional_fields,
                };
                
                return Ok(ResponseStreamResult::Ok(event));
            }
            
            // Check if this looks like an error response (has error)
            if let Some(error_obj) = value.get("error") {
                let error = serde_json::from_value::<ResponseStreamError>(error_obj.clone())?;
                return Ok(ResponseStreamResult::Err { error });
            }
            
            // If we can't determine the structure, fall back to standard parsing
            // This will likely fail, but at least we tried
            serde_json::from_str(line)
        }
    }
}

fn main() {
    // Test a DeepSeek-style response with extend_fields
    let deepseek_response = json!({
        "id": "21705079-0372-4995-8176-8556a50d7951",
        "object": "chat.completion.chunk",
        "created": 1765468032,
        "model": "deepseek-v3.2",
        "usage": {
            "prompt_tokens": 10772,
            "completion_tokens": 1,
            "total_tokens": 10773
        },
        "extend_fields": {
            "traceId": "21010f9017654680283934182e2513",
            "requestId": "8e2021a4be88b53de0fbbe3655fbad29"
        },
        "choices": [{
            "index": 0,
            "delta": {
                "role": "assistant",
                "content": "**",
                "tool_calls": []
            }
        }]
    });

    let response_str = deepseek_response.to_string();
    let result = parse_response_stream_result(&response_str);
    
    match result {
        Ok(ResponseStreamResult::Ok(event)) => {
            println!("✓ Successfully parsed DeepSeek response");
            println!("  - Choices: {}", event.choices.len());
            println!("  - Usage: {:?}", event.usage);
            println!("  - Additional fields: {}", event.additional_fields.len());
            println!("  - Has extend_fields: {}", event.additional_fields.contains_key("extend_fields"));
        }
        Ok(ResponseStreamResult::Err { error }) => {
            eprintln!("✗ Expected success response, got error: {}", error.message);
        }
        Err(e) => {
            eprintln!("✗ Failed to parse DeepSeek response: {}", e);
        }
    }
}
