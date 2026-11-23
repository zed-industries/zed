use editor::{
    Anchor, Editor, HighlightKey, MultiBufferOffset, MultiBufferSnapshot, SelectionEffects,
    ToOffset as _,
};
use gpui::{Context, Window, actions};
use language::{SelectionGoal, TransactionId};

use crate::{
    Helix, Mode,
    selection::{HelixRange, next_grapheme_boundary, prev_grapheme_boundary},
};

actions!(
    helix,
    [
        /// Returns to normal mode from insert or select mode.
        NormalMode,
        /// Switches to select mode, where motions extend the selections.
        SelectMode,
        /// Leaves select mode.
        ExitSelectMode,
        /// Switches to insert mode, inserting before each selection.
        InsertMode,
        /// Switches to insert mode, appending after each selection.
        AppendMode,
    ]
);

/// The state needed to turn the insert mode cursors back into Helix
/// selections when insert mode ends.
pub(crate) struct InsertSession {
    kind: InsertKind,
    /// For each selection, the side the cursor is not on: the end for
    /// [`InsertMode`], the start for [`AppendMode`].
    fixed_sides: Vec<Anchor>,
    /// Groups the whole insert session into one undo step.
    pub(crate) first_transaction: Option<TransactionId>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InsertKind {
    Insert,
    Append,
}

pub(crate) fn register(editor: &mut Editor, cx: &mut Context<Helix>) {
    Helix::action(editor, cx, |helix, _: &NormalMode, window, cx| {
        helix.normal_mode(window, cx)
    });
    Helix::action(editor, cx, |helix, _: &SelectMode, window, cx| {
        helix.select_mode(window, cx)
    });
    Helix::action(editor, cx, |helix, _: &ExitSelectMode, window, cx| {
        if helix.mode == Mode::Select {
            helix.switch_mode(Mode::Normal, window, cx);
        }
    });
    Helix::action(editor, cx, |helix, _: &InsertMode, window, cx| {
        helix.enter_insert_mode(InsertKind::Insert, window, cx)
    });
    Helix::action(editor, cx, |helix, _: &AppendMode, window, cx| {
        helix.enter_insert_mode(InsertKind::Append, window, cx)
    });
}

impl Helix {
    fn normal_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.mode {
            Mode::Normal => {}
            Mode::Select => self.switch_mode(Mode::Normal, window, cx),
            Mode::Insert => self.exit_insert_mode(window, cx),
        }
    }

    fn select_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.update_editor(cx, |_, editor, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.move_offsets_with(&mut |snapshot, selection| {
                    let range = HelixRange::from_selection(selection);
                    if range.is_empty() && range.head == snapshot.len() {
                        HelixRange::new(prev_grapheme_boundary(snapshot, range.anchor), range.head)
                            .apply_to(selection);
                    }
                });
            });
        });
        self.switch_mode(Mode::Select, window, cx);
    }

    fn enter_insert_mode(&mut self, kind: InsertKind, window: &mut Window, cx: &mut Context<Self>) {
        self.switch_mode(Mode::Insert, window, cx);
        let fixed_sides = self.update_editor(cx, |_, editor, cx| {
            let display_map = editor.display_snapshot(cx);
            let snapshot = display_map.buffer_snapshot();
            let selections = editor.selections.all::<MultiBufferOffset>(&display_map);
            let fixed_sides = selections
                .iter()
                .map(|selection| match kind {
                    InsertKind::Insert => snapshot.anchor_after(selection.end),
                    // An empty selection (only possible at the end of the
                    // buffer) stays empty, so it moves with the typed text.
                    InsertKind::Append if selection.is_empty() => {
                        snapshot.anchor_after(selection.start)
                    }
                    InsertKind::Append => snapshot.anchor_before(selection.start),
                })
                .collect();
            let mut cursors = selections
                .iter()
                .map(|selection| match kind {
                    InsertKind::Insert => selection.start,
                    InsertKind::Append => selection.end,
                })
                .collect::<Vec<_>>()
                .into_iter();

            // Like Helix, appending at the end of the buffer adds a line ending
            // so the cursor has a character to rest on after insert mode.
            if kind == InsertKind::Append
                && let Some(last) = selections.last()
                && !last.is_empty()
                && last.end == snapshot.len()
            {
                let end = last.end;
                editor.edit([(end..end, "\n")], cx);
            }

            editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
                selections.move_offsets_with(&mut |_, selection| {
                    if let Some(cursor) = cursors.next() {
                        selection.collapse_to(cursor, SelectionGoal::None);
                    }
                });
            });
            fixed_sides
        });
        self.insert_session = Some(InsertSession {
            kind,
            fixed_sides: fixed_sides.unwrap_or_default(),
            first_transaction: None,
        });
        self.highlight_insert_mode_ranges(cx);
    }

    fn exit_insert_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let session = self.insert_session.take();
        self.update_editor(cx, |_, editor, cx| {
            editor.clear_background_highlights(HighlightKey::HelixInsertSelections, cx);
            editor.dismiss_menus_and_popups(false, window, cx);
            let Some(session) = session else {
                return;
            };
            if let Some(transaction_id) = session.first_transaction {
                editor.group_until_transaction(transaction_id, cx);
            }
            let restore = editor.selections.count() == session.fixed_sides.len();
            let mut fixed_sides = session.fixed_sides.iter();
            editor.change_selections(SelectionEffects::default(), window, cx, |selections| {
                selections.move_offsets_with(&mut |snapshot, selection| {
                    let cursor = selection.head();
                    let fixed_side = fixed_sides
                        .next()
                        .filter(|_| restore)
                        .map(|anchor| anchor.to_offset(snapshot));
                    let range = match (session.kind, fixed_side) {
                        (InsertKind::Insert, Some(end)) if cursor <= end => {
                            HelixRange::new(end, cursor)
                        }
                        (InsertKind::Append, Some(start)) if start <= cursor => {
                            HelixRange::new(start, cursor)
                        }
                        _ => HelixRange::point(cursor),
                    };
                    range.min_width_1(snapshot).apply_to(selection);
                    selection.goal = SelectionGoal::None;
                });
            });
        });
        self.switch_mode(Mode::Normal, window, cx);
    }
}

impl Helix {
    /// Zed only has cursors during insert mode, so the selections Helix keeps
    /// showing there are drawn as highlights, up to the cursors. After
    /// [`InsertMode`], a single grapheme is the normal mode cursor rather than
    /// a selection, so it is not highlighted.
    pub(crate) fn highlight_insert_mode_ranges(&mut self, cx: &mut Context<Self>) {
        let Some(kind) = self.insert_session.as_ref().map(|session| session.kind) else {
            return;
        };
        self.update_editor(cx, |helix, editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let ranges = helix
                .insert_mode_sides(editor, &snapshot)
                .unwrap_or_default()
                .into_iter()
                .map(|(fixed_side, cursor)| fixed_side.min(cursor)..fixed_side.max(cursor))
                .filter(|range| {
                    !range.is_empty()
                        && !(kind == InsertKind::Insert
                            && next_grapheme_boundary(&snapshot, range.start) == range.end)
                })
                .map(|range| snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end))
                .collect::<Vec<_>>();
            editor.highlight_background(
                HighlightKey::HelixInsertSelections,
                &ranges,
                |_, theme| theme.players().local().selection,
                cx,
            );
        });
    }

    /// The fixed side and the cursor of each selection during insert mode.
    fn insert_mode_sides(
        &self,
        editor: &Editor,
        snapshot: &MultiBufferSnapshot,
    ) -> Option<Vec<(MultiBufferOffset, MultiBufferOffset)>> {
        let session = self.insert_session.as_ref()?;
        let cursors = editor
            .selections
            .disjoint_anchors_arc()
            .iter()
            .map(|selection| selection.head().to_offset(snapshot))
            .collect::<Vec<_>>();
        if cursors.len() != session.fixed_sides.len() {
            return None;
        }
        Some(
            session
                .fixed_sides
                .iter()
                .map(|fixed_side| fixed_side.to_offset(snapshot))
                .zip(cursors)
                .collect(),
        )
    }
}

#[cfg(test)]
impl Helix {
    /// The selections Helix would show during insert mode: the text typed
    /// so far belongs to them.
    pub(crate) fn insert_mode_ranges(
        &self,
        editor: &Editor,
        cx: &gpui::App,
    ) -> Option<Vec<HelixRange>> {
        let kind = self.insert_session.as_ref()?.kind;
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let sides = self.insert_mode_sides(editor, &snapshot)?;
        Some(
            sides
                .into_iter()
                .map(|(fixed_side, cursor)| match kind {
                    InsertKind::Insert => HelixRange::new(fixed_side, cursor),
                    InsertKind::Append => {
                        HelixRange::new(fixed_side, next_grapheme_boundary(&snapshot, cursor))
                    }
                })
                .collect(),
        )
    }
}
