use anyhow::Result;
use futures::{channel::mpsc, future::BoxFuture};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub trait LanguageModel: Send + Sync {
    fn stream_completion(
        &self,
        messages: Vec<Message>,
    ) -> BoxFuture<Result<mpsc::Receiver<String>>>;
}
