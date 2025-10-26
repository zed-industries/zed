use core::time::Duration;

use gpui::{App, AppContext, Entity, Styled, Subscription, Task, WeakEntity};
use settings::Settings;
use time::{
    OffsetDateTime,
    format_description::{
        BorrowedFormatItem, Component,
        modifier::{Hour, Minute, Padding, Period, Second},
    },
};
use ui::{
    BorrowAppContext, Button, ButtonCommon, Clickable, Context, FluentBuilder, IntoElement,
    LabelSize, ParentElement, Render, Tooltip, Window, div,
};
use util::ResultExt;
use workspace::{ClockSettings, StatusItemView, Workspace, item::ItemHandle};

pub struct Clock {
    update_time: Task<()>,
    _observe_active_editor: Option<Subscription>,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            update_time: Task::ready(()),
            _observe_active_editor: None,
        }
    }

    fn update_time(&mut self, cx: &mut Context<Self>) {
        // cx.background_spawn(async {

        // });

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

        let time = OffsetDateTime::now_local()
            .log_err()
            .unwrap_or_else(|| OffsetDateTime::now_utc());

        let mut hour = Hour::default();
        hour.is_12_hour_clock = clock.use_12_hour_clock;
        let mut period = Period::default();
        period.is_uppercase = true;

        let end = if clock.use_12_hour_clock {
            BorrowedFormatItem::Compound(&[
                BorrowedFormatItem::Literal(b" "),
                BorrowedFormatItem::Component(Component::Period(period)),
            ])
        } else {
            BorrowedFormatItem::Literal(&[])
        };
        let format = [
            BorrowedFormatItem::Component(Component::Hour(hour)),
            BorrowedFormatItem::Literal(b":"),
            BorrowedFormatItem::Component(Component::Minute(Minute::default())),
            end,
        ];
        let text = time
            .format(&format[..])
            .log_err()
            .unwrap_or_else(|| "00:00".to_owned());

        // struct CurrentTime(time::OffsetDateTime);
        // cx.observe_window_activation(window, |clock, window, cx| {

        // });
        // cx.observe_global::<CurrentTime>(|clock, cx| {
        //     let time: CurrentTime = cx.global();
        // });

        self.update_time(cx);

        div().child(
            Button::new("clock", text)
                .label_size(LabelSize::Small)
                .on_click(|_event, window, cx| {
                    window.dispatch_action(Box::new(zed_actions::OpenSettings), cx)
                })
                .tooltip(move |_window, cx| {
                    Tooltip::for_action(
                        "Open Settings and search for clock",
                        &zed_actions::OpenSettings,
                        cx,
                    )
                }),
        )
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
