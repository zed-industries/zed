use crate::{prelude::*, IconColor};
use crate::{theme, Icon, IconElement, Label, LabelColor};

#[derive(Element)]
pub struct Tab {
    title: &'static str,
    icon: Option<Icon>,
    current: bool,
    dirty: bool,
    fs_status: FileSystemStatus,
    git_status: GitStatus,
    diagnostic_status: DiagnosticStatus,
    close_side: IconSide,
}

impl Tab {
    pub fn new() -> Self {
        Self {
            title: "untitled",
            icon: None,
            current: false,
            dirty: false,
            fs_status: FileSystemStatus::None,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
            close_side: IconSide::Right,
        }
    }

    pub fn current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn icon<I>(mut self, icon: I) -> Self
    where
        I: Into<Option<Icon>>,
    {
        self.icon = icon.into();
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    pub fn fs_status(mut self, fs_status: FileSystemStatus) -> Self {
        self.fs_status = fs_status;
        self
    }

    pub fn git_status(mut self, git_status: GitStatus) -> Self {
        self.git_status = git_status;
        self
    }

    pub fn diagnostic_status(mut self, diagnostic_status: DiagnosticStatus) -> Self {
        self.diagnostic_status = diagnostic_status;
        self
    }

    pub fn close_side(mut self, close_side: IconSide) -> Self {
        self.close_side = close_side;
        self
    }

    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let label = match self.git_status {
            GitStatus::None => Label::new(self.title),
            GitStatus::Created => Label::new(self.title).color(LabelColor::Created),
            GitStatus::Modified => Label::new(self.title).color(LabelColor::Modified),
            GitStatus::Deleted => Label::new(self.title).color(LabelColor::Deleted),
            GitStatus::Renamed => Label::new(self.title).color(LabelColor::Accent),
            GitStatus::Conflict => Label::new(self.title),
        };

        let close_icon = IconElement::new(Icon::Close).color(IconColor::Muted);

        div()
            .px_2()
            .py_0p5()
            .flex()
            .items_center()
            .justify_center()
            .fill(if self.current {
                theme.highest.base.default.background
            } else {
                theme.middle.base.default.background
            })
            .child(
                div()
                    .px_1()
                    .flex()
                    .items_center()
                    .gap_1()
                    .children(self.icon.map(IconElement::new))
                    .children(if self.close_side == IconSide::Left {
                        Some(close_icon.clone())
                    } else {
                        None
                    })
                    .child(label)
                    .children(if self.close_side == IconSide::Right {
                        Some(close_icon)
                    } else {
                        None
                    }),
            )
    }
}
