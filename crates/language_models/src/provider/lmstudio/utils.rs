use anyhow::Result;
use language_model::{
    LanguageModelCompletionEvent,
    LanguageModelToolUse, LanguageModelToolUseId, StopReason,
};
use uuid;

#[derive(Clone)]
pub struct LmStudioStreamMapper {
    in_thinking_block: bool,
    thinking_buffer: String,
    pending_text: Option<String>,
    // Tool call accumulation state
    accumulating_tool_call: bool,
    tool_call_id: Option<String>,
    tool_call_name: Option<String>,
    tool_call_args_buffer: String,
}

impl LmStudioStreamMapper {
    pub fn new() -> Self {
        Self {
            in_thinking_block: false,
            thinking_buffer: String::new(),
            pending_text: None,
            accumulating_tool_call: false,
            tool_call_id: None,
            tool_call_name: None,
            tool_call_args_buffer: String::new(),
        }
    }

    pub fn process_fragment(&mut self, fragment: lmstudio::ChatResponse) -> Result<Option<LanguageModelCompletionEvent>> {
        // Most of the time, there will be only one choice
        let Some(choice) = fragment.choices.first() else {
            return Ok(None);
        };

        // Check for finish reason first
        if let Some(reason) = choice.finish_reason.as_deref() {
            let stop_reason = match reason {
                "length" => StopReason::MaxTokens,
                "tool_calls" => StopReason::ToolUse,
                _ => StopReason::EndTurn,
            };
            
            // If we were accumulating a tool call, emit it before stopping
            if self.accumulating_tool_call && self.tool_call_name.is_some() {
                let tool_use = self.create_tool_use_from_buffer();
                self.reset_tool_call_state();
                return Ok(Some(LanguageModelCompletionEvent::ToolUse(tool_use)));
            }
            
            // Reset state
            self.reset_state();
            return Ok(Some(LanguageModelCompletionEvent::Stop(stop_reason)));
        }

        // Extract the delta content
        if let Ok(delta) = serde_json::from_value::<lmstudio::ResponseMessageDelta>(choice.delta.clone()) {
            // Handle tool calls
            if let Some(tool_calls) = delta.tool_calls {
                for tool_call in tool_calls {
                    if let Some(function) = tool_call.function.clone() {
                        // Process tool call
                        if let Some(event) = self.process_tool_call(tool_call, function)? {
                            return Ok(Some(event));
                        }
                    }
                }
            }

            // Handle text content
            if let Some(content) = delta.content {
                if !content.is_empty() && !self.accumulating_tool_call {
                    return self.process_text_content(&content);
                }
            }
        }

        // Check for any pending text
        if let Some(text) = self.pending_text.take() {
            return Ok(Some(LanguageModelCompletionEvent::Text(text)));
        }

        Ok(None)
    }

    fn process_tool_call(
        &mut self,
        tool_call: lmstudio::ToolCallChunk,
        function: lmstudio::FunctionChunk,
    ) -> Result<Option<LanguageModelCompletionEvent>> {
        // Get or update the tool call ID
        if let Some(id) = &tool_call.id {
            if self.tool_call_id.is_none() {
                log::debug!("LMStudio: Starting tool call accumulation with ID: {}", id);
                self.tool_call_id = Some(id.clone());
                self.accumulating_tool_call = true;
            }
        }
        
        // Get or update the function name
        if let Some(name) = &function.name {
            if self.tool_call_name.is_none() && !name.trim().is_empty() {
                log::debug!("LMStudio: Tool call name: {}", name);
                self.tool_call_name = Some(name.clone());
            }
        }
        
        // Accumulate arguments
        if let Some(args) = function.arguments.as_ref() {
            log::debug!("LMStudio: Received argument fragment: {}", args);
            self.tool_call_args_buffer.push_str(&args);
            
            // Check if the accumulated arguments form valid JSON
            if self.is_likely_complete_json(&self.tool_call_args_buffer) {
                log::debug!("LMStudio: Detected complete JSON arguments, emitting tool use");
                let tool_use = self.create_tool_use_from_buffer();
                self.reset_tool_call_state();
                return Ok(Some(LanguageModelCompletionEvent::ToolUse(tool_use)));
            }
        }
        
        Ok(None)
    }

    fn process_text_content(&mut self, content: &str) -> Result<Option<LanguageModelCompletionEvent>> {
        if self.in_thinking_block {
            // Already in a thinking block
            if content.contains("</think>") {
                let parts: Vec<&str> = content.split("</think>").collect();
                let before_closing = parts[0];
                let thinking_text = before_closing.to_string();
                self.in_thinking_block = false;
                
                if parts.len() > 1 && !parts[1].is_empty() {
                    self.pending_text = Some(parts[1].to_string());
                }
                
                return Ok(Some(LanguageModelCompletionEvent::Thinking {
                    text: thinking_text,
                    signature: None,
                }));
            } else {
                return Ok(Some(LanguageModelCompletionEvent::Thinking {
                    text: content.to_string(),
                    signature: None,
                }));
            }
        } else if content.contains("<think>") {
            self.in_thinking_block = true;
            let parts: Vec<&str> = content.split("<think>").collect();
            let before_tag = parts[0];
            
            if !before_tag.is_empty() {
                self.pending_text = Some(before_tag.to_string());
                return Ok(Some(LanguageModelCompletionEvent::Text(before_tag.to_string())));
            }
            
            if parts.len() > 1 {
                let after_tag = parts[1];
                if after_tag.contains("</think>") {
                    let thinking_parts: Vec<&str> = after_tag.split("</think>").collect();
                    let thinking_text = thinking_parts[0].trim();
                    self.in_thinking_block = false;
                    
                    if thinking_parts.len() > 1 && !thinking_parts[1].is_empty() {
                        self.pending_text = Some(thinking_parts[1].to_string());
                    }
                    
                    return Ok(Some(LanguageModelCompletionEvent::Thinking {
                        text: thinking_text.to_string(),
                        signature: None,
                    }));
                } else if !after_tag.is_empty() {
                    return Ok(Some(LanguageModelCompletionEvent::Thinking {
                        text: after_tag.to_string(),
                        signature: None,
                    }));
                }
            }
            return Ok(None);
        }
        
        Ok(Some(LanguageModelCompletionEvent::Text(content.to_string())))
    }
    
    fn is_likely_complete_json(&self, json: &str) -> bool {
        if serde_json::from_str::<serde_json::Value>(json).is_ok() {
            return true;
        }
        
        let mut depth = 0;
        let mut inside_string = false;
        let mut was_escape = false;
        
        for c in json.chars() {
            match c {
                '"' if !was_escape => inside_string = !inside_string,
                '\\' if inside_string => was_escape = !was_escape,
                '{' if !inside_string => depth += 1,
                '}' if !inside_string => depth -= 1,
                _ => was_escape = false,
            }
        }
        
        depth == 0 && json.trim().starts_with('{') && json.trim().ends_with('}')
    }
    
    fn create_tool_use_from_buffer(&self) -> LanguageModelToolUse {
        let id = self.tool_call_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let name = self.tool_call_name
            .clone()
            .unwrap_or_else(|| "unknown_function".to_string())
            .trim()
            .to_string();
            
        let name = if name.is_empty() { 
            log::warn!("LMStudio: Empty function name detected when creating tool use, using fallback name");
            "unknown_function".to_string() 
        } else { 
            name 
        };
            
        let args = self.tool_call_args_buffer.clone();
        
        log::debug!("LMStudio: Creating tool use - Name: {}, Args: {}", name, args);
        
        LanguageModelToolUse {
            id: LanguageModelToolUseId::from(id),
            name: name.into(),
            raw_input: args.clone(),
            input: serde_json::from_str(&args).unwrap_or(serde_json::json!({})),
            is_input_complete: true,
        }
    }
    
    fn reset_tool_call_state(&mut self) {
        self.accumulating_tool_call = false;
        self.tool_call_id = None;
        self.tool_call_name = None;
        self.tool_call_args_buffer.clear();
        log::debug!("LMStudio: Reset tool call accumulation state");
    }
    
    fn reset_state(&mut self) {
        self.in_thinking_block = false;
        self.thinking_buffer.clear();
        self.pending_text = None;
        self.reset_tool_call_state();
    }
} 