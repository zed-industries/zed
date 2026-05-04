use crate::db::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub github_login: String,
    pub github_user_id: i32,
    pub name: Option<String>,
    pub admin: bool,
    pub connected_once: bool,
}
