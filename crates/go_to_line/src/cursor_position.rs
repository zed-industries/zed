use editor::{Editor, MultiBufferSnapshot};
use gpui::{App, Entity, FocusHandle, Focusable, Subscription, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::{fmt::Write, num::NonZeroU32, time::Duration};
use text::{Point, Selection};
use ui::{
    div, Button, ButtonCommon, Clickable, Context, FluentBuilder, IntoElement, LabelSize,
    ParentElement, Render, Tooltip, Window,
};
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::{item::ItemHandle, StatusItemView, Workspace};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub(crate) struct SelectionStats {
    pub lines: usize,
    pub characters: usize,
    pub selections: usize,
}

pub struct CursorPosition {
    position: Option<UserCaretPosition>,
    selected_count: SelectionStats,
    context: Option<FocusHandle>,
    workspace: WeakEntity<Workspace>,
    update_position: Task<()>,
    _observe_active_editor: Option<Subscription>,
}

/// A position in the editor, where user's caret is located at.
/// Lines are never zero as there is always at least one line in the editor.
/// Characters may start with zero as the caret may be at the beginning of a line, but all editors start counting characters from 1,
/// where "1" will mean "before the first character".
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UserCaretPosition {
    pub line: NonZeroU32,
    pub character: NonZeroU32,
}

impl UserCaretPosition {
    pub fn at_selection_end(selection: &Selection<Point>, snapshot: &MultiBufferSnapshot) -> Self {
        let selection_end = selection.head();
        let line_start = Point::new(selection_end.row, 0);
        let chars_to_last_position = snapshot
            .text_summary_for_range::<text::TextSummary, _>(line_start..selection_end)
            .chars as u32;
        Self {
            line: NonZeroU32::new(selection_end.row + 1).expect("added 1"),
            character: NonZeroU32::new(chars_to_last_position + 1).expect("added 1"),
        }
    }
}

impl CursorPosition {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            position: None,
            context: None,
            selected_count: Default::default(),
            workspace: workspace.weak_handle(),
            update_position: Task::ready(()),
            _observe_active_editor: None,
        }
    }

    fn update_position(
        &mut self,
        editor: Entity<Editor>,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.downgrade();
        self.update_position = cx.spawn_in(window, |cursor_position, mut cx| async move {
            let is_singleton = editor
                .update(&mut cx, |editor, cx| {
                    editor.buffer().read(cx).is_singleton()
                })
                .ok()
                .unwrap_or(true);

            if !is_singleton {
                if let Some(debounce) = debounce {
                    cx.background_executor().timer(debounce).await;
                }
            }

            editor
                .update(&mut cx, |editor, cx| {
                    cursor_position.update(cx, |cursor_position, cx| {
                        cursor_position.selected_count = SelectionStats::default();
                        cursor_position.selected_count.selections = editor.selections.count();
                        match editor.mode() {
                            editor::EditorMode::AutoHeight { .. }
                            | editor::EditorMode::SingleLine { .. } => {
                                cursor_position.position = None;
                                cursor_position.context = None;
                            }
                            editor::EditorMode::Full => {
                                let mut last_selection = None::<Selection<Point>>;
                                let snapshot = editor.buffer().read(cx).snapshot(cx);
                                if snapshot.excerpts().count() > 0 {
                                    for selection in editor.selections.all::<Point>(cx) {
                                        let selection_summary = snapshot
                                            .text_summary_for_range::<text::TextSummary, _>(
                                                selection.start..selection.end,
                                            );
                                        cursor_position.selected_count.characters +=
                                            selection_summary.chars;
                                        if selection.end != selection.start {
                                            cursor_position.selected_count.lines +=
                                                (selection.end.row - selection.start.row) as usize;
                                            if selection.end.column != 0 {
                                                cursor_position.selected_count.lines += 1;
                                            }
                                        }
                                        if last_selection.as_ref().map_or(true, |last_selection| {
                                            selection.id > last_selection.id
                                        }) {
                                            last_selection = Some(selection);
                                        }
                                    }
                                }
                                cursor_position.position = last_selection
                                    .map(|s| UserCaretPosition::at_selection_end(&s, &snapshot));
                                cursor_position.context = Some(editor.focus_handle(cx));
                            }
                        }

                        cx.notify();
                    })
                })
                .ok()
                .transpose()
                .ok()
                .flatten();
        });
    }

    fn write_position(&self, text: &mut String, cx: &App) {
        if self.selected_count
            <= (SelectionStats {
                selections: 1,
                ..Default::default()
            })
        {
            // Do not write out anything if we have just one empty selection.
            return;
        }
        let SelectionStats {
            lines,
            characters,
            selections,
        } = self.selected_count;
        let format = LineIndicatorFormat::get(None, cx);
        let is_short_format = format == &LineIndicatorFormat::Short;
        let lines = (lines > 1).then_some((lines, "line"));
        let selections = (selections > 1).then_some((selections, "selection"));
        let characters = (characters > 0).then_some((characters, "character"));
        if (None, None, None) == (characters, selections, lines) {
            // Nothing to display.
            return;
        }
        write!(text, " (").unwrap();
        let mut wrote_once = false;
        for (count, name) in [selections, lines, characters].into_iter().flatten() {
            if wrote_once {
                write!(text, ", ").unwrap();
            }
            let name = if is_short_format { &name[..1] } else { name };
            let plural_suffix = if count > 1 && !is_short_format {
                "s"
            } else {
                ""
            };
            write!(text, "{count} {name}{plural_suffix}").unwrap();
            wrote_once = true;
        }
        text.push(')');
    }

    #[cfg(test)]
    pub(crate) fn selection_stats(&self) -> &SelectionStats {
        &self.selected_count
    }

    #[cfg(test)]
    pub(crate) fn position(&self) -> Option<UserCaretPosition> {
        self.position
    }
}

impl Render for CursorPosition {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().when_some(self.position, |el, position| {
            let mut text = format!(
                "{}{FILE_ROW_COLUMN_DELIMITER}{}",
                position.line, position.character,
            );
            self.write_position(&mut text, cx);

            let context = self.context.clone();

            el.child(
                Button::new("go-to-line-column", text)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                if let Some(editor) = workspace
                                    .active_item(cx)
                                    .and_then(|item| item.act_as::<Editor>(cx))
                                {
                                    if let Some((_, buffer, _)) = editor.read(cx).active_excerpt(cx)
                                    {
                                        workspace.toggle_modal(window, cx, |window, cx| {
                                            crate::GoToLine::new(editor, buffer, window, cx)
                                        })
                                    }
                                }
                            });
                        }
                    }))
                    .tooltip(move |window, cx| match context.as_ref() {
                        Some(context) => Tooltip::for_action_in(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            context,
                            window,
                            cx,
                        ),
                        None => Tooltip::for_action(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            window,
                            cx,
                        ),
                    }),
            )
        })
    }
}

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);

impl StatusItemView for CursorPosition {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor =
                Some(
                    cx.observe_in(&editor, window, |cursor_position, editor, window, cx| {
                        Self::update_position(
                            cursor_position,
                            editor,
                            Some(UPDATE_DEBOUNCE),
                            window,
                            cx,
                        )
                    }),
                );
            self.update_position(editor, None, window, cx);
        } else {
            self.position = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}

#[derive(Clone, Copy, Default, PartialEq, JsonSchema, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LineIndicatorFormat {
    Short,
    #[default]
    Long,
}

/// Whether or not to automatically check for updates.
///
/// Values: short, long
/// Default: short
#[derive(Clone, Copy, Default, JsonSchema, Deserialize, Serialize)]
#[serde(transparent)]
pub(crate) struct LineIndicatorFormatContent(LineIndicatorFormat);

impl Settings for LineIndicatorFormat {
    const KEY: Option<&'static str> = Some("line_indicator_format");

    type FileContent = Option<LineIndicatorFormatContent>;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> anyhow::Result<Self> {
        let format = [sources.release_channel, sources.user]
            .into_iter()
            .find_map(|value| value.copied().flatten())
            .unwrap_or(sources.default.ok_or_else(Self::missing_default)?);

        Ok(format.0)
    }
}
