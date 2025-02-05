use gpui::{
    AnyElement, App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    Task, WeakEntity,
};
use picker::{Picker, PickerDelegate};
use project::{
    git::{GitState, Repository},
    Project,
};
use std::sync::Arc;
use ui::{prelude::*, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

pub struct RepositorySelector {
    picker: Entity<Picker<RepositorySelectorDelegate>>,
    /// The task used to update the picker's matches when there is a change to
    /// the repository list.
    update_matches_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl RepositorySelector {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let git_state = project.read(cx).git_state().clone();
        let all_repositories = git_state.read(cx).all_repositories();
        let filtered_repositories = all_repositories.clone();
        let delegate = RepositorySelectorDelegate {
            project: project.downgrade(),
            repository_selector: cx.entity().downgrade(),
            repository_entries: all_repositories,
            filtered_repositories,
            selected_index: 0,
        };

        let picker = cx.new(|cx| {
            Picker::nonsearchable_uniform_list(delegate, window, cx)
                .max_height(Some(rems(20.).into()))
        });

        let _subscriptions =
            vec![cx.subscribe_in(&git_state, window, Self::handle_project_git_event)];

        RepositorySelector {
            picker,
            update_matches_task: None,
            _subscriptions,
        }
    }

    fn handle_project_git_event(
        &mut self,
        git_state: &Entity<GitState>,
        _event: &project::git::GitEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // TODO handle events individually
        let task = self.picker.update(cx, |this, cx| {
            let query = this.query(cx);
            this.delegate.repository_entries = git_state.read(cx).all_repositories();
            this.delegate.update_matches(query, window, cx)
        });
        self.update_matches_task = Some(task);
    }
}

impl EventEmitter<DismissEvent> for RepositorySelector {}

impl Focusable for RepositorySelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RepositorySelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(IntoElement)]
pub struct RepositorySelectorPopoverMenu<T>
where
    T: PopoverTrigger,
{
    repository_selector: Entity<RepositorySelector>,
    trigger: T,
    handle: Option<PopoverMenuHandle<RepositorySelector>>,
}

impl<T: PopoverTrigger> RepositorySelectorPopoverMenu<T> {
    pub fn new(repository_selector: Entity<RepositorySelector>, trigger: T) -> Self {
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let repository_selector = self.repository_selector.clone();

        PopoverMenu::new("repository-switcher")
            .menu(move |_window, _cx| Some(repository_selector.clone()))
            .trigger(self.trigger)
            .attach(gpui::Corner::BottomLeft)
            .when_some(self.handle.clone(), |menu, handle| menu.with_handle(handle))
    }
}

pub struct RepositorySelectorDelegate {
    project: WeakEntity<Project>,
    repository_selector: WeakEntity<RepositorySelector>,
    repository_entries: Vec<Entity<Repository>>,
    filtered_repositories: Vec<Entity<Repository>>,
    selected_index: usize,
}

impl RepositorySelectorDelegate {
    pub fn update_repository_entries(&mut self, all_repositories: Vec<Entity<Repository>>) {
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix.min(self.filtered_repositories.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a repository...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_repositories = self.repository_entries.clone();

        cx.spawn_in(window, |this, mut cx| async move {
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

            this.update_in(&mut cx, |this, window, cx| {
                this.delegate.filtered_repositories = filtered_repositories;
                this.delegate.set_selected_index(0, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected_repo) = self.filtered_repositories.get(self.selected_index) else {
            return;
        };
        selected_repo.update(cx, |selected_repo, cx| selected_repo.activate(cx));
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.repository_selector
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .ok();
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        // TODO: Implement header rendering if needed
        None
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let project = self.project.upgrade()?;
        let repo_info = self.filtered_repositories.get(ix)?;
        let display_name = repo_info.read(cx).display_name(project.read(cx), cx);
        // TODO: Implement repository item rendering
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(display_name)),
        )
    }
}
