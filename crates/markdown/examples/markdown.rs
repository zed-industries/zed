use assets::Assets;
use gpui::{Application, Entity, KeyBinding, StyleRefinement, WindowOptions, prelude::*, rgb};
use language::{LanguageRegistry, language_settings::AllLanguageSettings};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use node_runtime::NodeRuntime;
use settings::SettingsStore;
use std::sync::Arc;
use theme::LoadThemes;
use ui::prelude::*;
use ui::{App, Window, div};

const MARKDOWN_EXAMPLE: &str = r#"
# Markdown Example Document

## Headings
Headings are created by adding one or more `#` symbols before your heading text. The number of `#` you use will determine the size of the heading.

```
function a(b: T) {

}
```


Remember, markdown processors may have slight differences and extensions, so always refer to the specific documentation or guides relevant to your platform or editor for the best practices and additional features.
"#;

pub fn main() {
    env_logger::init();
    Application::new().with_assets(Assets).run(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        language::init(cx);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });
        cx.bind_keys([KeyBinding::new("cmd-c", markdown::Copy, None)]);

        let node_runtime = NodeRuntime::unavailable();
        theme::init(LoadThemes::JustBase, cx);

        let language_registry = LanguageRegistry::new(cx.background_executor().clone());
        language_registry.set_theme(cx.theme().clone());
        let language_registry = Arc::new(language_registry);
        languages::init(language_registry.clone(), node_runtime, cx);
        Assets.load_fonts(cx).unwrap();

        cx.activate(true);
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| MarkdownExample::new(MARKDOWN_EXAMPLE.into(), language_registry, cx))
        })
        .unwrap();
    });
}

struct MarkdownExample {
    markdown: Entity<Markdown>,
}

impl MarkdownExample {
    pub fn new(text: SharedString, language_registry: Arc<LanguageRegistry>, cx: &mut App) -> Self {
        let markdown = cx.new(|cx| {
            Markdown::new(
                text,
                Some(language_registry),
                Some("TypeScript".to_string()),
                cx,
            )
        });
        Self { markdown }
    }
}

impl Render for MarkdownExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let markdown_style = MarkdownStyle {
            base_text_style: gpui::TextStyle {
                font_family: "Zed Plex Sans".into(),
                color: cx.theme().colors().terminal_ansi_black,
                ..Default::default()
            },
            code_block: StyleRefinement::default()
                .font_family("Zed Plex Mono")
                .m(rems(1.))
                .bg(rgb(0xAAAAAAA)),
            inline_code: gpui::TextStyleRefinement {
                font_family: Some("Zed Mono".into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(cx.theme().colors().editor_background),
                ..Default::default()
            },
            rule_color: Color::Muted.color(cx),
            block_quote_border_color: Color::Muted.color(cx),
            block_quote: gpui::TextStyleRefinement {
                color: Some(Color::Muted.color(cx)),
                ..Default::default()
            },
            link: gpui::TextStyleRefinement {
                color: Some(Color::Accent.color(cx)),
                underline: Some(gpui::UnderlineStyle {
                    thickness: px(1.),
                    color: Some(Color::Accent.color(cx)),
                    wavy: false,
                }),
                ..Default::default()
            },
            syntax: cx.theme().syntax().clone(),
            selection_background_color: {
                let mut selection = cx.theme().players().local().selection;
                selection.fade_out(0.7);
                selection
            },
            ..Default::default()
        };

        div()
            .id("markdown-example")
            .debug_selector(|| "foo".into())
            .relative()
            .bg(gpui::white())
            .size_full()
            .p_4()
            .overflow_y_scroll()
            .child(MarkdownElement::new(self.markdown.clone(), markdown_style))
    }
}
