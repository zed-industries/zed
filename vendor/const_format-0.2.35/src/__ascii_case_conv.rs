mod word_iterator;

use word_iterator::WordIterator;

/// The casing style of a string.
///
/// You can pass this to [`map_ascii_case`] to determine the casing style of the
/// returned `&'static str`.
///
///
/// [`map_ascii_case`]: ./macro.map_ascii_case.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Case {
    /// Lowercase
    Lower,
    /// Uppercase
    Upper,
    /// Pascal case, eg: `FooBarBaz`. The first character is always uppercase.
    Pascal,
    /// Camel case, eg: `fooBarBaz`. The first character is always lowercase.
    Camel,
    /// Snake case, eg: `foo_bar_baz`. Also turns the string lowercase.
    Snake,
    /// Snake case, eg: `FOO_BAR_BAZ`. Also turns the string uppercase.
    UpperSnake,
    /// Kebab case, eg: `foo-bar-baz`. Also turns the string lowercase.
    Kebab,
    /// Kebab case, eg: `FOO-BAR-BAZ`. Also turns the string uppercase.
    UpperKebab,
}

macro_rules! if_next_word {
    ($word_iterator:ident, $word_range:ident => $then:block $(else $else:block)? ) => {
        #[allow(unused_mut)]
        if let Some((niter, mut $word_range)) = $word_iterator.next() {
            $word_iterator = niter;

            $then
        } $(else $else)?
    };
}

macro_rules! while_next_word {
    ($word_iterator:ident, $word_range:ident => $then:block) => {
        #[allow(unused_mut)]
        while let Some((niter, mut $word_range)) = $word_iterator.next() {
            $word_iterator = niter;

            $then
        }
    };
}

struct WordCountAndLength {
    /// The amount of words
    count: usize,
    /// The length of all words added up
    length: usize,
}

const fn words_count_and_length(bytes: &[u8]) -> WordCountAndLength {
    let mut count = 0;
    let mut length = 0;
    let mut word_iter = WordIterator::new(bytes);
    while_next_word! {word_iter, word_range => {
        count += 1;
        length += word_range.end - word_range.start;
    }}
    WordCountAndLength { count, length }
}

pub const fn size_after_conversion(case: Case, s: &str) -> usize {
    match case {
        Case::Upper | Case::Lower => s.len(),
        Case::Pascal | Case::Camel => {
            let wcl = words_count_and_length(s.as_bytes());
            wcl.length
        }
        Case::Snake | Case::Kebab | Case::UpperSnake | Case::UpperKebab => {
            let wcl = words_count_and_length(s.as_bytes());
            wcl.length + wcl.count.saturating_sub(1)
        }
    }
}

pub const fn convert_str<const N: usize>(case: Case, s: &str) -> [u8; N] {
    let mut arr = [0; N];
    let mut inp = s.as_bytes();
    let mut o = 0;

    macro_rules! map_bytes {
        ($byte:ident => $e:expr) => {
            while let [$byte, rem @ ..] = inp {
                let $byte = *$byte;
                inp = rem;
                arr[o] = $e;
                o += 1;
            }
        };
    }

    macro_rules! write_byte {
        ($byte:expr) => {
            arr[o] = $byte;
            o += 1;
        };
    }

    macro_rules! write_range_from {
        ($range:expr, $from:expr, $byte:ident => $mapper:expr) => {{
            let mut range = $range;
            while range.start < range.end {
                let $byte = $from[range.start];
                arr[o] = $mapper;

                range.start += 1;
                o += 1;
            }
        }};
    }

    macro_rules! write_snake_kebab_case {
        ($separator:expr, $byte_conversion:expr) => {{
            let mut word_iter = WordIterator::new(inp);

            if_next_word! {word_iter, word_range => {
                write_range_from!(word_range, inp, byte => $byte_conversion(byte));

                while_next_word!{word_iter, word_range => {
                    write_byte!($separator);
                    write_range_from!(word_range, inp, byte => $byte_conversion(byte));
                }}
            }}
        }};
    }

    macro_rules! write_pascal_camel_case {
        ($first_word_conv:expr) => {{
            let mut word_iter = WordIterator::new(inp);

            if_next_word! {word_iter, word_range => {
                write_byte!($first_word_conv(inp[word_range.start]));
                word_range.start += 1;
                write_range_from!(word_range, inp, byte => lowercase_u8(byte));

                while_next_word!{word_iter, word_range => {
                    write_byte!(uppercase_u8(inp[word_range.start]));
                    word_range.start += 1;
                    write_range_from!(word_range, inp, byte => lowercase_u8(byte));
                }}
            }}
        }};
    }

    match case {
        Case::Upper => map_bytes!(b => uppercase_u8(b)),
        Case::Lower => map_bytes!(b => lowercase_u8(b)),
        Case::Snake => write_snake_kebab_case!(b'_', lowercase_u8),
        Case::UpperSnake => write_snake_kebab_case!(b'_', uppercase_u8),
        Case::Kebab => write_snake_kebab_case!(b'-', lowercase_u8),
        Case::UpperKebab => write_snake_kebab_case!(b'-', uppercase_u8),
        Case::Pascal => write_pascal_camel_case!(uppercase_u8),
        Case::Camel => write_pascal_camel_case!(lowercase_u8),
    }

    arr
}

const CASE_DIFF: u8 = b'a' - b'A';

const fn uppercase_u8(b: u8) -> u8 {
    if let b'a'..=b'z' = b {
        b - CASE_DIFF
    } else {
        b
    }
}

const fn lowercase_u8(b: u8) -> u8 {
    if let b'A'..=b'Z' = b {
        b + CASE_DIFF
    } else {
        b
    }
}
