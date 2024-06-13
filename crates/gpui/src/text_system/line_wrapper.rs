use crate::{px, FontId, FontRun, Pixels, PlatformTextSystem};
use collections::HashMap;
use std::{iter, sync::Arc};

/// The GPUI line wrapper, used to wrap lines of text to a given width.
pub struct LineWrapper {
    platform_text_system: Arc<dyn PlatformTextSystem>,
    pub(crate) font_id: FontId,
    pub(crate) font_size: Pixels,
    cached_ascii_char_widths: [Option<Pixels>; 128],
    cached_other_char_widths: HashMap<char, Pixels>,
}

impl LineWrapper {
    /// The maximum indent that can be applied to a line.
    pub const MAX_INDENT: u32 = 256;

    pub(crate) fn new(
        font_id: FontId,
        font_size: Pixels,
        text_system: Arc<dyn PlatformTextSystem>,
    ) -> Self {
        Self {
            platform_text_system: text_system,
            font_id,
            font_size,
            cached_ascii_char_widths: [None; 128],
            cached_other_char_widths: HashMap::default(),
        }
    }

    /// Wrap a line of text to the given width with this wrapper's font and font size.
    pub fn wrap_line<'a>(
        &'a mut self,
        line: &'a str,
        wrap_width: Pixels,
    ) -> impl Iterator<Item = Boundary> + 'a {
        let mut width = px(0.);
        let mut first_non_whitespace_ix = None;
        let mut indent = None;
        let mut last_candidate_ix = 0;
        let mut last_candidate_width = px(0.);
        let mut last_wrap_ix = 0;
        let mut prev_c = '\0';
        let mut char_indices = line.char_indices();
        iter::from_fn(move || {
            for (ix, c) in char_indices.by_ref() {
                if c == '\n' {
                    continue;
                }

                if prev_c == ' ' && c != ' ' && first_non_whitespace_ix.is_some() {
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

                    if let Some(indent) = indent {
                        width += self.width_for_char(' ') * indent as f32;
                    }

                    return Some(Boundary::new(last_wrap_ix, indent.unwrap_or(0)));
                }
                prev_c = c;
            }

            None
        })
    }

    #[inline(always)]
    fn width_for_char(&mut self, c: char) -> Pixels {
        if (c as u32) < 128 {
            if let Some(cached_width) = self.cached_ascii_char_widths[c as usize] {
                cached_width
            } else {
                let width = self.compute_width_for_char(c);
                self.cached_ascii_char_widths[c as usize] = Some(width);
                width
            }
        } else if let Some(cached_width) = self.cached_other_char_widths.get(&c) {
            *cached_width
        } else {
            let width = self.compute_width_for_char(c);
            self.cached_other_char_widths.insert(c, width);
            width
        }
    }

    fn compute_width_for_char(&self, c: char) -> Pixels {
        let mut buffer = [0; 4];
        let buffer = c.encode_utf8(&mut buffer);
        self.platform_text_system
            .layout_line(
                buffer,
                self.font_size,
                &[FontRun {
                    len: buffer.len(),
                    font_id: self.font_id,
                }],
            )
            .width
    }
}

/// A boundary between two lines of text.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Boundary {
    /// The index of the last character in a line
    pub ix: usize,
    /// The indent of the next line.
    pub next_indent: u32,
}

impl Boundary {
    fn new(ix: usize, next_indent: u32) -> Self {
        Self { ix, next_indent }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{font, TestAppContext, TestDispatcher};
    #[cfg(target_os = "macos")]
    use crate::{TextRun, WindowTextSystem, WrapBoundary};
    use rand::prelude::*;

    #[test]
    fn test_wrap_line() {
        let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(0));
        let cx = TestAppContext::new(dispatcher, None);
        cx.text_system()
            .add_fonts(vec![std::fs::read(
                "../../assets/fonts/zed-mono/zed-mono-extended.ttf",
            )
            .unwrap()
            .into()])
            .unwrap();
        let id = cx.text_system().font_id(&font("Zed Mono")).unwrap();

        cx.update(|cx| {
            let text_system = cx.text_system().clone();
            let mut wrapper =
                LineWrapper::new(id, px(16.), text_system.platform_text_system.clone());
            assert_eq!(
                wrapper
                    .wrap_line("aa bbb cccc ddddd eeee", px(72.))
                    .collect::<Vec<_>>(),
                &[
                    Boundary::new(7, 0),
                    Boundary::new(12, 0),
                    Boundary::new(18, 0)
                ],
            );
            assert_eq!(
                wrapper
                    .wrap_line("aaa aaaaaaaaaaaaaaaaaa", px(72.0))
                    .collect::<Vec<_>>(),
                &[
                    Boundary::new(4, 0),
                    Boundary::new(11, 0),
                    Boundary::new(18, 0)
                ],
            );
            assert_eq!(
                wrapper
                    .wrap_line("     aaaaaaa", px(72.))
                    .collect::<Vec<_>>(),
                &[
                    Boundary::new(7, 5),
                    Boundary::new(9, 5),
                    Boundary::new(11, 5),
                ]
            );
            assert_eq!(
                wrapper
                    .wrap_line("                            ", px(72.))
                    .collect::<Vec<_>>(),
                &[
                    Boundary::new(7, 0),
                    Boundary::new(14, 0),
                    Boundary::new(21, 0)
                ]
            );
            assert_eq!(
                wrapper
                    .wrap_line("          aaaaaaaaaaaaaa", px(72.))
                    .collect::<Vec<_>>(),
                &[
                    Boundary::new(7, 0),
                    Boundary::new(14, 3),
                    Boundary::new(18, 3),
                    Boundary::new(22, 3),
                ]
            );
        });
    }

    // For compatibility with the test macro
    #[cfg(target_os = "macos")]
    use crate as gpui;

    // These seem to vary wildly based on the the text system.
    #[cfg(target_os = "macos")]
    #[crate::test]
    fn test_wrap_shaped_line(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let text_system = WindowTextSystem::new(cx.text_system().clone());

            let normal = TextRun {
                len: 0,
                font: font("Helvetica"),
                color: Default::default(),
                underline: Default::default(),
                strikethrough: None,
                background_color: None,
            };
            let bold = TextRun {
                len: 0,
                font: font("Helvetica").bold(),
                color: Default::default(),
                underline: Default::default(),
                strikethrough: None,
                background_color: None,
            };

            impl TextRun {
                fn with_len(&self, len: usize) -> Self {
                    let mut this = self.clone();
                    this.len = len;
                    this
                }
            }

            let text = "aa bbb cccc ddddd eeee".into();
            let lines = text_system
                .shape_text(
                    text,
                    px(16.),
                    &[
                        normal.with_len(4),
                        bold.with_len(5),
                        normal.with_len(6),
                        bold.with_len(1),
                        normal.with_len(7),
                    ],
                    Some(px(72.)),
                )
                .unwrap();

            assert_eq!(
                lines[0].layout.wrap_boundaries(),
                &[
                    WrapBoundary {
                        run_ix: 1,
                        glyph_ix: 3
                    },
                    WrapBoundary {
                        run_ix: 2,
                        glyph_ix: 3
                    },
                    WrapBoundary {
                        run_ix: 4,
                        glyph_ix: 2
                    }
                ],
            );
        });
    }
}
