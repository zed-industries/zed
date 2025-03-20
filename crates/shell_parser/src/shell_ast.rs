use crate::{Error, ShellParser};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShellCmd {
    pub command: String,
    pub args: Vec<String>,
    pub stdout_redirect: Option<String>,
    pub stderr_redirect: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// The `|` shell operator (highest precedence)
    Pipe,
    /// The `&&` shell operator (medium precedence)
    And,
    /// The `;` shell operator (lowest precedence)
    Semicolon,
}

impl Operator {
    pub(crate) fn precedence(&self) -> u8 {
        match self {
            Operator::Pipe => 3,
            Operator::And => 2,
            Operator::Semicolon => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShellAst {
    Command(ShellCmd),
    Operation {
        operator: Operator,
        left: Box<ShellAst>,
        right: Box<ShellAst>,
    },
}

impl ShellAst {
    /// Parse a shell string and build an abstract syntax tree.
    pub fn parse(string: impl AsRef<str>) -> Result<Self, Error> {
        let string = string.as_ref();

        for unsupported_char in ['$', '`', '(', ')', '{', '}'] {
            if string.contains(unsupported_char) {
                return Err(Error::UnsupportedFeature(unsupported_char));
            }
        }

        let mut parser = ShellParser::new(string);
        parser.parse_expression(0)
    }
}
