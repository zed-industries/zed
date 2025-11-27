use std::hash::Hash;

use crate::hash::DftHashMap;

#[derive(Debug, PartialEq)]
pub(crate) enum DiffResult<T> {
    Left(T),
    Both(T, T),
    Right(T),
}

pub(crate) fn slice<'a, T: PartialEq + Clone>(
    lhs: &'a [T],
    rhs: &'a [T],
) -> Vec<DiffResult<&'a T>> {
    wu_diff::diff(lhs, rhs)
        .into_iter()
        .map(|result| match result {
            wu_diff::DiffResult::Removed(r) => DiffResult::Left(
                &lhs[r
                    .old_index
                    .expect("Index should be present for this diff variant")],
            ),
            wu_diff::DiffResult::Common(c) => {
                let lhs_id = c
                    .old_index
                    .expect("Index should be present for this diff variant");
                let rhs_id = c
                    .new_index
                    .expect("Index should be present for this diff variant");
                DiffResult::Both(&lhs[lhs_id], &rhs[rhs_id])
            }
            wu_diff::DiffResult::Added(a) => DiffResult::Right(
                &rhs[a
                    .new_index
                    .expect("Index should be present for this diff variant")],
            ),
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub(crate) fn slice_by_hash<'a, T: Eq + Hash>(
    lhs: &'a [T],
    rhs: &'a [T],
) -> Vec<DiffResult<&'a T>> {
    //
    // This is the decorate-sort-undecorate pattern, or Schwartzian
    // transform, for diffing.
    let mut value_ids: DftHashMap<&T, u32> = DftHashMap::default();
    let mut id_values: DftHashMap<u32, &T> = DftHashMap::default();

    let mut lhs_ids = Vec::with_capacity(lhs.len());
    for value in lhs {
        let id: u32 = match value_ids.get(value) {
            Some(id) => *id,
            None => {
                let new_id = value_ids.len() as u32;
                value_ids.insert(value, new_id);
                id_values.insert(new_id, value);
                new_id
            }
        };
        lhs_ids.push(id);
    }

    let mut rhs_ids = Vec::with_capacity(rhs.len());
    for value in rhs {
        let id = match value_ids.get(value) {
            Some(id) => *id,
            None => {
                let new_id = value_ids.len() as u32;
                value_ids.insert(value, new_id);
                id_values.insert(new_id, value);
                new_id
            }
        };
        rhs_ids.push(id);
    }

    slice(&lhs_ids[..], &rhs_ids[..])
        .into_iter()
        .map(|result| match result {
            DiffResult::Left(id) => {
                DiffResult::Left(*id_values.get(id).expect("ID should be present in map"))
            }
            DiffResult::Both(lhs_id, rhs_id) => DiffResult::Both(
                *id_values.get(lhs_id).expect("ID should be present in map"),
                *id_values.get(rhs_id).expect("ID should be present in map"),
            ),
            DiffResult::Right(id) => {
                DiffResult::Right(*id_values.get(id).expect("ID should be present in map"))
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_same_items() {
        let diff_items = slice(&["a", "b"], &["a", "b"]);
        assert_eq!(
            diff_items,
            vec![DiffResult::Both(&"a", &"a"), DiffResult::Both(&"b", &"b")]
        );
    }

    #[test]
    fn test_slice_different_items() {
        let diff_items = slice(&["a", "b"], &["c", "d"]);
        assert_eq!(
            diff_items,
            vec![
                DiffResult::Left(&"a"),
                DiffResult::Left(&"b"),
                DiffResult::Right(&"c"),
                DiffResult::Right(&"d"),
            ]
        );
    }

    #[test]
    fn test_slice_by_hash_same_items() {
        let diff_items = slice_by_hash(&["a", "b"], &["a", "b"]);
        assert_eq!(
            diff_items,
            vec![DiffResult::Both(&"a", &"a"), DiffResult::Both(&"b", &"b")]
        );
    }

    #[test]
    fn test_slice_by_hash_different_items() {
        let diff_items = slice_by_hash(&["a", "b"], &["c", "d"]);
        assert_eq!(
            diff_items,
            vec![
                DiffResult::Left(&"a"),
                DiffResult::Left(&"b"),
                DiffResult::Right(&"c"),
                DiffResult::Right(&"d"),
            ]
        );
    }
}
