use crate::{ItemViewHandle, Settings, StatusItemView};
use futures::StreamExt;
use gpui::{elements::*, Entity, RenderContext, View, ViewContext};
use language::LanguageRegistry;
use postage::watch;
use std::sync::Arc;

pub struct LspStatus {
    pending_lsp_binaries: usize,
    settings_rx: watch::Receiver<Settings>,
}

impl LspStatus {
    pub fn new(
        languages: Arc<LanguageRegistry>,
        settings_rx: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut pending_lsp_binaries = languages.pending_lsp_binaries();
        cx.spawn_weak(|this, mut cx| async move {
            while let Some(pending_lsp_binaries) = pending_lsp_binaries.next().await {
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        this.pending_lsp_binaries = pending_lsp_binaries;
                        cx.notify();
                    });
                } else {
                    break;
                }
            }
        })
        .detach();
        Self {
            pending_lsp_binaries: 0,
            settings_rx,
        }
    }
}

impl Entity for LspStatus {
    type Event = ();
}

impl View for LspStatus {
    fn ui_name() -> &'static str {
        "LspStatus"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        if self.pending_lsp_binaries == 0 {
            Empty::new().boxed()
        } else {
            let theme = &self.settings_rx.borrow().theme;
            Label::new(
                "Downloading language servers...".to_string(),
                theme.workspace.status_bar.lsp_message.clone(),
            )
            .boxed()
        }
    }
}

impl StatusItemView for LspStatus {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemViewHandle>, _: &mut ViewContext<Self>) {}
}
