use crate::AtspiError;
use std::{fmt, str::FromStr};

/// An operation can either be [`Self::Insert`] or [`Self::Delete`].
/// These correspond to methods available on [`Vec`].
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
pub enum Operation {
	#[default]
	#[serde(rename = "add")]
	#[serde(alias = "add/system")]
	#[serde(alias = "insert")]
	#[serde(alias = "insert/system")]
	Insert,
	#[serde(rename = "delete")]
	#[serde(alias = "delete/system")]
	#[serde(alias = "remove")]
	#[serde(alias = "remove/system")]
	Delete,
}

impl FromStr for Operation {
	type Err = AtspiError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"add" | "add/system" | "insert" | "insert/system" => Ok(Operation::Insert),
			"delete" | "delete/system" | "remove" | "remove/system" => Ok(Operation::Delete),
			_ => Err(AtspiError::KindMatch(format!("{s} is not a type of Operation"))),
		}
	}
}

impl fmt::Display for Operation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Operation::Insert => write!(f, "insert"),
			Operation::Delete => write!(f, "delete"),
		}
	}
}
