use collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct VSSnippetsFile {
    #[serde(flatten)]
    pub(crate) snippets: HashMap<String, VSCodeSnippet>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum ListOrDirect {
    Single(String),
    List(Vec<String>),
}

impl From<ListOrDirect> for Vec<String> {
    fn from(list: ListOrDirect) -> Self {
        match list {
            ListOrDirect::Single(entry) => vec![entry],
            ListOrDirect::List(entries) => entries,
        }
    }
}

impl std::fmt::Display for ListOrDirect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Single(v) => v.to_owned(),
                Self::List(v) => v.join("\n"),
            }
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct VSCodeSnippet {
    pub(crate) prefix: Option<ListOrDirect>,
    pub(crate) body: ListOrDirect,
    pub(crate) description: Option<ListOrDirect>,
}
