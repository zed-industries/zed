use crate::EditorSettings;
use gpui::{Entity, ModelContext};
use settings::SettingsStore;
use smol::Timer;
use std::time::Duration;

pub struct BlinkManager {
    blink_interval: Duration,

    blink_epoch: usize,
    blinking_paused: bool,
    visible: bool,
    enabled: bool,
}

impl BlinkManager {
    pub fn new(blink_interval: Duration, cx: &mut ModelContext<Self>) -> Self {
        // Make sure we blink the cursors if the setting is re-enabled
        cx.observe_global::<SettingsStore, _>(move |this, cx| {
            this.blink_cursors(this.blink_epoch, cx)
        })
        .detach();

        Self {
            blink_interval,

            blink_epoch: 0,
            blinking_paused: false,
            visible: true,
            enabled: false,
        }
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    pub fn pause_blinking(&mut self, cx: &mut ModelContext<Self>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }

        let epoch = self.next_blink_epoch();
        let interval = self.blink_interval;
        cx.spawn(|this, mut cx| {
            let this = this.downgrade();
            async move {
                Timer::after(interval).await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
                }
            }
        })
        .detach();
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut ModelContext<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.blink_cursors(epoch, cx);
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut ModelContext<Self>) {
        if settings::get::<EditorSettings>(cx).cursor_blink {
            if epoch == self.blink_epoch && self.enabled && !self.blinking_paused {
                self.visible = !self.visible;
                cx.notify();

                let epoch = self.next_blink_epoch();
                let interval = self.blink_interval;
                cx.spawn(|this, mut cx| {
                    let this = this.downgrade();
                    async move {
                        Timer::after(interval).await;
                        if let Some(this) = this.upgrade(&cx) {
                            this.update(&mut cx, |this, cx| this.blink_cursors(epoch, cx));
                        }
                    }
                })
                .detach();
            }
        } else if !self.visible {
            self.visible = true;
            cx.notify();
        }
    }

    pub fn enable(&mut self, cx: &mut ModelContext<Self>) {
        self.enabled = true;
        // Set cursors as invisible and start blinking: this causes cursors
        // to be visible during the next render.
        self.visible = false;
        self.blink_cursors(self.blink_epoch, cx);
    }

    pub fn disable(&mut self, _cx: &mut ModelContext<Self>) {
        self.enabled = false;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}

impl Entity for BlinkManager {
    type Event = ();
}
