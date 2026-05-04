use crate::db::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub admin: bool,
    pub connected_once: bool,
}
