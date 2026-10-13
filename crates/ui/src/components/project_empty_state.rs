use crate::{ButtonLike, Divider, DividerColor, KeyBinding, prelude::*};
use gpui::{ClickEvent, FocusHandle};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ProjectEmptyState {
    label: SharedString,
    focus_handle: FocusHandle,
    open_project_key_binding: KeyBinding,
    on_open_project: Option<ClickHandler>,
    on_clone_repo: Option<ClickHandler>,
}

impl ProjectEmptyState {
    pub fn new(
        label: impl Into<SharedString>,
        focus_handle: FocusHandle,
        open_project_key_binding: KeyBinding,
    ) -> Self {
        Self {
            label: label.into(),
            focus_handle,
            open_project_key_binding,
            on_open_project: None,
            on_clone_repo: None,
        }
    }

    pub fn on_open_project(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_project = Some(Box::new(handler));
        self
    }

    pub fn on_clone_repo(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_clone_repo = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ProjectEmptyState {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = format!("empty-state-{}", self.label);
        let label = format!("Choose one of the options below to use the {}", self.label);
        let keyshortcuts = self
            .open_project_key_binding
            .keyboard_shortcut_text(window, cx);
        let show_key_binding = KeyBinding::default_visibility(cx) && keyshortcuts.is_some();

        v_flex()
            .id(id)
            .p_4()
            .size_full()
            .items_center()
            .justify_center()
            .track_focus(&self.focus_handle)
            .child(
                v_flex()
                    .w_48()
                    .max_w_full()
                    .gap_1()
                    .child(
                        div()
                            .text_center()
                            .mb_2()
                            .child(Label::new(label).size(LabelSize::Small).color(Color::Muted)),
                    )
                    .child(
                        div().debug_selector(|| "OPEN_PROJECT_BUTTON".into()).child(
                            ButtonLike::new("open_project")
                                .full_width()
                                .auto_height()
                                .aria_label("Open Project")
                                .when_some(keyshortcuts, |button, keyshortcuts| {
                                    button.aria_keyshortcuts(keyshortcuts)
                                })
                                .child(
                                    h_flex()
                                        .w_full()
                                        .flex_wrap()
                                        .justify_center()
                                        .gap_x(DynamicSpacing::Base06.rems(cx))
                                        .child(
                                            h_flex()
                                                .debug_selector(|| "OPEN_PROJECT_LABEL".into())
                                                .flex_none()
                                                .h(ButtonSize::Default.rems())
                                                .child(Label::new("Open Project")),
                                        )
                                        .when(show_key_binding, |this| {
                                            this.child(
                                                h_flex()
                                                    .flex_none()
                                                    .min_h(ButtonSize::Default.rems())
                                                    .child(self.open_project_key_binding),
                                            )
                                        }),
                                )
                                .when_some(self.on_open_project, |button, handler| {
                                    button.on_click(handler)
                                }),
                        ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Divider::horizontal().color(DividerColor::Border))
                            .child(Label::new("or").size(LabelSize::XSmall).color(Color::Muted))
                            .child(Divider::horizontal().color(DividerColor::Border)),
                    )
                    .child(
                        Button::new("clone_repo", "Clone Repository")
                            .full_width()
                            .when_some(self.on_clone_repo, |button, handler| {
                                button.on_click(handler)
                            }),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{KeybindingKeystroke, Keystroke, Modifiers, TestAppContext};

    #[gpui::test]
    fn test_open_project_button_contains_label_and_shortcut(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let (view, cx) = cx.add_window_view(|_, cx| ProjectEmptyStateTestView {
            width: px(320.),
            key_binding: key_binding("cmd-o", false),
            focus_handle: cx.focus_handle(),
            clicks: 0,
        });

        for (width, shortcut, vim_mode, selector) in [
            (160., "ctrl-k ctrl-o", true, "KEY_BINDING-k o"),
            (320., "cmd-o", false, "KEY_BINDING-o"),
            (160., "cmd-o", false, "KEY_BINDING-o"),
            (320., "ctrl-k ctrl-o", true, "KEY_BINDING-k o"),
        ] {
            view.update(cx, |view, cx| {
                view.width = px(width);
                view.key_binding = key_binding(shortcut, vim_mode);
                cx.notify();
            });
            cx.run_until_parked();

            let button_bounds = cx
                .debug_bounds("OPEN_PROJECT_BUTTON")
                .expect("button bounds");
            let shortcut_bounds = cx.debug_bounds(selector).expect("shortcut bounds");
            assert_eq!(
                button_bounds.intersect(&shortcut_bounds),
                shortcut_bounds,
                "{shortcut} must fit inside the button at width {width}: {button_bounds:?}, {shortcut_bounds:?}"
            );
            let label_bounds = cx.debug_bounds("OPEN_PROJECT_LABEL").expect("label bounds");
            assert_eq!(button_bounds.intersect(&label_bounds), label_bounds);
            assert!(!label_bounds.intersects(&shortcut_bounds));
            if width == 320. && shortcut == "cmd-o" {
                assert_eq!(label_bounds.center().y, shortcut_bounds.center().y);
            }

            cx.simulate_mouse_move(button_bounds.center(), None, Modifiers::default());
            assert_eq!(cx.debug_bounds("OPEN_PROJECT_BUTTON"), Some(button_bounds));
            assert_eq!(cx.debug_bounds("OPEN_PROJECT_LABEL"), Some(label_bounds));
            assert_eq!(cx.debug_bounds(selector), Some(shortcut_bounds));
            cx.simulate_click(button_bounds.center(), Modifiers::default());
        }
        assert_eq!(view.read_with(cx, |view, _| view.clicks), 4);

        view.update(cx, |view, cx| {
            view.width = px(160.);
            KeyBinding::set_default_visibility(cx, false);
            cx.notify();
        });
        cx.run_until_parked();
        let button_bounds = cx
            .debug_bounds("OPEN_PROJECT_BUTTON")
            .expect("button bounds");
        let label_bounds = cx.debug_bounds("OPEN_PROJECT_LABEL").expect("label bounds");
        assert_eq!(button_bounds.size.height, label_bounds.size.height);
        assert!(cx.debug_bounds("KEY_BINDING-k o").is_none());
    }

    fn key_binding(shortcut: &str, vim_mode: bool) -> KeyBinding {
        let keystrokes = shortcut
            .split_whitespace()
            .map(|keystroke| {
                KeybindingKeystroke::from_keystroke(
                    Keystroke::parse(keystroke).expect("valid test keystroke"),
                )
            })
            .collect::<Vec<_>>()
            .into();
        KeyBinding::from_keystrokes(keystrokes, vim_mode)
    }

    struct ProjectEmptyStateTestView {
        width: Pixels,
        key_binding: KeyBinding,
        focus_handle: FocusHandle,
        clicks: usize,
    }

    impl Render for ProjectEmptyStateTestView {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div().w(self.width).h(px(400.)).child(
                ProjectEmptyState::new(
                    "project panel",
                    self.focus_handle.clone(),
                    self.key_binding.clone(),
                )
                .on_open_project(cx.listener(|view, _, _, _| view.clicks += 1)),
            )
        }
    }
}
