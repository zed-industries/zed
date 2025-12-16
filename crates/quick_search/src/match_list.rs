use crate::types::{MatchId, MatchKey, QuickMatch, QuickMatchPatch};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct MatchList {
    items: Vec<QuickMatch>,
    id_index: HashMap<MatchId, usize>,
    key_index: HashMap<MatchKey, MatchId>,
    pending_patches_by_key: HashMap<MatchKey, Vec<QuickMatchPatch>>,
    max_results: usize,
    truncated: bool,
}

impl MatchList {
    pub fn new(max_results: usize) -> Self {
        Self {
            items: Vec::new(),
            id_index: HashMap::new(),
            key_index: HashMap::new(),
            pending_patches_by_key: HashMap::new(),
            max_results,
            truncated: false,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.id_index.clear();
        self.key_index.clear();
        self.pending_patches_by_key.clear();
        self.truncated = false;
    }

    pub fn extend(&mut self, batch: Vec<QuickMatch>) -> bool {
        for mut match_item in batch {
            if self.items.len() >= self.max_results {
                self.truncated = true;
                break;
            }
            if self.id_index.contains_key(&match_item.id) {
                continue;
            }
            if self.key_index.contains_key(&match_item.key) {
                continue;
            }
            if let Some(patches) = self.pending_patches_by_key.remove(&match_item.key) {
                for patch in patches {
                    match_item.apply_patch(patch);
                }
            }
            let index = self.items.len();
            let id = match_item.id;
            let key = match_item.key;
            self.items.push(match_item);
            self.id_index.insert(id, index);
            self.key_index.insert(key, id);
        }
        self.truncated
    }

    pub fn match_count(&self) -> usize {
        self.items.len()
    }

    pub fn total_results(&self) -> usize {
        self.items.len()
    }

    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    pub fn item(&self, index: usize) -> Option<&QuickMatch> {
        self.items.get(index)
    }

    pub fn item_by_id(&self, id: MatchId) -> Option<&QuickMatch> {
        let index = self.id_index.get(&id).copied()?;
        self.items.get(index)
    }

    pub fn index_by_id(&self, id: MatchId) -> Option<usize> {
        self.id_index.get(&id).copied()
    }

    pub fn id_by_key(&self, key: MatchKey) -> Option<MatchId> {
        self.key_index.get(&key).copied()
    }

    pub fn key_by_id(&self, id: MatchId) -> Option<MatchKey> {
        self.item_by_id(id).map(|match_item| match_item.key)
    }

    pub fn update_by_id(&mut self, id: MatchId, patch: QuickMatchPatch) -> bool {
        let Some(&index) = self.id_index.get(&id) else {
            return false;
        };
        if let Some(item) = self.items.get_mut(index) {
            let changed = item.apply_patch(patch);
            return changed;
        }
        false
    }

    pub fn update_by_key_or_queue(&mut self, key: MatchKey, patch: QuickMatchPatch) -> bool {
        let Some(id) = self.id_by_key(key) else {
            if self.truncated {
                return false;
            }
            self.pending_patches_by_key
                .entry(key)
                .or_default()
                .push(patch);
            return false;
        };
        self.update_by_id(id, patch)
    }

    pub fn sort_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&QuickMatch, &QuickMatch) -> Ordering,
    {
        self.items
            .sort_by(|a, b| compare(a, b).then_with(|| a.id.cmp(&b.id)));
        self.id_index.clear();
        self.key_index.clear();
        for (index, match_item) in self.items.iter().enumerate() {
            self.id_index.insert(match_item.id, index);
            self.key_index.insert(match_item.key, match_item.id);
        }
    }
}
