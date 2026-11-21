use gpui::{
    App, Context, Element, EventEmitter, FontWeight, Global, Hsla, Render, Subscription, Window,
    div,
};
use ui::text_for_keystrokes;
use workspace::{HideStatusItem, StatusItemView, item::ItemHandle, ui::prelude::*};

/// Emitted by a mode indicator source when its editor gains focus, so the
/// indicator follows the focused editor.
pub struct ModeIndicatorFocused;

/// Per-editor state of a modal editing mode that the mode indicator displays.
pub trait ModeIndicatorSource: EventEmitter<ModeIndicatorFocused> + 'static {
    fn mode_status(&self, cx: &App) -> ModeStatus;
}

pub struct ModeStatus {
    /// Replaces both the mode and the pending input when set.
    pub label: Option<SharedString>,
    pub mode: String,
    pub pending: String,
    pub foreground: Hsla,
    pub background: Hsla,
}

#[derive(Default)]
struct ModeIndicatorSources(Vec<fn(&mut ModeIndicator, &mut Window, &mut Context<ModeIndicator>)>);

impl Global for ModeIndicatorSources {}

/// Makes mode indicators created afterwards follow entities of type `T`.
pub fn register_mode_indicator_source<T: ModeIndicatorSource>(cx: &mut App) {
    cx.default_global::<ModeIndicatorSources>()
        .0
        .push(ModeIndicator::observe_source::<T>);
}

/// The ModeIndicator displays the current mode in the status bar.
pub struct ModeIndicator {
    mode_status: Option<Box<dyn Fn(&App) -> Option<ModeStatus>>>,
    pending_keys: Option<String>,
    source_subscription: Option<Subscription>,
}

impl ModeIndicator {
    /// Construct a new mode indicator in this window.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe_pending_input(window, |this: &mut Self, window, cx| {
            this.update_pending_keys(window, cx);
            cx.notify();
        })
        .detach();

        let mut this = Self {
            mode_status: None,
            pending_keys: None,
            source_subscription: None,
        };
        let sources = cx
            .try_global::<ModeIndicatorSources>()
            .map(|sources| sources.0.clone())
            .unwrap_or_default();
        for observe_source in sources {
            observe_source(&mut this, window, cx);
        }
        this
    }

    fn observe_source<T: ModeIndicatorSource>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let handle = cx.entity();
        let window_handle = window.window_handle();
        cx.observe_new::<T>(move |_, window, cx| {
            let Some(window) = window else {
                return;
            };
            if window.window_handle() != window_handle {
                return;
            }
            let source = cx.entity();
            handle.update(cx, |_, cx| {
                cx.subscribe(
                    &source,
                    |mode_indicator, source, _: &ModeIndicatorFocused, cx| {
                        mode_indicator.source_subscription =
                            Some(cx.observe(&source, |_, _, cx| cx.notify()));
                        let source = source.downgrade();
                        mode_indicator.mode_status = Some(Box::new(move |cx| {
                            source
                                .upgrade()
                                .map(|source| source.read(cx).mode_status(cx))
                        }));
                    },
                )
                .detach()
            })
        })
        .detach();
    }

    fn update_pending_keys(&mut self, window: &mut Window, cx: &App) {
        self.pending_keys = window
            .pending_input_keystrokes()
            .map(|keystrokes| text_for_keystrokes(keystrokes, cx));
    }
}

impl Render for ModeIndicator {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(status) = self
            .mode_status
            .as_ref()
            .and_then(|mode_status| mode_status(cx))
        else {
            return div().hidden().into_any_element();
        };

        let system_transparent = gpui::hsla(0.0, 0.0, 0.0, 0.0);
        let vim_mode_text = status.foreground;
        let bg_color = status.background;

        let (label, mode): (SharedString, Option<SharedString>) = if let Some(label) = status.label
        {
            (label, None)
        } else {
            let mode_str = status.mode;
            let pending = self.pending_keys.as_ref().unwrap_or(&status.pending);
            let mode = if bg_color != system_transparent {
                mode_str.into()
            } else {
                format!("-- {} --", mode_str).into()
            };
            (pending.into(), Some(mode))
        };
        h_flex()
            .h(ButtonSize::Default.rems())
            .gap_1()
            .when(!label.is_empty(), |el| {
                el.child(
                    Label::new(label)
                        .size(LabelSize::Small)
                        .weight(FontWeight::MEDIUM),
                )
            })
            .when_some(mode, |el, mode| {
                el.child(
                    v_flex()
                        .when(bg_color != system_transparent, |el| el.px_2())
                        // match with other icons at the bottom that use default buttons
                        .h(ButtonSize::Default.rems())
                        .justify_center()
                        .rounded_sm()
                        .bg(bg_color)
                        .child(
                            Label::new(mode)
                                .size(LabelSize::Small)
                                .weight(FontWeight::MEDIUM)
                                .when(
                                    bg_color != system_transparent
                                        && vim_mode_text != system_transparent,
                                    |el| el.color(Color::Custom(vim_mode_text)),
                                ),
                        ),
                )
            })
            .into_any()
    }
}

impl StatusItemView for ModeIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        // The mode indicator is only visible while a modal editing mode is on.
        None
    }
}
