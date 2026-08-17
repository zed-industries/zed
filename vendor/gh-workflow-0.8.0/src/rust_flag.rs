//! A type-safe representation of the Rust toolchain.

use std::fmt::{Display, Formatter};

use indexmap::IndexMap;
use serde_json::Value;

use crate::Env;

#[derive(Clone)]
pub enum RustFlags {
    Lint(String, Lint),
    Combine(Box<RustFlags>, Box<RustFlags>),
}
#[derive(Clone)]
pub enum Lint {
    Allow,
    Warn,
    Deny,
    Forbid,
    Codegen,
    Experiment,
}

impl core::ops::Add for RustFlags {
    type Output = RustFlags;

    fn add(self, rhs: Self) -> Self::Output {
        RustFlags::Combine(Box::new(self), Box::new(rhs))
    }
}

impl RustFlags {
    pub fn allow<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Allow)
    }

    pub fn warn<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Warn)
    }

    pub fn deny<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Deny)
    }

    pub fn forbid<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Forbid)
    }

    pub fn codegen<S: ToString>(name: S) -> Self {
        RustFlags::Lint(name.to_string(), Lint::Codegen)
    }
}

impl Display for RustFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustFlags::Lint(name, lint) => match lint {
                Lint::Allow => write!(f, "-A{name}"),
                Lint::Warn => write!(f, "-W{name}"),
                Lint::Deny => write!(f, "-D{name}"),
                Lint::Forbid => write!(f, "-F{name}"),
                Lint::Codegen => write!(f, "-C{name}"),
                Lint::Experiment => write!(f, "-Z{name}"),
            },
            RustFlags::Combine(lhs, rhs) => write!(f, "{lhs} {rhs}"),
        }
    }
}

impl From<RustFlags> for Env {
    fn from(value: RustFlags) -> Self {
        let mut env = IndexMap::default();
        env.insert("RUSTFLAGS".to_string(), Value::from(value.to_string()));
        Env::from(env)
    }
}
