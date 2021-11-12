use std::cmp;

use super::wrap_map::{Edit as WrapEdit, Snapshot as WrapSnapshot};
use buffer::Bias;
use parking_lot::Mutex;
use sum_tree::SumTree;

struct BlockMap {
    transforms: Mutex<SumTree<Transform>>,
}

struct BlockMapWriter<'a>(&'a mut BlockMap);

struct BlockSnapshot {
    transforms: SumTree<Transform>,
}

#[derive(Clone)]
struct Transform {
    summary: TransformSummary,
}

#[derive(Copy, Clone, Debug, Default)]
struct TransformSummary {
    input_rows: u32,
    output_rows: u32,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct InputRow(u32);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OutputRow(u32);

impl BlockMap {
    fn new(wrap_snapshot: WrapSnapshot) -> Self {
        Self {
            transforms: Mutex::new(SumTree::from_item(
                Transform::isomorphic(wrap_snapshot.max_point().row() + 1),
                &(),
            )),
        }
    }

    fn read(&self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) -> BlockSnapshot {
        self.sync(wrap_snapshot, edits);
        BlockSnapshot {
            transforms: self.transforms.lock().clone(),
        }
    }

    fn write(&mut self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) -> BlockMapWriter {
        self.sync(wrap_snapshot, edits);
        BlockMapWriter(self)
    }

    fn sync(&self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) {
        let mut transforms = self.transforms.lock();
        let mut new_transforms = SumTree::new();
        let mut cursor = transforms.cursor::<InputRow>();
        let mut edits = edits.into_iter().peekable();
        while let Some(mut edit) = edits.next() {
            new_transforms.push_tree(
                cursor.slice(&InputRow(edit.old.start), Bias::Left, &()),
                &(),
            );

            let transform_start = cursor.start().0;
            edit.new.start -= edit.old.start - transform_start;
            edit.old.start = transform_start;

            loop {
                if edit.old.end > cursor.start().0 {
                    cursor.seek(&InputRow(edit.old.end), Bias::Left, &());
                    cursor.next(&());
                    let transform_end = cursor.start().0;
                    edit.new.end += transform_end - edit.old.end;
                    edit.old.end = transform_end;
                }

                if let Some(next_edit) = edits.peek() {
                    if edit.old.end >= next_edit.old.start {
                        let delta = next_edit.new.len() as i32 - next_edit.old.len() as i32;
                        edit.old.end = cmp::max(next_edit.old.end, edit.old.end);
                        edit.new.end = (edit.new.end as i32 + delta) as u32;
                        edits.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // TODO: process injections

            let new_transforms_end = new_transforms.summary().input_rows;
            if new_transforms_end < edit.new.end {
                new_transforms.push(
                    Transform::isomorphic(edit.new.end - new_transforms_end),
                    &(),
                );
            }
        }
        new_transforms.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        *transforms = new_transforms;
    }
}

impl Transform {
    fn isomorphic(rows: u32) -> Self {
        Self {
            summary: TransformSummary {
                input_rows: rows,
                output_rows: rows,
            },
        }
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self) -> Self::Summary {
        self.summary
    }
}

impl sum_tree::Summary for TransformSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.input_rows += summary.input_rows;
        self.output_rows += summary.output_rows;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for InputRow {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.input_rows;
    }
}

impl<'a> sum_tree::Dimension<'a, TransformSummary> for OutputRow {
    fn add_summary(&mut self, summary: &'a TransformSummary, _: &()) {
        self.0 += summary.output_rows;
    }
}

#[cfg(test)]
mod tests {
    use super::BlockMap;
    use crate::display_map::{fold_map::FoldMap, tab_map::TabMap, wrap_map::WrapMap};
    use buffer::RandomCharIter;
    use language::Buffer;
    use rand::prelude::*;
    use std::env;

    #[gpui::test(iterations = 100)]
    fn test_random(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
        let operations = env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let wrap_width = Some(rng.gen_range(0.0..=1000.0));
        let tab_size = 1;
        let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
        let font_id = cx
            .font_cache()
            .select_font(family_id, &Default::default())
            .unwrap();
        let font_size = 14.0;

        log::info!("Tab size: {}", tab_size);
        log::info!("Wrap width: {:?}", wrap_width);

        let buffer = cx.add_model(|cx| {
            let len = rng.gen_range(0..10);
            let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
            Buffer::new(0, text, cx)
        });
        let (fold_map, folds_snapshot) = FoldMap::new(buffer.clone(), cx);
        let (tab_map, tabs_snapshot) = TabMap::new(folds_snapshot.clone(), tab_size);
        let (wrap_map, wraps_snapshot) =
            WrapMap::new(tabs_snapshot, font_id, font_size, wrap_width, cx);
        let block_map = BlockMap::new(wraps_snapshot);

        for _ in 0..operations {
            match rng.gen_range(0..=100) {
                0..=19 => {
                    let wrap_width = if rng.gen_bool(0.2) {
                        None
                    } else {
                        Some(rng.gen_range(0.0..=1000.0))
                    };
                    log::info!("Setting wrap width to {:?}", wrap_width);
                    wrap_map.update(cx, |map, cx| map.set_wrap_width(wrap_width, cx));
                }
                _ => {
                    buffer.update(cx, |buffer, _| buffer.randomly_edit(&mut rng, 5));
                }
            }

            let (folds_snapshot, fold_edits) = fold_map.read(cx);
            let (tabs_snapshot, tab_edits) = tab_map.sync(folds_snapshot, fold_edits);
            let (wraps_snapshot, wrap_edits) = wrap_map.update(cx, |wrap_map, cx| {
                wrap_map.sync(tabs_snapshot, tab_edits, cx)
            });
            let blocks_snapshot = block_map.read(wraps_snapshot.clone(), wrap_edits);
            assert_eq!(
                blocks_snapshot.transforms.summary().input_rows,
                wraps_snapshot.max_point().row() + 1
            );
        }
    }
}
