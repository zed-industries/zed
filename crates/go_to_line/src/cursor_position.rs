use editor::{Editor, EditorEvent, MBTextSummary, MultiBufferSnapshot};
use gpui::{App, Entity, FocusHandle, Focusable, Styled, Subscription, Task, WeakEntity};
use settings::{RegisterSetting, Settings, SettingsStore};
use std::{fmt::Write, num::NonZeroU32, time::Duration};
use text::{Point, Selection};
use ui::{
    Button, ButtonCommon, Clickable, Context, FluentBuilder, IntoElement, LabelSize, ParentElement,
    Render, Tooltip, Window, div,
};
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::{HideStatusItem, StatusBarSettings, StatusItemView, Workspace, item::ItemHandle};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub(crate) struct SelectionStats {
    pub lines: usize,
    pub characters: usize,
    pub words: usize,
    pub selections: usize,
}

pub struct CursorPosition {
    position: Option<UserCaretPosition>,
    selected_count: SelectionStats,
    document_words: usize,
    active_editor: Option<WeakEntity<Editor>>,
    context: Option<FocusHandle>,
    workspace: WeakEntity<Workspace>,
    update_position: Task<()>,
    update_document_words: Task<()>,
    _observe_active_editor: Option<Subscription>,
    _observe_settings: Subscription,
}

/// Counts words as maximal runs of non-whitespace characters, matching the
/// behavior of `wc -w`. The `in_word` state is carried across chunk boundaries
/// so that a word split across two chunks is not double counted.
fn count_words<'a>(chunks: impl Iterator<Item = &'a str>) -> usize {
    let mut words = 0;
    let mut in_word = false;
    for chunk in chunks {
        for character in chunk.chars() {
            if character.is_whitespace() {
                in_word = false;
            } else if !in_word {
                in_word = true;
                words += 1;
            }
        }
    }
    words
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
    pub(crate) fn at_selection_end(
        selection: &Selection<Point>,
        snapshot: &MultiBufferSnapshot,
    ) -> Self {
        let selection_end = selection.head();
        let (line, character) =
            if let Some((buffer_snapshot, point)) = snapshot.point_to_buffer_point(selection_end) {
                let line_start = Point::new(point.row, 0);

                let chars_to_last_position = buffer_snapshot
                    .text_summary_for_range::<text::TextSummary, _>(line_start..point)
                    .chars as u32;
                (line_start.row, chars_to_last_position)
            } else {
                let line_start = Point::new(selection_end.row, 0);

                let chars_to_last_position = snapshot
                    .text_summary_for_range::<MBTextSummary, _>(line_start..selection_end)
                    .chars as u32;
                (selection_end.row, chars_to_last_position)
            };

        Self {
            line: NonZeroU32::new(line + 1).expect("added 1"),
            character: NonZeroU32::new(character + 1).expect("added 1"),
        }
    }
}

impl CursorPosition {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        // Recompute the document word count when the relevant setting is toggled
        // so the status bar reflects the change immediately.
        let observe_settings = cx.observe_global::<SettingsStore>(|cursor_position, cx| {
            cursor_position.update_document_words(None, cx);
            cx.notify();
        });
        Self {
            position: None,
            context: None,
            selected_count: Default::default(),
            document_words: 0,
            active_editor: None,
            workspace: workspace.weak_handle(),
            update_position: Task::ready(()),
            update_document_words: Task::ready(()),
            _observe_active_editor: None,
            _observe_settings: observe_settings,
        }
    }

    fn update_position(
        &mut self,
        editor: &Entity<Editor>,
        debounce: Option<Duration>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = editor.downgrade();
        self.update_position = cx.spawn_in(window, async move |cursor_position, cx| {
            let is_singleton = editor
                .update(cx, |editor, cx| editor.buffer().read(cx).is_singleton())
                .ok()
                .unwrap_or(true);

            if !is_singleton && let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }

            editor
                .update(cx, |editor, cx| {
                    cursor_position.update(cx, |cursor_position, cx| {
                        cursor_position.selected_count = SelectionStats::default();
                        cursor_position.selected_count.selections = editor.selections.count();
                        match editor.mode() {
                            editor::EditorMode::AutoHeight { .. }
                            | editor::EditorMode::SingleLine
                            | editor::EditorMode::Minimap { .. } => {
                                cursor_position.position = None;
                                cursor_position.context = None;
                            }
                            editor::EditorMode::Full { .. } => {
                                let word_count_enabled =
                                    StatusBarSettings::get_global(cx).word_count_button;
                                let mut last_selection = None::<Selection<Point>>;
                                let snapshot = editor.display_snapshot(cx);
                                if snapshot.buffer_snapshot().excerpts().count() > 0 {
                                    for selection in editor.selections.all_adjusted(&snapshot) {
                                        let selection_summary = snapshot
                                            .buffer_snapshot()
                                            .text_summary_for_range::<MBTextSummary, _>(
                                            selection.start..selection.end,
                                        );
                                        cursor_position.selected_count.characters +=
                                            selection_summary.chars;
                                        if word_count_enabled {
                                            cursor_position.selected_count.words += count_words(
                                                snapshot
                                                    .buffer_snapshot()
                                                    .text_for_range(selection.start..selection.end),
                                            );
                                        }
                                        if selection.end != selection.start {
                                            cursor_position.selected_count.lines +=
                                                (selection.end.row - selection.start.row) as usize;
                                            if selection.end.column != 0 {
                                                cursor_position.selected_count.lines += 1;
                                            }
                                        }
                                        if last_selection.as_ref().is_none_or(|last_selection| {
                                            selection.id > last_selection.id
                                        }) {
                                            last_selection = Some(selection);
                                        }
                                    }
                                }
                                cursor_position.position = last_selection.map(|s| {
                                    UserCaretPosition::at_selection_end(
                                        &s,
                                        snapshot.buffer_snapshot(),
                                    )
                                });
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

    fn update_document_words(&mut self, debounce: Option<Duration>, cx: &mut Context<Self>) {
        if !StatusBarSettings::get_global(cx).word_count_button {
            if self.document_words != 0 {
                self.document_words = 0;
                cx.notify();
            }
            return;
        }
        let Some(editor) = self.active_editor.clone() else {
            return;
        };
        self.update_document_words = cx.spawn(async move |cursor_position, cx| {
            if let Some(debounce) = debounce {
                cx.background_executor().timer(debounce).await;
            }
            let words = editor.update(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                count_words(snapshot.text_for_range(Point::zero()..snapshot.max_point()))
            });
            if let Ok(words) = words {
                cursor_position
                    .update(cx, |cursor_position, cx| {
                        if cursor_position.document_words != words {
                            cursor_position.document_words = words;
                            cx.notify();
                        }
                    })
                    .ok();
            }
        });
    }

    fn write_position(&self, text: &mut String, cx: &App) {
        let format = LineIndicatorFormat::get(None, cx);
        let is_short_format = format == &LineIndicatorFormat::Short;
        let word_count_enabled = StatusBarSettings::get_global(cx).word_count_button;

        let SelectionStats {
            lines,
            characters,
            words,
            selections,
        } = self.selected_count;
        let has_selection = characters > 0 || lines > 0 || selections > 1;

        let mut entries: Vec<(usize, &'static str)> = Vec::new();
        if has_selection {
            if selections > 1 {
                entries.push((selections, "selection"));
            }
            if lines > 1 {
                entries.push((lines, "line"));
            }
            if word_count_enabled {
                entries.push((words, "word"));
            }
            if characters > 0 {
                entries.push((characters, "character"));
            }
        } else if word_count_enabled {
            // With no active selection, show the whole document's word count.
            entries.push((self.document_words, "word"));
        }

        if entries.is_empty() {
            return;
        }

        write!(text, " (").unwrap();
        let mut wrote_once = false;
        for (count, name) in entries {
            if wrote_once {
                write!(text, ", ").unwrap();
            }
            let name = if is_short_format { &name[..1] } else { name };
            // Use a plural suffix for any count that isn't exactly one, so that a
            // zero document word count reads as "0 words" rather than "0 word".
            let plural_suffix = if count != 1 && !is_short_format {
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
    pub(crate) fn document_words(&self) -> usize {
        self.document_words
    }

    #[cfg(test)]
    pub(crate) fn position(&self) -> Option<UserCaretPosition> {
        self.position
    }
}

impl Render for CursorPosition {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).cursor_position_button {
            return div().hidden();
        }

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
                    .tab_index(0isize)
                    .aria_label(format!(
                        "Line {}, column {}",
                        position.line, position.character
                    ))
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                if let Some(editor) = workspace
                                    .active_item(cx)
                                    .and_then(|item| item.act_as::<Editor>(cx))
                                    && let Some(buffer) = editor.read(cx).active_buffer(cx)
                                {
                                    workspace.toggle_modal(window, cx, |window, cx| {
                                        crate::GoToLine::new(editor, buffer, window, cx)
                                    })
                                }
                            });
                        }
                    }))
                    .tooltip(move |_window, cx| match context.as_ref() {
                        Some(context) => Tooltip::for_action_in(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            context,
                            cx,
                        ),
                        None => Tooltip::for_action(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            cx,
                        ),
                    }),
            )
        })
    }
}

const UPDATE_DEBOUNCE: Duration = Duration::from_millis(50);
// The document word count requires scanning the whole buffer, so coalesce rapid
// edits (e.g. while typing) behind a slightly longer debounce.
const WORD_COUNT_DEBOUNCE: Duration = Duration::from_millis(250);

impl StatusItemView for CursorPosition {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.active_editor = Some(editor.downgrade());
            self._observe_active_editor = Some(cx.subscribe_in(
                &editor,
                window,
                |cursor_position, editor, event, window, cx| match event {
                    EditorEvent::SelectionsChanged { .. } => Self::update_position(
                        cursor_position,
                        editor,
                        Some(UPDATE_DEBOUNCE),
                        window,
                        cx,
                    ),
                    EditorEvent::BufferEdited => {
                        cursor_position.update_document_words(Some(WORD_COUNT_DEBOUNCE), cx)
                    }
                    _ => {}
                },
            ));
            self.update_position(&editor, None, window, cx);
            self.update_document_words(None, cx);
        } else {
            self.position = None;
            self.active_editor = None;
            self.document_words = 0;
            self._observe_active_editor = None;
        }

        cx.notify();
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|settings| {
            settings
                .status_bar
                .get_or_insert_default()
                .cursor_position_button = Some(false);
        }))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, RegisterSetting)]
pub enum LineIndicatorFormat {
    Short,
    Long,
}

impl From<settings::LineIndicatorFormat> for LineIndicatorFormat {
    fn from(format: settings::LineIndicatorFormat) -> Self {
        match format {
            settings::LineIndicatorFormat::Short => LineIndicatorFormat::Short,
            settings::LineIndicatorFormat::Long => LineIndicatorFormat::Long,
        }
    }
}

impl Settings for LineIndicatorFormat {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        content.line_indicator_format.unwrap().into()
    }
}
