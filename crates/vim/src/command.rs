use anyhow::{anyhow, Result};
use collections::HashMap;
use command_palette_hooks::CommandInterceptResult;
use editor::{
    actions::{SortLinesCaseInsensitive, SortLinesCaseSensitive},
    display_map::ToDisplayPoint,
    scroll::Autoscroll,
    Bias, Editor, ToPoint,
};
use gpui::{
    actions, impl_internal_actions, Action, AppContext, Global, ViewContext, WindowContext,
};
use language::Point;
use multi_buffer::MultiBufferRow;
use regex::Regex;
use schemars::JsonSchema;
use search::{BufferSearchBar, SearchOptions};
use serde::Deserialize;
use std::{
    io::Write,
    iter::Peekable,
    ops::{Deref, Range},
    process::Stdio,
    str::Chars,
    sync::OnceLock,
    time::Instant,
};
use task::{HideStrategy, RevealStrategy, SpawnInTerminal, TaskId};
use ui::ActiveTheme;
use util::ResultExt;
use workspace::{notifications::NotifyResultExt, SaveIntent};
use zed_actions::RevealTarget;

use crate::{
    motion::{EndOfDocument, Motion, StartOfDocument},
    normal::{
        search::{FindCommand, ReplaceCommand, Replacement},
        JoinLines,
    },
    object::Object,
    state::Mode,
    visual::VisualDeleteLine,
    Vim,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GoToLine {
    range: CommandRange,
}

#[derive(Clone, Debug, PartialEq)]
pub struct YankCommand {
    range: CommandRange,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithRange {
    restore_selection: bool,
    range: CommandRange,
    action: WrappedAction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithCount {
    count: u32,
    action: WrappedAction,
}

#[derive(Debug)]
struct WrappedAction(Box<dyn Action>);

actions!(vim, [VisualCommand, CountCommand, ShellCommand]);
impl_internal_actions!(
    vim,
    [
        GoToLine,
        YankCommand,
        WithRange,
        WithCount,
        OnMatchingLines,
        ShellExec
    ]
);

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

    Vim::action(editor, cx, |vim, _: &ShellCommand, cx| {
        let Some(workspace) = vim.workspace(cx) else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, "'<,'>!", cx);
        })
    });

    Vim::action(editor, cx, |vim, _: &CountCommand, cx| {
        let Some(workspace) = vim.workspace(cx) else {
            return;
        };
        let count = Vim::take_count(cx).unwrap_or(1);
        let n = if count > 1 {
            format!(".,.+{}", count.saturating_sub(1))
        } else {
            ".".to_string()
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, &n, cx);
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
                let selections = action.restore_selection.then(|| {
                    editor
                        .selections
                        .disjoint_anchor_ranges()
                        .collect::<Vec<_>>()
                });
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

    Vim::action(editor, cx, |vim, action: &OnMatchingLines, cx| {
        action.run(vim, cx)
    });

    Vim::action(editor, cx, |vim, action: &ShellExec, cx| {
        action.run(vim, cx)
    })
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

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
    } else if query.starts_with('g') || query.starts_with('v') {
        let mut global = "global".chars().peekable();
        let mut query = query.chars().peekable();
        let mut invert = false;
        if query.peek() == Some(&'v') {
            invert = true;
            query.next();
        }
        while global.peek().is_some_and(|char| Some(char) == query.peek()) {
            global.next();
            query.next();
        }
        if !invert && query.peek() == Some(&'!') {
            invert = true;
            query.next();
        }
        let range = range.clone().unwrap_or(CommandRange {
            start: Position::Line { row: 0, offset: 0 },
            end: Some(Position::LastLine { offset: 0 }),
        });
        if let Some(action) = OnMatchingLines::parse(query, invert, range, cx) {
            Some(action.boxed_clone())
        } else {
            None
        }
    } else if query.contains('!') {
        ShellExec::parse(query, range.clone())
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

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct OnMatchingLines {
    range: CommandRange,
    search: String,
    action: WrappedAction,
    invert: bool,
}

impl OnMatchingLines {
    // convert a vim query into something more usable by zed.
    // we don't attempt to fully convert between the two regex syntaxes,
    // but we do flip \( and \) to ( and ) (and vice-versa) in the pattern,
    // and convert \0..\9 to $0..$9 in the replacement so that common idioms work.
    pub(crate) fn parse(
        mut chars: Peekable<Chars>,
        invert: bool,
        range: CommandRange,
        cx: &AppContext,
    ) -> Option<Self> {
        let delimiter = chars.next().filter(|c| {
            !c.is_alphanumeric() && *c != '"' && *c != '|' && *c != '\'' && *c != '!'
        })?;

        let mut search = String::new();
        let mut escaped = false;

        while let Some(c) = chars.next() {
            if escaped {
                escaped = false;
                // unescape escaped parens
                if c != '(' && c != ')' && c != delimiter {
                    search.push('\\')
                }
                search.push(c)
            } else if c == '\\' {
                escaped = true;
            } else if c == delimiter {
                break;
            } else {
                // escape unescaped parens
                if c == '(' || c == ')' {
                    search.push('\\')
                }
                search.push(c)
            }
        }

        let command: String = chars.collect();

        let action = WrappedAction(command_interceptor(&command, cx)?.action);

        Some(Self {
            range,
            search,
            invert,
            action,
        })
    }

    pub fn run(&self, vim: &mut Vim, cx: &mut ViewContext<Vim>) {
        let result = vim.update_editor(cx, |vim, editor, cx| {
            self.range.buffer_range(vim, editor, cx)
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

        let mut action = self.action.boxed_clone();
        let mut last_pattern = self.search.clone();

        let mut regexes = match Regex::new(&self.search) {
            Ok(regex) => vec![(regex, !self.invert)],
            e @ Err(_) => {
                let Some(workspace) = vim.workspace(cx) else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    e.notify_err(workspace, cx);
                });
                return;
            }
        };
        while let Some(inner) = action
            .boxed_clone()
            .as_any()
            .downcast_ref::<OnMatchingLines>()
        {
            let Some(regex) = Regex::new(&inner.search).ok() else {
                break;
            };
            last_pattern = inner.search.clone();
            action = inner.action.boxed_clone();
            regexes.push((regex, !inner.invert))
        }

        if let Some(pane) = vim.pane(cx) {
            pane.update(cx, |pane, cx| {
                if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>()
                {
                    search_bar.update(cx, |search_bar, cx| {
                        if search_bar.show(cx) {
                            let _ = search_bar.search(
                                &last_pattern,
                                Some(SearchOptions::REGEX | SearchOptions::CASE_SENSITIVE),
                                cx,
                            );
                        }
                    });
                }
            });
        };

        vim.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(cx);
            let mut row = range.start.0;

            let point_range = Point::new(range.start.0, 0)
                ..snapshot
                    .buffer_snapshot
                    .clip_point(Point::new(range.end.0 + 1, 0), Bias::Left);
            cx.spawn(|editor, mut cx| async move {
                let new_selections = cx
                    .background_executor()
                    .spawn(async move {
                        let mut line = String::new();
                        let mut new_selections = Vec::new();
                        let chunks = snapshot
                            .buffer_snapshot
                            .text_for_range(point_range)
                            .chain(["\n"]);

                        for chunk in chunks {
                            for (newline_ix, text) in chunk.split('\n').enumerate() {
                                if newline_ix > 0 {
                                    if regexes.iter().all(|(regex, should_match)| {
                                        regex.is_match(&line) == *should_match
                                    }) {
                                        new_selections
                                            .push(Point::new(row, 0).to_display_point(&snapshot))
                                    }
                                    row += 1;
                                    line.clear();
                                }
                                line.push_str(text)
                            }
                        }

                        new_selections
                    })
                    .await;

                if new_selections.is_empty() {
                    return;
                }
                editor
                    .update(&mut cx, |editor, cx| {
                        editor.start_transaction_at(Instant::now(), cx);
                        editor.change_selections(None, cx, |s| {
                            s.replace_cursors_with(|_| new_selections);
                        });
                        cx.dispatch_action(action);
                        cx.defer(move |editor, cx| {
                            let newest = editor.selections.newest::<Point>(cx).clone();
                            editor.change_selections(None, cx, |s| {
                                s.select(vec![newest]);
                            });
                            editor.end_transaction_at(Instant::now(), cx);
                        })
                    })
                    .ok();
            })
            .detach();
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShellExec {
    command: String,
    range: Option<CommandRange>,
    is_read: bool,
}

impl Vim {
    pub fn cancel_running_command(&mut self, cx: &mut ViewContext<Self>) {
        if self.running_command.take().is_some() {
            self.update_editor(cx, |_, editor, cx| {
                editor.transact(cx, |editor, _| {
                    editor.clear_row_highlights::<ShellExec>();
                })
            });
        }
    }

    fn prepare_shell_command(&mut self, command: &str, cx: &mut ViewContext<Self>) -> String {
        let mut ret = String::new();
        // N.B. non-standard escaping rules:
        // * !echo % => "echo README.md"
        // * !echo \% => "echo %"
        // * !echo \\% => echo \%
        // * !echo \\\% => echo \\%
        for c in command.chars() {
            if c != '%' && c != '!' {
                ret.push(c);
                continue;
            } else if ret.chars().last() == Some('\\') {
                ret.pop();
                ret.push(c);
                continue;
            }
            match c {
                '%' => {
                    self.update_editor(cx, |_, editor, cx| {
                        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
                            if let Some(file) = buffer.read(cx).file() {
                                if let Some(local) = file.as_local() {
                                    if let Some(str) = local.path().to_str() {
                                        ret.push_str(str)
                                    }
                                }
                            }
                        }
                    });
                }
                '!' => {
                    if let Some(command) = &self.last_command {
                        ret.push_str(command)
                    }
                }
                _ => {}
            }
        }
        self.last_command = Some(ret.clone());
        ret
    }

    pub fn shell_command_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        cx: &mut ViewContext<Vim>,
    ) {
        self.stop_recording(cx);
        let Some(workspace) = self.workspace(cx) else {
            return;
        };
        let command = self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(cx);
            let start = editor.selections.newest_display(cx);
            let text_layout_details = editor.text_layout_details(cx);
            let mut range = motion
                .range(&snapshot, start.clone(), times, false, &text_layout_details)
                .unwrap_or(start.range());
            if range.start != start.start {
                editor.change_selections(None, cx, |s| {
                    s.select_ranges([
                        range.start.to_point(&snapshot)..range.start.to_point(&snapshot)
                    ]);
                })
            }
            if range.end.row() > range.start.row() && range.end.column() != 0 {
                *range.end.row_mut() -= 1
            }
            if range.end.row() == range.start.row() {
                ".!".to_string()
            } else {
                format!(".,.+{}!", (range.end.row() - range.start.row()).0)
            }
        });
        if let Some(command) = command {
            workspace.update(cx, |workspace, cx| {
                command_palette::CommandPalette::toggle(workspace, &command, cx);
            });
        }
    }

    pub fn shell_command_object(
        &mut self,
        object: Object,
        around: bool,
        cx: &mut ViewContext<Vim>,
    ) {
        self.stop_recording(cx);
        let Some(workspace) = self.workspace(cx) else {
            return;
        };
        let command = self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(cx);
            let start = editor.selections.newest_display(cx);
            let range = object
                .range(&snapshot, start.clone(), around)
                .unwrap_or(start.range());
            if range.start != start.start {
                editor.change_selections(None, cx, |s| {
                    s.select_ranges([
                        range.start.to_point(&snapshot)..range.start.to_point(&snapshot)
                    ]);
                })
            }
            if range.end.row() == range.start.row() {
                ".!".to_string()
            } else {
                format!(".,.+{}!", (range.end.row() - range.start.row()).0)
            }
        });
        if let Some(command) = command {
            workspace.update(cx, |workspace, cx| {
                command_palette::CommandPalette::toggle(workspace, &command, cx);
            });
        }
    }
}

impl ShellExec {
    pub fn parse(query: &str, range: Option<CommandRange>) -> Option<Box<dyn Action>> {
        let (before, after) = query.split_once('!')?;
        let before = before.trim();

        if !"read".starts_with(before) {
            return None;
        }

        Some(
            ShellExec {
                command: after.trim().to_string(),
                range,
                is_read: !before.is_empty(),
            }
            .boxed_clone(),
        )
    }

    pub fn run(&self, vim: &mut Vim, cx: &mut ViewContext<Vim>) {
        let Some(workspace) = vim.workspace(cx) else {
            return;
        };

        let project = workspace.read(cx).project().clone();
        let command = vim.prepare_shell_command(&self.command, cx);

        if self.range.is_none() && !self.is_read {
            workspace.update(cx, |workspace, cx| {
                let project = workspace.project().read(cx);
                let cwd = project.first_project_directory(cx);
                let shell = project.terminal_settings(&cwd, cx).shell.clone();
                cx.emit(workspace::Event::SpawnTask {
                    action: Box::new(SpawnInTerminal {
                        id: TaskId("vim".to_string()),
                        full_label: self.command.clone(),
                        label: self.command.clone(),
                        command: command.clone(),
                        args: Vec::new(),
                        command_label: self.command.clone(),
                        cwd,
                        env: HashMap::default(),
                        use_new_terminal: true,
                        allow_concurrent_runs: true,
                        reveal: RevealStrategy::NoFocus,
                        reveal_target: RevealTarget::Dock,
                        hide: HideStrategy::Never,
                        shell,
                        show_summary: false,
                        show_command: false,
                    }),
                });
            });
            return;
        };

        let mut input_snapshot = None;
        let mut input_range = None;
        let mut needs_newline_prefix = false;
        vim.update_editor(cx, |vim, editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let range = if let Some(range) = self.range.clone() {
                let Some(range) = range.buffer_range(vim, editor, cx).log_err() else {
                    return;
                };
                Point::new(range.start.0, 0)
                    ..snapshot.clip_point(Point::new(range.end.0 + 1, 0), Bias::Right)
            } else {
                let mut end = editor.selections.newest::<Point>(cx).range().end;
                end = snapshot.clip_point(Point::new(end.row + 1, 0), Bias::Right);
                needs_newline_prefix = end == snapshot.max_point();
                end..end
            };
            if self.is_read {
                input_range =
                    Some(snapshot.anchor_after(range.end)..snapshot.anchor_after(range.end));
            } else {
                input_range =
                    Some(snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end));
            }
            editor.highlight_rows::<ShellExec>(
                input_range.clone().unwrap(),
                cx.theme().status().unreachable_background,
                false,
                cx,
            );

            if !self.is_read {
                input_snapshot = Some(snapshot)
            }
        });

        let Some(range) = input_range else { return };

        let mut process = project.read(cx).exec_in_shell(command, cx);
        process.stdout(Stdio::piped());
        process.stderr(Stdio::piped());

        if input_snapshot.is_some() {
            process.stdin(Stdio::piped());
        } else {
            process.stdin(Stdio::null());
        };

        // https://registerspill.thorstenball.com/p/how-to-lose-control-of-your-shell
        //
        // safety: code in pre_exec should be signal safe.
        // https://man7.org/linux/man-pages/man7/signal-safety.7.html
        #[cfg(not(target_os = "windows"))]
        unsafe {
            use std::os::unix::process::CommandExt;
            process.pre_exec(|| {
                libc::setsid();
                Ok(())
            });
        };
        let is_read = self.is_read;

        let task = cx.spawn(|vim, mut cx| async move {
            let Some(mut running) = process.spawn().log_err() else {
                vim.update(&mut cx, |vim, cx| {
                    vim.cancel_running_command(cx);
                })
                .log_err();
                return;
            };

            if let Some(mut stdin) = running.stdin.take() {
                if let Some(snapshot) = input_snapshot {
                    let range = range.clone();
                    cx.background_executor()
                        .spawn(async move {
                            for chunk in snapshot.text_for_range(range) {
                                if stdin.write_all(chunk.as_bytes()).log_err().is_none() {
                                    return;
                                }
                            }
                            stdin.flush().log_err();
                        })
                        .detach();
                }
            };

            let output = cx
                .background_executor()
                .spawn(async move { running.wait_with_output() })
                .await;

            let Some(output) = output.log_err() else {
                vim.update(&mut cx, |vim, cx| {
                    vim.cancel_running_command(cx);
                })
                .log_err();
                return;
            };
            let mut text = String::new();
            if needs_newline_prefix {
                text.push('\n');
            }
            text.push_str(&String::from_utf8_lossy(&output.stdout));
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            if !text.is_empty() && text.chars().last() != Some('\n') {
                text.push('\n');
            }

            vim.update(&mut cx, |vim, cx| {
                vim.update_editor(cx, |_, editor, cx| {
                    editor.transact(cx, |editor, cx| {
                        editor.edit([(range.clone(), text)], cx);
                        let snapshot = editor.buffer().read(cx).snapshot(cx);
                        editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                            let point = if is_read {
                                let point = range.end.to_point(&snapshot);
                                Point::new(point.row.saturating_sub(1), 0)
                            } else {
                                let point = range.start.to_point(&snapshot);
                                Point::new(point.row, 0)
                            };
                            s.select_ranges([point..point]);
                        })
                    })
                });
                vim.cancel_running_command(cx);
            })
            .log_err();
        });
        vim.running_command.replace(task);
    }
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

    #[gpui::test]
    async fn test_command_matching_lines(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇa
            b
            a
            b
            a
        "})
            .await;

        cx.simulate_shared_keystrokes(":").await;
        cx.simulate_shared_keystrokes("g / a / d").await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {"
            b
            b
            ˇ"});

        cx.simulate_shared_keystrokes("u").await;

        cx.shared_state().await.assert_eq(indoc! {"
            ˇa
            b
            a
            b
            a
        "});

        cx.simulate_shared_keystrokes(":").await;
        cx.simulate_shared_keystrokes("v / a / d").await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {"
            a
            a
            ˇa"});
    }
}
