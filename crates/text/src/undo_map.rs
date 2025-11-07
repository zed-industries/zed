use crate::UndoOperation;
use clock::Lamport;
use std::cmp;
use sum_tree::{Bias, SumTree};

#[derive(Copy, Clone, Debug)]
struct UndoMapEntry {
    key: UndoMapKey,
    undo_count: u32,
}

impl sum_tree::Item for UndoMapEntry {
    type Summary = UndoMapKey;

    fn summary(&self, _cx: ()) -> Self::Summary {
        self.key
    }
}

impl sum_tree::KeyedItem for UndoMapEntry {
    type Key = UndoMapKey;

    fn key(&self) -> Self::Key {
        self.key
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct UndoMapKey {
    edit_id: clock::Lamport,
    undo_id: clock::Lamport,
}

impl sum_tree::ContextLessSummary for UndoMapKey {
    fn zero() -> Self {
        UndoMapKey {
            edit_id: Lamport::MIN,
            undo_id: Lamport::MIN,
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        *self = cmp::max(*self, *summary);
    }
}

#[derive(Clone, Default)]
pub struct UndoMap(SumTree<UndoMapEntry>);

impl UndoMap {
    pub fn insert(&mut self, undo: &UndoOperation) {
        let edits = undo
            .counts
            .iter()
            .map(|(edit_id, count)| {
                sum_tree::Edit::Insert(UndoMapEntry {
                    key: UndoMapKey {
                        edit_id: *edit_id,
                        undo_id: undo.timestamp,
                    },
                    undo_count: *count,
                })
            })
            .collect::<Vec<_>>();
        self.0.edit(edits, ());
    }

    pub fn is_undone(&self, edit_id: clock::Lamport) -> bool {
        self.undo_count(edit_id) % 2 == 1
    }
    pub fn was_undone(&self, edit_id: clock::Lamport, version: &clock::Global) -> bool {
        let mut cursor = self.0.cursor::<UndoMapKey>(());
        cursor.seek(
            &UndoMapKey {
                edit_id,
                undo_id: Lamport::MIN,
            },
            Bias::Left,
        );

        let mut undo_count = 0;
        for entry in cursor {
            if entry.key.edit_id != edit_id {
                break;
            }

            if version.observed(entry.key.undo_id) {
                undo_count = cmp::max(undo_count, entry.undo_count);
            }
        }

        undo_count % 2 == 1
    }

    pub fn undo_count(&self, edit_id: clock::Lamport) -> u32 {
        let mut cursor = self.0.cursor::<UndoMapKey>(());
        cursor.seek(
            &UndoMapKey {
                edit_id,
                undo_id: Lamport::MIN,
            },
            Bias::Left,
        );

        let mut undo_count = 0;
        for entry in cursor {
            if entry.key.edit_id != edit_id {
                break;
            }

            undo_count = cmp::max(undo_count, entry.undo_count);
        }
        undo_count
    }
}
