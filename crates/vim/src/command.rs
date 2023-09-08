use command_palette::{humanize_action_name, CommandInterceptResult};
use gpui::{actions, impl_actions, Action, AppContext, AsyncAppContext, ViewContext};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use workspace::{SaveBehavior, Workspace};

use crate::{
    motion::{motion, Motion},
    normal::JoinLines,
    Vim,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoToLine {
    pub line: u32,
}

impl_actions!(vim, [GoToLine]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(|_: &mut Workspace, action: &GoToLine, cx| {
        Vim::update(cx, |vim, cx| {
            vim.push_operator(crate::state::Operator::Number(action.line as usize), cx)
        });
        motion(Motion::StartOfDocument, cx)
    });
}

pub fn command_interceptor(mut query: &str, _: &AppContext) -> Option<CommandInterceptResult> {
    while query.starts_with(":") {
        query = &query[1..];
    }

    let (name, action) = match query {
        // :w
        "w" | "wr" | "wri" | "writ" | "write" => (
            "write",
            workspace::Save {
                save_behavior: Some(SaveBehavior::PromptOnConflict),
            }
            .boxed_clone(),
        ),
        "w!" | "wr!" | "wri!" | "writ!" | "write!" => (
            "write",
            workspace::Save {
                save_behavior: Some(SaveBehavior::SilentlyOverwrite),
            }
            .boxed_clone(),
        ),

        // :q
        "q" | "qu" | "qui" | "quit" => (
            "quit",
            workspace::CloseActiveItem {
                save_behavior: Some(SaveBehavior::PromptOnWrite),
            }
            .boxed_clone(),
        ),
        "q!" | "qu!" | "qui!" | "quit!" => (
            "quit!",
            workspace::CloseActiveItem {
                save_behavior: Some(SaveBehavior::DontSave),
            }
            .boxed_clone(),
        ),

        // :wq
        "wq" => (
            "wq",
            workspace::CloseActiveItem {
                save_behavior: Some(SaveBehavior::PromptOnConflict),
            }
            .boxed_clone(),
        ),
        "wq!" => (
            "wq!",
            workspace::CloseActiveItem {
                save_behavior: Some(SaveBehavior::SilentlyOverwrite),
            }
            .boxed_clone(),
        ),
        // :x
        "x" | "xi" | "xit" | "exi" | "exit" => (
            "exit",
            workspace::CloseActiveItem {
                save_behavior: Some(SaveBehavior::PromptOnConflict),
            }
            .boxed_clone(),
        ),
        "x!" | "xi!" | "xit!" | "exi!" | "exit!" => (
            "xit",
            workspace::CloseActiveItem {
                save_behavior: Some(SaveBehavior::SilentlyOverwrite),
            }
            .boxed_clone(),
        ),

        // :wa
        "wa" | "wal" | "wall" => (
            "wall",
            workspace::SaveAll {
                save_behavior: Some(SaveBehavior::PromptOnConflict),
            }
            .boxed_clone(),
        ),
        "wa!" | "wal!" | "wall!" => (
            "wall!",
            workspace::SaveAll {
                save_behavior: Some(SaveBehavior::SilentlyOverwrite),
            }
            .boxed_clone(),
        ),

        // :qa
        "qa" | "qal" | "qall" | "quita" | "quital" | "quitall" => (
            "quitall",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::PromptOnWrite),
            }
            .boxed_clone(),
        ),
        "qa!" | "qal!" | "qall!" | "quita!" | "quital!" | "quitall!" => (
            "quitall!",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::DontSave),
            }
            .boxed_clone(),
        ),

        // :cq
        "cq" | "cqu" | "cqui" | "cquit" | "cq!" | "cqu!" | "cqui!" | "cquit!" => (
            "cquit!",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::DontSave),
            }
            .boxed_clone(),
        ),

        // :xa
        "xa" | "xal" | "xall" => (
            "xall",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::PromptOnConflict),
            }
            .boxed_clone(),
        ),
        "xa!" | "xal!" | "xall!" => (
            "zall!",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::SilentlyOverwrite),
            }
            .boxed_clone(),
        ),

        // :wqa
        "wqa" | "wqal" | "wqall" => (
            "wqall",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::PromptOnConflict),
            }
            .boxed_clone(),
        ),
        "wqa!" | "wqal!" | "wqall!" => (
            "wqall!",
            workspace::CloseAllItemsAndPanes {
                save_behavior: Some(SaveBehavior::SilentlyOverwrite),
            }
            .boxed_clone(),
        ),

        "j" | "jo" | "joi" | "join" => ("join", JoinLines.boxed_clone()),

        "sp" | "spl" | "spli" | "split" => ("split", workspace::SplitUp.boxed_clone()),
        "vs" | "vsp" | "vspl" | "vspli" | "vsplit" => {
            ("vsplit", workspace::SplitLeft.boxed_clone())
        }
        "cn" | "cne" | "cnex" | "cnext" => ("cnext", editor::GoToDiagnostic.boxed_clone()),
        "cp" | "cpr" | "cpre" | "cprev" => ("cprev", editor::GoToPrevDiagnostic.boxed_clone()),

        _ => {
            if let Ok(line) = query.parse::<u32>() {
                (query, GoToLine { line }.boxed_clone())
            } else {
                return None;
            }
        }
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
    let mut chars = query.chars().into_iter();

    let Some(mut current) = chars.next() else {
        return positions;
    };

    for (i, c) in string.chars().enumerate() {
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
