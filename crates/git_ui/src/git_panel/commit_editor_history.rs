use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const MAX_HISTORY: usize = 50;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct SerializedCommitEditorHistory {
    entries: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct CommitEditorHistory {
    entries: VecDeque<String>,
    cursor: Option<usize>, // index into entries VecDeque (0..len-1), the greater the index, the older is an item; None means pointing before the first entry
    pending_edit: Option<String>,
}

impl CommitEditorHistory {
    pub fn entries(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }

    // TODO: add 2nd param, String, taked from the editor text upon commit_changes
    pub fn add_new_entry(&mut self, message: String) {
        // not checking for an empty message - relying on the commit editor not letting it through

        // remove any exact duplicate
        if let Some(pos) = self.entries.iter().position(|m| m == &message) {
            self.entries.remove(pos);
        }

        self.entries.push_front(message);

        while self.entries.len() > MAX_HISTORY {
            self.entries.truncate(MAX_HISTORY);
        }

        self.cursor = Some(0usize); // reset navigation state
    }

    pub fn prev(&mut self) -> Option<&str> {
        match self.cursor {
            Some(cursor) => {
                if cursor < self.entries.len() - 1 {
                    self.cursor = Some(cursor + 1);
                } else {
                    return None;
                }
            }
            None => {
                if self.entries.is_empty() {
                    return None;
                } else {
                    self.cursor = Some(0usize);
                }
            }
        }

        return self
            .entries
            .get(
                self.cursor
                    .expect("History must contain at least one entry"),
            )
            .map(|s| s.as_str());
    }

    pub fn next(&mut self) -> Option<&str> {
        match self.cursor {
            Some(cursor) => {
                if cursor > 0 {
                    self.cursor = Some(cursor - 1);
                    self.entries
                        .get(
                            self.cursor
                                .expect("History must contain at least one entry"),
                        )
                        .map(|s| s.as_str())
                } else {
                    self.cursor = None;
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_pending_edit(&self) -> Option<&str> {
        self.pending_edit.as_deref()
    }

    pub fn set_pending_edit(&mut self, message: String) {
        self.pending_edit = Some(message);
        self.cursor = None;
    }

    pub fn to_serialized(&self) -> SerializedCommitEditorHistory {
        let mut entries = Vec::with_capacity(MAX_HISTORY);
        let (front, back) = self.entries.as_slices();
        entries.extend_from_slice(front);
        entries.extend_from_slice(back);

        SerializedCommitEditorHistory { entries }
    }

    pub fn from_serialized(serialized: SerializedCommitEditorHistory) -> Self {
        Self {
            entries: VecDeque::from_iter(serialized.entries),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
   // TODO: add tests 
}
