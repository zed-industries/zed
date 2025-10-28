use crate::{
    App,
    zed::{TerminalPanel, with_active_or_new_workspace},
};
use editor::ToOffset;
use workspace::Panel;

pub(crate) fn send_to_terminal(cx: &mut App) {
    with_active_or_new_workspace(cx, |workspace, window, cx| {
        let Some(active_item) = workspace.active_item(cx) else {
            return;
        };

        let Some(editor) = active_item.act_as::<editor::Editor>(cx) else {
            return;
        };

        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let selection = editor.read(cx).selections.newest_anchor();
        let selection_range = selection.start.to_offset(&buffer)..selection.end.to_offset(&buffer);

        let (expression_text, expression_range) = if selection_range.is_empty() {
            let cursor_offset = selection_range.start;
            let cursor_point = buffer.offset_to_point(cursor_offset);

            let line_range = buffer.point_to_offset(language::Point::new(cursor_point.row, 0))
                ..buffer.point_to_offset(language::Point::new(cursor_point.row + 1, 0));
            let line_text = buffer
                .text_for_range(line_range.clone())
                .collect::<String>();

            let search_offset = if line_text.trim().is_empty() {
                line_range.end
            } else {
                line_range.start
            };

            let mut range = search_offset..search_offset;
            let mut found_valid_expression = false;

            while let Some((node, new_range)) = buffer.syntax_ancestor(range.clone()) {
                range = new_range;
                if node.is_named() {
                    let kind = node.kind();
                    if kind.contains("call")
                        || kind.contains("statement")
                        || kind.contains("expression")
                        || kind.contains("assignment")
                        || kind == "binary_operator"
                    {
                        found_valid_expression = true;
                        break;
                    }
                }
            }

            if !found_valid_expression {
                range = search_offset..search_offset;
                while let Some((node, new_range)) = buffer.syntax_ancestor(range.clone()) {
                    range = new_range;
                    if node.is_named() {
                        break;
                    }
                }
            }

            if range.is_empty() {
                (line_text, line_range)
            } else {
                (
                    buffer.text_for_range(range.clone()).collect::<String>(),
                    range,
                )
            }
        } else {
            (
                buffer
                    .text_for_range(selection_range.clone())
                    .collect::<String>(),
                selection_range,
            )
        };

        let expression_text = expression_text.trim().to_string();

        if expression_text.is_empty() {
            return;
        }

        let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) else {
            return;
        };

        let Some(terminal_view) = terminal_panel.read(cx).pane().and_then(|pane| {
            pane.read(cx)
                .active_item()
                .and_then(|item| item.downcast::<terminal_view::TerminalView>())
        }) else {
            return;
        };

        let command_with_newline = format!("{}\n", expression_text);
        terminal_view.update(cx, |view, cx| {
            view.terminal().update(cx, |terminal, _cx| {
                terminal.input(command_with_newline.into_bytes());
            });
        });

        let end_point = buffer.offset_to_point(expression_range.end);
        let next_line_offset = buffer.point_to_offset(language::Point::new(end_point.row + 1, 0));
        let anchor = buffer.anchor_after(next_line_offset);

        editor.update(cx, |editor, cx| {
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_anchor_ranges([anchor..anchor]);
            });
        });
    });
}
