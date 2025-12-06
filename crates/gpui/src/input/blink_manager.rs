use crate::Context;
use smol::Timer;
use std::time::Duration;

/// Manages cursor blinking state for text input components.
///
/// `BlinkManager` handles the timing and visibility state for cursor blinking.
/// It can be enabled/disabled and paused (e.g., during typing), and tracks
/// whether the cursor should be visible at any given moment.
pub struct BlinkManager {
    blink_interval: Duration,
    blink_epoch: usize,
    blinking_paused: bool,
    visible: bool,
    enabled: bool,
}

impl BlinkManager {
    /// Creates a new `BlinkManager` with the specified blink interval.
    ///
    /// The blink manager starts in a disabled state with the cursor visible.
    pub fn new(blink_interval: Duration, _cx: &mut Context<Self>) -> Self {
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

    /// Pauses cursor blinking temporarily, showing the cursor immediately.
    ///
    /// This is typically called when the user performs an action like typing,
    /// to ensure the cursor remains visible during interaction. Blinking will
    /// automatically resume after the blink interval has elapsed.
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
    }

    /// Makes the cursor visible immediately.
    ///
    /// If the cursor is already visible, this is a no-op. Otherwise, it sets
    /// the cursor to visible and notifies observers.
    pub fn show_cursor(&mut self, cx: &mut Context<BlinkManager>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }
    }

    /// Enables cursor blinking.
    ///
    /// When enabled, the cursor will alternate between visible and invisible
    /// states at the configured blink interval. If already enabled, this is a no-op.
    pub fn enable(&mut self, cx: &mut Context<Self>) {
        if self.enabled {
            return;
        }

        self.enabled = true;
        // Set cursor as invisible and start blinking: this causes cursor
        // to be visible during the next render.
        self.visible = false;
        self.blink_cursors(self.blink_epoch, cx);
    }

    /// Disables cursor blinking.
    ///
    /// When disabled, the cursor visibility is set to false and blinking stops.
    /// Call `show_cursor` after this if you want the cursor to remain visible
    /// while blinking is disabled.
    pub fn disable(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        self.enabled = false;
        cx.notify();
    }

    /// Returns whether the cursor should currently be rendered as visible.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Returns whether cursor blinking is currently enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
