//! The Day Planner Context panel (spec `v4-day-planner-panel.md`): a
//! right-dock panel that follows the active editor item. When that item is a
//! daily note of the current vault, its checklist is parsed into a vertical
//! day grid — timed tasks as duration-scaled blocks, unscheduled tasks as
//! chips. Read-only; the one interaction is reveal-on-click into the editor.

use anyhow::Result;
use chrono::{Local, NaiveDate, Timelike as _};
use editor::{Editor, EditorEvent, RowHighlightOptions, SelectionEffects, scroll::Autoscroll};
use gpui::{
    Action, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    Pixels, Subscription, Task, WeakEntity, Window, actions, div, px, relative,
};
use multi_buffer::MultiBufferRow;
use project::Project;
use std::time::Duration;
use text::{Bias, Point};
use ui::prelude::*;
use ui::{Icon, IconSize, Label};
use util::ResultExt as _;
use workspace::Workspace;
use workspace::dock::{DockPosition, Panel, PanelEvent};

use crate::day_plan::{self, DayPlan, PlacedBlock, PlanItem, parse_day_plan};
use crate::notes::format_date;
use crate::vault::VaultStatus;

const DAY_PLANNER_PANEL_KEY: &str = "BreadPaperDayPlannerPanel";
const HOUR_HEIGHT: f32 = 48.0;
const MIN_BLOCK_PX: f32 = 18.0;
const BLOCK_CAPTION_PX: f32 = 18.0;
const BLOCK_LABEL_LINE_PX: f32 = 16.0;
const GUTTER_WIDTH: f32 = 44.0;
const REPARSE_DEBOUNCE: Duration = Duration::from_millis(150);

/// Marker type isolating the panel's transient reveal highlight from other
/// row-highlight owners in the editor.
enum DayPlannerHighlight {}

actions!(
    breadpaper,
    [
        /// Toggles focus on the BreadPaper day planner panel.
        ToggleDayPlannerFocus
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleDayPlannerFocus, window, cx| {
            workspace.toggle_panel_focus::<DayPlannerPanel>(window, cx);
        });
    })
    .detach();
}

/// The daily note currently mirrored by the panel.
struct ActiveNote {
    editor: WeakEntity<Editor>,
    date: NaiveDate,
    plan: DayPlan,
    _editor_subscription: Subscription,
}

pub struct DayPlannerPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    position: DockPosition,
    vault_status: VaultStatus,
    active: Option<ActiveNote>,
    /// Panel-local UI state: the last clicked block/chip (spec §8).
    selected_item: Option<usize>,
    reparse_task: Option<Task<()>>,
    /// Coarse repaint driver for the "now" line.
    _now_tick: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl DayPlannerPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            DayPlannerPanel::new(workspace, window, cx)
        })
    }

    pub fn new(
        workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let weak_workspace = workspace.weak_handle();
        let workspace_entity = cx.entity();
        cx.new(|cx| {
            let project_subscription =
                cx.subscribe(&project, |this: &mut Self, _, event, cx| {
                    if matches!(
                        event,
                        project::Event::WorktreeAdded(_)
                            | project::Event::WorktreeRemoved(_)
                            | project::Event::WorktreeUpdatedEntries(..)
                    ) {
                        this.refresh_vault_status(cx);
                    }
                });
            let workspace_subscription = cx.subscribe(
                &workspace_entity,
                |this: &mut Self, _, event: &workspace::Event, cx| {
                    if matches!(event, workspace::Event::ActiveItemChanged) {
                        this.update_active_item(false, cx);
                    }
                },
            );
            let now_tick = cx.spawn(async move |this, cx| {
                loop {
                    cx.background_executor().timer(Duration::from_secs(60)).await;
                    if this.update(cx, |_, cx| cx.notify()).is_err() {
                        break;
                    }
                }
            });
            let mut this = Self {
                workspace: weak_workspace,
                project,
                focus_handle: cx.focus_handle(),
                position: DockPosition::Right,
                vault_status: VaultStatus::NotAVault,
                active: None,
                selected_item: None,
                reparse_task: None,
                _now_tick: now_tick,
                _subscriptions: vec![project_subscription, workspace_subscription],
            };
            this.vault_status = this.detect_vault_status(cx);
            // Resolving the active item reads the workspace entity, which is
            // still leased by the `workspace.update_in` that is constructing
            // this panel — reading it here would panic. Defer until the
            // current effect cycle returns the workspace to the app.
            let panel = cx.weak_entity();
            cx.defer(move |cx| {
                panel
                    .update(cx, |this, cx| this.update_active_item(false, cx))
                    .log_err();
            });
            this
        })
    }

    fn detect_vault_status(&self, cx: &App) -> VaultStatus {
        match self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
        {
            Some(root) => crate::vault::Vault::detect(&root),
            None => VaultStatus::NotAVault,
        }
    }

    fn refresh_vault_status(&mut self, cx: &mut Context<Self>) {
        let status = self.detect_vault_status(cx);
        let vault_changed = status != self.vault_status;
        if vault_changed {
            self.vault_status = status;
            cx.notify();
        }
        // The vault config feeds both parsing (heading, default duration)
        // and the note's daily-note-ness (the daily directory), so a
        // changed vault must re-parse even when the active editor is the
        // same.
        self.update_active_item(vault_changed, cx);
    }

    /// Re-resolves the active editor item (spec §9.1): when it is a daily
    /// note of the vault, mirror it; otherwise fall back to the hint state.
    /// `force_reparse` re-parses even when the active note is unchanged,
    /// for when the vault config changed under it.
    fn update_active_item(&mut self, force_reparse: bool, cx: &mut Context<Self>) {
        let resolved = self.resolve_active_daily_note(cx);
        let unchanged = match (&self.active, &resolved) {
            (Some(active), Some((editor, date))) => {
                active.editor.entity_id() == editor.entity_id() && active.date == *date
            }
            (None, None) => true,
            _ => false,
        };
        if unchanged {
            if force_reparse {
                self.reparse(cx);
            }
            return;
        }
        self.clear_transient_highlight(cx);
        self.selected_item = None;
        self.reparse_task = None;
        self.active = resolved.map(|(editor, date)| {
            let editor_subscription =
                cx.subscribe(&editor, |this, _, event: &EditorEvent, cx| {
                    if matches!(event, EditorEvent::BufferEdited) {
                        this.schedule_reparse(cx);
                    }
                });
            let plan = self.parse_editor_plan(&editor, cx);
            ActiveNote {
                editor: editor.downgrade(),
                date,
                plan,
                _editor_subscription: editor_subscription,
            }
        });
        cx.notify();
    }

    fn resolve_active_daily_note(&self, cx: &App) -> Option<(Entity<Editor>, NaiveDate)> {
        let VaultStatus::Valid(vault) = &self.vault_status else {
            return None;
        };
        let workspace = self.workspace.upgrade()?;
        let item = workspace.read(cx).active_item(cx)?;
        let editor = item.downcast::<Editor>()?;
        let project_path = item.project_path(cx)?;
        let abs_path = self.project.read(cx).absolute_path(&project_path, cx)?;
        let date = vault.daily_note_date(&abs_path)?;
        Some((editor, date))
    }

    fn parse_editor_plan(&self, editor: &Entity<Editor>, cx: &App) -> DayPlan {
        let VaultStatus::Valid(vault) = &self.vault_status else {
            return DayPlan::default();
        };
        let text = editor.read(cx).buffer().read(cx).snapshot(cx).text();
        parse_day_plan(&text, &vault.config.day_planner)
    }

    fn schedule_reparse(&mut self, cx: &mut Context<Self>) {
        self.reparse_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(REPARSE_DEBOUNCE).await;
            this.update(cx, |this, cx| this.reparse(cx)).log_err();
        }));
    }

    fn reparse(&mut self, cx: &mut Context<Self>) {
        let Some(active) = &self.active else {
            return;
        };
        let Some(editor) = active.editor.upgrade() else {
            return;
        };
        let plan = self.parse_editor_plan(&editor, cx);
        if let Some(active) = &mut self.active
            && active.plan != plan
        {
            active.plan = plan;
            // Item indices may have shifted; a stale selection would
            // outline the wrong block.
            self.selected_item = None;
            cx.notify();
        }
    }

    fn clear_transient_highlight(&mut self, cx: &mut Context<Self>) {
        if let Some(active) = &self.active
            && let Some(editor) = active.editor.upgrade()
        {
            editor.update(cx, |editor, cx| {
                editor.clear_row_highlights::<DayPlannerHighlight>();
                cx.notify();
            });
        }
    }

    /// Reveal-on-click (spec §8): select + scroll to the item's source line
    /// in the editor and paint the transient row highlight. Never modifies
    /// the note.
    fn reveal_item(&mut self, item_index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active) = &self.active else {
            return;
        };
        let Some(item) = active.plan.items.get(item_index) else {
            return;
        };
        let row = item.row;
        let Some(editor) = active.editor.upgrade() else {
            return;
        };
        editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            // Clip, don't index: the note may have shrunk since the parse.
            let start_point = snapshot.clip_point(Point::new(row, 0), Bias::Left);
            let mut end_point = Point::new(
                start_point.row,
                snapshot.line_len(MultiBufferRow(start_point.row)),
            );
            if end_point == start_point {
                // Force a non-empty range so the row still paints.
                end_point = snapshot.clip_point(Point::new(start_point.row + 1, 0), Bias::Left);
            }
            let start = snapshot.anchor_before(start_point);
            let end = snapshot.anchor_after(end_point);
            editor.clear_row_highlights::<DayPlannerHighlight>();
            editor.highlight_rows::<DayPlannerHighlight>(
                start..end,
                |cx| cx.theme().colors().editor_highlighted_line_background,
                RowHighlightOptions {
                    autoscroll: true,
                    ..Default::default()
                },
                cx,
            );
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::center()).nav_history(true),
                window,
                cx,
                |selections| selections.select_anchor_ranges([start..start]),
            );
            editor.focus_handle(cx).focus(window, cx);
        });
        self.selected_item = Some(item_index);
        cx.notify();
    }

    fn render_hint(&self, text: &'static str) -> Div {
        v_flex().p_3().child(
            Label::new(text)
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
    }

    fn render_planner(&self, cx: &Context<Self>) -> AnyElement {
        let (VaultStatus::Valid(vault), Some(active)) = (&self.vault_status, &self.active)
        else {
            return self
                .render_hint("Open a daily note to see its schedule.")
                .into_any_element();
        };
        let config = &vault.config.day_planner;
        let plan = &active.plan;

        let header = div()
            .px_2()
            .py_1p5()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(Label::new(format_date(active.date, "ddd, MMM D")));

        let mut content = v_flex().size_full().child(header);
        if let Some(strip) = self.render_unscheduled_strip(plan, cx) {
            content = content.child(strip);
        }
        if plan.items.is_empty() {
            content = content.child(self.render_hint(
                "No tasks yet. Add `- [ ] 09:00 – 10:00 Task` under your Day planner heading.",
            ));
        }
        content
            .child(self.render_grid(plan, config, active.date, cx))
            .into_any_element()
    }

    fn render_unscheduled_strip(
        &self,
        plan: &DayPlan,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        let unscheduled: Vec<usize> = plan.unscheduled_indices().collect();
        if unscheduled.is_empty() {
            return None;
        }
        let colors = cx.theme().colors();
        Some(
            h_flex()
                .flex_wrap()
                .gap_1()
                .p_2()
                .border_b_1()
                .border_color(colors.border_variant)
                .children(unscheduled.into_iter().filter_map(|item_index| {
                    let item = plan.items.get(item_index)?;
                    Some(self.render_chip(item_index, item, cx))
                }))
                .into_any_element(),
        )
    }

    fn render_chip(&self, item_index: usize, item: &PlanItem, cx: &Context<Self>) -> AnyElement {
        let colors = cx.theme().colors();
        let selected = self.selected_item == Some(item_index);
        let label = Label::new(if item.label.is_empty() {
            "…".to_string()
        } else {
            item.label.clone()
        })
        .size(LabelSize::Small)
        .truncate();
        let label = if item.done {
            label.strikethrough().color(Color::Muted)
        } else {
            label
        };
        h_flex()
            .id(("breadpaper-day-planner-chip", item_index))
            .max_w_full()
            .gap_1()
            .px_1p5()
            .py_0p5()
            .rounded_sm()
            .border_1()
            .border_color(if selected {
                colors.text_accent
            } else {
                colors.border_variant
            })
            .bg(colors.element_background)
            .cursor_pointer()
            .child(
                Icon::new(if item.done {
                    IconName::TodoComplete
                } else {
                    IconName::TodoPending
                })
                .size(IconSize::XSmall)
                .color(Color::Muted),
            )
            .child(label)
            .on_click(cx.listener(move |this, _, window, cx| {
                this.reveal_item(item_index, window, cx);
            }))
            .into_any_element()
    }

    fn render_grid(
        &self,
        plan: &DayPlan,
        config: &day_plan::DayPlannerConfig,
        date: NaiveDate,
        cx: &Context<Self>,
    ) -> AnyElement {
        let colors = cx.theme().colors();
        let (grid_start, grid_end) = day_plan::grid_bounds(plan, config);
        let min_visual_minutes = (MIN_BLOCK_PX / HOUR_HEIGHT * 60.0).ceil() as u32;
        let blocks = day_plan::layout_blocks(plan, min_visual_minutes);
        let total_height = (grid_end - grid_start) as f32 / 60.0 * HOUR_HEIGHT;
        let offset = |minutes: u32| {
            px((minutes.saturating_sub(grid_start)) as f32 / 60.0 * HOUR_HEIGHT)
        };

        let mut body = div().relative().w_full().h(px(total_height));
        for hour in grid_start / 60..grid_end / 60 {
            let minutes = hour * 60;
            body = body
                .child(
                    div()
                        .absolute()
                        .top(offset(minutes))
                        .left(px(GUTTER_WIDTH))
                        .right_0()
                        .h(px(1.0))
                        .bg(colors.border_variant),
                )
                .child(
                    h_flex()
                        .absolute()
                        .top(if minutes == grid_start {
                            offset(minutes)
                        } else {
                            offset(minutes) - px(8.0)
                        })
                        .left_0()
                        .w(px(GUTTER_WIDTH - 6.0))
                        .justify_end()
                        .child(
                            Label::new(hour.to_string())
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                );
        }

        let block_area = div()
            .absolute()
            .top_0()
            .bottom_0()
            .left(px(GUTTER_WIDTH))
            .right_0()
            .children(blocks.iter().filter_map(|block| {
                let item = plan.items.get(block.item_index)?;
                Some(self.render_block(block, item, grid_start, cx))
            }));
        body = body.child(block_area);

        if let Some(now_minutes) = self.now_line_minutes(config, date)
            && (grid_start..=grid_end).contains(&now_minutes)
        {
            let accent = colors.text_accent;
            body = body
                .child(
                    div()
                        .absolute()
                        .top(offset(now_minutes) - px(1.0))
                        .left(px(GUTTER_WIDTH - 2.0))
                        .right_0()
                        .h(px(2.0))
                        .bg(accent),
                )
                .child(
                    div()
                        .absolute()
                        .top(offset(now_minutes) - px(3.0))
                        .left(px(GUTTER_WIDTH - 5.0))
                        .size(px(6.0))
                        .rounded_full()
                        .bg(accent),
                );
        }

        div()
            .id("breadpaper-day-planner-grid")
            .flex_1()
            .overflow_y_scroll()
            .child(body)
            .into_any_element()
    }

    /// Minutes since midnight for the "now" line, when it should be drawn:
    /// only on today's note, and only when enabled (spec §7.4).
    fn now_line_minutes(
        &self,
        config: &day_plan::DayPlannerConfig,
        date: NaiveDate,
    ) -> Option<u32> {
        if !config.show_now_indicator {
            return None;
        }
        let now = Local::now();
        (now.date_naive() == date).then(|| now.hour() * 60 + now.minute())
    }

    fn render_block(
        &self,
        block: &PlacedBlock,
        item: &PlanItem,
        grid_start: u32,
        cx: &Context<Self>,
    ) -> AnyElement {
        let colors = cx.theme().colors();
        let accent = colors.text_accent;
        let item_index = block.item_index;
        let selected = self.selected_item == Some(item_index);
        let top =
            px(block.start_min.saturating_sub(grid_start) as f32 / 60.0 * HOUR_HEIGHT);
        let height = px(
            ((block.end_min - block.start_min) as f32 / 60.0 * HOUR_HEIGHT).max(MIN_BLOCK_PX),
        );
        let width = 1.0 / block.column_count as f32;
        let left = block.column as f32 * width;
        // The label wins over the time caption when the block is too short
        // for both: the caption is dropped unless it fits alongside at least
        // one line of label text (blocks with no label keep the caption).
        let has_label = !item.label.is_empty();
        let show_caption = !has_label
            || f32::from(height) >= BLOCK_CAPTION_PX + BLOCK_LABEL_LINE_PX;
        // Lines of wrapped label text that fit in the remaining height, so
        // the last visible line gets an ellipsis instead of a hard clip.
        let label_height = if show_caption {
            f32::from(height) - BLOCK_CAPTION_PX
        } else {
            f32::from(height)
        };
        let label_lines = ((label_height / BLOCK_LABEL_LINE_PX).floor() as usize).max(1);
        let (fill, border) = if item.done {
            (colors.text_muted.opacity(0.08), colors.border_variant)
        } else {
            (accent.opacity(0.15), accent.opacity(0.4))
        };
        let caption = format!(
            "{} – {}",
            format_minutes(block.start_min),
            format_minutes(block.end_min)
        );

        let label = (!item.label.is_empty()).then(|| {
            let label = Label::new(item.label.clone()).size(LabelSize::Small);
            let label = if item.done {
                label.strikethrough().color(Color::Muted)
            } else {
                label
            };
            // text_ellipsis supplies the "…" affix; line_clamp alone
            // silently drops overflowing lines.
            div().line_clamp(label_lines).text_ellipsis().child(label)
        });

        div()
            .absolute()
            .top(top)
            .left(relative(left))
            .w(relative(width))
            .h(height)
            .px(px(1.0))
            .child(
                v_flex()
                    .id(("breadpaper-day-planner-block", item_index))
                    .size_full()
                    .rounded_sm()
                    .overflow_hidden()
                    .bg(fill)
                    .border_1()
                    .border_color(if selected { accent } else { border })
                    .px_1()
                    .cursor_pointer()
                    .when(show_caption, |this| {
                        this.child(
                            Label::new(caption)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .children(label)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.reveal_item(item_index, window, cx);
                    })),
            )
            .into_any_element()
    }
}

fn format_minutes(minutes: u32) -> String {
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

impl Render for DayPlannerPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("BreadPaperDayPlannerPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .child(self.render_planner(cx))
    }
}

impl EventEmitter<PanelEvent> for DayPlannerPanel {}

impl Focusable for DayPlannerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DayPlannerPanel {
    fn persistent_name() -> &'static str {
        "BreadPaper Day Planner Panel"
    }

    fn panel_key() -> &'static str {
        DAY_PLANNER_PANEL_KEY
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(320.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::ListTodo)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Day Planner Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        ToggleDayPlannerFocus.boxed_clone()
    }

    fn activation_priority(&self) -> u32 {
        // Must be unique across all panels; 0-7 are taken (0-3 and 5-7
        // upstream, 4 by the Timeline panel).
        8
    }
}
