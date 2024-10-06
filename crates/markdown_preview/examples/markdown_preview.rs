
use anyhow::Result;
use gpui::{div, prelude::*, Model, Task, ViewContext};

use markdown_preview::{
    markdown_elements::ParsedMarkdown, markdown_parser::parse_markdown,
    markdown_renderer::render_markdown_block,
};

use gpui::App;
use gpui::WindowOptions;
use settings::SettingsStore;
use theme::LoadThemes;



const MARKDOWN_EXAMPLE: &str = r#"
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

## Tables
|  table1   | table2  |
|  ----  | ----  |
| item11  | item12 |
| item21  | item22 |


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
    // env_logger::init();
    App::new().run(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);

        theme::init(LoadThemes::JustBase, cx);

        cx.activate(true);
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| {
                MarkdownView::from(MARKDOWN_EXAMPLE.into(), cx)
            })
        })
        .unwrap();
    });
}



pub struct MarkdownView {
    raw_text: String,
    contents: Option<ParsedMarkdown>,
    parsing_markdown_task: Option<Task<Result<()>>>,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut ViewContext<Self>) -> Self {
        let task = cx.spawn(|markdown_view, mut cx| {
            let text = text.clone();
            let parsed = cx
                .background_executor()
                .spawn(async move { parse_markdown(&text, None, None).await });

            async move {
                let content = parsed.await;

                markdown_view.update(&mut cx, |markdown, cx| {
                    markdown.parsing_markdown_task.take();
                    markdown.contents = Some(content);
                    cx.notify();
                })
            }
        });

        Self {
            raw_text: text.clone(),
            contents: None,
            parsing_markdown_task: Some(task),
        }
    }
}


impl Render for MarkdownView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(parsed) = self.contents.as_ref() else {
            return div().into_any_element();
        };

        let mut markdown_render_context =
            markdown_preview::markdown_renderer::RenderContext::new(None, cx);

        div()
            .id("markdown-preview-example")
            .debug_selector(|| "foo".into())
            .relative()
            .bg(gpui::white())
            .size_full()
            .p_4()
            .overflow_y_scroll()
            .children(
                parsed.children.iter().map(|child| {
                    div().relative().child(
                        div()
                            .relative()
                            .child(render_markdown_block(child, &mut markdown_render_context)),
                    )
                })
            )
            .into_any_element()
    }
}