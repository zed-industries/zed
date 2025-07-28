use gpui::{
    Animation, AnimationExt, Context, EventEmitter, FocusHandle, Focusable, FontWeight, KeyContext,
    Keystroke, Modifiers, ModifiersChangedEvent, Subscription, actions,
};
use ui::{
    ActiveTheme as _, Color, IconButton, IconButtonShape, IconName, IconSize, Label, LabelSize,
    ParentElement as _, Render, Styled as _, Tooltip, Window, prelude::*,
};

actions!(
    keystroke_input,
    [
        /// Starts recording keystrokes
        StartRecording,
        /// Stops recording keystrokes
        StopRecording,
        /// Clears the recorded keystrokes
        ClearKeystrokes,
    ]
);

enum CloseKeystrokeResult {
    Partial,
    Close,
    None,
}

impl PartialEq for CloseKeystrokeResult {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (CloseKeystrokeResult::Partial, CloseKeystrokeResult::Partial)
                | (CloseKeystrokeResult::Close, CloseKeystrokeResult::Close)
                | (CloseKeystrokeResult::None, CloseKeystrokeResult::None)
        )
    }
}

pub struct KeystrokeInput {
    keystrokes: Vec<Keystroke>,
    placeholder_keystrokes: Option<Vec<Keystroke>>,
    outer_focus_handle: FocusHandle,
    inner_focus_handle: FocusHandle,
    intercept_subscription: Option<Subscription>,
    _focus_subscriptions: [Subscription; 2],
    search: bool,
    /// Handles triple escape to stop recording
    close_keystrokes: Option<Vec<Keystroke>>,
    close_keystrokes_start: Option<usize>,
    previous_modifiers: Modifiers,
}

impl KeystrokeInput {
    const KEYSTROKE_COUNT_MAX: usize = 3;

    pub fn new(
        placeholder_keystrokes: Option<Vec<Keystroke>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let outer_focus_handle = cx.focus_handle();
        let inner_focus_handle = cx.focus_handle();
        let _focus_subscriptions = [
            cx.on_focus_in(&inner_focus_handle, window, Self::on_inner_focus_in),
            cx.on_focus_out(&inner_focus_handle, window, Self::on_inner_focus_out),
        ];
        Self {
            keystrokes: Vec::new(),
            placeholder_keystrokes,
            inner_focus_handle,
            outer_focus_handle,
            intercept_subscription: None,
            _focus_subscriptions,
            search: false,
            close_keystrokes: None,
            close_keystrokes_start: None,
            previous_modifiers: Modifiers::default(),
        }
    }

    pub fn set_keystrokes(&mut self, keystrokes: Vec<Keystroke>, cx: &mut Context<Self>) {
        self.keystrokes = keystrokes;
        self.keystrokes_changed(cx);
    }

    pub fn set_search(&mut self, search: bool) {
        self.search = search;
    }

    pub fn keystrokes(&self) -> &[Keystroke] {
        if let Some(placeholders) = self.placeholder_keystrokes.as_ref()
            && self.keystrokes.is_empty()
        {
            return placeholders;
        }
        if !self.search
            && self
                .keystrokes
                .last()
                .map_or(false, |last| last.key.is_empty())
        {
            return &self.keystrokes[..self.keystrokes.len() - 1];
        }
        return &self.keystrokes;
    }

    fn dummy(modifiers: Modifiers) -> Keystroke {
        return Keystroke {
            modifiers,
            key: "".to_string(),
            key_char: None,
        };
    }

    fn keystrokes_changed(&self, cx: &mut Context<Self>) {
        cx.emit(());
        cx.notify();
    }

    fn key_context() -> KeyContext {
        let mut key_context = KeyContext::default();
        key_context.add("KeystrokeInput");
        key_context
    }

    fn handle_possible_close_keystroke(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CloseKeystrokeResult {
        let Some(keybind_for_close_action) = window
            .highest_precedence_binding_for_action_in_context(&StopRecording, Self::key_context())
        else {
            log::trace!("No keybinding to stop recording keystrokes in keystroke input");
            self.close_keystrokes.take();
            self.close_keystrokes_start.take();
            return CloseKeystrokeResult::None;
        };
        let action_keystrokes = keybind_for_close_action.keystrokes();

        if let Some(mut close_keystrokes) = self.close_keystrokes.take() {
            let mut index = 0;

            while index < action_keystrokes.len() && index < close_keystrokes.len() {
                if !close_keystrokes[index].should_match(&action_keystrokes[index]) {
                    break;
                }
                index += 1;
            }
            if index == close_keystrokes.len() {
                if index >= action_keystrokes.len() {
                    self.close_keystrokes_start.take();
                    return CloseKeystrokeResult::None;
                }
                if keystroke.should_match(&action_keystrokes[index]) {
                    if action_keystrokes.len() >= 1 && index == action_keystrokes.len() - 1 {
                        self.stop_recording(&StopRecording, window, cx);
                        return CloseKeystrokeResult::Close;
                    } else {
                        close_keystrokes.push(keystroke.clone());
                        self.close_keystrokes = Some(close_keystrokes);
                        return CloseKeystrokeResult::Partial;
                    }
                } else {
                    self.close_keystrokes_start.take();
                    return CloseKeystrokeResult::None;
                }
            }
        } else if let Some(first_action_keystroke) = action_keystrokes.first()
            && keystroke.should_match(first_action_keystroke)
        {
            self.close_keystrokes = Some(vec![keystroke.clone()]);
            return CloseKeystrokeResult::Partial;
        }
        self.close_keystrokes_start.take();
        return CloseKeystrokeResult::None;
    }

    fn on_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keystrokes_len = self.keystrokes.len();

        if self.previous_modifiers.modified()
            && event.modifiers.is_subset_of(&self.previous_modifiers)
        {
            self.previous_modifiers &= event.modifiers;
            cx.stop_propagation();
            return;
        }

        if let Some(last) = self.keystrokes.last_mut()
            && last.key.is_empty()
            && keystrokes_len <= Self::KEYSTROKE_COUNT_MAX
        {
            if self.search {
                if self.previous_modifiers.modified() {
                    last.modifiers |= event.modifiers;
                    self.previous_modifiers |= event.modifiers;
                } else {
                    self.keystrokes.push(Self::dummy(event.modifiers));
                    self.previous_modifiers |= event.modifiers;
                }
            } else if !event.modifiers.modified() {
                self.keystrokes.pop();
            } else {
                last.modifiers = event.modifiers;
            }

            self.keystrokes_changed(cx);
        } else if keystrokes_len < Self::KEYSTROKE_COUNT_MAX {
            self.keystrokes.push(Self::dummy(event.modifiers));
            if self.search {
                self.previous_modifiers |= event.modifiers;
            }
            self.keystrokes_changed(cx);
        }
        cx.stop_propagation();
    }

    fn handle_keystroke(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let close_keystroke_result = self.handle_possible_close_keystroke(keystroke, window, cx);
        if close_keystroke_result != CloseKeystrokeResult::Close {
            let key_len = self.keystrokes.len();
            if let Some(last) = self.keystrokes.last_mut()
                && last.key.is_empty()
                && key_len <= Self::KEYSTROKE_COUNT_MAX
            {
                if self.search {
                    last.key = keystroke.key.clone();
                    if close_keystroke_result == CloseKeystrokeResult::Partial
                        && self.close_keystrokes_start.is_none()
                    {
                        self.close_keystrokes_start = Some(self.keystrokes.len() - 1);
                    }
                    if self.search {
                        self.previous_modifiers = keystroke.modifiers;
                    }
                    self.keystrokes_changed(cx);
                    cx.stop_propagation();
                    return;
                } else {
                    self.keystrokes.pop();
                }
            }
            if self.keystrokes.len() < Self::KEYSTROKE_COUNT_MAX {
                if close_keystroke_result == CloseKeystrokeResult::Partial
                    && self.close_keystrokes_start.is_none()
                {
                    self.close_keystrokes_start = Some(self.keystrokes.len());
                }
                self.keystrokes.push(keystroke.clone());
                if self.search {
                    self.previous_modifiers = keystroke.modifiers;
                } else if self.keystrokes.len() < Self::KEYSTROKE_COUNT_MAX {
                    self.keystrokes.push(Self::dummy(keystroke.modifiers));
                }
            } else if close_keystroke_result != CloseKeystrokeResult::Partial {
                self.clear_keystrokes(&ClearKeystrokes, window, cx);
            }
        }
        self.keystrokes_changed(cx);
        cx.stop_propagation();
    }

    fn on_inner_focus_in(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.intercept_subscription.is_none() {
            let listener = cx.listener(|this, event: &gpui::KeystrokeEvent, window, cx| {
                this.handle_keystroke(&event.keystroke, window, cx);
            });
            self.intercept_subscription = Some(cx.intercept_keystrokes(listener))
        }
    }

    fn on_inner_focus_out(
        &mut self,
        _event: gpui::FocusOutEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.intercept_subscription.take();
        cx.notify();
    }

    fn render_keystrokes(&self, is_recording: bool) -> impl Iterator<Item = Div> {
        let keystrokes = if let Some(placeholders) = self.placeholder_keystrokes.as_ref()
            && self.keystrokes.is_empty()
        {
            if is_recording {
                &[]
            } else {
                placeholders.as_slice()
            }
        } else {
            &self.keystrokes
        };
        keystrokes.iter().map(move |keystroke| {
            h_flex().children(ui::render_keystroke(
                keystroke,
                Some(Color::Default),
                Some(rems(0.875).into()),
                ui::PlatformStyle::platform(),
                false,
            ))
        })
    }

    pub fn start_recording(
        &mut self,
        _: &StartRecording,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.inner_focus_handle);
        self.clear_keystrokes(&ClearKeystrokes, window, cx);
        self.previous_modifiers = window.modifiers();
        cx.stop_propagation();
    }

    pub fn stop_recording(
        &mut self,
        _: &StopRecording,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.inner_focus_handle.is_focused(window) {
            return;
        }
        window.focus(&self.outer_focus_handle);
        if let Some(close_keystrokes_start) = self.close_keystrokes_start.take()
            && close_keystrokes_start < self.keystrokes.len()
        {
            self.keystrokes.drain(close_keystrokes_start..);
        }
        self.close_keystrokes.take();
        cx.notify();
    }

    pub fn clear_keystrokes(
        &mut self,
        _: &ClearKeystrokes,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.keystrokes.clear();
        self.keystrokes_changed(cx);
    }
}

impl EventEmitter<()> for KeystrokeInput {}

impl Focusable for KeystrokeInput {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.outer_focus_handle.clone()
    }
}

impl Render for KeystrokeInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let is_focused = self.outer_focus_handle.contains_focused(window, cx);
        let is_recording = self.inner_focus_handle.is_focused(window);

        let horizontal_padding = rems_from_px(64.);

        let recording_bg_color = colors
            .editor_background
            .blend(colors.text_accent.opacity(0.1));

        let recording_pulse = |color: Color| {
            Icon::new(IconName::Circle)
                .size(IconSize::Small)
                .color(Color::Error)
                .with_animation(
                    "recording-pulse",
                    Animation::new(std::time::Duration::from_secs(2))
                        .repeat()
                        .with_easing(gpui::pulsating_between(0.4, 0.8)),
                    {
                        let color = color.color(cx);
                        move |this, delta| this.color(Color::Custom(color.opacity(delta)))
                    },
                )
        };

        let recording_indicator = h_flex()
            .h_4()
            .pr_1()
            .gap_0p5()
            .border_1()
            .border_color(colors.border)
            .bg(colors
                .editor_background
                .blend(colors.text_accent.opacity(0.1)))
            .rounded_sm()
            .child(recording_pulse(Color::Error))
            .child(
                Label::new("REC")
                    .size(LabelSize::XSmall)
                    .weight(FontWeight::SEMIBOLD)
                    .color(Color::Error),
            );

        let search_indicator = h_flex()
            .h_4()
            .pr_1()
            .gap_0p5()
            .border_1()
            .border_color(colors.border)
            .bg(colors
                .editor_background
                .blend(colors.text_accent.opacity(0.1)))
            .rounded_sm()
            .child(recording_pulse(Color::Accent))
            .child(
                Label::new("SEARCH")
                    .size(LabelSize::XSmall)
                    .weight(FontWeight::SEMIBOLD)
                    .color(Color::Accent),
            );

        let record_icon = if self.search {
            IconName::MagnifyingGlass
        } else {
            IconName::PlayFilled
        };

        h_flex()
            .id("keystroke-input")
            .track_focus(&self.outer_focus_handle)
            .py_2()
            .px_3()
            .gap_2()
            .min_h_10()
            .w_full()
            .flex_1()
            .justify_between()
            .rounded_lg()
            .overflow_hidden()
            .map(|this| {
                if is_recording {
                    this.bg(recording_bg_color)
                } else {
                    this.bg(colors.editor_background)
                }
            })
            .border_1()
            .border_color(colors.border_variant)
            .when(is_focused, |parent| {
                parent.border_color(colors.border_focused)
            })
            .key_context(Self::key_context())
            .on_action(cx.listener(Self::start_recording))
            .on_action(cx.listener(Self::clear_keystrokes))
            .child(
                h_flex()
                    .w(horizontal_padding)
                    .gap_0p5()
                    .justify_start()
                    .flex_none()
                    .when(is_recording, |this| {
                        this.map(|this| {
                            if self.search {
                                this.child(search_indicator)
                            } else {
                                this.child(recording_indicator)
                            }
                        })
                    }),
            )
            .child(
                h_flex()
                    .id("keystroke-input-inner")
                    .track_focus(&self.inner_focus_handle)
                    .on_modifiers_changed(cx.listener(Self::on_modifiers_changed))
                    .size_full()
                    .when(!self.search, |this| {
                        this.focus(|mut style| {
                            style.border_color = Some(colors.border_focused);
                            style
                        })
                    })
                    .w_full()
                    .min_w_0()
                    .justify_center()
                    .flex_wrap()
                    .gap(ui::DynamicSpacing::Base04.rems(cx))
                    .children(self.render_keystrokes(is_recording)),
            )
            .child(
                h_flex()
                    .w(horizontal_padding)
                    .gap_0p5()
                    .justify_end()
                    .flex_none()
                    .map(|this| {
                        if is_recording {
                            this.child(
                                IconButton::new("stop-record-btn", IconName::StopFilled)
                                    .shape(IconButtonShape::Square)
                                    .map(|this| {
                                        this.tooltip(Tooltip::for_action_title(
                                            if self.search {
                                                "Stop Searching"
                                            } else {
                                                "Stop Recording"
                                            },
                                            &StopRecording,
                                        ))
                                    })
                                    .icon_color(Color::Error)
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        this.stop_recording(&StopRecording, window, cx);
                                    })),
                            )
                        } else {
                            this.child(
                                IconButton::new("record-btn", record_icon)
                                    .shape(IconButtonShape::Square)
                                    .map(|this| {
                                        this.tooltip(Tooltip::for_action_title(
                                            if self.search {
                                                "Start Searching"
                                            } else {
                                                "Start Recording"
                                            },
                                            &StartRecording,
                                        ))
                                    })
                                    .when(!is_focused, |this| this.icon_color(Color::Muted))
                                    .on_click(cx.listener(|this, _event, window, cx| {
                                        this.start_recording(&StartRecording, window, cx);
                                    })),
                            )
                        }
                    })
                    .child(
                        IconButton::new("clear-btn", IconName::Delete)
                            .shape(IconButtonShape::Square)
                            .tooltip(Tooltip::for_action_title(
                                "Clear Keystrokes",
                                &ClearKeystrokes,
                            ))
                            .when(!is_recording || !is_focused, |this| {
                                this.icon_color(Color::Muted)
                            })
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.clear_keystrokes(&ClearKeystrokes, window, cx);
                            })),
                    ),
            )
    }
}
