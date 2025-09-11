use crate::{AgentPanel, RemoveSelectedThread};
use agent::history_store::HistoryEntry;
use gpui::{App, ClickEvent, WeakEntity, Window};
use time::{OffsetDateTime, UtcOffset};
use ui::{HighlightedLabel, IconButtonShape, ListItem, ListItemSpacing, Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct HistoryEntryElement {
    entry: HistoryEntry,
    agent_panel: WeakEntity<AgentPanel>,
    selected: bool,
    hovered: bool,
    highlight_positions: Vec<usize>,
    timestamp_format: EntryTimeFormat,
    on_hover: Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
}

impl HistoryEntryElement {
    pub fn new(entry: HistoryEntry, agent_panel: WeakEntity<AgentPanel>) -> Self {
        Self {
            entry,
            agent_panel,
            selected: false,
            hovered: false,
            highlight_positions: vec![],
            timestamp_format: EntryTimeFormat::DateAndTime,
            on_hover: Box::new(|_, _, _| {}),
        }
    }

    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    pub fn on_hover(mut self, on_hover: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Box::new(on_hover);
        self
    }
}

impl RenderOnce for HistoryEntryElement {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (id, summary, timestamp) = match &self.entry {
            HistoryEntry::Thread(thread) => (
                thread.id.to_string(),
                thread.summary.clone(),
                thread.updated_at.timestamp(),
            ),
            HistoryEntry::Context(context) => (
                context.path.to_string_lossy().to_string(),
                context.title.clone(),
                context.mtime.timestamp(),
            ),
        };

        let thread_timestamp =
            self.timestamp_format
                .format_timestamp(&self.agent_panel, timestamp, cx);

        ListItem::new(SharedString::from(id))
            .rounded()
            .toggle_state(self.selected)
            .spacing(ListItemSpacing::Sparse)
            .start_slot(
                h_flex()
                    .w_full()
                    .gap_2()
                    .justify_between()
                    .child(
                        HighlightedLabel::new(summary, self.highlight_positions)
                            .size(LabelSize::Small)
                            .truncate(),
                    )
                    .child(
                        Label::new(thread_timestamp)
                            .color(Color::Muted)
                            .size(LabelSize::XSmall),
                    ),
            )
            .on_hover(self.on_hover)
            .end_slot::<IconButton>(if self.hovered || self.selected {
                Some(
                    IconButton::new("delete", IconName::Trash)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .tooltip(move |window, cx| {
                            Tooltip::for_action("Delete", &RemoveSelectedThread, window, cx)
                        })
                        .on_click({
                            let agent_panel = self.agent_panel.clone();

                            let f: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> =
                                match &self.entry {
                                    HistoryEntry::Thread(thread) => {
                                        let id = thread.id.clone();

                                        Box::new(move |_event, _window, cx| {
                                            agent_panel
                                                .update(cx, |this, cx| {
                                                    this.delete_thread(&id, cx)
                                                        .detach_and_log_err(cx);
                                                })
                                                .ok();
                                        })
                                    }
                                    HistoryEntry::Context(context) => {
                                        let path = context.path.clone();

                                        Box::new(move |_event, _window, cx| {
                                            agent_panel
                                                .update(cx, |this, cx| {
                                                    this.delete_context(path.clone(), cx)
                                                        .detach_and_log_err(cx);
                                                })
                                                .ok();
                                        })
                                    }
                                };
                            f
                        }),
                )
            } else {
                None
            })
            .on_click({
                let agent_panel = self.agent_panel.clone();

                let f: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static> = match &self.entry
                {
                    HistoryEntry::Thread(thread) => {
                        let id = thread.id.clone();
                        Box::new(move |_event, window, cx| {
                            agent_panel
                                .update(cx, |this, cx| {
                                    this.open_thread_by_id(&id, window, cx)
                                        .detach_and_log_err(cx);
                                })
                                .ok();
                        })
                    }
                    HistoryEntry::Context(context) => {
                        let path = context.path.clone();
                        Box::new(move |_event, window, cx| {
                            agent_panel
                                .update(cx, |this, cx| {
                                    this.open_saved_prompt_editor(path.clone(), window, cx)
                                        .detach_and_log_err(cx);
                                })
                                .ok();
                        })
                    }
                };
                f
            })
    }
}

#[derive(Clone, Copy)]
pub enum EntryTimeFormat {
    DateAndTime,
}

impl EntryTimeFormat {
    fn format_timestamp(
        &self,
        agent_panel: &WeakEntity<AgentPanel>,
        timestamp: i64,
        cx: &App,
    ) -> String {
        let timestamp = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
        let timezone = agent_panel
            .read_with(cx, |this, _cx| this.local_timezone())
            .unwrap_or(UtcOffset::UTC);

        match &self {
            EntryTimeFormat::DateAndTime => time_format::format_localized_timestamp(
                timestamp,
                OffsetDateTime::now_utc(),
                timezone,
                time_format::TimestampFormat::EnhancedAbsolute,
            ),
        }
    }
}
