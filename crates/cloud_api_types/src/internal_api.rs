use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub legacy_user_id: i32,
    pub github_login: String,
    pub avatar_url: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzySearchUsersBody {
    pub query: String,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzySearchUsersResponse {
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzySearchChannelMembersByGithubLoginBody {
    pub channel_id: i32,
    pub query: String,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuzzySearchChannelMembersByGithubLoginResponse {
    pub channel_members: Vec<ChannelMember>,
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelMember {
    pub legacy_user_id: i32,
    pub kind: ChannelMemberKind,
    pub role: ChannelMemberRole,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMemberKind {
    Member,
    Invitee,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMemberRole {
    Admin,
    Member,
    Talker,
    Guest,
    Banned,
}
