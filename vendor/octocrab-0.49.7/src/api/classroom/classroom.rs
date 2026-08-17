use crate::models::ClassroomId;
use crate::Octocrab;

#[derive(serde::Serialize)]
pub struct ClassroomHandler<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> ClassroomHandler<'octo> {
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self {
            crab,
            per_page: None,
            page: None,
        }
    }

    /// Results per page (max 100).
    pub fn per_page(mut self, per_page: impl Into<u8>) -> Self {
        self.per_page = Some(per_page.into());
        self
    }

    /// Page number of the results to fetch.
    pub fn page(mut self, page: impl Into<u32>) -> Self {
        self.page = Some(page.into());
        self
    }

    /// ### List classrooms
    ///
    /// Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.
    ///
    /// Fine-grained access tokens for "List classrooms"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    pub async fn list_classrooms(&self) -> crate::Result<Vec<crate::models::classroom::Classroom>> {
        let route = String::from("/classrooms");
        self.crab.get(route, Some(&self)).await
    }

    /// ### Get a classroom
    ///
    /// Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.
    ///
    /// Fine-grained access tokens for "Get a classroom"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    pub async fn get_classroom(
        &self,
        classroom_id: ClassroomId,
    ) -> crate::Result<crate::models::classroom::Classroom> {
        let route = format!("/classrooms/{}", classroom_id);
        self.crab.get(route, Some(&self)).await
    }

    /// ### List assignments for a classroom
    ///
    /// Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.
    ///
    /// Fine-grained access tokens for "List assignments for a classroom"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    pub async fn list_assignments(
        &self,
        classroom_id: ClassroomId,
    ) -> crate::Result<Vec<crate::models::classroom::SimpleAssignment>> {
        let route = format!("/classrooms/{}/assignments", classroom_id);
        self.crab.get(route, Some(&self)).await
    }
}
