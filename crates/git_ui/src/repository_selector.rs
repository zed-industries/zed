use std::sync::Arc;

use gpui::{
    AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Subscription,
    Task, View, WeakView,
};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

pub struct RepositorySelector {
    picker: View<Picker<RepositorySelectorDelegate>>,
    /// The task used to update the picker's matches when there is a change to
    /// the repository list.
    update_matches_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl RepositorySelector {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let all_repositories = Self::all_repositories(cx);
        let delegate = RepositorySelectorDelegate {
            repository_selector: cx.view().downgrade(),
            all_repositories: all_repositories.clone(),
            filtered_repositories: all_repositories,
            selected_index: 0,
        };

        let picker =
            cx.new_view(|cx| Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into())));

        RepositorySelector {
            picker,
            update_matches_task: None,
            _subscriptions: vec![
                // TODO: Subscribe to repository list changes
            ],
        }
    }

    fn handle_project_event(&mut self, _event: &project::Event, cx: &mut ViewContext<Self>) {
        // TODO: handle when project/repo changes
        let task = self.picker.update(cx, |this, cx| {
            let query = this.query(cx);
            this.delegate.all_repositories = Self::all_repositories(cx);
            this.delegate.update_matches(query, cx)
        });
        self.update_matches_task = Some(task);
    }

    fn all_repositories(cx: &AppContext) -> Vec<RepositoryInfo> {
        // TODO: Implement fetching all repositories
        Vec::new()
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

#[derive(Clone)]
struct RepositoryInfo {
    // TODO: Define repository information fields
}

pub struct RepositorySelectorDelegate {
    repository_selector: WeakView<RepositorySelector>,
    all_repositories: Vec<RepositoryInfo>,
    filtered_repositories: Vec<RepositoryInfo>,
    selected_index: usize,
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
        let all_repositories = self.all_repositories.clone();

        cx.spawn(|this, mut cx| async move {
            let filtered_repositories = cx
                .background_executor()
                .spawn(async move {
                    if query.is_empty() {
                        all_repositories
                    } else {
                        all_repositories
                            .into_iter()
                            .filter(|repo_info| {
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
        if let Some(repo_info) = self.filtered_repositories.get(self.selected_index) {
            // TODO: Implement repository selection logic
            cx.emit(DismissEvent);
        }
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
        let repo_info = self.filtered_repositories.get(ix)?;

        // TODO: Implement repository item rendering
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new("Repository Name")), // Placeholder
        )
    }

    fn render_footer(&self, _cx: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
        // TODO: Implement footer rendering if needed
        None
    }
}
