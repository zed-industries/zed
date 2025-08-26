use gpui::{
    Animation, AnimationExt, Context, EventEmitter, FocusHandle, Focusable, FontWeight, KeyContext,
    Keystroke, Modifiers, ModifiersChangedEvent, Subscription, Task, actions,
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

const KEY_CONTEXT_VALUE: &str = "KeystrokeInput";

const CLOSE_KEYSTROKE_CAPTURE_END_TIMEOUT: std::time::Duration =
    std::time::Duration::from_millis(300);

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
    /// The sequence of close keystrokes being typed
    close_keystrokes: Option<Vec<Keystroke>>,
    close_keystrokes_start: Option<usize>,
    previous_modifiers: Modifiers,
    /// In order to support inputting keystrokes that end with a prefix of the
    /// close keybind keystrokes, we clear the close keystroke capture info
    /// on a timeout after a close keystroke is pressed
    ///
    /// e.g. if close binding is `esc esc esc` and user wants to search for
    /// `ctrl-g esc`, after entering the `ctrl-g esc`, hitting `esc` twice would
    /// stop recording because of the sequence of three escapes making it
    /// impossible to search for anything ending in `esc`
    clear_close_keystrokes_timer: Option<Task<()>>,
    #[cfg(test)]
    recording: bool,
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
            clear_close_keystrokes_timer: None,
            #[cfg(test)]
            recording: false,
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
                .is_some_and(|last| last.key.is_empty())
        {
            return &self.keystrokes[..self.keystrokes.len() - 1];
        }
        &self.keystrokes
    }

    fn dummy(modifiers: Modifiers) -> Keystroke {
        Keystroke {
            modifiers,
            key: "".to_string(),
            key_char: None,
        }
    }

    fn keystrokes_changed(&self, cx: &mut Context<Self>) {
        cx.emit(());
        cx.notify();
    }

    fn key_context() -> KeyContext {
        let mut key_context = KeyContext::default();
        key_context.add(KEY_CONTEXT_VALUE);
        key_context
    }

    fn determine_stop_recording_binding(window: &mut Window) -> Option<gpui::KeyBinding> {
        if cfg!(test) {
            Some(gpui::KeyBinding::new(
                "escape escape escape",
                StopRecording,
                Some(KEY_CONTEXT_VALUE),
            ))
        } else {
            window.highest_precedence_binding_for_action_in_context(
                &StopRecording,
                Self::key_context(),
            )
        }
    }

    fn upsert_close_keystrokes_start(&mut self, start: usize, cx: &mut Context<Self>) {
        if self.close_keystrokes_start.is_some() {
            return;
        }
        self.close_keystrokes_start = Some(start);
        self.update_clear_close_keystrokes_timer(cx);
    }

    fn update_clear_close_keystrokes_timer(&mut self, cx: &mut Context<Self>) {
        self.clear_close_keystrokes_timer = Some(cx.spawn(async |this, cx| {
            cx.background_executor()
                .timer(CLOSE_KEYSTROKE_CAPTURE_END_TIMEOUT)
                .await;
            this.update(cx, |this, _cx| {
                this.end_close_keystrokes_capture();
            })
            .ok();
        }));
    }

    /// Interrupt the capture of close keystrokes, but do not clear the close keystrokes
    /// from the input
    fn end_close_keystrokes_capture(&mut self) -> Option<usize> {
        self.close_keystrokes.take();
        self.clear_close_keystrokes_timer.take();
        self.close_keystrokes_start.take()
    }

    fn handle_possible_close_keystroke(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> CloseKeystrokeResult {
        let Some(keybind_for_close_action) = Self::determine_stop_recording_binding(window) else {
            log::trace!("No keybinding to stop recording keystrokes in keystroke input");
            self.end_close_keystrokes_capture();
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
                    self.end_close_keystrokes_capture();
                    return CloseKeystrokeResult::None;
                }
                if keystroke.should_match(&action_keystrokes[index]) {
                    close_keystrokes.push(keystroke.clone());
                    if close_keystrokes.len() == action_keystrokes.len() {
                        return CloseKeystrokeResult::Close;
                    } else {
                        self.close_keystrokes = Some(close_keystrokes);
                        self.update_clear_close_keystrokes_timer(cx);
                        return CloseKeystrokeResult::Partial;
                    }
                } else {
                    self.end_close_keystrokes_capture();
                    return CloseKeystrokeResult::None;
                }
            }
        } else if let Some(first_action_keystroke) = action_keystrokes.first()
            && keystroke.should_match(first_action_keystroke)
        {
            self.close_keystrokes = Some(vec![keystroke.clone()]);
            return CloseKeystrokeResult::Partial;
        }
        self.end_close_keystrokes_capture();
        CloseKeystrokeResult::None
    }

    fn on_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        let keystrokes_len = self.keystrokes.len();

        if self.previous_modifiers.modified()
            && event.modifiers.is_subset_of(&self.previous_modifiers)
        {
            self.previous_modifiers &= event.modifiers;
            return;
        }
        self.keystrokes_changed(cx);

        if let Some(last) = self.keystrokes.last_mut()
            && last.key.is_empty()
            && keystrokes_len <= Self::KEYSTROKE_COUNT_MAX
        {
            if !self.search && !event.modifiers.modified() {
                self.keystrokes.pop();
                return;
            }
            if self.search {
                if self.previous_modifiers.modified() {
                    last.modifiers |= event.modifiers;
                } else {
                    self.keystrokes.push(Self::dummy(event.modifiers));
                }
                self.previous_modifiers |= event.modifiers;
            } else {
                last.modifiers = event.modifiers;
                return;
            }
        } else if keystrokes_len < Self::KEYSTROKE_COUNT_MAX {
            self.keystrokes.push(Self::dummy(event.modifiers));
            if self.search {
                self.previous_modifiers |= event.modifiers;
            }
        }
        if keystrokes_len >= Self::KEYSTROKE_COUNT_MAX {
            self.clear_keystrokes(&ClearKeystrokes, window, cx);
        }
    }

    fn handle_keystroke(
        &mut self,
        keystroke: &Keystroke,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();

        let close_keystroke_result = self.handle_possible_close_keystroke(keystroke, window, cx);
        if close_keystroke_result == CloseKeystrokeResult::Close {
            self.stop_recording(&StopRecording, window, cx);
            return;
        }

        let mut keystroke = keystroke.clone();
        if let Some(last) = self.keystrokes.last()
            && last.key.is_empty()
            && (!self.search || self.previous_modifiers.modified())
        {
            let key = keystroke.key.clone();
            keystroke = last.clone();
            keystroke.key = key;
            self.keystrokes.pop();
        }

        if close_keystroke_result == CloseKeystrokeResult::Partial {
            self.upsert_close_keystrokes_start(self.keystrokes.len(), cx);
            if self.keystrokes.len() >= Self::KEYSTROKE_COUNT_MAX {
                return;
            }
        }

        if self.keystrokes.len() >= Self::KEYSTROKE_COUNT_MAX {
            self.clear_keystrokes(&ClearKeystrokes, window, cx);
            return;
        }

        self.keystrokes.push(keystroke.clone());
        self.keystrokes_changed(cx);

        if self.search {
            self.previous_modifiers = keystroke.modifiers;
            return;
        }
        if self.keystrokes.len() < Self::KEYSTROKE_COUNT_MAX && keystroke.modifiers.modified() {
            self.keystrokes.push(Self::dummy(keystroke.modifiers));
        }
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
        #[cfg(test)]
        {
            self.recording = true;
        }
        cx.stop_propagation();
    }

    pub fn stop_recording(
        &mut self,
        _: &StopRecording,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_recording(window) {
            return;
        }
        window.focus(&self.outer_focus_handle);
        if let Some(close_keystrokes_start) = self.close_keystrokes_start.take()
            && close_keystrokes_start < self.keystrokes.len()
        {
            self.keystrokes.drain(close_keystrokes_start..);
            self.keystrokes_changed(cx);
        }
        self.end_close_keystrokes_capture();
        #[cfg(test)]
        {
            self.recording = false;
        }
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
        self.end_close_keystrokes_capture();
    }

    fn is_recording(&self, window: &Window) -> bool {
        #[cfg(test)]
        {
            if true {
                // in tests, we just need a simple bool that is toggled on start and stop recording
                return self.recording;
            }
        }
        // however, in the real world, checking if the inner focus handle is focused
        // is a much more reliable check, as the intercept keystroke handlers are installed
        // on focus of the inner focus handle, thereby ensuring our recording state does
        // not get de-synced
        self.inner_focus_handle.is_focused(window)
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
        let is_recording = self.is_recording(window);

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
            .rounded_sm()
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
                                IconButton::new("stop-record-btn", IconName::Stop)
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
                        IconButton::new("clear-btn", IconName::Backspace)
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use itertools::Itertools as _;
    use project::Project;
    use settings::SettingsStore;
    use workspace::Workspace;

    pub struct KeystrokeInputTestHelper {
        input: Entity<KeystrokeInput>,
        current_modifiers: Modifiers,
        cx: VisualTestContext,
    }

    impl KeystrokeInputTestHelper {
        /// Creates a new test helper with default settings
        pub fn new(mut cx: VisualTestContext) -> Self {
            let input = cx.new_window_entity(|window, cx| KeystrokeInput::new(None, window, cx));

            let mut helper = Self {
                input,
                current_modifiers: Modifiers::default(),
                cx,
            };

            helper.start_recording();
            helper
        }

        /// Sets search mode on the input
        pub fn with_search_mode(&mut self, search: bool) -> &mut Self {
            self.input.update(&mut self.cx, |input, _| {
                input.set_search(search);
            });
            self
        }

        /// Sends a keystroke event based on string description
        /// Examples: "a", "ctrl-a", "cmd-shift-z", "escape"
        #[track_caller]
        pub fn send_keystroke(&mut self, keystroke_input: &str) -> &mut Self {
            self.expect_is_recording(true);
            let keystroke_str = if keystroke_input.ends_with('-') {
                format!("{}_", keystroke_input)
            } else {
                keystroke_input.to_string()
            };

            let mut keystroke = Keystroke::parse(&keystroke_str)
                .unwrap_or_else(|_| panic!("Invalid keystroke: {}", keystroke_input));

            // Remove the dummy key if we added it for modifier-only keystrokes
            if keystroke_input.ends_with('-') && keystroke_str.ends_with("_") {
                keystroke.key = "".to_string();
            }

            // Combine current modifiers with keystroke modifiers
            keystroke.modifiers |= self.current_modifiers;

            self.update_input(|input, window, cx| {
                input.handle_keystroke(&keystroke, window, cx);
            });

            // Don't update current_modifiers for keystrokes with actual keys
            if keystroke.key.is_empty() {
                self.current_modifiers = keystroke.modifiers;
            }
            self
        }

        /// Sends a modifier change event based on string description
        /// Examples: "+ctrl", "-ctrl", "+cmd+shift", "-all"
        #[track_caller]
        pub fn send_modifiers(&mut self, modifiers: &str) -> &mut Self {
            self.expect_is_recording(true);
            let new_modifiers = if modifiers == "-all" {
                Modifiers::default()
            } else {
                self.parse_modifier_change(modifiers)
            };

            let event = ModifiersChangedEvent {
                modifiers: new_modifiers,
                capslock: gpui::Capslock::default(),
            };

            self.update_input(|input, window, cx| {
                input.on_modifiers_changed(&event, window, cx);
            });

            self.current_modifiers = new_modifiers;
            self
        }

        /// Sends multiple events in sequence
        /// Each event string is either a keystroke or modifier change
        #[track_caller]
        pub fn send_events(&mut self, events: &[&str]) -> &mut Self {
            self.expect_is_recording(true);
            for event in events {
                if event.starts_with('+') || event.starts_with('-') {
                    self.send_modifiers(event);
                } else {
                    self.send_keystroke(event);
                }
            }
            self
        }

        #[track_caller]
        fn expect_keystrokes_equal(actual: &[Keystroke], expected: &[&str]) {
            let expected_keystrokes: Result<Vec<Keystroke>, _> = expected
                .iter()
                .map(|s| {
                    let keystroke_str = if s.ends_with('-') {
                        format!("{}_", s)
                    } else {
                        s.to_string()
                    };

                    let mut keystroke = Keystroke::parse(&keystroke_str)?;

                    // Remove the dummy key if we added it for modifier-only keystrokes
                    if s.ends_with('-') && keystroke_str.ends_with("_") {
                        keystroke.key = "".to_string();
                    }

                    Ok(keystroke)
                })
                .collect();

            let expected_keystrokes = expected_keystrokes
                .unwrap_or_else(|e: anyhow::Error| panic!("Invalid expected keystroke: {}", e));

            assert_eq!(
                actual.len(),
                expected_keystrokes.len(),
                "Keystroke count mismatch. Expected: {:?}, Actual: {:?}",
                expected_keystrokes
                    .iter()
                    .map(|k| k.unparse())
                    .collect::<Vec<_>>(),
                actual.iter().map(|k| k.unparse()).collect::<Vec<_>>()
            );

            for (i, (actual, expected)) in actual.iter().zip(expected_keystrokes.iter()).enumerate()
            {
                assert_eq!(
                    actual.unparse(),
                    expected.unparse(),
                    "Keystroke {} mismatch. Expected: '{}', Actual: '{}'",
                    i,
                    expected.unparse(),
                    actual.unparse()
                );
            }
        }

        /// Verifies that the keystrokes match the expected strings
        #[track_caller]
        pub fn expect_keystrokes(&mut self, expected: &[&str]) -> &mut Self {
            let actual = self
                .input
                .read_with(&self.cx, |input, _| input.keystrokes.clone());
            Self::expect_keystrokes_equal(&actual, expected);
            self
        }

        #[track_caller]
        pub fn expect_close_keystrokes(&mut self, expected: &[&str]) -> &mut Self {
            let actual = self
                .input
                .read_with(&self.cx, |input, _| input.close_keystrokes.clone())
                .unwrap_or_default();
            Self::expect_keystrokes_equal(&actual, expected);
            self
        }

        /// Verifies that there are no keystrokes
        #[track_caller]
        pub fn expect_empty(&mut self) -> &mut Self {
            self.expect_keystrokes(&[])
        }

        /// Starts recording keystrokes
        #[track_caller]
        pub fn start_recording(&mut self) -> &mut Self {
            self.expect_is_recording(false);
            self.input.update_in(&mut self.cx, |input, window, cx| {
                input.start_recording(&StartRecording, window, cx);
            });
            self
        }

        /// Stops recording keystrokes
        pub fn stop_recording(&mut self) -> &mut Self {
            self.expect_is_recording(true);
            self.input.update_in(&mut self.cx, |input, window, cx| {
                input.stop_recording(&StopRecording, window, cx);
            });
            self
        }

        /// Clears all keystrokes
        #[track_caller]
        pub fn clear_keystrokes(&mut self) -> &mut Self {
            let change_tracker = KeystrokeUpdateTracker::new(self.input.clone(), &mut self.cx);
            self.input.update_in(&mut self.cx, |input, window, cx| {
                input.clear_keystrokes(&ClearKeystrokes, window, cx);
            });
            KeystrokeUpdateTracker::finish(change_tracker, &self.cx);
            self.current_modifiers = Default::default();
            self
        }

        /// Verifies the recording state
        #[track_caller]
        pub fn expect_is_recording(&mut self, expected: bool) -> &mut Self {
            let actual = self
                .input
                .update_in(&mut self.cx, |input, window, _| input.is_recording(window));
            assert_eq!(
                actual, expected,
                "Recording state mismatch. Expected: {}, Actual: {}",
                expected, actual
            );
            self
        }

        pub async fn wait_for_close_keystroke_capture_end(&mut self) -> &mut Self {
            let task = self.input.update_in(&mut self.cx, |input, _, _| {
                input.clear_close_keystrokes_timer.take()
            });
            let task = task.expect("No close keystroke capture end timer task");
            self.cx
                .executor()
                .advance_clock(CLOSE_KEYSTROKE_CAPTURE_END_TIMEOUT);
            task.await;
            self
        }

        /// Parses modifier change strings like "+ctrl", "-shift", "+cmd+alt"
        #[track_caller]
        fn parse_modifier_change(&self, modifiers_str: &str) -> Modifiers {
            let mut modifiers = self.current_modifiers;

            assert!(!modifiers_str.is_empty(), "Empty modifier string");

            let value;
            let split_char;
            let remaining;
            if let Some(to_add) = modifiers_str.strip_prefix('+') {
                value = true;
                split_char = '+';
                remaining = to_add;
            } else {
                let to_remove = modifiers_str
                    .strip_prefix('-')
                    .expect("Modifier string must start with '+' or '-'");
                value = false;
                split_char = '-';
                remaining = to_remove;
            }

            for modifier in remaining.split(split_char) {
                match modifier {
                    "ctrl" | "control" => modifiers.control = value,
                    "alt" | "option" => modifiers.alt = value,
                    "shift" => modifiers.shift = value,
                    "cmd" | "command" | "platform" => modifiers.platform = value,
                    "fn" | "function" => modifiers.function = value,
                    _ => panic!("Unknown modifier: {}", modifier),
                }
            }

            modifiers
        }

        #[track_caller]
        fn update_input<R>(
            &mut self,
            cb: impl FnOnce(&mut KeystrokeInput, &mut Window, &mut Context<KeystrokeInput>) -> R,
        ) -> R {
            let change_tracker = KeystrokeUpdateTracker::new(self.input.clone(), &mut self.cx);
            let result = self.input.update_in(&mut self.cx, cb);
            KeystrokeUpdateTracker::finish(change_tracker, &self.cx);
            result
        }
    }

    struct KeystrokeUpdateTracker {
        initial_keystrokes: Vec<Keystroke>,
        _subscription: Subscription,
        input: Entity<KeystrokeInput>,
        received_keystrokes_updated: bool,
    }

    impl KeystrokeUpdateTracker {
        fn new(input: Entity<KeystrokeInput>, cx: &mut VisualTestContext) -> Entity<Self> {
            cx.new(|cx| Self {
                initial_keystrokes: input.read_with(cx, |input, _| input.keystrokes.clone()),
                _subscription: cx.subscribe(&input, |this: &mut Self, _, _, _| {
                    this.received_keystrokes_updated = true;
                }),
                input,
                received_keystrokes_updated: false,
            })
        }
        #[track_caller]
        fn finish(this: Entity<Self>, cx: &VisualTestContext) {
            let (received_keystrokes_updated, initial_keystrokes_str, updated_keystrokes_str) =
                this.read_with(cx, |this, cx| {
                    let updated_keystrokes = this
                        .input
                        .read_with(cx, |input, _| input.keystrokes.clone());
                    let initial_keystrokes_str = keystrokes_str(&this.initial_keystrokes);
                    let updated_keystrokes_str = keystrokes_str(&updated_keystrokes);
                    (
                        this.received_keystrokes_updated,
                        initial_keystrokes_str,
                        updated_keystrokes_str,
                    )
                });
            if received_keystrokes_updated {
                assert_ne!(
                    initial_keystrokes_str, updated_keystrokes_str,
                    "Received keystrokes_updated event, expected different keystrokes"
                );
            } else {
                assert_eq!(
                    initial_keystrokes_str, updated_keystrokes_str,
                    "Received no keystrokes_updated event, expected same keystrokes"
                );
            }

            fn keystrokes_str(ks: &[Keystroke]) -> String {
                ks.iter().map(|ks| ks.unparse()).join(" ")
            }
        }
    }

    async fn init_test(cx: &mut TestAppContext) -> KeystrokeInputTestHelper {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            project::Project::init_settings(cx);
            workspace::init_settings(cx);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let workspace =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let cx = VisualTestContext::from_window(*workspace, cx);
        KeystrokeInputTestHelper::new(cx)
    }

    #[gpui::test]
    async fn test_basic_keystroke_input(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .send_keystroke("a")
            .clear_keystrokes()
            .expect_empty();
    }

    #[gpui::test]
    async fn test_modifier_handling(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "a", "-ctrl"])
            .expect_keystrokes(&["ctrl-a"]);
    }

    #[gpui::test]
    async fn test_multiple_modifiers(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .send_keystroke("cmd-shift-z")
            .expect_keystrokes(&["cmd-shift-z", "cmd-shift-"]);
    }

    #[gpui::test]
    async fn test_search_mode_behavior(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+cmd", "shift-f", "-cmd"])
            // In search mode, when completing a modifier-only keystroke with a key,
            // only the original modifiers are preserved, not the keystroke's modifiers
            .expect_keystrokes(&["cmd-f"]);
    }

    #[gpui::test]
    async fn test_keystroke_limit(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .send_keystroke("a")
            .send_keystroke("b")
            .send_keystroke("c")
            .expect_keystrokes(&["a", "b", "c"]) // At max limit
            .send_keystroke("d")
            .expect_empty(); // Should clear when exceeding limit
    }

    #[gpui::test]
    async fn test_modifier_release_all(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl+shift", "a", "-all"])
            .expect_keystrokes(&["ctrl-shift-a"]);
    }

    #[gpui::test]
    async fn test_search_new_modifiers_not_added_until_all_released(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl+shift", "a", "-ctrl"])
            .expect_keystrokes(&["ctrl-shift-a"])
            .send_events(&["+ctrl"])
            .expect_keystrokes(&["ctrl-shift-a", "ctrl-shift-"]);
    }

    #[gpui::test]
    async fn test_previous_modifiers_no_effect_when_not_search(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(false)
            .send_events(&["+ctrl+shift", "a", "-all"])
            .expect_keystrokes(&["ctrl-shift-a"]);
    }

    #[gpui::test]
    async fn test_keystroke_limit_overflow_non_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(false)
            .send_events(&["a", "b", "c", "d"]) // 4 keystrokes, exceeds limit of 3
            .expect_empty(); // Should clear when exceeding limit
    }

    #[gpui::test]
    async fn test_complex_modifier_sequences(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "+shift", "+alt", "a", "-ctrl", "-shift", "-alt"])
            .expect_keystrokes(&["ctrl-shift-alt-a"]);
    }

    #[gpui::test]
    async fn test_modifier_only_keystrokes_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "+shift", "-ctrl", "-shift"])
            .expect_keystrokes(&["ctrl-shift-"]); // Modifier-only sequences create modifier-only keystrokes
    }

    #[gpui::test]
    async fn test_modifier_only_keystrokes_non_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(false)
            .send_events(&["+ctrl", "+shift", "-ctrl", "-shift"])
            .expect_empty(); // Modifier-only sequences get filtered in non-search mode
    }

    #[gpui::test]
    async fn test_rapid_modifier_changes(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "-ctrl", "+shift", "-shift", "+alt", "a", "-alt"])
            .expect_keystrokes(&["ctrl-", "shift-", "alt-a"]);
    }

    #[gpui::test]
    async fn test_clear_keystrokes_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "a", "-ctrl", "b"])
            .expect_keystrokes(&["ctrl-a", "b"])
            .clear_keystrokes()
            .expect_empty();
    }

    #[gpui::test]
    async fn test_non_search_mode_modifier_key_sequence(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(false)
            .send_events(&["+ctrl", "a"])
            .expect_keystrokes(&["ctrl-a", "ctrl-"])
            .send_events(&["-ctrl"])
            .expect_keystrokes(&["ctrl-a"]); // Non-search mode filters trailing empty keystrokes
    }

    #[gpui::test]
    async fn test_all_modifiers_at_once(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl+shift+alt+cmd", "a", "-all"])
            .expect_keystrokes(&["ctrl-shift-alt-cmd-a"]);
    }

    #[gpui::test]
    async fn test_keystrokes_at_exact_limit(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["a", "b", "c"]) // exactly 3 keystrokes (at limit)
            .expect_keystrokes(&["a", "b", "c"])
            .send_events(&["d"]) // should clear when exceeding
            .expect_empty();
    }

    #[gpui::test]
    async fn test_function_modifier_key(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+fn", "f1", "-fn"])
            .expect_keystrokes(&["fn-f1"]);
    }

    #[gpui::test]
    async fn test_start_stop_recording(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .send_events(&["a", "b"])
            .expect_keystrokes(&["a", "b"]) // start_recording clears existing keystrokes
            .stop_recording()
            .expect_is_recording(false)
            .start_recording()
            .send_events(&["c"])
            .expect_keystrokes(&["c"]);
    }

    #[gpui::test]
    async fn test_modifier_sequence_with_interruption(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "+shift", "a", "-shift", "b", "-ctrl"])
            .expect_keystrokes(&["ctrl-shift-a", "ctrl-b"]);
    }

    #[gpui::test]
    async fn test_empty_key_sequence_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&[]) // No events at all
            .expect_empty();
    }

    #[gpui::test]
    async fn test_modifier_sequence_completion_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "+shift", "-shift", "a", "-ctrl"])
            .expect_keystrokes(&["ctrl-shift-a"]);
    }

    #[gpui::test]
    async fn test_triple_escape_stops_recording_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["a", "escape", "escape", "escape"])
            .expect_keystrokes(&["a"]) // Triple escape removes final escape, stops recording
            .expect_is_recording(false);
    }

    #[gpui::test]
    async fn test_triple_escape_stops_recording_non_search_mode(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(false)
            .send_events(&["a", "escape", "escape", "escape"])
            .expect_keystrokes(&["a"]); // Triple escape stops recording but only removes final escape
    }

    #[gpui::test]
    async fn test_triple_escape_at_keystroke_limit(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["a", "b", "c", "escape", "escape", "escape"]) // 6 keystrokes total, exceeds limit
            .expect_keystrokes(&["a", "b", "c"]); // Triple escape stops recording and removes escapes, leaves original keystrokes
    }

    #[gpui::test]
    async fn test_interrupted_escape_sequence(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["escape", "escape", "a", "escape"]) // Partial escape sequence interrupted by 'a'
            .expect_keystrokes(&["escape", "escape", "a"]); // Escape sequence interrupted by 'a', no close triggered
    }

    #[gpui::test]
    async fn test_interrupted_escape_sequence_within_limit(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["escape", "escape", "a"]) // Partial escape sequence interrupted by 'a' (3 keystrokes, at limit)
            .expect_keystrokes(&["escape", "escape", "a"]); // Should not trigger close, interruption resets escape detection
    }

    #[gpui::test]
    async fn test_partial_escape_sequence_no_close(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["escape", "escape"]) // Only 2 escapes, not enough to close
            .expect_keystrokes(&["escape", "escape"])
            .expect_is_recording(true); // Should remain in keystrokes, no close triggered
    }

    #[gpui::test]
    async fn test_recording_state_after_triple_escape(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["a", "escape", "escape", "escape"])
            .expect_keystrokes(&["a"]) // Triple escape stops recording, removes final escape
            .expect_is_recording(false);
    }

    #[gpui::test]
    async fn test_triple_escape_mixed_with_other_keystrokes(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["a", "escape", "b", "escape", "escape"]) // Mixed sequence, should not trigger close
            .expect_keystrokes(&["a", "escape", "b"]); // No complete triple escape sequence, stays at limit
    }

    #[gpui::test]
    async fn test_triple_escape_only(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["escape", "escape", "escape"]) // Pure triple escape sequence
            .expect_empty();
    }

    #[gpui::test]
    async fn test_end_close_keystroke_capture(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .send_events(&["+ctrl", "g", "-ctrl", "escape"])
            .expect_keystrokes(&["ctrl-g", "escape"])
            .wait_for_close_keystroke_capture_end()
            .await
            .send_events(&["escape", "escape"])
            .expect_keystrokes(&["ctrl-g", "escape", "escape"])
            .expect_close_keystrokes(&["escape", "escape"])
            .send_keystroke("escape")
            .expect_keystrokes(&["ctrl-g", "escape"]);
    }

    #[gpui::test]
    async fn test_search_previous_modifiers_are_sticky(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl+alt", "-ctrl", "j"])
            .expect_keystrokes(&["ctrl-alt-j"]);
    }

    #[gpui::test]
    async fn test_previous_modifiers_can_be_entered_separately(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "-ctrl"])
            .expect_keystrokes(&["ctrl-"])
            .send_events(&["+alt", "-alt"])
            .expect_keystrokes(&["ctrl-", "alt-"]);
    }

    #[gpui::test]
    async fn test_previous_modifiers_reset_on_key(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl+alt", "-ctrl", "+shift"])
            .expect_keystrokes(&["ctrl-shift-alt-"])
            .send_keystroke("j")
            .expect_keystrokes(&["ctrl-shift-alt-j"])
            .send_keystroke("i")
            .expect_keystrokes(&["ctrl-shift-alt-j", "shift-alt-i"])
            .send_events(&["-shift-alt", "+cmd"])
            .expect_keystrokes(&["ctrl-shift-alt-j", "shift-alt-i", "cmd-"]);
    }

    #[gpui::test]
    async fn test_previous_modifiers_reset_on_release_all(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl+alt", "-ctrl", "+shift"])
            .expect_keystrokes(&["ctrl-shift-alt-"])
            .send_events(&["-all", "j"])
            .expect_keystrokes(&["ctrl-shift-alt-", "j"]);
    }

    #[gpui::test]
    async fn test_search_repeat_modifiers(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(true)
            .send_events(&["+ctrl", "-ctrl", "+alt", "-alt", "+shift", "-shift"])
            .expect_keystrokes(&["ctrl-", "alt-", "shift-"])
            .send_events(&["+cmd"])
            .expect_empty();
    }

    #[gpui::test]
    async fn test_not_search_repeat_modifiers(cx: &mut TestAppContext) {
        init_test(cx)
            .await
            .with_search_mode(false)
            .send_events(&["+ctrl", "-ctrl", "+alt", "-alt", "+shift", "-shift"])
            .expect_empty();
    }
}
