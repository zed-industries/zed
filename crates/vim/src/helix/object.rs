use std::{
    error::Error,
    fmt::{self, Display},
    ops::Range,
};

use editor::{DisplayPoint, display_map::DisplaySnapshot, movement};
use text::Selection;

use crate::{
    helix::boundary::{FuzzyBoundary, ImmediateBoundary, NearestPair},
    object::Object as VimObject,
};

/// A text object from helix or an extra one
pub trait HelixTextObject {
    fn range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>>;

    fn next_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>>;

    fn previous_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>>;
}

impl VimObject {
    /// Returns the range of the object the cursor is over.
    /// Follows helix convention.
    pub fn helix_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Result<Option<Range<DisplayPoint>>, VimToHelixError> {
        let relative_to = if self == VimObject::AnyPair && !selection.is_empty() {
            selection.range()
        } else {
            cursor_range(&selection, map)
        };
        if let Some(helix_object) = self.to_helix_object() {
            Ok(helix_object.range(map, relative_to, around))
        } else {
            Err(VimToHelixError)
        }
    }
    /// Returns the range of the next object the cursor is not over.
    /// Follows helix convention.
    pub fn helix_next_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Result<Option<Range<DisplayPoint>>, VimToHelixError> {
        let cursor = cursor_range(&selection, map);
        if let Some(helix_object) = self.to_helix_object() {
            Ok(helix_object.next_range(map, cursor, around))
        } else {
            Err(VimToHelixError)
        }
    }
    /// Returns the range of the previous object the cursor is not over.
    /// Follows helix convention.
    pub fn helix_previous_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Result<Option<Range<DisplayPoint>>, VimToHelixError> {
        let cursor = cursor_range(&selection, map);
        if let Some(helix_object) = self.to_helix_object() {
            Ok(helix_object.previous_range(map, cursor, around))
        } else {
            Err(VimToHelixError)
        }
    }
}

#[derive(Debug)]
pub struct VimToHelixError;
impl Display for VimToHelixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Not all vim text objects have an implemented helix equivalent"
        )
    }
}
impl Error for VimToHelixError {}

impl VimObject {
    fn to_helix_object(self) -> Option<Box<dyn HelixTextObject>> {
        Some(match self {
            Self::AngleBrackets => Box::new(ImmediateBoundary::AngleBrackets),
            Self::AnyPair => Box::new(NearestPair),
            Self::BackQuotes => Box::new(ImmediateBoundary::BackQuotes),
            Self::CurlyBrackets => Box::new(ImmediateBoundary::CurlyBrackets),
            Self::DoubleQuotes => Box::new(ImmediateBoundary::DoubleQuotes),
            Self::Paragraph => Box::new(FuzzyBoundary::Paragraph),
            Self::Parentheses => Box::new(ImmediateBoundary::Parentheses),
            Self::Quotes => Box::new(ImmediateBoundary::SingleQuotes),
            Self::Sentence => Box::new(FuzzyBoundary::Sentence),
            Self::SquareBrackets => Box::new(ImmediateBoundary::SquareBrackets),
            Self::Subword { ignore_punctuation } => {
                Box::new(ImmediateBoundary::Subword { ignore_punctuation })
            }
            Self::VerticalBars => Box::new(ImmediateBoundary::VerticalBars),
            Self::Word { ignore_punctuation } => {
                Box::new(ImmediateBoundary::Word { ignore_punctuation })
            }
            _ => return None,
        })
    }
}

/// Returns the start of the cursor of a selection, whether that is collapsed or not.
pub(crate) fn cursor_range(
    selection: &Selection<DisplayPoint>,
    map: &DisplaySnapshot,
) -> Range<DisplayPoint> {
    if selection.is_empty() | selection.reversed {
        selection.head()..movement::right(map, selection.head())
    } else {
        movement::left(map, selection.head())..selection.head()
    }
}

#[cfg(test)]
mod test {
    use db::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_select_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let start = indoc! {"
                The quick brˇowˇnˇ
                fox «ˇjumps» ov«er
                the laˇ»zy dogˇ

                "
        };

        cx.set_state(start, Mode::HelixNormal);

        cx.simulate_keystrokes("m i w");

        cx.assert_state(
            indoc! {"
            The quick «brownˇ»
            fox «jumpsˇ» over
            the «lazyˇ» dogˇ

            "
            },
            Mode::HelixNormal,
        );

        cx.set_state(start, Mode::HelixNormal);

        cx.simulate_keystrokes("m a w");

        cx.assert_state(
            indoc! {"
            The quick« brownˇ»
            fox «jumps ˇ»over
            the «lazy ˇ»dogˇ

            "
            },
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_select_any_pair_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // The innermost surrounding pair wins, regardless of its kind.
        cx.set_state("(foo \"bar bˇaz\")", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("(foo \"«bar bazˇ»\")", Mode::HelixNormal);

        cx.set_state("(foo \"bar bˇaz\")", Mode::HelixNormal);
        cx.simulate_keystrokes("m a m");
        cx.assert_state("(foo «\"bar baz\"ˇ»)", Mode::HelixNormal);

        cx.set_state("[foo (bˇar) baz]", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("[foo («barˇ») baz]", Mode::HelixNormal);

        // Between nested pairs the outer pair is the closest one.
        cx.set_state("{foo (bar) ˇbaz}", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("{«foo (bar) bazˇ»}", Mode::HelixNormal);

        // Without a surrounding pair the selection is unchanged.
        cx.set_state("foo bˇar baz", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("foo bˇar baz", Mode::HelixNormal);

        // Pairs come from the language's bracket queries: parentheses inside
        // a string literal are plain text, so the quotes are the closest pair.
        cx.set_state("let s = (\"a (bˇc) d\");", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("let s = (\"«a (bc) dˇ»\");", Mode::HelixNormal);

        // A lifetime apostrophe is not a quote.
        cx.set_state("fn f<'a>(x: ˇ&'a str) {}", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("fn f<'a>(«x: &'a strˇ») {}", Mode::HelixNormal);

        // `|` pairs where it delimits closure parameters, but not where it is
        // a binary operator.
        cx.set_state("let f = |aˇ, b| a;", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("let f = |«a, bˇ»| a;", Mode::HelixNormal);

        cx.set_state("let x = a | bˇ | c;", Mode::HelixNormal);
        cx.simulate_keystrokes("m i m");
        cx.assert_state("let x = a | bˇ | c;", Mode::HelixNormal);

        // Composes with surround add.
        cx.set_state("let a: Vec<iˇ32> = vec![];", Mode::HelixNormal);
        cx.simulate_keystrokes("m a m m s (");
        cx.assert_state("let a: Vec«(<i32>)ˇ» = vec![];", Mode::HelixNormal);

        // Repeated `m` objects expand from the current selection, not the
        // cursor position within that selection.
        cx.set_state(
            "#[cfg_attr(feature = \"arbitˇrary\", derive(arbitrary::Arbitrary))]",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m a m");
        cx.assert_state(
            "#[cfg_attr(feature = «\"arbitrary\"ˇ», derive(arbitrary::Arbitrary))]",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m i m");
        cx.assert_state(
            "#[cfg_attr(«feature = \"arbitrary\", derive(arbitrary::Arbitrary)ˇ»)]",
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m i m");
        cx.assert_state(
            "#[«cfg_attr(feature = \"arbitrary\", derive(arbitrary::Arbitrary))ˇ»]",
            Mode::HelixNormal,
        );

        // Like other text objects, `m` also works with `]` and `[`.
        cx.set_state("foo ˇbar (baz)", Mode::HelixNormal);
        cx.simulate_keystrokes("] m");
        cx.assert_state("foo bar «(baz)ˇ»", Mode::HelixNormal);

        cx.set_state("(baz) foˇo", Mode::HelixNormal);
        cx.simulate_keystrokes("[ m");
        cx.assert_state("«ˇ(baz)» foo", Mode::HelixNormal);
    }

    #[gpui::test]
    async fn test_helix_treesitter_object_keys(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        // In Helix, `t` is the type text object, not an HTML tag.
        cx.set_state(
            indoc! {"
            const A: usize = 1;
            struct Foo {
                barˇ: usize,
            }"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m a t");
        cx.assert_state(
            indoc! {"
            const A: usize = 1;
            «struct Foo {
                bar: usize,
            }ˇ»"},
            Mode::HelixNormal,
        );

        // In Helix, `c` is the comment text object, not a class.
        cx.set_state(
            indoc! {"
            fn foo() {
                // some coˇmment
                bar();
            }"},
            Mode::HelixNormal,
        );
        cx.simulate_keystrokes("m a c");
        cx.assert_state(
            indoc! {"
            fn foo() {
                «// some commentˇ»
                bar();
            }"},
            Mode::HelixNormal,
        );
    }

    #[gpui::test]
    async fn test_helix_xml_element_object_key(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new_html(cx).await;

        // In Helix, `x` is the (X)HTML element text object.
        cx.set_state("<div>heˇllo</div>", Mode::HelixNormal);
        cx.simulate_keystrokes("m i x");
        cx.assert_state("<div>«helloˇ»</div>", Mode::HelixNormal);

        cx.set_state("<div>heˇllo</div>", Mode::HelixNormal);
        cx.simulate_keystrokes("m a x");
        cx.assert_state("«<div>hello</div>ˇ»", Mode::HelixNormal);
    }
}
