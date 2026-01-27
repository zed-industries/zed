mod active_buffer_encoding;
pub use active_buffer_encoding::ActiveBufferEncoding;

use editor::Editor;
use encoding_rs::Encoding;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, ParentElement, Render, Styled, Task, WeakEntity, Window, actions,
};
use language::Buffer;
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, Toggleable, v_flex};
use util::ResultExt;
use workspace::{ModalView, Toast, Workspace, notifications::NotificationId};

actions!(
    encoding_selector,
    [
        /// Toggles the encoding selector modal.
        Toggle
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(EncodingSelector::register).detach();
}

pub struct EncodingSelector {
    picker: Entity<Picker<EncodingSelectorDelegate>>,
}

impl EncodingSelector {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    pub fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;

        let buffer_handle = buffer.read(cx);
        let project = workspace.project().read(cx);

        if buffer_handle.is_dirty() {
            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<EncodingSelector>(),
                    "Save file to change encoding",
                ),
                cx,
            );
            return Some(());
        }
        if project.is_shared() {
            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<EncodingSelector>(),
                    "Cannot change encoding during collaboration",
                ),
                cx,
            );
            return Some(());
        }
        if project.is_via_remote_server() {
            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<EncodingSelector>(),
                    "Cannot change encoding of remote server file",
                ),
                cx,
            );
            return Some(());
        }

        workspace.toggle_modal(window, cx, move |window, cx| {
            EncodingSelector::new(buffer, window, cx)
        });
        Some(())
    }

    fn new(buffer: Entity<Buffer>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = EncodingSelectorDelegate::new(cx.entity().downgrade(), buffer);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for EncodingSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .key_context("EncodingSelector")
            .w(gpui::rems(34.))
            .child(self.picker.clone())
    }
}

impl Focusable for EncodingSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for EncodingSelector {}
impl ModalView for EncodingSelector {}

pub struct EncodingSelectorDelegate {
    encoding_selector: WeakEntity<EncodingSelector>,
    buffer: Entity<Buffer>,
    encodings: Vec<&'static Encoding>,
    match_candidates: Arc<Vec<StringMatchCandidate>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl EncodingSelectorDelegate {
    fn new(encoding_selector: WeakEntity<EncodingSelector>, buffer: Entity<Buffer>) -> Self {
        let encodings = available_encodings();
        let match_candidates = encodings
            .iter()
            .enumerate()
            .map(|(id, enc)| StringMatchCandidate::new(id, enc.name()))
            .collect::<Vec<_>>();
        Self {
            encoding_selector,
            buffer,
            encodings,
            match_candidates: Arc::new(match_candidates),
            matches: vec![],
            selected_index: 0,
        }
    }

    fn render_data_for_match(&self, mat: &StringMatch, cx: &App) -> String {
        let candidate_encoding = self.encodings[mat.candidate_id];
        let current_encoding = self.buffer.read(cx).encoding();

        if candidate_encoding.name() == current_encoding.name() {
            format!("{} (current)", candidate_encoding.name())
        } else {
            candidate_encoding.name().to_string()
        }
    }
}

fn available_encodings() -> Vec<&'static Encoding> {
    let mut encodings = vec![
        // Unicode
        encoding_rs::UTF_8,
        encoding_rs::UTF_16LE,
        encoding_rs::UTF_16BE,
        // Japanese
        encoding_rs::SHIFT_JIS,
        encoding_rs::EUC_JP,
        encoding_rs::ISO_2022_JP,
        // Chinese
        encoding_rs::GBK,
        encoding_rs::GB18030,
        encoding_rs::BIG5,
        // Korean
        encoding_rs::EUC_KR,
        // Windows / Single Byte Series
        encoding_rs::WINDOWS_1252, // Western (ISO-8859-1 unified)
        encoding_rs::WINDOWS_1250, // Central European
        encoding_rs::WINDOWS_1251, // Cyrillic
        encoding_rs::WINDOWS_1253, // Greek
        encoding_rs::WINDOWS_1254, // Turkish (ISO-8859-9 unified)
        encoding_rs::WINDOWS_1255, // Hebrew
        encoding_rs::WINDOWS_1256, // Arabic
        encoding_rs::WINDOWS_1257, // Baltic
        encoding_rs::WINDOWS_1258, // Vietnamese
        encoding_rs::WINDOWS_874,  // Thai
        // ISO-8859 Series (others)
        encoding_rs::ISO_8859_2,
        encoding_rs::ISO_8859_3,
        encoding_rs::ISO_8859_4,
        encoding_rs::ISO_8859_5,
        encoding_rs::ISO_8859_6,
        encoding_rs::ISO_8859_7,
        encoding_rs::ISO_8859_8,
        encoding_rs::ISO_8859_8_I, // Logical Hebrew
        encoding_rs::ISO_8859_10,
        encoding_rs::ISO_8859_13,
        encoding_rs::ISO_8859_14,
        encoding_rs::ISO_8859_15,
        encoding_rs::ISO_8859_16,
        // Cyrillic / Legacy Misc
        encoding_rs::KOI8_R,
        encoding_rs::KOI8_U,
        encoding_rs::IBM866,
        encoding_rs::MACINTOSH,
        encoding_rs::X_MAC_CYRILLIC,
        // NOTE: The following encodings are intentionally excluded from the list:
        //
        // 1. encoding_rs::REPLACEMENT
        //    Used internally for decoding errors. Not suitable for user selection.
        //
        // 2. encoding_rs::X_USER_DEFINED
        //    Used for binary data emulation (legacy web behavior). Not for general text editing.
    ];

    encodings.sort_by_key(|enc| enc.name());

    encodings
}

impl PickerDelegate for EncodingSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Reopen with encoding...".into()
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
        _: &mut Context<Picker<Self>>,
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
        let candidates = self.match_candidates.clone();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string.clone(),
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let selected_encoding = self.encodings[mat.candidate_id];

            self.buffer.update(cx, |buffer, cx| {
                let _ = buffer.reload_with_encoding(selected_encoding, cx);
            });
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.encoding_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches.get(ix)?;

        let label = self.render_data_for_match(mat, cx);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}
