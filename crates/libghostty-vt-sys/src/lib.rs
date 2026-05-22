#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
#![allow(rustdoc::all)]

mod bindings;

use std::ops::Deref;

pub use bindings::*;

/// Initialize a "sized" FFI object.
#[macro_export]
macro_rules! sized {
    ($ty:ty) => {{
        let mut t = <$ty as ::std::default::Default>::default();
        t.size = ::std::mem::size_of::<$ty>();
        t
    }};
}

impl<S> From<S> for GhosttyString
where
    S: Deref<Target = str>,
{
    fn from(value: S) -> Self {
        Self {
            ptr: value.as_ptr(),
            len: value.len(),
        }
    }
}

impl GhosttyString {
    /// # Safety
    ///
    /// The caller must uphold that the associated lifetime is valid
    /// with the given context behind the FFI string, and that it contains
    /// valid UTF-8 data.
    pub unsafe fn to_str<'a>(self) -> &'a str {
        // SAFETY: To be upheld by caller
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

/// Canonical list of exported `libghostty-vt` C functions represented by checked-in bindings.
pub const EXPORTED_API_SYMBOLS: &[&str] = &[
    "ghostty_build_info",
    "ghostty_cell_get",
    "ghostty_color_rgb_get",
    "ghostty_focus_encode",
    "ghostty_formatter_format_alloc",
    "ghostty_formatter_format_buf",
    "ghostty_formatter_free",
    "ghostty_formatter_terminal_new",
    "ghostty_grid_ref_cell",
    "ghostty_grid_ref_graphemes",
    "ghostty_grid_ref_row",
    "ghostty_grid_ref_style",
    "ghostty_key_encoder_encode",
    "ghostty_key_encoder_free",
    "ghostty_key_encoder_new",
    "ghostty_key_encoder_setopt",
    "ghostty_key_encoder_setopt_from_terminal",
    "ghostty_key_event_free",
    "ghostty_key_event_get_action",
    "ghostty_key_event_get_composing",
    "ghostty_key_event_get_consumed_mods",
    "ghostty_key_event_get_key",
    "ghostty_key_event_get_mods",
    "ghostty_key_event_get_unshifted_codepoint",
    "ghostty_key_event_get_utf8",
    "ghostty_key_event_new",
    "ghostty_key_event_set_action",
    "ghostty_key_event_set_composing",
    "ghostty_key_event_set_consumed_mods",
    "ghostty_key_event_set_key",
    "ghostty_key_event_set_mods",
    "ghostty_key_event_set_unshifted_codepoint",
    "ghostty_key_event_set_utf8",
    "ghostty_mode_report_encode",
    "ghostty_mouse_encoder_encode",
    "ghostty_mouse_encoder_free",
    "ghostty_mouse_encoder_new",
    "ghostty_mouse_encoder_reset",
    "ghostty_mouse_encoder_setopt",
    "ghostty_mouse_encoder_setopt_from_terminal",
    "ghostty_mouse_event_clear_button",
    "ghostty_mouse_event_free",
    "ghostty_mouse_event_get_action",
    "ghostty_mouse_event_get_button",
    "ghostty_mouse_event_get_mods",
    "ghostty_mouse_event_get_position",
    "ghostty_mouse_event_new",
    "ghostty_mouse_event_set_action",
    "ghostty_mouse_event_set_button",
    "ghostty_mouse_event_set_mods",
    "ghostty_mouse_event_set_position",
    "ghostty_osc_command_data",
    "ghostty_osc_command_type",
    "ghostty_osc_end",
    "ghostty_osc_free",
    "ghostty_osc_new",
    "ghostty_osc_next",
    "ghostty_osc_reset",
    "ghostty_paste_is_safe",
    "ghostty_render_state_colors_get",
    "ghostty_render_state_free",
    "ghostty_render_state_get",
    "ghostty_render_state_new",
    "ghostty_render_state_row_cells_free",
    "ghostty_render_state_row_cells_get",
    "ghostty_render_state_row_cells_new",
    "ghostty_render_state_row_cells_next",
    "ghostty_render_state_row_cells_select",
    "ghostty_render_state_row_get",
    "ghostty_render_state_row_iterator_free",
    "ghostty_render_state_row_iterator_new",
    "ghostty_render_state_row_iterator_next",
    "ghostty_render_state_row_set",
    "ghostty_render_state_set",
    "ghostty_render_state_update",
    "ghostty_row_get",
    "ghostty_sgr_attribute_tag",
    "ghostty_sgr_attribute_value",
    "ghostty_sgr_free",
    "ghostty_sgr_new",
    "ghostty_sgr_next",
    "ghostty_sgr_reset",
    "ghostty_sgr_set_params",
    "ghostty_sgr_unknown_full",
    "ghostty_sgr_unknown_partial",
    "ghostty_size_report_encode",
    "ghostty_style_default",
    "ghostty_style_is_default",
    "ghostty_terminal_free",
    "ghostty_terminal_get",
    "ghostty_terminal_grid_ref",
    "ghostty_terminal_mode_get",
    "ghostty_terminal_mode_set",
    "ghostty_terminal_new",
    "ghostty_terminal_reset",
    "ghostty_terminal_resize",
    "ghostty_terminal_scroll_viewport",
    "ghostty_terminal_vt_write",
];

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::EXPORTED_API_SYMBOLS;

    fn parse_binding_symbols(input: &str) -> BTreeSet<String> {
        input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if !line.starts_with("pub fn ghostty_") {
                    return None;
                }

                let start = "pub fn ".len();
                let rest = &line[start..];
                let end = rest.find('(')?;
                Some(rest[..end].to_owned())
            })
            .collect()
    }

    fn parse_header_symbols(input: &str) -> BTreeSet<String> {
        let mut symbols = BTreeSet::new();
        let mut statement = String::new();

        for line in input.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.is_empty() {
                continue;
            }

            // Skip static inline functions (they are inlined, not exported symbols)
            if trimmed.starts_with("static") {
                continue;
            }

            if !statement.is_empty() {
                statement.push(' ');
            }
            statement.push_str(trimmed);

            if !trimmed.ends_with(';') && !trimmed.ends_with('{') {
                continue;
            }

            if let Some(end) = statement.find('(') {
                let before_paren = &statement[..end];
                if let Some(candidate) = before_paren.split_whitespace().last() {
                    // Strip leading * for pointer-returning functions
                    let candidate = candidate.trim_start_matches('*');
                    if candidate.starts_with("ghostty_")
                        && candidate
                            .chars()
                            .all(|char| char.is_ascii_alphanumeric() || char == '_')
                    {
                        symbols.insert(candidate.to_owned());
                    }
                }
            }

            statement.clear();
        }

        symbols
    }

    #[test]
    fn exported_manifest_matches_bindings() {
        let from_bindings = parse_binding_symbols(include_str!("bindings.rs"));
        let from_manifest: BTreeSet<String> = EXPORTED_API_SYMBOLS
            .iter()
            .map(|symbol| (*symbol).to_owned())
            .collect();
        assert_eq!(from_manifest, from_bindings);
    }

    #[test]
    fn exported_manifest_is_sorted_and_unique() {
        let mut prev = "";
        for symbol in EXPORTED_API_SYMBOLS {
            assert!(
                *symbol > prev,
                "EXPORTED_API_SYMBOLS is not sorted or has duplicates: {prev:?} >= {symbol:?}"
            );
            prev = symbol;
        }
    }
}
