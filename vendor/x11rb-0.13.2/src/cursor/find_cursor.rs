//! Find the right cursor file from a cursor name

// Based on libxcb-cursor's load_cursor.c which has:
//
//   Copyright © 2013 Michael Stapelberg
//   Copyright © 2002 Keith Packard
//
// and is licensed under MIT/X Consortium License

use std::fs::File;
use xcursor::CursorTheme;

static CORE_CURSORS: &[(&str, u16)] = &[
    ("X_cursor", 0),
    ("arrow", 1),
    ("based_arrow_down", 2),
    ("based_arrow_up", 3),
    ("boat", 4),
    ("bogosity", 5),
    ("bottom_left_corner", 6),
    ("bottom_right_corner", 7),
    ("bottom_side", 8),
    ("bottom_tee", 9),
    ("box_spiral", 10),
    ("center_ptr", 11),
    ("circle", 12),
    ("clock", 13),
    ("coffee_mug", 14),
    ("cross", 15),
    ("cross_reverse", 16),
    ("crosshair", 17),
    ("diamond_cross", 18),
    ("dot", 19),
    ("dotbox", 20),
    ("double_arrow", 21),
    ("draft_large", 22),
    ("draft_small", 23),
    ("draped_box", 24),
    ("exchange", 25),
    ("fleur", 26),
    ("gobbler", 27),
    ("gumby", 28),
    ("hand1", 29),
    ("hand2", 30),
    ("heart", 31),
    ("icon", 32),
    ("iron_cross", 33),
    ("left_ptr", 34),
    ("left_side", 35),
    ("left_tee", 36),
    ("leftbutton", 37),
    ("ll_angle", 38),
    ("lr_angle", 39),
    ("man", 40),
    ("middlebutton", 41),
    ("mouse", 42),
    ("pencil", 43),
    ("pirate", 44),
    ("plus", 45),
    ("question_arrow", 46),
    ("right_ptr", 47),
    ("right_side", 48),
    ("right_tee", 49),
    ("rightbutton", 50),
    ("rtl_logo", 51),
    ("sailboat", 52),
    ("sb_down_arrow", 53),
    ("sb_h_double_arrow", 54),
    ("sb_left_arrow", 55),
    ("sb_right_arrow", 56),
    ("sb_up_arrow", 57),
    ("sb_v_double_arrow", 58),
    ("shuttle", 59),
    ("sizing", 60),
    ("spider", 61),
    ("spraycan", 62),
    ("star", 63),
    ("target", 64),
    ("tcross", 65),
    ("top_left_arrow", 66),
    ("top_left_corner", 67),
    ("top_right_corner", 68),
    ("top_side", 69),
    ("top_tee", 70),
    ("trek", 71),
    ("ul_angle", 72),
    ("umbrella", 73),
    ("ur_angle", 74),
    ("watch", 75),
    ("xterm", 76),
];

/// Find a core cursor based on its name
///
/// This function checks a built-in list of known names.
fn cursor_shape_to_id(name: &str) -> Option<u16> {
    CORE_CURSORS
        .iter()
        .find(|&(name2, _)| name == *name2)
        .map(|&(_, id)| id)
}

#[cfg(test)]
mod test_cursor_shape_to_id {
    use super::cursor_shape_to_id;

    #[test]
    fn test_cursor_shape_to_id() {
        assert_eq!(cursor_shape_to_id("heart"), Some(31));
    }
}

/// The result of finding a cursor
#[derive(Debug)]
pub(crate) enum Cursor {
    /// The cursor is a core cursor that can be created with xproto's `CreateGlyphCursor`
    CoreChar(u16),

    /// A cursor file was opened
    File(File),
}

/// Find a cursor file based on the name of a cursor theme and the name of the cursor.
pub(crate) fn find_cursor(theme: &str, name: &str) -> Option<Cursor> {
    if theme == "core" {
        if let Some(id) = cursor_shape_to_id(name) {
            return Some(Cursor::CoreChar(id));
        }
    }
    if let Some(path) = CursorTheme::load(theme).load_icon(name) {
        if let Ok(file) = File::open(path) {
            return Some(Cursor::File(file));
        }
    }
    None
}
