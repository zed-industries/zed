use anyhow::Result;
use client::Client;
use gpui::{AppContext, Task};
use std::sync::Arc;

pub struct ZedDotDevCompletionProvider {
    client: Arc<Client>,
}

impl ZedDotDevCompletionProvider {
    pub fn is_authenticated(&self) -> bool {
        self.client.status().borrow().is_connected()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |cx| async move { client.authenticate_and_connect(true, &cx).await })
    }
}
