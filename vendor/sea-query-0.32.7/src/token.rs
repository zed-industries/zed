//! Tokenizer for processing SQL.

use std::fmt::Write;
use std::iter::Iterator;

#[derive(Debug, Default)]
pub struct Tokenizer {
    pub chars: Vec<char>,
    pub p: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Quoted(String),
    Unquoted(String),
    Space(String),
    Punctuation(String),
}

impl Tokenizer {
    pub fn new(string: &str) -> Self {
        Self {
            chars: string.chars().collect(),
            p: 0,
        }
    }

    pub fn iter(self) -> impl Iterator<Item = Token> {
        self
    }

    fn get(&self) -> char {
        self.chars[self.p]
    }

    fn inc(&mut self) {
        self.p += 1;
    }

    fn end(&self) -> bool {
        self.p == self.chars.len()
    }

    fn space(&mut self) -> Option<Token> {
        let mut string = String::new();
        while !self.end() {
            let c = self.get();
            if Self::is_space(c) {
                write!(string, "{c}").unwrap();
            } else {
                break;
            }
            self.inc();
        }
        if !string.is_empty() {
            Some(Token::Space(string))
        } else {
            None
        }
    }

    fn unquoted(&mut self) -> Option<Token> {
        let mut string = String::new();
        let mut first = true;
        while !self.end() {
            let c = self.get();
            if Self::is_alphanumeric(c) {
                write!(string, "{c}").unwrap();
                first = false;
                self.inc();
            } else if !first && Self::is_identifier(c) {
                write!(string, "{c}").unwrap();
                self.inc();
            } else {
                break;
            }
        }
        if !string.is_empty() {
            Some(Token::Unquoted(string))
        } else {
            None
        }
    }

    fn quoted(&mut self) -> Option<Token> {
        let mut string = String::new();
        let mut first = true;
        let mut escape = false;
        let mut start = ' ';
        while !self.end() {
            let c = self.get();
            if first && Self::is_string_delimiter_start(c) {
                write!(string, "{c}").unwrap();
                first = false;
                start = c;
                self.inc();
            } else if !first && !escape && Self::is_string_delimiter_end_for(start, c) {
                write!(string, "{c}").unwrap();
                self.inc();
                if self.end() {
                    break;
                }
                if !Self::is_string_escape_for(start, self.get()) {
                    break;
                } else {
                    write!(string, "{}", self.get()).unwrap();
                    self.inc();
                }
            } else if !first {
                escape = !escape && Self::is_escape_char(c);
                write!(string, "{c}").unwrap();
                self.inc();
            } else {
                break;
            }
        }
        if !string.is_empty() {
            Some(Token::Quoted(string))
        } else {
            None
        }
    }

    /// unquote a quoted string
    fn unquote(mut self) -> String {
        let mut string = String::new();
        let mut first = true;
        let mut escape = false;
        let mut start = ' ';
        while !self.end() {
            let c = self.get();
            if first && Self::is_string_delimiter_start(c) {
                first = false;
                start = c;
                self.inc();
            } else if !first && !escape && Self::is_string_delimiter_end_for(start, c) {
                self.inc();
                if self.end() {
                    break;
                }
                if !Self::is_string_escape_for(start, self.get()) {
                    break;
                } else {
                    write!(string, "{c}").unwrap();
                    self.inc();
                }
            } else if !first {
                escape = !escape && Self::is_escape_char(c);
                write!(string, "{c}").unwrap();
                self.inc();
            } else {
                break;
            }
        }
        string
    }

    fn punctuation(&mut self) -> Option<Token> {
        let mut string = String::new();
        if !self.end() {
            let c = self.get();
            if !Self::is_space(c) && !Self::is_alphanumeric(c) {
                write!(string, "{c}").unwrap();
                self.inc();
            }
        }
        if !string.is_empty() {
            Some(Token::Punctuation(string))
        } else {
            None
        }
    }

    fn is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\r' | '\n')
    }

    fn is_identifier(c: char) -> bool {
        matches!(c, '_' | '$')
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_alphabetic() || c.is_ascii_digit()
    }

    fn is_string_delimiter_start(c: char) -> bool {
        matches!(c, '`' | '[' | '\'' | '"')
    }

    fn is_string_escape_for(start: char, c: char) -> bool {
        match start {
            '`' => c == '`',
            '\'' => c == '\'',
            '"' => c == '"',
            _ => false,
        }
    }

    fn is_string_delimiter_end_for(start: char, c: char) -> bool {
        match start {
            '`' => c == '`',
            '[' => c == ']',
            '\'' => c == '\'',
            '"' => c == '"',
            _ => false,
        }
    }

    fn is_escape_char(c: char) -> bool {
        c == '\\'
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(space) = self.space() {
            return Some(space);
        }
        if let Some(unquoted) = self.unquoted() {
            return Some(unquoted);
        }
        if let Some(quoted) = self.quoted() {
            return Some(quoted);
        }
        if let Some(punctuation) = self.punctuation() {
            return Some(punctuation);
        }
        None
    }
}

impl Token {
    pub fn is_quoted(&self) -> bool {
        matches!(self, Self::Quoted(_))
    }

    pub fn is_unquoted(&self) -> bool {
        matches!(self, Self::Unquoted(_))
    }

    pub fn is_space(&self) -> bool {
        matches!(self, Self::Space(_))
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::Punctuation(_))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Quoted(string) => string,
            Self::Unquoted(string) => string,
            Self::Space(string) => string,
            Self::Punctuation(string) => string,
        }
    }

    pub fn unquote(&self) -> Option<String> {
        if self.is_quoted() {
            let tokenizer = Tokenizer::new(self.as_str());
            Some(tokenizer.unquote())
        } else {
            None
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Unquoted(string) => string,
                Token::Space(string) => string,
                Token::Quoted(string) => string,
                Token::Punctuation(string) => string,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_0() {
        let tokenizer = Tokenizer::new("");
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_1() {
        let string = "SELECT * FROM `character`";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("*".to_string()),
                Token::Space(" ".to_string()),
                Token::Unquoted("FROM".to_string()),
                Token::Space(" ".to_string()),
                Token::Quoted("`character`".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_2() {
        let string = "SELECT * FROM `character` WHERE id = ?";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("SELECT".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("*".to_string()),
                Token::Space(" ".to_string()),
                Token::Unquoted("FROM".to_string()),
                Token::Space(" ".to_string()),
                Token::Quoted("`character`".to_string()),
                Token::Space(" ".to_string()),
                Token::Unquoted("WHERE".to_string()),
                Token::Space(" ".to_string()),
                Token::Unquoted("id".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("=".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("?".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_3() {
        let string = r#"? = "?" "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("?".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("=".to_string()),
                Token::Space(" ".to_string()),
                Token::Quoted(r#""?""#.to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_4() {
        let string = r#""a\"bc""#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Quoted("\"a\\\"bc\"".to_string())]);
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_5() {
        let string = "abc123";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Unquoted(string.to_string())]);
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_6() {
        let string = "2.3*4";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Unquoted("2".to_string()),
                Token::Punctuation(".".to_string()),
                Token::Unquoted("3".to_string()),
                Token::Punctuation("*".to_string()),
                Token::Unquoted("4".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_7() {
        let string = r#""a\\" B"#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Quoted("\"a\\\\\"".to_string()),
                Token::Space(" ".to_string()),
                Token::Unquoted("B".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_8() {
        let string = r#"`a"b` "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Quoted("`a\"b`".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_9() {
        let string = r"[ab] ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Quoted("[ab]".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_10() {
        let string = r#" 'a"b' "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" ".to_string()),
                Token::Quoted("'a\"b'".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_11() {
        let string = r" `a``b` ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" ".to_string()),
                Token::Quoted("`a``b`".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_12() {
        let string = r" 'a''b' ";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" ".to_string()),
                Token::Quoted("'a''b'".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_13() {
        let string = r"(?)";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("(".to_string()),
                Token::Punctuation("?".to_string()),
                Token::Punctuation(")".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_14() {
        let string = r"($1 = $2)";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("(".to_string()),
                Token::Punctuation("$".to_string()),
                Token::Unquoted("1".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("=".to_string()),
                Token::Space(" ".to_string()),
                Token::Punctuation("$".to_string()),
                Token::Unquoted("2".to_string()),
                Token::Punctuation(")".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_15() {
        let string = r#" "Hello World" "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" ".to_string()),
                Token::Quoted("\"Hello World\"".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_16() {
        let string = "abc_$123";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(tokens, vec![Token::Unquoted(string.to_string())]);
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_17() {
        let string = "$abc$123";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("$".to_string()),
                Token::Unquoted("abc$123".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_18() {
        let string = "_$abc_123$";
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Punctuation("_".to_string()),
                Token::Punctuation("$".to_string()),
                Token::Unquoted("abc_123$".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }

    #[test]
    fn test_19() {
        let string = r#""a\"bc""#;
        let tokenizer = Tokenizer::new(string);
        assert_eq!(tokenizer.unquote(), "a\\\"bc".to_owned());
    }

    #[test]
    fn test_20() {
        let string = r#""a""bc""#;
        let tokenizer = Tokenizer::new(string);
        assert_eq!(tokenizer.unquote(), "a\"bc".to_owned());
    }

    #[test]
    fn test_21() {
        assert_eq!(
            Token::Quoted("'a\\nb'".to_owned()).unquote().unwrap(),
            "a\\nb".to_owned()
        );
    }

    #[test]
    fn test_22() {
        let string = r#" "Hello\nWorld" "#;
        let tokenizer = Tokenizer::new(string);
        let tokens: Vec<Token> = tokenizer.iter().collect();
        assert_eq!(
            tokens,
            vec![
                Token::Space(" ".to_string()),
                Token::Quoted("\"Hello\\nWorld\"".to_string()),
                Token::Space(" ".to_string()),
            ]
        );
        assert_eq!(
            string,
            tokens.iter().map(|x| x.to_string()).collect::<String>()
        );
    }
}
