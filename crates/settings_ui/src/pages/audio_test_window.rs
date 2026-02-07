use audio::{AudioSettings, CHANNEL_COUNT, RodioExt, SAMPLE_RATE};
use cpal::{
    DeviceId, default_host,
    traits::{DeviceTrait, HostTrait},
};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Render, Size, Window, WindowBounds, WindowKind,
    WindowOptions, prelude::*, px,
};
use log::info;
use platform_title_bar::PlatformTitleBar;
use release_channel::ReleaseChannel;
use rodio::{DeviceSinkBuilder, Source};
use settings::{AudioDeviceName, Settings};
use std::{
    any::Any,
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use ui::{Button, ButtonStyle, Label, prelude::*};
use util::ResultExt;
use workspace::client_side_decorations;

use super::audio_input_output_setup::{
    AudioDeviceKind, SYSTEM_DEFAULT, render_audio_device_dropdown,
};
use crate::{SettingsUiFile, update_settings_file};

pub struct AudioTestWindow {
    title_bar: Option<Entity<PlatformTitleBar>>,
    input_device_id: String,
    output_device_id: String,
    focus_handle: FocusHandle,
    _stop_playback: Option<Box<dyn Any + Send>>,
}

impl AudioTestWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let title_bar = if !cfg!(target_os = "macos") {
            Some(cx.new(|cx| PlatformTitleBar::new("audio-test-title-bar", cx)))
        } else {
            None
        };

        let audio_settings = AudioSettings::get_global(cx);
        let input_device_id = audio_settings
            .input_audio_device
            .clone()
            .unwrap_or_else(|| SYSTEM_DEFAULT.to_string());
        let output_device_id = audio_settings
            .output_audio_device
            .clone()
            .unwrap_or_else(|| SYSTEM_DEFAULT.to_string());

        Self {
            title_bar,
            input_device_id,
            output_device_id,
            focus_handle: cx.focus_handle(),
            _stop_playback: None,
        }
    }

    fn toggle_testing(&mut self, cx: &mut Context<Self>) {
        if let Some(_cb) = self._stop_playback.take() {
            cx.notify();
            return;
        }

        if let Some(cb) =
            start_test_playback(self.input_device_id.clone(), self.output_device_id.clone()).ok()
        {
            self._stop_playback = Some(cb);
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

fn start_test_playback(
    input_device_id: String,
    output_device_id: String,
) -> anyhow::Result<Box<dyn Any + Send>> {
    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_for_mic = stop_signal.clone();
    let stop_signal_for_defer = stop_signal.clone();

    thread::Builder::new()
        .name("AudioTestPlayback".to_string())
        .spawn(move || {
            log::info!(
                "Audio test: output_device_id string = {:?}",
                output_device_id
            );
            let output_device_id = DeviceId::from_str(&output_device_id).ok();
            log::info!(
                "Audio test: parsed output DeviceId = {:?}",
                output_device_id
            );
            let output = if let Some(ref id) = output_device_id {
                log::info!("Audio test: looking for device by id: {id}");
                if let Some(device) = default_host().device_by_id(id) {
                    if let Ok(desc) = device.description() {
                        log::info!(
                            "Audio test: found device - name: {:?}, manufacturer: {:?}",
                            desc.name(),
                            desc.manufacturer()
                        );
                    } else {
                        log::info!("Audio test: found device (no description available)");
                    }
                    DeviceSinkBuilder::from_device(device).and_then(|builder| {
                        log::info!("Audio test: opening stream on specific device");
                        builder.open_stream()
                    })
                } else {
                    log::warn!("Audio test: device_by_id returned None, falling back to default");
                    DeviceSinkBuilder::open_default_sink()
                }
            } else {
                log::info!("Audio test: using default sink (no device id specified)");
                DeviceSinkBuilder::open_default_sink()
            };
            let Ok(output) = output else {
                log::error!("Could not open output device for audio test");
                return;
            };
            log::info!("Audio test: output device opened successfully");

            let input_device_id = DeviceId::from_str(&input_device_id).ok();
            let microphone = match open_test_microphone(input_device_id, stop_signal_for_mic) {
                Ok(mic) => mic,
                Err(e) => {
                    log::error!("Could not open microphone for audio test: {e}");
                    return;
                }
            };

            output.mixer().add(microphone);

            // Keep thread (and output device) alive until stop signal
            while !stop_signal.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
            }
        })?;

    Ok(Box::new(util::defer(move || {
        stop_signal_for_defer.store(true, Ordering::Relaxed);
    })))
}

fn open_test_microphone(
    input_device_id: Option<DeviceId>,
    stop_signal: Arc<AtomicBool>,
) -> anyhow::Result<impl Source<Item = f32>> {
    let builder = rodio::microphone::MicrophoneBuilder::new();
    let builder = if let Some(id) = input_device_id {
        let mut found = None;
        for input in rodio::microphone::available_inputs()? {
            if input.clone().into_inner().id()? == id {
                found = Some(builder.device(input));
                break;
            }
        }
        found.unwrap_or_else(|| builder.default_device())?
    } else {
        builder.default_device()?
    };

    let stream = builder
        .default_config()?
        .prefer_sample_rates([
            SAMPLE_RATE,
            SAMPLE_RATE.saturating_mul(rodio::nz!(2)),
            SAMPLE_RATE.saturating_mul(rodio::nz!(3)),
            SAMPLE_RATE.saturating_mul(rodio::nz!(4)),
        ])
        .prefer_channel_counts([rodio::nz!(1), rodio::nz!(2), rodio::nz!(3), rodio::nz!(4)])
        .prefer_buffer_sizes(512..)
        .open_stream()?;
    info!("Opened test microphone: {:?}", stream.config());

    let stream = stream
        .possibly_disconnected_channels_to_mono()
        .constant_samplerate(SAMPLE_RATE)
        .constant_params(CHANNEL_COUNT, SAMPLE_RATE)
        .stoppable()
        .periodic_access(
            Duration::from_millis(50),
            move |stoppable: &mut rodio::source::Stoppable<_>| {
                if stop_signal.load(Ordering::Relaxed) {
                    stoppable.stop();
                }
            },
        );

    Ok(stream)
}

impl Render for AudioTestWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_testing = self._stop_playback.is_some();
        let button_text = if is_testing {
            "Stop Testing"
        } else {
            "Start Testing"
        };

        let button_style = if is_testing {
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
                move |device_id, window, cx| {
                    weak_entity
                        .update(cx, |this, cx| this.set_input_device(device_id.clone(), cx))
                        .log_err();
                    let value: Option<Option<AudioDeviceName>> =
                        device_id.map(|id| Some(AudioDeviceName(id)));
                    update_settings_file(
                        SettingsUiFile::User,
                        Some("audio.experimental.input_audio_device"),
                        window,
                        cx,
                        move |settings, _cx| {
                            settings.audio.get_or_insert_default().input_audio_device =
                                value.clone().flatten();
                        },
                    )
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
            move |device_id, window, cx| {
                weak_entity
                    .update(cx, |this, cx| this.set_output_device(device_id.clone(), cx))
                    .log_err();
                let value: Option<Option<AudioDeviceName>> =
                    device_id.map(|id| Some(AudioDeviceName(id)));
                update_settings_file(
                    SettingsUiFile::User,
                    Some("audio.experimental.output_audio_device"),
                    window,
                    cx,
                    move |settings, _cx| {
                        settings.audio.get_or_insert_default().output_audio_device =
                            value.clone().flatten();
                    },
                )
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
                    .child(Label::new("Output Device"))
                    .child(output_dropdown),
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Input Device"))
                    .child(input_dropdown),
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
        let _ = self._stop_playback.take();
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
