use std::{iter::Peekable, str::CharIndices};

#[derive(Clone, Copy, PartialEq, Eq)]
enum CharClass {
    Identifier,
    Newline,
    Whitespace,
    Punctuation,
}

const MULTI_CHAR_PUNCTUATION: &[&str] = &[
    ">>>=", "<<=", ">>=", "...", "..=", "??=", "**=", ">>>", "::", "->", "=>", "==", "!=", "<=",
    ">=", "&&", "||", "<<", ">>", "..", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "++", "--",
    "**", "??", "?.", ":=", "<-", "//", "/*", "*/",
];

fn char_class(character: char) -> CharClass {
    if character == '\n' || character == '\r' {
        CharClass::Newline
    } else if character.is_whitespace() {
        CharClass::Whitespace
    } else if character.is_alphanumeric() || character == '_' {
        CharClass::Identifier
    } else {
        CharClass::Punctuation
    }
}

fn is_identifier_boundary(previous: char, current: char, next: Option<char>) -> bool {
    (current.is_uppercase() && (previous.is_lowercase() || previous.is_numeric()))
        || (current.is_uppercase()
            && previous.is_uppercase()
            && next.is_some_and(|next| next.is_lowercase()))
}

fn push_identifier_tokens<'a>(identifier: &'a str, tokens: &mut Vec<&'a str>) {
    let characters: Vec<(usize, char)> = identifier.char_indices().collect();
    let mut segment_start = 0;
    let mut index = 0;

    while index < characters.len() {
        let (byte_index, character) = characters[index];

        if character == '_' {
            if segment_start < byte_index {
                tokens.push(&identifier[segment_start..byte_index]);
            }

            let mut underscore_end = byte_index + character.len_utf8();
            index += 1;

            while index < characters.len() && characters[index].1 == '_' {
                underscore_end = characters[index].0 + characters[index].1.len_utf8();
                index += 1;
            }

            tokens.push(&identifier[byte_index..underscore_end]);
            segment_start = underscore_end;
            continue;
        }

        if byte_index > segment_start {
            let previous = characters[index - 1].1;
            let next = characters.get(index + 1).map(|(_, character)| *character);

            if is_identifier_boundary(previous, character, next) {
                tokens.push(&identifier[segment_start..byte_index]);
                segment_start = byte_index;
            }
        }

        index += 1;
    }

    if segment_start < identifier.len() {
        tokens.push(&identifier[segment_start..]);
    }
}

fn push_punctuation_token<'a>(
    text: &'a str,
    start: usize,
    character: char,
    characters: &mut Peekable<CharIndices<'a>>,
    tokens: &mut Vec<&'a str>,
) {
    let remaining = &text[start..];

    for punctuation in MULTI_CHAR_PUNCTUATION {
        if remaining.starts_with(punctuation) {
            for _ in punctuation.chars().skip(1) {
                characters.next();
            }

            tokens.push(&remaining[..punctuation.len()]);
            return;
        }
    }

    let end = start + character.len_utf8();
    tokens.push(&text[start..end]);
}

pub(crate) fn tokenize(text: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut characters = text.char_indices().peekable();

    while let Some((start, character)) = characters.next() {
        match char_class(character) {
            CharClass::Identifier => {
                let mut end = start + character.len_utf8();

                while let Some(&(next_start, next_character)) = characters.peek() {
                    if char_class(next_character) != CharClass::Identifier {
                        break;
                    }

                    end = next_start + next_character.len_utf8();
                    characters.next();
                }

                push_identifier_tokens(&text[start..end], &mut tokens);
            }
            CharClass::Newline => {
                let mut end = start + character.len_utf8();

                while let Some(&(next_start, next_character)) = characters.peek() {
                    if char_class(next_character) != CharClass::Newline {
                        break;
                    }

                    end = next_start + next_character.len_utf8();
                    characters.next();
                }

                tokens.push(&text[start..end]);
            }
            CharClass::Whitespace => {
                let mut end = start + character.len_utf8();

                while let Some(&(next_start, next_character)) = characters.peek() {
                    if char_class(next_character) != CharClass::Whitespace {
                        break;
                    }

                    end = next_start + next_character.len_utf8();
                    characters.next();
                }

                tokens.push(&text[start..end]);
            }
            CharClass::Punctuation => {
                push_punctuation_token(text, start, character, &mut characters, &mut tokens);
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn tokenizes_code() {
        assert_eq!(tokenize("hello world"), vec!["hello", " ", "world"]);
        assert_eq!(
            tokenize("foo_bar123 + baz"),
            vec!["foo", "_", "bar123", " ", "+", " ", "baz"]
        );
        assert_eq!(
            tokenize("print(\"hello\")"),
            vec!["print", "(", "\"", "hello", "\"", ")"]
        );
        assert_eq!(tokenize("hello_world"), vec!["hello", "_", "world"]);
        assert_eq!(tokenize("fn();"), vec!["fn", "(", ")", ";"]);
    }

    #[test]
    fn tokenizes_identifier_case_styles() {
        assert_eq!(
            tokenize("camelCase PascalCase snake_case"),
            vec![
                "camel", "Case", " ", "Pascal", "Case", " ", "snake", "_", "case"
            ]
        );
        assert_eq!(
            tokenize("myHTTPServer __private_value foo__bar"),
            vec![
                "my", "HTTP", "Server", " ", "__", "private", "_", "value", " ", "foo", "__", "bar"
            ]
        );
        assert_eq!(
            tokenize("XMLHttpRequest Version2Update"),
            vec!["XML", "Http", "Request", " ", "Version2", "Update"]
        );
    }

    #[test]
    fn tokenizes_grouped_punctuation() {
        assert_eq!(
            tokenize("a::b -> c != d ..= e"),
            vec![
                "a", "::", "b", " ", "->", " ", "c", " ", "!=", " ", "d", " ", "..=", " ", "e"
            ]
        );
        assert_eq!(
            tokenize("foo?.bar ?? baz"),
            vec!["foo", "?.", "bar", " ", "??", " ", "baz"]
        );
    }

    #[test]
    fn tokenize_whitespace_runs() {
        assert_eq!(tokenize("  "), vec!["  "]);
        assert_eq!(tokenize("  \n   foo"), vec!["  ", "\n", "   ", "foo"]);
        assert_eq!(tokenize("\r\n\nfoo"), vec!["\r\n\n", "foo"]);
    }
}
