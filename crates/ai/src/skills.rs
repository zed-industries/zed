use crate::function_calling::OpenAIFunction;
use gpui::{AppContext, ModelHandle};
use project::Project;
use serde::{Serialize, Serializer};
use serde_json::json;

pub struct RewritePrompt;
impl Serialize for RewritePrompt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        json!({"name": self.name(),
            "description": self.description(),
            "parameters": self.parameters()})
        .serialize(serializer)
    }
}

impl RewritePrompt {
    pub fn load() -> Self {
        Self {}
    }
}

impl OpenAIFunction for RewritePrompt {
    fn name(&self) -> String {
        "rewrite_prompt".to_string()
    }
    fn description(&self) -> String {
        "Rewrite prompt given prompt from user".to_string()
    }
    fn system_prompt(&self) -> String {
        "'rewrite_prompt':
        If all information is available in the above prompt, and you need no further information.
        Rewrite the entire prompt to clarify what should be generated, do not actually complete the users request.
        Assume this rewritten message will be passed to another completion agent, to fulfill the users request.".to_string()
    }
    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "prompt": {}
            }
        })
    }
    fn complete(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        Ok(arguments.get("prompt").unwrap().to_string())
    }
}
