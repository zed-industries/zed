use crate::{px, FontId, FontRun, Pixels, PlatformTextSystem, SharedString};
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

                if Self::is_word_char(c) {
                    if prev_c == ' ' && c != ' ' && first_non_whitespace_ix.is_some() {
                        last_candidate_ix = ix;
                        last_candidate_width = width;
                    }
                } else {
                    // CJK may not be space separated, e.g.: `Hello world你好世界`
                    if c != ' ' && first_non_whitespace_ix.is_some() {
                        last_candidate_ix = ix;
                        last_candidate_width = width;
                    }
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

    /// Truncate a line of text to the given width with this wrapper's font and font size.
    pub fn truncate_line(
        &mut self,
        line: SharedString,
        truncate_width: Pixels,
        ellipsis: Option<&str>,
    ) -> SharedString {
        let mut width = px(0.);
        let mut ellipsis_width = px(0.);
        if let Some(ellipsis) = ellipsis {
            for c in ellipsis.chars() {
                ellipsis_width += self.width_for_char(c);
            }
        }

        let mut char_indices = line.char_indices();
        let mut truncate_ix = 0;
        for (ix, c) in char_indices {
            if width + ellipsis_width <= truncate_width {
                truncate_ix = ix;
            }

            let char_width = self.width_for_char(c);
            width += char_width;

            if width.floor() > truncate_width {
                return SharedString::from(format!(
                    "{}{}",
                    &line[..truncate_ix],
                    ellipsis.unwrap_or("")
                ));
            }
        }

        line.clone()
    }

    pub(crate) fn is_word_char(c: char) -> bool {
        // ASCII alphanumeric characters, for English, numbers: `Hello123`, etc.
        c.is_ascii_alphanumeric() ||
        // Latin script in Unicode for French, German, Spanish, etc.
        // Latin-1 Supplement
        // https://en.wikipedia.org/wiki/Latin-1_Supplement
        matches!(c, '\u{00C0}'..='\u{00FF}') ||
        // Latin Extended-A
        // https://en.wikipedia.org/wiki/Latin_Extended-A
        matches!(c, '\u{0100}'..='\u{017F}') ||
        // Latin Extended-B
        // https://en.wikipedia.org/wiki/Latin_Extended-B
        matches!(c, '\u{0180}'..='\u{024F}') ||
        // Cyrillic for Russian, Ukrainian, etc.
        // https://en.wikipedia.org/wiki/Cyrillic_script_in_Unicode
        matches!(c, '\u{0400}'..='\u{04FF}') ||
        // Some other known special characters that should be treated as word characters,
        // e.g. `a-b`, `var_name`, `I'm`, '@mention`, `#hashtag`, `100%`, `3.1415`, `2^3`, `a~b`, etc.
        matches!(c, '-' | '_' | '.' | '\'' | '$' | '%' | '@' | '#' | '^' | '~') ||
        // Characters that used in URL, e.g. `https://github.com/zed-industries/zed?a=1&b=2` for better wrapping a long URL.
        matches!(c,  '/' | ':' | '?' | '&' | '=') ||
        // `⋯` character is special used in Zed, to keep this at the end of the line.
        matches!(c, '⋯')
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

    fn build_wrapper() -> LineWrapper {
        let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(0));
        let cx = TestAppContext::new(dispatcher, None);
        cx.text_system()
            .add_fonts(vec![std::fs::read(
                "../../assets/fonts/plex-mono/ZedPlexMono-Regular.ttf",
            )
            .unwrap()
            .into()])
            .unwrap();
        let id = cx.text_system().font_id(&font("Zed Plex Mono")).unwrap();
        LineWrapper::new(id, px(16.), cx.text_system().platform_text_system.clone())
    }

    #[test]
    fn test_wrap_line() {
        let mut wrapper = build_wrapper();

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
    }

    #[test]
    fn test_truncate_line() {
        let mut wrapper = build_wrapper();

        assert_eq!(
            wrapper.truncate_line("aa bbb cccc ddddd eeee ffff gggg".into(), px(220.), None),
            "aa bbb cccc ddddd eeee"
        );
        assert_eq!(
            wrapper.truncate_line(
                "aa bbb cccc ddddd eeee ffff gggg".into(),
                px(220.),
                Some("…")
            ),
            "aa bbb cccc ddddd eee…"
        );
        assert_eq!(
            wrapper.truncate_line(
                "aa bbb cccc ddddd eeee ffff gggg".into(),
                px(220.),
                Some("......")
            ),
            "aa bbb cccc dddd......"
        );
    }

    #[test]
    fn test_is_word_char() {
        #[track_caller]
        fn assert_word(word: &str) {
            for c in word.chars() {
                assert!(LineWrapper::is_word_char(c), "assertion failed for '{}'", c);
            }
        }

        #[track_caller]
        fn assert_not_word(word: &str) {
            let found = word.chars().any(|c| !LineWrapper::is_word_char(c));
            assert!(found, "assertion failed for '{}'", word);
        }

        assert_word("Hello123");
        assert_word("non-English");
        assert_word("var_name");
        assert_word("123456");
        assert_word("3.1415");
        assert_word("10^2");
        assert_word("1~2");
        assert_word("100%");
        assert_word("@mention");
        assert_word("#hashtag");
        assert_word("$variable");
        assert_word("more⋯");

        // Space
        assert_not_word("foo bar");

        // URL case
        assert_word("https://github.com/zed-industries/zed/");
        assert_word("github.com");
        assert_word("a=1&b=2");

        // Latin-1 Supplement
        assert_word("ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏ");
        // Latin Extended-A
        assert_word("ĀāĂăĄąĆćĈĉĊċČčĎď");
        // Latin Extended-B
        assert_word("ƀƁƂƃƄƅƆƇƈƉƊƋƌƍƎƏ");
        // Cyrillic
        assert_word("АБВГДЕЖЗИЙКЛМНОП");

        // non-word characters
        assert_not_word("你好");
        assert_not_word("안녕하세요");
        assert_not_word("こんにちは");
        assert_not_word("😀😁😂");
        assert_not_word("()[]{}<>");
    }

    // For compatibility with the test macro
    #[cfg(target_os = "macos")]
    use crate as gpui;

    // These seem to vary wildly based on the text system.
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
