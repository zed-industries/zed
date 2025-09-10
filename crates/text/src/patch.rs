use crate::Edit;
use std::{
    cmp, mem,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Patch<T>(Vec<Edit<T>>);

impl<T> Patch<T>
where
    T: 'static
        + Clone
        + Copy
        + Ord
        + Sub<T, Output = T>
        + Add<T, Output = T>
        + AddAssign
        + Default
        + PartialEq,
{
    pub fn new(edits: Vec<Edit<T>>) -> Self {
        #[cfg(debug_assertions)]
        {
            let mut last_edit: Option<&Edit<T>> = None;
            for edit in &edits {
                if let Some(last_edit) = last_edit {
                    assert!(edit.old.start > last_edit.old.end);
                    assert!(edit.new.start > last_edit.new.end);
                }
                last_edit = Some(edit);
            }
        }
        Self(edits)
    }

    pub fn edits(&self) -> &[Edit<T>] {
        &self.0
    }

    pub fn into_inner(self) -> Vec<Edit<T>> {
        self.0
    }

    #[must_use]
    pub fn compose(&self, new_edits_iter: impl IntoIterator<Item = Edit<T>>) -> Self {
        let mut old_edits_iter = self.0.iter().cloned().peekable();
        let mut new_edits_iter = new_edits_iter.into_iter().peekable();
        let mut composed = Patch(Vec::new());

        let mut old_start = T::default();
        let mut new_start = T::default();
        loop {
            let old_edit = old_edits_iter.peek_mut();
            let new_edit = new_edits_iter.peek_mut();

            // Push the old edit if its new end is before the new edit's old start.
            if let Some(old_edit) = old_edit.as_ref() {
                let new_edit = new_edit.as_ref();
                if new_edit.is_none_or(|new_edit| old_edit.new.end < new_edit.old.start) {
                    let catchup = old_edit.old.start - old_start;
                    old_start += catchup;
                    new_start += catchup;

                    let old_end = old_start + old_edit.old_len();
                    let new_end = new_start + old_edit.new_len();
                    composed.push(Edit {
                        old: old_start..old_end,
                        new: new_start..new_end,
                    });
                    old_start = old_end;
                    new_start = new_end;
                    old_edits_iter.next();
                    continue;
                }
            }

            // Push the new edit if its old end is before the old edit's new start.
            if let Some(new_edit) = new_edit.as_ref() {
                let old_edit = old_edit.as_ref();
                if old_edit.is_none_or(|old_edit| new_edit.old.end < old_edit.new.start) {
                    let catchup = new_edit.new.start - new_start;
                    old_start += catchup;
                    new_start += catchup;

                    let old_end = old_start + new_edit.old_len();
                    let new_end = new_start + new_edit.new_len();
                    composed.push(Edit {
                        old: old_start..old_end,
                        new: new_start..new_end,
                    });
                    old_start = old_end;
                    new_start = new_end;
                    new_edits_iter.next();
                    continue;
                }
            }

            // If we still have edits by this point then they must intersect, so we compose them.
            if let Some((old_edit, new_edit)) = old_edit.zip(new_edit) {
                if old_edit.new.start < new_edit.old.start {
                    let catchup = old_edit.old.start - old_start;
                    old_start += catchup;
                    new_start += catchup;

                    let overshoot = new_edit.old.start - old_edit.new.start;
                    let old_end = cmp::min(old_start + overshoot, old_edit.old.end);
                    let new_end = new_start + overshoot;
                    composed.push(Edit {
                        old: old_start..old_end,
                        new: new_start..new_end,
                    });

                    old_edit.old.start = old_end;
                    old_edit.new.start += overshoot;
                    old_start = old_end;
                    new_start = new_end;
                } else {
                    let catchup = new_edit.new.start - new_start;
                    old_start += catchup;
                    new_start += catchup;

                    let overshoot = old_edit.new.start - new_edit.old.start;
                    let old_end = old_start + overshoot;
                    let new_end = cmp::min(new_start + overshoot, new_edit.new.end);
                    composed.push(Edit {
                        old: old_start..old_end,
                        new: new_start..new_end,
                    });

                    new_edit.old.start += overshoot;
                    new_edit.new.start = new_end;
                    old_start = old_end;
                    new_start = new_end;
                }

                if old_edit.new.end > new_edit.old.end {
                    let old_end = old_start + cmp::min(old_edit.old_len(), new_edit.old_len());
                    let new_end = new_start + new_edit.new_len();
                    composed.push(Edit {
                        old: old_start..old_end,
                        new: new_start..new_end,
                    });

                    old_edit.old.start = old_end;
                    old_edit.new.start = new_edit.old.end;
                    old_start = old_end;
                    new_start = new_end;
                    new_edits_iter.next();
                } else {
                    let old_end = old_start + old_edit.old_len();
                    let new_end = new_start + cmp::min(old_edit.new_len(), new_edit.new_len());
                    composed.push(Edit {
                        old: old_start..old_end,
                        new: new_start..new_end,
                    });

                    new_edit.old.start = old_edit.new.end;
                    new_edit.new.start = new_end;
                    old_start = old_end;
                    new_start = new_end;
                    old_edits_iter.next();
                }
            } else {
                break;
            }
        }

        composed
    }

    pub fn invert(&mut self) -> &mut Self {
        for edit in &mut self.0 {
            mem::swap(&mut edit.old, &mut edit.new);
        }
        self
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, edit: Edit<T>) {
        if edit.is_empty() {
            return;
        }

        if let Some(last) = self.0.last_mut() {
            if last.old.end >= edit.old.start {
                last.old.end = edit.old.end;
                last.new.end = edit.new.end;
            } else {
                self.0.push(edit);
            }
        } else {
            self.0.push(edit);
        }
    }

    pub fn old_to_new(&self, old: T) -> T {
        let ix = match self.0.binary_search_by(|probe| probe.old.start.cmp(&old)) {
            Ok(ix) => ix,
            Err(ix) => {
                if ix == 0 {
                    return old;
                } else {
                    ix - 1
                }
            }
        };
        if let Some(edit) = self.0.get(ix) {
            if old >= edit.old.end {
                edit.new.end + (old - edit.old.end)
            } else {
                edit.new.start
            }
        } else {
            old
        }
    }
}

impl<T> Patch<T> {
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut Edit<T>) -> bool,
    {
        self.0.retain_mut(f);
    }
}

impl<T: Clone> IntoIterator for Patch<T> {
    type Item = Edit<T>;
    type IntoIter = std::vec::IntoIter<Edit<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Clone> IntoIterator for &'a Patch<T> {
    type Item = Edit<T>;
    type IntoIter = std::iter::Cloned<std::slice::Iter<'a, Edit<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}

impl<'a, T: Clone> IntoIterator for &'a mut Patch<T> {
    type Item = Edit<T>;
    type IntoIter = std::iter::Cloned<std::slice::Iter<'a, Edit<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::env;

    #[gpui::test]
    fn test_one_disjoint_edit() {
        assert_patch_composition(
            Patch(vec![Edit {
                old: 1..3,
                new: 1..4,
            }]),
            Patch(vec![Edit {
                old: 0..0,
                new: 0..4,
            }]),
            Patch(vec![
                Edit {
                    old: 0..0,
                    new: 0..4,
                },
                Edit {
                    old: 1..3,
                    new: 5..8,
                },
            ]),
        );

        assert_patch_composition(
            Patch(vec![Edit {
                old: 1..3,
                new: 1..4,
            }]),
            Patch(vec![Edit {
                old: 5..9,
                new: 5..7,
            }]),
            Patch(vec![
                Edit {
                    old: 1..3,
                    new: 1..4,
                },
                Edit {
                    old: 4..8,
                    new: 5..7,
                },
            ]),
        );
    }

    #[gpui::test]
    fn test_one_overlapping_edit() {
        assert_patch_composition(
            Patch(vec![Edit {
                old: 1..3,
                new: 1..4,
            }]),
            Patch(vec![Edit {
                old: 3..5,
                new: 3..6,
            }]),
            Patch(vec![Edit {
                old: 1..4,
                new: 1..6,
            }]),
        );
    }

    #[gpui::test]
    fn test_two_disjoint_and_overlapping() {
        assert_patch_composition(
            Patch(vec![
                Edit {
                    old: 1..3,
                    new: 1..4,
                },
                Edit {
                    old: 8..12,
                    new: 9..11,
                },
            ]),
            Patch(vec![
                Edit {
                    old: 0..0,
                    new: 0..4,
                },
                Edit {
                    old: 3..10,
                    new: 7..9,
                },
            ]),
            Patch(vec![
                Edit {
                    old: 0..0,
                    new: 0..4,
                },
                Edit {
                    old: 1..12,
                    new: 5..10,
                },
            ]),
        );
    }

    #[gpui::test]
    fn test_two_new_edits_overlapping_one_old_edit() {
        assert_patch_composition(
            Patch(vec![Edit {
                old: 0..0,
                new: 0..3,
            }]),
            Patch(vec![
                Edit {
                    old: 0..0,
                    new: 0..1,
                },
                Edit {
                    old: 1..2,
                    new: 2..2,
                },
            ]),
            Patch(vec![Edit {
                old: 0..0,
                new: 0..3,
            }]),
        );

        assert_patch_composition(
            Patch(vec![Edit {
                old: 2..3,
                new: 2..4,
            }]),
            Patch(vec![
                Edit {
                    old: 0..2,
                    new: 0..1,
                },
                Edit {
                    old: 3..3,
                    new: 2..5,
                },
            ]),
            Patch(vec![Edit {
                old: 0..3,
                new: 0..6,
            }]),
        );

        assert_patch_composition(
            Patch(vec![Edit {
                old: 0..0,
                new: 0..2,
            }]),
            Patch(vec![
                Edit {
                    old: 0..0,
                    new: 0..2,
                },
                Edit {
                    old: 2..5,
                    new: 4..4,
                },
            ]),
            Patch(vec![Edit {
                old: 0..3,
                new: 0..4,
            }]),
        );
    }

    #[gpui::test]
    fn test_two_new_edits_touching_one_old_edit() {
        assert_patch_composition(
            Patch(vec![
                Edit {
                    old: 2..3,
                    new: 2..4,
                },
                Edit {
                    old: 7..7,
                    new: 8..11,
                },
            ]),
            Patch(vec![
                Edit {
                    old: 2..3,
                    new: 2..2,
                },
                Edit {
                    old: 4..4,
                    new: 3..4,
                },
            ]),
            Patch(vec![
                Edit {
                    old: 2..3,
                    new: 2..4,
                },
                Edit {
                    old: 7..7,
                    new: 8..11,
                },
            ]),
        );
    }

    #[gpui::test]
    fn test_old_to_new() {
        let patch = Patch(vec![
            Edit {
                old: 2..4,
                new: 2..4,
            },
            Edit {
                old: 7..8,
                new: 7..11,
            },
        ]);
        assert_eq!(patch.old_to_new(0), 0);
        assert_eq!(patch.old_to_new(1), 1);
        assert_eq!(patch.old_to_new(2), 2);
        assert_eq!(patch.old_to_new(3), 2);
        assert_eq!(patch.old_to_new(4), 4);
        assert_eq!(patch.old_to_new(5), 5);
        assert_eq!(patch.old_to_new(6), 6);
        assert_eq!(patch.old_to_new(7), 7);
        assert_eq!(patch.old_to_new(8), 11);
        assert_eq!(patch.old_to_new(9), 12);
    }

    #[gpui::test(iterations = 100)]
    fn test_random_patch_compositions(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(20);

        let initial_chars = (0..rng.random_range(0..=100))
            .map(|_| rng.random_range(b'a'..=b'z') as char)
            .collect::<Vec<_>>();
        log::info!("initial chars: {:?}", initial_chars);

        // Generate two sequential patches
        let mut patches = Vec::new();
        let mut expected_chars = initial_chars.clone();
        for i in 0..2 {
            log::info!("patch {}:", i);

            let mut delta = 0i32;
            let mut last_edit_end = 0;
            let mut edits = Vec::new();

            for _ in 0..operations {
                if last_edit_end >= expected_chars.len() {
                    break;
                }

                let end = rng.random_range(last_edit_end..=expected_chars.len());
                let start = rng.random_range(last_edit_end..=end);
                let old_len = end - start;

                let mut new_len = rng.random_range(0..=3);
                if start == end && new_len == 0 {
                    new_len += 1;
                }

                last_edit_end = start + new_len + 1;

                let new_chars = (0..new_len)
                    .map(|_| rng.random_range(b'A'..=b'Z') as char)
                    .collect::<Vec<_>>();
                log::info!(
                    "  editing {:?}: {:?}",
                    start..end,
                    new_chars.iter().collect::<String>()
                );
                edits.push(Edit {
                    old: (start as i32 - delta) as u32..(end as i32 - delta) as u32,
                    new: start as u32..(start + new_len) as u32,
                });
                expected_chars.splice(start..end, new_chars);

                delta += new_len as i32 - old_len as i32;
            }

            patches.push(Patch(edits));
        }

        log::info!("old patch: {:?}", &patches[0]);
        log::info!("new patch: {:?}", &patches[1]);
        log::info!("initial chars: {:?}", initial_chars);
        log::info!("final chars: {:?}", expected_chars);

        // Compose the patches, and verify that it has the same effect as applying the
        // two patches separately.
        let composed = patches[0].compose(&patches[1]);
        log::info!("composed patch: {:?}", &composed);

        let mut actual_chars = initial_chars;
        for edit in composed.0 {
            actual_chars.splice(
                edit.new.start as usize..edit.new.start as usize + edit.old.len(),
                expected_chars[edit.new.start as usize..edit.new.end as usize]
                    .iter()
                    .copied(),
            );
        }

        assert_eq!(actual_chars, expected_chars);
    }

    #[track_caller]
    #[allow(clippy::almost_complete_range)]
    fn assert_patch_composition(old: Patch<u32>, new: Patch<u32>, composed: Patch<u32>) {
        let original = ('a'..'z').collect::<Vec<_>>();
        let inserted = ('A'..'Z').collect::<Vec<_>>();

        let mut expected = original.clone();
        apply_patch(&mut expected, &old, &inserted);
        apply_patch(&mut expected, &new, &inserted);

        let mut actual = original;
        apply_patch(&mut actual, &composed, &expected);
        assert_eq!(
            actual.into_iter().collect::<String>(),
            expected.into_iter().collect::<String>(),
            "expected patch is incorrect"
        );

        assert_eq!(old.compose(&new), composed);
    }

    fn apply_patch(text: &mut Vec<char>, patch: &Patch<u32>, new_text: &[char]) {
        for edit in patch.0.iter().rev() {
            text.splice(
                edit.old.start as usize..edit.old.end as usize,
                new_text[edit.new.start as usize..edit.new.end as usize]
                    .iter()
                    .copied(),
            );
        }
    }
}
