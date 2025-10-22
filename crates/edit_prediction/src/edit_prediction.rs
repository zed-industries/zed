use std::ops::Range;

use client::EditPredictionUsage;
use gpui::{App, Context, Entity, SharedString};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt};

// TODO: Find a better home for `Direction`.
//
// This should live in an ancestor crate of `editor` and `edit_prediction`,
// but at time of writing there isn't an obvious spot.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}

#[derive(Clone)]
pub enum EditPrediction {
    /// Edits within the buffer that requested the prediction
    Local {
        id: Option<SharedString>,
        edits: Vec<(Range<language::Anchor>, String)>,
        edit_preview: Option<language::EditPreview>,
    },
    /// Jump to a different file from the one that requested the prediction
    Jump {
        id: Option<SharedString>,
        snapshot: language::BufferSnapshot,
        target: language::Anchor,
    },
}

pub enum DataCollectionState {
    /// The provider doesn't support data collection.
    Unsupported,
    /// Data collection is enabled.
    Enabled { is_project_open_source: bool },
    /// Data collection is disabled or unanswered.
    Disabled { is_project_open_source: bool },
}

impl DataCollectionState {
    pub fn is_supported(&self) -> bool {
        !matches!(self, DataCollectionState::Unsupported)
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, DataCollectionState::Enabled { .. })
    }

    pub fn is_project_open_source(&self) -> bool {
        match self {
            Self::Enabled {
                is_project_open_source,
            }
            | Self::Disabled {
                is_project_open_source,
            } => *is_project_open_source,
            _ => false,
        }
    }
}

pub trait EditPredictionProvider: 'static + Sized {
    fn name() -> &'static str;
    fn display_name() -> &'static str;
    fn show_completions_in_menu() -> bool;
    fn show_tab_accept_marker() -> bool {
        false
    }
    fn supports_jump_to_edit() -> bool {
        true
    }

    fn data_collection_state(&self, _cx: &App) -> DataCollectionState {
        DataCollectionState::Unsupported
    }

    fn usage(&self, _cx: &App) -> Option<EditPredictionUsage> {
        None
    }

    fn toggle_data_collection(&mut self, _cx: &mut App) {}
    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool;
    fn is_refreshing(&self) -> bool;
    fn refresh(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    );
    fn cycle(
        &mut self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut Context<Self>,
    );
    fn accept(&mut self, cx: &mut Context<Self>);
    fn discard(&mut self, cx: &mut Context<Self>);
    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction>;
}

pub trait EditPredictionProviderHandle {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool;
    fn show_completions_in_menu(&self) -> bool;
    fn show_tab_accept_marker(&self) -> bool;
    fn supports_jump_to_edit(&self) -> bool;
    fn data_collection_state(&self, cx: &App) -> DataCollectionState;
    fn usage(&self, cx: &App) -> Option<EditPredictionUsage>;
    fn toggle_data_collection(&self, cx: &mut App);
    fn is_refreshing(&self, cx: &App) -> bool;
    fn refresh(
        &self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut App,
    );
    fn cycle(
        &self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut App,
    );
    fn accept(&self, cx: &mut App);
    fn discard(&self, cx: &mut App);
    fn suggest(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut App,
    ) -> Option<EditPrediction>;
}

impl<T> EditPredictionProviderHandle for Entity<T>
where
    T: EditPredictionProvider,
{
    fn name(&self) -> &'static str {
        T::name()
    }

    fn display_name(&self) -> &'static str {
        T::display_name()
    }

    fn show_completions_in_menu(&self) -> bool {
        T::show_completions_in_menu()
    }

    fn show_tab_accept_marker(&self) -> bool {
        T::show_tab_accept_marker()
    }

    fn supports_jump_to_edit(&self) -> bool {
        T::supports_jump_to_edit()
    }

    fn data_collection_state(&self, cx: &App) -> DataCollectionState {
        self.read(cx).data_collection_state(cx)
    }

    fn usage(&self, cx: &App) -> Option<EditPredictionUsage> {
        self.read(cx).usage(cx)
    }

    fn toggle_data_collection(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.toggle_data_collection(cx))
    }

    fn is_enabled(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &App,
    ) -> bool {
        self.read(cx).is_enabled(buffer, cursor_position, cx)
    }

    fn is_refreshing(&self, cx: &App) -> bool {
        self.read(cx).is_refreshing()
    }

    fn refresh(
        &self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        debounce: bool,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.refresh(buffer, cursor_position, debounce, cx)
        })
    }

    fn cycle(
        &self,
        buffer: Entity<Buffer>,
        cursor_position: language::Anchor,
        direction: Direction,
        cx: &mut App,
    ) {
        self.update(cx, |this, cx| {
            this.cycle(buffer, cursor_position, direction, cx)
        })
    }

    fn accept(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.accept(cx))
    }

    fn discard(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.discard(cx))
    }

    fn suggest(
        &self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut App,
    ) -> Option<EditPrediction> {
        self.update(cx, |this, cx| this.suggest(buffer, cursor_position, cx))
    }
}

/// Returns edits updated based on user edits since the old snapshot. None is returned if any user
/// edit is not a prefix of a predicted insertion.
pub fn interpolate_edits(
    old_snapshot: &BufferSnapshot,
    new_snapshot: &BufferSnapshot,
    current_edits: &[(Range<Anchor>, String)],
) -> Option<Vec<(Range<Anchor>, String)>> {
    let mut edits = Vec::new();

    let mut model_edits = current_edits.iter().peekable();
    for user_edit in new_snapshot.edits_since::<usize>(&old_snapshot.version) {
        while let Some((model_old_range, _)) = model_edits.peek() {
            let model_old_range = model_old_range.to_offset(old_snapshot);
            if model_old_range.end < user_edit.old.start {
                let (model_old_range, model_new_text) = model_edits.next().unwrap();
                edits.push((model_old_range.clone(), model_new_text.clone()));
            } else {
                break;
            }
        }

        if let Some((model_old_range, model_new_text)) = model_edits.peek() {
            let model_old_offset_range = model_old_range.to_offset(old_snapshot);
            if user_edit.old == model_old_offset_range {
                let user_new_text = new_snapshot
                    .text_for_range(user_edit.new.clone())
                    .collect::<String>();

                if let Some(model_suffix) = model_new_text.strip_prefix(&user_new_text) {
                    if !model_suffix.is_empty() {
                        let anchor = old_snapshot.anchor_after(user_edit.old.end);
                        edits.push((anchor..anchor, model_suffix.to_string()));
                    }

                    model_edits.next();
                    continue;
                }
            }
        }

        return None;
    }

    edits.extend(model_edits.cloned());

    if edits.is_empty() { None } else { Some(edits) }
}
