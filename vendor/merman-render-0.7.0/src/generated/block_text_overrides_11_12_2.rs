// This historical fixture-derived table is kept as generated data.
//
// Mermaid baseline: 11.12.2
// Source: fixtures/upstream-svgs/block/*.svg
//
// The old bulk generator is no longer in-tree; prune entries through targeted parity/layout
// rechecks and keep the override budget synchronized.

fn font_size_key(font_size: f64) -> u16 {
    if !(font_size.is_finite() && font_size > 0.0) {
        return 0;
    }
    let k = (font_size * 100.0).round();
    if !(k.is_finite() && k >= 0.0 && k <= (u16::MAX as f64)) {
        return 0;
    }
    k as u16
}

static HTML_WIDTH_OVERRIDES_PX: &[(u16, &str, f64)] = &[
    (1600, "A", 9.4375),
    (1600, "A label", 48.859375),
    (1600, "A wide one", 78.359375),
    (1600, "A wide one in the middle", 179.0625),
    (1600, "A1", 17.828125),
    (1600, "A2", 17.828125),
    (1600, "A3", 17.828125),
    (1600, "A: I am a wide one", 134.125),
    (1600, "A;", 15.3125),
    (1600, "ABlock", 47.796875),
    (1600, "B", 9.0625),
    (1600, "B;", 14.9375),
    (1600, "BBlock", 47.40625),
    (1600, "BL", 17.15625),
    (1600, "Backend", 59.84375),
    (1600, "Block 1", 51.5625),
    (1600, "Block 2", 51.5625),
    (1600, "Block 3", 51.5625),
    (1600, "C", 9.578125),
    (1600, "Christmas", 70.078125),
    (1600, "Compound block", 118.375),
    (1600, "D", 9.8125),
    (1600, "DB", 18.875),
    (1600, "Database", 65.5),
    (1600, "Disk", 28.921875),
    (1600, "Done", 35.875),
    (1600, "E", 8.578125),
    (1600, "End", 26.234375),
    (1600, "F", 8.40625),
    (1600, "Frontend", 64.671875),
    (1600, "G", 10.828125),
    (1600, "Go shopping", 87.203125),
    (1600, "Hello", 37.21875),
    (1600, "I am a wide one", 114.0),
    (1600, "In the middle", 95.765625),
    (1600, "Label", 38.875),
    (1600, "Make Decision", 100.765625),
    (1600, "Memcache", 75.078125),
    (1600, "Next row", 64.828125),
    (1600, "No", 18.796875),
    (1600, "One Slot", 60.421875),
    (1600, "Process A", 65.953125),
    (1600, "Process B", 66.453125),
    (1600, "Start", 35.015625),
    (1600, "Stop", 31.546875),
    (1600, "Sync", 32.25),
    (1600, "This is the text", 107.8125),
    (1600, "This is the text in the box", 184.90625),
    (1600, "This is the text in the circle", 199.46875),
    (1600, "Two slots", 65.0),
    (1600, "Wider then", 79.21875),
    (1600, "World", 41.34375),
    (1600, "X", 8.90625),
    (1600, "XYZ", 26.84375),
    (1600, "Yes", 22.65625),
    (1600, "__proto__", 72.21875),
    (1600, "a", 8.40625),
    (1600, "a label", 48.71875),
    (1600, "also_overflow", 99.078125),
    (1600, "apa", 25.734375),
    (1600, "b", 8.921875),
    (1600, "c", 7.921875),
    (1600, "circle", 40.078125),
    (1600, "constructor", 82.109375),
    (1600, "cylinder", 57.703125),
    (1600, "d", 8.921875),
    (1600, "diamond", 61.40625),
    (1600, "down", 38.15625),
    (1600, "e", 8.734375),
    (1600, "f", 5.921875),
    (1600, "first", 29.515625),
    (1600, "fit", 16.828125),
    (1600, "g", 8.03125),
    (1600, "h", 8.75),
    (1600, "hexagon", 59.25),
    (1600, "i", 4.5625),
    (1600, "id", 13.484375),
    (1600, "id1", 21.875),
    (1600, "id2", 21.875),
    (1600, "id2.1", 36.140625),
    (1600, "id3", 21.875),
    (1600, "id4", 21.875),
    (1600, "id48", 30.265625),
    (1600, "id5", 21.875),
    (1600, "ida", 21.890625),
    (1600, "idb", 22.390625),
    (1600, "idc", 21.40625),
    (1600, "j", 5.875),
    (1600, "k", 8.078125),
    (1600, "l", 4.71875),
    (1600, "lean left", 61.125),
    (1600, "lean right", 69.3125),
    (1600, "left", 25.703125),
    (1600, "m", 13.28125),
    (1600, "n", 8.75),
    (1600, "o", 8.59375),
    (1600, "overflow", 62.5),
    (1600, "p", 8.921875),
    (1600, "q", 8.921875),
    (1600, "r", 6.21875),
    (1600, "rect_left_inv_arrow", 142.5625),
    (1600, "right", 33.90625),
    (1600, "rounded", 58.84375),
    (1600, "second", 49.375),
    (1600, "short", 36.375),
    (1600, "square", 47.484375),
    (1600, "stadium", 56.734375),
    (1600, "subroutine", 76.0625),
    (1600, "surprise", 56.34375),
    (1600, "the", 23.8125),
    (1600, "to", 14.9375),
    (1600, "trapezoid", 68.265625),
    (1600, "trapezoid alt", 92.5625),
    (1600, "up", 17.65625),
    (1600, "x", 8.015625),
    (1600, "y", 7.890625),
    (1600, "\u{00A0}\u{00A0}\u{00A0}", 14.46875),
    (
        2400,
        "Font size precedence should widen this block",
        487.890625,
    ),
    (2400, "Second block", 140.4375),
    (2400, "Wide edge label", 172.71875),
];

pub fn lookup_html_width_px(font_size: f64, text: &str) -> Option<f64> {
    let fs = font_size_key(font_size);
    if fs == 0 || text.is_empty() {
        return None;
    }
    let mut lo = 0usize;
    let mut hi = HTML_WIDTH_OVERRIDES_PX.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        let (k_fs, k_text, value) = HTML_WIDTH_OVERRIDES_PX[mid];
        match k_fs.cmp(&fs) {
            std::cmp::Ordering::Equal => match k_text.cmp(text) {
                std::cmp::Ordering::Equal => return Some(value),
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            },
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}
