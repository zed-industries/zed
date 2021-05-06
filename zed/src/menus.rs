use crate::settings::Settings;
use gpui::{Menu, MenuItem};
use postage::watch;

#[cfg(target_os = "macos")]
pub fn menus(settings: watch::Receiver<Settings>) -> Vec<Menu<'static>> {
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
                    action: "buffer:undo",
                    arg: None,
                },
                MenuItem::Action {
                    name: "Redo",
                    keystroke: Some("cmd-Z"),
                    action: "buffer:redo",
                    arg: None,
                },
                MenuItem::Separator,
                MenuItem::Action {
                    name: "Cut",
                    keystroke: Some("cmd-x"),
                    action: "buffer:cut",
                    arg: None,
                },
                MenuItem::Action {
                    name: "Copy",
                    keystroke: Some("cmd-c"),
                    action: "buffer:copy",
                    arg: None,
                },
                MenuItem::Action {
                    name: "Paste",
                    keystroke: Some("cmd-v"),
                    action: "buffer:paste",
                    arg: None,
                },
            ],
        },
    ]
}
