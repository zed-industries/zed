use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum ShellAst {
    Cmd(ShellCmd),
    Op {
        operator: Operator,
        left: Box<ShellAst>,
        right: Box<ShellAst>,
    },
    /// Prints the given string to stdout
    Echo(String),
    /// Reads delimited elems from stdin and passes them to the
    /// given command.
    /// See https://www.man7.org/linux/man-pages/man1/xargs.1.html
    Xargs(XargsOptions),
}

pub struct XargsOptions {
    /// Default is "\n\n"
    pub delimiter: String,
    /// Quotes include both single quotes and double quotes
    pub escape_quotes_and_backslashes: bool,
}

impl XargsOptions {
    fn from_args<S: AsRef<str>>(args: impl IntoIterator<Item = S>) -> Option<Self> {
        let mut args = args.into_iter();

        for arg in args {
            match arg.as_ref() {
                "-0" | "-null" => {
                    //
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShellCmd {
    pub command: VarString,
    pub args: Vec<VarString>,
    pub stdout_redirect: Option<VarString>,
    pub stderr_redirect: Option<VarString>,
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

/// A string, possibly with shell variable substitutions
/// in it (e.g. "foo${bar}").
#[derive(Debug, Clone, PartialEq)]
pub enum VarString {
    Plaintext(String),
    Vars {
        prefix: String,
        vars: Vec<(String, String)>,
    },
}

impl<T: Into<String>> From<T> for VarString {
    fn from(plaintext: T) -> Self {
        Self::Plaintext(plaintext.into())
    }
}

impl Default for VarString {
    fn default() -> Self {
        Self::Plaintext(String::new())
    }
}

impl VarString {
    /// If there is a syntax error, like an unclosed '{' or '$' at the end of the string,
    /// return Err with the original (owned) token in it.
    pub fn from_token(token: String) -> Result<Self, String> {
        let todo = todo!(); // TODO split it up etc.
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Plaintext(string) => string.is_empty(),
            Self::Vars { prefix, vars } => prefix.is_empty() && vars.is_empty(),
        }
    }

    /// If the VarString contains a var that lookup_var returns None for,
    /// return Err with that var name.
    pub fn resolve<'a>(
        &'a self,
        lookup_var: impl Fn(&str) -> Option<&str>,
    ) -> Result<Cow<'a, str>, &'a str> {
        match self {
            Self::Plaintext(string) => Ok(Cow::Borrowed(string)),
            Self::Vars { prefix, vars } => {
                let mut answer = prefix.to_string();

                for (var, suffix) in vars {
                    answer.push_str(lookup_var(&var).ok_or(var.as_str())?);
                    answer.push_str(&suffix);
                }

                Ok(Cow::Owned(answer))
            }
        }
    }
}
