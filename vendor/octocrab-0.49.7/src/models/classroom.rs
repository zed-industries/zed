use crate::models::{AssignmentId, ClassroomId, OrgId, RepositoryId, UserId};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
/// A GitHub Classroom assignment
pub struct Assignment {
    pub id: AssignmentId,
    pub public_repo: bool,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: AssignmentType,
    pub invite_link: String,
    pub invitations_enabled: bool,
    pub slug: String,
    pub students_are_repo_admins: bool,
    pub feedback_pull_requests_enabled: bool,
    pub max_teams: Option<u32>,
    pub max_members: Option<u32>,
    pub editor: String,
    pub accepted: u32,
    pub submitted: u32,
    pub passing: u32,
    pub language: String,
    pub deadline: Option<DateTime<chrono::Utc>>,
    pub starter_code_repository: SimpleCodeRepository,
    pub classroom: Classroom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AssignmentType {
    Individual,
    Group,
}

/// GitHub repository view for Classroom
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleCodeRepository {
    pub id: RepositoryId,
    pub full_name: String,
    pub html_url: String,
    pub node_id: String,
    pub private: bool,
    pub default_branch: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Classroom {
    pub id: ClassroomId,
    pub name: String,
    pub archived: bool,
    pub organization: Option<SimpleOrganization>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleOrganization {
    pub id: OrgId,
    pub login: String,
    pub node_id: String,
    pub html_url: String,
    pub name: String,
    pub avatar_url: String,
}

/// A GitHub Classroom Accepted assignment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AcceptedAssignment {
    pub id: AssignmentId,
    pub submitted: bool,
    pub passing: bool,
    pub commit_count: u32,
    pub grade: String,
    pub students: Vec<SimpleClassroomUser>,
    pub repository: SimpleCodeRepository,
    pub assignment: SimpleAssignment,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleClassroomUser {
    pub id: UserId,
    pub login: String,
    pub avatar_url: Url,
    pub html_url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleAssignment {
    pub id: AssignmentId,
    pub public_repo: bool,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: AssignmentType,
    pub invite_link: String,
    pub invitations_enabled: bool,
    pub slug: String,
    pub students_are_repo_admins: bool,
    pub feedback_pull_requests_enabled: bool,
    pub max_teams: Option<u32>,
    pub max_members: Option<u32>,
    pub editor: String,
    pub accepted: u32,
    pub submitted: u32,
    pub passing: u32,
    pub language: String,
    pub deadline: Option<DateTime<chrono::Utc>>,
    pub classroom: SimpleClassroom,
}

/// A GitHub Classroom simple classroom
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SimpleClassroom {
    pub id: ClassroomId,
    pub name: String,
    pub archived: bool,
    pub url: String,
}

/// Classroom Assignment Grade
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssignmentGrade {
    pub assignment_name: String,
    pub assignment_url: String,
    pub starter_code_url: String,
    pub github_username: String,
    pub roster_identifier: String,
    pub student_repository_name: String,
    pub student_repository_url: String,
    pub submission_timestamp: DateTime<chrono::Utc>,
    pub points_awarded: u32,
    pub points_available: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}
