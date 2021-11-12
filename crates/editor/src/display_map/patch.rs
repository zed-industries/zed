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
            eprintln!("edit {:?}", new_edit);

            let new_edit_delta = new_edit.new.len() as i32 - new_edit.old.len() as i32;

            if let Some(last_edit) = composed.last_mut() {
                if intermediate_end >= new_edit.old.start {
                    if new_edit.old.end > intermediate_end {
                        last_edit.old.end += new_edit.old.end - intermediate_end;
                        last_edit.new.end += new_edit.old.end - intermediate_end;
                        intermediate_end = new_edit.old.end;
                    }
                    last_edit.new.end = (last_edit.new.end as i32 + new_edit_delta) as u32;
                    new_delta += new_edit_delta;
                    eprintln!("  merged {:?}", &composed);
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
                    eprintln!("  pushed preceding {:?}", &composed);
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
                    eprintln!("  expanded w/ intersecting {:?} - {:?}", old_edit, new_edit);
                    new_edit.old.end = (new_edit.old.end as i32 - old_edit_delta) as u32;
                } else {
                    break;
                }

                old_delta += old_edit_delta;
                old_edits.next();
            }

            new_delta += new_edit_delta;
            composed.push(new_edit);
            eprintln!("  pushing {:?}", &composed);
        }

        while let Some(mut old_edit) = old_edits.next() {
            let old_edit_delta = old_edit.new.len() as i32 - old_edit.old.len() as i32;

            if let Some(last_edit) = composed.last_mut() {
                if intermediate_end >= old_edit.new.start {
                    if old_edit.new.end > intermediate_end {
                        last_edit.old.end += old_edit.new.end - intermediate_end;
                        last_edit.new.end += old_edit.new.end - intermediate_end;
                        intermediate_end = old_edit.new.end;
                    }
                    last_edit.old.end = (last_edit.old.end as i32 - old_edit_delta) as u32;
                    eprintln!("  merged {:?}", &composed);
                    continue;
                }
            }

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

    #[gpui::test(iterations = 1000)]
    fn test_random(mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(2);

        let initial_chars = (0..rng.gen_range(0..=10))
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect::<Vec<_>>();
        println!("initial chars: {:?}", initial_chars);

        // Generate two sequential patches
        let mut patches = Vec::new();
        let mut expected_chars = initial_chars.clone();
        for i in 0..2 {
            println!("patch {}:", i);

            let mut delta = 0i32;
            let mut last_edit_end = 0;
            let mut edits = Vec::new();
            for _ in 0..operations {
                if last_edit_end >= expected_chars.len() {
                    break;
                }

                let end = rng.gen_range(last_edit_end..=expected_chars.len());
                let start = rng.gen_range(last_edit_end..=end);
                let old_len = end - start;

                let mut new_len = rng.gen_range(0..=3);
                if start == end && new_len == 0 {
                    new_len += 1;
                }

                last_edit_end = start + new_len + 1;

                let new_chars = (0..new_len)
                    .map(|_| rng.gen_range(b'A'..=b'Z') as char)
                    .collect::<Vec<_>>();
                println!(
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

        println!("old patch: {:?}", &patches[0]);
        println!("new patch: {:?}", &patches[1]);
        println!("initial chars: {:?}", initial_chars);
        println!("final chars: {:?}", expected_chars);

        // Compose the patches, and verify that it has the same effect as applying the
        // two patches separately.
        let composed = patches[0].compose(&patches[1]);
        println!("composed patch: {:?}", &composed);

        let mut actual_chars = initial_chars.clone();
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
