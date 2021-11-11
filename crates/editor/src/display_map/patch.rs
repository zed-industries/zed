use std::mem;

type Edit = buffer::Edit<u32>;

#[derive(Default, Debug)]
struct Patch(Vec<Edit>);

impl Patch {
    fn compose(&self, new: &Self) -> Patch {
        let mut composed = Vec::new();
        let mut new_edits = new.0.iter().cloned().peekable();
        let mut old_delta = 0;
        let mut new_delta = 0;

        for mut old_edit in self.0.iter().cloned() {
            let old_edit_new_start = old_edit.new.start;
            let old_edit_new_end = old_edit.new.end;
            let mut next_new_delta = new_delta;
            while let Some(mut new_edit) = new_edits.peek().cloned() {
                let new_edit_delta = new_edit.new.len() as i32 - new_edit.old.len() as i32;
                if new_edit.old.end < old_edit_new_start {
                    new_edit.old.start = (new_edit.old.start as i32 - old_delta) as u32;
                    new_edit.old.end = (new_edit.old.end as i32 - old_delta) as u32;
                    new_edits.next();
                    new_delta += new_edit_delta;
                    next_new_delta += new_edit_delta;
                    composed.push(new_edit);
                } else if new_edit.old.start <= old_edit_new_end {
                    if new_edit.old.start < old_edit_new_start {
                        old_edit.old.start -= old_edit_new_start - new_edit.old.start;
                        old_edit.new.start -= old_edit_new_start - new_edit.old.start;
                    }
                    if new_edit.old.end > old_edit_new_end {
                        old_edit.old.end += new_edit.old.end - old_edit_new_end;
                        old_edit.new.end += new_edit.old.end - old_edit_new_end;
                    }

                    old_edit.new.end = (old_edit.new.end as i32 + new_edit_delta) as u32;
                    new_edits.next();
                    next_new_delta += new_edit_delta;
                } else {
                    break;
                }
            }

            old_edit.new.start = (old_edit.new.start as i32 + new_delta) as u32;
            old_edit.new.end = (old_edit.new.end as i32 + new_delta) as u32;
            old_delta += old_edit.new.len() as i32 - old_edit.old.len() as i32;
            new_delta = next_new_delta;
            composed.push(old_edit);
        }
        composed.extend(new_edits.map(|mut new_edit| {
            new_edit.old.start = (new_edit.old.start as i32 - old_delta) as u32;
            new_edit.old.end = (new_edit.old.end as i32 - old_delta) as u32;
            new_edit
        }));

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
}
