use crate::{
    Buffer, BufferRow, BufferRows, GitStatus, HighlightColor, HighlightedLine, HighlightedText,
    Theme,
};

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
