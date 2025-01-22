use gpui::{
    AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    Subscription, Task, View, WeakModel, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::{
    git::{GitState, RepositoryHandle},
    Project,
};
use std::sync::Arc;
use ui::{prelude::*, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

pub struct RepositorySelector {
    picker: View<Picker<RepositorySelectorDelegate>>,
    /// The task used to update the picker's matches when there is a change to
    /// the repository list.
    update_matches_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl RepositorySelector {
    pub fn new(project: Model<Project>, cx: &mut ViewContext<Self>) -> Self {
        let git_state = project.read(cx).git_state().cloned();
        let all_repositories = git_state
            .as_ref()
            .map_or(vec![], |git_state| git_state.read(cx).all_repositories());
        let filtered_repositories = all_repositories.clone();
        let delegate = RepositorySelectorDelegate {
            project: project.downgrade(),
            repository_selector: cx.view().downgrade(),
            repository_entries: all_repositories,
            filtered_repositories,
            selected_index: 0,
        };

        let picker =
            cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into())));

        let _subscriptions = if let Some(git_state) = git_state {
            vec![cx.subscribe(&git_state, Self::handle_project_git_event)]
        } else {
            Vec::new()
        };

        RepositorySelector {
            picker,
            update_matches_task: None,
            _subscriptions,
        }
    }

    fn handle_project_git_event(
        &mut self,
        git_state: Model<GitState>,
        _event: &project::git::Event,
        cx: &mut ViewContext<Self>,
    ) {
        // TODO handle events individually
        let task = self.picker.update(cx, |this, cx| {
            let query = this.query(cx);
            this.delegate.repository_entries = git_state.read(cx).all_repositories();
            this.delegate.update_matches(query, cx)
        });
        self.update_matches_task = Some(task);
    }
}

impl EventEmitter<DismissEvent> for RepositorySelector {}

impl FocusableView for RepositorySelector {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RepositorySelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(IntoElement)]
pub struct RepositorySelectorPopoverMenu<T>
where
    T: PopoverTrigger,
{
    repository_selector: View<RepositorySelector>,
    trigger: T,
    handle: Option<PopoverMenuHandle<RepositorySelector>>,
}

impl<T: PopoverTrigger> RepositorySelectorPopoverMenu<T> {
    pub fn new(repository_selector: View<RepositorySelector>, trigger: T) -> Self {
        Self {
            repository_selector,
            trigger,
            handle: None,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<RepositorySelector>) -> Self {
        self.handle = Some(handle);
        self
    }
}

impl<T: PopoverTrigger> RenderOnce for RepositorySelectorPopoverMenu<T> {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let repository_selector = self.repository_selector.clone();

        PopoverMenu::new("repository-switcher")
            .menu(move |_cx| Some(repository_selector.clone()))
            .trigger(self.trigger)
            .attach(gpui::Corner::BottomLeft)
            .when_some(self.handle.clone(), |menu, handle| menu.with_handle(handle))
    }
}

pub struct RepositorySelectorDelegate {
    project: WeakModel<Project>,
    repository_selector: WeakView<RepositorySelector>,
    repository_entries: Vec<RepositoryHandle>,
    filtered_repositories: Vec<RepositoryHandle>,
    selected_index: usize,
}

impl RepositorySelectorDelegate {
    pub fn update_repository_entries(&mut self, all_repositories: Vec<RepositoryHandle>) {
        self.repository_entries = all_repositories.clone();
        self.filtered_repositories = all_repositories;
        self.selected_index = 0;
    }
}

impl PickerDelegate for RepositorySelectorDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_repositories.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_repositories.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a repository...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let all_repositories = self.repository_entries.clone();

        cx.spawn(|this, mut cx| async move {
            let filtered_repositories = cx
                .background_executor()
                .spawn(async move {
                    if query.is_empty() {
                        all_repositories
                    } else {
                        all_repositories
                            .into_iter()
                            .filter(|_repo_info| {
                                // TODO: Implement repository filtering logic
                                true
                            })
                            .collect()
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.delegate.filtered_repositories = filtered_repositories;
                this.delegate.set_selected_index(0, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(selected_repo) = self.filtered_repositories.get(self.selected_index) else {
            return;
        };
        selected_repo.activate(cx);
        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.repository_selector
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_header(&self, _cx: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        // TODO: Implement header rendering if needed
        None
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let project = self.project.upgrade()?;
        let repo_info = self.filtered_repositories.get(ix)?;
        let display_name = repo_info.display_name(project.read(cx), cx);
        // TODO: Implement repository item rendering
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(display_name)),
        )
    }

    fn render_footer(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
        // TODO: Implement footer rendering if needed
        Some(
            div()
                .text_ui_sm(cx)
                .child("Temporary location for repo selector")
                .into_any_element(),
        )
    }
}
