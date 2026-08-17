use super::*;

use chrono;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Team {
    pub id: TeamId,
    pub node_id: String,
    pub url: Url,
    pub html_url: Url,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: TeamPrivacy,
    pub permission: String,
    pub members_url: Url,
    pub repositories_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<orgs::Organization>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestedReviewers {
    pub users: Vec<Author>,
    pub teams: Vec<Team>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestedTeam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TeamId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub privacy: TeamPrivacy,
    pub permission: String,
    pub members_url: Url,
    pub repositories_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Team>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TeamInvitation {
    pub id: TeamInvitationId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_reason: Option<String>,
    pub inviter: Author,
    pub team_count: u32,
    pub node_id: String,
    pub invitation_teams_url: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TeamPrivacy {
    Open,
    Closed,
    Secret,
}
