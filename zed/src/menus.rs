use crate::{settings::Settings, watch::Receiver};
use gpui::{Menu, MenuItem};

#[cfg(target_os = "macos")]
pub fn menus(settings: Receiver<Settings>) -> Vec<Menu<'static>> {
    vec![
        Menu {
            name: "Zed",
            items: vec![
                MenuItem::Action {
                    name: "About Zed…",
                    keystroke: None,
                    action: "app:about-zed",
                    arg: None,
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Quit",
                    keystroke: Some("cmd-q"),
                    action: "app:quit",
                    arg: None,
                },
            ],
        },
        Menu {
            name: "File",
            items: vec![MenuItem::Action {
                name: "Open…",
                keystroke: Some("cmd-o"),
                action: "workspace:open",
                arg: Some(Box::new(settings)),
            }],
        },
        Menu {
            name: "Edit",
            items: vec![
                MenuItem::Action {
                    name: "Undo",
                    keystroke: Some("cmd-z"),
                    action: "editor:undo",
                    arg: None,
                },
                MenuItem::Action {
                    name: "Redo",
                    keystroke: Some("cmd-Z"),
                    action: "editor:redo",
                    arg: None,
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Cut",
                    keystroke: Some("cmd-x"),
                    action: "editor:cut",
                    arg: None,
                },
                MenuItem::Action {
                    name: "Copy",
                    keystroke: Some("cmd-c"),
                    action: "editor:copy",
                    arg: None,
                },
                MenuItem::Action {
                    name: "Paste",
                    keystroke: Some("cmd-v"),
                    action: "editor:paste",
                    arg: None,
                },
            ],
        },
    ]
}
