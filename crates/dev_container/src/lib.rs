use std::{collections::HashMap, sync::Arc};

use futures::AsyncReadExt;
use http::Request;
use http_client::{AsyncBody, HttpClient};
use serde::Deserialize;
use smol::fs::File;
use util::command::new_smol_command;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubTokenResponse {
    token: String,
}

fn ghcr_url() -> &'static str {
    "https://ghcr.io"
}

fn devcontainer_templates_repository() -> &'static str {
    "devcontainers/templates"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestLayer {
    digest: String,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TemplateOptions {
    #[serde(rename = "type")]
    pub option_type: String,
    pub description: String,
    pub proposals: Option<Vec<String>>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
    pub default: String,
}

impl TemplateOptions {
    // TODO put this under test
    pub fn possible_values(&self) -> Vec<String> {
        match self.option_type.as_str() {
            "string" => self
                .enum_values
                .clone()
                .or(self.proposals.clone().or(Some(vec![self.default.clone()])))
                .unwrap_or_default(),
            // If not string, must be boolean
            _ => {
                if self.default == "true" {
                    vec!["true".to_string(), "false".to_string()]
                } else {
                    vec!["false".to_string(), "true".to_string()]
                }
            }
        }
    }
}

// https://distribution.github.io/distribution/spec/api/#pulling-an-image-manifest
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerManifestsResponse {
    schema_version: u32,
    media_type: String,
    layers: Vec<ManifestLayer>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DevContainerTemplate {
    pub id: String,
    pub name: String,
    pub options: Option<HashMap<String, TemplateOptions>>,
}

// https://ghcr.io/v2/devcontainers/templates/blobs/sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevContainerTemplatesResponse {
    pub templates: Vec<DevContainerTemplate>,
}

pub async fn get_template_text(
    client: Arc<dyn HttpClient>,
    template: &DevContainerTemplate,
) -> Result<HashMap<String, String>, String> {
    let id = template.id.clone();
    let token = get_ghcr_token(&client).await?;
    let manifest = get_latest_manifest_for_id(&id, &token.token, &client).await?;

    let file_contents =
        get_devcontainer_template_files(&id, &token.token, &manifest.layers[0].digest, &client)
            .await?;

    Ok(file_contents)
}

pub async fn get_templates(
    client: Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let token = get_ghcr_token(&client).await?;
    let manifest = get_latest_manifest(&token.token, &client).await?;

    get_devcontainer_templates(&token.token, &manifest.layers[0].digest, &client).await
}

// Once we get the list of templates, and select the ID, we need to
// Get the manifest of that specific template, e.g. https://ghcr.io/v2/devcontainers/templates/alpine/manifests/latest
// /// Layer mediatype:   "mediaType": "application/vnd.devcontainers.layer.v1+tar",
// As opposed to "application/vnd.devcontainers.collection.layer.v1+json" for the list of templates
// Get the content (sent as a tarball) for the layer, e.g. https://ghcr.io/v2/devcontainers/templates/alpine/blobs/sha256:723fb0b5fc6eedd76957710cd45b287ef31362f900ea61190c1472910317bcb1

pub async fn get_ghcr_token(client: &Arc<dyn HttpClient>) -> Result<GithubTokenResponse, String> {
    let url = format!(
        "{}/token?service=ghcr.io&scope=repository:{}:pull",
        ghcr_url(),
        devcontainer_templates_repository()
    );
    get_deserialized_response("", &url, client).await
}

pub async fn get_latest_manifest_for_id(
    id: &str,
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "{}/v2/{}/{}/manifests/latest",
        ghcr_url(),
        devcontainer_templates_repository(),
        id
    );
    dbg!(&url, token);
    get_deserialized_response(token, &url, client).await
}

pub async fn get_latest_manifest(
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "{}/v2/{}/manifests/latest",
        ghcr_url(),
        devcontainer_templates_repository()
    );
    dbg!(&url, token);
    get_deserialized_response(token, &url, client).await
}

pub async fn get_devcontainer_template_files(
    id: &str,
    token: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<HashMap<String, String>, String> {
    let url = format!(
        "{}/v2/{}/{}/blobs/{}",
        ghcr_url(),
        devcontainer_templates_repository(),
        id,
        blob_digest
    );
    dbg!(&url, token);
    let request = Request::get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.oci.image.manifest.v1+json")
        .body(AsyncBody::default())
        .unwrap();
    // client.send(request).await.unwrap();

    // Need to organize this code better
    let temp_dir = tempfile::Builder::new().tempdir().unwrap();
    let target_path = temp_dir.path().join("downloadme.tar");
    let mut target_file = File::create(&target_path).await.unwrap();
    let extracted = temp_dir.path().join("extracted");
    std::fs::create_dir(&extracted).unwrap();
    let Ok(mut response) = client.send(request).await else {
        return Err("Failed get reponse - TODO fix error handling".to_string());
    };

    smol::io::copy(response.body_mut(), &mut target_file)
        .await
        .unwrap();

    let command_output = new_smol_command("tar")
        .arg("-xvf")
        .arg(&target_path)
        .arg("-C")
        .arg(&extracted)
        .output()
        .await
        .unwrap();

    dbg!(&command_output);

    let extracted_location = &extracted.join(".devcontainer/devcontainer.json");

    dbg!(&extracted_location);

    let files = match std::fs::read_dir(&extracted.join(".devcontainer")) {
        Ok(files) => files,
        Err(e) => {
            println!("Error reading directory: {}", e);
            return Err("didn't read files".to_string());
        }
    };

    let mut file_contents: HashMap<String, String> = HashMap::new();
    for file in files {
        let Ok(file) = file else {
            continue;
        };

        file_contents.insert(
            file.file_name().into_string().unwrap(),
            std::fs::read_to_string(file.path()).unwrap(),
        );
    }

    Ok(file_contents)
}

pub async fn get_devcontainer_templates(
    token: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let url = format!(
        "{}/v2/{}/blobs/{}",
        ghcr_url(),
        devcontainer_templates_repository(),
        blob_digest
    );
    get_deserialized_response(token, &url, client).await
}

pub async fn get_deserialized_response<T>(
    token: &str,
    url: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let request = Request::get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.oci.image.manifest.v1+json")
        .body(AsyncBody::default())
        .unwrap();
    // client.send(request).await.unwrap();
    let Ok(response) = client.send(request).await else {
        return Err("Failed get reponse - TODO fix error handling".to_string());
    };

    let mut output = String::new();

    let Ok(_) = response.into_body().read_to_string(&mut output).await else {
        return Err("Failed to read response body - TODO fix error handling".to_string());
    };

    let structured_response: T = serde_json::from_str(&output).unwrap(); // TODO
    Ok(structured_response)
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, anyhow};

    use crate::{
        GithubTokenResponse, devcontainer_templates_repository, get_deserialized_response,
        get_devcontainer_templates, get_ghcr_token, get_latest_manifest,
    };

    #[gpui::test]
    async fn test_get_deserialized_response(_cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response =
            get_deserialized_response::<GithubTokenResponse>("", "https://ghcr.io/token", &client)
                .await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string())
    }

    #[gpui::test]
    async fn test_get_ghcr_token() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path != "/token" {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            let query = request.uri().query();
            if query.is_none()
                || query.unwrap()
                    != format!(
                        "service=ghcr.io&scope=repository:{}:pull",
                        devcontainer_templates_repository()
                    )
            {
                return Err(anyhow!("Unexpected query: {}", query.unwrap_or_default()));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response = get_ghcr_token(&client).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string());
    }

    #[gpui::test]
    async fn test_get_latest_manifests() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path
                != format!(
                    "/v2/{}/manifests/latest",
                    devcontainer_templates_repository()
                )
            {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"schemaVersion\": 2,
                    \"mediaType\": \"application/vnd.oci.image.manifest.v1+json\",
                    \"config\": {
                        \"mediaType\": \"application/vnd.devcontainers\",
                        \"digest\": \"sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a\",
                        \"size\": 2
                    },
                    \"layers\": [
                        {
                            \"mediaType\": \"application/vnd.devcontainers.collection.layer.v1+json\",
                            \"digest\": \"sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09\",
                            \"size\": 65235,
                            \"annotations\": {
                                \"org.opencontainers.image.title\": \"devcontainer-collection.json\"
                            }
                        }
                    ],
                    \"annotations\": {
                        \"com.github.package.type\": \"devcontainer_collection\"
                    }
                }".into())
                .unwrap())
        });

        let response = get_latest_manifest("", &client).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.schema_version, 2);
        assert_eq!(
            response.media_type,
            "application/vnd.oci.image.manifest.v1+json"
        );
        assert_eq!(response.layers.len(), 1);
        assert_eq!(
            response.layers[0].digest,
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09"
        );
    }

    #[gpui::test]
    async fn test_get_devcontainer_templates() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path
                != format!(
                    "/v2/{}/blobs/sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
                    devcontainer_templates_repository()
                )
            {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"sourceInformation\": {
                        \"source\": \"devcontainer-cli\"
                    },
                    \"templates\": [
                        {
                            \"id\": \"alpine\",
                            \"version\": \"3.4.0\",
                            \"name\": \"Alpine\",
                            \"description\": \"Simple Alpine container with Git installed.\",
                            \"documentationURL\": \"https://github.com/devcontainers/templates/tree/main/src/alpine\",
                            \"publisher\": \"Dev Container Spec Maintainers\",
                            \"licenseURL\": \"https://github.com/devcontainers/templates/blob/main/LICENSE\",
                            \"options\": {
                                \"imageVariant\": {
                                    \"type\": \"string\",
                                    \"description\": \"Alpine version:\",
                                    \"proposals\": [
                                        \"3.21\",
                                        \"3.20\",
                                        \"3.19\",
                                        \"3.18\"
                                    ],
                                    \"default\": \"3.20\"
                                }
                            },
                            \"platforms\": [
                                \"Any\"
                            ],
                            \"optionalPaths\": [
                                \".github/dependabot.yml\"
                            ],
                            \"type\": \"image\",
                            \"files\": [
                                \"NOTES.md\",
                                \"README.md\",
                                \"devcontainer-template.json\",
                                \".devcontainer/devcontainer.json\",
                                \".github/dependabot.yml\"
                            ],
                            \"fileCount\": 5,
                            \"featureIds\": []
                        }
                    ]
                }".into())
                .unwrap())
        });
        let response = get_devcontainer_templates(
            "",
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
            &client,
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.templates.len(), 1);
        assert_eq!(response.templates[0].name, "Alpine");
    }
}
