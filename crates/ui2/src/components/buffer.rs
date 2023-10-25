use std::marker::PhantomData;

use gpui2::{Hsla, WindowContext};

use crate::prelude::*;
use crate::{h_stack, v_stack, Icon, IconElement};

#[derive(Default, PartialEq, Copy, Clone)]
pub struct PlayerCursor {
    color: Hsla,
    index: usize,
}

#[derive(Default, PartialEq, Clone)]
pub struct HighlightedText {
    pub text: String,
    pub color: Hsla,
}

#[derive(Default, PartialEq, Clone)]
pub struct HighlightedLine {
    pub highlighted_texts: Vec<HighlightedText>,
}

#[derive(Default, PartialEq, Clone)]
pub struct BufferRow {
    pub line_number: usize,
    pub code_action: bool,
    pub current: bool,
    pub line: Option<HighlightedLine>,
    pub cursors: Option<Vec<PlayerCursor>>,
    pub status: GitStatus,
    pub show_line_number: bool,
}

#[derive(Clone)]
pub struct BufferRows {
    pub show_line_numbers: bool,
    pub rows: Vec<BufferRow>,
}

impl Default for BufferRows {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            rows: vec![BufferRow {
                line_number: 1,
                code_action: false,
                current: true,
                line: None,
                cursors: None,
                status: GitStatus::None,
                show_line_number: true,
            }],
        }
    }
}

impl BufferRow {
    pub fn new(line_number: usize) -> Self {
        Self {
            line_number,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number: true,
        }
    }

    pub fn set_line(mut self, line: Option<HighlightedLine>) -> Self {
        self.line = line;
        self
    }

    pub fn set_cursors(mut self, cursors: Option<Vec<PlayerCursor>>) -> Self {
        self.cursors = cursors;
        self
    }

    pub fn add_cursor(mut self, cursor: PlayerCursor) -> Self {
        if let Some(cursors) = &mut self.cursors {
            cursors.push(cursor);
        } else {
            self.cursors = Some(vec![cursor]);
        }
        self
    }

    pub fn set_status(mut self, status: GitStatus) -> Self {
        self.status = status;
        self
    }

    pub fn set_show_line_number(mut self, show_line_number: bool) -> Self {
        self.show_line_number = show_line_number;
        self
    }

    pub fn set_code_action(mut self, code_action: bool) -> Self {
        self.code_action = code_action;
        self
    }

    pub fn set_current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }
}

#[derive(Element, Clone)]
pub struct Buffer<S: 'static + Send + Sync + Clone> {
    id: ElementId,
    state_type: PhantomData<S>,
    rows: Option<BufferRows>,
    readonly: bool,
    language: Option<String>,
    title: Option<String>,
    path: Option<String>,
}

impl<S: 'static + Send + Sync + Clone> Buffer<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
            rows: Some(BufferRows::default()),
            readonly: false,
            language: None,
            title: Some("untitled".to_string()),
            path: None,
        }
    }

    pub fn set_title<T: Into<Option<String>>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn set_path<P: Into<Option<String>>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
    }

    pub fn set_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn set_rows<R: Into<Option<BufferRows>>>(mut self, rows: R) -> Self {
        self.rows = rows.into();
        self
    }

    pub fn set_language<L: Into<Option<String>>>(mut self, language: L) -> Self {
        self.language = language.into();
        self
    }

    fn render_row(row: BufferRow, cx: &WindowContext) -> impl Element<ViewState = S> {
        let theme = theme(cx);

        let line_background = if row.current {
            theme.editor_active_line
        } else {
            theme.transparent
        };

        let line_number_color = if row.current {
            theme.text
        } else {
            theme.syntax.comment
        };

        h_stack()
            .bg(line_background)
            .w_full()
            .gap_2()
            .px_1()
            .child(
                h_stack()
                    .w_4()
                    .h_full()
                    .px_0p5()
                    .when(row.code_action, |c| {
                        div().child(IconElement::new(Icon::Bolt))
                    }),
            )
            .when(row.show_line_number, |this| {
                this.child(
                    h_stack().justify_end().px_0p5().w_3().child(
                        div()
                            .text_color(line_number_color)
                            .child(row.line_number.to_string()),
                    ),
                )
            })
            .child(div().mx_0p5().w_1().h_full().bg(row.status.hsla(cx)))
            .children(row.line.map(|line| {
                div()
                    .flex()
                    .children(line.highlighted_texts.iter().map(|highlighted_text| {
                        div()
                            .text_color(highlighted_text.color)
                            .child(highlighted_text.text.clone())
                    }))
            }))
    }

    fn render_rows(&self, cx: &WindowContext) -> Vec<impl Element<ViewState = S>> {
        match &self.rows {
            Some(rows) => rows
                .rows
                .iter()
                .map(|row| Self::render_row(row.clone(), cx))
                .collect(),
            None => vec![],
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);
        let rows = self.render_rows(cx);

        v_stack()
            .flex_1()
            .w_full()
            .h_full()
            .bg(theme.editor)
            .children(rows)
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui2::rems;

    use crate::{
        empty_buffer_example, hello_world_rust_buffer_example,
        hello_world_rust_buffer_with_status_example, Story,
    };

    use super::*;

    #[derive(Element)]
    pub struct BufferStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> BufferStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            let theme = theme(cx);

            Story::container(cx)
                .child(Story::title_for::<_, Buffer<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(div().w(rems(64.)).h_96().child(empty_buffer_example()))
                .child(Story::label(cx, "Hello World (Rust)"))
                .child(
                    div()
                        .w(rems(64.))
                        .h_96()
                        .child(hello_world_rust_buffer_example(&theme)),
                )
                .child(Story::label(cx, "Hello World (Rust) with Status"))
                .child(
                    div()
                        .w(rems(64.))
                        .h_96()
                        .child(hello_world_rust_buffer_with_status_example(&theme)),
                )
        }
    }
}
