use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    All,
    Private,
    Selected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSecret {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub visibility: Visibility,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repositories_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrganizationSecrets {
    pub total_count: i32,
    pub secrets: Vec<OrganizationSecret>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateOrganizationSecret<'a> {
    /// Value for your secret,
    /// encrypted with LibSodium using the public key retrieved from the Get an organization public key endpoint.
    pub encrypted_value: &'a str,
    /// ID of the key you used to encrypt the secret.
    pub key_id: &'a str,
    /// Which type of organization repositories have access to the organization secret.
    pub visibility: Visibility,
    /// An array of repository ids that can access the organization secret.
    /// You can only provide a list of repository ids when the visibility is set to selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_repository_ids: Option<&'a [u32]>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrganizationSecretResponse {
    Created,
    Updated,
}
