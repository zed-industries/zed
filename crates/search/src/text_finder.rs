use std::{ops::Range, sync::atomic::Ordering};

use editor::Editor;
use gpui::{
    App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Modifiers, Subscription, Task, WeakEntity, actions,
};
use language::Buffer;
use picker::Picker;

use project::ProjectPath;
use text::Anchor;
use ui::Window;
use workspace::{DismissDecision, ModalView, Workspace, searchable::SearchableItemHandle};

mod delegate;
mod render;
use delegate::{Delegate, matches_to_multibuffer};
use util::ResultExt as _;

use crate::{ProjectSearchView, text_finder::delegate::PopulateProjectSearch};

actions!(text_finder, [ToProjectSearch,]);

pub struct TextFinder {
    picker: Entity<Picker<Delegate>>,
    init_modifiers: Option<Modifiers>,
    _subscription: Subscription,
}

pub fn init(cx: &mut App) {
    cx.observe_new(TextFinder::register).detach();
}

impl TextFinder {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        pub use zed_actions::text_finder::Toggle;
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let Some(text_picker) = workspace.active_modal::<Self>(cx) else {
                let seed_query = Self::seed_query(workspace, window, cx);
                Self::open(seed_query, window, cx).detach();
                return;
            };

            text_picker.update(cx, |text_picker, cx| {
                text_picker.init_modifiers = Some(window.modifiers());
                text_picker.picker.update(cx, |picker, cx| {
                    picker.cycle_selection(window, cx);
                });
            })
        });
    }

    pub fn open_from_project_search<T: 'static>(
        project_search_view: Entity<ProjectSearchView>,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Task<()> {
        let project_search_item_id = project_search_view.entity_id();
        cx.spawn_in(window, async move |_, cx| {
            let workspace =
                project_search_view.read_with(cx, |view, _| WeakEntity::clone(&view.workspace));
            let delegate = Delegate::new_from_project_search(project_search_view, cx).await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    remove_project_search_tab(project_search_item_id, workspace, window, cx);
                    workspace.toggle_modal(window, cx, |window, cx| {
                        Self::new(delegate, None, window, cx)
                    });
                })
                .ok();
        })
    }

    /// Transition this text finder into a project search tab, carrying over the
    /// current results (and any in-progress search stream) instead of re-running
    /// the search. Inverse of [`Self::open_from_project_search`].
    fn to_project_search(
        &mut self,
        _: &ToProjectSearch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let picker = Entity::clone(&self.picker);
        let workspace = self.weak_workspace(cx);

        let connected_task = self.take_search_task(cx);
        let project_search_view = self.project_search_view(cx);
        let query = picker.read(cx).delegate.active_query.clone();
        let search_options = picker.read(cx).delegate.search_options;

        cx.spawn_in(window, async move |this, cx| {
            let search_stream = connected_task.unwrap_or(gpui::Task::ready(None)).await;
            let matches =
                picker.update(cx, |picker, _| std::mem::take(&mut picker.delegate.matches));

            project_search_view
                .update_in(cx, |view, window, cx| {
                    view.adopt_text_finder_state(search_options, query, window, cx);
                })
                .log_err();

            this.update(cx, |_, cx| cx.emit(DismissEvent)).log_err();
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.add_item_to_active_pane(
                        Box::new(project_search_view.clone()),
                        None,
                        true, // focus item
                        window,
                        cx,
                    );
                })
                .log_err();

            if let PopulateProjectSearch::SupersededByNewSearch =
                matches_to_multibuffer(&project_search_view, &matches, cx).await
            {
                return;
            }

            if let Some(stream) = search_stream {
                project_search_view.update(cx, |view, cx| {
                    view.entity
                        .update(cx, |search, cx| search.hook_up_ongoing_search(stream, cx));
                });
            }
        })
        .detach();
    }

    fn split_left(
        &mut self,
        _: &workspace::pane::SplitLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Left, window, cx);
    }

    fn split_right(
        &mut self,
        _: &workspace::pane::SplitRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Right, window, cx);
    }

    fn split_up(
        &mut self,
        _: &workspace::pane::SplitUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Up, window, cx);
    }

    fn split_down(
        &mut self,
        _: &workspace::pane::SplitDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_in_split(workspace::SplitDirection::Down, window, cx);
    }

    fn open_in_split(
        &mut self,
        direction: workspace::SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.open_in_split(direction, window, cx);
        });
    }

    fn weak_workspace(&self, cx: &App) -> WeakEntity<Workspace> {
        let workspace = WeakEntity::clone(
            &self
                .picker
                .read(cx)
                .delegate
                .project_search_view
                .read(cx)
                .workspace,
        );
        workspace
    }

    fn take_search_task(
        &self,
        cx: &mut App,
    ) -> Option<Task<Option<project::SearchResults<project::search::SearchResult>>>> {
        self.picker
            .read(cx)
            .delegate
            .text_finder_turning_into_project_search
            .store(true, Ordering::Relaxed);
        self.picker
            .update(cx, |p, _| p.delegate.in_progress_search.take_connected())
    }

    /// The word under the cursor (or current selection) of the active editor,
    /// used to pre-populate the text finder query. Honors the
    /// `seed_search_query_from_cursor` setting, matching project search.
    fn seed_query(
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<String> {
        let editor = workspace.active_item(cx)?.act_as::<Editor>(cx)?;
        let query = editor.query_suggestion(None, window, cx);
        (!query.is_empty()).then_some(query)
    }

    pub fn open(
        seed_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |workspace, cx| {
            let Ok(delegate_task) = workspace.update_in(cx, |workspace, window, cx| {
                Delegate::new(workspace, window, cx)
            }) else {
                return;
            };

            let delegate = delegate_task.await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.toggle_modal(window, cx, |window, cx| {
                        Self::new(delegate, seed_query, window, cx)
                    });
                })
                .ok();
        })
    }

    fn new(
        delegate: Delegate,
        seed_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = delegate.project(cx).clone();
        let picker = cx.new(|cx| Picker::list_with_preview(delegate, project, window, cx));
        let picker_weak = picker.downgrade();
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, cx| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
            picker.delegate.hook_up_any_ongoing_search(picker_weak, cx);
            if let Some(seed_query) = seed_query.as_deref() {
                picker.set_query(seed_query, window, cx);
                picker.select_query(window, cx);
            }
        });
        let subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            init_modifiers: window.modifiers().modified().then_some(window.modifiers()),
            _subscription: subscription,
        }
    }

    fn project_search_view(&self, cx: &mut App) -> Entity<ProjectSearchView> {
        Entity::clone(&self.picker.read(cx).delegate.project_search_view)
    }
}

fn remove_project_search_tab(
    project_search_item_id: gpui::EntityId,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<'_, Workspace>,
) {
    if let Some(pane) = workspace.pane_for_item_id(project_search_item_id) {
        pane.update(cx, |pane, cx| {
            pane.remove_item(project_search_item_id, false, false, window, cx);
        });
    }
}

impl ModalView for TextFinder {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> DismissDecision {
        DismissDecision::Dismiss(true)
    }
}

impl EventEmitter<DismissEvent> for TextFinder {}

impl Focusable for TextFinder {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

#[derive(Clone)]
pub struct SearchMatch {
    pub path: ProjectPath,
    pub buffer: Entity<Buffer>,
    pub anchor_range: Range<Anchor>,
    pub range: Range<usize>,
    pub relative_range: Range<usize>,
    pub line_text: String,
    pub line_number: u32,
}
