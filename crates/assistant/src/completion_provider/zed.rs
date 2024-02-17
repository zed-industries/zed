use anyhow::Result;
use client::{proto, Client};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt, TryFutureExt};
use gpui::{AppContext, Task};
use std::sync::Arc;

use crate::{
    assistant_settings::ZedDotDevModel, CompletionProvider, LanguageModelRequest,
    LanguageModelRequestMessage,
};

pub struct ZedDotDevCompletionProvider {
    client: Arc<Client>,
    default_model: ZedDotDevModel,
    status: client::Status,
    _maintain_client_status: Task<()>,
}

impl ZedDotDevCompletionProvider {
    pub fn new(default_model: ZedDotDevModel, client: Arc<Client>, cx: &mut AppContext) -> Self {
        let mut status_rx = client.status();
        let status = status_rx.borrow().clone();
        let maintain_client_status = cx.spawn(|mut cx| async move {
            while let Some(status) = status_rx.next().await {
                let _ = cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    if let CompletionProvider::ZedDotDev(provider) = provider {
                        provider.status = status;
                    } else {
                        unreachable!()
                    }
                });
            }
        });
        Self {
            client,
            default_model,
            status,
            _maintain_client_status: maintain_client_status,
        }
    }

    pub fn update(&mut self, default_model: ZedDotDevModel) {
        self.default_model = default_model;
    }

    pub fn default_model(&self) -> ZedDotDevModel {
        self.default_model.clone()
    }

    pub fn is_authenticated(&self) -> bool {
        self.status.is_connected()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        let client = self.client.clone();
        cx.spawn(move |cx| async move { client.authenticate_and_connect(true, &cx).await })
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        self.client
            .request_stream(proto::CompleteWithLanguageModel {
                model: request.model.map(|model| model.id().to_string()),
                messages: request.messages.map(|message| message.to_proto()),
                stop: request.stop,
                temperature: todo!(),
            })
            .map_ok(|stream| stream.boxed())
    }
}
