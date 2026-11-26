use gpui::{
    App, Decorations, Entity, EventEmitter, FocusHandle, Focusable, PromptButton, PromptHandle,
    PromptLevel, PromptResponse, RenderablePromptHandle, ScrollHandle, SharedString,
    TextStyleRefinement, Window, div, prelude::*,
};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use settings::{Settings, SettingsStore};
use theme::ClientDecorationsExt;
use theme_settings::ThemeSettings;
use ui::{ButtonLike, FluentBuilder, TintColor, prelude::*};
use workspace::WorkspaceSettings;

pub fn init(cx: &mut App) {
    process_settings(cx);

    cx.observe_global::<SettingsStore>(process_settings)
        .detach();
}

fn process_settings(cx: &mut App) {
    if use_system_prompts(cx) {
        cx.reset_prompt_builder();
    } else {
        cx.set_prompt_builder(zed_prompt_renderer);
    }
}

fn use_system_prompts(cx: &App) -> bool {
    WorkspaceSettings::get_global(cx).use_system_prompts
        && cfg!(not(any(target_os = "linux", target_os = "freebsd")))
}

/// Present a scrollable list of descriptive choices followed by Cancel.
pub fn prompt_for_choices(
    message: &str,
    detail: &str,
    choices: Vec<SharedString>,
    window: &mut Window,
    cx: &mut App,
) -> futures::channel::oneshot::Receiver<usize> {
    let mut actions: Vec<_> = choices.into_iter().map(PromptButton::new).collect();
    actions.push(PromptButton::cancel("Cancel"));
    if use_system_prompts(cx) {
        return window.prompt(PromptLevel::Info, message, Some(detail), &actions, cx);
    }
    let view = cx.new(|cx| {
        ZedPromptRenderer::new(
            message,
            Some(detail),
            actions,
            Some(ScrollHandle::new()),
            cx,
        )
    });
    window.prompt_with_view(view, cx)
}

/// Use this function in conjunction with [App::set_prompt_builder] to force
/// GPUI to use the internal prompt system.
fn zed_prompt_renderer(
    _level: PromptLevel,
    message: &str,
    detail: Option<&str>,
    actions: &[PromptButton],
    handle: PromptHandle,
    window: &mut Window,
    cx: &mut App,
) -> RenderablePromptHandle {
    let renderer = cx.new(|cx| ZedPromptRenderer::new(message, detail, actions.to_vec(), None, cx));
    handle.with_view(renderer, window, cx)
}

pub struct ZedPromptRenderer {
    message: Entity<Markdown>,
    actions: Vec<PromptButton>,
    choice_scroll: Option<ScrollHandle>,
    focus: FocusHandle,
    active_action_id: usize,
    detail: Option<Entity<Markdown>>,
}

impl ZedPromptRenderer {
    fn new(
        message: &str,
        detail: Option<&str>,
        actions: Vec<PromptButton>,
        choice_scroll: Option<ScrollHandle>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            message: cx.new(|cx| Markdown::new(SharedString::new(message), None, None, cx)),
            actions,
            choice_scroll,
            focus: cx.focus_handle(),
            active_action_id: 0,
            detail: detail
                .filter(|text| !text.is_empty())
                .map(|text| cx.new(|cx| Markdown::new(SharedString::new(text), None, None, cx))),
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PromptResponse(self.active_action_id));
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.actions.iter().position(|action| {
            if self.choice_scroll.is_some() {
                action.is_cancel()
            } else {
                action.label() == "Cancel"
            }
        }) {
            cx.emit(PromptResponse(ix));
        }
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select(self.actions.len().saturating_sub(1), cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        self.select(0, cx);
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        self.select((self.active_action_id + 1) % self.actions.len(), cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let index = if self.active_action_id > 0 {
            self.active_action_id - 1
        } else {
            self.actions.len().saturating_sub(1)
        };
        self.select(index, cx);
    }

    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        self.active_action_id = index;
        if let Some(scroll) = &self.choice_scroll {
            scroll.scroll_to_item(index);
        }
        cx.notify();
    }
}

impl Render for ZedPromptRenderer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);

        let dialog = v_flex()
            .key_context("Prompt")
            .cursor_default()
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .w_80()
            .when(self.choice_scroll.is_some(), |dialog| {
                dialog.max_w_full().max_h_full()
            })
            .p_4()
            .gap_4()
            .elevation_3(cx)
            .overflow_hidden()
            .font_family(settings.ui_font.family.clone())
            .child(div().w_full().child(MarkdownElement::new(
                self.message.clone(),
                markdown_style(true, window, cx),
            )))
            .children(self.detail.clone().map(|detail| {
                div().w_full().text_xs().child(MarkdownElement::new(
                    detail,
                    markdown_style(false, window, cx),
                ))
            }))
            .child(
                v_flex()
                    .id("prompt-actions")
                    .when_some(self.choice_scroll.as_ref(), |list, scroll| {
                        list.min_h_0().overflow_y_scroll().track_scroll(scroll)
                    })
                    .gap_1()
                    .children(self.actions.iter().enumerate().map(|(ix, action)| {
                        let label = action.label();
                        let button = if self.choice_scroll.is_some() && !action.is_cancel() {
                            ButtonLike::new(ix)
                                .auto_height()
                                .aria_label(label.clone())
                                .child(div().min_w_0().child(Label::new(label.clone())))
                        } else {
                            Button::new(ix, label.clone()).render(window, cx)
                        };
                        button
                            .full_width()
                            .style(ButtonStyle::Outlined)
                            .when(ix == self.active_action_id, |s| {
                                s.style(ButtonStyle::Tinted(TintColor::Accent))
                            })
                            .tab_index(ix as isize)
                            .on_click(cx.listener(move |_, _, _window, cx| {
                                cx.emit(PromptResponse(ix));
                            }))
                    })),
            );

        let decorations = window.window_decorations();
        let inset = window.client_inset().unwrap_or(Pixels::ZERO);

        div().size_full().child(
            v_flex()
                .occlude()
                .absolute()
                .inset_0()
                .bg(gpui::black().opacity(0.2))
                .map(|this| match decorations {
                    Decorations::Server => this,
                    Decorations::Client { tiling } => this
                        .when(!tiling.top, |this| this.top(inset))
                        .when(!tiling.bottom, |this| this.bottom(inset))
                        .when(!tiling.left, |this| this.left(inset))
                        .when(!tiling.right, |this| this.right(inset))
                        .rounded_client_corners(tiling),
                })
                .items_center()
                .justify_center()
                .child(dialog),
        )
    }
}

fn markdown_style(main_message: bool, window: &Window, cx: &App) -> MarkdownStyle {
    let mut base_text_style = window.text_style();
    let settings = ThemeSettings::get_global(cx);
    let font_size = settings.ui_font_size(cx).into();

    let color = if main_message {
        Color::Default.color(cx)
    } else {
        Color::Muted.color(cx)
    };

    base_text_style.refine(&TextStyleRefinement {
        font_family: Some(settings.ui_font.family.clone()),
        font_size: Some(font_size),
        color: Some(color),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style,
        selection_background_color: cx.theme().colors().element_selection_background,
        ..Default::default()
    }
}

impl EventEmitter<PromptResponse> for ZedPromptRenderer {}

impl Focusable for ZedPromptRenderer {
    fn focus_handle(&self, _: &crate::App) -> FocusHandle {
        self.focus.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use settings::SettingsStore;

    #[gpui::test]
    fn test_long_action_label_wraps_inside_dialog(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let scroll = ScrollHandle::new();
        let cx = cx.add_empty_window();
        let actions = vec![
            PromptButton::new("python3 (Jupyter Server: http://127.0.0.1:18889)"),
            PromptButton::new(
                "Project Python (/data/long-experimental-data-science-notebook-project/.venv/bin/python)",
            ),
            PromptButton::new("Cancel"),
            PromptButton::cancel("Cancel"),
        ];
        let mut response = cx.update(|window, cx| {
            let view = cx.new(|cx| {
                ZedPromptRenderer::new("Select a kernel", None, actions, Some(scroll.clone()), cx)
            });
            window.prompt_with_view(view, cx)
        });
        let viewport = gpui::size(px(280.), px(180.));
        cx.simulate_resize(viewport);
        cx.run_until_parked();

        let list = scroll.bounds();
        assert!(list.left() >= px(16.) && list.right() <= viewport.width - px(16.));
        let short = scroll.bounds_for_item(2).expect("short choice");
        for index in [0, 1] {
            let wrapped = scroll.bounds_for_item(index).expect("long choice");
            assert_eq!(wrapped.size.width, short.size.width);
            assert!(wrapped.size.height > short.size.height);
        }
        let cancel = scroll.bounds_for_item(3).expect("cancel button");
        assert!(cancel.bottom() > list.bottom());
        cx.dispatch_action(menu::SelectPrevious);
        cx.run_until_parked();
        assert!(cancel.bottom() + scroll.offset().y <= list.bottom());
        cx.dispatch_action(menu::Cancel);
        assert_eq!(response.try_recv().expect("prompt response"), Some(3));
    }
}
