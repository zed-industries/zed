use std::sync::Arc;

use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{
    App, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity, Window,
    prelude::*,
};
use jj::{Bookmark, JujutsuStore};
use picker::{Picker, PickerDelegate};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
}

fn open(
    workspace: &mut Workspace,
    _: &zed_actions::jj::BookmarkList,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(jj_store) = JujutsuStore::try_global(cx) else {
        return;
    };

    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = BookmarkPickerDelegate::new(cx.entity().downgrade(), jj_store, cx);
        BookmarkPicker::new(delegate, window, cx)
    });
}

pub struct BookmarkPicker {
    picker: Entity<Picker<BookmarkPickerDelegate>>,
}

impl BookmarkPicker {
    pub fn new(
        delegate: BookmarkPickerDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl ModalView for BookmarkPicker {}

impl EventEmitter<DismissEvent> for BookmarkPicker {}

impl Focusable for BookmarkPicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for BookmarkPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

#[derive(Debug, Clone)]
struct BookmarkEntry {
    bookmark: Bookmark,
    positions: Vec<usize>,
}

pub struct BookmarkPickerDelegate {
    picker: WeakEntity<BookmarkPicker>,
    matches: Vec<BookmarkEntry>,
    all_bookmarks: Vec<Bookmark>,
    selected_index: usize,
}

impl BookmarkPickerDelegate {
    fn new(
        picker: WeakEntity<BookmarkPicker>,
        jj_store: Entity<JujutsuStore>,
        cx: &mut Context<BookmarkPicker>,
    ) -> Self {
        let bookmarks = jj_store.read(cx).repository().list_bookmarks();

        Self {
            picker,
            matches: Vec::new(),
            all_bookmarks: bookmarks,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for BookmarkPickerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select Bookmark…".into()
    }

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

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let all_bookmarks = self.all_bookmarks.clone();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                all_bookmarks
                    .into_iter()
                    .map(|bookmark| BookmarkEntry {
                        bookmark,
                        positions: Vec::new(),
                    })
                    .collect()
            } else {
                let candidates = all_bookmarks
                    .iter()
                    .enumerate()
                    .map(|(ix, bookmark)| StringMatchCandidate::new(ix, &bookmark.ref_name))
                    .collect::<Vec<_>>();
                match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    background,
                )
                .await
                .into_iter()
                .map(|mat| BookmarkEntry {
                    bookmark: all_bookmarks[mat.candidate_id].clone(),
                    positions: mat.positions,
                })
                .collect()
            };

            this.update(cx, |this, _cx| {
                this.delegate.matches = matches;
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        //
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.picker
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = &self.matches[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    entry.bookmark.ref_name.clone(),
                    entry.positions.clone(),
                )),
        )
    }
}
