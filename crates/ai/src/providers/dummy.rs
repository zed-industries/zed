use crate::completion::CompletionRequest;
use serde::Serialize;

#[derive(Serialize)]
pub struct DummyCompletionRequest {
    pub name: String,
}

impl CompletionRequest for DummyCompletionRequest {
    fn data(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
