use anyhow::anyhow;

use collections::{HashMap, HashSet};
use tree_sitter::{Language, Node, Parser};

extern "C" {
    fn tree_sitter_context_predicate() -> Language;
}

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
    pub fn parse(source: &str) -> anyhow::Result<Self> {
        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_context_predicate() };
        parser.set_language(language).unwrap();
        let source = source.as_bytes();
        let tree = parser.parse(source, None).unwrap();
        Self::from_node(tree.root_node(), source)
    }

    fn from_node(node: Node, source: &[u8]) -> anyhow::Result<Self> {
        let parse_error = "error parsing context predicate";
        let kind = node.kind();

        match kind {
            "source" => Self::from_node(node.child(0).ok_or_else(|| anyhow!(parse_error))?, source),
            "identifier" => Ok(Self::Identifier(node.utf8_text(source)?.into())),
            "not" => {
                let child = Self::from_node(
                    node.child_by_field_name("expression")
                        .ok_or_else(|| anyhow!(parse_error))?,
                    source,
                )?;
                Ok(Self::Not(Box::new(child)))
            }
            "and" | "or" => {
                let left = Box::new(Self::from_node(
                    node.child_by_field_name("left")
                        .ok_or_else(|| anyhow!(parse_error))?,
                    source,
                )?);
                let right = Box::new(Self::from_node(
                    node.child_by_field_name("right")
                        .ok_or_else(|| anyhow!(parse_error))?,
                    source,
                )?);
                if kind == "and" {
                    Ok(Self::And(left, right))
                } else {
                    Ok(Self::Or(left, right))
                }
            }
            "equal" | "not_equal" => {
                let left = node
                    .child_by_field_name("left")
                    .ok_or_else(|| anyhow!(parse_error))?
                    .utf8_text(source)?
                    .into();
                let right = node
                    .child_by_field_name("right")
                    .ok_or_else(|| anyhow!(parse_error))?
                    .utf8_text(source)?
                    .into();
                if kind == "equal" {
                    Ok(Self::Equal(left, right))
                } else {
                    Ok(Self::NotEqual(left, right))
                }
            }
            "parenthesized" => Self::from_node(
                node.child_by_field_name("expression")
                    .ok_or_else(|| anyhow!(parse_error))?,
                source,
            ),
            _ => Err(anyhow!(parse_error)),
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
}
