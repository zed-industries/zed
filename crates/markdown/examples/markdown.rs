use assets::Assets;
use gpui::{prelude::*, rgb, App, KeyBinding, StyleRefinement, Task, View, WindowOptions};
use language::{language_settings::AllLanguageSettings, LanguageRegistry};
use markdown::{Markdown, MarkdownStyle};
use node_runtime::FakeNodeRuntime;
use settings::SettingsStore;
use std::sync::Arc;
use theme::LoadThemes;
use ui::prelude::*;
use ui::{div, WindowContext};

const MARKDOWN_EXAMPLE: &'static str = r#"
# Markdown Example Document

## Headings
Headings are created by adding one or more `#` symbols before your heading text. The number of `#` you use will determine the size of the heading.

```rust
gpui::window::ViewContext
impl<'a, V> ViewContext<'a, V>
pub fn on_blur(&mut self, handle: &FocusHandle, listener: impl FnMut(&mut V, &mut iewContext<V>) + 'static) -> Subscription
where
    // Bounds from impl:
    V: 'static,
```

## Emphasis
Emphasis can be added with italics or bold. *This text will be italic*. _This will also be italic_

## Lists

### Unordered Lists
Unordered lists use asterisks `*`, plus `+`, or minus `-` as list markers.

* Item 1
* Item 2
  * Item 2a
  * Item 2b

### Ordered Lists
Ordered lists use numbers followed by a period.

1. Item 1
2. Item 2
3. Item 3
   1. Item 3a
   2. Item 3b

## Links
Links are created using the format [http://zed.dev](https://zed.dev).

They can also be detected automatically, for example https://zed.dev/blog.

## Images
Images are like links, but with an exclamation mark `!` in front.

```todo!
![This is an image](/images/logo.png)
```

## Code
Inline `code` can be wrapped with backticks `` ` ``.

```markdown
Inline `code` has `back-ticks around` it.
```

Code blocks can be created by indenting lines by four spaces or with triple backticks ```.

```javascript
function test() {
  console.log("notice the blank line before this function?");
}
```

## Blockquotes
Blockquotes are created with `>`.

> This is a blockquote.

## Horizontal Rules
Horizontal rules are created using three or more asterisks `***`, dashes `---`, or underscores `___`.

## Line breaks
This is a
\
line break!

---

Remember, markdown processors may have slight differences and extensions, so always refer to the specific documentation or guides relevant to your platform or editor for the best practices and additional features.
"#;

pub fn main() {
    env_logger::init();
    App::new().with_assets(Assets).run(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        language::init(cx);
        SettingsStore::update(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });
        cx.bind_keys([KeyBinding::new("cmd-c", markdown::Copy, None)]);

        let node_runtime = FakeNodeRuntime::new();
        theme::init(LoadThemes::JustBase, cx);

        let language_registry =
            LanguageRegistry::new(Task::ready(()), cx.background_executor().clone());
        language_registry.set_theme(cx.theme().clone());
        let language_registry = Arc::new(language_registry);
        languages::init(language_registry.clone(), node_runtime, cx);
        Assets.load_fonts(cx).unwrap();

        cx.activate(true);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| {
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

                MarkdownExample::new(
                    MARKDOWN_EXAMPLE.to_string(),
                    markdown_style,
                    language_registry,
                    cx,
                )
            })
        })
        .unwrap();
    });
}

struct MarkdownExample {
    markdown: View<Markdown>,
}

impl MarkdownExample {
    pub fn new(
        text: String,
        style: MarkdownStyle,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut WindowContext,
    ) -> Self {
        let markdown =
            cx.new_view(|cx| Markdown::new(text, style, Some(language_registry), cx, None));
        Self { markdown }
    }
}

impl Render for MarkdownExample {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("markdown-example")
            .debug_selector(|| "foo".into())
            .relative()
            .bg(gpui::white())
            .size_full()
            .p_4()
            .overflow_y_scroll()
            .child(self.markdown.clone())
    }
}
