//! # Functions to escape raw text into HTML

use pastey::paste;
use std::borrow::Cow;

/// Find a `u8` in a slice. You may specify as many bytes to search for as you
/// want. If you are searching for 3 or fewer bytes, this will use [`memchr`].
macro_rules! find_u8_body {
    ($slice:expr, $ch1:literal $(,)?) => {
        memchr::memchr($ch1, $slice)
    };
    ($slice:expr, $ch1:literal, $ch2:literal $(,)?) => {
        memchr::memchr2($ch1, $ch2, $slice)
    };
    ($slice:expr, $ch1:literal, $ch2:literal, $ch3:literal $(,)?) => {
        memchr::memchr3($ch1, $ch2, $ch3, $slice)
    };
    ($slice:expr, $($ch:literal),+) => {
        $slice.iter().position(|c| matches!(c, $($ch)|+))
    };
}

/// Generate string and byte string versions of an escape function.
macro_rules! escape_fn {
    (
        $(#[$meta:meta])*
        $vis:vis fn $name:ident;

        $(#[$bytes_meta:meta])*
        $bytes_vis:vis fn $bytes_name:ident;

        {
            $($ch:literal => $entity:literal,)+
        }
    ) => {
        paste! {
            $(#[$meta])*
            $vis fn $name<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
                let input = input.into();

                match [<$name _bytes_internal>](input.as_bytes()) {
                    Some(output) => String::from_utf8(output).unwrap().into(),
                    None => input,
                }
            }

            $(#[$bytes_meta])*
            $bytes_vis fn $bytes_name<'a, S: Into<Cow<'a, [u8]>>>(input: S) -> Cow<'a, [u8]> {
                let input = input.into();

                match [<$name _bytes_internal>](&*input) {
                    Some(output) => output.into(),
                    None => input,
                }
            }

            #[inline(always)]
            fn [<$name _bytes_internal>](raw: &[u8]) -> Option<Vec<u8>> {
                #[inline]
                fn find_u8(haystack: &[u8]) -> Option<usize> {
                    find_u8_body!(haystack, $($ch),+)
                }

                #[inline]
                const fn map_u8(c: u8) -> &'static [u8] {
                    match c {
                        $( $ch => $entity, )+
                        // This should never happen, but using unreachable!()
                        // actually makes other parts of the function slower.
                        _ => b"",
                    }
                }

                if let Some(i) = find_u8(raw) {
                    let mut output: Vec<u8> = Vec::with_capacity(raw.len().saturating_mul(2));
                    output.extend_from_slice(&raw[..i]);
                    output.extend_from_slice(map_u8(raw[i]));

                    // i is a valid index, so it can't be usize::MAX.
                    debug_assert!(i < usize::MAX);
                    #[allow(clippy::arithmetic_side_effects)]
                    let mut remainder = &raw[i+1..];

                    while let Some(i) = find_u8(remainder) {
                        output.extend_from_slice(&remainder[..i]);
                        output.extend_from_slice(map_u8(remainder[i]));

                        // i is a valid index, so it can't be usize::MAX.
                        debug_assert!(i < usize::MAX);
                        #[allow(clippy::arithmetic_side_effects)]
                        let n = i + 1; // Work around https://github.com/rust-lang/rust/issues/15701
                        remainder = &remainder[n..];
                    }

                    output.extend_from_slice(&remainder);

                    Some(output)
                } else {
                    None
                }
            }
        }
    }
}

escape_fn! {
    /// Escape a string used in a text node, i.e. regular text.
    ///
    /// **Do not use this in attributes.**
    ///
    /// ```rust
    /// use htmlize::escape_text;
    /// # use assert2::assert;
    ///
    /// assert!(
    ///     escape_text("test: &<>\"'é×😀") == "test: &amp;&lt;&gt;\"'é×😀"
    /// );
    /// ```
    ///
    /// To work with bytes (`[u8]`) instead of strings, see
    /// [`escape_text_bytes()`].
    pub fn escape_text;

    /// Escape a byte string used in a text node, i.e. regular text.
    ///
    /// **Do not use this in attributes.**
    ///
    /// ```rust
    /// use htmlize::escape_text_bytes;
    /// # use assert2::assert;
    ///
    /// assert!(
    ///     escape_text_bytes(b"test: &<>\"'\xFF".as_slice())
    ///         == b"test: &amp;&lt;&gt;\"'\xFF".as_slice()
    /// );
    /// ```
    ///
    /// To work with `String` instead of bytes, see [`escape_text()`].
    pub fn escape_text_bytes;

    {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
    }
}

escape_fn! {
    /// Escape a string to be used in a quoted attribute.
    ///
    /// ```rust
    /// use htmlize::escape_attribute;
    /// # use assert2::assert;
    ///
    /// assert!(
    ///     escape_attribute("test: &<>\"'é×😀")
    ///         == "test: &amp;&lt;&gt;&quot;'é×😀"
    /// );
    /// ```
    ///
    /// To work with bytes (`[u8]`) instead of strings, see
    /// [`escape_attribute_bytes()`].
    pub fn escape_attribute;

    /// Escape a byte string to be used in a quoted attribute.
    ///
    /// ```rust
    /// use htmlize::escape_attribute_bytes;
    ///
    /// assert!(
    ///     escape_attribute_bytes(b"test: &<>\"'\xFF".as_slice())
    ///         == b"test: &amp;&lt;&gt;&quot;'\xFF".as_slice()
    /// );
    /// ```
    ///
    /// To work with `String` instead of bytes, see [`escape_attribute()`].
    pub fn escape_attribute_bytes;

    {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
        b'"' => b"&quot;", // Attributes
    }
}

escape_fn! {
    /// Escape a string including both single and double quotes.
    ///
    /// Generally, it is safe to leave single quotes (apostrophes) unescaped, so you
    /// should use [`escape_text()`] or [`escape_attribute()`].
    ///
    /// ```rust
    /// use htmlize::escape_all_quotes;
    /// # use assert2::assert;
    ///
    /// assert!(
    ///     escape_all_quotes("test: &<>\"'é×😀")
    ///         == "test: &amp;&lt;&gt;&quot;&apos;é×😀"
    /// );
    /// ```
    ///
    /// To work with bytes (`[u8]`) instead of strings, see
    /// [`escape_all_quotes_bytes()`].
    pub fn escape_all_quotes;

    /// Escape a byte string including both single and double quotes.
    ///
    /// Generally, it is safe to leave single quotes (apostrophes) unescaped, so you
    /// should use [`escape_text_bytes()`] or [`escape_attribute_bytes()`].
    ///
    /// ```rust
    /// use htmlize::escape_all_quotes_bytes;
    /// # use assert2::assert;
    ///
    /// assert!(
    ///     escape_all_quotes_bytes(b"test: &<>\"'\xFF".as_slice())
    ///         == b"test: &amp;&lt;&gt;&quot;&apos;\xFF".as_slice()
    /// );
    /// ```
    ///
    /// To work with `String` instead of bytes, see [`escape_all_quotes()`].
    pub fn escape_all_quotes_bytes;

    {
        b'&' => b"&amp;",
        b'<' => b"&lt;",
        b'>' => b"&gt;",
        b'"' => b"&quot;",  // Attributes
        b'\'' => b"&apos;", // Single quoted attributes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;
    use pastey::paste;

    macro_rules! test {
        ($name:ident, $($test:tt)+) => {
            #[test]
            fn $name() {
                #![allow(clippy::string_lit_as_bytes)]
                assert!($($test)+);
            }
        };
    }

    // Test all escape functions
    macro_rules! test_all {
        ($name:ident, $in:expr, $out:expr) => {
            paste! {
                test!([<escape_text_ $name>], escape_text($in) == $out);
                test!(
                    [<escape_attribute_ $name>],
                    escape_attribute($in) == $out
                );
                test!(
                    [<escape_all_quotes_ $name>],
                    escape_all_quotes($in) == $out
                );
                test!(
                    [<escape_text_bytes_ $name>],
                    escape_text_bytes($in.as_bytes()) == $out.as_bytes()
                );
                test!(
                    [<escape_attribute_bytes_ $name>],
                    escape_attribute_bytes($in.as_bytes()) == $out.as_bytes()
                );
                test!(
                    [<escape_all_quotes_bytes_ $name>],
                    escape_all_quotes_bytes($in.as_bytes()) == $out.as_bytes()
                );
            }
        };
    }

    test_all!(none, "", "");
    test_all!(clean, "clean", "clean");
    test_all!(lt_gt, "< >", "&lt; &gt;");
    test_all!(amp, "&amp;", "&amp;amp;");
    test_all!(prefix_amp, "prefix&", "prefix&amp;");
    test_all!(emoji_amp, "☺️&☺️", "☺️&amp;☺️");
    test_all!(
        special_clean,
        "Björk and Борис OBrien ❤️, “love beats hate”",
        "Björk and Борис OBrien ❤️, “love beats hate”"
    );

    test!(
        escape_text_quotes,
        escape_text("He said, \"That's mine.\"") == "He said, \"That's mine.\""
    );

    test!(
        escape_attribute_quotes,
        escape_attribute("He said, \"That's mine.\"")
            == "He said, &quot;That's mine.&quot;"
    );

    test!(
        escape_all_quotes_quotes,
        escape_all_quotes("He said, \"That's mine.\"")
            == "He said, &quot;That&apos;s mine.&quot;"
    );

    test!(
        escape_all_quotes_bytes_quotes,
        &*escape_all_quotes_bytes(&b"He said, \"That's mine.\""[..])
            == b"He said, &quot;That&apos;s mine.&quot;"
    );

    test!(
        escape_text_bytes_quotes,
        &*escape_text_bytes(&b"He said, \"That's mine.\""[..])
            == b"He said, \"That's mine.\""
    );

    test!(
        escape_attribute_bytes_quotes,
        &*escape_attribute_bytes(&b"He said, \"That's mine.\""[..])
            == b"He said, &quot;That's mine.&quot;"
    );

    const HTML_DIRTY: &str = include_str!("../tests/corpus/html-raw.txt");
    const HTML_DIRTY_ESCAPED: &str =
        include_str!("../tests/corpus/html-escaped.txt");
    const HTML_CLEAN: &str = include_str!("../tests/corpus/html-cleaned.txt");

    test!(
        escape_text_dirty_html,
        escape_text(HTML_DIRTY) == HTML_DIRTY_ESCAPED
    );
    test!(
        escape_text_clean_html,
        escape_text(HTML_CLEAN) == HTML_CLEAN
    );

    test!(
        escape_text_bytes_dirty_html,
        escape_text_bytes(HTML_DIRTY.as_bytes())
            == HTML_DIRTY_ESCAPED.as_bytes()
    );
    test!(
        escape_text_bytes_clean_html,
        escape_text_bytes(HTML_CLEAN.as_bytes()) == HTML_CLEAN.as_bytes()
    );

    test!(
        escape_text_bytes_invalid_utf8,
        escape_text_bytes(&b"\xa1"[..]) == &b"\xa1"[..]
    );
    test!(
        escape_attribute_bytes_invalid_utf8,
        escape_attribute_bytes(&b"\xa1"[..]) == &b"\xa1"[..]
    );
    test!(
        escape_all_quotes_bytes_invalid_utf8,
        escape_all_quotes_bytes(&b"\xa1"[..]) == &b"\xa1"[..]
    );
}
