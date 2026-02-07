use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Render, Size, Window, WindowBounds, WindowKind,
    WindowOptions, prelude::*, px,
};
use platform_title_bar::PlatformTitleBar;
use release_channel::ReleaseChannel;
use ui::{Button, ButtonStyle, Label, prelude::*};
use util::ResultExt;
use workspace::client_side_decorations;

use super::audio_input_output_setup::{
    AudioDeviceKind, SYSTEM_DEFAULT, render_audio_device_dropdown,
};

pub struct AudioTestWindow {
    title_bar: Option<Entity<PlatformTitleBar>>,
    testing: bool,
    input_device_id: String,
    output_device_id: String,
    focus_handle: FocusHandle,
}

impl AudioTestWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let title_bar = if !cfg!(target_os = "macos") {
            Some(cx.new(|cx| PlatformTitleBar::new("audio-test-title-bar", cx)))
        } else {
            None
        };

        Self {
            title_bar,
            testing: false,
            input_device_id: SYSTEM_DEFAULT.to_string(),
            output_device_id: SYSTEM_DEFAULT.to_string(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn toggle_testing(&mut self, cx: &mut Context<Self>) {
        self.testing = !self.testing;
        if self.testing {
            // TODO: Start audio routing (microphone → speaker)
        } else {
            // TODO: Stop audio routing
        }
        cx.notify();
    }

    fn set_input_device(&mut self, device_id: Option<String>, cx: &mut Context<Self>) {
        self.input_device_id = device_id.unwrap_or_else(|| SYSTEM_DEFAULT.to_string());
        cx.notify();
    }

    fn set_output_device(&mut self, device_id: Option<String>, cx: &mut Context<Self>) {
        self.output_device_id = device_id.unwrap_or_else(|| SYSTEM_DEFAULT.to_string());
        cx.notify();
    }
}

impl Render for AudioTestWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let button_text = if self.testing {
            "Stop Testing"
        } else {
            "Start Testing"
        };

        let button_style = if self.testing {
            ButtonStyle::Tinted(ui::TintColor::Error)
        } else {
            ButtonStyle::Filled
        };

        let weak_entity = cx.entity().downgrade();
        let input_dropdown = {
            let weak_entity = weak_entity.clone();
            render_audio_device_dropdown(
                "audio-test-input-dropdown",
                AudioDeviceKind::Input,
                self.input_device_id.clone(),
                move |device_id, _, cx| {
                    weak_entity
                        .update(cx, |this, cx| this.set_input_device(device_id, cx))
                        .log_err();
                },
                window,
                cx,
            )
        };

        let output_dropdown = render_audio_device_dropdown(
            "audio-test-output-dropdown",
            AudioDeviceKind::Output,
            self.output_device_id.clone(),
            move |device_id, _, cx| {
                weak_entity
                    .update(cx, |this, cx| this.set_output_device(device_id, cx))
                    .log_err();
            },
            window,
            cx,
        );

        let content = v_flex()
            .id("audio-test-window")
            .track_focus(&self.focus_handle)
            .size_full()
            .p_4()
            .gap_4()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Input Device"))
                    .child(input_dropdown),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Output Device"))
                    .child(output_dropdown),
            )
            .child(
                h_flex().w_full().justify_center().pt_4().child(
                    Button::new("test-audio-toggle", button_text)
                        .style(button_style)
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_testing(cx))),
                ),
            );

        client_side_decorations(
            v_flex()
                .size_full()
                .text_color(cx.theme().colors().text)
                .children(self.title_bar.clone())
                .child(content),
            window,
            cx,
        )
    }
}

impl Focusable for AudioTestWindow {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Drop for AudioTestWindow {
    fn drop(&mut self) {
        if self.testing {
            // TODO: Clean up audio streams
        }
    }
}

pub fn open_audio_test_window(_window: &mut Window, cx: &mut App) {
    let existing = cx
        .windows()
        .into_iter()
        .find_map(|w| w.downcast::<AudioTestWindow>());

    if let Some(existing) = existing {
        existing
            .update(cx, |_, window, _| window.activate_window())
            .log_err();
        return;
    }

    let app_id = ReleaseChannel::global(cx).app_id();
    let window_size = Size {
        width: px(400.0),
        height: px(240.0),
    };

    cx.open_window(
        WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Audio Test".into()),
                appears_transparent: true,
                traffic_light_position: Some(gpui::point(px(12.0), px(12.0))),
            }),
            focus: true,
            show: true,
            is_movable: true,
            kind: WindowKind::Normal,
            window_background: cx.theme().window_background_appearance(),
            app_id: Some(app_id.to_owned()),
            window_decorations: Some(gpui::WindowDecorations::Client),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            window_min_size: Some(window_size),
            ..Default::default()
        },
        |_, cx| cx.new(AudioTestWindow::new),
    )
    .log_err();
}
