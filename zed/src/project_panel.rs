use crate::{
    project::Project,
    theme::Theme,
    worktree::{self, Worktree},
    Settings,
};
use gpui::{
    elements::{Empty, Label, List, ListState, Orientation},
    AppContext, Element, ElementBox, Entity, ModelHandle, View, ViewContext,
};
use postage::watch;

pub struct ProjectPanel {
    project: ModelHandle<Project>,
    list: ListState,
    settings: watch::Receiver<Settings>,
}

pub enum Event {}

impl ProjectPanel {
    pub fn new(
        project: ModelHandle<Project>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&project, |this, project, cx| {
            let project = project.read(cx);
            this.list.reset(Self::entry_count(project, cx));
            cx.notify();
        })
        .detach();

        Self {
            list: ListState::new(
                {
                    let project = project.read(cx);
                    Self::entry_count(project, cx)
                },
                Orientation::Top,
                1000.,
                {
                    let project = project.clone();
                    let settings = settings.clone();
                    move |ix, cx| {
                        let project = project.read(cx);
                        Self::render_entry_at_index(project, ix, &settings.borrow().theme, cx)
                    }
                },
            ),
            project,
            settings,
        }
    }

    fn entry_count(project: &Project, cx: &AppContext) -> usize {
        project
            .worktrees()
            .iter()
            .map(|worktree| worktree.read(cx).visible_entry_count())
            .sum()
    }

    fn render_entry_at_index(
        project: &Project,
        mut ix: usize,
        theme: &Theme,
        cx: &AppContext,
    ) -> ElementBox {
        for worktree in project.worktrees() {
            let worktree = worktree.read(cx);
            let visible_entry_count = worktree.visible_entry_count();
            if ix < visible_entry_count {
                let entry = worktree.visible_entries(ix).next().unwrap();
                return Self::render_entry(worktree, entry, theme, cx);
            } else {
                ix -= visible_entry_count;
            }
        }
        Empty::new().boxed()
    }

    fn render_entry(
        worktree: &Worktree,
        entry: &worktree::Entry,
        theme: &Theme,
        _: &AppContext,
    ) -> ElementBox {
        let path = &entry.path;
        let depth = path.iter().count() as f32;
        Label::new(
            path.file_name()
                .map_or(String::new(), |s| s.to_string_lossy().to_string()),
            theme.project_panel.entry.clone(),
        )
        .contained()
        .with_margin_left(depth * 20.)
        .boxed()
    }
}

impl View for ProjectPanel {
    fn ui_name() -> &'static str {
        "ProjectPanel"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        let theme = &self.settings.borrow().theme.project_panel;
        List::new(self.list.clone())
            .contained()
            .with_style(theme.container)
            .boxed()
    }
}

impl Entity for ProjectPanel {
    type Event = Event;
}
