//! dialoguer is a library for Rust that helps you build useful small
//! interactive user inputs for the command line.  It provides utilities
//! to render various simple dialogs like confirmation prompts, text
//! inputs and more.
//!
//! Best paired with other libraries in the family:
//!
//! * [indicatif](https://docs.rs/indicatif)
//! * [console](https://docs.rs/console)
//!
//! # Crate Contents
//!
//! * Confirmation prompts
//! * Input prompts (regular and password)
//! * Input validation
//! * Selections prompts (single and multi)
//! * Fuzzy select prompt
//! * Other kind of prompts
//! * Editor launching
//!
//! # Crate Features
//!
//! The following crate features are available:
//! * `editor`: enables bindings to launch editor to edit strings
//! * `fuzzy-select`: enables fuzzy select prompt
//! * `history`: enables input prompts to be able to track history of inputs
//! * `password`: enables password input prompt
//! * `completion`: enables ability to implement custom tab-completion for input prompts
//!
//! By default `editor` and `password` are enabled.

#![deny(clippy::all)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use console;

#[cfg(feature = "completion")]
pub use completion::Completion;
#[cfg(feature = "editor")]
pub use edit::Editor;
pub use error::{Error, Result};
#[cfg(feature = "history")]
pub use history::{BasicHistory, History};
use paging::Paging;
pub use validate::{InputValidator, PasswordValidator};

#[cfg(feature = "fuzzy-select")]
pub use prompts::fuzzy_select::FuzzySelect;
#[cfg(feature = "password")]
pub use prompts::password::Password;
pub use prompts::{
    confirm::Confirm, input::Input, multi_select::MultiSelect, select::Select, sort::Sort,
};

#[cfg(feature = "completion")]
mod completion;
#[cfg(feature = "editor")]
mod edit;
mod error;
#[cfg(feature = "history")]
mod history;
mod paging;
mod prompts;
pub mod theme;
mod validate;
