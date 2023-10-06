use crate::{
    Buffer, BufferRow, BufferRows, GitStatus, HighlightColor, HighlightedLine, HighlightedText,
    Icon, Label, LabelColor, ListEntry, ListItem, Theme, ToggleState,
};

pub fn static_project_panel_project_items<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("zed"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(0)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new(".cargo"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".config"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".git").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".cargo"))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".idea").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new("assets"))
            .left_icon(Icon::Folder.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("cargo-target").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new("crates"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("activity_indicator"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("ai"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("audio"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("auto_update"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("breadcrumbs"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("call"))
            .left_icon(Icon::Folder.into())
            .indent_level(2),
        ListEntry::new(Label::new("sqlez").color(LabelColor::Modified))
            .left_icon(Icon::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::NotToggled),
        ListEntry::new(Label::new("gpui2"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("src"))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("derive_element.rs"))
            .left_icon(Icon::FileRust.into())
            .indent_level(4),
        ListEntry::new(Label::new("storybook").color(LabelColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("docs").color(LabelColor::Default))
            .left_icon(Icon::Folder.into())
            .indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("src").color(LabelColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("ui").color(LabelColor::Modified))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(4)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("component").color(LabelColor::Created))
            .left_icon(Icon::FolderOpen.into())
            .indent_level(5)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("facepile.rs").color(LabelColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("follow_group.rs").color(LabelColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("list_item.rs").color(LabelColor::Created))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("tab.rs").color(LabelColor::Default))
            .left_icon(Icon::FileRust.into())
            .indent_level(6),
        ListEntry::new(Label::new("target").color(LabelColor::Hidden))
            .left_icon(Icon::Folder.into())
            .indent_level(1),
        ListEntry::new(Label::new(".dockerignore"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new(".DS_Store").color(LabelColor::Hidden))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new("Cargo.lock"))
            .left_icon(Icon::FileLock.into())
            .indent_level(1),
        ListEntry::new(Label::new("Cargo.toml"))
            .left_icon(Icon::FileToml.into())
            .indent_level(1),
        ListEntry::new(Label::new("Dockerfile"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new("Procfile"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(1),
        ListEntry::new(Label::new("README.md"))
            .left_icon(Icon::FileDoc.into())
            .indent_level(1),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_project_panel_single_items<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("todo.md"))
            .left_icon(Icon::FileDoc.into())
            .indent_level(0),
        ListEntry::new(Label::new("README.md"))
            .left_icon(Icon::FileDoc.into())
            .indent_level(0),
        ListEntry::new(Label::new("config.json"))
            .left_icon(Icon::FileGeneric.into())
            .indent_level(0),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn empty_buffer_example<S: 'static + Send + Sync + Clone>() -> Buffer<S> {
    Buffer::new().set_rows(Some(BufferRows::default()))
}

pub fn hello_world_rust_buffer_example<S: 'static + Send + Sync + Clone>(
    theme: &Theme,
) -> Buffer<S> {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_buffer_rows(theme),
        }))
}

pub fn hello_world_rust_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Statements here are executed when the compiled binary is called."
                        .to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 3,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 4,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Print text to the console.".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 5,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "    println!(".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".to_string(),
                        color: HighlightColor::String.hsla(&theme),
                    },
                    HighlightedText {
                        text: ");".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 6,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}

pub fn hello_world_rust_buffer_with_status_example<S: 'static + Send + Sync + Clone>(
    theme: &Theme,
) -> Buffer<S> {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_with_status_buffer_rows(theme),
        }))
}

pub fn hello_world_rust_with_status_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "// Statements here are executed when the compiled binary is called."
                        .to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Modified,
            show_line_number,
        },
        BufferRow {
            line_number: 3,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 4,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Print text to the console.".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 5,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "    println!(".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".to_string(),
                        color: HighlightColor::String.hsla(&theme),
                    },
                    HighlightedText {
                        text: ");".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 6,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 7,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
        BufferRow {
            line_number: 8,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "// Marshall and Nate were here".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
    ]
}

pub fn terminal_buffer<S: 'static + Send + Sync + Clone>(theme: &Theme) -> Buffer<S> {
    Buffer::new()
        .set_title("zed — fish".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: false,
            rows: terminal_buffer_rows(theme),
        }))
}

pub fn terminal_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = false;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "maxdeviant ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "in ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "profaned-capital ".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "in ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "~/p/zed ".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "on ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: " gpui2-ui ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "λ ".to_string(),
                    color: HighlightColor::String.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}
