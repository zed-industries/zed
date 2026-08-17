use super::*;
pub mod secrets;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Organization {
    pub login: String,
    pub id: OrgId,
    pub node_id: String,
    pub url: Url,
    pub repos_url: Url,
    pub events_url: Url,
    pub hooks_url: Url,
    pub issues_url: Url,
    pub members_url: Url,
    pub public_members_url: Url,
    pub avatar_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_organization_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_repository_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_repos: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_gists: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_private_repos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_private_repos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_gists: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repository_settings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_requirement_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_allowed_repository_creation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_public_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_private_repositories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_internal_repositories: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MembershipInvitation {
    pub url: Url,
    pub state: String,
    pub role: String,
    pub organization_url: Url,
    pub organization: Organization,
    pub user: Author,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plan {
    pub name: String,
    pub space: i64,
    pub private_repos: i64,
}
