use crate::PreprocessorContext;
use regex::Regex;
use std::collections::HashMap;

mod action;
mod keybinding;

pub use action::*;
pub use keybinding::*;

pub trait Template {
    fn key(&self) -> &'static str;
    fn regex(&self) -> Regex;
    fn parse_args(&self, args: &str) -> HashMap<String, String>;
    fn render(&self, context: &PreprocessorContext, args: &HashMap<String, String>) -> String;

    fn process(&self, context: &PreprocessorContext, content: &str) -> String {
        self.regex()
            .replace_all(content, |caps: &regex::Captures| {
                let args = self.parse_args(&caps[1]);
                self.render(context, &args)
            })
            .into_owned()
    }
}
