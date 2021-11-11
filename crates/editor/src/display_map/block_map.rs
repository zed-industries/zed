use std::cmp;

use super::wrap_map::{Edit as WrapEdit, Snapshot as WrapSnapshot};
use buffer::Bias;
use parking_lot::Mutex;
use sum_tree::SumTree;

struct BlockMap {
    transforms: Mutex<SumTree<Transform>>,
}

struct BlockMapWriter<'a>(&'a mut BlockMap);

struct BlockSnapshot {}

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
        BlockSnapshot {}
    }

    fn write(&mut self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) -> BlockMapWriter {
        self.sync(wrap_snapshot, edits);
        BlockMapWriter(self)
    }

    fn sync(&self, wrap_snapshot: WrapSnapshot, edits: Vec<WrapEdit>) {
        let transforms = self.transforms.lock();
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
                        edit.old.end = cmp::max(next_edit.old.end, edit.old.end);
                        edit.new.end += (edit.new.len() as i32 - edit.old.len() as i32) as u32;
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
