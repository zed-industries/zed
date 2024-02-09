#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extension {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub repository: String,
    pub url_download: String,
}
