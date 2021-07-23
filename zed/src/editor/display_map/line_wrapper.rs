use crate::Settings;
use gpui::{fonts::FontId, FontCache, FontSystem};
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

pub struct LineWrapper {
    font_system: Arc<dyn FontSystem>,
    font_cache: Arc<FontCache>,
    font_id: FontId,
    font_size: f32,
    cached_ascii_char_widths: Mutex<[f32; 128]>,
    cached_other_char_widths: Mutex<HashMap<char, f32>>,
}

impl LineWrapper {
    pub fn new(
        font_system: Arc<dyn FontSystem>,
        font_cache: Arc<FontCache>,
        settings: Settings,
    ) -> Self {
        let font_id = font_cache
            .select_font(settings.buffer_font_family, &Default::default())
            .unwrap();
        let font_size = settings.buffer_font_size;
        Self {
            font_cache,
            font_system,
            font_id,
            font_size,
            cached_ascii_char_widths: Mutex::new([f32::NAN; 128]),
            cached_other_char_widths: Mutex::new(HashMap::new()),
        }
    }

    pub fn wrap_line_with_shaping(&self, line: &str, wrap_width: f32) -> Vec<usize> {
        self.font_system
            .wrap_line(line, self.font_id, self.font_size, wrap_width)
    }

    pub fn wrap_line_without_shaping(&self, line: &str, wrap_width: f32) -> Vec<usize> {
        let mut width = 0.0;
        let mut boundaries = Vec::new();
        let mut last_boundary_ix = 0;
        let mut last_boundary_width = 0.0;
        let mut prev_c = '\0';
        for (ix, c) in line.char_indices() {
            if c == '\n' {
                break;
            }

            if self.is_boundary(prev_c, c) {
                last_boundary_ix = ix;
                last_boundary_width = width;
            }

            let char_width = self.width_for_char(c);
            width += char_width;
            if width > wrap_width && ix > *boundaries.last().unwrap_or(&0) {
                if last_boundary_ix > 0 {
                    boundaries.push(last_boundary_ix);
                    width -= last_boundary_width;
                    last_boundary_ix = 0;
                } else {
                    boundaries.push(ix);
                    width = char_width;
                }
            }
            prev_c = c;
        }
        boundaries
    }

    fn is_boundary(&self, prev: char, next: char) -> bool {
        if prev == ' ' || next == ' ' {
            return true;
        }
        false
    }

    fn width_for_char(&self, c: char) -> f32 {
        if (c as u32) < 128 {
            let mut cached_ascii_char_widths = self.cached_ascii_char_widths.lock();
            let mut width = cached_ascii_char_widths[c as usize];
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                cached_ascii_char_widths[c as usize] = width;
            }
            width
        } else {
            let mut cached_other_char_widths = self.cached_other_char_widths.lock();
            let mut width = cached_other_char_widths
                .get(&c)
                .copied()
                .unwrap_or(f32::NAN);
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                cached_other_char_widths.insert(c, width);
            }
            width
        }
    }

    fn compute_width_for_char(&self, c: char) -> f32 {
        self.font_system
            .layout_line(
                &c.to_string(),
                self.font_size,
                &[(1, self.font_id, Default::default())],
            )
            .width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_line_wrapper(cx: &mut gpui::MutableAppContext) {
        let font_cache = cx.font_cache().clone();
        let font_system = cx.platform().fonts();
        let settings = Settings {
            tab_size: 4,
            buffer_font_family: font_cache.load_family(&["Courier"]).unwrap(),
            buffer_font_size: 16.0,
            ..Settings::new(&font_cache).unwrap()
        };

        let wrapper = LineWrapper::new(font_system, font_cache, settings);

        assert_eq!(
            wrapper.wrap_line_with_shaping("aa bbb cccc ddddd eeee", 72.0),
            &[7, 12, 18],
        );
        assert_eq!(
            wrapper.wrap_line_without_shaping("aa bbb cccc ddddd eeee", 72.0),
            &[7, 12, 18],
        );

        assert_eq!(
            wrapper.wrap_line_with_shaping("aaa aaaaaaaaaaaaaaaaaa", 72.0),
            &[4, 11, 18],
        );
        assert_eq!(
            wrapper.wrap_line_without_shaping("aaa aaaaaaaaaaaaaaaaaa", 72.0),
            &[4, 11, 18],
        );
    }
}
