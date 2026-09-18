use gpui::{Context, Task};
use settings::SettingsStore;
use std::time::Duration;
use ui::App;

pub struct BlinkManager {
    blink_interval: Duration,
    blink_epoch: usize,
    blink_task: Option<Task<()>>,
    /// Whether the blinking is paused.
    blinking_paused: bool,
    /// Whether the cursor should be visibly rendered or not.
    visible: bool,
    /// Whether the blinking is currently enabled.
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
            blink_task: None,
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
        self.blink_task = None;
        if !self.enabled || !(self.blink_enabled_in_settings)(cx) {
            return;
        }

        let interval = Duration::from_millis(500);
        self.blink_task = Some(cx.spawn(async move |blink_manager, cx| {
            cx.background_executor().timer(interval).await;
            blink_manager
                .update(cx, |blink_manager, cx| {
                    blink_manager.resume_cursor_blinking(epoch, cx)
                })
                .ok();
        }));
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
                self.blink_task = Some(cx.spawn(async move |blink_manager, cx| {
                    cx.background_executor().timer(interval).await;
                    if let Some(blink_manager) = blink_manager.upgrade() {
                        blink_manager.update(cx, |blink_manager, cx| {
                            blink_manager.blink_cursors(epoch, cx)
                        });
                    }
                }));
            }
        } else {
            self.next_blink_epoch();
            self.blink_task = None;
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
        self.next_blink_epoch();
        self.blink_task = None;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    #[cfg(test)]
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor_settings::EditorSettings;
    use gpui::{AppContext, TestAppContext, UpdateGlobal};
    use settings::Settings;

    #[gpui::test]
    fn test_blink_timer_replacement_and_cancellation(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            EditorSettings::register(cx);
        });
        let blink_manager = cx.new(|cx| {
            BlinkManager::new(
                Duration::from_millis(300),
                |cx| EditorSettings::get_global(cx).cursor_blink,
                cx,
            )
        });
        let state = |cx: &TestAppContext| {
            blink_manager.read_with(cx, |blink_manager, _| {
                (blink_manager.visible(), blink_manager.blink_task.is_some())
            })
        };

        let start = cx.background_executor.now();
        let advance_to = |cx: &TestAppContext, milliseconds| {
            let deadline = start + Duration::from_millis(milliseconds);
            cx.background_executor
                .advance_clock(deadline - cx.background_executor.now());
        };
        blink_manager.update(cx, BlinkManager::enable);
        for deadline in [0, 100] {
            advance_to(cx, deadline);
            blink_manager.update(cx, BlinkManager::pause_blinking);
        }
        for (deadline, visible) in [(300, true), (500, true), (599, true), (600, false)] {
            advance_to(cx, deadline);
            assert_eq!(state(cx), (visible, true));
        }

        blink_manager.update(cx, BlinkManager::pause_blinking);
        cx.run_until_parked();
        blink_manager.update(cx, BlinkManager::disable);
        assert_eq!(state(cx), (false, false));
        blink_manager.update(cx, BlinkManager::enable);
        for (deadline, visible) in [(899, true), (900, false), (1100, false)] {
            advance_to(cx, deadline);
            assert_eq!(state(cx), (visible, true));
        }

        blink_manager.update(cx, BlinkManager::pause_blinking);
        cx.run_until_parked();
        cx.update(|cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.editor.cursor_blink = Some(false);
                });
            });
        });
        assert_eq!(state(cx), (true, false));
        blink_manager.update(cx, BlinkManager::pause_blinking);
        assert_eq!(state(cx), (true, false));
        blink_manager.update(cx, BlinkManager::disable);
        advance_to(cx, 2100);
        assert_eq!(state(cx), (false, false));
        blink_manager.update(cx, BlinkManager::enable);
        assert_eq!(state(cx), (true, false));
    }
}
