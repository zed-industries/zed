use gpui::{prelude::*, App, ScrollHandle, Task, View, WindowOptions};
use language::{language_settings::AllLanguageSettings, LanguageRegistry};
use markdown::Markdown;
use node_runtime::FakeNodeRuntime;
use settings::SettingsStore;
use std::sync::Arc;
use ui::prelude::*;
use ui::{div, WindowContext};

const MARKDOWN_EXAMPLE: &'static str = r#"
# Markdown Example Document

## Headings
Headings are created by adding one or more `#` symbols before your heading text. The number of `#` you use will determine the size of the heading.

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
Links are created using the format [zed.dev](https://zed.dev).

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

---

## Tables
Tables are created using pipes `|` and dashes `-`. Alignments are controlled by colons `:`.

| Left-aligned | Center-aligned | Right-aligned |
| :---         |      :----:    |          ---: |
| Text         |      Text      | Text          |

---

Remember, markdown processors may have slight differences and extensions, so always refer to the specific documentation or guides relevant to your platform or editor for the best practices and additional features.
"#;

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
            cx.new_view(|cx| {
                MarkdownExample::new(MARKDOWN_EXAMPLE.to_string(), language_registry, cx)
            })
        });
    });
}

struct MarkdownExample {
    markdown: View<Markdown>,
}

impl MarkdownExample {
    pub fn new(
        text: String,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut WindowContext,
    ) -> Self {
        let markdown = cx.new_view(|cx| Markdown::new(text, language_registry, cx));
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
            .overflow_y_scroll()
            .child(self.markdown.clone())
    }
}
