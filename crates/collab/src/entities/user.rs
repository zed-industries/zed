use crate::db::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub avatar_url: String,
    pub name: Option<String>,
    pub admin: bool,
    pub connected_once: bool,
}
