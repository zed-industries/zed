//! An index-accessed table implementation that avoids duplicate entries.
use std::collections::HashMap;
use std::hash::Hash;
use std::slice;

/// Collect items into the `table` list, removing duplicates.
pub(crate) struct UniqueTable<'entries, T: Eq + Hash> {
    table: Vec<&'entries T>,
    map: HashMap<&'entries T, usize>,
}

impl<'entries, T: Eq + Hash> UniqueTable<'entries, T> {
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, entry: &'entries T) -> usize {
        match self.map.get(&entry) {
            None => {
                let i = self.table.len();
                self.table.push(entry);
                self.map.insert(entry, i);
                i
            }
            Some(&i) => i,
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
    pub fn iter(&self) -> slice::Iter<'_, &'entries T> {
        self.table.iter()
    }
}

/// A table of sequences which tries to avoid common subsequences.
pub(crate) struct UniqueSeqTable<T: PartialEq + Clone> {
    table: Vec<T>,
}

impl<T: PartialEq + Clone> UniqueSeqTable<T> {
    pub fn new() -> Self {
        Self { table: Vec::new() }
    }
    pub fn add(&mut self, values: &[T]) -> usize {
        if values.is_empty() {
            return 0;
        }
        if let Some(offset) = find_subsequence(values, &self.table) {
            offset
        } else {
            let table_len = self.table.len();

            // Try to put in common the last elements of the table if they're a prefix of the new
            // sequence.
            //
            // We know there wasn't a full match, so the best prefix we can hope to find contains
            // all the values but the last one.
            let mut start_from = usize::min(table_len, values.len() - 1);
            while start_from != 0 {
                // Loop invariant: start_from <= table_len, so table_len - start_from >= 0.
                if values[0..start_from] == self.table[table_len - start_from..table_len] {
                    break;
                }
                start_from -= 1;
            }

            self.table
                .extend(values[start_from..values.len()].iter().cloned());
            table_len - start_from
        }
    }
    pub fn len(&self) -> usize {
        self.table.len()
    }
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.table.iter()
    }
}

/// Try to find the subsequence `sub` in the `whole` sequence. Returns None if
/// it's not been found, or Some(index) if it has been. Naive implementation
/// until proven we need something better.
fn find_subsequence<T: PartialEq>(sub: &[T], whole: &[T]) -> Option<usize> {
    assert!(!sub.is_empty());
    // We want i + sub.len() <= whole.len(), i.e. i < whole.len() + 1 - sub.len().
    if whole.len() < sub.len() {
        return None;
    }
    let max = whole.len() - sub.len();
    for i in 0..=max {
        if whole[i..i + sub.len()] == sub[..] {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_find_subsequence() {
    assert_eq!(find_subsequence(&vec![1], &vec![4]), None);
    assert_eq!(find_subsequence(&vec![1], &vec![1]), Some(0));
    assert_eq!(find_subsequence(&vec![1, 2], &vec![1]), None);
    assert_eq!(find_subsequence(&vec![1, 2], &vec![1, 2]), Some(0));
    assert_eq!(find_subsequence(&vec![1, 2], &vec![1, 3]), None);
    assert_eq!(find_subsequence(&vec![1, 2], &vec![0, 1, 2]), Some(1));
    assert_eq!(find_subsequence(&vec![1, 2], &vec![0, 1, 3, 1]), None);
    assert_eq!(find_subsequence(&vec![1, 2], &vec![0, 1, 3, 1, 2]), Some(3));
    assert_eq!(
        find_subsequence(&vec![1, 1, 3], &vec![1, 1, 1, 3, 3]),
        Some(1)
    );
}

#[test]
fn test_optimal_add() {
    let mut seq_table = UniqueSeqTable::new();
    // [0, 1, 2, 3]
    assert_eq!(seq_table.add(&vec![0, 1, 2, 3]), 0);
    assert_eq!(seq_table.add(&vec![0, 1, 2, 3]), 0);
    assert_eq!(seq_table.add(&vec![1, 2, 3]), 1);
    assert_eq!(seq_table.add(&vec![2, 3]), 2);
    assert_eq!(seq_table.len(), 4);
    // [0, 1, 2, 3, 4]
    assert_eq!(seq_table.add(&vec![2, 3, 4]), 2);
    assert_eq!(seq_table.len(), 5);
    // [0, 1, 2, 3, 4, 6, 5, 7]
    assert_eq!(seq_table.add(&vec![4, 6, 5, 7]), 4);
    assert_eq!(seq_table.len(), 8);
    // [0, 1, 2, 3, 4, 6, 5, 7, 8, 2, 3, 4]
    assert_eq!(seq_table.add(&vec![8, 2, 3, 4]), 8);
    assert_eq!(seq_table.add(&vec![8]), 8);
    assert_eq!(seq_table.len(), 12);
}
