//! # autoclose_tags

use multi_buffer::MultiBufferSnapshot;
use rope::Point;

pub fn predict_at(
    snapshot: &MultiBufferSnapshot,
    cursor_pos: Point,
    text: &str,
) -> Option<Prediction> {
    let excerpt = snapshot.excerpt_containing(cursor_pos..cursor_pos).unwrap();
    let tree = excerpt
        .buffer()
        .syntax_layer_at(excerpt.start_offset())
        .unwrap();
    _ = tree;
    None
}

pub struct Prediction {
    tag_name: String,
}
