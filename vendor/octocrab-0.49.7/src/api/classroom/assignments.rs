use crate::models::AssignmentId;
use crate::{models, Octocrab};

#[derive(serde::Serialize)]
pub struct AssignmentsHandler<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'octo> AssignmentsHandler<'octo> {
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

    /// ### Gets a GitHub Classroom assignment.
    ///
    /// Assignment will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    ///
    /// ```no_run
    /// use octocrab::models::AssignmentId;
    /// async fn run() -> octocrab::Result<()> {
    ///  let client = octocrab::Octocrab::default();
    ///  let assignment_id: AssignmentId = 42.into();
    ///  let result = client
    ///         .assignments()
    ///         .get(assignment_id.into())
    ///         .await;
    ///     Ok(())
    /// }
    pub async fn get(
        &self,
        assignment_id: AssignmentId,
    ) -> crate::Result<models::classroom::Assignment> {
        let route = format!("/assignments/{}", assignment_id);
        self.crab.get(route, Some(&self)).await
    }

    /// ### List accepted assignments for an assignment
    ///
    /// Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// Fine-grained access tokens for "List accepted assignments for an assignment"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    ///
    pub async fn list_accepted(
        &self,
        assignment_id: AssignmentId,
    ) -> crate::Result<Vec<models::classroom::AcceptedAssignment>> {
        let route = format!("/assignments/{assignment_id}/accepted_assignments");
        self.crab.get(route, Some(&self)).await
    }

    /// ### Get assignment grades
    ///
    /// Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.
    ///
    /// Fine-grained access tokens for "Get assignment grades"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    ///
    pub async fn get_grades(
        &self,
        assignment_id: AssignmentId,
    ) -> crate::Result<Vec<models::classroom::AssignmentGrade>> {
        let route = format!("/assignments/{assignment_id}/grades");
        self.crab.get(route, Some(&self)).await
    }
}
