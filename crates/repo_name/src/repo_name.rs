use gpui::Global;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RepoName(pub String);

impl Global for RepoName {}

impl Display for RepoName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
