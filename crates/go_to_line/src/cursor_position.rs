use editor::{Editor, ToPoint};
use gpui::{AppContext, Subscription, View, WeakView};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::fmt::Write;
use text::{Point, Selection};
use ui::{
    div, Button, ButtonCommon, Clickable, FluentBuilder, IntoElement, LabelSize, ParentElement,
    Render, Tooltip, ViewContext,
};
use util::paths::FILE_ROW_COLUMN_DELIMITER;
use workspace::{item::ItemHandle, StatusItemView, Workspace};

#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
struct SelectionStats {
    lines: usize,
    characters: usize,
    selections: usize,
}

pub struct CursorPosition {
    position: Option<Point>,
    selected_count: SelectionStats,
    workspace: WeakView<Workspace>,
    _observe_active_editor: Option<Subscription>,
}

impl CursorPosition {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            position: None,
            selected_count: Default::default(),
            workspace: workspace.weak_handle(),
            _observe_active_editor: None,
        }
    }

    fn update_position(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);

        self.selected_count = Default::default();
        self.selected_count.selections = editor.selections.count();
        let mut last_selection: Option<Selection<usize>> = None;
        for selection in editor.selections.all::<usize>(cx) {
            self.selected_count.characters += selection.end - selection.start;
            if last_selection
                .as_ref()
                .map_or(true, |last_selection| selection.id > last_selection.id)
            {
                last_selection = Some(selection);
            }
        }
        for selection in editor.selections.all::<Point>(cx) {
            if selection.end != selection.start {
                self.selected_count.lines += (selection.end.row - selection.start.row) as usize;
                if selection.end.column != 0 {
                    self.selected_count.lines += 1;
                }
            }
        }
        self.position = last_selection.map(|s| s.head().to_point(&buffer));

        cx.notify();
    }

    fn write_position(&self, text: &mut String, cx: &AppContext) {
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
            let name = if is_short_format { &name[..1] } else { &name };
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
}

impl Render for CursorPosition {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().when_some(self.position, |el, position| {
            let mut text = format!(
                "{}{FILE_ROW_COLUMN_DELIMITER}{}",
                position.row + 1,
                position.column + 1
            );
            self.write_position(&mut text, cx);

            el.child(
                Button::new("go-to-line-column", text)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                if let Some(editor) = workspace
                                    .active_item(cx)
                                    .and_then(|item| item.act_as::<Editor>(cx))
                                {
                                    workspace
                                        .toggle_modal(cx, |cx| crate::GoToLine::new(editor, cx))
                                }
                            });
                        }
                    }))
                    .tooltip(|cx| {
                        Tooltip::for_action(
                            "Go to Line/Column",
                            &editor::actions::ToggleGoToLine,
                            cx,
                        )
                    }),
            )
        })
    }
}

impl StatusItemView for CursorPosition {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_position));
            self.update_position(editor, cx);
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

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut AppContext,
    ) -> anyhow::Result<Self> {
        let format = [sources.release_channel, sources.user]
            .into_iter()
            .find_map(|value| value.copied().flatten())
            .unwrap_or(sources.default.ok_or_else(Self::missing_default)?);

        Ok(format.0)
    }
}
