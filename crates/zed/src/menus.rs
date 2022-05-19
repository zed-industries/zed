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
                    action: Box::new(super::About),
                },
                MenuItem::Action {
                    name: "Check for Updates",
                    action: Box::new(auto_update::Check),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Install CLI",
                    action: Box::new(super::InstallCommandLineInterface),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Quit",
                    action: Box::new(super::Quit),
                },
            ],
        },
        Menu {
            name: "File",
            items: vec![
                MenuItem::Action {
                    name: "New",
                    action: Box::new(workspace::OpenNew(state.clone())),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Open…",
                    action: Box::new(workspace::Open(state.clone())),
                },
            ],
        },
        Menu {
            name: "Edit",
            items: vec![
                MenuItem::Action {
                    name: "Undo",
                    action: Box::new(editor::Undo),
                },
                MenuItem::Action {
                    name: "Redo",
                    action: Box::new(editor::Redo),
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Cut",
                    action: Box::new(editor::Cut),
                },
                MenuItem::Action {
                    name: "Copy",
                    action: Box::new(editor::Copy),
                },
                MenuItem::Action {
                    name: "Paste",
                    action: Box::new(editor::Paste),
                },
            ],
        },
    ]
}
