use crate::models::codes_of_conduct::CodeOfConduct;
use crate::Octocrab;

#[derive(serde::Serialize)]
pub struct CodesOfConductHandler<'octo> {
    #[serde(skip)]
    crab: &'octo Octocrab,
}
impl<'octo> CodesOfConductHandler<'octo> {
    pub fn new(crab: &'octo Octocrab) -> Self {
        Self { crab }
    }

    /// ### Get all codes of conduct
    ///
    /// Returns array of all GitHub's codes of conduct.
    ///
    /// Fine-grained access tokens for "Get all codes of conduct"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - GitHub App installation access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    pub async fn list_all_codes_of_conduct(&self) -> crate::Result<Vec<CodeOfConduct>> {
        let route = String::from("/codes_of_conduct");
        self.crab.get(route, Some(&self)).await
    }

    /// ### Get a code of conduct
    ///
    /// Returns information about the specified GitHub code of conduct.
    ///
    /// Fine-grained access tokens for "Get a code of conduct"
    ///
    /// This endpoint works with the following fine-grained token types:
    ///
    /// - GitHub App user access tokens
    /// - GitHub App installation access tokens
    /// - Fine-grained personal access tokens
    ///
    /// The fine-grained token does not require any permissions.
    ///
    /// This endpoint can be used without authentication if only public resources are requested.
    pub async fn get_code_of_conduct(&self, key: String) -> crate::Result<CodeOfConduct> {
        let route = format!("/codes_of_conduct/{}", key);
        self.crab.get(route, Some(&self)).await
    }
}
