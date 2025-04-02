use crate::SharedString;
use anyhow::{Result, anyhow};
use std::fmt;

/// A datastructure for resolving whether an action should be dispatched
/// at this point in the element tree. Contains a set of identifiers
/// and/or key value pairs representing the current context for the
/// keymap.
#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct KeyContext(Vec<ContextEntry>);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// An entry in a KeyContext
pub struct ContextEntry {
    /// The key (or name if no value)
    pub key: SharedString,
    /// The value
    pub value: Option<SharedString>,
}

impl<'a> TryFrom<&'a str> for KeyContext {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Self::parse(value)
    }
}

impl KeyContext {
    /// Initialize a new [`KeyContext`] that contains an `os` key set to either `macos`, `linux`, `windows` or `unknown`.
    pub fn new_with_defaults() -> Self {
        let mut context = Self::default();
        #[cfg(target_os = "macos")]
        context.set("os", "macos");
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        context.set("os", "linux");
        #[cfg(target_os = "windows")]
        context.set("os", "windows");
        #[cfg(not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "freebsd",
            target_os = "windows"
        )))]
        context.set("os", "unknown");
        context
    }

    /// Returns the primary context entry (usually the name of the component)
    pub fn primary(&self) -> Option<&ContextEntry> {
        self.0.iter().find(|p| p.value.is_none())
    }

    /// Returns everything except the primary context entry.
    pub fn secondary(&self) -> impl Iterator<Item = &ContextEntry> {
        let primary = self.primary();
        self.0.iter().filter(move |&p| Some(p) != primary)
    }

    /// Parse a key context from a string.
    /// The key context format is very simple:
    /// - either a single identifier, such as `StatusBar`
    /// - or a key value pair, such as `mode = visible`
    /// - separated by whitespace, such as `StatusBar mode = visible`
    pub fn parse(source: &str) -> Result<Self> {
        let mut context = Self::default();
        let source = skip_whitespace(source);
        Self::parse_expr(source, &mut context)?;
        Ok(context)
    }

    fn parse_expr(mut source: &str, context: &mut Self) -> Result<()> {
        if source.is_empty() {
            return Ok(());
        }

        let key = source
            .chars()
            .take_while(|c| is_identifier_char(*c))
            .collect::<String>();
        source = skip_whitespace(&source[key.len()..]);
        if let Some(suffix) = source.strip_prefix('=') {
            source = skip_whitespace(suffix);
            let value = source
                .chars()
                .take_while(|c| is_identifier_char(*c))
                .collect::<String>();
            source = skip_whitespace(&source[value.len()..]);
            context.set(key, value);
        } else {
            context.add(key);
        }

        Self::parse_expr(source, context)
    }

    /// Check if this context is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clear this context.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Extend this context with another context.
    pub fn extend(&mut self, other: &Self) {
        for entry in &other.0 {
            if !self.contains(&entry.key) {
                self.0.push(entry.clone());
            }
        }
    }

    /// Add an identifier to this context, if it's not already in this context.
    pub fn add<I: Into<SharedString>>(&mut self, identifier: I) {
        let key = identifier.into();

        if !self.contains(&key) {
            self.0.push(ContextEntry { key, value: None })
        }
    }

    /// Set a key value pair in this context, if it's not already set.
    pub fn set<S1: Into<SharedString>, S2: Into<SharedString>>(&mut self, key: S1, value: S2) {
        let key = key.into();
        if !self.contains(&key) {
            self.0.push(ContextEntry {
                key,
                value: Some(value.into()),
            })
        }
    }

    /// Check if this context contains a given identifier or key.
    pub fn contains(&self, key: &str) -> bool {
        self.0.iter().any(|entry| entry.key.as_ref() == key)
    }

    /// Get the associated value for a given identifier or key.
    pub fn get(&self, key: &str) -> Option<&SharedString> {
        self.0
            .iter()
            .find(|entry| entry.key.as_ref() == key)?
            .value
            .as_ref()
    }
}

impl fmt::Debug for KeyContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entries = self.0.iter().peekable();
        while let Some(entry) = entries.next() {
            if let Some(ref value) = entry.value {
                write!(f, "{}={}", entry.key, value)?;
            } else {
                write!(f, "{}", entry.key)?;
            }
            if entries.peek().is_some() {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

/// A datastructure for resolving whether an action should be dispatched
/// Representing a small language for describing which contexts correspond
/// to which actions.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum KeyBindingContextPredicate {
    /// A predicate that will match a given identifier.
    Identifier(SharedString),
    /// A predicate that will match a given key-value pair.
    Equal(SharedString, SharedString),
    /// A predicate that will match a given key-value pair not being present.
    NotEqual(SharedString, SharedString),
    /// A predicate that will match a given predicate appearing below another predicate.
    /// in the element tree
    Child(
        Box<KeyBindingContextPredicate>,
        Box<KeyBindingContextPredicate>,
    ),
    /// Predicate that will invert another predicate.
    Not(Box<KeyBindingContextPredicate>),
    /// A predicate that will match if both of its children match.
    And(
        Box<KeyBindingContextPredicate>,
        Box<KeyBindingContextPredicate>,
    ),
    /// A predicate that will match if either of its children match.
    Or(
        Box<KeyBindingContextPredicate>,
        Box<KeyBindingContextPredicate>,
    ),
}

impl fmt::Display for KeyBindingContextPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "{}", name),
            Self::Equal(left, right) => write!(f, "{} == {}", left, right),
            Self::NotEqual(left, right) => write!(f, "{} != {}", left, right),
            Self::Not(pred) => write!(f, "!{}", pred),
            Self::Child(parent, child) => write!(f, "{} > {}", parent, child),
            Self::And(left, right) => write!(f, "({} && {})", left, right),
            Self::Or(left, right) => write!(f, "({} || {})", left, right),
        }
    }
}

impl KeyBindingContextPredicate {
    /// Parse a string in the same format as the keymap's context field.
    ///
    /// A basic equivalence check against a set of identifiers can performed by
    /// simply writing a string:
    ///
    /// `StatusBar` -> A predicate that will match a context with the identifier `StatusBar`
    ///
    /// You can also specify a key-value pair:
    ///
    /// `mode == visible` -> A predicate that will match a context with the key `mode`
    ///                      with the value `visible`
    ///
    /// And a logical operations combining these two checks:
    ///
    /// `StatusBar && mode == visible` -> A predicate that will match a context with the
    ///                                   identifier `StatusBar` and the key `mode`
    ///                                   with the value `visible`
    ///
    ///
    /// There is also a special child `>` operator that will match a predicate that is
    /// below another predicate:
    ///
    /// `StatusBar > mode == visible` -> A predicate that will match a context identifier `StatusBar`
    ///                                  and a child context that has the key `mode` with the
    ///                                  value `visible`
    ///
    /// This syntax supports `!=`, `||` and `&&` as logical operators.
    /// You can also preface an operation or check with a `!` to negate it.
    pub fn parse(source: &str) -> Result<Self> {
        let source = skip_whitespace(source);
        let (predicate, rest) = Self::parse_expr(source, 0)?;
        if let Some(next) = rest.chars().next() {
            Err(anyhow!("unexpected character '{next:?}'"))
        } else {
            Ok(predicate)
        }
    }

    /// Eval a predicate against a set of contexts, arranged from lowest to highest.
    pub fn eval(&self, contexts: &[KeyContext]) -> bool {
        let Some(context) = contexts.last() else {
            return false;
        };
        match self {
            Self::Identifier(name) => context.contains(name),
            Self::Equal(left, right) => context
                .get(left)
                .map(|value| value == right)
                .unwrap_or(false),
            Self::NotEqual(left, right) => context
                .get(left)
                .map(|value| value != right)
                .unwrap_or(true),
            Self::Not(pred) => !pred.eval(contexts),
            Self::Child(parent, child) => {
                parent.eval(&contexts[..contexts.len() - 1]) && child.eval(contexts)
            }
            Self::And(left, right) => left.eval(contexts) && right.eval(contexts),
            Self::Or(left, right) => left.eval(contexts) || right.eval(contexts),
        }
    }

    /// Returns whether or not this predicate matches all possible contexts matched by
    /// the other predicate.
    pub fn is_superset(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        if let KeyBindingContextPredicate::Or(left, right) = self {
            return left.is_superset(other) || right.is_superset(other);
        }

        match other {
            KeyBindingContextPredicate::Child(_, child) => self.is_superset(child),
            KeyBindingContextPredicate::And(left, right) => {
                self.is_superset(left) || self.is_superset(right)
            }
            KeyBindingContextPredicate::Identifier(_) => false,
            KeyBindingContextPredicate::Equal(_, _) => false,
            KeyBindingContextPredicate::NotEqual(_, _) => false,
            KeyBindingContextPredicate::Not(_) => false,
            KeyBindingContextPredicate::Or(_, _) => false,
        }
    }

    fn parse_expr(mut source: &str, min_precedence: u32) -> anyhow::Result<(Self, &str)> {
        type Op = fn(
            KeyBindingContextPredicate,
            KeyBindingContextPredicate,
        ) -> Result<KeyBindingContextPredicate>;

        let (mut predicate, rest) = Self::parse_primary(source)?;
        source = rest;

        'parse: loop {
            for (operator, precedence, constructor) in [
                (">", PRECEDENCE_CHILD, Self::new_child as Op),
                ("&&", PRECEDENCE_AND, Self::new_and as Op),
                ("||", PRECEDENCE_OR, Self::new_or as Op),
                ("==", PRECEDENCE_EQ, Self::new_eq as Op),
                ("!=", PRECEDENCE_EQ, Self::new_neq as Op),
            ] {
                if source.starts_with(operator) && precedence >= min_precedence {
                    source = skip_whitespace(&source[operator.len()..]);
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

    fn parse_primary(mut source: &str) -> anyhow::Result<(Self, &str)> {
        let next = source
            .chars()
            .next()
            .ok_or_else(|| anyhow!("unexpected end"))?;
        match next {
            '(' => {
                source = skip_whitespace(&source[1..]);
                let (predicate, rest) = Self::parse_expr(source, 0)?;
                if let Some(stripped) = rest.strip_prefix(')') {
                    source = skip_whitespace(stripped);
                    Ok((predicate, source))
                } else {
                    Err(anyhow!("expected a ')'"))
                }
            }
            '!' => {
                let source = skip_whitespace(&source[1..]);
                let (predicate, source) = Self::parse_expr(source, PRECEDENCE_NOT)?;
                Ok((KeyBindingContextPredicate::Not(Box::new(predicate)), source))
            }
            _ if is_identifier_char(next) => {
                let len = source
                    .find(|c: char| !is_identifier_char(c) && !is_vim_operator_char(c))
                    .unwrap_or(source.len());
                let (identifier, rest) = source.split_at(len);
                source = skip_whitespace(rest);
                Ok((
                    KeyBindingContextPredicate::Identifier(identifier.to_string().into()),
                    source,
                ))
            }
            _ if is_vim_operator_char(next) => {
                let (operator, rest) = source.split_at(1);
                source = skip_whitespace(rest);
                Ok((
                    KeyBindingContextPredicate::Identifier(operator.to_string().into()),
                    source,
                ))
            }
            _ => Err(anyhow!("unexpected character '{next:?}'")),
        }
    }

    fn new_or(self, other: Self) -> Result<Self> {
        Ok(Self::Or(Box::new(self), Box::new(other)))
    }

    fn new_and(self, other: Self) -> Result<Self> {
        Ok(Self::And(Box::new(self), Box::new(other)))
    }

    fn new_child(self, other: Self) -> Result<Self> {
        Ok(Self::Child(Box::new(self), Box::new(other)))
    }

    fn new_eq(self, other: Self) -> Result<Self> {
        if let (Self::Identifier(left), Self::Identifier(right)) = (self, other) {
            Ok(Self::Equal(left, right))
        } else {
            Err(anyhow!("operands of == must be identifiers"))
        }
    }

    fn new_neq(self, other: Self) -> Result<Self> {
        if let (Self::Identifier(left), Self::Identifier(right)) = (self, other) {
            Ok(Self::NotEqual(left, right))
        } else {
            Err(anyhow!("operands of != must be identifiers"))
        }
    }
}

const PRECEDENCE_CHILD: u32 = 1;
const PRECEDENCE_OR: u32 = 2;
const PRECEDENCE_AND: u32 = 3;
const PRECEDENCE_EQ: u32 = 4;
const PRECEDENCE_NOT: u32 = 5;

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn is_vim_operator_char(c: char) -> bool {
    c == '>' || c == '<' || c == '~' || c == '"' || c == '?'
}

fn skip_whitespace(source: &str) -> &str {
    let len = source
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(source.len());
    &source[len..]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as gpui;
    use KeyBindingContextPredicate::*;

    #[test]
    fn test_actions_definition() {
        {
            actions!(test, [A, B, C, D, E, F, G]);
        }

        {
            actions!(
                test,
                [
                    A, B, C, D, E, F, G, // Don't wrap, test the trailing comma
                ]
            );
        }
    }

    #[test]
    fn test_parse_context() {
        let mut expected = KeyContext::default();
        expected.add("baz");
        expected.set("foo", "bar");
        assert_eq!(KeyContext::parse("baz foo=bar").unwrap(), expected);
        assert_eq!(KeyContext::parse("baz foo = bar").unwrap(), expected);
        assert_eq!(
            KeyContext::parse("  baz foo   =   bar baz").unwrap(),
            expected
        );
        assert_eq!(KeyContext::parse(" baz foo = bar").unwrap(), expected);
    }

    #[test]
    fn test_parse_identifiers() {
        // Identifiers
        assert_eq!(
            KeyBindingContextPredicate::parse("abc12").unwrap(),
            Identifier("abc12".into())
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("_1a").unwrap(),
            Identifier("_1a".into())
        );
    }

    #[test]
    fn test_parse_negations() {
        assert_eq!(
            KeyBindingContextPredicate::parse("!abc").unwrap(),
            Not(Box::new(Identifier("abc".into())))
        );
        assert_eq!(
            KeyBindingContextPredicate::parse(" ! ! abc").unwrap(),
            Not(Box::new(Not(Box::new(Identifier("abc".into())))))
        );
    }

    #[test]
    fn test_parse_equality_operators() {
        assert_eq!(
            KeyBindingContextPredicate::parse("a == b").unwrap(),
            Equal("a".into(), "b".into())
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("c!=d").unwrap(),
            NotEqual("c".into(), "d".into())
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("c == !d")
                .unwrap_err()
                .to_string(),
            "operands of == must be identifiers"
        );
    }

    #[test]
    fn test_parse_boolean_operators() {
        assert_eq!(
            KeyBindingContextPredicate::parse("a || b").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(Identifier("b".into()))
            )
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("a || !b && c").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(And(
                    Box::new(Not(Box::new(Identifier("b".into())))),
                    Box::new(Identifier("c".into()))
                ))
            )
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("a && b || c&&d").unwrap(),
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
            KeyBindingContextPredicate::parse("a == b && c || d == e && f").unwrap(),
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
            KeyBindingContextPredicate::parse("a && b && c && d").unwrap(),
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
            KeyBindingContextPredicate::parse("a && (b == c || d != e)").unwrap(),
            And(
                Box::new(Identifier("a".into())),
                Box::new(Or(
                    Box::new(Equal("b".into(), "c".into())),
                    Box::new(NotEqual("d".into(), "e".into())),
                )),
            ),
        );
        assert_eq!(
            KeyBindingContextPredicate::parse(" ( a || b ) ").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(Identifier("b".into())),
            )
        );
    }

    #[test]
    fn test_is_superset() {
        assert_is_superset("editor", "editor", true);
        assert_is_superset("editor", "workspace", false);

        assert_is_superset("editor", "editor && vim_mode", true);
        assert_is_superset("editor", "mode == full && editor", true);
        assert_is_superset("editor && mode == full", "editor", false);

        assert_is_superset("editor", "something > editor", true);
        assert_is_superset("editor", "editor > menu", false);

        assert_is_superset("foo || bar || baz", "bar", true);
        assert_is_superset("foo || bar || baz", "quux", false);

        #[track_caller]
        fn assert_is_superset(a: &str, b: &str, result: bool) {
            let a = KeyBindingContextPredicate::parse(a).unwrap();
            let b = KeyBindingContextPredicate::parse(b).unwrap();
            assert_eq!(a.is_superset(&b), result, "({a:?}).is_superset({b:?})");
        }
    }
}
