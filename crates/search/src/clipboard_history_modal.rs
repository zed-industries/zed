use clipboard_history::{ClipboardEntry, ClipboardHistory};
use gpui::{
    App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    KeyBinding, KeyContext, Render, Subscription, UniformListScrollHandle, WeakEntity, Window,
    actions,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{Label, ListItem, prelude::*};
use workspace::{ModalView, Workspace};

actions!(clipboard_history_modal, [ToggleClipboardHistory]);

pub fn init(cx: &mut App) {
    clipboard_history::init(cx);
    cx.bind_keys([KeyBinding::new(
        "cmd-shift-v",
        ToggleClipboardHistory,
        Some("Workspace"),
    )]);
    cx.observe_new(ClipboardHistoryModal::register).detach();
}

pub struct ClipboardHistoryDelegate {
    entries: Vec<ClipboardEntry>,
    selected_index: usize,
    matches: Vec<usize>,
    clipboard_history_modal: WeakEntity<ClipboardHistoryModal>,
}

pub struct ClipboardHistoryModal {
    picker: Entity<Picker<ClipboardHistoryDelegate>>,
    _picker_subscription: Subscription,
}

impl ModalView for ClipboardHistoryModal {}

impl EventEmitter<DismissEvent> for ClipboardHistoryModal {}

impl Focusable for ClipboardHistoryModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ClipboardHistoryModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("ClipboardHistoryModal");

        let viewport_size = window.viewport_size();
        let modal_width = (viewport_size.width * 0.7).min(viewport_size.width);
        let modal_height = (viewport_size.height * 0.7).min(viewport_size.height);

        v_flex()
            .key_context(key_context)
            .elevation_3(cx)
            .w(modal_width)
            .h(modal_height)
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
    }
}

impl ClipboardHistoryModal {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &ToggleClipboardHistory, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        workspace.toggle_modal(window, cx, |window, cx| Self::new(window, cx));
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();
        let scroll_handle = UniformListScrollHandle::new();

        let entries = ClipboardHistory::entries();

        let delegate = ClipboardHistoryDelegate {
            entries: entries.clone(),
            selected_index: 0,
            matches: (0..entries.len()).collect(),
            clipboard_history_modal: weak_self,
        };

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .modal(false)
                .max_height(None)
                .track_scroll(scroll_handle.clone())
                .show_scrollbar(true)
        });

        let picker_subscription = cx.subscribe_in(&picker, window, Self::on_picker_event);

        Self {
            picker,
            _picker_subscription: picker_subscription,
        }
    }

    fn on_picker_event(
        &mut self,
        _picker: &Entity<Picker<ClipboardHistoryDelegate>>,
        _event: &DismissEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DismissEvent);
    }
}

impl PickerDelegate for ClipboardHistoryDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Choose from clipboard history to paste...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.to_lowercase();

        if query.is_empty() {
            self.matches = (0..self.entries.len()).collect();
        } else {
            self.matches = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.text.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
        }

        self.selected_index = 0;
        gpui::Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(&entry_index) = self.matches.get(self.selected_index) {
            if let Some(entry) = self.entries.get(entry_index) {
                let text = entry.text.clone();

                // Move the selected entry to the top of the history
                ClipboardHistory::add_entry(text.clone(), cx);

                // Write to clipboard first
                cx.write_to_clipboard(ClipboardItem::new_string(text));

                // Dismiss the modal
                if let Some(modal) = self.clipboard_history_modal.upgrade() {
                    modal.update(cx, |_, cx| {
                        cx.emit(DismissEvent);
                    });
                }

                // Defer the paste action to ensure the modal is fully dismissed and focus is back on the editor
                cx.defer(move |cx| {
                    cx.dispatch_action(&editor::actions::Paste);
                });
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_index = *self.matches.get(ix)?;
        let entry = self.entries.get(entry_index)?;

        let preview = entry.preview();

        Some(
            ListItem::new(ix).inset(true).toggle_state(selected).child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .child(Label::new(preview)),
            ),
        )
    }
}
