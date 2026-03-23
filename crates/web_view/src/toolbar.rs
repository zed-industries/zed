use editor::Editor;
use gpui::{Context, Entity, EventEmitter, Focusable, Render, Subscription, Window};
use menu::Confirm;
use ui::{Tooltip, prelude::*};
use workspace::item::ItemHandle;
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

use crate::WebView;

pub struct WebViewToolbar {
    active_web_view: Option<Entity<WebView>>,
    url_editor: Option<Entity<Editor>>,
    validation_error: Option<SharedString>,
    _observe_webview: Option<Subscription>,
}

impl Default for WebViewToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl WebViewToolbar {
    pub fn new() -> Self {
        Self {
            active_web_view: None,
            url_editor: None,
            validation_error: None,
            _observe_webview: None,
        }
    }

    fn normalized_url(input: &str) -> Option<String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return None;
        }
        if url::Url::parse(trimmed).is_ok() {
            return Some(trimmed.to_string());
        }

        let prefixed = format!("https://{trimmed}");
        url::Url::parse(&prefixed).ok()?;
        Some(prefixed)
    }

    fn navigate(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let (Some(web_view), Some(url_editor)) = (&self.active_web_view, &self.url_editor) {
            let input = url_editor.read(cx).text(cx);
            if let Some(url) = Self::normalized_url(&input) {
                self.validation_error = None;
                web_view.update(cx, |view, cx| view.navigate(url, cx));
                web_view.read(cx).focus_webview();
            } else {
                self.validation_error = Some("Enter a valid URL".into());
            }
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.navigate(window, cx);
    }
}

impl EventEmitter<ToolbarItemEvent> for WebViewToolbar {}

impl Render for WebViewToolbar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(web_view) = &self.active_web_view else {
            return div().into_any_element();
        };
        let Some(url_editor) = &self.url_editor else {
            return div().into_any_element();
        };

        let should_focus_url_bar =
            web_view.update(cx, |view, _cx| view.take_focus_url_bar_request());
        if should_focus_url_bar {
            url_editor.focus_handle(cx).focus(window, cx);
        }

        let (url, is_loading, can_go_back, can_go_forward) = {
            let state = web_view.read(cx);
            (
                state.url().to_string(),
                state.is_loading(),
                state.can_go_back(),
                state.can_go_forward(),
            )
        };

        let is_url_editor_focused = url_editor.focus_handle(cx).is_focused(window);
        url_editor.update(cx, |editor, cx| {
            if !is_url_editor_focused && editor.text(cx) != url {
                editor.set_text(url.as_str(), window, cx);
            }
        });

        v_flex()
            .child(
                h_flex()
                    .key_context("WebViewToolbar")
                    .on_action(cx.listener(Self::confirm))
                    .gap_1()
                    .px_2()
                    .py_1()
                    .child(
                        IconButton::new("back", IconName::ArrowLeft)
                            .icon_size(IconSize::Small)
                            .disabled(!can_go_back)
                            .tooltip(Tooltip::text("Back"))
                            .on_click({
                                let web_view = web_view.clone();
                                move |_event, _window, cx| {
                                    web_view.read(cx).go_back();
                                }
                            }),
                    )
                    .child(
                        IconButton::new("forward", IconName::ArrowRight)
                            .icon_size(IconSize::Small)
                            .disabled(!can_go_forward)
                            .tooltip(Tooltip::text("Forward"))
                            .on_click({
                                let web_view = web_view.clone();
                                move |_event, _window, cx| {
                                    web_view.read(cx).go_forward();
                                }
                            }),
                    )
                    .child(
                        IconButton::new("reload", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Reload"))
                            .on_click({
                                let web_view = web_view.clone();
                                move |_event, _window, cx| {
                                    web_view.update(cx, |view, cx| view.reload_page(cx));
                                }
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_64()
                            .h_7()
                            .px_2()
                            .py_1()
                            .bg(cx.theme().colors().editor_background)
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .rounded_md()
                            .child(url_editor.clone()),
                    )
                    .when(is_loading, |this| {
                        this.child(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                    })
                    .when(!is_loading, |this| {
                        this.child(
                            IconButton::new("navigate", IconName::ArrowRight)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text("Navigate"))
                                .on_click(cx.listener(|this, _event, window, cx| {
                                    this.navigate(window, cx);
                                })),
                        )
                    }),
            )
            .when_some(self.validation_error.clone(), |this, message| {
                this.child(
                    h_flex()
                        .px_2()
                        .pb_1()
                        .child(Label::new(message).color(Color::Error)),
                )
            })
            .into_any_element()
    }
}

impl ToolbarItemView for WebViewToolbar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active_web_view = active_pane_item
            .and_then(|item| item.act_as::<WebView>(cx))
            .map(|entity| entity.clone());
        self.validation_error = None;
        self._observe_webview = None;

        if let Some(web_view) = &self.active_web_view {
            self._observe_webview = Some(cx.observe(web_view, |_, _, cx| {
                cx.notify();
            }));

            let url_editor = self.url_editor.get_or_insert_with(|| {
                cx.new(|cx| {
                    let mut editor = Editor::single_line(window, cx);
                    editor.set_placeholder_text("https://example.com", window, cx);
                    editor
                })
            });

            let url = web_view.read(cx).url().to_string();
            url_editor.update(cx, |editor, cx| editor.set_text(&*url, window, cx));
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}
