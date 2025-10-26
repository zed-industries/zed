use gpui::{App, AppContext, Entity, Styled, Subscription, Task, WeakEntity};
use settings::Settings;
use std::time::Duration;
use ui::{
    BorrowAppContext, Button, ButtonCommon, Clickable, Context, FluentBuilder, IntoElement,
    LabelSize, ParentElement, Render, Tooltip, Window, div,
};
use util::ResultExt;
use workspace::{StatusBarSettings, StatusItemView, Workspace, item::ItemHandle};

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
            cx.background_executor()
                .timer(std::time::Duration::from_secs(60))
                .await;
            cx.update(|cx| {
                cx.notify(clock.entity_id());
            })
            .log_err();
        });
    }
}

impl Render for Clock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let clock = &StatusBarSettings::get_global(cx).clock;
        if !clock.show {
            return div().hidden();
        }

        // struct CurrentTime(time::OffsetDateTime);
        // cx.observe_window_activation(window, |clock, window, cx| {

        // });
        // cx.observe_global::<CurrentTime>(|clock, cx| {
        //     let time: CurrentTime = cx.global();
        // });

        let time = time::OffsetDateTime::now_local()
            .log_err()
            .unwrap_or_else(|| time::OffsetDateTime::now_utc());
        use time::format_description::{
            BorrowedFormatItem, Component,
            modifier::{Hour, Minute, Padding, Period, Second},
        };

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
        let text = time.format(&format[..]).unwrap();

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
