use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};

use crate::providers::open_ai::completion::OpenAIRequest;

pub trait CompletionProvider {
    fn complete(
        &self,
        prompt: OpenAIRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}
