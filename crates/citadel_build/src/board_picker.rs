/// Modal picker for the user to identify a connected but unrecognized board
/// as Uno / Nano / Pro Mini / Other. UI-only: no polling, no signature reads.
/// Depends on a plain callback rather than on `BoardMonitor` directly so this
/// module stays buildable/testable in isolation.
use crate::board_detect::{VidPid, board_kvp_key};
use crate::board_registry::{BoardKind, board_kind_display_name};
use db::kvp::KeyValueStore;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity,
    Window,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

const BOARD_KINDS: [BoardKind; 4] = [
    BoardKind::Uno,
    BoardKind::Nano,
    BoardKind::ProMini,
    BoardKind::Other,
];

pub struct BoardPicker {
    picker: Entity<Picker<BoardPickerDelegate>>,
}

impl BoardPicker {
    pub fn toggle(
        vid_pid: VidPid,
        on_picked: Arc<dyn Fn(&str, &mut App) + Send + Sync>,
        workspace: &WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    BoardPicker::new(vid_pid, on_picked, window, cx)
                });
            })
            .log_err();
    }

    fn new(
        vid_pid: VidPid,
        on_picked: Arc<dyn Fn(&str, &mut App) + Send + Sync>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = BoardPickerDelegate::new(cx.entity().downgrade(), vid_pid, on_picked);
        let picker = cx.new(|cx| Picker::nonsearchable_uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for BoardPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

impl Focusable for BoardPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for BoardPicker {}
impl ModalView for BoardPicker {}

struct BoardPickerDelegate {
    board_picker: WeakEntity<BoardPicker>,
    vid_pid: VidPid,
    on_picked: Arc<dyn Fn(&str, &mut App) + Send + Sync>,
    selected_index: usize,
}

impl BoardPickerDelegate {
    fn new(
        board_picker: WeakEntity<BoardPicker>,
        vid_pid: VidPid,
        on_picked: Arc<dyn Fn(&str, &mut App) + Send + Sync>,
    ) -> Self {
        Self {
            board_picker,
            vid_pid,
            on_picked,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for BoardPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "board picker"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a board…".into()
    }

    fn match_count(&self) -> usize {
        BOARD_KINDS.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(board_kind) = BOARD_KINDS.get(self.selected_index) {
            let display_name = board_kind_display_name(*board_kind);
            let key = board_kvp_key(self.vid_pid);
            let kvp = KeyValueStore::global(cx);
            db::write_and_log(cx, move || async move {
                kvp.write_kvp(key, display_name.to_string()).await
            });
            (self.on_picked)(display_name, cx);
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.board_picker
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let board_kind = BOARD_KINDS.get(ix)?;
        let label = board_kind_display_name(*board_kind);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(label)),
        )
    }
}
