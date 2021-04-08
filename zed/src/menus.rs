use gpui::{Menu, MenuItem};

#[cfg(target_os = "macos")]
pub const MENUS: &'static [Menu] = &[
    Menu {
        name: "Zed",
        items: &[
            MenuItem::Action {
                name: "About Zed...",
                keystroke: None,
                action: "app:about-zed",
            },
            MenuItem::Separator,
            MenuItem::Action {
                name: "Quit",
                keystroke: Some("cmd-q"),
                action: "app:quit",
            },
        ],
    },
    Menu {
        name: "File",
        items: &[
            MenuItem::Action {
                name: "Undo",
                keystroke: Some("cmd-z"),
                action: "editor:undo",
            },
            MenuItem::Action {
                name: "Redo",
                keystroke: Some("cmd-Z"),
                action: "editor:redo",
            },
            MenuItem::Separator,
            MenuItem::Action {
                name: "Cut",
                keystroke: Some("cmd-x"),
                action: "editor:cut",
            },
            MenuItem::Action {
                name: "Copy",
                keystroke: Some("cmd-c"),
                action: "editor:copy",
            },
            MenuItem::Action {
                name: "Paste",
                keystroke: Some("cmd-v"),
                action: "editor:paste",
            },
        ],
    },
];
