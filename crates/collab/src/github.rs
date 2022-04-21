use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
struct Installation {
    #[allow(unused)]
    id: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
}
