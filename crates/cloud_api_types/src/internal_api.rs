use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub legacy_user_id: i32,
    pub github_login: String,
    pub github_user_id: i32,
    pub name: Option<String>,
    pub admin: bool,
    pub connected_once: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookUpUsersByLegacyIdBody {
    pub legacy_user_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookUpUsersByLegacyIdResponse {
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookUpUserByGithubLoginBody {
    pub github_login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LookUpUserByGithubLoginResponse {
    pub user: Option<User>,
}
