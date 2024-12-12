use std::{
    iter::Peekable,
    ops::{Deref, Range},
    str::Chars,
    sync::OnceLock,
};

use anyhow::{anyhow, Result};
use command_palette_hooks::CommandInterceptResult;
use editor::{
    actions::{SortLinesCaseInsensitive, SortLinesCaseSensitive},
    Editor, ToPoint,
};
use gpui::{actions, impl_actions, Action, AppContext, Global, ViewContext};
use language::Point;
use multi_buffer::MultiBufferRow;
use serde::Deserialize;
use ui::WindowContext;
use util::ResultExt;
use workspace::{notifications::NotifyResultExt, SaveIntent};

use crate::{
    motion::{EndOfDocument, Motion, StartOfDocument},
    normal::{
        search::{FindCommand, ReplaceCommand, Replacement},
        JoinLines,
    },
    state::Mode,
    visual::VisualDeleteLine,
    Vim,
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GoToLine {
    range: CommandRange,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct YankCommand {
    range: CommandRange,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WithRange {
    restore_selection: bool,
    range: CommandRange,
    action: WrappedAction,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WithCount {
    count: u32,
    action: WrappedAction,
}

#[derive(Debug)]
struct WrappedAction(Box<dyn Action>);

actions!(vim, [VisualCommand, CountCommand]);
impl_actions!(vim, [GoToLine, YankCommand, WithRange, WithCount]);

impl<'de> Deserialize<'de> for WrappedAction {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom("Cannot deserialize WrappedAction"))
    }
}

impl PartialEq for WrappedAction {
    fn eq(&self, other: &Self) -> bool {
        self.0.partial_eq(&*other.0)
    }
}

impl Clone for WrappedAction {
    fn clone(&self) -> Self {
        Self(self.0.boxed_clone())
    }
}

impl Deref for WrappedAction {
    type Target = dyn Action;
    fn deref(&self) -> &dyn Action {
        &*self.0
    }
}

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, |vim, _: &VisualCommand, cx| {
        let Some(workspace) = vim.workspace(cx) else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, "'<,'>", cx);
        })
    });

    Vim::action(editor, cx, |vim, _: &CountCommand, cx| {
        let Some(workspace) = vim.workspace(cx) else {
            return;
        };
        let count = Vim::take_count(cx).unwrap_or(1);
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(
                workspace,
                &format!(".,.+{}", count.saturating_sub(1)),
                cx,
            );
        })
    });

    Vim::action(editor, cx, |vim, action: &GoToLine, cx| {
        vim.switch_mode(Mode::Normal, false, cx);
        let result = vim.update_editor(cx, |vim, editor, cx| {
            action.range.head().buffer_row(vim, editor, cx)
        });
        let buffer_row = match result {
            None => return,
            Some(e @ Err(_)) => {
                let Some(workspace) = vim.workspace(cx) else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    e.notify_err(workspace, cx);
                });
                return;
            }
            Some(Ok(result)) => result,
        };
        vim.move_cursor(Motion::StartOfDocument, Some(buffer_row.0 as usize + 1), cx);
    });

    Vim::action(editor, cx, |vim, action: &YankCommand, cx| {
        vim.update_editor(cx, |vim, editor, cx| {
            let snapshot = editor.snapshot(cx);
            if let Ok(range) = action.range.buffer_range(vim, editor, cx) {
                let end = if range.end < snapshot.buffer_snapshot.max_row() {
                    Point::new(range.end.0 + 1, 0)
                } else {
                    snapshot.buffer_snapshot.max_point()
                };
                vim.copy_ranges(
                    editor,
                    true,
                    true,
                    vec![Point::new(range.start.0, 0)..end],
                    cx,
                )
            }
        });
    });

    Vim::action(editor, cx, |_, action: &WithCount, cx| {
        for _ in 0..action.count {
            cx.dispatch_action(action.action.boxed_clone())
        }
    });

    Vim::action(editor, cx, |vim, action: &WithRange, cx| {
        let result = vim.update_editor(cx, |vim, editor, cx| {
            action.range.buffer_range(vim, editor, cx)
        });

        let range = match result {
            None => return,
            Some(e @ Err(_)) => {
                let Some(workspace) = vim.workspace(cx) else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    e.notify_err(workspace, cx);
                });
                return;
            }
            Some(Ok(result)) => result,
        };

        let previous_selections = vim
            .update_editor(cx, |_, editor, cx| {
                let selections = action
                    .restore_selection
                    .then(|| editor.selections.disjoint_anchor_ranges());
                editor.change_selections(None, cx, |s| {
                    let end = Point::new(range.end.0, s.buffer().line_len(range.end));
                    s.select_ranges([end..Point::new(range.start.0, 0)]);
                });
                selections
            })
            .flatten();
        cx.dispatch_action(action.action.boxed_clone());
        cx.defer(move |vim, cx| {
            vim.update_editor(cx, |_, editor, cx| {
                editor.change_selections(None, cx, |s| {
                    if let Some(previous_selections) = previous_selections {
                        s.select_ranges(previous_selections);
                    } else {
                        s.select_ranges([
                            Point::new(range.start.0, 0)..Point::new(range.start.0, 0)
                        ]);
                    }
                })
            });
        });
    });
}

#[derive(Default)]
struct VimCommand {
    prefix: &'static str,
    suffix: &'static str,
    action: Option<Box<dyn Action>>,
    action_name: Option<&'static str>,
    bang_action: Option<Box<dyn Action>>,
    range: Option<
        Box<
            dyn Fn(Box<dyn Action>, &CommandRange) -> Option<Box<dyn Action>>
                + Send
                + Sync
                + 'static,
        >,
    >,
    has_count: bool,
}

impl VimCommand {
    fn new(pattern: (&'static str, &'static str), action: impl Action) -> Self {
        Self {
            prefix: pattern.0,
            suffix: pattern.1,
            action: Some(action.boxed_clone()),
            ..Default::default()
        }
    }

    // from_str is used for actions in other crates.
    fn str(pattern: (&'static str, &'static str), action_name: &'static str) -> Self {
        Self {
            prefix: pattern.0,
            suffix: pattern.1,
            action_name: Some(action_name),
            ..Default::default()
        }
    }

    fn bang(mut self, bang_action: impl Action) -> Self {
        self.bang_action = Some(bang_action.boxed_clone());
        self
    }

    fn range(
        mut self,
        f: impl Fn(Box<dyn Action>, &CommandRange) -> Option<Box<dyn Action>> + Send + Sync + 'static,
    ) -> Self {
        self.range = Some(Box::new(f));
        self
    }

    fn count(mut self) -> Self {
        self.has_count = true;
        self
    }

    fn parse(
        &self,
        mut query: &str,
        range: &Option<CommandRange>,
        cx: &AppContext,
    ) -> Option<Box<dyn Action>> {
        let has_bang = query.ends_with('!');
        if has_bang {
            query = &query[..query.len() - 1];
        }

        let suffix = query.strip_prefix(self.prefix)?;
        if !self.suffix.starts_with(suffix) {
            return None;
        }

        let action = if has_bang && self.bang_action.is_some() {
            self.bang_action.as_ref().unwrap().boxed_clone()
        } else if let Some(action) = self.action.as_ref() {
            action.boxed_clone()
        } else if let Some(action_name) = self.action_name {
            cx.build_action(action_name, None).log_err()?
        } else {
            return None;
        };

        if let Some(range) = range {
            self.range.as_ref().and_then(|f| f(action, range))
        } else {
            Some(action)
        }
    }

    // TODO: ranges with search queries
    fn parse_range(query: &str) -> (Option<CommandRange>, String) {
        let mut chars = query.chars().peekable();

        match chars.peek() {
            Some('%') => {
                chars.next();
                return (
                    Some(CommandRange {
                        start: Position::Line { row: 1, offset: 0 },
                        end: Some(Position::LastLine { offset: 0 }),
                    }),
                    chars.collect(),
                );
            }
            Some('*') => {
                chars.next();
                return (
                    Some(CommandRange {
                        start: Position::Mark {
                            name: '<',
                            offset: 0,
                        },
                        end: Some(Position::Mark {
                            name: '>',
                            offset: 0,
                        }),
                    }),
                    chars.collect(),
                );
            }
            _ => {}
        }

        let start = Self::parse_position(&mut chars);

        match chars.peek() {
            Some(',' | ';') => {
                chars.next();
                (
                    Some(CommandRange {
                        start: start.unwrap_or(Position::CurrentLine { offset: 0 }),
                        end: Self::parse_position(&mut chars),
                    }),
                    chars.collect(),
                )
            }
            _ => (
                start.map(|start| CommandRange { start, end: None }),
                chars.collect(),
            ),
        }
    }

    fn parse_position(chars: &mut Peekable<Chars>) -> Option<Position> {
        match chars.peek()? {
            '0'..='9' => {
                let row = Self::parse_u32(chars);
                Some(Position::Line {
                    row,
                    offset: Self::parse_offset(chars),
                })
            }
            '\'' => {
                chars.next();
                let name = chars.next()?;
                Some(Position::Mark {
                    name,
                    offset: Self::parse_offset(chars),
                })
            }
            '.' => {
                chars.next();
                Some(Position::CurrentLine {
                    offset: Self::parse_offset(chars),
                })
            }
            '+' | '-' => Some(Position::CurrentLine {
                offset: Self::parse_offset(chars),
            }),
            '$' => {
                chars.next();
                Some(Position::LastLine {
                    offset: Self::parse_offset(chars),
                })
            }
            _ => None,
        }
    }

    fn parse_offset(chars: &mut Peekable<Chars>) -> i32 {
        let mut res: i32 = 0;
        while matches!(chars.peek(), Some('+' | '-')) {
            let sign = if chars.next().unwrap() == '+' { 1 } else { -1 };
            let amount = if matches!(chars.peek(), Some('0'..='9')) {
                (Self::parse_u32(chars) as i32).saturating_mul(sign)
            } else {
                sign
            };
            res = res.saturating_add(amount)
        }
        res
    }

    fn parse_u32(chars: &mut Peekable<Chars>) -> u32 {
        let mut res: u32 = 0;
        while matches!(chars.peek(), Some('0'..='9')) {
            res = res
                .saturating_mul(10)
                .saturating_add(chars.next().unwrap() as u32 - '0' as u32);
        }
        res
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum Position {
    Line { row: u32, offset: i32 },
    Mark { name: char, offset: i32 },
    LastLine { offset: i32 },
    CurrentLine { offset: i32 },
}

impl Position {
    fn buffer_row(
        &self,
        vim: &Vim,
        editor: &mut Editor,
        cx: &mut WindowContext,
    ) -> Result<MultiBufferRow> {
        let snapshot = editor.snapshot(cx);
        let target = match self {
            Position::Line { row, offset } => row.saturating_add_signed(offset.saturating_sub(1)),
            Position::Mark { name, offset } => {
                let Some(mark) = vim.marks.get(&name.to_string()).and_then(|vec| vec.last()) else {
                    return Err(anyhow!("mark {} not set", name));
                };
                mark.to_point(&snapshot.buffer_snapshot)
                    .row
                    .saturating_add_signed(*offset)
            }
            Position::LastLine { offset } => snapshot
                .buffer_snapshot
                .max_row()
                .0
                .saturating_add_signed(*offset),
            Position::CurrentLine { offset } => editor
                .selections
                .newest_anchor()
                .head()
                .to_point(&snapshot.buffer_snapshot)
                .row
                .saturating_add_signed(*offset),
        };

        Ok(MultiBufferRow(target).min(snapshot.buffer_snapshot.max_row()))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct CommandRange {
    start: Position,
    end: Option<Position>,
}

impl CommandRange {
    fn head(&self) -> &Position {
        self.end.as_ref().unwrap_or(&self.start)
    }

    pub(crate) fn buffer_range(
        &self,
        vim: &Vim,
        editor: &mut Editor,
        cx: &mut WindowContext,
    ) -> Result<Range<MultiBufferRow>> {
        let start = self.start.buffer_row(vim, editor, cx)?;
        let end = if let Some(end) = self.end.as_ref() {
            end.buffer_row(vim, editor, cx)?
        } else {
            start
        };
        if end < start {
            anyhow::Ok(end..start)
        } else {
            anyhow::Ok(start..end)
        }
    }

    pub fn as_count(&self) -> Option<u32> {
        if let CommandRange {
            start: Position::Line { row, offset: 0 },
            end: None,
        } = &self
        {
            Some(*row)
        } else {
            None
        }
    }
}

fn generate_commands(_: &AppContext) -> Vec<VimCommand> {
    vec![
        VimCommand::new(
            ("w", "rite"),
            workspace::Save {
                save_intent: Some(SaveIntent::Save),
            },
        )
        .bang(workspace::Save {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(
            ("q", "uit"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Skip),
        }),
        VimCommand::new(
            ("wq", ""),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Save),
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(
            ("x", "it"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::SaveAll),
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(
            ("ex", "it"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::SaveAll),
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(
            ("up", "date"),
            workspace::Save {
                save_intent: Some(SaveIntent::SaveAll),
            },
        ),
        VimCommand::new(
            ("wa", "ll"),
            workspace::SaveAll {
                save_intent: Some(SaveIntent::SaveAll),
            },
        )
        .bang(workspace::SaveAll {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(
            ("qa", "ll"),
            workspace::CloseAllItemsAndPanes {
                save_intent: Some(SaveIntent::Close),
            },
        )
        .bang(workspace::CloseAllItemsAndPanes {
            save_intent: Some(SaveIntent::Skip),
        }),
        VimCommand::new(
            ("quita", "ll"),
            workspace::CloseAllItemsAndPanes {
                save_intent: Some(SaveIntent::Close),
            },
        )
        .bang(workspace::CloseAllItemsAndPanes {
            save_intent: Some(SaveIntent::Skip),
        }),
        VimCommand::new(
            ("xa", "ll"),
            workspace::CloseAllItemsAndPanes {
                save_intent: Some(SaveIntent::SaveAll),
            },
        )
        .bang(workspace::CloseAllItemsAndPanes {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(
            ("wqa", "ll"),
            workspace::CloseAllItemsAndPanes {
                save_intent: Some(SaveIntent::SaveAll),
            },
        )
        .bang(workspace::CloseAllItemsAndPanes {
            save_intent: Some(SaveIntent::Overwrite),
        }),
        VimCommand::new(("cq", "uit"), zed_actions::Quit),
        VimCommand::new(("sp", "lit"), workspace::SplitHorizontal),
        VimCommand::new(("vs", "plit"), workspace::SplitVertical),
        VimCommand::new(
            ("bd", "elete"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Skip),
        }),
        VimCommand::new(("bn", "ext"), workspace::ActivateNextItem).count(),
        VimCommand::new(("bN", "ext"), workspace::ActivatePrevItem).count(),
        VimCommand::new(("bp", "revious"), workspace::ActivatePrevItem).count(),
        VimCommand::new(("bf", "irst"), workspace::ActivateItem(0)),
        VimCommand::new(("br", "ewind"), workspace::ActivateItem(0)),
        VimCommand::new(("bl", "ast"), workspace::ActivateLastItem),
        VimCommand::new(("new", ""), workspace::NewFileSplitHorizontal),
        VimCommand::new(("vne", "w"), workspace::NewFileSplitVertical),
        VimCommand::new(("tabe", "dit"), workspace::NewFile),
        VimCommand::new(("tabnew", ""), workspace::NewFile),
        VimCommand::new(("tabn", "ext"), workspace::ActivateNextItem).count(),
        VimCommand::new(("tabp", "revious"), workspace::ActivatePrevItem).count(),
        VimCommand::new(("tabN", "ext"), workspace::ActivatePrevItem).count(),
        VimCommand::new(
            ("tabc", "lose"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
            },
        ),
        VimCommand::new(
            ("tabo", "nly"),
            workspace::CloseInactiveItems {
                save_intent: Some(SaveIntent::Close),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseInactiveItems {
            save_intent: Some(SaveIntent::Skip),
            close_pinned: false,
        }),
        VimCommand::new(
            ("on", "ly"),
            workspace::CloseInactiveTabsAndPanes {
                save_intent: Some(SaveIntent::Close),
            },
        )
        .bang(workspace::CloseInactiveTabsAndPanes {
            save_intent: Some(SaveIntent::Skip),
        }),
        VimCommand::str(("cl", "ist"), "diagnostics::Deploy"),
        VimCommand::new(("cc", ""), editor::actions::Hover),
        VimCommand::new(("ll", ""), editor::actions::Hover),
        VimCommand::new(("cn", "ext"), editor::actions::GoToDiagnostic).range(wrap_count),
        VimCommand::new(("cp", "revious"), editor::actions::GoToPrevDiagnostic).range(wrap_count),
        VimCommand::new(("cN", "ext"), editor::actions::GoToPrevDiagnostic).range(wrap_count),
        VimCommand::new(("lp", "revious"), editor::actions::GoToPrevDiagnostic).range(wrap_count),
        VimCommand::new(("lN", "ext"), editor::actions::GoToPrevDiagnostic).range(wrap_count),
        VimCommand::new(("j", "oin"), JoinLines).range(select_range),
        VimCommand::new(("fo", "ld"), editor::actions::FoldSelectedRanges).range(act_on_range),
        VimCommand::new(("foldo", "pen"), editor::actions::UnfoldLines)
            .bang(editor::actions::UnfoldRecursive)
            .range(act_on_range),
        VimCommand::new(("foldc", "lose"), editor::actions::Fold)
            .bang(editor::actions::FoldRecursive)
            .range(act_on_range),
        VimCommand::new(("dif", "fupdate"), editor::actions::ToggleHunkDiff).range(act_on_range),
        VimCommand::new(("rev", "ert"), editor::actions::RevertSelectedHunks).range(act_on_range),
        VimCommand::new(("d", "elete"), VisualDeleteLine).range(select_range),
        VimCommand::new(("y", "ank"), gpui::NoAction).range(|_, range| {
            Some(
                YankCommand {
                    range: range.clone(),
                }
                .boxed_clone(),
            )
        }),
        VimCommand::new(("sor", "t"), SortLinesCaseSensitive).range(select_range),
        VimCommand::new(("sort i", ""), SortLinesCaseInsensitive).range(select_range),
        VimCommand::str(("E", "xplore"), "project_panel::ToggleFocus"),
        VimCommand::str(("H", "explore"), "project_panel::ToggleFocus"),
        VimCommand::str(("L", "explore"), "project_panel::ToggleFocus"),
        VimCommand::str(("S", "explore"), "project_panel::ToggleFocus"),
        VimCommand::str(("Ve", "xplore"), "project_panel::ToggleFocus"),
        VimCommand::str(("te", "rm"), "terminal_panel::ToggleFocus"),
        VimCommand::str(("T", "erm"), "terminal_panel::ToggleFocus"),
        VimCommand::str(("C", "ollab"), "collab_panel::ToggleFocus"),
        VimCommand::str(("Ch", "at"), "chat_panel::ToggleFocus"),
        VimCommand::str(("No", "tifications"), "notification_panel::ToggleFocus"),
        VimCommand::str(("A", "I"), "assistant::ToggleFocus"),
        VimCommand::new(("noh", "lsearch"), search::buffer_search::Dismiss),
        VimCommand::new(("$", ""), EndOfDocument),
        VimCommand::new(("%", ""), EndOfDocument),
        VimCommand::new(("0", ""), StartOfDocument),
        VimCommand::new(("e", "dit"), editor::actions::ReloadFile)
            .bang(editor::actions::ReloadFile),
        VimCommand::new(("cpp", "link"), editor::actions::CopyPermalinkToLine).range(act_on_range),
    ]
}

struct VimCommands(Vec<VimCommand>);
// safety: we only ever access this from the main thread (as ensured by the cx argument)
// actions are not Sync so we can't otherwise use a OnceLock.
unsafe impl Sync for VimCommands {}
impl Global for VimCommands {}

fn commands(cx: &AppContext) -> &Vec<VimCommand> {
    static COMMANDS: OnceLock<VimCommands> = OnceLock::new();
    &COMMANDS
        .get_or_init(|| VimCommands(generate_commands(cx)))
        .0
}

fn act_on_range(action: Box<dyn Action>, range: &CommandRange) -> Option<Box<dyn Action>> {
    Some(
        WithRange {
            restore_selection: true,
            range: range.clone(),
            action: WrappedAction(action),
        }
        .boxed_clone(),
    )
}

fn select_range(action: Box<dyn Action>, range: &CommandRange) -> Option<Box<dyn Action>> {
    Some(
        WithRange {
            restore_selection: false,
            range: range.clone(),
            action: WrappedAction(action),
        }
        .boxed_clone(),
    )
}

fn wrap_count(action: Box<dyn Action>, range: &CommandRange) -> Option<Box<dyn Action>> {
    range.as_count().map(|count| {
        WithCount {
            count,
            action: WrappedAction(action),
        }
        .boxed_clone()
    })
}

pub fn command_interceptor(mut input: &str, cx: &AppContext) -> Option<CommandInterceptResult> {
    // NOTE: We also need to support passing arguments to commands like :w
    // (ideally with filename autocompletion).
    while input.starts_with(':') {
        input = &input[1..];
    }

    let (range, query) = VimCommand::parse_range(input);
    let range_prefix = input[0..(input.len() - query.len())].to_string();
    let query = query.as_str().trim();

    let action = if range.is_some() && query.is_empty() {
        Some(
            GoToLine {
                range: range.clone().unwrap(),
            }
            .boxed_clone(),
        )
    } else if query.starts_with('/') || query.starts_with('?') {
        Some(
            FindCommand {
                query: query[1..].to_string(),
                backwards: query.starts_with('?'),
            }
            .boxed_clone(),
        )
    } else if query.starts_with('s') {
        let mut substitute = "substitute".chars().peekable();
        let mut query = query.chars().peekable();
        while substitute
            .peek()
            .is_some_and(|char| Some(char) == query.peek())
        {
            substitute.next();
            query.next();
        }
        if let Some(replacement) = Replacement::parse(query) {
            let range = range.clone().unwrap_or(CommandRange {
                start: Position::CurrentLine { offset: 0 },
                end: None,
            });
            Some(ReplaceCommand { replacement, range }.boxed_clone())
        } else {
            None
        }
    } else {
        None
    };
    if let Some(action) = action {
        let string = input.to_string();
        let positions = generate_positions(&string, &(range_prefix + query));
        return Some(CommandInterceptResult {
            action,
            string,
            positions,
        });
    }

    for command in commands(cx).iter() {
        if let Some(action) = command.parse(query, &range, cx) {
            let mut string = ":".to_owned() + &range_prefix + command.prefix + command.suffix;
            if query.ends_with('!') {
                string.push('!');
            }
            let positions = generate_positions(&string, &(range_prefix + query));

            return Some(CommandInterceptResult {
                action,
                string,
                positions,
            });
        }
    }
    None
}

fn generate_positions(string: &str, query: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut chars = query.chars();

    let Some(mut current) = chars.next() else {
        return positions;
    };

    for (i, c) in string.char_indices() {
        if c == current {
            positions.push(i);
            if let Some(c) = chars.next() {
                current = c;
            } else {
                break;
            }
        }
    }

    positions
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use editor::Editor;
    use gpui::TestAppContext;
    use indoc::indoc;
    use ui::ViewContext;
    use workspace::Workspace;

    #[gpui::test]
    async fn test_command_basics(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇa
            b
            c"})
            .await;

        cx.simulate_shared_keystrokes(": j enter").await;

        // hack: our cursor positioning after a join command is wrong
        cx.simulate_shared_keystrokes("^").await;
        cx.shared_state().await.assert_eq(indoc! {
            "ˇa b
            c"
        });
    }

    #[gpui::test]
    async fn test_command_goto(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇa
            b
            c"})
            .await;
        cx.simulate_shared_keystrokes(": 3 enter").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            b
            ˇc"});
    }

    #[gpui::test]
    async fn test_command_replace(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇa
            b
            b
            c"})
            .await;
        cx.simulate_shared_keystrokes(": % s / b / d enter").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            d
            ˇd
            c"});
        cx.simulate_shared_keystrokes(": % s : . : \\ 0 \\ 0 enter")
            .await;
        cx.shared_state().await.assert_eq(indoc! {"
            aa
            dd
            dd
            ˇcc"});
        cx.simulate_shared_keystrokes("k : s / dd / ee enter").await;
        cx.shared_state().await.assert_eq(indoc! {"
            aa
            dd
            ˇee
            cc"});
    }

    #[gpui::test]
    async fn test_command_search(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
                ˇa
                b
                a
                c"})
            .await;
        cx.simulate_shared_keystrokes(": / b enter").await;
        cx.shared_state().await.assert_eq(indoc! {"
                a
                ˇb
                a
                c"});
        cx.simulate_shared_keystrokes(": ? a enter").await;
        cx.shared_state().await.assert_eq(indoc! {"
                ˇa
                b
                a
                c"});
    }

    #[gpui::test]
    async fn test_command_write(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let path = Path::new("/root/dir/file.rs");
        let fs = cx.workspace(|workspace, cx| workspace.project().read(cx).fs().clone());

        cx.simulate_keystrokes("i @ escape");
        cx.simulate_keystrokes(": w enter");

        assert_eq!(fs.load(path).await.unwrap(), "@\n");

        fs.as_fake().insert_file(path, b"oops\n".to_vec()).await;

        // conflict!
        cx.simulate_keystrokes("i @ escape");
        cx.simulate_keystrokes(": w enter");
        assert!(cx.has_pending_prompt());
        // "Cancel"
        cx.simulate_prompt_answer(0);
        assert_eq!(fs.load(path).await.unwrap(), "oops\n");
        assert!(!cx.has_pending_prompt());
        // force overwrite
        cx.simulate_keystrokes(": w ! enter");
        assert!(!cx.has_pending_prompt());
        assert_eq!(fs.load(path).await.unwrap(), "@@\n");
    }

    #[gpui::test]
    async fn test_command_quit(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.simulate_keystrokes(": n e w enter");
        cx.workspace(|workspace, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.simulate_keystrokes(": q enter");
        cx.workspace(|workspace, cx| assert_eq!(workspace.items(cx).count(), 1));
        cx.simulate_keystrokes(": n e w enter");
        cx.workspace(|workspace, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.simulate_keystrokes(": q a enter");
        cx.workspace(|workspace, cx| assert_eq!(workspace.items(cx).count(), 0));
    }

    #[gpui::test]
    async fn test_offsets(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇ1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n")
            .await;

        cx.simulate_shared_keystrokes(": + enter").await;
        cx.shared_state()
            .await
            .assert_eq("1\nˇ2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n");

        cx.simulate_shared_keystrokes(": 1 0 - enter").await;
        cx.shared_state()
            .await
            .assert_eq("1\n2\n3\n4\n5\n6\n7\n8\nˇ9\n10\n11\n");

        cx.simulate_shared_keystrokes(": . - 2 enter").await;
        cx.shared_state()
            .await
            .assert_eq("1\n2\n3\n4\n5\n6\nˇ7\n8\n9\n10\n11\n");

        cx.simulate_shared_keystrokes(": % enter").await;
        cx.shared_state()
            .await
            .assert_eq("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\nˇ");
    }

    #[gpui::test]
    async fn test_command_ranges(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇ1\n2\n3\n4\n4\n3\n2\n1").await;

        cx.simulate_shared_keystrokes(": 2 , 4 d enter").await;
        cx.shared_state().await.assert_eq("1\nˇ4\n3\n2\n1");

        cx.simulate_shared_keystrokes(": 2 , 4 s o r t enter").await;
        cx.shared_state().await.assert_eq("1\nˇ2\n3\n4\n1");

        cx.simulate_shared_keystrokes(": 2 , 4 j o i n enter").await;
        cx.shared_state().await.assert_eq("1\nˇ2 3 4\n1");
    }

    #[gpui::test]
    async fn test_command_visual_replace(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇ1\n2\n3\n4\n4\n3\n2\n1").await;

        cx.simulate_shared_keystrokes("v 2 j : s / . / k enter")
            .await;
        cx.shared_state().await.assert_eq("k\nk\nˇk\n4\n4\n3\n2\n1");
    }

    fn assert_active_item(
        workspace: &mut Workspace,
        expected_path: &str,
        expected_text: &str,
        cx: &mut ViewContext<Workspace>,
    ) {
        let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();

        let buffer = active_editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .unwrap();

        let text = buffer.read(cx).text();
        let file = buffer.read(cx).file().unwrap();
        let file_path = file.as_local().unwrap().abs_path(cx);

        assert_eq!(text, expected_text);
        assert_eq!(file_path.to_str().unwrap(), expected_path);
    }

    #[gpui::test]
    async fn test_command_gf(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Assert base state, that we're in /root/dir/file.rs
        cx.workspace(|workspace, cx| {
            assert_active_item(workspace, "/root/dir/file.rs", "", cx);
        });

        // Insert a new file
        let fs = cx.workspace(|workspace, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file("/root/dir/file2.rs", "This is file2.rs".as_bytes().to_vec())
            .await;
        fs.as_fake()
            .insert_file("/root/dir/file3.rs", "go to file3".as_bytes().to_vec())
            .await;

        // Put the path to the second file into the currently open buffer
        cx.set_state(indoc! {"go to fiˇle2.rs"}, Mode::Normal);

        // Go to file2.rs
        cx.simulate_keystrokes("g f");

        // We now have two items
        cx.workspace(|workspace, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.workspace(|workspace, cx| {
            assert_active_item(workspace, "/root/dir/file2.rs", "This is file2.rs", cx);
        });

        // Update editor to point to `file2.rs`
        cx.editor = cx.workspace(|workspace, cx| workspace.active_item_as::<Editor>(cx).unwrap());

        // Put the path to the third file into the currently open buffer,
        // but remove its suffix, because we want that lookup to happen automatically.
        cx.set_state(indoc! {"go to fiˇle3"}, Mode::Normal);

        // Go to file3.rs
        cx.simulate_keystrokes("g f");

        // We now have three items
        cx.workspace(|workspace, cx| assert_eq!(workspace.items(cx).count(), 3));
        cx.workspace(|workspace, cx| {
            assert_active_item(workspace, "/root/dir/file3.rs", "go to file3", cx);
        });
    }
}
