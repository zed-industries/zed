use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, EditorEvent, MultiBuffer};
use gpui::{
    Action, AnyElement, App, AppContext as _, AsyncWindowContext, ClickEvent, Context, Corner,
    Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ListAlignment,
    ListState, ModifiersChangedEvent, Pixels, Render, SharedString, Task, WeakEntity, Window,
    actions, black, div, list, px,
};
use language::Buffer;
use project::{
    LocalHistoryEntry, LocalHistoryEvent, LocalHistorySettings, LocalHistoryTransferMode, Project,
    ProjectPath,
};
use serde::{Deserialize, Serialize};
use settings::{
    DockPosition as SettingsDockPosition, LocalHistoryPanelHeaderVisibility, RegisterSetting,
    Settings, SettingsStore,
};
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;
use time::OffsetDateTime;
use time_format::{TimestampFormat, format_local_timestamp};
use ui::{
    Color, ContextMenu, Icon, IconButton, IconName, IconSize, Label, LabelSize, ListItem,
    ListItemSpacing, PopoverMenu, Tab, Tooltip, h_flex, prelude::*, v_flex,
};
use util::paths::PathExt as _;
use util::{ResultExt, TryFutureExt};
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, Toast, Workspace,
    dock::{DockPosition, Panel, PanelEvent},
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
    notifications::NotificationId,
    searchable::SearchableItemHandle,
};

const LOCAL_HISTORY_PANEL_KEY: &str = "LocalHistoryPanel";
const LOCAL_HISTORY_ENTRY_LABEL_SIZE: LabelSize = LabelSize::Small;

fn compute_entry_gaps(
    entries: &[LocalHistoryEntry],
    min_gap: Pixels,
    max_gap: Pixels,
) -> Vec<Pixels> {
    let mut gaps = vec![px(0.); entries.len()];
    if entries.len() <= 1 {
        return gaps;
    }

    let min_gap_f: f32 = min_gap.into();
    let max_gap_f: f32 = max_gap.into();
    let max_gap_f = max_gap_f.max(min_gap_f);

    let mut deltas = Vec::with_capacity(entries.len().saturating_sub(1));
    for i in 1..entries.len() {
        let prev = entries[i - 1].timestamp;
        let curr = entries[i].timestamp;
        let delta = (prev - curr).whole_milliseconds().max(0) as f32;
        deltas.push(delta);
    }

    let Some((&min_delta, &max_delta)) = deltas
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .zip(
            deltas
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        )
    else {
        return gaps;
    };

    if max_delta <= min_delta {
        let mid_gap = (min_gap_f + max_gap_f) * 0.5;
        for gap in gaps.iter_mut().skip(1) {
            *gap = px(mid_gap);
        }
        return gaps;
    }

    let range = max_delta - min_delta;
    for (ix, delta) in deltas.into_iter().enumerate() {
        let t = (delta - min_delta) / range;
        let gap = min_gap_f + (max_gap_f - min_gap_f) * t;
        gaps[ix + 1] = px(gap);
    }

    gaps
}

fn format_relative_compact(timestamp: OffsetDateTime, reference: OffsetDateTime) -> String {
    let difference = reference - timestamp;
    if difference.is_negative() {
        return "now".to_string();
    }

    let minutes = difference.whole_minutes();
    if minutes <= 0 {
        return "now".to_string();
    }
    if minutes < 60 {
        return format!("{minutes}m");
    }

    let hours = difference.whole_hours();
    if hours < 24 {
        return format!("{hours}h");
    }

    let date_diff = reference.date() - timestamp.date();
    let days = date_diff.whole_days();
    if days <= 0 {
        return "now".to_string();
    }
    if days < 7 {
        return format!("{days}d");
    }

    let weeks = date_diff.whole_weeks();
    if weeks <= 4 {
        return format!("{weeks}w");
    }

    let months = calculate_month_difference(timestamp, reference);
    if months <= 1 {
        return "1mo".to_string();
    }
    if months <= 11 {
        return format!("{months}mo");
    }

    let years = (reference.date().year() - timestamp.date().year()).max(1);
    format!("{years}y")
}

fn calculate_month_difference(timestamp: OffsetDateTime, reference: OffsetDateTime) -> usize {
    let timestamp_year = timestamp.year();
    let reference_year = reference.year();
    let timestamp_month: u8 = timestamp.month().into();
    let reference_month: u8 = reference.month().into();

    let month_diff = if reference_month >= timestamp_month {
        reference_month as usize - timestamp_month as usize
    } else {
        12 - timestamp_month as usize + reference_month as usize
    };

    let year_diff = (reference_year - timestamp_year) as usize;
    if year_diff == 0 {
        reference_month as usize - timestamp_month as usize
    } else if month_diff == 0 {
        year_diff * 12
    } else if timestamp_month > reference_month {
        (year_diff - 1) * 12 + month_diff
    } else {
        year_diff * 12 + month_diff
    }
}

actions!(
    local_history_panel,
    [
        /// Toggles the local history panel.
        ToggleFocus,
    ]
);

#[derive(Debug, RegisterSetting)]
pub struct LocalHistoryPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
    pub show_relative_path: bool,
    pub timeline_gap_min_px: Pixels,
    pub timeline_gap_max_px: Pixels,
    pub header_metadata_visibility: LocalHistoryPanelHeaderVisibility,
}

impl Settings for LocalHistoryPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let panel = content.local_history_panel.clone().unwrap_or_default();
        let min_gap = panel.timeline_gap_min_px.unwrap_or(0.0).max(0.0);
        let max_gap = panel.timeline_gap_max_px.unwrap_or(12.0).max(0.0);
        let (min_gap, max_gap) = if max_gap < min_gap {
            (max_gap, min_gap)
        } else {
            (min_gap, max_gap)
        };
        let header_metadata_visibility = panel
            .header_metadata_visibility
            .unwrap_or(LocalHistoryPanelHeaderVisibility::Auto);
        Self {
            button: panel.button.unwrap_or(true),
            dock: panel.dock.unwrap_or(SettingsDockPosition::Right).into(),
            default_width: panel.default_width.map(px).unwrap_or_else(|| px(300.)),
            show_relative_path: panel.show_relative_path.unwrap_or(false),
            timeline_gap_min_px: px(min_gap),
            timeline_gap_max_px: px(max_gap),
            header_metadata_visibility,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedLocalHistoryPanel {
    width: Option<Pixels>,
    active: Option<bool>,
}

pub enum Event {
    Focus,
}

pub struct LocalHistoryPanel {
    fs: Arc<dyn project::Fs>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    width: Option<Pixels>,
    active: bool,
    focus_handle: FocusHandle,
    entries: Vec<LocalHistoryEntry>,
    entry_gaps: Vec<Pixels>,
    list_state: ListState,
    active_project_path: Option<ProjectPath>,
    active_path_display: Option<String>,
    header_hovered: bool,
    header_shift_pressed: bool,
    last_opened_entry_ids: Vec<String>,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            toggle_local_history_panel(workspace, window, cx);
        });
    })
    .detach();
}

fn toggle_local_history_panel(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let dock_position = workspace
        .panel::<LocalHistoryPanel>(cx)
        .map(|panel| panel.read(cx).position(window, cx));

    let should_hide_panel = dock_position
        .map(|dock_position| {
            let dock = workspace.dock_at_position(dock_position).read(cx);
            dock.is_open()
                && dock
                    .active_panel()
                    .is_some_and(|panel| panel.panel_key() == LOCAL_HISTORY_PANEL_KEY)
        })
        .unwrap_or(false);

    if let Some(dock_position) = dock_position.filter(|_| should_hide_panel) {
        workspace.toggle_dock(dock_position, window, cx);
    } else {
        workspace.toggle_panel_focus::<LocalHistoryPanel>(window, cx);
    }
}

impl LocalHistoryPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let serialized_panel = cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(LOCAL_HISTORY_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
                .map(|panel| serde_json::from_str::<SerializedLocalHistoryPanel>(&panel))
                .transpose()
                .log_err()
                .flatten();

            workspace.update_in(cx, |workspace, window, cx| {
                let panel = Self::new(workspace, window, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width.map(|w: Pixels| w.round());
                        panel.active = serialized_panel.active.unwrap_or(false);
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let workspace_handle = cx.entity().downgrade();
        let workspace_for_subscription = workspace_handle.clone();
        let fs = workspace.app_state().fs.clone();

        cx.new(|cx| {
            let list_state = ListState::new(0, ListAlignment::Top, px(28.));
            let focus_handle = cx.focus_handle();

            let mut panel = Self {
                fs,
                project: project.clone(),
                workspace: workspace_handle,
                width: None,
                active: false,
                focus_handle,
                entries: Vec::new(),
                entry_gaps: Vec::new(),
                list_state,
                active_project_path: None,
                active_path_display: None,
                header_hovered: false,
                header_shift_pressed: false,
                last_opened_entry_ids: Vec::new(),
                pending_serialization: Task::ready(None),
                subscriptions: Vec::new(),
            };

            if let Some(workspace_entity) = workspace_for_subscription.upgrade() {
                panel.subscriptions.push(cx.subscribe_in(
                    &workspace_entity,
                    window,
                    |panel: &mut Self, workspace, event, window, cx| {
                        if matches!(event, workspace::Event::ActiveItemChanged) {
                            let (new_path, new_display) = {
                                let workspace = workspace.read(cx);
                                Self::resolve_active_context(&workspace, cx)
                            };
                            panel.update_active_context(new_path, new_display, window, cx);
                        }
                    },
                ));
            }

            let local_history_store = project.read(cx).local_history_store().clone();
            panel.subscriptions.push(cx.subscribe_in(
                &local_history_store,
                window,
                |panel: &mut Self, _, event, window, cx| match event {
                    LocalHistoryEvent::EntriesUpdated => {
                        panel.refresh_entries(window, cx);
                    }
                },
            ));

            panel
                .subscriptions
                .push(cx.observe_global_in::<SettingsStore>(
                    window,
                    |panel: &mut Self, window, cx| {
                        panel.refresh_entries(window, cx);
                    },
                ));

            let (new_path, new_display) = Self::resolve_active_context(workspace, cx);
            panel.update_active_context(new_path, new_display, window, cx);
            panel
        })
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let active = self.active;
        self.pending_serialization = cx.background_spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        LOCAL_HISTORY_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedLocalHistoryPanel {
                            width,
                            active: Some(active),
                        })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn show_toast(&self, message: impl Into<Cow<'static, str>>, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let message = message.into();
        workspace.update(cx, move |workspace, cx| {
            struct LocalHistoryToast;
            workspace.show_toast(
                Toast::new(NotificationId::unique::<LocalHistoryToast>(), message).autohide(),
                cx,
            );
        });
    }

    fn resolve_active_context(
        workspace: &Workspace,
        cx: &App,
    ) -> (Option<ProjectPath>, Option<String>) {
        let new_path = current_project_path(workspace, cx);
        let new_display = new_path.as_ref().and_then(|path| {
            workspace
                .project()
                .read(cx)
                .worktree_for_id(path.worktree_id, cx)
                .map(|worktree| {
                    let style = worktree.read(cx).path_style();
                    path.path.display(style).to_string()
                })
        });
        (new_path, new_display)
    }

    fn update_active_context(
        &mut self,
        new_path: Option<ProjectPath>,
        new_display: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if new_path.is_none() && self.active_project_path.is_some() {
            return;
        }
        let new_path = new_path;
        let new_display = new_display;

        if self.active_project_path != new_path {
            self.active_project_path = new_path;
            self.active_path_display = new_display;
            self.refresh_entries(window, cx);
        }
    }

    fn refresh_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(project_path) = self.active_project_path.clone() else {
            self.set_entries(Vec::new(), cx);
            return;
        };

        let local_history_task = self
            .project
            .read(cx)
            .local_history_store()
            .read(cx)
            .load_entries_for_path(project_path, cx);

        let panel = cx.weak_entity();
        cx.spawn_in(window, async move |_, cx| {
            let entries = local_history_task.await?;
            if let Some(panel) = panel.upgrade() {
                panel.update(cx, |panel, cx| {
                    panel.set_entries(entries, cx);
                });
            }
            anyhow::Ok(())
        })
        .detach();
    }

    fn set_entries(&mut self, entries: Vec<LocalHistoryEntry>, cx: &mut Context<Self>) {
        let old_count = self.entries.len();
        let new_count = entries.len();
        let panel_settings = LocalHistoryPanelSettings::get_global(cx);
        self.entry_gaps = compute_entry_gaps(
            &entries,
            panel_settings.timeline_gap_min_px,
            panel_settings.timeline_gap_max_px,
        );
        self.entries = entries;
        self.list_state.splice(0..old_count, new_count);
        cx.notify();
    }

    fn add_endpoint(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let prompt = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Select local history directory".into()),
        });
        let fs = self.fs.clone();
        let panel = cx.weak_entity();
        cx.spawn_in(window, async move |_, cx| {
            let Some(mut paths) = prompt.await.ok().and_then(|result| result.ok()).flatten() else {
                return anyhow::Ok(());
            };

            if let Some(path) = paths.pop() {
                let path_string = path.to_string_lossy().to_string();
                if let Some(panel) = panel.upgrade() {
                    panel.update(cx, |_, cx| {
                        settings::update_settings_file(fs.clone(), cx, |settings, _| {
                            let local_history = settings.local_history.get_or_insert_default();
                            let list = local_history.storage_paths.get_or_insert_with(Vec::new);
                            if !list.contains(&path_string) {
                                list.push(path_string.clone());
                            }
                            local_history.active_storage_path = Some(path_string);
                        });
                    });
                }
            }
            anyhow::Ok(())
        })
        .detach();
    }

    fn set_active_endpoint(&mut self, path: String, cx: &mut Context<Self>) {
        let fs = self.fs.clone();
        settings::update_settings_file(fs, cx, move |settings, _| {
            let local_history = settings.local_history.get_or_insert_default();
            local_history.active_storage_path = Some(path);
        });
    }

    fn remove_endpoint(&mut self, path: String, cx: &mut Context<Self>) {
        let fs = self.fs.clone();
        settings::update_settings_file(fs, cx, move |settings, _| {
            let local_history = settings.local_history.get_or_insert_default();
            if let Some(storage_paths) = local_history.storage_paths.as_mut() {
                storage_paths.retain(|entry| entry != &path);
            }
            if local_history
                .active_storage_path
                .as_ref()
                .is_some_and(|active| active == &path)
            {
                local_history.active_storage_path = None;
            }
        });
    }

    fn transfer_history(
        &mut self,
        source: PathBuf,
        mode: LocalHistoryTransferMode,
        cx: &mut Context<Self>,
    ) {
        let store = self.project.read(cx).local_history_store().clone();
        let destination = LocalHistorySettings::get_global(cx).resolved_active_path();
        let task = store
            .read(cx)
            .transfer_history(source, destination, mode, cx);
        task.detach_and_log_err(cx);
    }

    fn cleanup_active_worktree(&mut self, cx: &mut Context<Self>) {
        let Some(project_path) = self.active_project_path.clone() else {
            return;
        };
        let store = self.project.read(cx).local_history_store().clone();
        let task = store.read(cx).prune_active_worktree(project_path, cx);
        task.detach_and_log_err(cx);
    }

    fn update_header_hovered(&mut self, hovered: bool, cx: &mut Context<Self>) {
        if self.header_hovered != hovered {
            self.header_hovered = hovered;
            cx.notify();
        }
    }

    fn mark_entry_opened(&mut self, entry_id: &str, cx: &mut Context<Self>) {
        if let Some(pos) = self
            .last_opened_entry_ids
            .iter()
            .position(|id| id == entry_id)
        {
            self.last_opened_entry_ids.remove(pos);
        }
        self.last_opened_entry_ids.insert(0, entry_id.to_string());
        self.last_opened_entry_ids.truncate(3);
        cx.notify();
    }

    fn is_entry_recently_opened(&self, entry_id: &str) -> bool {
        self.last_opened_entry_ids.iter().any(|id| id == entry_id)
    }

    fn open_entry_diff(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(ix).cloned() else {
            self.show_toast("Local history entry not found.", cx);
            return;
        };
        let Some(project_path) = self.active_project_path.clone() else {
            self.show_toast("No active file for local history.", cx);
            return;
        };
        self.mark_entry_opened(&entry.id, cx);
        self.show_toast("Opening local history diff…", cx);
        let project = self.project.clone();
        let workspace = self.workspace.clone();
        let store = project.read(cx).local_history_store().clone();

        let load_task = store.read(cx).load_entry_text(entry.clone(), cx);

        cx.spawn_in(window, async move |_, cx| {
            let base_text = match load_task.await {
                Ok(text) => text,
                Err(err) => {
                    if let Some(workspace) = workspace.upgrade() {
                        let message = format!("Local history diff failed: {err}");
                        workspace.update(cx, move |workspace, cx| {
                            struct LocalHistoryLoadError;
                            workspace.show_toast(
                                Toast::new(
                                    NotificationId::unique::<LocalHistoryLoadError>(),
                                    message,
                                ),
                                cx,
                            );
                        });
                    }
                    return Err(err);
                }
            };
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await
                .context("opening buffer for local history diff")?;

            if let Some(workspace) = workspace.upgrade() {
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        let file_name = buffer
                            .read(cx)
                            .file()
                            .and_then(|file| {
                                file.full_path(cx)
                                    .file_name()
                                    .map(|name| name.to_string_lossy().to_string())
                            })
                            .unwrap_or_else(|| "untitled".into());
                        let title = SharedString::from(format!("Local History: {file_name}"));
                        let diff_view = cx.new(|cx| {
                            LocalHistoryDiffView::new(
                                buffer.clone(),
                                base_text.clone(),
                                title,
                                entry.timestamp,
                                project.clone(),
                                window,
                                cx,
                            )
                        });
                        workspace.active_pane().update(cx, |pane, cx| {
                            pane.add_item(
                                Box::new(diff_view.clone()),
                                true,
                                true,
                                None,
                                window,
                                cx,
                            );
                        });
                        diff_view
                    })
                    .ok();
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn restore_entry(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.entries.get(ix).cloned() else {
            self.show_toast("Local history entry not found.", cx);
            return;
        };
        let Some(project_path) = self.active_project_path.clone() else {
            self.show_toast("No active file for local history.", cx);
            return;
        };
        self.mark_entry_opened(&entry.id, cx);
        self.show_toast("Restoring local history snapshot…", cx);
        let project = self.project.clone();
        let store = project.read(cx).local_history_store().clone();
        let load_task = store.read(cx).load_entry_text(entry, cx);

        cx.spawn_in(window, async move |_, cx| {
            let text = load_task
                .await
                .context("loading local history snapshot for restore")?;
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await
                .context("opening buffer for local history restore")?;
            buffer.update(cx, |buffer, cx| {
                buffer.set_text(text, cx);
            });
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn render_entry(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let entry = self.entries.get(ix)?.clone();
        self.render_local_history_entry(ix, entry, cx)
    }

    fn render_local_history_entry(
        &mut self,
        ix: usize,
        entry: LocalHistoryEntry,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let now = OffsetDateTime::now_utc();
        let timestamp = format_relative_compact(entry.timestamp, now);
        let relative_path: SharedString = entry.relative_path.to_string().into();
        let is_recently_opened = self.is_entry_recently_opened(&entry.id);
        let show_relative_path = LocalHistoryPanelSettings::get_global(cx).show_relative_path;

        let open_diff_row = cx.listener(move |panel: &mut Self, event: &ClickEvent, window, cx| {
            if event.modifiers().platform {
                panel.restore_entry(ix, window, cx);
            } else {
                panel.open_entry_diff(ix, window, cx);
            }
        });
        let restore = cx.listener(move |panel: &mut Self, _event: &ClickEvent, window, cx| {
            panel.restore_entry(ix, window, cx);
        });

        let gap = self.entry_gaps.get(ix).copied().unwrap_or_else(|| px(0.));
        let is_dark = !cx.theme().appearance().is_light();
        let entry_bg = if is_dark {
            black()
        } else {
            cx.theme().colors().panel_background
        };
        let gap_bg = if is_dark {
            cx.theme().colors().panel_background
        } else {
            entry_bg
        };

        let item = ListItem::new(format!("local-history-entry-{}", entry.id))
            .spacing(ListItemSpacing::Dense)
            .start_slot(
                h_flex().h_full().items_center().child(
                    Icon::new(IconName::HistoryRerun)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                ),
            )
            .on_click(open_diff_row)
            .end_hover_slot(
                h_flex().h_full().items_center().gap_1().child(
                    IconButton::new(
                        format!("local-history-restore-{}", entry.id),
                        IconName::Undo,
                    )
                    .icon_size(IconSize::XSmall)
                    .on_click(restore)
                    .tooltip(Tooltip::text("Restore (Cmd-Click entry)")),
                ),
            )
            .child(
                h_flex().h_full().items_center().child(
                    v_flex()
                        .gap_0()
                        .justify_center()
                        .child(
                            h_flex()
                                .items_center()
                                .gap_1()
                                .child(Label::new("Saved").size(LOCAL_HISTORY_ENTRY_LABEL_SIZE))
                                .child(
                                    Label::new(timestamp)
                                        .size(LOCAL_HISTORY_ENTRY_LABEL_SIZE)
                                        .color(Color::Muted),
                                )
                                .when(is_recently_opened, |row| {
                                    row.child(
                                        Label::new("Last")
                                            .size(LOCAL_HISTORY_ENTRY_LABEL_SIZE)
                                            .color(Color::Hint),
                                    )
                                }),
                        )
                        .when(show_relative_path, |column| {
                            column.child(
                                Label::new(relative_path)
                                    .size(LOCAL_HISTORY_ENTRY_LABEL_SIZE)
                                    .color(Color::Muted),
                            )
                        }),
                ),
            );

        let item = div().w_full().bg(entry_bg).child(item);
        if f32::from(gap) > 0.0 {
            Some(div().pt(gap).bg(gap_bg).child(item).into_any_element())
        } else {
            Some(item.into_any_element())
        }
    }

    fn render_header_reveal_zone(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let panel_settings = LocalHistoryPanelSettings::get_global(cx);
        if panel_settings.header_metadata_visibility != LocalHistoryPanelHeaderVisibility::Auto {
            return None;
        }

        let header_hover = cx.listener(|panel: &mut Self, hovered: &bool, _window, cx| {
            panel.update_header_hovered(*hovered, cx);
        });

        Some(
            div()
                .id("local-history-header-reveal-zone")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .right(px(0.))
                .h(px(16.))
                .on_hover(header_hover)
                .into_any_element(),
        )
    }

    fn render_header(&self, _window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let panel_settings = LocalHistoryPanelSettings::get_global(cx);
        let show_header = match panel_settings.header_metadata_visibility {
            LocalHistoryPanelHeaderVisibility::Always => true,
            LocalHistoryPanelHeaderVisibility::Never => false,
            LocalHistoryPanelHeaderVisibility::Auto => {
                self.header_hovered && self.header_shift_pressed
            }
        };
        if !show_header {
            return None;
        }

        let header_hover = cx.listener(|panel: &mut Self, hovered: &bool, _window, cx| {
            panel.update_header_hovered(*hovered, cx);
        });

        let is_dark = !cx.theme().appearance().is_light();
        let header_bg = if is_dark {
            black()
        } else {
            cx.theme().colors().panel_background
        };

        let settings = LocalHistorySettings::get_global(cx);
        let active_root = settings.resolved_active_path();
        let endpoints = settings.resolved_storage_paths();
        let active_root_display = active_root.compact().to_string_lossy().into_owned();
        let endpoint_label = if endpoints.len() > 1 {
            format!(
                "Storage: {active_root_display} (+{} more)",
                endpoints.len() - 1
            )
        } else {
            format!("Storage: {active_root_display}")
        };

        let panel = cx.weak_entity();
        let endpoints_for_menu = endpoints.clone();
        let active_for_menu = active_root.clone();

        let menu = PopoverMenu::new("local-history-menu")
            .trigger_with_tooltip(
                IconButton::new("local-history-settings", IconName::Settings)
                    .icon_size(IconSize::Small),
                Tooltip::text("Local history"),
            )
            .anchor(Corner::TopRight)
            .menu(move |window, cx| {
                let panel = panel.clone();
                let endpoints = endpoints_for_menu.clone();
                let active_for_menu = active_for_menu.clone();
                let menu = ContextMenu::build(window, cx, move |menu, _window, _cx| {
                    let panel_add = panel.clone();
                    let endpoints_for_set = endpoints.clone();
                    let panel_set = panel.clone();
                    let endpoints_for_remove = endpoints.clone();
                    let panel_remove = panel.clone();
                    let endpoints_for_copy = endpoints.clone();
                    let panel_copy = panel.clone();
                    let active_for_copy = active_for_menu.clone();
                    let endpoints_for_move = endpoints.clone();
                    let panel_move = panel.clone();
                    let active_for_move = active_for_menu.clone();
                    let panel_cleanup = panel.clone();

                    let menu = menu
                        .entry("Add Endpoint…", None, move |window, cx| {
                            if let Some(panel) = panel_add.upgrade() {
                                panel.update(cx, |panel, cx| panel.add_endpoint(window, cx));
                            }
                        })
                        .separator()
                        .submenu("Set Active Endpoint", move |menu, _, _| {
                            let mut menu = menu;
                            for endpoint in &endpoints_for_set {
                                let label = endpoint.compact().to_string_lossy().into_owned();
                                let endpoint_string = endpoint.to_string_lossy().into_owned();
                                let panel = panel_set.clone();
                                menu = menu.entry(label, None, move |_, cx| {
                                    if let Some(panel) = panel.upgrade() {
                                        panel.update(cx, |panel, cx| {
                                            panel.set_active_endpoint(endpoint_string.clone(), cx);
                                        });
                                    }
                                });
                            }
                            menu
                        })
                        .submenu("Remove Endpoint", move |menu, _, _| {
                            let mut menu = menu;
                            for endpoint in &endpoints_for_remove {
                                let label = endpoint.compact().to_string_lossy().into_owned();
                                let endpoint_string = endpoint.to_string_lossy().into_owned();
                                let panel = panel_remove.clone();
                                menu = menu.entry(label, None, move |_, cx| {
                                    if let Some(panel) = panel.upgrade() {
                                        panel.update(cx, |panel, cx| {
                                            panel.remove_endpoint(endpoint_string.clone(), cx);
                                        });
                                    }
                                });
                            }
                            menu
                        })
                        .separator()
                        .submenu("Copy History From…", move |menu, _, _| {
                            let mut menu = menu;
                            for endpoint in &endpoints_for_copy {
                                if endpoint == &active_for_copy {
                                    continue;
                                }
                                let label = endpoint.compact().to_string_lossy().into_owned();
                                let endpoint_path = endpoint.clone();
                                let panel = panel_copy.clone();
                                menu = menu.entry(label, None, move |_, cx| {
                                    if let Some(panel) = panel.upgrade() {
                                        panel.update(cx, |panel, cx| {
                                            panel.transfer_history(
                                                endpoint_path.clone(),
                                                LocalHistoryTransferMode::Copy,
                                                cx,
                                            );
                                        });
                                    }
                                });
                            }
                            menu
                        })
                        .submenu("Move History From…", move |menu, _, _| {
                            let mut menu = menu;
                            for endpoint in &endpoints_for_move {
                                if endpoint == &active_for_move {
                                    continue;
                                }
                                let label = endpoint.compact().to_string_lossy().into_owned();
                                let endpoint_path = endpoint.clone();
                                let panel = panel_move.clone();
                                menu = menu.entry(label, None, move |_, cx| {
                                    if let Some(panel) = panel.upgrade() {
                                        panel.update(cx, |panel, cx| {
                                            panel.transfer_history(
                                                endpoint_path.clone(),
                                                LocalHistoryTransferMode::Move,
                                                cx,
                                            );
                                        });
                                    }
                                });
                            }
                            menu
                        })
                        .separator()
                        .entry("Clean Up Now", None, move |_, cx| {
                            if let Some(panel) = panel_cleanup.upgrade() {
                                panel.update(cx, |panel, cx| panel.cleanup_active_worktree(cx));
                            }
                        });
                    menu
                });
                Some(menu)
            });

        Some(
            div()
                .id("local-history-header")
                .absolute()
                .top(px(0.))
                .left(px(0.))
                .right(px(0.))
                .on_hover(header_hover)
                .bg(header_bg)
                .child(
                    v_flex()
                        .child(
                            h_flex()
                                .h(Tab::container_height(cx))
                                .items_center()
                                .justify_between()
                                .px_2()
                                .child(Label::new("Timeline").size(LabelSize::Small))
                                .child(menu),
                        )
                        .child(
                            h_flex().px_2().pb_1().child(
                                Label::new(endpoint_label)
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                        ),
                )
                .into_any_element(),
        )
    }
}

impl Render for LocalHistoryPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = self.render_header(window, cx);
        let header_reveal_zone = self.render_header_reveal_zone(cx);
        let is_dark = !cx.theme().appearance().is_light();
        let panel_bg = if is_dark {
            black()
        } else {
            cx.theme().colors().panel_background
        };
        v_flex()
            .id("local-history-panel")
            .size_full()
            .track_focus(&self.focus_handle)
            .bg(panel_bg)
            .on_modifiers_changed(cx.listener(
                |panel, event: &ModifiersChangedEvent, _window, cx| {
                    let shift_pressed = event.modifiers.shift;
                    if panel.header_shift_pressed != shift_pressed {
                        panel.header_shift_pressed = shift_pressed;
                        cx.notify();
                    }
                },
            ))
            .child(
                div()
                    .size_full()
                    .relative()
                    .child(div().size_full().overflow_hidden().map(|content| {
                        if self.active_project_path.is_none() {
                            content.child(
                                div().p_3().child(
                                    Label::new("Focus an editor to view timeline.")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                            )
                        } else if self.entries.is_empty() {
                            content.child(
                                div().p_3().child(
                                    Label::new("No history entries yet.")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                            )
                        } else {
                            content.child(
                                list(
                                    self.list_state.clone(),
                                    cx.processor(|panel, ix, window, cx| {
                                        panel
                                            .render_entry(ix, window, cx)
                                            .unwrap_or_else(|| div().into_any_element())
                                    }),
                                )
                                .size_full(),
                            )
                        }
                    }))
                    .when_some(header_reveal_zone, |container, zone| container.child(zone))
                    .when_some(header, |container, header| container.child(header)),
            )
    }
}

impl Focusable for LocalHistoryPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<Event> for LocalHistoryPanel {}
impl EventEmitter<PanelEvent> for LocalHistoryPanel {}

impl Panel for LocalHistoryPanel {
    fn persistent_name() -> &'static str {
        "Local History Panel"
    }

    fn panel_key() -> &'static str {
        LOCAL_HISTORY_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        LocalHistoryPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.local_history_panel.get_or_insert_default().dock = Some(position.into())
        });
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| LocalHistoryPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.active = active;
        if self.active {
            cx.emit(Event::Focus);
        }
        self.serialize(cx);
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        LocalHistoryPanelSettings::get_global(cx)
            .button
            .then_some(IconName::HistoryRerun)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Local History")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        7
    }

    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        self.active
    }
}

struct LocalHistoryDiffView {
    editor: Entity<Editor>,
    buffer: Entity<Buffer>,
    title: SharedString,
    timestamp: OffsetDateTime,
}

impl LocalHistoryDiffView {
    fn new(
        buffer: Entity<Buffer>,
        base_text: Arc<str>,
        title: SharedString,
        timestamp: OffsetDateTime,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let snapshot = buffer.read(cx).snapshot();
        let diff = cx.new(|cx| BufferDiff::new(&snapshot.text, cx));
        diff.update(cx, |diff, cx| {
            let _ = diff.set_base_text(
                Some(base_text.clone()),
                snapshot.language().cloned(),
                snapshot.text.clone(),
                cx,
            );
        });

        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::singleton(buffer.clone(), cx);
            multibuffer.add_diff(diff.clone(), cx);
            multibuffer
        });

        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer.clone(), Some(project.clone()), window, cx);
            editor.start_temporary_diff_override();
            editor.disable_diagnostics(cx);
            editor.set_expand_all_diff_hunks(cx);
            editor.set_render_diff_hunk_controls(
                Arc::new(|_, _, _, _, _, _, _, _| gpui::Empty.into_any_element()),
                cx,
            );
            editor
        });

        Self {
            editor,
            buffer,
            title,
            timestamp,
        }
    }
}

impl EventEmitter<EditorEvent> for LocalHistoryDiffView {}

impl Focusable for LocalHistoryDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for LocalHistoryDiffView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Diff).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.title.clone()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let path = self
            .buffer
            .read(cx)
            .file()
            .map(|file| file.full_path(cx).compact().to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".into());
        let time = format_local_timestamp(
            self.timestamp,
            OffsetDateTime::now_utc(),
            TimestampFormat::EnhancedAbsolute,
        );
        Some(format!("{path} • {time}").into())
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Local History Diff Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: std::any::TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == std::any::TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == std::any::TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.editor.for_each_project_item(cx, f)
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn std::any::Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn breadcrumb_location(&self, _: &App) -> workspace::ToolbarItemLocation {
        workspace::ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn can_save(&self, cx: &App) -> bool {
        self.editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.editor
            .update(cx, |editor, cx| editor.save(options, project, window, cx))
    }
}

impl Render for LocalHistoryDiffView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}

fn current_project_path(workspace: &Workspace, cx: &App) -> Option<ProjectPath> {
    let active_item = workspace.active_item(cx)?;
    let active_editor = active_item
        .act_as::<Editor>(cx)
        .filter(|editor| editor.read(cx).mode().is_full())?;
    let (_, buffer, _) = active_editor.read(cx).active_excerpt(cx)?;
    let file = buffer.read(cx).file()?.clone();
    let project_path = ProjectPath::from_file(file.as_ref(), cx);
    Some(project_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use settings::SettingsStore;
    use tempfile::tempdir;
    use time::Duration;
    use util::path;
    use workspace::Workspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
        });
    }

    fn configure_local_history_path(cx: &mut TestAppContext, path: String) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |content| {
                    content.local_history = Some(settings::LocalHistorySettingsContent {
                        enabled: Some(true),
                        capture_on_save: Some(true),
                        capture_on_edit_idle_ms: Some(0.into()),
                        capture_on_focus_change: Some(false),
                        capture_on_window_change: Some(false),
                        capture_on_task: Some(false),
                        capture_on_external_change: Some(false),
                        storage_paths: Some(vec![path.clone()]),
                        active_storage_path: Some(path.clone()),
                        ..Default::default()
                    });
                });
            });
        });
    }

    #[gpui::test]
    async fn test_open_diff_from_local_history_entry(cx: &mut TestAppContext) {
        init_test(cx);

        let temp = tempdir().unwrap();
        let history_root = temp.path().to_path_buf();
        configure_local_history_path(cx, history_root.to_string_lossy().to_string());

        let snapshot_text = "old text\n";
        let compressed = zstd::encode_all(snapshot_text.as_bytes(), 0).unwrap();
        let snapshot_rel = PathBuf::from("snapshots/entry.zst");
        std::fs::create_dir_all(history_root.join("snapshots")).unwrap();
        std::fs::write(history_root.join(&snapshot_rel), compressed).unwrap();

        let entry = LocalHistoryEntry {
            id: "entry".into(),
            timestamp: OffsetDateTime::now_utc(),
            relative_path: Arc::from("file.txt"),
            endpoint_root: history_root.clone(),
            snapshot_relative_path: snapshot_rel,
            compressed_bytes: 1,
            uncompressed_bytes: snapshot_text.len() as u64,
        };

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "file.txt": "new text\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(path!("/test/file.txt"), cx))
            .await
            .unwrap();
        let project_path = buffer.read_with(cx, |buffer, cx| {
            let file = buffer.file().unwrap();
            ProjectPath::from_file(file.as_ref(), cx)
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            LocalHistoryPanel::new(workspace, window, cx)
        });

        panel.update(cx, |panel, _cx| {
            panel.entries = vec![entry];
            panel.active_project_path = Some(project_path);
            panel.active_path_display = Some("file.txt".into());
        });

        cx.update(|window, cx| {
            panel.update(cx, |panel, cx| panel.open_entry_diff(0, window, cx));
        });

        cx.run_until_parked();

        let diff_count = workspace.read_with(cx, |workspace, cx| {
            workspace.items_of_type::<LocalHistoryDiffView>(cx).count()
        });
        assert_eq!(diff_count, 1);
    }

    #[gpui::test]
    async fn test_toggle_focus_hides_active_local_history_panel(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "file.txt": "text\n",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            let panel = LocalHistoryPanel::new(workspace, window, cx);
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });

        workspace.update_in(cx, |workspace, window, cx| {
            toggle_local_history_panel(workspace, window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let dock_position = panel.read(cx).position(window, cx);
            let (is_open, active_panel_key) = {
                let dock = workspace.dock_at_position(dock_position).read(cx);
                (
                    dock.is_open(),
                    dock.active_panel().map(|panel| panel.panel_key()),
                )
            };

            assert!(is_open);
            assert_eq!(active_panel_key, Some(LOCAL_HISTORY_PANEL_KEY));
            assert!(panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });

        let active_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
        active_pane.update_in(cx, |pane, window, cx| {
            window.focus(&pane.focus_handle(cx), cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let dock_position = panel.read(cx).position(window, cx);
            let (is_open, active_panel_key) = {
                let dock = workspace.dock_at_position(dock_position).read(cx);
                (
                    dock.is_open(),
                    dock.active_panel().map(|panel| panel.panel_key()),
                )
            };

            assert!(is_open);
            assert_eq!(active_panel_key, Some(LOCAL_HISTORY_PANEL_KEY));
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));

            toggle_local_history_panel(workspace, window, cx);
        });

        workspace.update_in(cx, |workspace, window, cx| {
            let dock_position = panel.read(cx).position(window, cx);
            let is_open = workspace.dock_at_position(dock_position).read(cx).is_open();

            assert!(!is_open);
            assert!(!panel.read(cx).focus_handle(cx).contains_focused(window, cx));
        });
    }

    #[gpui::test]
    async fn test_restore_from_local_history_entry(cx: &mut TestAppContext) {
        init_test(cx);

        let temp = tempdir().unwrap();
        let history_root = temp.path().to_path_buf();
        configure_local_history_path(cx, history_root.to_string_lossy().to_string());

        let snapshot_text = "restored text\n";
        let compressed = zstd::encode_all(snapshot_text.as_bytes(), 0).unwrap();
        let snapshot_rel = PathBuf::from("snapshots/restore.zst");
        std::fs::create_dir_all(history_root.join("snapshots")).unwrap();
        std::fs::write(history_root.join(&snapshot_rel), compressed).unwrap();

        let entry = LocalHistoryEntry {
            id: "restore".into(),
            timestamp: OffsetDateTime::now_utc(),
            relative_path: Arc::from("file.txt"),
            endpoint_root: history_root.clone(),
            snapshot_relative_path: snapshot_rel,
            compressed_bytes: 1,
            uncompressed_bytes: snapshot_text.len() as u64,
        };

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "file.txt": "new text\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(path!("/test/file.txt"), cx))
            .await
            .unwrap();
        let project_path = buffer.read_with(cx, |buffer, cx| {
            let file = buffer.file().unwrap();
            ProjectPath::from_file(file.as_ref(), cx)
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            LocalHistoryPanel::new(workspace, window, cx)
        });

        panel.update(cx, |panel, _cx| {
            panel.entries = vec![entry];
            panel.active_project_path = Some(project_path);
            panel.active_path_display = Some("file.txt".into());
        });

        cx.update(|window, cx| {
            panel.update(cx, |panel, cx| panel.restore_entry(0, window, cx));
        });

        cx.run_until_parked();

        let text = buffer.read_with(cx, |buffer, _| buffer.text().to_string());
        assert_eq!(text, snapshot_text);
    }

    #[gpui::test]
    async fn test_local_history_open_diff_missing_snapshot_does_not_create_view(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let temp = tempdir().unwrap();
        let history_root = temp.path().to_path_buf();
        configure_local_history_path(cx, history_root.to_string_lossy().to_string());

        let entry = LocalHistoryEntry {
            id: "missing".into(),
            timestamp: OffsetDateTime::now_utc(),
            relative_path: Arc::from("file.txt"),
            endpoint_root: history_root.clone(),
            snapshot_relative_path: PathBuf::from("snapshots/missing.zst"),
            compressed_bytes: 1,
            uncompressed_bytes: 1,
        };

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "file.txt": "new text\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(path!("/test/file.txt"), cx))
            .await
            .unwrap();
        let project_path = buffer.read_with(cx, |buffer, cx| {
            let file = buffer.file().unwrap();
            ProjectPath::from_file(file.as_ref(), cx)
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            LocalHistoryPanel::new(workspace, window, cx)
        });

        panel.update(cx, |panel, _cx| {
            panel.entries = vec![entry];
            panel.active_project_path = Some(project_path);
            panel.active_path_display = Some("file.txt".into());
        });

        cx.update(|window, cx| {
            panel.update(cx, |panel, cx| panel.open_entry_diff(0, window, cx));
        });

        cx.run_until_parked();

        let diff_count = workspace.read_with(cx, |workspace, cx| {
            workspace.items_of_type::<LocalHistoryDiffView>(cx).count()
        });
        assert_eq!(diff_count, 0);
    }

    #[gpui::test]
    async fn test_local_history_open_diff_corrupt_snapshot_does_not_create_view(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let temp = tempdir().unwrap();
        let history_root = temp.path().to_path_buf();
        configure_local_history_path(cx, history_root.to_string_lossy().to_string());

        let snapshot_rel = PathBuf::from("snapshots/corrupt.zst");
        std::fs::create_dir_all(history_root.join("snapshots")).unwrap();
        std::fs::write(history_root.join(&snapshot_rel), b"not zstd").unwrap();

        let entry = LocalHistoryEntry {
            id: "corrupt".into(),
            timestamp: OffsetDateTime::now_utc(),
            relative_path: Arc::from("file.txt"),
            endpoint_root: history_root.clone(),
            snapshot_relative_path: snapshot_rel,
            compressed_bytes: 1,
            uncompressed_bytes: 1,
        };

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            serde_json::json!({
                "file.txt": "new text\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;
        let buffer = project
            .update(cx, |p, cx| p.open_local_buffer(path!("/test/file.txt"), cx))
            .await
            .unwrap();
        let project_path = buffer.read_with(cx, |buffer, cx| {
            let file = buffer.file().unwrap();
            ProjectPath::from_file(file.as_ref(), cx)
        });

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let panel = workspace.update_in(cx, |workspace, window, cx| {
            LocalHistoryPanel::new(workspace, window, cx)
        });

        panel.update(cx, |panel, _cx| {
            panel.entries = vec![entry];
            panel.active_project_path = Some(project_path);
            panel.active_path_display = Some("file.txt".into());
        });

        cx.update(|window, cx| {
            panel.update(cx, |panel, cx| panel.open_entry_diff(0, window, cx));
        });

        cx.run_until_parked();

        let diff_count = workspace.read_with(cx, |workspace, cx| {
            workspace.items_of_type::<LocalHistoryDiffView>(cx).count()
        });
        assert_eq!(diff_count, 0);
    }

    #[test]
    fn timeline_gap_scales_with_entry_deltas() {
        let now = OffsetDateTime::from_unix_timestamp(1_000).unwrap();
        let entry = |id: &str, timestamp: OffsetDateTime| LocalHistoryEntry {
            id: id.into(),
            timestamp,
            relative_path: Arc::from("file.txt"),
            endpoint_root: PathBuf::from("/tmp"),
            snapshot_relative_path: PathBuf::from("snapshots/entry.zst"),
            compressed_bytes: 1,
            uncompressed_bytes: 1,
        };

        let entries = vec![
            entry("a", now),
            entry("b", now - Duration::seconds(10)),
            entry("c", now - Duration::seconds(110)),
        ];

        let gaps = compute_entry_gaps(&entries, px(0.), px(12.));
        assert_eq!(gaps.len(), 3);
        assert!(f32::from(gaps[0]) <= 0.0);
        assert!(f32::from(gaps[1]) < f32::from(gaps[2]));
    }
}
// (moved into impl LocalHistoryPanel)
