use std::{ops::Range, path::PathBuf, sync::Arc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Difference {
    Changed,
    CurrentOnly,
    ReferenceOnly,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(super) struct DiffStats {
    pub changed: usize,
    pub current_only: usize,
    pub reference_only: usize,
    pub regions: usize,
}

pub(super) struct Comparison {
    pub path: PathBuf,
    pub bytes: Arc<[u8]>,
    pub stats: Option<(u64, DiffStats)>,
}

pub(super) fn difference(current: &[u8], reference: &[u8], offset: usize) -> Option<Difference> {
    match (current.get(offset), reference.get(offset)) {
        (Some(left), Some(right)) if left != right => Some(Difference::Changed),
        (Some(_), None) => Some(Difference::CurrentOnly),
        (None, Some(_)) => Some(Difference::ReferenceOnly),
        _ => None,
    }
}

pub(super) fn stats(current: &[u8], reference: &[u8]) -> DiffStats {
    let mut stats = DiffStats::default();
    let mut previous = false;
    for offset in 0..current.len().max(reference.len()) {
        let difference = difference(current, reference, offset);
        match difference {
            Some(Difference::Changed) => stats.changed += 1,
            Some(Difference::CurrentOnly) => stats.current_only += 1,
            Some(Difference::ReferenceOnly) => stats.reference_only += 1,
            None => {}
        }
        if difference.is_some() && !previous {
            stats.regions += 1;
        }
        previous = difference.is_some();
    }
    stats
}

pub(super) fn find_region(
    current: &[u8],
    reference: &[u8],
    start: usize,
    backwards: bool,
) -> Option<Range<usize>> {
    let length = current.len().max(reference.len());
    let start = start.min(length);
    let differs = |offset| difference(current, reference, offset).is_some();
    let offset = if backwards {
        (0..start)
            .rev()
            .find(|offset| differs(*offset))
            .or_else(|| (start..length).rev().find(|offset| differs(*offset)))
    } else {
        (start..length)
            .find(|offset| differs(*offset))
            .or_else(|| (0..start).find(|offset| differs(*offset)))
    }?;
    let mut start = offset;
    let mut end = offset + 1;
    while start > 0 && differs(start - 1) {
        start -= 1;
    }
    while end < length && differs(end) {
        end += 1;
    }
    Some(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regions_wrapping_and_unequal_lengths() {
        let left = [0, 1, 2, 3, 4, 5];
        let right = [0, 9, 8, 3, 7, 5, 6];
        assert_eq!(
            stats(&left, &right),
            DiffStats {
                changed: 3,
                current_only: 0,
                reference_only: 1,
                regions: 3
            }
        );
        assert_eq!(find_region(&left, &right, 0, false), Some(1..3));
        assert_eq!(find_region(&left, &right, 3, false), Some(4..5));
        assert_eq!(find_region(&left, &right, 5, false), Some(6..7));
        assert_eq!(find_region(&left, &right, 7, false), Some(1..3));
        assert_eq!(find_region(&left, &right, 0, true), Some(6..7));
        assert_eq!(find_region(&left, &right, 6, true), Some(4..5));
        assert_eq!(find_region(&[], &[], 0, false), None);
        assert_eq!(find_region(&left, &left, 0, false), None);
        assert_eq!(stats(&[1, 2], &[]).current_only, 2);
    }
}
