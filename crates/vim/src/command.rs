use std::sync::OnceLock;

use command_palette_hooks::CommandInterceptResult;
use editor::actions::{SortLinesCaseInsensitive, SortLinesCaseSensitive};
use gpui::{impl_actions, Action, AppContext, Global, ViewContext};
use serde_derive::Deserialize;
use util::ResultExt;
use workspace::{SaveIntent, Workspace};

use crate::{
    motion::{EndOfDocument, Motion, StartOfDocument},
    normal::{
        move_cursor,
        search::{range_regex, FindCommand, ReplaceCommand},
        JoinLines,
    },
    state::Mode,
    Vim,
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GoToLine {
    pub line: u32,
}

impl_actions!(vim, [GoToLine]);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, action: &GoToLine, cx| {
        Vim::update(cx, |vim, cx| {
            vim.switch_mode(Mode::Normal, false, cx);
            move_cursor(vim, Motion::StartOfDocument, Some(action.line as usize), cx);
        });
    });
}

struct VimCommand {
    prefix: &'static str,
    suffix: &'static str,
    action: Option<Box<dyn Action>>,
    action_name: Option<&'static str>,
    bang_action: Option<Box<dyn Action>>,
}

impl VimCommand {
    fn new(pattern: (&'static str, &'static str), action: impl Action) -> Self {
        Self {
            prefix: pattern.0,
            suffix: pattern.1,
            action: Some(action.boxed_clone()),
            action_name: None,
            bang_action: None,
        }
    }

    // from_str is used for actions in other crates.
    fn str(pattern: (&'static str, &'static str), action_name: &'static str) -> Self {
        Self {
            prefix: pattern.0,
            suffix: pattern.1,
            action: None,
            action_name: Some(action_name),
            bang_action: None,
        }
    }

    fn bang(mut self, bang_action: impl Action) -> Self {
        self.bang_action = Some(bang_action.boxed_clone());
        self
    }

    fn parse(&self, mut query: &str, cx: &AppContext) -> Option<Box<dyn Action>> {
        let has_bang = query.ends_with('!');
        if has_bang {
            query = &query[..query.len() - 1];
        }

        let Some(suffix) = query.strip_prefix(self.prefix) else {
            return None;
        };
        if !self.suffix.starts_with(suffix) {
            return None;
        }

        if has_bang && self.bang_action.is_some() {
            Some(self.bang_action.as_ref().unwrap().boxed_clone())
        } else if let Some(action) = self.action.as_ref() {
            Some(action.boxed_clone())
        } else if let Some(action_name) = self.action_name {
            cx.build_action(action_name, None).log_err()
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
        VimCommand::new(("sp", "lit"), workspace::SplitUp),
        VimCommand::new(("vs", "plit"), workspace::SplitLeft),
        VimCommand::new(
            ("bd", "elete"),
            workspace::CloseActiveItem {
                save_intent: Some(SaveIntent::Close),
            },
        )
        .bang(workspace::CloseActiveItem {
            save_intent: Some(SaveIntent::Skip),
        }),
        VimCommand::new(("bn", "ext"), workspace::ActivateNextItem),
        VimCommand::new(("bN", "ext"), workspace::ActivatePrevItem),
        VimCommand::new(("bp", "revious"), workspace::ActivatePrevItem),
        VimCommand::new(("bf", "irst"), workspace::ActivateItem(0)),
        VimCommand::new(("br", "ewind"), workspace::ActivateItem(0)),
        VimCommand::new(("bl", "ast"), workspace::ActivateLastItem),
        VimCommand::new(
            ("new", ""),
            workspace::NewFileInDirection(workspace::SplitDirection::Up),
        ),
        VimCommand::new(
            ("vne", "w"),
            workspace::NewFileInDirection(workspace::SplitDirection::Left),
        ),
        VimCommand::new(("tabe", "dit"), workspace::NewFile),
        VimCommand::new(("tabnew", ""), workspace::NewFile),
        VimCommand::new(("tabn", "ext"), workspace::ActivateNextItem),
        VimCommand::new(("tabp", "revious"), workspace::ActivatePrevItem),
        VimCommand::new(("tabN", "ext"), workspace::ActivatePrevItem),
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
            },
        )
        .bang(workspace::CloseInactiveItems {
            save_intent: Some(SaveIntent::Skip),
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
        VimCommand::new(("cn", "ext"), editor::actions::GoToDiagnostic),
        VimCommand::new(("cp", "revious"), editor::actions::GoToPrevDiagnostic),
        VimCommand::new(("cN", "ext"), editor::actions::GoToPrevDiagnostic),
        VimCommand::new(("lp", "revious"), editor::actions::GoToPrevDiagnostic),
        VimCommand::new(("lN", "ext"), editor::actions::GoToPrevDiagnostic),
        VimCommand::new(("j", "oin"), JoinLines),
        VimCommand::new(("d", "elete"), editor::actions::DeleteLine),
        VimCommand::new(("sor", "t"), SortLinesCaseSensitive),
        VimCommand::new(("sort i", ""), SortLinesCaseInsensitive),
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
        VimCommand::new(("$", ""), EndOfDocument),
        VimCommand::new(("%", ""), EndOfDocument),
        VimCommand::new(("0", ""), StartOfDocument),
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

pub fn command_interceptor(mut query: &str, cx: &AppContext) -> Option<CommandInterceptResult> {
    // Note: this is a very poor simulation of vim's command palette.
    // In the future we should adjust it to handle parsing range syntax,
    // and then calling the appropriate commands with/without ranges.
    //
    // We also need to support passing arguments to commands like :w
    // (ideally with filename autocompletion).
    while query.starts_with(':') {
        query = &query[1..];
    }

    for command in commands(cx).iter() {
        if let Some(action) = command.parse(query, cx) {
            let string = ":".to_owned() + command.prefix + command.suffix;
            let positions = generate_positions(&string, query);

            return Some(CommandInterceptResult {
                action,
                string,
                positions,
            });
        }
    }

    let (name, action) = if query.starts_with('/') || query.starts_with('?') {
        (
            query,
            FindCommand {
                query: query[1..].to_string(),
                backwards: query.starts_with('?'),
            }
            .boxed_clone(),
        )
    } else if query.starts_with('%') {
        (
            query,
            ReplaceCommand {
                query: query.to_string(),
            }
            .boxed_clone(),
        )
    } else if let Ok(line) = query.parse::<u32>() {
        (query, GoToLine { line }.boxed_clone())
    } else if range_regex().is_match(query) {
        (
            query,
            ReplaceCommand {
                query: query.to_string(),
            }
            .boxed_clone(),
        )
    } else {
        return None;
    };

    let string = ":".to_owned() + name;
    let positions = generate_positions(&string, query);

    Some(CommandInterceptResult {
        action,
        string,
        positions,
    })
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

    use crate::test::{NeovimBackedTestContext, VimTestContext};
    use gpui::TestAppContext;
    use indoc::indoc;

    #[gpui::test]
    async fn test_command_basics(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ˇa
            b
            c"})
            .await;

        cx.simulate_shared_keystrokes(": j enter").await;

        // hack: our cursor positionining after a join command is wrong
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
            c"})
            .await;
        cx.simulate_shared_keystrokes(": % s / b / d enter").await;
        cx.shared_state().await.assert_eq(indoc! {"
            a
            ˇd
            c"});
        cx.simulate_shared_keystrokes(": % s : . : \\ 0 \\ 0 enter")
            .await;
        cx.shared_state().await.assert_eq(indoc! {"
            aa
            dd
            ˇcc"});
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

        assert_eq!(fs.load(&path).await.unwrap(), "@\n");

        fs.as_fake().insert_file(path, b"oops\n".to_vec()).await;

        // conflict!
        cx.simulate_keystrokes("i @ escape");
        cx.simulate_keystrokes(": w enter");
        assert!(cx.has_pending_prompt());
        // "Cancel"
        cx.simulate_prompt_answer(0);
        assert_eq!(fs.load(&path).await.unwrap(), "oops\n");
        assert!(!cx.has_pending_prompt());
        // force overwrite
        cx.simulate_keystrokes(": w ! enter");
        assert!(!cx.has_pending_prompt());
        assert_eq!(fs.load(&path).await.unwrap(), "@@\n");
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
}
