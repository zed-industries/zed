use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Notification {
    pub id: NotificationId,
    pub repository: Repository,
    pub subject: Subject,
    pub reason: String,
    pub unread: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_read_at: Option<chrono::DateTime<chrono::Utc>>,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Reason {
    Assign,
    Author,
    Comment,
    Invitation,
    Manual,
    Mention,
    #[serde(rename = "review_requested")]
    ReviewRequested,
    #[serde(rename = "security_alert")]
    SecurityAlert,
    #[serde(rename = "state_change")]
    StateChange,
    Subscribed,
    #[serde(rename = "team_mention")]
    TeamMention,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Subject {
    pub title: String,
    pub url: Option<Url>,
    pub latest_comment_url: Option<Url>,
    pub r#type: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadSubscription {
    pub subscribed: bool,
    pub ignored: bool,
    pub reason: Option<Reason>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub url: Url,
    pub thread_url: Url,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StarredRepository {
    pub repo: Repository,
    pub starred_at: DateTime<Utc>,
}
