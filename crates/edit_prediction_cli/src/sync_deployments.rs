use anyhow::{Context as _, Result};
use http_client::{AsyncBody, HttpClient, Method, Request};
use serde::Deserialize;
use serde_json::{Value as JsonValue, json};
use std::sync::Arc;

use crate::pull_examples::{
    self, MAX_POLL_ATTEMPTS, POLL_INTERVAL, SNOWFLAKE_ASYNC_IN_PROGRESS_CODE,
    SNOWFLAKE_SUCCESS_CODE,
};

const DEFAULT_BASETEN_MODEL_NAME: &str = "zeta-2";
const DEFAULT_STATEMENT_TIMEOUT_SECONDS: u64 = 120;

#[derive(Debug, Clone, Deserialize)]
struct BasetenModelsResponse {
    models: Vec<BasetenModel>,
}

#[derive(Debug, Clone, Deserialize)]
struct BasetenModel {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct BasetenEnvironmentsResponse {
    environments: Vec<BasetenEnvironment>,
}

#[derive(Debug, Clone, Deserialize)]
struct BasetenEnvironment {
    name: String,
    current_deployment: Option<BasetenDeployment>,
}

#[derive(Debug, Clone, Deserialize)]
struct BasetenDeployment {
    name: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
}

#[derive(Debug, Clone)]
struct DeploymentRecord {
    model_id: String,
    model_version_id: String,
    experiment_name: String,
    environment: String,
    status: String,
    created_at: String,
}

async fn fetch_baseten_models(
    http_client: &Arc<dyn HttpClient>,
    api_key: &str,
) -> Result<Vec<BasetenModel>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri("https://api.baseten.co/v1/models")
        .header("Authorization", format!("Api-Key {api_key}"))
        .header("Accept", "application/json")
        .body(AsyncBody::empty())?;

    let response = http_client
        .send(request)
        .await
        .context("failed to fetch baseten models")?;

    let status = response.status();
    let body_bytes = {
        use futures::AsyncReadExt as _;
        let mut body = response.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes)
            .await
            .context("failed to read baseten models response")?;
        bytes
    };

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!("baseten models API http {}: {}", status.as_u16(), body_text);
    }

    let parsed: BasetenModelsResponse =
        serde_json::from_slice(&body_bytes).context("failed to parse baseten models response")?;
    Ok(parsed.models)
}

async fn fetch_baseten_environments(
    http_client: &Arc<dyn HttpClient>,
    api_key: &str,
    model_id: &str,
) -> Result<Vec<BasetenEnvironment>> {
    let url = format!("https://api.baseten.co/v1/models/{model_id}/environments");
    let request = Request::builder()
        .method(Method::GET)
        .uri(url.as_str())
        .header("Authorization", format!("Api-Key {api_key}"))
        .header("Accept", "application/json")
        .body(AsyncBody::empty())?;

    let response = http_client
        .send(request)
        .await
        .context("failed to fetch baseten environments")?;

    let status = response.status();
    let body_bytes = {
        use futures::AsyncReadExt as _;
        let mut body = response.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes)
            .await
            .context("failed to read baseten environments response")?;
        bytes
    };

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!(
            "baseten environments API http {}: {}",
            status.as_u16(),
            body_text
        );
    }

    let raw: serde_json::Value =
        serde_json::from_slice(&body_bytes).context("failed to parse environments JSON")?;
    eprintln!(
        "Raw baseten environments response:\n{}",
        serde_json::to_string_pretty(&raw).unwrap_or_else(|_| String::from("<failed to format>"))
    );

    let parsed: BasetenEnvironmentsResponse =
        serde_json::from_value(raw).context("failed to deserialize environments response")?;
    Ok(parsed.environments)
}

fn collect_deployment_records(
    model_id: &str,
    environments: &[BasetenEnvironment],
) -> Vec<DeploymentRecord> {
    let mut records = Vec::new();
    for env in environments {
        let Some(deployment) = &env.current_deployment else {
            eprintln!("  Environment '{}': no deployment, skipping", env.name);
            continue;
        };

        let model_version_id = match &deployment.id {
            Some(id) => id.clone(),
            None => {
                eprintln!(
                    "  Environment '{}' deployment '{}': no id field, skipping (this deployment cannot be linked to x_baseten_model_version_id)",
                    env.name, deployment.name
                );
                continue;
            }
        };

        eprintln!(
            "  Environment '{}': deployment '{}' (model_version_id={})",
            env.name, deployment.name, model_version_id
        );

        records.push(DeploymentRecord {
            model_id: model_id.to_string(),
            model_version_id,
            experiment_name: deployment.name.clone(),
            environment: env.name.clone(),
            status: deployment
                .status
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            created_at: deployment
                .created_at
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        });
    }
    records
}

async fn upsert_deployment_to_snowflake(
    http_client: &Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    role: &Option<String>,
    record: &DeploymentRecord,
) -> Result<()> {
    let event_properties = json!({
        "model_id": record.model_id,
        "model_version_id": record.model_version_id,
        "experiment_name": record.experiment_name,
        "environment": record.environment,
        "status": record.status,
        "created_at": record.created_at,
    });

    let event_properties_str =
        serde_json::to_string(&event_properties).context("failed to serialize event_properties")?;

    let statement = r#"
MERGE INTO events AS target
USING (
    SELECT
        'Edit Prediction Deployment' AS event_type,
        PARSE_JSON(?) AS event_properties,
        'ep-cli' AS device_id,
        CURRENT_TIMESTAMP() AS time
) AS source
ON target.event_type = 'Edit Prediction Deployment'
   AND target.event_properties:model_id::string = source.event_properties:model_id::string
   AND target.event_properties:model_version_id::string = source.event_properties:model_version_id::string
WHEN MATCHED THEN UPDATE SET
    target.event_properties = source.event_properties,
    target.time = source.time
WHEN NOT MATCHED THEN INSERT (event_type, event_properties, device_id, time)
    VALUES (source.event_type, source.event_properties, source.device_id, source.time)
"#;

    let bindings = json!({
        "1": { "type": "TEXT", "value": event_properties_str }
    });

    let request = json!({
        "statement": statement,
        "timeout": DEFAULT_STATEMENT_TIMEOUT_SECONDS,
        "database": "EVENTS",
        "schema": "PUBLIC",
        "warehouse": "DBT",
        "role": role,
        "bindings": bindings
    });

    let mut response =
        pull_examples::run_sql(http_client.clone(), base_url, token, &request).await?;

    if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
        let statement_handle = response
            .statement_handle
            .as_ref()
            .context("async query response missing statementHandle")?
            .clone();

        for attempt in 1..=MAX_POLL_ATTEMPTS {
            eprint!("  polling ({attempt})...");
            std::thread::sleep(POLL_INTERVAL);

            response = pull_examples::fetch_partition(
                http_client.clone(),
                base_url,
                token,
                &statement_handle,
                0,
            )
            .await?;

            if response.code.as_deref() != Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
                eprintln!(" done");
                break;
            }
        }

        if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
            anyhow::bail!(
                "MERGE still running after {} poll attempts ({} seconds)",
                MAX_POLL_ATTEMPTS,
                MAX_POLL_ATTEMPTS as u64 * POLL_INTERVAL.as_secs()
            );
        }
    }

    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake MERGE failed: code={} message={}",
                code,
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    Ok(())
}

pub async fn run_sync_deployments(
    http_client: Arc<dyn HttpClient>,
    model_name: Option<String>,
) -> Result<()> {
    let baseten_api_key = std::env::var("BASETEN_API_KEY")
        .context("missing required environment variable BASETEN_API_KEY")?;
    let snowflake_token = std::env::var("EP_SNOWFLAKE_API_KEY")
        .context("missing required environment variable EP_SNOWFLAKE_API_KEY")?;
    let snowflake_base_url = std::env::var("EP_SNOWFLAKE_BASE_URL").context(
        "missing required environment variable EP_SNOWFLAKE_BASE_URL (e.g. https://<account>.snowflakecomputing.com)",
    )?;
    let snowflake_role = std::env::var("EP_SNOWFLAKE_ROLE").ok();

    let model_name = model_name.unwrap_or_else(|| DEFAULT_BASETEN_MODEL_NAME.to_string());

    eprintln!("Fetching baseten models...");
    let models = fetch_baseten_models(&http_client, &baseten_api_key).await?;

    let model = models
        .iter()
        .find(|m| m.name == model_name)
        .with_context(|| {
            let available: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
            format!(
                "model '{}' not found on baseten. Available: {:?}",
                model_name, available
            )
        })?;

    eprintln!(
        "Found model '{}' (id={}). Fetching environments...",
        model.name, model.id
    );

    let environments = fetch_baseten_environments(&http_client, &baseten_api_key, &model.id)
        .await
        .with_context(|| format!("failed to fetch environments for model '{}'", model.name))?;

    eprintln!("Found {} environment(s):", environments.len());
    let records = collect_deployment_records(&model.id, &environments);

    if records.is_empty() {
        eprintln!("No deployments to sync.");
        return Ok(());
    }

    eprintln!(
        "\nUpserting {} deployment(s) to Snowflake...",
        records.len()
    );

    for record in &records {
        eprintln!(
            "  MERGE: model_id={} model_version_id={} experiment={} env={}",
            record.model_id, record.model_version_id, record.experiment_name, record.environment
        );
        upsert_deployment_to_snowflake(
            &http_client,
            &snowflake_base_url,
            &snowflake_token,
            &snowflake_role,
            record,
        )
        .await
        .with_context(|| {
            format!(
                "failed to upsert deployment '{}' (model_version_id={})",
                record.experiment_name, record.model_version_id
            )
        })?;
    }

    eprintln!("Done. Synced {} deployment(s).\n", records.len());

    query_and_display_deployments(
        &http_client,
        &snowflake_base_url,
        &snowflake_token,
        &snowflake_role,
    )
    .await?;

    Ok(())
}

async fn query_and_display_deployments(
    http_client: &Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    role: &Option<String>,
) -> Result<()> {
    let statement = r#"
SELECT
    event_properties:model_id::string AS model_id,
    event_properties:model_version_id::string AS model_version_id,
    event_properties:experiment_name::string AS experiment_name,
    event_properties:environment::string AS environment,
    event_properties:status::string AS status,
    event_properties:created_at::string AS created_at,
    time::string AS synced_at
FROM events
WHERE event_type = 'Edit Prediction Deployment'
ORDER BY event_properties:environment::string, event_properties:created_at::string DESC
"#;

    let request = json!({
        "statement": statement,
        "timeout": DEFAULT_STATEMENT_TIMEOUT_SECONDS,
        "database": "EVENTS",
        "schema": "PUBLIC",
        "warehouse": "DBT",
        "role": role,
    });

    let mut response =
        pull_examples::run_sql(http_client.clone(), base_url, token, &request).await?;

    if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
        let statement_handle = response
            .statement_handle
            .as_ref()
            .context("async query response missing statementHandle")?
            .clone();

        for attempt in 1..=MAX_POLL_ATTEMPTS {
            eprint!("  polling ({attempt})...");
            std::thread::sleep(POLL_INTERVAL);

            response = pull_examples::fetch_partition(
                http_client.clone(),
                base_url,
                token,
                &statement_handle,
                0,
            )
            .await?;

            if response.code.as_deref() != Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
                eprintln!(" done");
                break;
            }
        }

        if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
            anyhow::bail!(
                "deployment query still running after {} poll attempts",
                MAX_POLL_ATTEMPTS
            );
        }
    }

    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "deployment query failed: code={} message={}",
                code,
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    let col_names = [
        "model_id",
        "model_version_id",
        "experiment_name",
        "environment",
        "status",
        "created_at",
        "synced_at",
    ];
    let mut col_widths: Vec<usize> = col_names.iter().map(|n| n.len()).collect();

    let rows: Vec<Vec<String>> = response
        .data
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, val)| {
                    let s = match val {
                        JsonValue::String(s) => s.clone(),
                        JsonValue::Null => "—".to_string(),
                        other => other.to_string(),
                    };
                    if i < col_widths.len() {
                        col_widths[i] = col_widths[i].max(s.len());
                    }
                    s
                })
                .collect()
        })
        .collect();

    eprintln!("Deployments in Snowflake ({} total):\n", rows.len());

    for (i, name) in col_names.iter().enumerate() {
        if i > 0 {
            eprint!("  ");
        }
        eprint!("{:width$}", name, width = col_widths[i]);
    }
    eprintln!();

    for (i, width) in col_widths.iter().enumerate() {
        if i > 0 {
            eprint!("  ");
        }
        eprint!("{}", "─".repeat(*width));
    }
    eprintln!();

    for row in &rows {
        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                eprint!("  ");
            }
            let width = col_widths.get(i).copied().unwrap_or(0);
            eprint!("{:width$}", val, width = width);
        }
        eprintln!();
    }

    Ok(())
}
