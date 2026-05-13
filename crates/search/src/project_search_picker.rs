use std::ops::Range;

use gpui::{
    App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Modifiers, Task, actions,
};
use language::Buffer;
use picker::Picker;

use project::ProjectPath;
use project::search::SearchInputKind;
use text::Anchor;
use ui::Window;
use workspace::{DismissDecision, ModalView, Workspace};

/// Approach:
///     - implement picker for this
///     - turn picker into the quick search
mod delegate;
mod render;
use delegate::TextPickerDelegate;

actions!(
    // TODO! reuse most of the ones from project search
    text_picker,
    [
        ReplaceNext,
        ReplaceAll,
        ToggleFilters,
        ToggleLayout,
        ToggleSplitMenu,
        ToggleHistory
    ]
);

pub struct TextPicker {
    picker: Entity<Picker<TextPickerDelegate>>,
    picker_focus_handle: FocusHandle,
    init_modifiers: Option<Modifiers>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(TextPicker::register).detach();
}

impl TextPicker {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        pub use zed_actions::text_finder::Toggle;
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            let initial_query = None;

            let Some(text_picker) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, initial_query, window, cx).detach();
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

    fn open(
        workspace: &mut Workspace,
        initial_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<()> {
        let project = workspace.project().read(cx);

        let initial_query = initial_query.or_else(|| {
            project
                .search_history(SearchInputKind::Query)
                .iter()
                .next()
                .map(|s| s.to_string())
        });

        cx.spawn_in(window, async move |workspace, cx| {
            workspace
                .update_in(cx, |workspace, window, cx| {
                    let project = workspace.project().clone();
                    let weak_workspace = cx.entity().downgrade();
                    workspace.toggle_modal(window, cx, |window, cx| {
                        let delegate =
                            TextPickerDelegate::new(weak_workspace, project, initial_query, cx);

                        Self::new(delegate, window, cx)
                    });
                })
                .ok();
        })
    }

    fn new(delegate: TextPickerDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list_with_preview(delegate, window, cx));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });

        Self {
            picker,
            picker_focus_handle,
            init_modifiers: window.modifiers().modified().then_some(window.modifiers()),
        }
    }
}

impl ModalView for TextPicker {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> DismissDecision {
        // self.save_layout(cx); // TODO! move to Picker
        DismissDecision::Dismiss(true)
    }

    fn render_bare(&self) -> bool {
        true
    }
}

impl EventEmitter<DismissEvent> for TextPicker {}

impl Focusable for TextPicker {
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
