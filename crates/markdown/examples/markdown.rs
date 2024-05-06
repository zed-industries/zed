use gpui::{prelude::*, App, Task, WindowOptions};
use language::{language_settings::AllLanguageSettings, LanguageRegistry};
use markdown::Markdown;
use node_runtime::FakeNodeRuntime;
use settings::SettingsStore;
use std::sync::Arc;

const MARKDOWN_EXAMPLE: &'static str =
    "# Hey\nLorem ipsum dolor, `sit` amet **consecteur**:\n\n```rust\nfn foo() {}\n```\n_Hey_, wassup?\n1. A\n    1. A1\n    2. A2\n2. B\n3. C";

pub fn main() {
    App::new().run(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        language::init(cx);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });

        let node_runtime = FakeNodeRuntime::new();
        let language_registry = Arc::new(LanguageRegistry::new(
            Task::ready(()),
            cx.background_executor().clone(),
        ));
        languages::init(language_registry.clone(), node_runtime, cx);

        cx.activate(true);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| Markdown::new(MARKDOWN_EXAMPLE.to_string(), language_registry, cx))
        });
    });
}
