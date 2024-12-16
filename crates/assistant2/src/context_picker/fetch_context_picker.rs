use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{bail, Context as _, Result};
use futures::AsyncReadExt as _;
use gpui::{AppContext, DismissEvent, FocusHandle, FocusableView, Task, View, WeakModel, WeakView};
use html_to_markdown::{convert_html_to_markdown, markdown, TagHandler};
use http_client::{AsyncBody, HttpClientWithUrl};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem, ViewContext};
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;

pub struct FetchContextPicker {
    picker: View<Picker<FetchContextPickerDelegate>>,
}

impl FetchContextPicker {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = FetchContextPickerDelegate::new(context_picker, workspace, context_store);
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));

        Self { picker }
    }
}

impl FocusableView for FetchContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for FetchContextPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContentType {
    Html,
    Plaintext,
    Json,
}

pub struct FetchContextPickerDelegate {
    context_picker: WeakView<ContextPicker>,
    workspace: WeakView<Workspace>,
    context_store: WeakModel<ContextStore>,
    url: String,
}

impl FetchContextPickerDelegate {
    pub fn new(
        context_picker: WeakView<ContextPicker>,
        workspace: WeakView<Workspace>,
        context_store: WeakModel<ContextStore>,
    ) -> Self {
        FetchContextPickerDelegate {
            context_picker,
            workspace,
            context_store,
            url: String::new(),
        }
    }

    async fn build_message(http_client: Arc<HttpClientWithUrl>, url: &str) -> Result<String> {
        let mut url = url.to_owned();
        if !url.starts_with("https://") && !url.starts_with("http://") {
            url = format!("https://{url}");
        }

        let mut response = http_client.get(&url, AsyncBody::default(), true).await?;

        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("error reading response body")?;

        if response.status().is_client_error() {
            let text = String::from_utf8_lossy(body.as_slice());
            bail!(
                "status error {}, response: {text:?}",
                response.status().as_u16()
            );
        }

        let Some(content_type) = response.headers().get("content-type") else {
            bail!("missing Content-Type header");
        };
        let content_type = content_type
            .to_str()
            .context("invalid Content-Type header")?;
        let content_type = match content_type {
            "text/html" => ContentType::Html,
            "text/plain" => ContentType::Plaintext,
            "application/json" => ContentType::Json,
            _ => ContentType::Html,
        };

        match content_type {
            ContentType::Html => {
                let mut handlers: Vec<TagHandler> = vec![
                    Rc::new(RefCell::new(markdown::WebpageChromeRemover)),
                    Rc::new(RefCell::new(markdown::ParagraphHandler)),
                    Rc::new(RefCell::new(markdown::HeadingHandler)),
                    Rc::new(RefCell::new(markdown::ListHandler)),
                    Rc::new(RefCell::new(markdown::TableHandler::new())),
                    Rc::new(RefCell::new(markdown::StyledTextHandler)),
                ];
                if url.contains("wikipedia.org") {
                    use html_to_markdown::structure::wikipedia;

                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaChromeRemover)));
                    handlers.push(Rc::new(RefCell::new(wikipedia::WikipediaInfoboxHandler)));
                    handlers.push(Rc::new(
                        RefCell::new(wikipedia::WikipediaCodeHandler::new()),
                    ));
                } else {
                    handlers.push(Rc::new(RefCell::new(markdown::CodeHandler)));
                }

                convert_html_to_markdown(&body[..], &mut handlers)
            }
            ContentType::Plaintext => Ok(std::str::from_utf8(&body)?.to_owned()),
            ContentType::Json => {
                let json: serde_json::Value = serde_json::from_slice(&body)?;

                Ok(format!(
                    "```json\n{}\n```",
                    serde_json::to_string_pretty(&json)?
                ))
            }
        }
    }
}

impl PickerDelegate for FetchContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        if self.url.is_empty() {
            0
        } else {
            1
        }
    }

    fn no_matches_text(&self, _cx: &mut WindowContext) -> SharedString {
        "Enter the URL that you would like to fetch".into()
    }

    fn selected_index(&self) -> usize {
        0
    }

    fn set_selected_index(&mut self, _ix: usize, _cx: &mut ViewContext<Picker<Self>>) {}

    fn placeholder_text(&self, _cx: &mut ui::WindowContext) -> Arc<str> {
        "Enter a URL…".into()
    }

    fn update_matches(&mut self, query: String, _cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        self.url = query;

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let http_client = workspace.read(cx).client().http_client().clone();
        let url = self.url.clone();
        cx.spawn(|this, mut cx| async move {
            let text = Self::build_message(http_client, &url).await?;

            this.update(&mut cx, |this, cx| {
                this.delegate
                    .context_store
                    .update(cx, |context_store, _cx| {
                        context_store.insert_context(ContextKind::FetchedUrl, url, text);
                    })
            })??;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |this, cx| {
                this.reset_mode();
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(
            ListItem::new(ix)
                .inset(true)
                .toggle_state(selected)
                .child(Label::new(self.url.clone())),
        )
    }
}
