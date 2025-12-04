use gpui::Context;
use settings::SettingsStore;
use smol::Timer;
use std::time::Duration;
use ui::App;

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
        }
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
        if (self.blink_enabled_in_settings)(cx) {
            if epoch == self.blink_epoch && self.enabled && !self.blinking_paused {
                self.visible = !self.visible;
                cx.notify();

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

    pub fn show_cursor(&mut self, cx: &mut Context<BlinkManager>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }
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
        self.blink_cursors(self.blink_epoch, cx);
    }

    /// Disable the blinking of the cursor.
    pub fn disable(&mut self, _cx: &mut Context<Self>) {
        self.visible = false;
        self.enabled = false;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
