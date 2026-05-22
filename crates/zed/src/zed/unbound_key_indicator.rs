use gpui::{App, Context, IntoElement, Keystroke, Render, SharedString, Subscription, Window};
use ui::text_for_keystrokes;
use workspace::{HideStatusItem, StatusItemView, item::ItemHandle, ui::prelude::*};

pub struct UnboundKeyIndicator {
    message: Option<SharedString>,
    pending_keystrokes: Option<Vec<Keystroke>>,
    _keystroke_subscription: Subscription,
    _pending_input_subscription: Subscription,
}

impl UnboundKeyIndicator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let keystroke_subscription = cx.observe_keystrokes(|this, event, _window, cx| {
            if event.action.is_some() {
                this.pending_keystrokes = None;
                this.message = None;
                cx.notify();
            } else if let Some(ref mut keystrokes) = this.pending_keystrokes {
                keystrokes.push(event.keystroke.clone());
            } else if should_show_unbound(&event.keystroke) {
                let text = text_for_keystrokes(std::slice::from_ref(&event.keystroke), cx);
                this.message = Some(format!("{text} is not bound").into());
                cx.notify();
            } else if this.message.is_some() {
                this.message = None;
                cx.notify();
            }
        });

        let pending_input_subscription =
            cx.observe_pending_input(window, |this, window, cx| {
                if let Some(keystrokes) = window.pending_input_keystrokes() {
                    this.pending_keystrokes = Some(keystrokes.to_vec());
                    if this.message.is_some() {
                        this.message = None;
                        cx.notify();
                    }
                } else if let Some(keystrokes) = this.pending_keystrokes.take() {
                    let text = text_for_keystrokes(&keystrokes, cx);
                    this.message = Some(format!("{text} is not bound").into());
                    cx.notify();
                }
            });

        Self {
            message: None,
            pending_keystrokes: None,
            _keystroke_subscription: keystroke_subscription,
            _pending_input_subscription: pending_input_subscription,
        }
    }
}

fn should_show_unbound(keystroke: &Keystroke) -> bool {
    const MODIFIER_KEYS: &[&str] = &["shift", "control", "alt", "platform", "function"];
    if MODIFIER_KEYS.contains(&keystroke.key.as_str()) {
        return false;
    }

    let has_command_modifiers =
        keystroke.modifiers.control || keystroke.modifiers.alt || keystroke.modifiers.platform;

    has_command_modifiers || keystroke.key_char.is_none()
}

impl Render for UnboundKeyIndicator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let Some(message) = self.message.clone() else {
            return div().into_any_element();
        };

        Label::new(message)
            .size(LabelSize::Small)
            .color(Color::Muted)
            .into_any_element()
    }
}

impl StatusItemView for UnboundKeyIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _cx: &App) -> Option<HideStatusItem> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        self as gpui, Entity, FocusHandle, InteractiveElement, KeyBinding, Modifiers,
        ParentElement, TestAppContext, actions, div,
    };

    fn keystroke(key: &str, modifiers: Modifiers, key_char: Option<&str>) -> Keystroke {
        Keystroke {
            key: key.to_string(),
            modifiers,
            key_char: key_char.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_modifier_only_keystrokes_are_hidden() {
        for key in &["shift", "control", "alt", "platform", "function"] {
            let ks = keystroke(key, Modifiers::default(), None);
            assert!(
                !should_show_unbound(&ks),
                "modifier-only key '{key}' should not trigger the indicator"
            );
        }
    }

    #[test]
    fn test_plain_character_input_is_hidden() {
        let ks = keystroke("a", Modifiers::default(), Some("a"));
        assert!(
            !should_show_unbound(&ks),
            "plain 'a' with key_char should not trigger the indicator"
        );
    }

    #[test]
    fn test_shift_only_character_is_hidden() {
        let ks = keystroke(
            "a",
            Modifiers {
                shift: true,
                ..Modifiers::default()
            },
            Some("A"),
        );
        assert!(
            !should_show_unbound(&ks),
            "shift-a should not trigger the indicator"
        );
    }

    #[test]
    fn test_ctrl_modified_keystroke_is_shown() {
        let ks = keystroke(
            "k",
            Modifiers {
                control: true,
                ..Modifiers::default()
            },
            None,
        );
        assert!(
            should_show_unbound(&ks),
            "ctrl-k should trigger the indicator"
        );
    }

    #[test]
    fn test_alt_modified_keystroke_is_shown() {
        let ks = keystroke(
            "f",
            Modifiers {
                alt: true,
                ..Modifiers::default()
            },
            None,
        );
        assert!(
            should_show_unbound(&ks),
            "alt-f should trigger the indicator"
        );
    }

    #[test]
    fn test_platform_modified_keystroke_is_shown() {
        let ks = keystroke(
            "s",
            Modifiers {
                platform: true,
                ..Modifiers::default()
            },
            None,
        );
        assert!(
            should_show_unbound(&ks),
            "cmd/super-s should trigger the indicator"
        );
    }

    #[test]
    fn test_ctrl_shift_modified_keystroke_is_shown() {
        let ks = keystroke(
            "k",
            Modifiers {
                control: true,
                shift: true,
                ..Modifiers::default()
            },
            None,
        );
        assert!(
            should_show_unbound(&ks),
            "ctrl-shift-k should trigger the indicator"
        );
    }

    #[test]
    fn test_function_key_without_modifiers_is_shown() {
        let ks = keystroke("f5", Modifiers::default(), None);
        assert!(
            should_show_unbound(&ks),
            "F5 (no key_char) should trigger the indicator"
        );
    }

    #[test]
    fn test_arrow_key_without_modifiers_is_shown() {
        let ks = keystroke("left", Modifiers::default(), None);
        assert!(
            should_show_unbound(&ks),
            "arrow keys (no key_char) should trigger the indicator"
        );
    }

    #[test]
    fn test_ctrl_with_key_char_is_shown() {
        let ks = keystroke(
            "c",
            Modifiers {
                control: true,
                ..Modifiers::default()
            },
            Some("c"),
        );
        assert!(
            should_show_unbound(&ks),
            "ctrl-c with key_char should still trigger (command modifier takes priority)"
        );
    }

    struct TestView {
        focus_handle: FocusHandle,
    }

    actions!(unbound_key_indicator_test, [TestAction]);

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .key_context("test")
                .track_focus(&self.focus_handle)
                .on_action(cx.listener(|_: &mut Self, _: &TestAction, _, _| {}))
                .child(div())
        }
    }

    fn setup_indicator(cx: &mut TestAppContext) -> (gpui::AnyWindowHandle, Entity<UnboundKeyIndicator>) {
        let window = cx.update(|cx| {
            cx.open_window(Default::default(), |_, cx| {
                cx.new(|cx| TestView {
                    focus_handle: cx.focus_handle(),
                })
            })
            .unwrap()
        });

        let indicator = window
            .update(cx, |view, window, cx| {
                window.focus(&view.focus_handle, cx);
                cx.new(|cx| UnboundKeyIndicator::new(window, cx))
            })
            .unwrap();

        (*window, indicator)
    }

    #[gpui::test]
    fn test_indicator_initially_empty(cx: &mut TestAppContext) {
        let (_, indicator) = setup_indicator(cx);

        indicator.read_with(cx, |indicator, _| {
            assert!(indicator.message.is_none());
        });
    }

    #[gpui::test]
    fn test_indicator_shows_unbound_keystroke(cx: &mut TestAppContext) {
        let (window, indicator) = setup_indicator(cx);

        cx.dispatch_keystroke(window, Keystroke::parse("ctrl-shift-k").unwrap());

        indicator.read_with(cx, |indicator, _| {
            let message = indicator.message.as_ref().expect("should show unbound message");
            assert!(
                message.contains("is not bound"),
                "expected 'is not bound' in message, got: {message}"
            );
        });
    }

    #[gpui::test]
    fn test_indicator_hidden_for_plain_typing(cx: &mut TestAppContext) {
        let (window, indicator) = setup_indicator(cx);

        cx.dispatch_keystroke(window, Keystroke::parse("a").unwrap());

        indicator.read_with(cx, |indicator, _| {
            assert!(
                indicator.message.is_none(),
                "plain typing should not show unbound message"
            );
        });
    }

    #[gpui::test]
    fn test_indicator_cleared_by_next_plain_keystroke(cx: &mut TestAppContext) {
        let (window, indicator) = setup_indicator(cx);

        cx.dispatch_keystroke(window, Keystroke::parse("ctrl-shift-k").unwrap());
        indicator.read_with(cx, |indicator, _| {
            assert!(indicator.message.is_some());
        });

        cx.dispatch_keystroke(window, Keystroke::parse("a").unwrap());
        indicator.read_with(cx, |indicator, _| {
            assert!(
                indicator.message.is_none(),
                "message should be cleared after plain keystroke"
            );
        });
    }

    #[gpui::test]
    fn test_indicator_cleared_by_bound_action(cx: &mut TestAppContext) {
        let (window, indicator) = setup_indicator(cx);

        cx.update(|cx| {
            cx.bind_keys(vec![KeyBinding::new("ctrl-g", TestAction, Some("test"))]);
        });

        cx.dispatch_keystroke(window, Keystroke::parse("ctrl-shift-k").unwrap());
        indicator.read_with(cx, |indicator, _| {
            assert!(indicator.message.is_some());
        });

        cx.dispatch_keystroke(window, Keystroke::parse("ctrl-g").unwrap());
        indicator.read_with(cx, |indicator, _| {
            assert!(
                indicator.message.is_none(),
                "message should be cleared when a bound action fires"
            );
        });
    }
}
