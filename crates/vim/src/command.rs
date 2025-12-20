use anyhow::{Result, anyhow};
use collections::{HashMap, HashSet};
use command_palette_hooks::{CommandInterceptItem, CommandInterceptResult};
use editor::{
    Bias, Editor, EditorSettings, SelectionEffects, ToPoint,
    actions::{SortLinesCaseInsensitive, SortLinesCaseSensitive},
    display_map::ToDisplayPoint,
};
use futures::AsyncWriteExt as _;
use gpui::{
    Action, App, AppContext as _, Context, Global, Keystroke, Task, WeakEntity, Window, actions,
};
use itertools::Itertools;
use language::Point;
use multi_buffer::MultiBufferRow;
use project::ProjectPath;
use regex::Regex;
use schemars::JsonSchema;
use search::{BufferSearchBar, SearchOptions};
use serde::Deserialize;
use settings::{Settings, SettingsStore};
use std::{
    iter::Peekable,
    ops::{Deref, Range},
    path::{Path, PathBuf},
    process::Stdio,
    str::Chars,
    sync::OnceLock,
    time::Instant,
};
use task::{HideStrategy, RevealStrategy, SpawnInTerminal, TaskId};
use ui::ActiveTheme;
use util::{
    ResultExt,
    paths::PathStyle,
    rel_path::{RelPath, RelPathBuf},
};
use workspace::{Item, SaveIntent, Workspace, notifications::NotifyResultExt};
use workspace::{SplitDirection, notifications::DetachAndPromptErr};
use zed_actions::{OpenDocs, RevealTarget};

use crate::{
    ToggleMarksView, ToggleRegistersView, Vim,
    motion::{EndOfDocument, Motion, MotionKind, StartOfDocument},
    normal::{
        JoinLines,
        search::{FindCommand, ReplaceCommand, Replacement},
    },
    object::Object,
    state::{Mark, Mode},
    visual::VisualDeleteLine,
};

/// Goes to the specified line number in the editor.
#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct GoToLine {
    range: CommandRange,
}

/// Yanks (copies) text based on the specified range.
#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct YankCommand {
    range: CommandRange,
}

/// Executes a command with the specified range.
#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct WithRange {
    restore_selection: bool,
    range: CommandRange,
    action: WrappedAction,
}

/// Executes a command with the specified count.
#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct WithCount {
    count: u32,
    action: WrappedAction,
}

#[derive(Clone, Deserialize, JsonSchema, PartialEq)]
pub enum VimOption {
    Wrap(bool),
    Number(bool),
    RelativeNumber(bool),
    IgnoreCase(bool),
}

impl VimOption {
    fn possible_commands(query: &str) -> Vec<CommandInterceptItem> {
        let mut prefix_of_options = Vec::new();
        let mut options = query.split(" ").collect::<Vec<_>>();
        let prefix = options.pop().unwrap_or_default();
        for option in options {
            if let Some(opt) = Self::from(option) {
                prefix_of_options.push(opt)
            } else {
                return vec![];
            }
        }

        Self::possibilities(prefix)
            .map(|possible| {
                let mut options = prefix_of_options.clone();
                options.push(possible);

                CommandInterceptItem {
                    string: format!(
                        ":set {}",
                        options.iter().map(|opt| opt.to_string()).join(" ")
                    ),
                    action: VimSet { options }.boxed_clone(),
                    positions: vec![],
                }
            })
            .collect()
    }

    fn possibilities(query: &str) -> impl Iterator<Item = Self> + '_ {
        [
            (None, VimOption::Wrap(true)),
            (None, VimOption::Wrap(false)),
            (None, VimOption::Number(true)),
            (None, VimOption::Number(false)),
            (None, VimOption::RelativeNumber(true)),
            (None, VimOption::RelativeNumber(false)),
            (Some("rnu"), VimOption::RelativeNumber(true)),
            (Some("nornu"), VimOption::RelativeNumber(false)),
            (None, VimOption::IgnoreCase(true)),
            (None, VimOption::IgnoreCase(false)),
            (Some("ic"), VimOption::IgnoreCase(true)),
            (Some("noic"), VimOption::IgnoreCase(false)),
        ]
        .into_iter()
        .filter(move |(prefix, option)| prefix.unwrap_or(option.to_string()).starts_with(query))
        .map(|(_, option)| option)
    }

    fn from(option: &str) -> Option<Self> {
        match option {
            "wrap" => Some(Self::Wrap(true)),
            "nowrap" => Some(Self::Wrap(false)),

            "number" => Some(Self::Number(true)),
            "nu" => Some(Self::Number(true)),
            "nonumber" => Some(Self::Number(false)),
            "nonu" => Some(Self::Number(false)),

            "relativenumber" => Some(Self::RelativeNumber(true)),
            "rnu" => Some(Self::RelativeNumber(true)),
            "norelativenumber" => Some(Self::RelativeNumber(false)),
            "nornu" => Some(Self::RelativeNumber(false)),

            "ignorecase" => Some(Self::IgnoreCase(true)),
            "ic" => Some(Self::IgnoreCase(true)),
            "noignorecase" => Some(Self::IgnoreCase(false)),
            "noic" => Some(Self::IgnoreCase(false)),

            _ => None,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            VimOption::Wrap(true) => "wrap",
            VimOption::Wrap(false) => "nowrap",
            VimOption::Number(true) => "number",
            VimOption::Number(false) => "nonumber",
            VimOption::RelativeNumber(true) => "relativenumber",
            VimOption::RelativeNumber(false) => "norelativenumber",
            VimOption::IgnoreCase(true) => "ignorecase",
            VimOption::IgnoreCase(false) => "noignorecase",
        }
    }
}

/// Sets vim options and configuration values.
#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct VimSet {
    options: Vec<VimOption>,
}

/// Saves the current file with optional save intent.
#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
struct VimSave {
    pub range: Option<CommandRange>,
    pub save_intent: Option<SaveIntent>,
    pub filename: String,
}

/// Deletes the specified marks from the editor.
#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
struct VimSplit {
    pub vertical: bool,
    pub filename: String,
}

#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
enum DeleteMarks {
    Marks(String),
    AllLocal,
}

actions!(
    vim,
    [
        /// Executes a command in visual mode.
        VisualCommand,
        /// Executes a command with a count prefix.
        CountCommand,
        /// Executes a shell command.
        ShellCommand,
        /// Indicates that an argument is required for the command.
        ArgumentRequired
    ]
);

/// Opens the specified file for editing.
#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
struct VimEdit {
    pub filename: String,
}

/// Pastes the specified file's contents.
#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
struct VimRead {
    pub range: Option<CommandRange>,
    pub filename: String,
}

#[derive(Clone, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
struct VimNorm {
    pub range: Option<CommandRange>,
    pub command: String,
}

#[derive(Debug)]
struct WrappedAction(Box<dyn Action>);

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

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    // Vim::action(editor, cx, |vim, action: &StartOfLine, window, cx| {
    Vim::action(editor, cx, |vim, action: &VimSet, _, cx| {
        for option in action.options.iter() {
            vim.update_editor(cx, |_, editor, cx| match option {
                VimOption::Wrap(true) => {
                    editor
                        .set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
                }
                VimOption::Wrap(false) => {
                    editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
                }
                VimOption::Number(enabled) => {
                    editor.set_show_line_numbers(*enabled, cx);
                }
                VimOption::RelativeNumber(enabled) => {
                    editor.set_relative_line_number(Some(*enabled), cx);
                }
                VimOption::IgnoreCase(enabled) => {
                    let mut settings = EditorSettings::get_global(cx).clone();
                    settings.search.case_sensitive = !*enabled;
                    SettingsStore::update(cx, |store, _| {
                        store.override_global(settings);
                    });
                }
            });
        }
    });
    Vim::action(editor, cx, |vim, _: &VisualCommand, window, cx| {
        let Some(workspace) = vim.workspace(window) else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, "'<,'>", window, cx);
        })
    });

    Vim::action(editor, cx, |vim, _: &ShellCommand, window, cx| {
        let Some(workspace) = vim.workspace(window) else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, "'<,'>!", window, cx);
        })
    });

    Vim::action(editor, cx, |_, _: &ArgumentRequired, window, cx| {
        let _ = window.prompt(
            gpui::PromptLevel::Critical,
            "Argument required",
            None,
            &["Cancel"],
            cx,
        );
    });

    Vim::action(editor, cx, |vim, _: &ShellCommand, window, cx| {
        let Some(workspace) = vim.workspace(window) else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, "'<,'>!", window, cx);
        })
    });

    Vim::action(editor, cx, |vim, action: &VimSave, window, cx| {
        if let Some(range) = &action.range {
            vim.update_editor(cx, |vim, editor, cx| {
                let Some(range) = range.buffer_range(vim, editor, window, cx).ok() else {
                    return;
                };
                let Some((line_ending, encoding, has_bom, text, whole_buffer)) = editor.buffer().update(cx, |multi, cx| {
                    Some(multi.as_singleton()?.update(cx, |buffer, _| {
                        (
                            buffer.line_ending(),
                            buffer.encoding(),
                            buffer.has_bom(),
                            buffer.as_rope().slice_rows(range.start.0..range.end.0 + 1),
                            range.start.0 == 0 && range.end.0 + 1 >= buffer.row_count(),
                        )
                    }))
                }) else {
                    return;
                };

                let filename = action.filename.clone();
                let filename = if filename.is_empty() {
                    let Some(file) = editor
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .and_then(|buffer| buffer.read(cx).file())
                    else {
                        let _ = window.prompt(
                            gpui::PromptLevel::Warning,
                            "No file name",
                            Some("Partial buffer write requires file name."),
                            &["Cancel"],
                            cx,
                        );
                        return;
                    };
                    file.path().display(file.path_style(cx)).to_string()
                } else {
                    filename
                };

                if action.filename.is_empty() {
                    if whole_buffer {
                        if let Some(workspace) = vim.workspace(window) {
                            workspace.update(cx, |workspace, cx| {
                                workspace
                                    .save_active_item(
                                        action.save_intent.unwrap_or(SaveIntent::Save),
                                        window,
                                        cx,
                                    )
                                    .detach_and_prompt_err("Failed to save", window, cx, |_, _, _| None);
                            });
                        }
                        return;
                    }
                    if Some(SaveIntent::Overwrite) != action.save_intent {
                        let _ = window.prompt(
                            gpui::PromptLevel::Warning,
                            "Use ! to write partial buffer",
                            Some("Overwriting the current file with selected buffer content requires '!'."),
                            &["Cancel"],
                            cx,
                        );
                        return;
                    }
                    editor.buffer().update(cx, |multi, cx| {
                        if let Some(buffer) = multi.as_singleton() {
                            buffer.update(cx, |buffer, _| buffer.set_conflict());
                        }
                    });
                };

                editor.project().unwrap().update(cx, |project, cx| {
                    let worktree = project.visible_worktrees(cx).next().unwrap();

                    worktree.update(cx, |worktree, cx| {
                        let path_style = worktree.path_style();
                        let Some(path) = RelPath::new(Path::new(&filename), path_style).ok() else {
                            return;
                        };

                        let rx = (worktree.entry_for_path(&path).is_some() && Some(SaveIntent::Overwrite) != action.save_intent).then(|| {
                            window.prompt(
                                gpui::PromptLevel::Warning,
                                &format!("{path:?} already exists. Do you want to replace it?"),
                                Some(
                                    "A file or folder with the same name already exists. Replacing it will overwrite its current contents.",
                                ),
                                &["Replace", "Cancel"],
                                cx
                            )
                        });
                        let filename = filename.clone();
                        cx.spawn_in(window, async move |this, cx| {
                            if let Some(rx) = rx
                                && Ok(0) != rx.await
                            {
                                return;
                            }

                            let _ = this.update_in(cx, |worktree, window, cx| {
                                let Some(path) = RelPath::new(Path::new(&filename), path_style).ok() else {
                                    return;
                                };
                                worktree
                                    .write_file(path.into_arc(), text.clone(), line_ending, encoding, has_bom, cx)
                                    .detach_and_prompt_err("Failed to write lines", window, cx, |_, _, _| None);
                            });
                        })
                        .detach();
                    });
                });
            });
            return;
        }
        if action.filename.is_empty() {
            if let Some(workspace) = vim.workspace(window) {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .save_active_item(
                            action.save_intent.unwrap_or(SaveIntent::Save),
                            window,
                            cx,
                        )
                        .detach_and_prompt_err("Failed to save", window, cx, |_, _, _| None);
                });
            }
            return;
        }
        vim.update_editor(cx, |_, editor, cx| {
            let Some(project) = editor.project().cloned() else {
                return;
            };
            let Some(worktree) = project.read(cx).visible_worktrees(cx).next() else {
                return;
            };
            let path_style = worktree.read(cx).path_style();
            let Ok(project_path) =
                RelPath::new(Path::new(&action.filename), path_style).map(|path| ProjectPath {
                    worktree_id: worktree.read(cx).id(),
                    path: path.into_arc(),
                })
            else {
                // TODO implement save_as with absolute path
                Task::ready(Err::<(), _>(anyhow!(
                    "Cannot save buffer with absolute path"
                )))
                .detach_and_prompt_err(
                    "Failed to save",
                    window,
                    cx,
                    |_, _, _| None,
                );
                return;
            };

            if project.read(cx).entry_for_path(&project_path, cx).is_some()
                && action.save_intent != Some(SaveIntent::Overwrite)
            {
                let answer = window.prompt(
                    gpui::PromptLevel::Critical,
                    &format!(
                        "{} already exists. Do you want to replace it?",
                        project_path.path.display(path_style)
                    ),
                    Some(
                        "A file or folder with the same name already exists. \
                        Replacing it will overwrite its current contents.",
                    ),
                    &["Replace", "Cancel"],
                    cx,
                );
                cx.spawn_in(window, async move |editor, cx| {
                    if answer.await.ok() != Some(0) {
                        return;
                    }

                    let _ = editor.update_in(cx, |editor, window, cx| {
                        editor
                            .save_as(project, project_path, window, cx)
                            .detach_and_prompt_err("Failed to :w", window, cx, |_, _, _| None);
                    });
                })
                .detach();
            } else {
                editor
                    .save_as(project, project_path, window, cx)
                    .detach_and_prompt_err("Failed to :w", window, cx, |_, _, _| None);
            }
        });
    });

    Vim::action(editor, cx, |vim, action: &VimSplit, window, cx| {
        let Some(workspace) = vim.workspace(window) else {
            return;
        };

        workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            let Some(worktree) = project.read(cx).visible_worktrees(cx).next() else {
                return;
            };
            let path_style = worktree.read(cx).path_style();
            let Some(path) = RelPath::new(Path::new(&action.filename), path_style).log_err() else {
                return;
            };
            let project_path = ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: path.into_arc(),
            };

            let direction = if action.vertical {
                SplitDirection::vertical(cx)
            } else {
                SplitDirection::horizontal(cx)
            };

            workspace
                .split_path_preview(project_path, false, Some(direction), window, cx)
                .detach_and_log_err(cx);
        })
    });

    Vim::action(editor, cx, |vim, action: &DeleteMarks, window, cx| {
        fn err(s: String, window: &mut Window, cx: &mut Context<Editor>) {
            let _ = window.prompt(
                gpui::PromptLevel::Critical,
                &format!("Invalid argument: {}", s),
                None,
                &["Cancel"],
                cx,
            );
        }
        vim.update_editor(cx, |vim, editor, cx| match action {
            DeleteMarks::Marks(s) => {
                if s.starts_with('-') || s.ends_with('-') || s.contains(['\'', '`']) {
                    err(s.clone(), window, cx);
                    return;
                }

                let to_delete = if s.len() < 3 {
                    Some(s.clone())
                } else {
                    s.chars()
                        .tuple_windows::<(_, _, _)>()
                        .map(|(a, b, c)| {
                            if b == '-' {
                                if match a {
                                    'a'..='z' => a <= c && c <= 'z',
                                    'A'..='Z' => a <= c && c <= 'Z',
                                    '0'..='9' => a <= c && c <= '9',
                                    _ => false,
                                } {
                                    Some((a..=c).collect_vec())
                                } else {
                                    None
                                }
                            } else if a == '-' {
                                if c == '-' { None } else { Some(vec![c]) }
                            } else if c == '-' {
                                if a == '-' { None } else { Some(vec![a]) }
                            } else {
                                Some(vec![a, b, c])
                            }
                        })
                        .fold_options(HashSet::<char>::default(), |mut set, chars| {
                            set.extend(chars.iter().copied());
                            set
                        })
                        .map(|set| set.iter().collect::<String>())
                };

                let Some(to_delete) = to_delete else {
                    err(s.clone(), window, cx);
                    return;
                };

                for c in to_delete.chars().filter(|c| !c.is_whitespace()) {
                    vim.delete_mark(c.to_string(), editor, window, cx);
                }
            }
            DeleteMarks::AllLocal => {
                for s in 'a'..='z' {
                    vim.delete_mark(s.to_string(), editor, window, cx);
                }
            }
        });
    });

    Vim::action(editor, cx, |vim, action: &VimEdit, window, cx| {
        vim.update_editor(cx, |vim, editor, cx| {
            let Some(workspace) = vim.workspace(window) else {
                return;
            };
            let Some(project) = editor.project().cloned() else {
                return;
            };
            let Some(worktree) = project.read(cx).visible_worktrees(cx).next() else {
                return;
            };
            let path_style = worktree.read(cx).path_style();
            let Some(path) = RelPath::new(Path::new(&action.filename), path_style).log_err() else {
                return;
            };
            let project_path = ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: path.into_arc(),
            };

            let _ = workspace.update(cx, |workspace, cx| {
                workspace
                    .open_path(project_path, None, true, window, cx)
                    .detach_and_log_err(cx);
            });
        });
    });

    Vim::action(editor, cx, |vim, action: &VimRead, window, cx| {
        vim.update_editor(cx, |vim, editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let end = if let Some(range) = action.range.clone() {
                let Some(multi_range) = range.buffer_range(vim, editor, window, cx).log_err()
                else {
                    return;
                };

                match &range.start {
                    // inserting text above the first line uses the command ":0r {name}"
                    Position::Line { row: 0, offset: 0 } if range.end.is_none() => {
                        snapshot.clip_point(Point::new(0, 0), Bias::Right)
                    }
                    _ => snapshot.clip_point(Point::new(multi_range.end.0 + 1, 0), Bias::Right),
                }
            } else {
                let end_row = editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .range()
                    .end
                    .row;
                snapshot.clip_point(Point::new(end_row + 1, 0), Bias::Right)
            };
            let is_end_of_file = end == snapshot.max_point();
            let edit_range = snapshot.anchor_before(end)..snapshot.anchor_before(end);

            let mut text = if is_end_of_file {
                String::from('\n')
            } else {
                String::new()
            };

            let mut task = None;
            if action.filename.is_empty() {
                text.push_str(
                    &editor
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .map(|buffer| buffer.read(cx).text())
                        .unwrap_or_default(),
                );
            } else {
                if let Some(project) = editor.project().cloned() {
                    project.update(cx, |project, cx| {
                        let Some(worktree) = project.visible_worktrees(cx).next() else {
                            return;
                        };
                        let path_style = worktree.read(cx).path_style();
                        let Some(path) =
                            RelPath::new(Path::new(&action.filename), path_style).log_err()
                        else {
                            return;
                        };
                        task =
                            Some(worktree.update(cx, |worktree, cx| worktree.load_file(&path, cx)));
                    });
                } else {
                    return;
                }
            };

            cx.spawn_in(window, async move |editor, cx| {
                if let Some(task) = task {
                    text.push_str(
                        &task
                            .await
                            .log_err()
                            .map(|loaded_file| loaded_file.text)
                            .unwrap_or_default(),
                    );
                }

                if !text.is_empty() && !is_end_of_file {
                    text.push('\n');
                }

                let _ = editor.update_in(cx, |editor, window, cx| {
                    editor.transact(window, cx, |editor, window, cx| {
                        editor.edit([(edit_range.clone(), text)], cx);
                        let snapshot = editor.buffer().read(cx).snapshot(cx);
                        editor.change_selections(Default::default(), window, cx, |s| {
                            let point = if is_end_of_file {
                                Point::new(
                                    edit_range.start.to_point(&snapshot).row.saturating_add(1),
                                    0,
                                )
                            } else {
                                Point::new(edit_range.start.to_point(&snapshot).row, 0)
                            };
                            s.select_ranges([point..point]);
                        })
                    });
                });
            })
            .detach();
        });
    });

    Vim::action(editor, cx, |vim, action: &VimNorm, window, cx| {
        let keystrokes = action
            .command
            .chars()
            .map(|c| Keystroke::parse(&c.to_string()).unwrap())
            .collect();
        vim.switch_mode(Mode::Normal, true, window, cx);
        let initial_selections =
            vim.update_editor(cx, |_, editor, _| editor.selections.disjoint_anchors_arc());
        if let Some(range) = &action.range {
            let result = vim.update_editor(cx, |vim, editor, cx| {
                let range = range.buffer_range(vim, editor, window, cx)?;
                editor.change_selections(
                    SelectionEffects::no_scroll().nav_history(false),
                    window,
                    cx,
                    |s| {
                        s.select_ranges(
                            (range.start.0..=range.end.0)
                                .map(|line| Point::new(line, 0)..Point::new(line, 0)),
                        );
                    },
                );
                anyhow::Ok(())
            });
            if let Some(Err(err)) = result {
                log::error!("Error selecting range: {}", err);
                return;
            }
        };

        let Some(workspace) = vim.workspace(window) else {
            return;
        };
        let task = workspace.update(cx, |workspace, cx| {
            workspace.send_keystrokes_impl(keystrokes, window, cx)
        });
        let had_range = action.range.is_some();

        cx.spawn_in(window, async move |vim, cx| {
            task.await;
            vim.update_in(cx, |vim, window, cx| {
                vim.update_editor(cx, |_, editor, cx| {
                    if had_range {
                        editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                            s.select_anchor_ranges([s.newest_anchor().range()]);
                        })
                    }
                });
                if matches!(vim.mode, Mode::Insert | Mode::Replace) {
                    vim.normal_before(&Default::default(), window, cx);
                } else {
                    vim.switch_mode(Mode::Normal, true, window, cx);
                }
                vim.update_editor(cx, |_, editor, cx| {
                    if let Some(first_sel) = initial_selections
                        && let Some(tx_id) = editor
                            .buffer()
                            .update(cx, |multi, cx| multi.last_transaction_id(cx))
                    {
                        let last_sel = editor.selections.disjoint_anchors_arc();
                        editor.modify_transaction_selection_history(tx_id, |old| {
                            old.0 = first_sel;
                            old.1 = Some(last_sel);
                        });
                    }
                });
            })
            .ok();
        })
        .detach();
    });

    Vim::action(editor, cx, |vim, _: &CountCommand, window, cx| {
        let Some(workspace) = vim.workspace(window) else {
            return;
        };
        let count = Vim::take_count(cx).unwrap_or(1);
        Vim::take_forced_motion(cx);
        let n = if count > 1 {
            format!(".,.+{}", count.saturating_sub(1))
        } else {
            ".".to_string()
        };
        workspace.update(cx, |workspace, cx| {
            command_palette::CommandPalette::toggle(workspace, &n, window, cx);
        })
    });

    Vim::action(editor, cx, |vim, action: &GoToLine, window, cx| {
        vim.switch_mode(Mode::Normal, false, window, cx);
        let result = vim.update_editor(cx, |vim, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let buffer_row = action.range.head().buffer_row(vim, editor, window, cx)?;
            let current = editor
                .selections
                .newest::<Point>(&editor.display_snapshot(cx));
            let target = snapshot
                .buffer_snapshot()
                .clip_point(Point::new(buffer_row.0, current.head().column), Bias::Left);
            editor.change_selections(Default::default(), window, cx, |s| {
                s.select_ranges([target..target]);
            });

            anyhow::Ok(())
        });
        if let Some(e @ Err(_)) = result {
            let Some(workspace) = vim.workspace(window) else {
                return;
            };
            workspace.update(cx, |workspace, cx| {
                e.notify_err(workspace, cx);
            });
        }
    });

    Vim::action(editor, cx, |vim, action: &YankCommand, window, cx| {
        vim.update_editor(cx, |vim, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            if let Ok(range) = action.range.buffer_range(vim, editor, window, cx) {
                let end = if range.end < snapshot.buffer_snapshot().max_row() {
                    Point::new(range.end.0 + 1, 0)
                } else {
                    snapshot.buffer_snapshot().max_point()
                };
                vim.copy_ranges(
                    editor,
                    MotionKind::Linewise,
                    true,
                    vec![Point::new(range.start.0, 0)..end],
                    window,
                    cx,
                )
            }
        });
    });

    Vim::action(editor, cx, |_, action: &WithCount, window, cx| {
        for _ in 0..action.count {
            window.dispatch_action(action.action.boxed_clone(), cx)
        }
    });

    Vim::action(editor, cx, |vim, action: &WithRange, window, cx| {
        let result = vim.update_editor(cx, |vim, editor, cx| {
            action.range.buffer_range(vim, editor, window, cx)
        });

        let range = match result {
            None => return,
            Some(e @ Err(_)) => {
                let Some(workspace) = vim.workspace(window) else {
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
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    let end = Point::new(range.end.0, snapshot.line_len(range.end));
                    s.select_ranges([end..Point::new(range.start.0, 0)]);
                });
                selections
            })
            .flatten();
        window.dispatch_action(action.action.boxed_clone(), cx);
        cx.defer_in(window, move |vim, window, cx| {
            vim.update_editor(cx, |_, editor, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
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

    Vim::action(editor, cx, |vim, action: &OnMatchingLines, window, cx| {
        action.run(vim, window, cx)
    });

    Vim::action(editor, cx, |vim, action: &ShellExec, window, cx| {
        action.run(vim, window, cx)
    })
}

#[derive(Default)]
struct VimCommand {
    prefix: &'static str,
    suffix: &'static str,
    action: Option<Box<dyn Action>>,
    action_name: Option<&'static str>,
    bang_action: Option<Box<dyn Action>>,
    args: Option<
        Box<dyn Fn(Box<dyn Action>, String) -> Option<Box<dyn Action>> + Send + Sync + 'static>,
    >,
    /// Optional range Range to use if no range is specified.
    default_range: Option<CommandRange>,
    range: Option<
        Box<
            dyn Fn(Box<dyn Action>, &CommandRange) -> Option<Box<dyn Action>>
                + Send
                + Sync
                + 'static,
        >,
    >,
    has_count: bool,
    has_filename: bool,
}

struct ParsedQuery {
    args: String,
    has_bang: bool,
    has_space: bool,
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

    fn args(
        mut self,
        f: impl Fn(Box<dyn Action>, String) -> Option<Box<dyn Action>> + Send + Sync + 'static,
    ) -> Self {
        self.args = Some(Box::new(f));
        self
    }

    fn filename(
        mut self,
        f: impl Fn(Box<dyn Action>, String) -> Option<Box<dyn Action>> + Send + Sync + 'static,
    ) -> Self {
        self.args = Some(Box::new(f));
        self.has_filename = true;
        self
    }

    fn range(
        mut self,
        f: impl Fn(Box<dyn Action>, &CommandRange) -> Option<Box<dyn Action>> + Send + Sync + 'static,
    ) -> Self {
        self.range = Some(Box::new(f));
        self
    }

    fn default_range(mut self, range: CommandRange) -> Self {
        self.default_range = Some(range);
        self
    }

    fn count(mut self) -> Self {
        self.has_count = true;
        self
    }

    fn generate_filename_completions(
        parsed_query: &ParsedQuery,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> Task<Vec<String>> {
        let ParsedQuery {
            args,
            has_bang: _,
            has_space: _,
        } = parsed_query;
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Vec::new());
        };

        let (task, args_path) = workspace.update(cx, |workspace, cx| {
            let prefix = workspace
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                .next()
                .or_else(std::env::home_dir)
                .unwrap_or_else(|| PathBuf::from(""));

            let rel_path = match RelPath::new(Path::new(&args), PathStyle::local()) {
                Ok(path) => path.to_rel_path_buf(),
                Err(_) => {
                    return (Task::ready(Ok(Vec::new())), RelPathBuf::new());
                }
            };

            let rel_path = if args.ends_with(PathStyle::local().primary_separator()) {
                rel_path
            } else {
                rel_path
                    .parent()
                    .map(|rel_path| rel_path.to_rel_path_buf())
                    .unwrap_or(RelPathBuf::new())
            };

            let task = workspace.project().update(cx, |project, cx| {
                let path = prefix
                    .join(rel_path.as_std_path())
                    .to_string_lossy()
                    .to_string();
                project.list_directory(path, cx)
            });

            (task, rel_path)
        });

        cx.background_spawn(async move {
            let directories = task.await.unwrap_or_default();
            directories
                .iter()
                .map(|dir| {
                    let path = RelPath::new(dir.path.as_path(), PathStyle::local())
                        .map(|cow| cow.into_owned())
                        .unwrap_or(RelPathBuf::new());
                    let mut path_string = args_path
                        .join(&path)
                        .display(PathStyle::local())
                        .to_string();
                    if dir.is_dir {
                        path_string.push_str(PathStyle::local().primary_separator());
                    }
                    path_string
                })
                .collect()
        })
    }

    fn get_parsed_query(&self, query: String) -> Option<ParsedQuery> {
        let rest = query
            .strip_prefix(self.prefix)?
            .to_string()
            .chars()
            .zip_longest(self.suffix.to_string().chars())
            .skip_while(|e| e.clone().both().map(|(s, q)| s == q).unwrap_or(false))
            .filter_map(|e| e.left())
            .collect::<String>();
        let has_bang = rest.starts_with('!');
        let has_space = rest.starts_with("! ") || rest.starts_with(' ');
        let args = if has_bang {
            rest.strip_prefix('!')?.trim().to_string()
        } else if rest.is_empty() {
            "".into()
        } else {
            rest.strip_prefix(' ')?.trim().to_string()
        };
        Some(ParsedQuery {
            args,
            has_bang,
            has_space,
        })
    }

    fn parse(
        &self,
        query: &str,
        range: &Option<CommandRange>,
        cx: &App,
    ) -> Option<Box<dyn Action>> {
        let ParsedQuery {
            args,
            has_bang,
            has_space: _,
        } = self.get_parsed_query(query.to_string())?;
        let action = if has_bang && self.bang_action.is_some() {
            self.bang_action.as_ref().unwrap().boxed_clone()
        } else if let Some(action) = self.action.as_ref() {
            action.boxed_clone()
        } else if let Some(action_name) = self.action_name {
            cx.build_action(action_name, None).log_err()?
        } else {
            return None;
        };

        let action = if args.is_empty() {
            action
        } else {
            // if command does not accept args and we have args then we should do no action
            self.args.as_ref()?(action, args)?
        };

        let range = range.as_ref().or(self.default_range.as_ref());
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
        window: &mut Window,
        cx: &mut App,
    ) -> Result<MultiBufferRow> {
        let snapshot = editor.snapshot(window, cx);
        let target = match self {
            Position::Line { row, offset } => {
                if let Some(anchor) = editor.active_excerpt(cx).and_then(|(_, buffer, _)| {
                    editor.buffer().read(cx).buffer_point_to_anchor(
                        &buffer,
                        Point::new(row.saturating_sub(1), 0),
                        cx,
                    )
                }) {
                    anchor
                        .to_point(&snapshot.buffer_snapshot())
                        .row
                        .saturating_add_signed(*offset)
                } else {
                    row.saturating_add_signed(offset.saturating_sub(1))
                }
            }
            Position::Mark { name, offset } => {
                let Some(Mark::Local(anchors)) =
                    vim.get_mark(&name.to_string(), editor, window, cx)
                else {
                    anyhow::bail!("mark {name} not set");
                };
                let Some(mark) = anchors.last() else {
                    anyhow::bail!("mark {name} contains empty anchors");
                };
                mark.to_point(&snapshot.buffer_snapshot())
                    .row
                    .saturating_add_signed(*offset)
            }
            Position::LastLine { offset } => snapshot
                .buffer_snapshot()
                .max_row()
                .0
                .saturating_add_signed(*offset),
            Position::CurrentLine { offset } => editor
                .selections
                .newest_anchor()
                .head()
                .to_point(&snapshot.buffer_snapshot())
                .row
                .saturating_add_signed(*offset),
        };

        Ok(MultiBufferRow(target).min(snapshot.buffer_snapshot().max_row()))
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

    /// Convert the `CommandRange` into a `Range<MultiBufferRow>`.
    pub(crate) fn buffer_range(
        &self,
        vim: &Vim,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Range<MultiBufferRow>> {
        let start = self.start.buffer_row(vim, editor, window, cx)?;
        let end = if let Some(end) = self.end.as_ref() {
            end.buffer_row(vim, editor, window, cx)?
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

    /// The `CommandRange` representing the entire buffer.
    fn buffer() -> Self {
        Self {
            start: Position::Line { row: 1, offset: 0 },
            end: Some(Position::LastLine { offset: 0 }),
        }
    }
}

fn generate_commands(_: &App) -> Vec<VimCommand> {
    vec![
        VimCommand::new(
            ("w", "rite"),
            VimSave {
                save_intent: Some(SaveIntent::Save),
                filename: "".into(),
                range: None,
            },
        )
        .bang(VimSave {
            save_intent: Some(SaveIntent::Overwrite),
            filename: "".into(),
            range: None,
        })
        .filename(|action, filename| {
            Some(
                VimSave {
                    save_intent: action
                        .as_any()
                        .downcast_ref::<VimSave>()
                        .and_then(|action| action.save_intent),
                    filename,
                    range: None,
                }
                .boxed_clone(),
            )
        })
        .range(|action, range| {
            let mut action: VimSave = action.as_any().downcast_ref::<VimSave>().unwrap().clone();
            action.range.replace(range.clone());
            Some(Box::new(action))
        }),
        VimCommand::new(("e", "dit"), editor::actions::ReloadFile)
            .bang(editor::actions::ReloadFile)
            .filename(|_, filename| Some(VimEdit { filename }.boxed_clone())),
        VimCommand::new(
            ("r", "ead"),
            VimRead {
                range: None,
                filename: "".into(),
            },
        )
        .filename(|_, filename| {
            Some(
                VimRead {
                    range: None,
                    filename,
                }
                .boxed_clone(),
            )
        })
        .range(|action, range| {
            let mut action: VimRead = action.as_any().downcast_ref::<VimRead>().unwrap().clone();
            action.range.replace(range.clone());
            Some(Box::new(action))
        }),
        VimCommand::new(("sp", "lit"), workspace::SplitHorizontal).filename(|_, filename| {
            Some(
                VimSplit {
                    vertical: false,
                    filename,
                }
                .boxed_clone(),
            )
        }),
        VimCommand::new(("vs", "plit"), workspace::SplitVertical).filename(|_, filename| {
            Some(
                VimSplit {
                    vertical: true,
                    filename,
                }
                .boxed_clone(),
            )
        }),
        VimCommand::new(("tabe", "dit"), workspace::NewFile)
            .filename(|_action, filename| Some(VimEdit { filename }.boxed_clone())),
        VimCommand::new(("tabnew", ""), workspace::NewFile)
            .filename(|_action, filename| Some(VimEdit { filename }.boxed_clone())),
        VimCommand::new(
            ("q", "uit"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Skip),
            close_pinned: true,
        }),
        VimCommand::new(
            ("wq", ""),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Save),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Overwrite),
            close_pinned: true,
        }),
        VimCommand::new(
            ("x", "it"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::SaveAll),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Overwrite),
            close_pinned: true,
        }),
        VimCommand::new(
            ("exi", "t"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::SaveAll),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Overwrite),
            close_pinned: true,
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
        VimCommand::new(
            ("bd", "elete"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Skip),
            close_pinned: true,
        }),
        VimCommand::new(
            ("norm", "al"),
            VimNorm {
                command: "".into(),
                range: None,
            },
        )
        .args(|_, args| {
            Some(
                VimNorm {
                    command: args,
                    range: None,
                }
                .boxed_clone(),
            )
        })
        .range(|action, range| {
            let mut action: VimNorm = action.as_any().downcast_ref::<VimNorm>().unwrap().clone();
            action.range.replace(range.clone());
            Some(Box::new(action))
        }),
        VimCommand::new(("bn", "ext"), workspace::ActivateNextItem).count(),
        VimCommand::new(("bN", "ext"), workspace::ActivatePreviousItem).count(),
        VimCommand::new(("bp", "revious"), workspace::ActivatePreviousItem).count(),
        VimCommand::new(("bf", "irst"), workspace::ActivateItem(0)),
        VimCommand::new(("br", "ewind"), workspace::ActivateItem(0)),
        VimCommand::new(("bl", "ast"), workspace::ActivateLastItem),
        VimCommand::str(("buffers", ""), "tab_switcher::ToggleAll"),
        VimCommand::str(("ls", ""), "tab_switcher::ToggleAll"),
        VimCommand::new(("new", ""), workspace::NewFileSplitHorizontal),
        VimCommand::new(("vne", "w"), workspace::NewFileSplitVertical),
        VimCommand::new(("tabn", "ext"), workspace::ActivateNextItem).count(),
        VimCommand::new(("tabp", "revious"), workspace::ActivatePreviousItem).count(),
        VimCommand::new(("tabN", "ext"), workspace::ActivatePreviousItem).count(),
        VimCommand::new(
            ("tabc", "lose"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
                close_pinned: false,
            },
        ),
        VimCommand::new(
            ("tabo", "nly"),
            workspace::CloseOtherItems {
                save_intent: Some(SaveIntent::Close),
                close_pinned: false,
            },
        )
        .bang(workspace::CloseOtherItems {
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
        VimCommand::new(("cn", "ext"), editor::actions::GoToDiagnostic::default())
            .range(wrap_count),
        VimCommand::new(
            ("cp", "revious"),
            editor::actions::GoToPreviousDiagnostic::default(),
        )
        .range(wrap_count),
        VimCommand::new(
            ("cN", "ext"),
            editor::actions::GoToPreviousDiagnostic::default(),
        )
        .range(wrap_count),
        VimCommand::new(
            ("lp", "revious"),
            editor::actions::GoToPreviousDiagnostic::default(),
        )
        .range(wrap_count),
        VimCommand::new(
            ("lN", "ext"),
            editor::actions::GoToPreviousDiagnostic::default(),
        )
        .range(wrap_count),
        VimCommand::new(("j", "oin"), JoinLines).range(select_range),
        VimCommand::new(("fo", "ld"), editor::actions::FoldSelectedRanges).range(act_on_range),
        VimCommand::new(("foldo", "pen"), editor::actions::UnfoldLines)
            .bang(editor::actions::UnfoldRecursive)
            .range(act_on_range),
        VimCommand::new(("foldc", "lose"), editor::actions::Fold)
            .bang(editor::actions::FoldRecursive)
            .range(act_on_range),
        VimCommand::new(("dif", "fupdate"), editor::actions::ToggleSelectedDiffHunks)
            .range(act_on_range),
        VimCommand::str(("rev", "ert"), "git::Restore").range(act_on_range),
        VimCommand::new(("d", "elete"), VisualDeleteLine).range(select_range),
        VimCommand::new(("y", "ank"), gpui::NoAction).range(|_, range| {
            Some(
                YankCommand {
                    range: range.clone(),
                }
                .boxed_clone(),
            )
        }),
        VimCommand::new(("reg", "isters"), ToggleRegistersView).bang(ToggleRegistersView),
        VimCommand::new(("di", "splay"), ToggleRegistersView).bang(ToggleRegistersView),
        VimCommand::new(("marks", ""), ToggleMarksView).bang(ToggleMarksView),
        VimCommand::new(("delm", "arks"), ArgumentRequired)
            .bang(DeleteMarks::AllLocal)
            .args(|_, args| Some(DeleteMarks::Marks(args).boxed_clone())),
        VimCommand::new(("sor", "t"), SortLinesCaseSensitive)
            .range(select_range)
            .default_range(CommandRange::buffer()),
        VimCommand::new(("sort i", ""), SortLinesCaseInsensitive)
            .range(select_range)
            .default_range(CommandRange::buffer()),
        VimCommand::str(("E", "xplore"), "project_panel::ToggleFocus"),
        VimCommand::str(("H", "explore"), "project_panel::ToggleFocus"),
        VimCommand::str(("L", "explore"), "project_panel::ToggleFocus"),
        VimCommand::str(("S", "explore"), "project_panel::ToggleFocus"),
        VimCommand::str(("Ve", "xplore"), "project_panel::ToggleFocus"),
        VimCommand::str(("te", "rm"), "terminal_panel::Toggle"),
        VimCommand::str(("T", "erm"), "terminal_panel::Toggle"),
        VimCommand::str(("C", "ollab"), "collab_panel::ToggleFocus"),
        VimCommand::str(("No", "tifications"), "notification_panel::ToggleFocus"),
        VimCommand::str(("A", "I"), "agent::ToggleFocus"),
        VimCommand::str(("G", "it"), "git_panel::ToggleFocus"),
        VimCommand::str(("D", "ebug"), "debug_panel::ToggleFocus"),
        VimCommand::new(("noh", "lsearch"), search::buffer_search::Dismiss),
        VimCommand::new(("$", ""), EndOfDocument),
        VimCommand::new(("%", ""), EndOfDocument),
        VimCommand::new(("0", ""), StartOfDocument),
        VimCommand::new(("ex", ""), editor::actions::ReloadFile).bang(editor::actions::ReloadFile),
        VimCommand::new(("cpp", "link"), editor::actions::CopyPermalinkToLine).range(act_on_range),
        VimCommand::str(("opt", "ions"), "zed::OpenDefaultSettings"),
        VimCommand::str(("map", ""), "vim::OpenDefaultKeymap"),
        VimCommand::new(("h", "elp"), OpenDocs),
    ]
}

struct VimCommands(Vec<VimCommand>);
// safety: we only ever access this from the main thread (as ensured by the cx argument)
// actions are not Sync so we can't otherwise use a OnceLock.
unsafe impl Sync for VimCommands {}
impl Global for VimCommands {}

fn commands(cx: &App) -> &Vec<VimCommand> {
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

pub fn command_interceptor(
    mut input: &str,
    workspace: WeakEntity<Workspace>,
    cx: &mut App,
) -> Task<CommandInterceptResult> {
    while input.starts_with(':') {
        input = &input[1..];
    }

    let (range, query) = VimCommand::parse_range(input);
    let range_prefix = input[0..(input.len() - query.len())].to_string();
    let has_trailing_space = query.ends_with(" ");
    let mut query = query.as_str().trim();

    let on_matching_lines = (query.starts_with('g') || query.starts_with('v'))
        .then(|| {
            let (pattern, range, search, invert) = OnMatchingLines::parse(query, &range)?;
            let start_idx = query.len() - pattern.len();
            query = query[start_idx..].trim();
            Some((range, search, invert))
        })
        .flatten();

    let mut action = if range.is_some() && query.is_empty() {
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
    } else if query.starts_with("se ") || query.starts_with("set ") {
        let (prefix, option) = query.split_once(' ').unwrap();
        let mut commands = VimOption::possible_commands(option);
        if !commands.is_empty() {
            let query = prefix.to_string() + " " + option;
            for command in &mut commands {
                command.positions = generate_positions(&command.string, &query);
            }
        }
        return Task::ready(CommandInterceptResult {
            results: commands,
            exclusive: false,
        });
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
    } else if query.contains('!') {
        ShellExec::parse(query, range.clone())
    } else if on_matching_lines.is_some() {
        commands(cx)
            .iter()
            .find_map(|command| command.parse(query, &range, cx))
    } else {
        None
    };

    if let Some((range, search, invert)) = on_matching_lines
        && let Some(ref inner) = action
    {
        action = Some(Box::new(OnMatchingLines {
            range,
            search,
            action: WrappedAction(inner.boxed_clone()),
            invert,
        }));
    };

    if let Some(action) = action {
        let string = input.to_string();
        let positions = generate_positions(&string, &(range_prefix + query));
        return Task::ready(CommandInterceptResult {
            results: vec![CommandInterceptItem {
                action,
                string,
                positions,
            }],
            exclusive: false,
        });
    }

    let Some((mut results, filenames)) =
        commands(cx).iter().enumerate().find_map(|(idx, command)| {
            let action = command.parse(query, &range, cx)?;
            let parsed_query = command.get_parsed_query(query.into())?;
            let display_string = ":".to_owned()
                + &range_prefix
                + command.prefix
                + command.suffix
                + if parsed_query.has_bang { "!" } else { "" };
            let space = if parsed_query.has_space { " " } else { "" };

            let string = format!("{}{}{}", &display_string, &space, &parsed_query.args);
            let positions = generate_positions(&string, &(range_prefix.clone() + query));

            let results = vec![CommandInterceptItem {
                action,
                string,
                positions,
            }];

            let no_args_positions =
                generate_positions(&display_string, &(range_prefix.clone() + query));

            // The following are valid autocomplete scenarios:
            // :w!filename.txt
            // :w filename.txt
            // :w[space]
            if !command.has_filename
                || (!has_trailing_space && !parsed_query.has_bang && parsed_query.args.is_empty())
            {
                return Some((results, None));
            }

            Some((
                results,
                Some((idx, parsed_query, display_string, no_args_positions)),
            ))
        })
    else {
        return Task::ready(CommandInterceptResult::default());
    };

    if let Some((cmd_idx, parsed_query, display_string, no_args_positions)) = filenames {
        let filenames = VimCommand::generate_filename_completions(&parsed_query, workspace, cx);
        cx.spawn(async move |cx| {
            let filenames = filenames.await;
            const MAX_RESULTS: usize = 100;
            let executor = cx.background_executor().clone();
            let mut candidates = Vec::with_capacity(filenames.len());

            for (idx, filename) in filenames.iter().enumerate() {
                candidates.push(fuzzy::StringMatchCandidate::new(idx, &filename));
            }
            let filenames = fuzzy::match_strings(
                &candidates,
                &parsed_query.args,
                false,
                true,
                MAX_RESULTS,
                &Default::default(),
                executor,
            )
            .await;

            for fuzzy::StringMatch {
                candidate_id: _,
                score: _,
                positions,
                string,
            } in filenames
            {
                let offset = display_string.len() + 1;
                let mut positions: Vec<_> = positions.iter().map(|&pos| pos + offset).collect();
                positions.splice(0..0, no_args_positions.clone());
                let string = format!("{display_string} {string}");
                let (range, query) = VimCommand::parse_range(&string[1..]);
                let action =
                    match cx.update(|cx| commands(cx).get(cmd_idx)?.parse(&query, &range, cx)) {
                        Ok(Some(action)) => action,
                        _ => continue,
                    };
                results.push(CommandInterceptItem {
                    action,
                    string,
                    positions,
                });
            }
            CommandInterceptResult {
                results,
                exclusive: true,
            }
        })
    } else {
        Task::ready(CommandInterceptResult {
            results,
            exclusive: false,
        })
    }
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

/// Applies a command to all lines matching a pattern.
#[derive(Debug, PartialEq, Clone, Action)]
#[action(namespace = vim, no_json, no_register)]
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
        query: &str,
        range: &Option<CommandRange>,
    ) -> Option<(String, CommandRange, String, bool)> {
        let mut global = "global".chars().peekable();
        let mut query_chars = query.chars().peekable();
        let mut invert = false;
        if query_chars.peek() == Some(&'v') {
            invert = true;
            query_chars.next();
        }
        while global
            .peek()
            .is_some_and(|char| Some(char) == query_chars.peek())
        {
            global.next();
            query_chars.next();
        }
        if !invert && query_chars.peek() == Some(&'!') {
            invert = true;
            query_chars.next();
        }
        let range = range.clone().unwrap_or(CommandRange {
            start: Position::Line { row: 0, offset: 0 },
            end: Some(Position::LastLine { offset: 0 }),
        });

        let delimiter = query_chars.next().filter(|c| {
            !c.is_alphanumeric() && *c != '"' && *c != '|' && *c != '\'' && *c != '!'
        })?;

        let mut search = String::new();
        let mut escaped = false;

        for c in query_chars.by_ref() {
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

        Some((query_chars.collect::<String>(), range, search, invert))
    }

    pub fn run(&self, vim: &mut Vim, window: &mut Window, cx: &mut Context<Vim>) {
        let result = vim.update_editor(cx, |vim, editor, cx| {
            self.range.buffer_range(vim, editor, window, cx)
        });

        let range = match result {
            None => return,
            Some(e @ Err(_)) => {
                let Some(workspace) = vim.workspace(window) else {
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
                let Some(workspace) = vim.workspace(window) else {
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

        if let Some(pane) = vim.pane(window, cx) {
            pane.update(cx, |pane, cx| {
                if let Some(search_bar) = pane.toolbar().read(cx).item_of_type::<BufferSearchBar>()
                {
                    search_bar.update(cx, |search_bar, cx| {
                        if search_bar.show(window, cx) {
                            let _ = search_bar.search(
                                &last_pattern,
                                Some(SearchOptions::REGEX | SearchOptions::CASE_SENSITIVE),
                                false,
                                window,
                                cx,
                            );
                        }
                    });
                }
            });
        };

        vim.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let mut row = range.start.0;

            let point_range = Point::new(range.start.0, 0)
                ..snapshot
                    .buffer_snapshot()
                    .clip_point(Point::new(range.end.0 + 1, 0), Bias::Left);
            cx.spawn_in(window, async move |editor, cx| {
                let new_selections = cx
                    .background_spawn(async move {
                        let mut line = String::new();
                        let mut new_selections = Vec::new();
                        let chunks = snapshot
                            .buffer_snapshot()
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
                    .update_in(cx, |editor, window, cx| {
                        editor.start_transaction_at(Instant::now(), window, cx);
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.replace_cursors_with(|_| new_selections);
                        });
                        window.dispatch_action(action, cx);
                        cx.defer_in(window, move |editor, window, cx| {
                            let newest = editor
                                .selections
                                .newest::<Point>(&editor.display_snapshot(cx));
                            editor.change_selections(
                                SelectionEffects::no_scroll(),
                                window,
                                cx,
                                |s| {
                                    s.select(vec![newest]);
                                },
                            );
                            editor.end_transaction_at(Instant::now(), cx);
                        })
                    })
                    .ok();
            })
            .detach();
        });
    }
}

/// Executes a shell command and returns the output.
#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = vim, no_json, no_register)]
pub struct ShellExec {
    command: String,
    range: Option<CommandRange>,
    is_read: bool,
}

impl Vim {
    pub fn cancel_running_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.running_command.take().is_some() {
            self.update_editor(cx, |_, editor, cx| {
                editor.transact(window, cx, |editor, _window, _cx| {
                    editor.clear_row_highlights::<ShellExec>();
                })
            });
        }
    }

    fn prepare_shell_command(
        &mut self,
        command: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> String {
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
                        if let Some((_, buffer, _)) = editor.active_excerpt(cx)
                            && let Some(file) = buffer.read(cx).file()
                            && let Some(local) = file.as_local()
                        {
                            ret.push_str(&local.path().display(local.path_style(cx)));
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
        forced_motion: bool,
        window: &mut Window,
        cx: &mut Context<Vim>,
    ) {
        self.stop_recording(cx);
        let Some(workspace) = self.workspace(window) else {
            return;
        };
        let command = self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let start = editor
                .selections
                .newest_display(&editor.display_snapshot(cx));
            let text_layout_details = editor.text_layout_details(window);
            let (mut range, _) = motion
                .range(
                    &snapshot,
                    start.clone(),
                    times,
                    &text_layout_details,
                    forced_motion,
                )
                .unwrap_or((start.range(), MotionKind::Exclusive));
            if range.start != start.start {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
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
                command_palette::CommandPalette::toggle(workspace, &command, window, cx);
            });
        }
    }

    pub fn shell_command_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Vim>,
    ) {
        self.stop_recording(cx);
        let Some(workspace) = self.workspace(window) else {
            return;
        };
        let command = self.update_editor(cx, |_, editor, cx| {
            let snapshot = editor.snapshot(window, cx);
            let start = editor
                .selections
                .newest_display(&editor.display_snapshot(cx));
            let range = object
                .range(&snapshot, start.clone(), around, None)
                .unwrap_or(start.range());
            if range.start != start.start {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
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
                command_palette::CommandPalette::toggle(workspace, &command, window, cx);
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

    pub fn run(&self, vim: &mut Vim, window: &mut Window, cx: &mut Context<Vim>) {
        let Some(workspace) = vim.workspace(window) else {
            return;
        };

        let project = workspace.read(cx).project().clone();
        let command = vim.prepare_shell_command(&self.command, window, cx);

        if self.range.is_none() && !self.is_read {
            workspace.update(cx, |workspace, cx| {
                let project = workspace.project().read(cx);
                let cwd = project.first_project_directory(cx);
                let shell = project.terminal_settings(&cwd, cx).shell.clone();

                let spawn_in_terminal = SpawnInTerminal {
                    id: TaskId("vim".to_string()),
                    full_label: command.clone(),
                    label: command.clone(),
                    command: Some(command.clone()),
                    args: Vec::new(),
                    command_label: command.clone(),
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
                    show_rerun: false,
                };

                let task_status = workspace.spawn_in_terminal(spawn_in_terminal, window, cx);
                cx.background_spawn(async move {
                    match task_status.await {
                        Some(Ok(status)) => {
                            if status.success() {
                                log::debug!("Vim shell exec succeeded");
                            } else {
                                log::debug!("Vim shell exec failed, code: {:?}", status.code());
                            }
                        }
                        Some(Err(e)) => log::error!("Vim shell exec failed: {e}"),
                        None => log::debug!("Vim shell exec got cancelled"),
                    }
                })
                .detach();
            });
            return;
        };

        let mut input_snapshot = None;
        let mut input_range = None;
        let mut needs_newline_prefix = false;
        vim.update_editor(cx, |vim, editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let range = if let Some(range) = self.range.clone() {
                let Some(range) = range.buffer_range(vim, editor, window, cx).log_err() else {
                    return;
                };
                Point::new(range.start.0, 0)
                    ..snapshot.clip_point(Point::new(range.end.0 + 1, 0), Bias::Right)
            } else {
                let mut end = editor
                    .selections
                    .newest::<Point>(&editor.display_snapshot(cx))
                    .range()
                    .end;
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
                Default::default(),
                cx,
            );

            if !self.is_read {
                input_snapshot = Some(snapshot)
            }
        });

        let Some(range) = input_range else { return };

        let process_task = project.update(cx, |project, cx| project.exec_in_shell(command, cx));

        let is_read = self.is_read;

        let task = cx.spawn_in(window, async move |vim, cx| {
            let Some(mut process) = process_task.await.log_err() else {
                return;
            };
            process.stdout(Stdio::piped());
            process.stderr(Stdio::piped());

            if input_snapshot.is_some() {
                process.stdin(Stdio::piped());
            } else {
                process.stdin(Stdio::null());
            };

            let Some(mut running) = process.spawn().log_err() else {
                vim.update_in(cx, |vim, window, cx| {
                    vim.cancel_running_command(window, cx);
                })
                .log_err();
                return;
            };

            if let Some(mut stdin) = running.stdin.take()
                && let Some(snapshot) = input_snapshot
            {
                let range = range.clone();
                cx.background_spawn(async move {
                    for chunk in snapshot.text_for_range(range) {
                        if stdin.write_all(chunk.as_bytes()).await.log_err().is_none() {
                            return;
                        }
                    }
                    stdin.flush().await.log_err();
                })
                .detach();
            };

            let output = cx.background_spawn(running.output()).await;

            let Some(output) = output.log_err() else {
                vim.update_in(cx, |vim, window, cx| {
                    vim.cancel_running_command(window, cx);
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

            vim.update_in(cx, |vim, window, cx| {
                vim.update_editor(cx, |_, editor, cx| {
                    editor.transact(window, cx, |editor, window, cx| {
                        editor.edit([(range.clone(), text)], cx);
                        let snapshot = editor.buffer().read(cx).snapshot(cx);
                        editor.change_selections(Default::default(), window, cx, |s| {
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
                vim.cancel_running_command(window, cx);
            })
            .log_err();
        });
        vim.running_command.replace(task);
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use crate::{
        VimAddon,
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use editor::{Editor, EditorSettings};
    use gpui::{Context, TestAppContext};
    use indoc::indoc;
    use settings::Settings;
    use util::path;
    use workspace::{OpenOptions, Workspace};

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
        cx.simulate_shared_keystrokes("k : s / d d / e e enter")
            .await;
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
        let path = Path::new(path!("/root/dir/file.rs"));
        let fs = cx.workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());

        cx.simulate_keystrokes("i @ escape");
        cx.simulate_keystrokes(": w enter");

        assert_eq!(fs.load(path).await.unwrap().replace("\r\n", "\n"), "@\n");

        fs.as_fake().insert_file(path, b"oops\n".to_vec()).await;

        // conflict!
        cx.simulate_keystrokes("i @ escape");
        cx.simulate_keystrokes(": w enter");
        cx.simulate_prompt_answer("Cancel");

        assert_eq!(fs.load(path).await.unwrap().replace("\r\n", "\n"), "oops\n");
        assert!(!cx.has_pending_prompt());
        cx.simulate_keystrokes(": w !");
        cx.simulate_keystrokes("enter");
        assert!(!cx.has_pending_prompt());
        assert_eq!(fs.load(path).await.unwrap().replace("\r\n", "\n"), "@@\n");
    }

    #[gpui::test]
    async fn test_command_read(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        let fs = cx.workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        let path = Path::new(path!("/root/dir/other.rs"));
        fs.as_fake().insert_file(path, "1\n2\n3".into()).await;

        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file.rs"), "", cx);
        });

        // File without trailing newline
        cx.set_state("one\ntwo\nthreeˇ", Mode::Normal);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\ntwo\nthree\nˇ1\n2\n3", Mode::Normal);

        cx.set_state("oneˇ\ntwo\nthree", Mode::Normal);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\nˇ1\n2\n3\ntwo\nthree", Mode::Normal);

        cx.set_state("one\nˇtwo\nthree", Mode::Normal);
        cx.simulate_keystrokes(": 0 r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("ˇ1\n2\n3\none\ntwo\nthree", Mode::Normal);

        cx.set_state("one\n«ˇtwo\nthree\nfour»\nfive", Mode::Visual);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();
        cx.assert_state("one\ntwo\nthree\nfour\nˇ1\n2\n3\nfive", Mode::Normal);

        // Empty filename
        cx.set_state("oneˇ\ntwo\nthree", Mode::Normal);
        cx.simulate_keystrokes(": r");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\nˇone\ntwo\nthree\ntwo\nthree", Mode::Normal);

        // File with trailing newline
        fs.as_fake().insert_file(path, "1\n2\n3\n".into()).await;
        cx.set_state("one\ntwo\nthreeˇ", Mode::Normal);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\ntwo\nthree\nˇ1\n2\n3\n", Mode::Normal);

        cx.set_state("oneˇ\ntwo\nthree", Mode::Normal);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\nˇ1\n2\n3\n\ntwo\nthree", Mode::Normal);

        cx.set_state("one\n«ˇtwo\nthree\nfour»\nfive", Mode::Visual);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\ntwo\nthree\nfour\nˇ1\n2\n3\n\nfive", Mode::Normal);

        cx.set_state("«one\ntwo\nthreeˇ»", Mode::Visual);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\ntwo\nthree\nˇ1\n2\n3\n", Mode::Normal);

        // Empty file
        fs.as_fake().insert_file(path, "".into()).await;
        cx.set_state("ˇone\ntwo\nthree", Mode::Normal);
        cx.simulate_keystrokes(": r space d i r / o t h e r . r s");
        cx.simulate_keystrokes("enter");
        cx.assert_state("one\nˇtwo\nthree", Mode::Normal);
    }

    #[gpui::test]
    async fn test_command_quit(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.simulate_keystrokes(": n e w enter");
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.simulate_keystrokes(": q enter");
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 1));
        cx.simulate_keystrokes(": n e w enter");
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.simulate_keystrokes(": q a enter");
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 0));
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

    #[track_caller]
    fn assert_active_item(
        workspace: &mut Workspace,
        expected_path: &str,
        expected_text: &str,
        cx: &mut Context<Workspace>,
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
        assert_eq!(file_path, Path::new(expected_path));
    }

    #[gpui::test]
    async fn test_command_gf(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Assert base state, that we're in /root/dir/file.rs
        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file.rs"), "", cx);
        });

        // Insert a new file
        let fs = cx.workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file2.rs"),
                "This is file2.rs".as_bytes().to_vec(),
            )
            .await;
        fs.as_fake()
            .insert_file(
                path!("/root/dir/file3.rs"),
                "go to file3".as_bytes().to_vec(),
            )
            .await;

        // Put the path to the second file into the currently open buffer
        cx.set_state(indoc! {"go to fiˇle2.rs"}, Mode::Normal);

        // Go to file2.rs
        cx.simulate_keystrokes("g f");

        // We now have two items
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));
        cx.workspace(|workspace, _, cx| {
            assert_active_item(
                workspace,
                path!("/root/dir/file2.rs"),
                "This is file2.rs",
                cx,
            );
        });

        // Update editor to point to `file2.rs`
        cx.editor =
            cx.workspace(|workspace, _, cx| workspace.active_item_as::<Editor>(cx).unwrap());

        // Put the path to the third file into the currently open buffer,
        // but remove its suffix, because we want that lookup to happen automatically.
        cx.set_state(indoc! {"go to fiˇle3"}, Mode::Normal);

        // Go to file3.rs
        cx.simulate_keystrokes("g f");

        // We now have three items
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 3));
        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file3.rs"), "go to file3", cx);
        });
    }

    #[gpui::test]
    async fn test_command_write_filename(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file.rs"), "", cx);
        });

        cx.simulate_keystrokes(": w space other.rs");
        cx.simulate_keystrokes("enter");

        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/other.rs"), "", cx);
        });

        cx.simulate_keystrokes(": w space dir/file.rs");
        cx.simulate_keystrokes("enter");

        cx.simulate_prompt_answer("Replace");
        cx.run_until_parked();

        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file.rs"), "", cx);
        });

        cx.simulate_keystrokes(": w ! space other.rs");
        cx.simulate_keystrokes("enter");

        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/other.rs"), "", cx);
        });
    }

    #[gpui::test]
    async fn test_command_write_range(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file.rs"), "", cx);
        });

        cx.set_state(
            indoc! {"
                    The quick
                    brown« fox
                    jumpsˇ» over
                    the lazy dog
                "},
            Mode::Visual,
        );

        cx.simulate_keystrokes(": w space dir/other.rs");
        cx.simulate_keystrokes("enter");

        let other = path!("/root/dir/other.rs");

        let _ = cx
            .workspace(|workspace, window, cx| {
                workspace.open_abs_path(PathBuf::from(other), OpenOptions::default(), window, cx)
            })
            .await;

        cx.workspace(|workspace, _, cx| {
            assert_active_item(
                workspace,
                other,
                indoc! {"
                        brown fox
                        jumps over
                    "},
                cx,
            );
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

    #[gpui::test]
    async fn test_del_marks(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇa
            b
            a
            b
            a
        "})
            .await;

        cx.simulate_shared_keystrokes("m a").await;

        let mark = cx.update_editor(|editor, window, cx| {
            let vim = editor.addon::<VimAddon>().unwrap().entity.clone();
            vim.update(cx, |vim, cx| vim.get_mark("a", editor, window, cx))
        });
        assert!(mark.is_some());

        cx.simulate_shared_keystrokes(": d e l m space a").await;
        cx.simulate_shared_keystrokes("enter").await;

        let mark = cx.update_editor(|editor, window, cx| {
            let vim = editor.addon::<VimAddon>().unwrap().entity.clone();
            vim.update(cx, |vim, cx| vim.get_mark("a", editor, window, cx))
        });
        assert!(mark.is_none())
    }

    #[gpui::test]
    async fn test_normal_command(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            The quick
            brown« fox
            jumpsˇ» over
            the lazy dog
        "})
            .await;

        cx.simulate_shared_keystrokes(": n o r m space w C w o r d")
            .await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {"
            The quick
            brown word
            jumps worˇd
            the lazy dog
        "});

        cx.simulate_shared_keystrokes(": n o r m space _ w c i w t e s t")
            .await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {"
            The quick
            brown word
            jumps tesˇt
            the lazy dog
        "});

        cx.simulate_shared_keystrokes("_ l v l : n o r m space s l a")
            .await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {"
            The quick
            brown word
            lˇaumps test
            the lazy dog
        "});

        cx.set_shared_state(indoc! {"
            ˇThe quick
            brown fox
            jumps over
            the lazy dog
        "})
            .await;

        cx.simulate_shared_keystrokes("c i w M y escape").await;

        cx.shared_state().await.assert_eq(indoc! {"
            Mˇy quick
            brown fox
            jumps over
            the lazy dog
        "});

        cx.simulate_shared_keystrokes(": n o r m space u").await;
        cx.simulate_shared_keystrokes("enter").await;

        cx.shared_state().await.assert_eq(indoc! {"
            ˇThe quick
            brown fox
            jumps over
            the lazy dog
        "});
        // Once ctrl-v to input character literals is added there should be a test for redo
    }

    #[gpui::test]
    async fn test_command_tabnew(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Create a new file to ensure that, when the filename is used with
        // `:tabnew`, it opens the existing file in a new tab.
        let fs = cx.workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(path!("/root/dir/file_2.rs"), "file_2".as_bytes().to_vec())
            .await;

        cx.simulate_keystrokes(": tabnew");
        cx.simulate_keystrokes("enter");
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));

        // Assert that the new tab is empty and not associated with any file, as
        // no file path was provided to the `:tabnew` command.
        cx.workspace(|workspace, _window, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
            let buffer = active_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap();

            assert!(&buffer.read(cx).file().is_none());
        });

        // Leverage the filename as an argument to the `:tabnew` command,
        // ensuring that the file, instead of an empty buffer, is opened in a
        // new tab.
        cx.simulate_keystrokes(": tabnew space dir/file_2.rs");
        cx.simulate_keystrokes("enter");

        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 3));
        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file_2.rs"), "file_2", cx);
        });

        // If the `filename` argument provided to the `:tabnew` command is for a
        // file that doesn't yet exist, it should still associate the buffer
        // with that file path, so that when the buffer contents are saved, the
        // file is created.
        cx.simulate_keystrokes(": tabnew space dir/file_3.rs");
        cx.simulate_keystrokes("enter");

        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 4));
        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file_3.rs"), "", cx);
        });
    }

    #[gpui::test]
    async fn test_command_tabedit(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // Create a new file to ensure that, when the filename is used with
        // `:tabedit`, it opens the existing file in a new tab.
        let fs = cx.workspace(|workspace, _, cx| workspace.project().read(cx).fs().clone());
        fs.as_fake()
            .insert_file(path!("/root/dir/file_2.rs"), "file_2".as_bytes().to_vec())
            .await;

        cx.simulate_keystrokes(": tabedit");
        cx.simulate_keystrokes("enter");
        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 2));

        // Assert that the new tab is empty and not associated with any file, as
        // no file path was provided to the `:tabedit` command.
        cx.workspace(|workspace, _window, cx| {
            let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
            let buffer = active_editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .unwrap();

            assert!(&buffer.read(cx).file().is_none());
        });

        // Leverage the filename as an argument to the `:tabedit` command,
        // ensuring that the file, instead of an empty buffer, is opened in a
        // new tab.
        cx.simulate_keystrokes(": tabedit space dir/file_2.rs");
        cx.simulate_keystrokes("enter");

        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 3));
        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file_2.rs"), "file_2", cx);
        });

        // If the `filename` argument provided to the `:tabedit` command is for a
        // file that doesn't yet exist, it should still associate the buffer
        // with that file path, so that when the buffer contents are saved, the
        // file is created.
        cx.simulate_keystrokes(": tabedit space dir/file_3.rs");
        cx.simulate_keystrokes("enter");

        cx.workspace(|workspace, _, cx| assert_eq!(workspace.items(cx).count(), 4));
        cx.workspace(|workspace, _, cx| {
            assert_active_item(workspace, path!("/root/dir/file_3.rs"), "", cx);
        });
    }

    #[gpui::test]
    async fn test_ignorecase_command(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.read(|cx| {
            assert_eq!(
                EditorSettings::get_global(cx).search.case_sensitive,
                false,
                "The `case_sensitive` setting should be `false` by default."
            );
        });
        cx.simulate_keystrokes(": set space noignorecase");
        cx.simulate_keystrokes("enter");
        cx.read(|cx| {
            assert_eq!(
                EditorSettings::get_global(cx).search.case_sensitive,
                true,
                "The `case_sensitive` setting should have been enabled with `:set noignorecase`."
            );
        });
        cx.simulate_keystrokes(": set space ignorecase");
        cx.simulate_keystrokes("enter");
        cx.read(|cx| {
            assert_eq!(
                EditorSettings::get_global(cx).search.case_sensitive,
                false,
                "The `case_sensitive` setting should have been disabled with `:set ignorecase`."
            );
        });
        cx.simulate_keystrokes(": set space noic");
        cx.simulate_keystrokes("enter");
        cx.read(|cx| {
            assert_eq!(
                EditorSettings::get_global(cx).search.case_sensitive,
                true,
                "The `case_sensitive` setting should have been enabled with `:set noic`."
            );
        });
        cx.simulate_keystrokes(": set space ic");
        cx.simulate_keystrokes("enter");
        cx.read(|cx| {
            assert_eq!(
                EditorSettings::get_global(cx).search.case_sensitive,
                false,
                "The `case_sensitive` setting should have been disabled with `:set ic`."
            );
        });
    }

    #[gpui::test]
    async fn test_sort_commands(cx: &mut TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
                «hornet
                quirrel
                elderbug
                cornifer
                idaˇ»
            "},
            Mode::Visual,
        );

        cx.simulate_keystrokes(": sort");
        cx.simulate_keystrokes("enter");

        cx.assert_state(
            indoc! {"
                ˇcornifer
                elderbug
                hornet
                ida
                quirrel
            "},
            Mode::Normal,
        );

        // Assert that, by default, `:sort` takes case into consideration.
        cx.set_state(
            indoc! {"
                «hornet
                quirrel
                Elderbug
                cornifer
                idaˇ»
            "},
            Mode::Visual,
        );

        cx.simulate_keystrokes(": sort");
        cx.simulate_keystrokes("enter");

        cx.assert_state(
            indoc! {"
                ˇElderbug
                cornifer
                hornet
                ida
                quirrel
            "},
            Mode::Normal,
        );

        // Assert that, if the `i` option is passed, `:sort` ignores case.
        cx.set_state(
            indoc! {"
                «hornet
                quirrel
                Elderbug
                cornifer
                idaˇ»
            "},
            Mode::Visual,
        );

        cx.simulate_keystrokes(": sort space i");
        cx.simulate_keystrokes("enter");

        cx.assert_state(
            indoc! {"
                ˇcornifer
                Elderbug
                hornet
                ida
                quirrel
            "},
            Mode::Normal,
        );

        // When no range is provided, sorts the whole buffer.
        cx.set_state(
            indoc! {"
                ˇhornet
                quirrel
                elderbug
                cornifer
                ida
            "},
            Mode::Normal,
        );

        cx.simulate_keystrokes(": sort");
        cx.simulate_keystrokes("enter");

        cx.assert_state(
            indoc! {"
                ˇcornifer
                elderbug
                hornet
                ida
                quirrel
            "},
            Mode::Normal,
        );
    }
}
