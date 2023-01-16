use anyhow::{anyhow, Result};
use collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KeymapContext {
    pub set: HashSet<String>,
    pub map: HashMap<String, String>,
}

impl KeymapContext {
    pub fn extend(&mut self, other: &Self) {
        for v in &other.set {
            self.set.insert(v.clone());
        }
        for (k, v) in &other.map {
            self.map.insert(k.clone(), v.clone());
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum KeymapContextPredicate {
    Identifier(String),
    Equal(String, String),
    NotEqual(String, String),
    Not(Box<KeymapContextPredicate>),
    And(Box<KeymapContextPredicate>, Box<KeymapContextPredicate>),
    Or(Box<KeymapContextPredicate>, Box<KeymapContextPredicate>),
}

impl KeymapContextPredicate {
    pub fn parse(source: &str) -> Result<Self> {
        let source = Self::skip_whitespace(source);
        let (predicate, rest) = Self::parse_expr(source, 0)?;
        if let Some(next) = rest.chars().next() {
            Err(anyhow!("unexpected character {next:?}"))
        } else {
            Ok(predicate)
        }
    }

    pub fn eval(&self, context: &KeymapContext) -> bool {
        match self {
            Self::Identifier(name) => context.set.contains(name.as_str()),
            Self::Equal(left, right) => context
                .map
                .get(left)
                .map(|value| value == right)
                .unwrap_or(false),
            Self::NotEqual(left, right) => context
                .map
                .get(left)
                .map(|value| value != right)
                .unwrap_or(true),
            Self::Not(pred) => !pred.eval(context),
            Self::And(left, right) => left.eval(context) && right.eval(context),
            Self::Or(left, right) => left.eval(context) || right.eval(context),
        }
    }

    fn parse_expr(
        mut source: &str,
        min_precedence: u32,
    ) -> anyhow::Result<(KeymapContextPredicate, &str)> {
        type Op =
            fn(KeymapContextPredicate, KeymapContextPredicate) -> Result<KeymapContextPredicate>;

        let (mut predicate, rest) = Self::parse_primary(source)?;
        source = rest;

        'parse: loop {
            for (operator, precedence, constructor) in [
                ("&&", PRECEDENCE_AND, KeymapContextPredicate::new_and as Op),
                ("||", PRECEDENCE_OR, KeymapContextPredicate::new_or as Op),
                ("==", PRECEDENCE_EQ, KeymapContextPredicate::new_eq as Op),
                ("!=", PRECEDENCE_EQ, KeymapContextPredicate::new_neq as Op),
            ] {
                if source.starts_with(operator) && precedence >= min_precedence {
                    source = Self::skip_whitespace(&source[operator.len()..]);
                    let (right, rest) = Self::parse_expr(source, precedence + 1)?;
                    predicate = constructor(predicate, right)?;
                    source = rest;
                    continue 'parse;
                }
            }
            break;
        }

        Ok((predicate, source))
    }

    fn parse_primary(mut source: &str) -> anyhow::Result<(KeymapContextPredicate, &str)> {
        let next = source
            .chars()
            .next()
            .ok_or_else(|| anyhow!("unexpected eof"))?;
        match next {
            '(' => {
                source = Self::skip_whitespace(&source[1..]);
                let (predicate, rest) = Self::parse_expr(source, 0)?;
                if rest.starts_with(')') {
                    source = Self::skip_whitespace(&rest[1..]);
                    Ok((predicate, source))
                } else {
                    Err(anyhow!("expected a ')'"))
                }
            }
            '!' => {
                let source = Self::skip_whitespace(&source[1..]);
                let (predicate, source) = Self::parse_expr(&source, PRECEDENCE_NOT)?;
                Ok((KeymapContextPredicate::Not(Box::new(predicate)), source))
            }
            _ if next.is_alphanumeric() || next == '_' => {
                let len = source
                    .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .unwrap_or(source.len());
                let (identifier, rest) = source.split_at(len);
                source = Self::skip_whitespace(rest);
                Ok((
                    KeymapContextPredicate::Identifier(identifier.into()),
                    source,
                ))
            }
            _ => Err(anyhow!("unexpected character {next:?}")),
        }
    }

    fn skip_whitespace(source: &str) -> &str {
        let len = source
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(source.len());
        &source[len..]
    }

    fn new_or(self, other: Self) -> Result<Self> {
        Ok(Self::Or(Box::new(self), Box::new(other)))
    }

    fn new_and(self, other: Self) -> Result<Self> {
        Ok(Self::And(Box::new(self), Box::new(other)))
    }

    fn new_eq(self, other: Self) -> Result<Self> {
        if let (Self::Identifier(left), Self::Identifier(right)) = (self, other) {
            Ok(Self::Equal(left, right))
        } else {
            Err(anyhow!("operands must be identifiers"))
        }
    }

    fn new_neq(self, other: Self) -> Result<Self> {
        if let (Self::Identifier(left), Self::Identifier(right)) = (self, other) {
            Ok(Self::NotEqual(left, right))
        } else {
            Err(anyhow!("operands must be identifiers"))
        }
    }
}

const PRECEDENCE_OR: u32 = 1;
const PRECEDENCE_AND: u32 = 2;
const PRECEDENCE_EQ: u32 = 3;
const PRECEDENCE_NOT: u32 = 4;

#[cfg(test)]
mod tests {
    use super::KeymapContextPredicate::{self, *};

    #[test]
    fn test_parse_identifiers() {
        // Identifiers
        assert_eq!(
            KeymapContextPredicate::parse("abc12").unwrap(),
            Identifier("abc12".into())
        );
        assert_eq!(
            KeymapContextPredicate::parse("_1a").unwrap(),
            Identifier("_1a".into())
        );
    }

    #[test]
    fn test_parse_negations() {
        assert_eq!(
            KeymapContextPredicate::parse("!abc").unwrap(),
            Not(Box::new(Identifier("abc".into())))
        );
        assert_eq!(
            KeymapContextPredicate::parse(" ! ! abc").unwrap(),
            Not(Box::new(Not(Box::new(Identifier("abc".into())))))
        );
    }

    #[test]
    fn test_parse_equality_operators() {
        assert_eq!(
            KeymapContextPredicate::parse("a == b").unwrap(),
            Equal("a".into(), "b".into())
        );
        assert_eq!(
            KeymapContextPredicate::parse("c!=d").unwrap(),
            NotEqual("c".into(), "d".into())
        );
        assert_eq!(
            KeymapContextPredicate::parse("c == !d")
                .unwrap_err()
                .to_string(),
            "operands must be identifiers"
        );
    }

    #[test]
    fn test_parse_boolean_operators() {
        assert_eq!(
            KeymapContextPredicate::parse("a || b").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(Identifier("b".into()))
            )
        );
        assert_eq!(
            KeymapContextPredicate::parse("a || !b && c").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(And(
                    Box::new(Not(Box::new(Identifier("b".into())))),
                    Box::new(Identifier("c".into()))
                ))
            )
        );
        assert_eq!(
            KeymapContextPredicate::parse("a && b || c&&d").unwrap(),
            Or(
                Box::new(And(
                    Box::new(Identifier("a".into())),
                    Box::new(Identifier("b".into()))
                )),
                Box::new(And(
                    Box::new(Identifier("c".into())),
                    Box::new(Identifier("d".into()))
                ))
            )
        );
        assert_eq!(
            KeymapContextPredicate::parse("a == b && c || d == e && f").unwrap(),
            Or(
                Box::new(And(
                    Box::new(Equal("a".into(), "b".into())),
                    Box::new(Identifier("c".into()))
                )),
                Box::new(And(
                    Box::new(Equal("d".into(), "e".into())),
                    Box::new(Identifier("f".into()))
                ))
            )
        );
        assert_eq!(
            KeymapContextPredicate::parse("a && b && c && d").unwrap(),
            And(
                Box::new(And(
                    Box::new(And(
                        Box::new(Identifier("a".into())),
                        Box::new(Identifier("b".into()))
                    )),
                    Box::new(Identifier("c".into())),
                )),
                Box::new(Identifier("d".into()))
            ),
        );
    }

    #[test]
    fn test_parse_parenthesized_expressions() {
        assert_eq!(
            KeymapContextPredicate::parse("a && (b == c || d != e)").unwrap(),
            And(
                Box::new(Identifier("a".into())),
                Box::new(Or(
                    Box::new(Equal("b".into(), "c".into())),
                    Box::new(NotEqual("d".into(), "e".into())),
                )),
            ),
        );
        assert_eq!(
            KeymapContextPredicate::parse(" ( a || b ) ").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(Identifier("b".into())),
            )
        );
    }
}
