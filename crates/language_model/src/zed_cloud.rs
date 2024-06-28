use client::Client;
use gpui::ModelContext;
use std::sync::Arc;

use crate::{
    LanguageModel, LanguageModelId, LanguageModelProvider, LanguageModelProviderName,
    ProvidedLanguageModel,
};

pub struct ZedCloudModelProvider {
    client: Arc<Client>,
    available_models: Vec<ProvidedLanguageModel>,
}

impl ZedCloudModelProvider {
    pub fn new(client: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let authenticated = client.user_id().is_some();
        if authenticated {
            client.request(client::proto::GetAvail {});

            let models = cx.spawn(|this, mut cx| async move {
                let response = this.update(&mut cx, |this, _| {})?;
                response.await
            });

            match models.await {
                Ok(models) => {
                    self.available_models = models
                        .models
                        .into_iter()
                        .map(|model| ProvidedLanguageModel {
                            id: LanguageModelId(model.id),
                            name: model.name,
                        })
                        .collect();
                }
                Err(err) => {
                    log::error!("Failed to fetch language models: {}", err);
                }
            }
        }

        Self {
            client,
            available_models: Vec::new(),
        }
    }
}

impl LanguageModelProvider for ZedCloudModelProvider {
    fn name(&self, _cx: &gpui::AppContext) -> crate::LanguageModelProviderName {
        LanguageModelProviderName("Zed Cloud".into())
    }

    fn provided_models(&self, _cx: &gpui::AppContext) -> Vec<ProvidedLanguageModel> {
        self.available_models.clone()
    }

    fn model(
        &self,
        id: LanguageModelId,
        _cx: &gpui::AppContext,
    ) -> gpui::Result<Arc<dyn LanguageModel>> {
        todo!()
    }
}

struct ZedCloudModel {
    id: LanguageModelId,
    client: Arc<Client>,
}

impl LanguageModel for ZedCloudModel {
    fn is_authenticated(&self, cx: &mut gpui::AppContext) -> bool {
        todo!()
    }

    fn authenticate(&self, cx: &mut gpui::AppContext) -> gpui::Task<gpui::Result<()>> {
        todo!()
    }

    fn authentication_prompt(&self, cx: &mut gpui::WindowContext) -> gpui::AnyView {
        todo!()
    }

    fn reset_credentials(&self, cx: &mut gpui::AppContext) -> gpui::Task<gpui::Result<()>> {
        todo!()
    }

    fn count_tokens(
        &self,
        request: crate::LanguageModelRequest,
        cx: &mut gpui::AppContext,
    ) -> futures::future::BoxFuture<'static, gpui::Result<usize>> {
        todo!()
    }

    fn complete(
        &self,
        request: crate::LanguageModelRequest,
        cx: &mut gpui::AppContext,
    ) -> futures::future::BoxFuture<
        'static,
        gpui::Result<futures::stream::BoxStream<'static, gpui::Result<crate::LanguageModelOutput>>>,
    > {
        todo!()
    }
}
