use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReactionContent {
    #[serde(rename = "heart")]
    Heart,
    #[serde(rename = "+1")]
    PlusOne,
    #[serde(rename = "laugh")]
    Laugh,
    #[serde(rename = "confused")]
    Confused,
    #[serde(rename = "hooray")]
    Hooray,
    #[serde(rename = "-1")]
    MinusOne,
    #[serde(rename = "rocket")]
    Rocket,
    #[serde(rename = "eyes")]
    Eyes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Reaction {
    pub id: ReactionId,
    pub node_id: String,
    pub user: Author,
    pub content: ReactionContent,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
