use core::time::Duration;

use gpui::{Styled, Task, WeakEntity};
use settings::Settings;
use time::{OffsetDateTime, UtcOffset};
use ui::{Context, IntoElement, ParentElement, Render, Window, div};
use util::ResultExt;
use workspace::{ClockSettings, StatusItemView, Workspace, item::ItemHandle};

// TODO: show in TitleBar
pub struct Clock {
    update_time: Task<()>,
    workspace: WeakEntity<Workspace>,
}

impl Clock {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            update_time: Task::ready(()),
            workspace: workspace.weak_handle(),
        }
    }

    fn update_time(&mut self, cx: &mut Context<Self>) {
        self.update_time = cx.spawn(async move |clock, cx| {
            let now = time::OffsetDateTime::now_utc();

            // compute the start of the next minute
            let next_minute = (now + time::Duration::minutes(1))
                .replace_second(0)
                .unwrap() // should not fail, 0 is in the expected range
                .replace_millisecond(0)
                .unwrap();

            // compute how long to wait, fall back to full minute
            let wait: Duration = (next_minute - now)
                .try_into()
                .log_err()
                .unwrap_or(Duration::from_secs(60));

            cx.background_executor().timer(wait).await;
            cx.update(|cx| {
                cx.notify(clock.entity_id());
            })
            .log_err();
        });
    }
}

impl Render for Clock {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let clock = ClockSettings::get_global(cx);
        if !clock.show {
            return div().hidden();
        }

        let format = if clock.use_12_hour_clock {
            time::macros::format_description!("[hour repr:12]:[minute] [period case:upper]")
        } else {
            time::macros::format_description!("[hour repr:24]:[minute]")
        };

        let offset = clock.offset.trim();
        let offset = if offset.is_empty() {
            None
        } else {
            let description =
                time::macros::format_description!("[offset_hour padding:none]:[offset_minute]");
            UtcOffset::parse(&offset, description)
                .inspect_err(|error| {
                    // TODO: allow ignoring notification, maybe use global or state to not show again
                    if let Some(workspace) = self.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            workspace.show_error(
                                &format!("Wrong UTC offset format. Example: +0:00 {error}"),
                                cx,
                            );
                        })
                    }
                })
                .ok()
        };
        let offset = offset.unwrap_or_else(|| {
            UtcOffset::current_local_offset()
                .log_err()
                .unwrap_or(UtcOffset::UTC)
        });

        let now = OffsetDateTime::now_utc().to_offset(offset);
        let text = now
            .format(format)
            .log_err()
            .unwrap_or_else(|| "00:00".to_owned());

        // cx.observe_window_activation(window, |clock, window, cx| {

        // });

        self.update_time(cx);

        div().child(text).text_sm()
    }
}

impl StatusItemView for Clock {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
