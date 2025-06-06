use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{Context as _, Result, bail};
use futures::AsyncReadExt as _;
use gpui::{App, DismissEvent, Entity, FocusHandle, Focusable, Task, WeakEntity};
use html_to_markdown::{TagHandler, convert_html_to_markdown, markdown};
use http_client::{AsyncBody, HttpClientWithUrl};
use picker::{Picker, PickerDelegate};
use ui::{Context, ListItem, Window, prelude::*};
use workspace::Workspace;

use crate::context_picker::ContextPicker;
use crate::context_store::ContextStore;

pub struct FetchContextPicker {
    picker: Entity<Picker<FetchContextPickerDelegate>>,
}

impl FetchContextPicker {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = FetchContextPickerDelegate::new(context_picker, workspace, context_store);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self { picker }
    }
}

impl Focusable for FetchContextPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for FetchContextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
    context_picker: WeakEntity<ContextPicker>,
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    url: String,
}

impl FetchContextPickerDelegate {
    pub fn new(
        context_picker: WeakEntity<ContextPicker>,
        workspace: WeakEntity<Workspace>,
        context_store: WeakEntity<ContextStore>,
    ) -> Self {
        FetchContextPickerDelegate {
            context_picker,
            workspace,
            context_store,
            url: String::new(),
        }
    }
}

pub(crate) async fn fetch_url_content(
    http_client: Arc<HttpClientWithUrl>,
    url: String,
) -> Result<String> {
    let url = if !url.starts_with("https://") && !url.starts_with("http://") {
        format!("https://{url}")
    } else {
        url
    };

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

impl PickerDelegate for FetchContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        if self.url.is_empty() { 0 } else { 1 }
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("Enter the URL that you would like to fetch".into())
    }

    fn selected_index(&self) -> usize {
        0
    }

    fn set_selected_index(
        &mut self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Enter a URL…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.url = query;

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let http_client = workspace.read(cx).client().http_client();
        let url = self.url.clone();
        cx.spawn_in(window, async move |this, cx| {
            let text = cx
                .background_spawn(fetch_url_content(http_client, url.clone()))
                .await?;

            this.update(cx, |this, cx| {
                this.delegate.context_store.update(cx, |context_store, cx| {
                    context_store.add_fetched_url(url, text, cx)
                })
            })??;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.context_picker
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let added = self.context_store.upgrade().map_or(false, |context_store| {
            context_store.read(cx).includes_url(&self.url)
        });

        Some(
            ListItem::new(ix)
                .inset(true)
                .toggle_state(selected)
                .child(Label::new(self.url.clone()))
                .when(added, |child| {
                    child.disabled(true).end_slot(
                        h_flex()
                            .gap_1()
                            .child(
                                Icon::new(IconName::Check)
                                    .size(IconSize::Small)
                                    .color(Color::Success),
                            )
                            .child(Label::new("Added").size(LabelSize::Small)),
                    )
                }),
        )
    }
}
