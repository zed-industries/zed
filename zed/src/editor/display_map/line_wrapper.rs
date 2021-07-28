use crate::Settings;
use gpui::{fonts::FontId, FontCache, FontSystem};
use std::{
    cell::RefCell,
    collections::HashMap,
    iter,
    ops::{Deref, DerefMut},
    sync::Arc,
};

thread_local! {
    static WRAPPERS: RefCell<Vec<LineWrapper>> = Default::default();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Boundary {
    pub ix: usize,
    pub next_indent: u32,
}

impl Boundary {
    fn new(ix: usize, next_indent: u32) -> Self {
        Self { ix, next_indent }
    }
}

pub struct LineWrapper {
    font_system: Arc<dyn FontSystem>,
    font_id: FontId,
    font_size: f32,
    cached_ascii_char_widths: [f32; 128],
    cached_other_char_widths: HashMap<char, f32>,
}

impl LineWrapper {
    pub const MAX_INDENT: u32 = 256;

    pub fn thread_local(
        font_system: Arc<dyn FontSystem>,
        font_cache: &FontCache,
        settings: Settings,
    ) -> LineWrapperHandle {
        let wrapper =
            if let Some(mut wrapper) = WRAPPERS.with(|wrappers| wrappers.borrow_mut().pop()) {
                let font_id = font_cache
                    .select_font(settings.buffer_font_family, &Default::default())
                    .unwrap();
                let font_size = settings.buffer_font_size;
                if wrapper.font_id != font_id || wrapper.font_size != font_size {
                    wrapper.cached_ascii_char_widths = [f32::NAN; 128];
                    wrapper.cached_other_char_widths.clear();
                }
                wrapper
            } else {
                LineWrapper::new(font_system, font_cache, settings)
            };
        LineWrapperHandle(Some(wrapper))
    }

    pub fn new(
        font_system: Arc<dyn FontSystem>,
        font_cache: &FontCache,
        settings: Settings,
    ) -> Self {
        let font_id = font_cache
            .select_font(settings.buffer_font_family, &Default::default())
            .unwrap();
        let font_size = settings.buffer_font_size;
        Self {
            font_system,
            font_id,
            font_size,
            cached_ascii_char_widths: [f32::NAN; 128],
            cached_other_char_widths: HashMap::new(),
        }
    }

    pub fn wrap_line<'a>(
        &'a mut self,
        line: &'a str,
        wrap_width: f32,
    ) -> impl Iterator<Item = Boundary> + 'a {
        let mut width = 0.0;
        let mut first_non_whitespace_ix = None;
        let mut indent = None;
        let mut last_candidate_ix = 0;
        let mut last_candidate_width = 0.0;
        let mut last_wrap_ix = 0;
        let mut prev_c = '\0';
        let mut char_indices = line.char_indices();
        iter::from_fn(move || {
            while let Some((ix, c)) = char_indices.next() {
                if c == '\n' {
                    continue;
                }

                if self.is_boundary(prev_c, c) && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = ix;
                    last_candidate_width = width;
                }

                if c != ' ' && first_non_whitespace_ix.is_none() {
                    first_non_whitespace_ix = Some(ix);
                }

                let char_width = self.width_for_char(c);
                width += char_width;
                if width > wrap_width && ix > last_wrap_ix {
                    if let (None, Some(first_non_whitespace_ix)) = (indent, first_non_whitespace_ix)
                    {
                        indent = Some(
                            Self::MAX_INDENT.min((first_non_whitespace_ix - last_wrap_ix) as u32),
                        );
                    }

                    if last_candidate_ix > 0 {
                        last_wrap_ix = last_candidate_ix;
                        width -= last_candidate_width;
                        last_candidate_ix = 0;
                    } else {
                        last_wrap_ix = ix;
                        width = char_width;
                    }

                    let indent_width =
                        indent.map(|indent| indent as f32 * self.width_for_char(' '));
                    width += indent_width.unwrap_or(0.);

                    return Some(Boundary::new(last_wrap_ix, indent.unwrap_or(0)));
                }
                prev_c = c;
            }

            None
        })
    }

    fn is_boundary(&self, prev: char, next: char) -> bool {
        (prev == ' ') && (next != ' ')
    }

    #[inline(always)]
    fn width_for_char(&mut self, c: char) -> f32 {
        if (c as u32) < 128 {
            let mut width = self.cached_ascii_char_widths[c as usize];
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                self.cached_ascii_char_widths[c as usize] = width;
            }
            width
        } else {
            let mut width = self
                .cached_other_char_widths
                .get(&c)
                .copied()
                .unwrap_or(f32::NAN);
            if width.is_nan() {
                width = self.compute_width_for_char(c);
                self.cached_other_char_widths.insert(c, width);
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

pub struct LineWrapperHandle(Option<LineWrapper>);

impl Drop for LineWrapperHandle {
    fn drop(&mut self) {
        let wrapper = self.0.take().unwrap();
        WRAPPERS.with(|wrappers| wrappers.borrow_mut().push(wrapper))
    }
}

impl Deref for LineWrapperHandle {
    type Target = LineWrapper;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for LineWrapperHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
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

        let mut wrapper = LineWrapper::new(font_system, &font_cache, settings);
        assert_eq!(
            wrapper
                .wrap_line("aa bbb cccc ddddd eeee", 72.0)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(12, 0),
                Boundary::new(18, 0)
            ],
        );
        assert_eq!(
            wrapper
                .wrap_line("aaa aaaaaaaaaaaaaaaaaa", 72.0)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(4, 0),
                Boundary::new(11, 0),
                Boundary::new(18, 0)
            ],
        );
        assert_eq!(
            wrapper.wrap_line("     aaaaaaa", 72.).collect::<Vec<_>>(),
            &[
                Boundary::new(7, 5),
                Boundary::new(9, 5),
                Boundary::new(11, 5),
            ]
        );
        assert_eq!(
            wrapper
                .wrap_line("                            ", 72.)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(14, 0),
                Boundary::new(21, 0)
            ]
        );
        assert_eq!(
            wrapper
                .wrap_line("          aaaaaaaaaaaaaa", 72.)
                .collect::<Vec<_>>(),
            &[
                Boundary::new(7, 0),
                Boundary::new(14, 3),
                Boundary::new(18, 3),
                Boundary::new(22, 3),
            ]
        );
    }
}
