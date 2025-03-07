mod session;
mod tag;

pub use session::*;
pub use tag::*;

pub const SCRIPTING_PROMPT: &str = include_str!("./system_prompt.txt");
