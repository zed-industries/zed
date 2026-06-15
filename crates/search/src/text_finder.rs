use std::ops::Range;

use gpui::{
    App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Modifiers, Task, WeakEntity, actions,
};
use language::Buffer;
use picker::Picker;

use project::ProjectPath;
use text::Anchor;
use ui::Window;
use workspace::{DismissDecision, ModalView, Workspace};

mod delegate;
mod render;
use delegate::Delegate;

use crate::{ProjectSearchView, project_search::ProjectSearchBar};

actions!(
    // TODO! reuse most of the ones from project search
    text_finder,
    [ToProjectSearch,]
);

pub struct TextFinder {
    picker: Entity<Picker<Delegate>>,
    init_modifiers: Option<Modifiers>,
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
                Self::open(window, cx).detach();
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

    pub fn open_from_project_search(
        project_search_view: Entity<ProjectSearchView>,
        window: &mut Window,
        cx: &mut Context<ProjectSearchBar>,
    ) -> Task<()> {
        let workspace = WeakEntity::clone(&project_search_view.read(cx).workspace);
        cx.spawn_in(window, async move |_, cx| {
            let delegate = Delegate::new_from_project_search(project_search_view, cx).await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace
                        .toggle_modal(window, cx, |window, cx| Self::new(delegate, window, cx));
                })
                .ok();
        })
    }

    pub fn open(window: &mut Window, cx: &mut Context<Workspace>) -> Task<()> {
        cx.spawn_in(window, async move |workspace, cx| {
            let Ok(delegate_task) = workspace.update_in(cx, |workspace, window, cx| {
                Delegate::new(workspace, window, cx)
            }) else {
                return;
            };

            let delegate = delegate_task.await;
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace
                        .toggle_modal(window, cx, |window, cx| Self::new(delegate, window, cx));
                })
                .ok();
        })
    }

    fn new(delegate: Delegate, window: &mut Window, cx: &mut App) -> Self {
        let project = delegate.project(cx).clone();
        let picker = cx.new(|cx| Picker::uniform_list_with_preview(delegate, project, window, cx));
        let picker_weak = picker.downgrade();
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, cx| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
            picker.delegate.hook_up_any_ongoing_search(picker_weak, cx);
        });

        Self {
            picker,
            init_modifiers: window.modifiers().modified().then_some(window.modifiers()),
        }
    }
}

impl ModalView for TextFinder {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> DismissDecision {
        // self.save_layout(cx); // TODO! move to Picker
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
