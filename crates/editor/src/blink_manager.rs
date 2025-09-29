//! Cursor blink animation system for the editor.

use crate::EditorSettings;
use gpui::Context;
use settings::Settings;
use settings::SettingsStore;
use smol::Timer;
use std::time::{Duration, Instant};

/// Default interval between cursor blink cycles.
const DEFAULT_BLINK_INTERVAL: Duration = Duration::from_millis(530);

#[derive(Clone, Debug)]
pub enum BlinkAnimation {
    Instant,
    Fade {
        duration: Duration,
    },
    Pulse {
        fade_in_duration: Duration,
        hold_duration: Duration,
        fade_out_duration: Duration,
    },
    Zoom {
        duration: Duration,
        scale_factor: f32,
    },
    Slide {
        duration: Duration,
    },
    Breathe {
        duration: Duration,
    },
}

impl Default for BlinkAnimation {
    fn default() -> Self {
        BlinkAnimation::Fade {
            duration: Duration::from_millis(150),
        }
    }
}

pub struct BlinkManager {
    blink_interval: Duration,
    blink_epoch: usize,
    blinking_paused: bool,
    visible: bool,
    enabled: bool,
    animation: BlinkAnimation,
    animation_progress: f32,
    animation_scale: f32,
    animation_start_time: Option<Instant>,
    target_visible: bool,
    slide_offset: f32,
}

impl BlinkManager {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<SettingsStore>(move |this, cx| {
            this.update_from_settings(cx);
            this.blink_cursors(this.blink_epoch, cx)
        })
        .detach();

        let settings = crate::EditorSettings::get_global(cx);
        Self {
            blink_interval: DEFAULT_BLINK_INTERVAL,
            blink_epoch: 0,
            blinking_paused: false,
            visible: true,
            enabled: false,
            animation: settings.cursor_blink_animation.clone(),
            animation_progress: 1.0,
            animation_scale: 1.0,
            animation_start_time: None,
            target_visible: true,
            slide_offset: 0.0,
        }
    }

    pub fn set_animation(&mut self, animation: BlinkAnimation) {
        self.animation = animation;
    }

    fn update_from_settings(&mut self, cx: &mut Context<Self>) {
        let settings = crate::EditorSettings::get_global(cx);
        self.animation = settings.cursor_blink_animation.clone();
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    pub fn pause_blinking(&mut self, cx: &mut Context<Self>) {
        self.show_cursor(cx);

        let epoch = self.next_blink_epoch();
        let interval = self.blink_interval;
        cx.spawn(async move |this, cx| {
            Timer::after(interval).await;
            this.update(cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
        })
        .detach();
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.blink_cursors(epoch, cx);
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if EditorSettings::get_global(cx).cursor_blink {
            if epoch == self.blink_epoch && self.enabled && !self.blinking_paused {
                self.target_visible = !self.target_visible;
                self.start_animation_to_target(cx);

                let epoch = self.next_blink_epoch();
                let interval = self.blink_interval;
                cx.spawn(async move |this, cx| {
                    Timer::after(interval).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| this.blink_cursors(epoch, cx))
                            .ok();
                    }
                })
                .detach();
            }
        } else {
            self.show_cursor(cx);
        }
    }

    fn start_animation_to_target(&mut self, cx: &mut Context<Self>) {
        match &self.animation {
            BlinkAnimation::Instant => {
                self.visible = self.target_visible;
                self.animation_progress = if self.target_visible { 1.0 } else { 0.0 };
                self.animation_scale = 1.0;
                self.slide_offset = 0.0;
                cx.notify();
            }
            BlinkAnimation::Fade { duration } => {
                self.animation_start_time = Some(Instant::now());
                let animation_duration = *duration;
                self.animate_step_fade(animation_duration, cx);
            }
            BlinkAnimation::Pulse {
                fade_in_duration,
                hold_duration,
                fade_out_duration,
            } => {
                self.animation_start_time = Some(Instant::now());
                let _total_duration = *fade_in_duration + *hold_duration + *fade_out_duration;
                self.animate_step_pulse(*fade_in_duration, *hold_duration, *fade_out_duration, cx);
            }
            BlinkAnimation::Zoom {
                duration,
                scale_factor,
            } => {
                self.animation_start_time = Some(Instant::now());
                self.animate_step_zoom(*duration, *scale_factor, cx);
            }
            BlinkAnimation::Slide { duration } => {
                self.animation_start_time = Some(Instant::now());
                self.animate_step_slide(*duration, cx);
            }
            BlinkAnimation::Breathe { duration } => {
                self.animation_start_time = Some(Instant::now());
                self.animate_step_breathe(*duration, cx);
            }
        }
    }

    fn animate_step_fade(&mut self, animation_duration: Duration, cx: &mut Context<Self>) {
        if let Some(start_time) = self.animation_start_time {
            let elapsed = start_time.elapsed();
            let progress = (elapsed.as_secs_f32() / animation_duration.as_secs_f32()).min(1.0);

            // Calculate the animation progress based on target direction
            self.animation_progress = if self.target_visible {
                progress
            } else {
                1.0 - progress
            };

            self.animation_scale = 1.0;
            self.visible = self.animation_progress > 0.0;
            cx.notify();

            // Continue animation if not complete
            if progress < 1.0 {
                let frame_duration = Duration::from_millis(16); // ~60fps
                cx.spawn(async move |this, cx| {
                    Timer::after(frame_duration).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.animate_step_fade(animation_duration, cx)
                        })
                        .ok();
                    }
                })
                .detach();
            } else {
                self.animation_start_time = None;
                self.visible = self.target_visible;
                self.animation_progress = if self.target_visible { 1.0 } else { 0.0 };
                self.animation_scale = 1.0;
            }
        }
    }

    fn animate_step_pulse(
        &mut self,
        fade_in_duration: Duration,
        hold_duration: Duration,
        fade_out_duration: Duration,
        cx: &mut Context<Self>,
    ) {
        if let Some(start_time) = self.animation_start_time {
            let elapsed = start_time.elapsed();
            let total_duration = fade_in_duration + hold_duration + fade_out_duration;
            let total_progress = (elapsed.as_secs_f32() / total_duration.as_secs_f32()).min(1.0);

            let fade_in_end = fade_in_duration.as_secs_f32() / total_duration.as_secs_f32();
            let hold_end =
                (fade_in_duration + hold_duration).as_secs_f32() / total_duration.as_secs_f32();

            self.animation_progress = if self.target_visible {
                if total_progress <= fade_in_end {
                    total_progress / fade_in_end
                } else if total_progress <= hold_end {
                    1.0
                } else {
                    1.0 - (total_progress - hold_end) / (1.0 - hold_end)
                }
            } else {
                if total_progress <= fade_in_end {
                    1.0 - (total_progress / fade_in_end)
                } else if total_progress <= hold_end {
                    0.0
                } else {
                    (total_progress - hold_end) / (1.0 - hold_end)
                }
            };

            self.animation_scale = 1.0;
            self.visible = self.animation_progress > 0.0;
            cx.notify();

            if total_progress < 1.0 {
                let frame_duration = Duration::from_millis(16);
                cx.spawn(async move |this, cx| {
                    Timer::after(frame_duration).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.animate_step_pulse(
                                fade_in_duration,
                                hold_duration,
                                fade_out_duration,
                                cx,
                            )
                        })
                        .ok();
                    }
                })
                .detach();
            } else {
                self.animation_start_time = None;
                self.visible = self.target_visible;
                self.animation_progress = if self.target_visible { 1.0 } else { 0.0 };
                self.animation_scale = 1.0;
            }
        }
    }

    fn animate_step_zoom(
        &mut self,
        animation_duration: Duration,
        scale_factor: f32,
        cx: &mut Context<Self>,
    ) {
        if let Some(start_time) = self.animation_start_time {
            let elapsed = start_time.elapsed();
            let progress = (elapsed.as_secs_f32() / animation_duration.as_secs_f32()).min(1.0);

            let eased_progress = {
                let t = progress;
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            };

            if self.target_visible {
                self.animation_progress = eased_progress;
                self.animation_scale = scale_factor + (1.0 - scale_factor) * eased_progress;
            } else {
                self.animation_progress = 1.0 - eased_progress;
                self.animation_scale = 1.0 - (1.0 - scale_factor) * eased_progress;
            }

            self.visible = self.animation_progress > 0.0;
            cx.notify();

            if progress < 1.0 {
                let frame_duration = Duration::from_millis(16);
                cx.spawn(async move |this, cx| {
                    Timer::after(frame_duration).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.animate_step_zoom(animation_duration, scale_factor, cx)
                        })
                        .ok();
                    }
                })
                .detach();
            } else {
                self.animation_start_time = None;
                self.visible = self.target_visible;
                self.animation_progress = if self.target_visible { 1.0 } else { 0.0 };
                self.animation_scale = 1.0;
                self.slide_offset = 0.0;
            }
        }
    }

    fn animate_step_slide(&mut self, animation_duration: Duration, cx: &mut Context<Self>) {
        if let Some(start_time) = self.animation_start_time {
            let elapsed = start_time.elapsed();
            let progress = (elapsed.as_secs_f32() / animation_duration.as_secs_f32()).min(1.0);

            let eased_progress = progress * progress * (3.0 - 2.0 * progress); // smoothstep

            if self.target_visible {
                self.animation_progress = eased_progress;
                self.slide_offset = 4.0 * (1.0 - eased_progress); // slide in from right
            } else {
                self.animation_progress = 1.0 - eased_progress;
                self.slide_offset = 4.0 * eased_progress; // slide out to right
            }

            self.animation_scale = 1.0;
            self.visible = self.animation_progress > 0.0;
            cx.notify();

            if progress < 1.0 {
                let frame_duration = Duration::from_millis(16);
                cx.spawn(async move |this, cx| {
                    Timer::after(frame_duration).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.animate_step_slide(animation_duration, cx)
                        })
                        .ok();
                    }
                })
                .detach();
            } else {
                self.animation_start_time = None;
                self.visible = self.target_visible;
                self.animation_progress = if self.target_visible { 1.0 } else { 0.0 };
                self.animation_scale = 1.0;
                self.slide_offset = 0.0;
            }
        }
    }

    fn animate_step_breathe(&mut self, animation_duration: Duration, cx: &mut Context<Self>) {
        if let Some(start_time) = self.animation_start_time {
            let elapsed = start_time.elapsed();
            let progress = (elapsed.as_secs_f32() / animation_duration.as_secs_f32()).min(1.0);

            // Sine wave for organic breathing effect
            let sine_progress = (progress * 2.0 * std::f32::consts::PI).sin() * 0.5 + 0.5;

            if self.target_visible {
                // Breathe in (scale up slightly while fading in)
                self.animation_progress = progress;
                self.animation_scale = 1.0 + sine_progress * 0.1; // subtle scale 1.0 to 1.1
            } else {
                // Breathe out (scale down while fading out)
                self.animation_progress = 1.0 - progress;
                self.animation_scale = 1.0 + (1.0 - sine_progress) * 0.1;
            }

            self.slide_offset = 0.0;
            self.visible = self.animation_progress > 0.0;
            cx.notify();

            if progress < 1.0 {
                let frame_duration = Duration::from_millis(16);
                cx.spawn(async move |this, cx| {
                    Timer::after(frame_duration).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| {
                            this.animate_step_breathe(animation_duration, cx)
                        })
                        .ok();
                    }
                })
                .detach();
            } else {
                self.animation_start_time = None;
                self.visible = self.target_visible;
                self.animation_progress = if self.target_visible { 1.0 } else { 0.0 };
                self.animation_scale = 1.0;
                self.slide_offset = 0.0;
            }
        }
    }

    pub fn show_cursor(&mut self, cx: &mut Context<BlinkManager>) {
        if !self.visible || self.animation_progress < 1.0 {
            self.visible = true;
            self.target_visible = true;
            self.animation_progress = 1.0;
            self.animation_scale = 1.0;
            self.slide_offset = 0.0;
            self.animation_start_time = None;
            cx.notify();
        }
    }

    pub fn enable(&mut self, cx: &mut Context<Self>) {
        if self.enabled {
            return;
        }

        self.enabled = true;

        self.visible = false;
        self.target_visible = false;
        self.animation_progress = 0.0;
        self.animation_scale = 1.0;
        self.slide_offset = 0.0;
        self.animation_start_time = None;
        self.blink_cursors(self.blink_epoch, cx);
    }

    pub fn disable(&mut self, _cx: &mut Context<Self>) {
        self.visible = false;
        self.target_visible = false;
        self.animation_progress = 0.0;
        self.animation_start_time = None;
        self.animation_scale = 1.0;
        self.slide_offset = 0.0;
        self.enabled = false;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn opacity(&self) -> f32 {
        if self.visible {
            self.animation_progress
        } else {
            0.0
        }
    }

    pub fn scale(&self) -> f32 {
        if self.visible {
            self.animation_scale
        } else {
            1.0
        }
    }

    pub fn slide_offset(&self) -> f32 {
        if self.visible { self.slide_offset } else { 0.0 }
    }
}
