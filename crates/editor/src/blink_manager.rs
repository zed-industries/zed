use gpui::Context;
use settings::SettingsStore;
use std::time::{Duration, Instant};
use ui::App;

const SMOOTH_BLINK_FADE_DURATION_MS: f32 = 150.0;

pub struct BlinkManager {
    blink_interval: Duration,
    blink_epoch: usize,
    /// Whether the blinking is paused.
    blinking_paused: bool,
    /// Whether the cursor should be visibly rendered or not.
    visible: bool,
    /// Whether the blinking currently enabled.
    enabled: bool,
    /// Whether the blinking is enabled in the settings.
    blink_enabled_in_settings: fn(&App) -> bool,
    /// Whether smooth blink transitions are enabled.
    smooth_blink_enabled: bool,
    /// Current opacity for smooth blink transitions (0.0-1.0).
    current_opacity: f32,
    /// Target opacity for smooth blink transitions.
    target_opacity: f32,
    /// Last update time for smooth blink interpolation.
    last_smooth_blink_update: Option<Instant>,
}

impl BlinkManager {
    pub fn new(
        blink_interval: Duration,
        blink_enabled_in_settings: fn(&App) -> bool,
        cx: &mut Context<Self>,
    ) -> Self {
        // Make sure we blink the cursors if the setting is re-enabled
        cx.observe_global::<SettingsStore>(move |this, cx| {
            this.blink_cursors(this.blink_epoch, cx)
        })
        .detach();

        Self {
            blink_interval,
            blink_epoch: 0,
            blinking_paused: false,
            visible: true,
            enabled: false,
            blink_enabled_in_settings,
            smooth_blink_enabled: false,
            current_opacity: 1.0,
            target_opacity: 1.0,
            last_smooth_blink_update: None,
        }
    }

    pub fn set_smooth_blink_enabled(&mut self, enabled: bool) {
        self.smooth_blink_enabled = enabled;
        self.current_opacity = if self.visible { 1.0 } else { 0.0 };
        self.target_opacity = self.current_opacity;
        self.last_smooth_blink_update = None;
    }

    pub fn opacity(&mut self, cx: &mut Context<Self>) -> f32 {
        if !self.smooth_blink_enabled {
            return if self.visible { 1.0 } else { 0.0 };
        }

        let now = Instant::now();
        if let Some(last_update) = self.last_smooth_blink_update {
            let dt_ms = now.duration_since(last_update).as_secs_f32() * 1000.0;
            let blend_factor = (dt_ms / SMOOTH_BLINK_FADE_DURATION_MS).clamp(0.0, 1.0);

            let new_opacity =
                self.current_opacity + (self.target_opacity - self.current_opacity) * blend_factor;
            if (new_opacity - self.current_opacity).abs() > 0.001 {
                self.current_opacity = new_opacity;
                cx.notify();
            }
        }
        self.last_smooth_blink_update = Some(now);
        self.current_opacity
    }

    pub fn is_smooth_blink_animating(&self) -> bool {
        self.smooth_blink_enabled && (self.current_opacity - self.target_opacity).abs() > 0.01
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    /// Reset blink state after a logical cursor move.
    /// Cursor becomes visible immediately and blinking resumes after a short pause.
    pub fn cursor_moved(&mut self, cx: &mut Context<Self>) {
        self.show_cursor(cx);
        self.blinking_paused = true;

        let epoch = self.next_blink_epoch();
        let interval = Duration::from_millis(500);
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(interval).await;
            this.update(cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
        })
        .detach();
    }

    pub fn pause_blinking(&mut self, cx: &mut Context<Self>) {
        self.cursor_moved(cx);
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.show_cursor(cx);
            let interval = self.blink_interval;
            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(interval).await;
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| this.blink_cursors(epoch, cx));
                }
            })
            .detach();
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if (self.blink_enabled_in_settings)(cx) {
            if epoch == self.blink_epoch && self.enabled && !self.blinking_paused {
                self.visible = !self.visible;
                self.target_opacity = if self.visible { 1.0 } else { 0.0 };
                cx.notify();

                let epoch = self.next_blink_epoch();
                let interval = self.blink_interval;
                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(interval).await;
                    if let Some(this) = this.upgrade() {
                        this.update(cx, |this, cx| this.blink_cursors(epoch, cx));
                    }
                })
                .detach();
            }
        } else {
            self.show_cursor(cx);
        }
    }

    pub fn show_cursor(&mut self, cx: &mut Context<BlinkManager>) {
        self.visible = true;
        self.target_opacity = 1.0;
        if self.smooth_blink_enabled {
            self.current_opacity = 1.0;
            self.last_smooth_blink_update = None;
        }
        cx.notify();
    }

    /// Enable the blinking of the cursor.
    pub fn enable(&mut self, cx: &mut Context<Self>) {
        if self.enabled {
            return;
        }

        self.enabled = true;
        // Set cursors as invisible and start blinking: this causes cursors
        // to be visible during the next render.
        self.visible = false;
        self.target_opacity = 0.0;
        self.blink_cursors(self.blink_epoch, cx);
    }

    /// Disable the blinking of the cursor.
    pub fn disable(&mut self, _cx: &mut Context<Self>) {
        self.visible = false;
        self.enabled = false;
        self.target_opacity = 0.0;
        if !self.smooth_blink_enabled {
            self.current_opacity = 0.0;
        }
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn should_render(&self) -> bool {
        if self.smooth_blink_enabled {
            self.visible || self.is_smooth_blink_animating()
        } else {
            self.visible
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};

    #[gpui::test]
    async fn cursor_move_forces_cursor_visible_immediately(cx: &mut TestAppContext) {
        let blink_manager =
            cx.new(|cx| BlinkManager::new(Duration::from_millis(500), |_| true, cx));

        blink_manager.update(cx, |blink_manager: &mut BlinkManager, cx| {
            blink_manager.disable(cx);
            blink_manager.set_smooth_blink_enabled(true);

            assert_eq!(blink_manager.opacity(cx), 0.0);
            blink_manager.cursor_moved(cx);
            assert_eq!(blink_manager.opacity(cx), 1.0);
            assert!(blink_manager.should_render());
            assert!(!blink_manager.is_smooth_blink_animating());
        });
    }
}
