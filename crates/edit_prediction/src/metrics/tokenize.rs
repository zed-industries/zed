fn char_class(character: char) -> u8 {
    if character.is_alphanumeric() || character == '_' {
        0
    } else if character.is_whitespace() {
        1
    } else {
        2
    }
}

pub(crate) fn tokenize(text: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut characters = text.char_indices().peekable();

    while let Some((start, character)) = characters.next() {
        let class = char_class(character);
        if class == 2 {
            tokens.push(&text[start..start + character.len_utf8()]);
            continue;
        }

        let mut end = start + character.len_utf8();
        while let Some(&(_, next_character)) = characters.peek() {
            if char_class(next_character) != class {
                break;
            }
            end += next_character.len_utf8();
            characters.next();
        }
        tokens.push(&text[start..end]);
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn tokenizes_code_like_text() {
        assert_eq!(tokenize("hello world"), vec!["hello", " ", "world"]);
        assert_eq!(
            tokenize("foo_bar123 + baz"),
            vec!["foo_bar123", " ", "+", " ", "baz"]
        );
        assert_eq!(
            tokenize("print(\"hello\")"),
            vec!["print", "(", "\"", "hello", "\"", ")"]
        );
        assert_eq!(tokenize("hello_world"), vec!["hello_world"]);
        assert_eq!(tokenize("fn();"), vec!["fn", "(", ")", ";"]);
    }
}
