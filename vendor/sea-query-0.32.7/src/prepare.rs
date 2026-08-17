//! Helper for preparing SQL statements.

use crate::*;
pub use std::fmt::Write;

pub trait SqlWriter: Write + ToString {
    fn push_param(&mut self, value: Value, query_builder: &dyn QueryBuilder);

    fn as_writer(&mut self) -> &mut dyn Write;
}

impl SqlWriter for String {
    fn push_param(&mut self, value: Value, query_builder: &dyn QueryBuilder) {
        self.push_str(&query_builder.value_to_string(&value))
    }

    fn as_writer(&mut self) -> &mut dyn Write {
        self as _
    }
}

#[derive(Debug, Clone)]
pub struct SqlWriterValues {
    counter: usize,
    placeholder: String,
    numbered: bool,
    string: String,
    values: Vec<Value>,
}

impl SqlWriterValues {
    pub fn new<T>(placeholder: T, numbered: bool) -> Self
    where
        T: Into<String>,
    {
        Self {
            counter: 0,
            placeholder: placeholder.into(),
            numbered,
            string: String::with_capacity(256),
            values: Vec::new(),
        }
    }

    pub fn into_parts(self) -> (String, Values) {
        (self.string, Values(self.values))
    }
}

impl Write for SqlWriterValues {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.string, "{s}")
    }
}

impl std::fmt::Display for SqlWriterValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl SqlWriter for SqlWriterValues {
    fn push_param(&mut self, value: Value, _: &dyn QueryBuilder) {
        self.counter += 1;
        if self.numbered {
            let counter = self.counter;
            write!(self.string, "{}{}", self.placeholder, counter).unwrap();
        } else {
            write!(self.string, "{}", self.placeholder).unwrap();
        }
        self.values.push(value)
    }

    fn as_writer(&mut self) -> &mut dyn Write {
        self as _
    }
}

pub fn inject_parameters<I>(sql: &str, params: I, query_builder: &dyn QueryBuilder) -> String
where
    I: IntoIterator<Item = Value>,
{
    let params: Vec<Value> = params.into_iter().collect();
    let tokenizer = Tokenizer::new(sql);
    let tokens: Vec<Token> = tokenizer.iter().collect();
    let mut counter = 0;
    let mut output = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            Token::Punctuation(mark) => {
                if (mark.as_ref(), false) == query_builder.placeholder() {
                    output.push(query_builder.value_to_string(&params[counter]));
                    counter += 1;
                    i += 1;
                    continue;
                } else if (mark.as_ref(), true) == query_builder.placeholder()
                    && i + 1 < tokens.len()
                {
                    if let Token::Unquoted(next) = &tokens[i + 1] {
                        if let Ok(num) = next.parse::<usize>() {
                            output.push(query_builder.value_to_string(&params[num - 1]));
                            i += 2;
                            continue;
                        }
                    }
                }
                output.push(mark.clone())
            }
            _ => output.push(token.to_string()),
        }
        i += 1;
    }
    output.into_iter().collect()
}

#[cfg(test)]
#[cfg(feature = "backend-mysql")]
mod tests_mysql {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn inject_parameters_1() {
        assert_eq!(
            inject_parameters("WHERE A = ?", ["B".into()], &MysqlQueryBuilder),
            "WHERE A = 'B'"
        );
    }

    #[test]
    fn inject_parameters_2() {
        assert_eq!(
            inject_parameters("WHERE A = '?' AND B = ?", ["C".into()], &MysqlQueryBuilder),
            "WHERE A = '?' AND B = 'C'"
        );
    }

    #[test]
    fn inject_parameters_3() {
        assert_eq!(
            inject_parameters(
                "WHERE A = ? AND C = ?",
                ["B".into(), "D".into()],
                &MysqlQueryBuilder
            ),
            "WHERE A = 'B' AND C = 'D'"
        );
    }

    #[test]
    fn inject_parameters_4() {
        assert_eq!(
            inject_parameters("?", [vec![0xABu8, 0xCD, 0xEF].into()], &MysqlQueryBuilder),
            "x'ABCDEF'"
        );
    }
}

#[cfg(test)]
#[cfg(feature = "backend-postgres")]
mod tests_postgres {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn inject_parameters_5() {
        assert_eq!(
            inject_parameters(
                "WHERE A = $1 AND C = $2",
                ["B".into(), "D".into()],
                &PostgresQueryBuilder
            ),
            "WHERE A = 'B' AND C = 'D'"
        );
    }

    #[test]
    fn inject_parameters_6() {
        assert_eq!(
            inject_parameters(
                "WHERE A = $2 AND C = $1",
                ["B".into(), "D".into()],
                &PostgresQueryBuilder
            ),
            "WHERE A = 'D' AND C = 'B'"
        );
    }

    #[test]
    fn inject_parameters_7() {
        assert_eq!(
            inject_parameters("WHERE A = $1", ["B'C".into()], &PostgresQueryBuilder),
            "WHERE A = E'B\\'C'"
        );
    }
}
