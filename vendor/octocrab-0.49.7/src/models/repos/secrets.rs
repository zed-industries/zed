use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositorySecret {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RepositorySecrets {
    pub total_count: i32,
    pub secrets: Vec<RepositorySecret>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateRepositorySecret<'a> {
    /// Value for your secret,
    /// encrypted with LibSodium using the public key retrieved from the Get an organization public key endpoint.
    pub encrypted_value: &'a str,
    /// ID of the key you used to encrypt the secret.
    pub key_id: &'a str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateRepositorySecretResponse {
    Created,
    Updated,
}
