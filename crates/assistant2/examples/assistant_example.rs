use std::sync::Arc;

use assets::Assets;
use assistant2::AssistantPanel;
use client::Client;
use gpui::{App, Task, View, WindowOptions};
use language::LanguageRegistry;
use settings::{KeymapFile, DEFAULT_KEYMAP_PATH};
use theme::LoadThemes;
use ui::{div, prelude::*, Render};

fn main() {
    env_logger::init();
    App::new().with_assets(Assets).run(|cx| {
        settings::init(cx);
        language::init(cx);
        editor::init(cx);
        theme::init(LoadThemes::JustBase, cx);
        Assets.load_fonts(cx).unwrap();
        KeymapFile::load_asset(DEFAULT_KEYMAP_PATH, cx).unwrap();
        client::init_settings(cx);
        release_channel::init("0.130.0", cx);

        let client = Client::production(cx);
        {
            let client = client.clone();
            cx.spawn(|cx| async move { client.authenticate_and_connect(false, &cx).await })
                .detach_and_log_err(cx);
        }
        assistant2::init(client.clone(), cx);

        let language_registry = Arc::new(LanguageRegistry::new(
            Task::ready(()),
            cx.background_executor().clone(),
        ));
        let node_runtime = node_runtime::RealNodeRuntime::new(client.http_client());
        languages::init(language_registry.clone(), node_runtime, cx);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| Example::new(language_registry, cx))
        });
        cx.activate(true);
    })
}

struct Example {
    assistant_panel: View<AssistantPanel>,
}

impl Example {
    fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            assistant_panel: cx.new_view(|cx| AssistantPanel::new(language_registry, cx)),
        }
    }
}

impl Render for Example {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl ui::prelude::IntoElement {
        div().size_full().child(self.assistant_panel.clone())
    }
}
