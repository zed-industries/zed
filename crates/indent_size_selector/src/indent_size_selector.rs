mod indentation;

use gpui::{
    actions, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, Styled, WeakEntity, Window,
};
pub use indentation::Indentation;
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(indent_size_selector, [Toggle]);

pub fn init(cx: &mut App) {
    cx.observe_new(IndentSizeSelector::register).detach();
}

pub struct IndentSizeSelector {
    picker: Entity<Picker<IndentSizeSelectorDelegate>>,
}

impl IndentSizeSelector {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        workspace.toggle_modal(window, cx, move |window, cx| {
            IndentSizeSelector::new(window, cx)
        });
        Some(())
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = IndentSizeSelectorDelegate::new(cx.entity().downgrade());

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for IndentSizeSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl Focusable for IndentSizeSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for IndentSizeSelector {}
impl ModalView for IndentSizeSelector {}

pub struct IndentSizeSelectorDelegate {
    indent_size_selector: WeakEntity<IndentSizeSelector>,
}

impl IndentSizeSelectorDelegate {
    fn new(indent_size_selector: WeakEntity<IndentSizeSelector>) -> Self {
        Self {
            indent_size_selector,
        }
    }
}

impl PickerDelegate for IndentSizeSelectorDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        0
    }

    fn selected_index(&self) -> usize {
        0
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        // todo!()
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "Set Indentation".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        todo!()
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.indent_size_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        todo!()
    }
}

// if let Some(workspace) = this.workspace.upgrade() {
//     workspace.update(cx, |workspace, cx| {
//         if let Some(editor) = workspace
//             .active_item(cx)
//             .and_then(|item| item.act_as::<Editor>(cx))
//         {
//             let editor = editor.downgrade();
//             let _ = editor.update(cx, |editor, cx| {
//                 if let Some(file) = editor.file_at(Point::zero(), cx) {
//                     let _ = cx.update_global(|store: &mut SettingsStore, cx| {
//                         let worktree_id = file.worktree_id(cx);
//                         let path = file.path().clone();
//                         let _ = store.set_local_settings(
//                             worktree_id,
//                             path,
//                             LocalSettingsKind::Editorconfig,
//                             Some("[/**]\nindent_size = 3\nindent_style = space\ntab_width=3"),
//                             cx
//                         ).inspect_err(|e| log::error!("set_indent failed: {e}"));
//                     });
//                 }
//             });
//         }
//     })
// }
