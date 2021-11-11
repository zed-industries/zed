use std::mem;

type Edit = buffer::Edit<u32>;

#[derive(Default, Debug, PartialEq, Eq)]
struct Patch(Vec<Edit>);

impl Patch {
    fn compose(&self, new: &Self) -> Patch {
        let mut composed = Vec::<Edit>::new();
        let mut old_edits = self.0.iter().cloned().peekable();
        let mut old_delta = 0;
        let mut new_delta = 0;
        let mut intermediate_start;
        let mut intermediate_end = 0;

        for mut new_edit in new.0.iter().cloned() {
            let new_edit_delta = new_edit.new.len() as i32 - new_edit.old.len() as i32;

            if let Some(last_edit) = composed.last_mut() {
                if intermediate_end >= new_edit.old.start {
                    if new_edit.old.end > intermediate_end {
                        last_edit.old.end += new_edit.old.end - intermediate_end;
                        last_edit.new.end += new_edit.old.end - intermediate_end;
                        intermediate_end = new_edit.old.end;
                    }
                    last_edit.new.end = (last_edit.new.end as i32 + new_edit_delta) as u32;
                    continue;
                }
            }

            intermediate_start = new_edit.old.start;
            intermediate_end = new_edit.old.end;
            new_edit.old.start = (new_edit.old.start as i32 - old_delta) as u32;
            new_edit.old.end = (new_edit.old.end as i32 - old_delta) as u32;

            while let Some(old_edit) = old_edits.peek() {
                let old_edit_delta = old_edit.new.len() as i32 - old_edit.old.len() as i32;

                if old_edit.new.end < intermediate_start {
                    let mut old_edit = old_edit.clone();
                    old_edit.new.start = (old_edit.new.start as i32 + new_delta) as u32;
                    old_edit.new.end = (old_edit.new.end as i32 + new_delta) as u32;
                    new_edit.old.start = (new_edit.old.start as i32 - old_edit_delta) as u32;
                    new_edit.old.end = (new_edit.old.end as i32 - old_edit_delta) as u32;
                    composed.push(old_edit);
                } else if old_edit.new.start <= intermediate_end {
                    if old_edit.new.start < intermediate_start {
                        new_edit.new.start -= intermediate_start - old_edit.new.start;
                        new_edit.old.start -= intermediate_start - old_edit.new.start;
                    }
                    if old_edit.new.end > intermediate_end {
                        new_edit.new.end += old_edit.new.end - intermediate_end;
                        new_edit.old.end += old_edit.new.end - intermediate_end;
                        intermediate_end = old_edit.new.end;
                    }
                    new_edit.old.end = (new_edit.old.end as i32 - old_edit_delta) as u32;
                } else {
                    break;
                }

                old_delta += old_edit_delta;
                old_edits.next();
            }

            new_delta += new_edit_delta;
            composed.push(new_edit);
        }

        while let Some(mut old_edit) = old_edits.next() {
            old_edit.new.start = (old_edit.new.start as i32 + new_delta) as u32;
            old_edit.new.end = (old_edit.new.end as i32 + new_delta) as u32;
            composed.push(old_edit);
        }

        Patch(composed)
    }

    fn invert(&mut self) -> &mut Self {
        for edit in &mut self.0 {
            mem::swap(&mut edit.old, &mut edit.new);
        }
        self
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

    #[gpui::test(iterations = 1000, seed = 131)]
    fn test_random(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(3);

        let initial_chars = (0..rng.gen_range(0..=5))
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect::<Vec<_>>();
        let mut final_chars = initial_chars.clone();
        let mut patches = Vec::new();

        println!("initial chars: {:?}", initial_chars);
        for _ in 0..operations {
            let end = rng.gen_range(0..=final_chars.len());
            let start = rng.gen_range(0..=end);
            let mut len = rng.gen_range(0..=3);
            if start == end && len == 0 {
                len += 1;
            }
            let new_chars = (0..len)
                .map(|_| rng.gen_range(b'a'..=b'z') as char)
                .collect::<Vec<_>>();
            println!(
                "editing {:?}: {:?}",
                start..end,
                new_chars.iter().collect::<String>()
            );

            let patch = Patch(vec![Edit {
                old: start as u32..end as u32,
                new: start as u32..start as u32 + new_chars.len() as u32,
            }]);
            if patches.is_empty() || rng.gen() {
                println!("pushing singleton patch: {:?}", patch.0);
                patches.push(patch);
            } else {
                let patch = patches.pop().unwrap().compose(&patch);
                println!("composed patches: {:?}", patch.0);
                patches.push(patch);
            }
            final_chars.splice(start..end, new_chars);
        }

        println!("final chars: {:?}", final_chars);
        println!("final patches: {:?}", patches);

        let mut composed = Patch::default();
        for patch in patches {
            println!("composing patches {:?} and {:?}", composed, patch);
            composed = composed.compose(&patch);
            println!("composed {:?}", composed);
        }
        println!("composed edits: {:?}", composed);
        let mut chars = initial_chars.clone();
        for edit in composed.0 {
            chars.splice(
                edit.new.start as usize..edit.new.start as usize + edit.old.len(),
                final_chars[edit.new.start as usize..edit.new.end as usize]
                    .iter()
                    .copied(),
            );
        }

        assert_eq!(chars, final_chars);
    }

    #[track_caller]
    fn assert_patch_composition(old: Patch, new: Patch, composed: Patch) {
        let original = ('a'..'z').collect::<Vec<_>>();
        let inserted = ('A'..'Z').collect::<Vec<_>>();

        let mut expected = original.clone();
        apply_patch(&mut expected, &old, &inserted);
        apply_patch(&mut expected, &new, &inserted);

        let mut actual = original.clone();
        apply_patch(&mut actual, &composed, &expected);
        assert_eq!(
            actual.into_iter().collect::<String>(),
            expected.into_iter().collect::<String>(),
            "expected patch is incorrect"
        );

        assert_eq!(old.compose(&new), composed);
    }

    fn apply_patch(text: &mut Vec<char>, patch: &Patch, new_text: &[char]) {
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
