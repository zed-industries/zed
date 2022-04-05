use crate::AppState;
use gpui::{Menu, MenuItem};
use std::sync::Arc;

#[cfg(target_os = "macos")]
pub fn menus(state: &Arc<AppState>) -> Vec<Menu<'static>> {
    vec![
        Menu {
            name: "Zed",
            items: vec![
                MenuItem::Action {
                    name: "About Zed…",
                    keystroke: None,
                    action: Box::new(super::About),
                },
                MenuItem::Action {
                    name: "Check for Updates",
                    keystroke: None,
                    action: Box::new(super::CheckForUpdates),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Quit",
                    keystroke: Some("cmd-q"),
                    action: Box::new(super::Quit),
                },
            ],
        },
        Menu {
            name: "File",
            items: vec![
                MenuItem::Action {
                    name: "New",
                    keystroke: Some("cmd-n"),
                    action: Box::new(workspace::OpenNew(state.clone())),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Open…",
                    keystroke: Some("cmd-o"),
                    action: Box::new(workspace::Open(state.clone())),
                },
            ],
        },
        Menu {
            name: "Edit",
            items: vec![
                MenuItem::Action {
                    name: "Undo",
                    keystroke: Some("cmd-z"),
                    action: Box::new(editor::Undo),
                },
                MenuItem::Action {
                    name: "Redo",
                    keystroke: Some("cmd-Z"),
                    action: Box::new(editor::Redo),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Cut",
                    keystroke: Some("cmd-x"),
                    action: Box::new(editor::Cut),
                },
                MenuItem::Action {
                    name: "Copy",
                    keystroke: Some("cmd-c"),
                    action: Box::new(editor::Copy),
                },
                MenuItem::Action {
                    name: "Paste",
                    keystroke: Some("cmd-v"),
                    action: Box::new(editor::Paste),
                },
            ],
        },
    ]
}
