use std::collections::HashSet;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use db::kvp::KEY_VALUE_STORE;
use gpui::{App, EntityId, EventEmitter, Subscription};
use ui::{IconButtonShape, Tooltip, prelude::*};
use workspace::item::{ItemEvent, ItemHandle};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

pub struct MultibufferHint {
    shown_on: HashSet<EntityId>,
    active_item: Option<Box<dyn ItemHandle>>,
    subscription: Option<Subscription>,
}

const NUMBER_OF_HINTS: usize = 10;

const SHOWN_COUNT_KEY: &str = "MULTIBUFFER_HINT_SHOWN_COUNT";

impl Default for MultibufferHint {
    fn default() -> Self {
        Self::new()
    }
}

impl MultibufferHint {
    pub fn new() -> Self {
        Self {
            shown_on: Default::default(),
            active_item: None,
            subscription: None,
        }
    }
}

impl MultibufferHint {
    fn counter() -> &'static AtomicUsize {
        static SHOWN_COUNT: OnceLock<AtomicUsize> = OnceLock::new();
        SHOWN_COUNT.get_or_init(|| {
            let value: usize = KEY_VALUE_STORE
                .read_kvp(SHOWN_COUNT_KEY)
                .ok()
                .flatten()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            AtomicUsize::new(value)
        })
    }

    fn shown_count() -> usize {
        Self::counter().load(Ordering::Relaxed)
    }

    fn increment_count(cx: &mut App) {
        Self::set_count(Self::shown_count() + 1, cx)
    }

    pub(crate) fn set_count(count: usize, cx: &mut App) {
        Self::counter().store(count, Ordering::Relaxed);

        db::write_and_log(cx, move || {
            KEY_VALUE_STORE.write_kvp(SHOWN_COUNT_KEY.to_string(), format!("{}", count))
        });
    }

    fn dismiss(&mut self, cx: &mut App) {
        Self::set_count(NUMBER_OF_HINTS, cx)
    }

    /// Determines the toolbar location for this [`MultibufferHint`].
    fn determine_toolbar_location(&mut self, cx: &mut Context<Self>) -> ToolbarItemLocation {
        if Self::shown_count() >= NUMBER_OF_HINTS {
            return ToolbarItemLocation::Hidden;
        }

        let Some(active_pane_item) = self.active_item.as_ref() else {
            return ToolbarItemLocation::Hidden;
        };

        if active_pane_item.is_singleton(cx)
            || active_pane_item.breadcrumbs(cx.theme(), cx).is_none()
            || !active_pane_item.can_save(cx)
        {
            return ToolbarItemLocation::Hidden;
        }

        if self.shown_on.insert(active_pane_item.item_id()) {
            Self::increment_count(cx);
        }

        ToolbarItemLocation::Secondary
    }
}

impl EventEmitter<ToolbarItemEvent> for MultibufferHint {}

impl ToolbarItemView for MultibufferHint {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_item = active_pane_item.map(|item| item.boxed_clone());

        let Some(active_pane_item) = active_pane_item else {
            return ToolbarItemLocation::Hidden;
        };

        let this = cx.entity().downgrade();
        self.subscription = Some(active_pane_item.subscribe_to_item_events(
            window,
            cx,
            Box::new(move |event, _, cx| {
                if let ItemEvent::UpdateBreadcrumbs = event {
                    this.update(cx, |this, cx| {
                        cx.notify();
                        let location = this.determine_toolbar_location(cx);
                        cx.emit(ToolbarItemEvent::ChangeLocation(location))
                    })
                    .ok();
                }
            }),
        ));

        self.determine_toolbar_location(cx)
    }
}

impl Render for MultibufferHint {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .px_2()
            .py_0p5()
            .justify_between()
            .bg(cx.theme().status().info_background.opacity(0.5))
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_sm()
            .overflow_hidden()
            .child(
                h_flex()
                    .gap_0p5()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Info)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(Label::new(
                                "Edit and save files directly in the results multibuffer!",
                            )),
                    )
                    .child(
                        Button::new("open_docs", "Learn More")
                            .icon(IconName::ArrowUpRight)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .icon_position(IconPosition::End)
                            .on_click(move |_event, _, cx| {
                                cx.open_url("https://zed.dev/docs/multibuffers")
                            }),
                    ),
            )
            .child(
                IconButton::new("dismiss", IconName::Close)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .on_click(cx.listener(|this, _event, _, cx| {
                        this.dismiss(cx);
                        cx.emit(ToolbarItemEvent::ChangeLocation(
                            ToolbarItemLocation::Hidden,
                        ))
                    }))
                    .tooltip(Tooltip::text("Dismiss Hint")),
            )
            .into_any_element()
    }
}
