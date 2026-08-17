use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take, take_until, take_while1};
use nom::character::complete::{anychar, char, digit0, digit1, not_line_ending};
use nom::combinator::{eof, opt, peek, recognize, verify};
use nom::error::ParseError;
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::{terminated, tuple};
use nom::{AsChar, Err, IResult};
use std::borrow::Cow;
use unicode_categories::UnicodeCategories;

pub(crate) fn tokenize(mut input: &str, named_placeholders: bool) -> Vec<Token<'_>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut last_reserved_token = None;
    let mut last_reserved_top_level_token = None;

    // Keep processing the string until it is empty
    while let Ok(result) = get_next_token(
        input,
        tokens.last().cloned(),
        last_reserved_token.clone(),
        last_reserved_top_level_token.clone(),
        named_placeholders,
    ) {
        if result.1.kind == TokenKind::Reserved {
            last_reserved_token = Some(result.1.clone());
        } else if result.1.kind == TokenKind::ReservedTopLevel {
            last_reserved_top_level_token = Some(result.1.clone());
        }
        input = result.0;

        tokens.push(result.1);
    }
    tokens
}

#[derive(Debug, Clone)]
pub(crate) struct Token<'a> {
    pub kind: TokenKind,
    pub value: &'a str,
    // Only used for placeholder--there is a reason this isn't on the enum
    pub key: Option<PlaceholderKind<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Whitespace,
    String,
    Reserved,
    ReservedTopLevel,
    ReservedTopLevelNoIndent,
    ReservedNewline,
    Operator,
    OpenParen,
    CloseParen,
    LineComment,
    BlockComment,
    Number,
    Placeholder,
    Word,
}

#[derive(Debug, Clone)]
pub(crate) enum PlaceholderKind<'a> {
    Named(Cow<'a, str>),
    ZeroIndexed(usize),
    OneIndexed(usize),
}

impl<'a> PlaceholderKind<'a> {
    pub fn named(&'a self) -> &'a str {
        match self {
            PlaceholderKind::Named(val) => val.as_ref(),
            _ => "",
        }
    }

    pub fn indexed(&self) -> Option<usize> {
        match self {
            PlaceholderKind::ZeroIndexed(val) => Some(*val),
            PlaceholderKind::OneIndexed(val) => Some(*val - 1),
            _ => None,
        }
    }
}

fn get_next_token<'a>(
    input: &'a str,
    previous_token: Option<Token<'a>>,
    last_reserved_token: Option<Token<'a>>,
    last_reserved_top_level_token: Option<Token<'a>>,
    named_placeholders: bool,
) -> IResult<&'a str, Token<'a>> {
    get_whitespace_token(input)
        .or_else(|_| get_comment_token(input))
        .or_else(|_| get_string_token(input))
        .or_else(|_| get_open_paren_token(input))
        .or_else(|_| get_close_paren_token(input))
        .or_else(|_| get_number_token(input))
        .or_else(|_| {
            get_reserved_word_token(
                input,
                previous_token,
                last_reserved_token,
                last_reserved_top_level_token,
            )
        })
        .or_else(|_| get_placeholder_token(input, named_placeholders))
        .or_else(|_| get_word_token(input))
        .or_else(|_| get_operator_token(input))
}

fn get_whitespace_token(input: &str) -> IResult<&str, Token<'_>> {
    take_while1(char::is_whitespace)(input).map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::Whitespace,
                value: token,
                key: None,
            },
        )
    })
}

fn get_comment_token(input: &str) -> IResult<&str, Token<'_>> {
    get_line_comment_token(input).or_else(|_| get_block_comment_token(input))
}

fn get_line_comment_token(input: &str) -> IResult<&str, Token<'_>> {
    recognize(tuple((alt((tag("#"), tag("--"))), not_line_ending)))(input).map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::LineComment,
                value: token,
                key: None,
            },
        )
    })
}

fn get_block_comment_token(input: &str) -> IResult<&str, Token<'_>> {
    recognize(tuple((
        tag("/*"),
        alt((take_until("*/"), recognize(many0(anychar)))),
        opt(take(2usize)),
    )))(input)
    .map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::BlockComment,
                value: token,
                key: None,
            },
        )
    })
}

pub fn take_till_escaping<'a, Error: ParseError<&'a str>>(
    desired: char,
    escapes: &'static [char],
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, Error> {
    move |input: &str| {
        let mut chars = input.chars().enumerate().peekable();
        loop {
            let item = chars.next();
            let next = chars.peek().map(|item| item.1);
            match item {
                Some(item) => {
                    // escape?
                    if escapes.contains(&item.1) && next.map(|n| n == desired).unwrap_or(false) {
                        // consume this and next char
                        chars.next();
                        continue;
                    }

                    if item.1 == desired {
                        let byte_pos = input.chars().take(item.0).map(|c| c.len()).sum::<usize>();
                        return Ok((&input[byte_pos..], &input[..byte_pos]));
                    }
                }
                None => {
                    return Ok(("", input));
                }
            }
        }
    }
}

// This enables the following string patterns:
// 1. backtick quoted string using `` to escape
// 2. square bracket quoted string (SQL Server) using ]] to escape
// 3. double quoted string using "" or \" to escape
// 4. single quoted string using '' or \' to escape
// 5. national character quoted string using N'' or N\' to escape
fn get_string_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((
        recognize(tuple((
            char('`'),
            take_till_escaping('`', &['`']),
            take(1usize),
        ))),
        recognize(tuple((
            char('['),
            take_till_escaping(']', &[']']),
            take(1usize),
        ))),
        recognize(tuple((
            char('"'),
            take_till_escaping('"', &['"', '\\']),
            take(1usize),
        ))),
        recognize(tuple((
            char('\''),
            take_till_escaping('\'', &['\'', '\\']),
            take(1usize),
        ))),
        recognize(tuple((
            tag("N'"),
            take_till_escaping('\'', &['\'', '\\']),
            take(1usize),
        ))),
        recognize(tuple((
            tag("E'"),
            take_till_escaping('\'', &['\'', '\\']),
            take(1usize),
        ))),
    ))(input)
    .map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::String,
                value: token,
                key: None,
            },
        )
    })
}

// Like above but it doesn't replace double quotes
fn get_placeholder_string_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((
        recognize(tuple((
            char('`'),
            take_till_escaping('`', &['`']),
            take(1usize),
        ))),
        recognize(tuple((
            char('['),
            take_till_escaping(']', &[']']),
            take(1usize),
        ))),
        recognize(tuple((
            char('"'),
            take_till_escaping('"', &['\\']),
            take(1usize),
        ))),
        recognize(tuple((
            char('\''),
            take_till_escaping('\'', &['\\']),
            take(1usize),
        ))),
        recognize(tuple((
            tag("N'"),
            take_till_escaping('\'', &['\\']),
            take(1usize),
        ))),
    ))(input)
    .map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::String,
                value: token,
                key: None,
            },
        )
    })
}

fn get_open_paren_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((tag("("), terminated(tag_no_case("CASE"), end_of_word)))(input).map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::OpenParen,
                value: token,
                key: None,
            },
        )
    })
}

fn get_close_paren_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((tag(")"), terminated(tag_no_case("END"), end_of_word)))(input).map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::CloseParen,
                value: token,
                key: None,
            },
        )
    })
}

fn get_placeholder_token(input: &str, named_placeholders: bool) -> IResult<&str, Token<'_>> {
    // The precedence changes based on 'named_placeholders' but not the exhaustiveness.
    // This is to ensure the formatting is the same even if parameters aren't used.

    if named_placeholders {
        alt((
            get_ident_named_placeholder_token,
            get_string_named_placeholder_token,
            get_indexed_placeholder_token,
        ))(input)
    } else {
        alt((
            get_indexed_placeholder_token,
            get_ident_named_placeholder_token,
            get_string_named_placeholder_token,
        ))(input)
    }
}

fn get_indexed_placeholder_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((
        recognize(tuple((alt((char('?'), char('$'))), digit1))),
        recognize(char('?')),
    ))(input)
    .map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::Placeholder,
                value: token,
                key: if token.len() > 1 {
                    if let Ok(index) = token[1..].parse::<usize>() {
                        Some(if token.starts_with('$') {
                            PlaceholderKind::OneIndexed(index)
                        } else {
                            PlaceholderKind::ZeroIndexed(index)
                        })
                    } else {
                        None
                    }
                } else {
                    None
                },
            },
        )
    })
}

fn get_ident_named_placeholder_token(input: &str) -> IResult<&str, Token<'_>> {
    recognize(tuple((
        alt((char('@'), char(':'), char('$'))),
        take_while1(|item: char| {
            item.is_alphanumeric() || item == '.' || item == '_' || item == '$'
        }),
    )))(input)
    .map(|(input, token)| {
        let index = Cow::Borrowed(&token[1..]);
        (
            input,
            Token {
                kind: TokenKind::Placeholder,
                value: token,
                key: Some(PlaceholderKind::Named(index)),
            },
        )
    })
}

fn get_string_named_placeholder_token(input: &str) -> IResult<&str, Token<'_>> {
    recognize(tuple((
        alt((char('@'), char(':'))),
        get_placeholder_string_token,
    )))(input)
    .map(|(input, token)| {
        let index =
            get_escaped_placeholder_key(&token[2..token.len() - 1], &token[token.len() - 1..]);
        (
            input,
            Token {
                kind: TokenKind::Placeholder,
                value: token,
                key: Some(PlaceholderKind::Named(index)),
            },
        )
    })
}

fn get_escaped_placeholder_key<'a>(key: &'a str, quote_char: &str) -> Cow<'a, str> {
    Cow::Owned(key.replace(&format!("\\{}", quote_char), quote_char))
}

fn get_number_token(input: &str) -> IResult<&str, Token<'_>> {
    recognize(tuple((
        opt(tag("-")),
        alt((scientific_notation, decimal_number, digit1)),
    )))(input)
    .map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::Number,
                value: token,
                key: None,
            },
        )
    })
}

fn decimal_number(input: &str) -> IResult<&str, &str> {
    recognize(tuple((digit1, tag("."), digit0)))(input)
}

fn scientific_notation(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((decimal_number, digit1)),
        tag("e"),
        alt((tag("-"), tag("+"), tag(""))),
        digit1,
    )))(input)
}

fn get_reserved_word_token<'a>(
    input: &'a str,
    previous_token: Option<Token<'a>>,
    last_reserved_token: Option<Token<'a>>,
    last_reserved_top_level_token: Option<Token<'a>>,
) -> IResult<&'a str, Token<'a>> {
    // A reserved word cannot be preceded by a "."
    // this makes it so in "my_table.from", "from" is not considered a reserved word
    if let Some(token) = previous_token {
        if token.value == "." {
            return Err(Err::Error(Error::new(input, ErrorKind::IsNot)));
        }
    }

    alt((
        get_top_level_reserved_token(last_reserved_top_level_token),
        get_newline_reserved_token(last_reserved_token),
        get_top_level_reserved_token_no_indent,
        get_plain_reserved_token,
    ))(input)
}

// We have to be a bit creative here for performance reasons
fn get_uc_words(input: &str, words: usize) -> String {
    input
        .split_whitespace()
        .take(words)
        .collect::<Vec<&str>>()
        .join(" ")
        .to_ascii_uppercase()
}

fn get_top_level_reserved_token<'a>(
    last_reserved_top_level_token: Option<Token<'a>>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Token<'a>> {
    move |input: &'a str| {
        let uc_input: String = get_uc_words(input, 3);
        let result: IResult<&str, &str> = alt((
            terminated(tag("ADD"), end_of_word),
            terminated(tag("AFTER"), end_of_word),
            terminated(tag("ALTER COLUMN"), end_of_word),
            terminated(tag("ALTER TABLE"), end_of_word),
            terminated(tag("DELETE FROM"), end_of_word),
            terminated(tag("EXCEPT"), end_of_word),
            terminated(tag("FETCH FIRST"), end_of_word),
            terminated(tag("FROM"), end_of_word),
            terminated(tag("GROUP BY"), end_of_word),
            terminated(tag("GO"), end_of_word),
            terminated(tag("HAVING"), end_of_word),
            terminated(tag("INSERT INTO"), end_of_word),
            terminated(tag("INSERT"), end_of_word),
            terminated(tag("LIMIT"), end_of_word),
            terminated(tag("MODIFY"), end_of_word),
            terminated(tag("ORDER BY"), end_of_word),
            terminated(tag("SELECT"), end_of_word),
            terminated(tag("SET CURRENT SCHEMA"), end_of_word),
            terminated(tag("SET SCHEMA"), end_of_word),
            terminated(tag("SET"), end_of_word),
            alt((
                terminated(tag("UPDATE"), end_of_word),
                terminated(tag("VALUES"), end_of_word),
                terminated(tag("WHERE"), end_of_word),
                terminated(tag("RETURNING"), end_of_word),
            )),
        ))(&uc_input);
        if let Ok((_, token)) = result {
            let final_word = token.split(' ').last().unwrap();
            let input_end_pos =
                input.to_ascii_uppercase().find(final_word).unwrap() + final_word.len();
            let (token, input) = input.split_at(input_end_pos);
            let kind = if token == "EXCEPT"
                && last_reserved_top_level_token.is_some()
                && last_reserved_top_level_token.as_ref().unwrap().value == "SELECT"
            {
                // If the query statement before and after the except keyword is not complete, mark it a `Word`
                TokenKind::Word
            } else {
                TokenKind::ReservedTopLevel
            };
            Ok((
                input,
                Token {
                    kind,
                    value: token,
                    key: None,
                },
            ))
        } else {
            Err(Err::Error(Error::new(input, ErrorKind::Alt)))
        }
    }
}

fn get_newline_reserved_token<'a>(
    last_reserved_token: Option<Token<'a>>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Token<'a>> {
    move |input: &'a str| {
        let uc_input = get_uc_words(input, 3);
        let result: IResult<&str, &str> = alt((
            terminated(tag("AND"), end_of_word),
            terminated(tag("CROSS APPLY"), end_of_word),
            terminated(tag("CROSS JOIN"), end_of_word),
            terminated(tag("ELSE"), end_of_word),
            terminated(tag("INNER JOIN"), end_of_word),
            terminated(tag("JOIN"), end_of_word),
            terminated(tag("LEFT JOIN"), end_of_word),
            terminated(tag("LEFT OUTER JOIN"), end_of_word),
            terminated(tag("OR"), end_of_word),
            terminated(tag("OUTER APPLY"), end_of_word),
            terminated(tag("OUTER JOIN"), end_of_word),
            terminated(tag("RIGHT JOIN"), end_of_word),
            terminated(tag("RIGHT OUTER JOIN"), end_of_word),
            terminated(tag("WHEN"), end_of_word),
            terminated(tag("XOR"), end_of_word),
        ))(&uc_input);
        if let Ok((_, token)) = result {
            let final_word = token.split(' ').last().unwrap();
            let input_end_pos =
                input.to_ascii_uppercase().find(final_word).unwrap() + final_word.len();
            let (token, input) = input.split_at(input_end_pos);
            let kind = if token == "AND"
                && last_reserved_token.is_some()
                && last_reserved_token.as_ref().unwrap().value == "BETWEEN"
            {
                // If the "AND" is part of a "BETWEEN" clause, we want to handle it as one clause by not adding a new line.
                TokenKind::Reserved
            } else {
                TokenKind::ReservedNewline
            };
            Ok((
                input,
                Token {
                    kind,
                    value: token,
                    key: None,
                },
            ))
        } else {
            Err(Err::Error(Error::new(input, ErrorKind::Alt)))
        }
    }
}

fn get_top_level_reserved_token_no_indent(input: &str) -> IResult<&str, Token<'_>> {
    let uc_input = get_uc_words(input, 2);
    let result: IResult<&str, &str> = alt((
        terminated(tag("BEGIN"), end_of_word),
        terminated(tag("DECLARE"), end_of_word),
        terminated(tag("INTERSECT"), end_of_word),
        terminated(tag("INTERSECT ALL"), end_of_word),
        terminated(tag("MINUS"), end_of_word),
        terminated(tag("UNION"), end_of_word),
        terminated(tag("UNION ALL"), end_of_word),
        terminated(tag("$$"), end_of_word),
    ))(&uc_input);
    if let Ok((_, token)) = result {
        let final_word = token.split(' ').last().unwrap();
        let input_end_pos = input.to_ascii_uppercase().find(final_word).unwrap() + final_word.len();
        let (token, input) = input.split_at(input_end_pos);
        Ok((
            input,
            Token {
                kind: TokenKind::ReservedTopLevelNoIndent,
                value: token,
                key: None,
            },
        ))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Alt)))
    }
}
fn get_plain_reserved_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((get_plain_reserved_two_token, get_plain_reserved_one_token))(input)
}
fn get_plain_reserved_one_token(input: &str) -> IResult<&str, Token<'_>> {
    let uc_input = get_uc_words(input, 1);
    let result: IResult<&str, &str> = alt((
        terminated(tag("ACCESSIBLE"), end_of_word),
        terminated(tag("ACTION"), end_of_word),
        terminated(tag("AGAINST"), end_of_word),
        terminated(tag("AGGREGATE"), end_of_word),
        terminated(tag("ALGORITHM"), end_of_word),
        terminated(tag("ALL"), end_of_word),
        terminated(tag("ALTER"), end_of_word),
        terminated(tag("ANALYSE"), end_of_word),
        terminated(tag("ANALYZE"), end_of_word),
        terminated(tag("AS"), end_of_word),
        terminated(tag("ASC"), end_of_word),
        terminated(tag("AUTOCOMMIT"), end_of_word),
        terminated(tag("AUTO_INCREMENT"), end_of_word),
        terminated(tag("BACKUP"), end_of_word),
        terminated(tag("BETWEEN"), end_of_word),
        terminated(tag("BINLOG"), end_of_word),
        terminated(tag("BOTH"), end_of_word),
        terminated(tag("CASCADE"), end_of_word),
        terminated(tag("CASE"), end_of_word),
        alt((
            terminated(tag("CHANGE"), end_of_word),
            terminated(tag("CHANGED"), end_of_word),
            terminated(tag("CHARSET"), end_of_word),
            terminated(tag("CHECK"), end_of_word),
            terminated(tag("CHECKSUM"), end_of_word),
            terminated(tag("COLLATE"), end_of_word),
            terminated(tag("COLLATION"), end_of_word),
            terminated(tag("COLUMN"), end_of_word),
            terminated(tag("COLUMNS"), end_of_word),
            terminated(tag("COMMENT"), end_of_word),
            terminated(tag("COMMIT"), end_of_word),
            terminated(tag("COMMITTED"), end_of_word),
            terminated(tag("COMPRESSED"), end_of_word),
            terminated(tag("CONCURRENT"), end_of_word),
            terminated(tag("CONSTRAINT"), end_of_word),
            terminated(tag("CONTAINS"), end_of_word),
            terminated(tag("CONVERT"), end_of_word),
            terminated(tag("CREATE"), end_of_word),
            terminated(tag("CROSS"), end_of_word),
            alt((
                terminated(tag("CURRENT_TIMESTAMP"), end_of_word),
                terminated(tag("DATABASE"), end_of_word),
                terminated(tag("DATABASES"), end_of_word),
                terminated(tag("DAY"), end_of_word),
                terminated(tag("DAY_HOUR"), end_of_word),
                terminated(tag("DAY_MINUTE"), end_of_word),
                terminated(tag("DAY_SECOND"), end_of_word),
                terminated(tag("DEFAULT"), end_of_word),
                terminated(tag("DEFINER"), end_of_word),
                terminated(tag("DELAYED"), end_of_word),
                terminated(tag("DELETE"), end_of_word),
                terminated(tag("DESC"), end_of_word),
                terminated(tag("DESCRIBE"), end_of_word),
                terminated(tag("DETERMINISTIC"), end_of_word),
                terminated(tag("DISTINCT"), end_of_word),
                terminated(tag("DISTINCTROW"), end_of_word),
                terminated(tag("DIV"), end_of_word),
                terminated(tag("DO"), end_of_word),
                terminated(tag("DROP"), end_of_word),
                terminated(tag("DUMPFILE"), end_of_word),
                alt((
                    terminated(tag("DUPLICATE"), end_of_word),
                    terminated(tag("DYNAMIC"), end_of_word),
                    terminated(tag("ELSE"), end_of_word),
                    terminated(tag("ENCLOSED"), end_of_word),
                    terminated(tag("END"), end_of_word),
                    terminated(tag("ENGINE"), end_of_word),
                    terminated(tag("ENGINES"), end_of_word),
                    terminated(tag("ENGINE_TYPE"), end_of_word),
                    terminated(tag("ESCAPE"), end_of_word),
                    terminated(tag("ESCAPED"), end_of_word),
                    terminated(tag("EVENTS"), end_of_word),
                    terminated(tag("EXEC"), end_of_word),
                    terminated(tag("EXECUTE"), end_of_word),
                    terminated(tag("EXISTS"), end_of_word),
                    terminated(tag("EXPLAIN"), end_of_word),
                    terminated(tag("EXTENDED"), end_of_word),
                    terminated(tag("FAST"), end_of_word),
                    terminated(tag("FETCH"), end_of_word),
                    terminated(tag("FIELDS"), end_of_word),
                    alt((
                        terminated(tag("FILE"), end_of_word),
                        terminated(tag("FIRST"), end_of_word),
                        terminated(tag("FIXED"), end_of_word),
                        terminated(tag("FLUSH"), end_of_word),
                        terminated(tag("FOR"), end_of_word),
                        terminated(tag("FORCE"), end_of_word),
                        terminated(tag("FOREIGN"), end_of_word),
                        terminated(tag("FULL"), end_of_word),
                        terminated(tag("FULLTEXT"), end_of_word),
                        terminated(tag("FUNCTION"), end_of_word),
                        terminated(tag("GLOBAL"), end_of_word),
                        terminated(tag("GRANT"), end_of_word),
                        terminated(tag("GRANTS"), end_of_word),
                        terminated(tag("GROUP_CONCAT"), end_of_word),
                        terminated(tag("HEAP"), end_of_word),
                        terminated(tag("HIGH_PRIORITY"), end_of_word),
                        terminated(tag("HOSTS"), end_of_word),
                        terminated(tag("HOUR"), end_of_word),
                        terminated(tag("HOUR_MINUTE"), end_of_word),
                        terminated(tag("HOUR_SECOND"), end_of_word),
                        alt((
                            terminated(tag("IDENTIFIED"), end_of_word),
                            terminated(tag("IF"), end_of_word),
                            terminated(tag("IFNULL"), end_of_word),
                            terminated(tag("IGNORE"), end_of_word),
                            terminated(tag("IN"), end_of_word),
                            terminated(tag("INDEX"), end_of_word),
                            terminated(tag("INDEXES"), end_of_word),
                            terminated(tag("INFILE"), end_of_word),
                            terminated(tag("INSERT"), end_of_word),
                            terminated(tag("INSERT_ID"), end_of_word),
                            terminated(tag("INSERT_METHOD"), end_of_word),
                            terminated(tag("INTERVAL"), end_of_word),
                            terminated(tag("INTO"), end_of_word),
                            terminated(tag("INVOKER"), end_of_word),
                            terminated(tag("IS"), end_of_word),
                            terminated(tag("ISOLATION"), end_of_word),
                            terminated(tag("KEY"), end_of_word),
                            terminated(tag("KEYS"), end_of_word),
                            terminated(tag("KILL"), end_of_word),
                            terminated(tag("LAST_INSERT_ID"), end_of_word),
                            alt((
                                terminated(tag("LEADING"), end_of_word),
                                terminated(tag("LEVEL"), end_of_word),
                                terminated(tag("LIKE"), end_of_word),
                                terminated(tag("LINEAR"), end_of_word),
                                terminated(tag("LINES"), end_of_word),
                                terminated(tag("LOAD"), end_of_word),
                                terminated(tag("LOCAL"), end_of_word),
                                terminated(tag("LOCK"), end_of_word),
                                terminated(tag("LOCKS"), end_of_word),
                                terminated(tag("LOGS"), end_of_word),
                                terminated(tag("LOW_PRIORITY"), end_of_word),
                                terminated(tag("MARIA"), end_of_word),
                                terminated(tag("MASTER"), end_of_word),
                                terminated(tag("MASTER_CONNECT_RETRY"), end_of_word),
                                terminated(tag("MASTER_HOST"), end_of_word),
                                terminated(tag("MASTER_LOG_FILE"), end_of_word),
                                terminated(tag("MATCH"), end_of_word),
                                terminated(tag("MAX_CONNECTIONS_PER_HOUR"), end_of_word),
                                terminated(tag("MAX_QUERIES_PER_HOUR"), end_of_word),
                                terminated(tag("MAX_ROWS"), end_of_word),
                                alt((
                                    terminated(tag("MAX_UPDATES_PER_HOUR"), end_of_word),
                                    terminated(tag("MAX_USER_CONNECTIONS"), end_of_word),
                                    terminated(tag("MEDIUM"), end_of_word),
                                    terminated(tag("MERGE"), end_of_word),
                                    terminated(tag("MINUTE"), end_of_word),
                                    terminated(tag("MINUTE_SECOND"), end_of_word),
                                    terminated(tag("MIN_ROWS"), end_of_word),
                                    terminated(tag("MODE"), end_of_word),
                                    terminated(tag("MODIFY"), end_of_word),
                                    terminated(tag("MONTH"), end_of_word),
                                    terminated(tag("MRG_MYISAM"), end_of_word),
                                    terminated(tag("MYISAM"), end_of_word),
                                    terminated(tag("NAMES"), end_of_word),
                                    terminated(tag("NATURAL"), end_of_word),
                                    terminated(tag("NOT"), end_of_word),
                                    terminated(tag("NOW()"), end_of_word),
                                    terminated(tag("NULL"), end_of_word),
                                    terminated(tag("OFFSET"), end_of_word),
                                    alt((
                                        terminated(tag("ON"), end_of_word),
                                        terminated(tag("ONLY"), end_of_word),
                                        terminated(tag("OPEN"), end_of_word),
                                        terminated(tag("OPTIMIZE"), end_of_word),
                                        terminated(tag("OPTION"), end_of_word),
                                        terminated(tag("OPTIONALLY"), end_of_word),
                                        terminated(tag("OUTFILE"), end_of_word),
                                        terminated(tag("PACK_KEYS"), end_of_word),
                                        terminated(tag("PAGE"), end_of_word),
                                        terminated(tag("PARTIAL"), end_of_word),
                                        terminated(tag("PARTITION"), end_of_word),
                                        terminated(tag("PARTITIONS"), end_of_word),
                                        terminated(tag("PASSWORD"), end_of_word),
                                        terminated(tag("PRIMARY"), end_of_word),
                                        terminated(tag("PRIVILEGES"), end_of_word),
                                        terminated(tag("PROCEDURE"), end_of_word),
                                        terminated(tag("PROCESS"), end_of_word),
                                        terminated(tag("PROCESSLIST"), end_of_word),
                                        terminated(tag("PURGE"), end_of_word),
                                        terminated(tag("QUICK"), end_of_word),
                                        alt((
                                            terminated(tag("RAID0"), end_of_word),
                                            terminated(tag("RAID_CHUNKS"), end_of_word),
                                            terminated(tag("RAID_CHUNKSIZE"), end_of_word),
                                            terminated(tag("RAID_TYPE"), end_of_word),
                                            terminated(tag("RANGE"), end_of_word),
                                            terminated(tag("READ"), end_of_word),
                                            terminated(tag("READ_ONLY"), end_of_word),
                                            terminated(tag("READ_WRITE"), end_of_word),
                                            terminated(tag("REFERENCES"), end_of_word),
                                            terminated(tag("REGEXP"), end_of_word),
                                            terminated(tag("RELOAD"), end_of_word),
                                            terminated(tag("RENAME"), end_of_word),
                                            terminated(tag("REPAIR"), end_of_word),
                                            terminated(tag("REPEATABLE"), end_of_word),
                                            terminated(tag("REPLACE"), end_of_word),
                                            terminated(tag("REPLICATION"), end_of_word),
                                            terminated(tag("RESET"), end_of_word),
                                            terminated(tag("RESTORE"), end_of_word),
                                            terminated(tag("RESTRICT"), end_of_word),
                                            terminated(tag("RETURN"), end_of_word),
                                            alt((
                                                terminated(tag("RETURNS"), end_of_word),
                                                terminated(tag("REVOKE"), end_of_word),
                                                terminated(tag("RLIKE"), end_of_word),
                                                terminated(tag("ROLLBACK"), end_of_word),
                                                terminated(tag("ROW"), end_of_word),
                                                terminated(tag("ROWS"), end_of_word),
                                                terminated(tag("ROW_FORMAT"), end_of_word),
                                                terminated(tag("SECOND"), end_of_word),
                                                terminated(tag("SECURITY"), end_of_word),
                                                terminated(tag("SEPARATOR"), end_of_word),
                                                terminated(tag("SERIALIZABLE"), end_of_word),
                                                terminated(tag("SESSION"), end_of_word),
                                                terminated(tag("SHARE"), end_of_word),
                                                terminated(tag("SHOW"), end_of_word),
                                                terminated(tag("SHUTDOWN"), end_of_word),
                                                terminated(tag("SLAVE"), end_of_word),
                                                terminated(tag("SONAME"), end_of_word),
                                                terminated(tag("SOUNDS"), end_of_word),
                                                terminated(tag("SQL"), end_of_word),
                                                terminated(tag("SQL_AUTO_IS_NULL"), end_of_word),
                                                alt((
                                                    terminated(tag("SQL_BIG_RESULT"), end_of_word),
                                                    terminated(tag("SQL_BIG_SELECTS"), end_of_word),
                                                    terminated(tag("SQL_BIG_TABLES"), end_of_word),
                                                    terminated(
                                                        tag("SQL_BUFFER_RESULT"),
                                                        end_of_word,
                                                    ),
                                                    terminated(tag("SQL_CACHE"), end_of_word),
                                                    terminated(
                                                        tag("SQL_CALC_FOUND_ROWS"),
                                                        end_of_word,
                                                    ),
                                                    terminated(tag("SQL_LOG_BIN"), end_of_word),
                                                    terminated(tag("SQL_LOG_OFF"), end_of_word),
                                                    terminated(tag("SQL_LOG_UPDATE"), end_of_word),
                                                    terminated(
                                                        tag("SQL_LOW_PRIORITY_UPDATES"),
                                                        end_of_word,
                                                    ),
                                                    terminated(
                                                        tag("SQL_MAX_JOIN_SIZE"),
                                                        end_of_word,
                                                    ),
                                                    terminated(tag("SQL_NO_CACHE"), end_of_word),
                                                    terminated(
                                                        tag("SQL_QUOTE_SHOW_CREATE"),
                                                        end_of_word,
                                                    ),
                                                    terminated(
                                                        tag("SQL_SAFE_UPDATES"),
                                                        end_of_word,
                                                    ),
                                                    terminated(
                                                        tag("SQL_SELECT_LIMIT"),
                                                        end_of_word,
                                                    ),
                                                    terminated(
                                                        tag("SQL_SLAVE_SKIP_COUNTER"),
                                                        end_of_word,
                                                    ),
                                                    terminated(
                                                        tag("SQL_SMALL_RESULT"),
                                                        end_of_word,
                                                    ),
                                                    terminated(tag("SQL_WARNINGS"), end_of_word),
                                                    terminated(tag("START"), end_of_word),
                                                    terminated(tag("STARTING"), end_of_word),
                                                    alt((
                                                        terminated(tag("STATUS"), end_of_word),
                                                        terminated(tag("STOP"), end_of_word),
                                                        terminated(tag("STORAGE"), end_of_word),
                                                        terminated(
                                                            tag("STRAIGHT_JOIN"),
                                                            end_of_word,
                                                        ),
                                                        terminated(tag("STRING"), end_of_word),
                                                        terminated(tag("STRIPED"), end_of_word),
                                                        terminated(tag("SUPER"), end_of_word),
                                                        terminated(tag("TABLE"), end_of_word),
                                                        terminated(tag("TABLES"), end_of_word),
                                                        terminated(tag("TEMPORARY"), end_of_word),
                                                        terminated(tag("TERMINATED"), end_of_word),
                                                        terminated(tag("THEN"), end_of_word),
                                                        terminated(tag("TO"), end_of_word),
                                                        terminated(tag("TRAILING"), end_of_word),
                                                        terminated(
                                                            tag("TRANSACTIONAL"),
                                                            end_of_word,
                                                        ),
                                                        terminated(tag("TRUE"), end_of_word),
                                                        terminated(tag("TRUNCATE"), end_of_word),
                                                        terminated(tag("TYPE"), end_of_word),
                                                        terminated(tag("TYPES"), end_of_word),
                                                        terminated(tag("UNCOMMITTED"), end_of_word),
                                                        alt((
                                                            terminated(tag("UNIQUE"), end_of_word),
                                                            terminated(tag("UNLOCK"), end_of_word),
                                                            terminated(
                                                                tag("UNSIGNED"),
                                                                end_of_word,
                                                            ),
                                                            terminated(tag("USAGE"), end_of_word),
                                                            terminated(tag("USE"), end_of_word),
                                                            terminated(tag("USING"), end_of_word),
                                                            terminated(
                                                                tag("VARIABLES"),
                                                                end_of_word,
                                                            ),
                                                            terminated(tag("VIEW"), end_of_word),
                                                            terminated(tag("WHEN"), end_of_word),
                                                            terminated(tag("WITH"), end_of_word),
                                                            terminated(tag("WORK"), end_of_word),
                                                            terminated(tag("WRITE"), end_of_word),
                                                            terminated(
                                                                tag("YEAR_MONTH"),
                                                                end_of_word,
                                                            ),
                                                        )),
                                                    )),
                                                )),
                                            )),
                                        )),
                                    )),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    ))(&uc_input);
    if let Ok((_, token)) = result {
        let input_end_pos = token.len();
        let (token, input) = input.split_at(input_end_pos);
        Ok((
            input,
            Token {
                kind: TokenKind::Reserved,
                value: token,
                key: None,
            },
        ))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Alt)))
    }
}

fn get_plain_reserved_two_token(input: &str) -> IResult<&str, Token<'_>> {
    let uc_input = get_uc_words(input, 2);
    let result: IResult<&str, &str> = alt((
        terminated(tag("CHARACTER SET"), end_of_word),
        terminated(tag("ON DELETE"), end_of_word),
        terminated(tag("ON UPDATE"), end_of_word),
    ))(&uc_input);
    if let Ok((_, token)) = result {
        let final_word = token.split(' ').last().unwrap();
        let input_end_pos = input.to_ascii_uppercase().find(final_word).unwrap() + final_word.len();
        let (token, input) = input.split_at(input_end_pos);
        Ok((
            input,
            Token {
                kind: TokenKind::Reserved,
                value: token,
                key: None,
            },
        ))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Alt)))
    }
}

fn get_word_token(input: &str) -> IResult<&str, Token<'_>> {
    take_while1(is_word_character)(input).map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::Word,
                value: token,
                key: None,
            },
        )
    })
}

fn get_operator_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((
        tag("!="),
        tag("<>"),
        tag("=="),
        tag("<="),
        tag(">="),
        tag("!<"),
        tag("!>"),
        tag("||"),
        tag("::"),
        tag("->>"),
        tag("->"),
        tag("~~*"),
        tag("~~"),
        tag("!~~*"),
        tag("!~~"),
        tag("~*"),
        tag("!~*"),
        tag("!~"),
        tag(":="),
        recognize(verify(take(1usize), |token: &str| {
            token != "\n" && token != "\r"
        })),
    ))(input)
    .map(|(input, token)| {
        (
            input,
            Token {
                kind: TokenKind::Operator,
                value: token,
                key: None,
            },
        )
    })
}

fn end_of_word(input: &str) -> IResult<&str, &str> {
    peek(alt((
        eof,
        verify(take(1usize), |val: &str| {
            !is_word_character(val.chars().next().unwrap())
        }),
    )))(input)
}

fn is_word_character(item: char) -> bool {
    item.is_alphanumeric() || item.is_mark() || item.is_punctuation_connector()
}
