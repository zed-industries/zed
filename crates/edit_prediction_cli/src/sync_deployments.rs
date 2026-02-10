use anyhow::{Context as _, Result};
use http_client::{AsyncBody, HttpClient, Method, Request};
use serde::Deserialize;
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
use std::sync::Arc;

use crate::pull_examples::{
    self, MAX_POLL_ATTEMPTS, POLL_INTERVAL, SNOWFLAKE_ASYNC_IN_PROGRESS_CODE,
    SNOWFLAKE_SUCCESS_CODE,
};

const DEFAULT_BASETEN_MODEL_NAME: &str = "zeta-2";
const DEFAULT_STATEMENT_TIMEOUT_SECONDS: u64 = 120;
pub(crate) const EDIT_PREDICTION_DEPLOYMENT_EVENT: &str = "Edit Prediction Deployment";

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
struct BasetenDeploymentsResponse {
    deployments: Vec<BasetenDeployment>,
}

#[derive(Debug, Clone, Deserialize)]
struct BasetenDeployment {
    id: String,
    name: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    environment: Option<String>,
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

#[derive(Debug, Clone)]
struct ExistingDeployment {
    experiment_name: String,
    environment: String,
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

async fn fetch_baseten_deployments(
    http_client: &Arc<dyn HttpClient>,
    api_key: &str,
    model_id: &str,
) -> Result<Vec<BasetenDeployment>> {
    let url = format!("https://api.baseten.co/v1/models/{model_id}/deployments");
    let request = Request::builder()
        .method(Method::GET)
        .uri(url.as_str())
        .header("Authorization", format!("Api-Key {api_key}"))
        .header("Accept", "application/json")
        .body(AsyncBody::empty())?;

    let response = http_client
        .send(request)
        .await
        .context("failed to fetch baseten deployments")?;

    let status = response.status();
    let body_bytes = {
        use futures::AsyncReadExt as _;
        let mut body = response.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes)
            .await
            .context("failed to read baseten deployments response")?;
        bytes
    };

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!(
            "baseten deployments API http {}: {}",
            status.as_u16(),
            body_text
        );
    }

    let parsed: BasetenDeploymentsResponse =
        serde_json::from_slice(&body_bytes).context("failed to parse deployments response")?;
    Ok(parsed.deployments)
}

fn collect_deployment_records(
    model_id: &str,
    deployments: &[BasetenDeployment],
) -> Vec<DeploymentRecord> {
    deployments
        .iter()
        .map(|deployment| DeploymentRecord {
            model_id: model_id.to_string(),
            model_version_id: deployment.id.clone(),
            experiment_name: deployment.name.clone(),
            environment: deployment
                .environment
                .clone()
                .unwrap_or_else(|| "none".to_string()),
            status: deployment
                .status
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            created_at: deployment
                .created_at
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        })
        .collect()
}

async fn run_sql_with_polling(
    http_client: Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    request: &serde_json::Value,
) -> Result<pull_examples::SnowflakeStatementResponse> {
    let mut response =
        pull_examples::run_sql(http_client.clone(), base_url, token, request).await?;

    if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
        let statement_handle = response
            .statement_handle
            .as_ref()
            .context("async query response missing statementHandle")?
            .clone();

        for _attempt in 1..=MAX_POLL_ATTEMPTS {
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
                break;
            }
        }

        if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
            anyhow::bail!(
                "query still running after {} poll attempts ({} seconds)",
                MAX_POLL_ATTEMPTS,
                MAX_POLL_ATTEMPTS as u64 * POLL_INTERVAL.as_secs()
            );
        }
    }

    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake error: code={} message={}",
                code,
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    Ok(response)
}

async fn fetch_existing_deployments(
    http_client: &Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    role: &Option<String>,
) -> Result<HashMap<String, ExistingDeployment>> {
    let statement = format!(
        r#"
SELECT
    event_properties:model_version_id::string AS model_version_id,
    event_properties:experiment_name::string AS experiment_name,
    event_properties:environment::string AS environment
FROM events
WHERE event_type = '{EDIT_PREDICTION_DEPLOYMENT_EVENT}'
"#
    );

    let request = json!({
        "statement": statement,
        "timeout": DEFAULT_STATEMENT_TIMEOUT_SECONDS,
        "database": "EVENTS",
        "schema": "PUBLIC",
        "warehouse": "DBT",
        "role": role,
    });

    let response = run_sql_with_polling(http_client.clone(), base_url, token, &request).await?;

    let col_names = ["model_version_id", "experiment_name", "environment"];
    let column_indices =
        pull_examples::get_column_indices(&response.result_set_meta_data, &col_names);

    let mut existing = HashMap::new();

    for data_row in &response.data {
        let get_string = |name: &str| -> Option<String> {
            let &index = column_indices.get(name)?;
            match data_row.get(index) {
                Some(JsonValue::String(s)) => Some(s.clone()),
                _ => None,
            }
        };

        let Some(model_version_id) = get_string("model_version_id") else {
            continue;
        };
        let experiment_name = get_string("experiment_name").unwrap_or_default();
        let environment = get_string("environment").unwrap_or_default();

        existing.insert(
            model_version_id,
            ExistingDeployment {
                experiment_name,
                environment,
            },
        );
    }

    Ok(existing)
}

async fn insert_deployment(
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
INSERT INTO events (event_type, event_properties, device_id, time)
VALUES (?, PARSE_JSON(?), 'ep-cli', CURRENT_TIMESTAMP())
"#;

    let bindings = json!({
        "1": { "type": "TEXT", "value": EDIT_PREDICTION_DEPLOYMENT_EVENT },
        "2": { "type": "TEXT", "value": event_properties_str }
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

    run_sql_with_polling(http_client.clone(), base_url, token, &request).await?;
    Ok(())
}

async fn update_deployment(
    http_client: &Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    role: &Option<String>,
    record: &DeploymentRecord,
) -> Result<()> {
    let statement = format!(
        r#"
UPDATE events
SET
    event_properties = OBJECT_INSERT(
        OBJECT_INSERT(event_properties, 'environment', ?::VARIANT, true),
        'experiment_name', ?::VARIANT, true
    ),
    time = CURRENT_TIMESTAMP()
WHERE event_type = '{EDIT_PREDICTION_DEPLOYMENT_EVENT}'
    AND event_properties:model_version_id::string = ?
"#
    );

    let bindings = json!({
        "1": { "type": "TEXT", "value": record.environment },
        "2": { "type": "TEXT", "value": record.experiment_name },
        "3": { "type": "TEXT", "value": record.model_version_id }
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

    run_sql_with_polling(http_client.clone(), base_url, token, &request).await?;
    Ok(())
}

fn display_deployments(existing: &HashMap<String, ExistingDeployment>) {
    let col_names = ["version_id", "experiment", "environment"];

    let mut col_widths: Vec<usize> = col_names.iter().map(|n| n.len()).collect();
    let mut rows: Vec<[String; 3]> = Vec::new();

    for (version_id, deployment) in existing {
        let row = [
            version_id.clone(),
            deployment.experiment_name.clone(),
            deployment.environment.clone(),
        ];
        for (i, val) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(val.len());
        }
        rows.push(row);
    }

    rows.sort_by(|a, b| a[2].cmp(&b[2]).then_with(|| a[1].cmp(&b[1])));

    let print_row = |values: &[&str]| {
        for (i, val) in values.iter().enumerate() {
            if i > 0 {
                eprint!("  ");
            }
            eprint!("{:width$}", val, width = col_widths[i]);
        }
        eprintln!();
    };

    eprintln!();
    print_row(&col_names);

    let separators: Vec<String> = col_widths.iter().map(|w| "─".repeat(*w)).collect();
    let separator_refs: Vec<&str> = separators.iter().map(|s| s.as_str()).collect();
    print_row(&separator_refs);

    for row in &rows {
        let refs: Vec<&str> = row.iter().map(|s| s.as_str()).collect();
        print_row(&refs);
    }
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

    eprintln!("Fetching existing deployments from Snowflake...");
    let mut existing = fetch_existing_deployments(
        &http_client,
        &snowflake_base_url,
        &snowflake_token,
        &snowflake_role,
    )
    .await
    .context("failed to fetch existing deployments from Snowflake")?;

    eprintln!(
        "Found {} existing deployment(s) in Snowflake.",
        existing.len()
    );

    let baseten_deployments = fetch_baseten_deployments(&http_client, &baseten_api_key, &model.id)
        .await
        .with_context(|| format!("failed to fetch deployments for model '{}'", model.name))?;

    let records = collect_deployment_records(&model.id, &baseten_deployments);

    if records.is_empty() {
        eprintln!("No deployments found on Baseten.");
        return Ok(());
    }

    eprintln!(
        "Found {} deployment(s) on Baseten for model '{}'.",
        records.len(),
        model.name
    );

    let mut inserts = Vec::new();
    let mut updates = Vec::new();
    let mut unchanged = 0;

    for record in &records {
        match existing.get(&record.model_version_id) {
            Some(existing_deployment) => {
                let environment_changed = existing_deployment.environment != record.environment;
                let experiment_changed =
                    existing_deployment.experiment_name != record.experiment_name;

                if environment_changed || experiment_changed {
                    updates.push(record);
                } else {
                    unchanged += 1;
                }
            }
            None => {
                inserts.push(record);
            }
        }
    }

    eprintln!(
        "Diff: {} insert(s), {} update(s), {} unchanged",
        inserts.len(),
        updates.len(),
        unchanged,
    );

    for (i, record) in inserts.iter().enumerate() {
        eprintln!(
            "  INSERT [{}/{}] {} -> {} (version_id={})",
            i + 1,
            inserts.len(),
            record.experiment_name,
            record.environment,
            record.model_version_id,
        );
        insert_deployment(
            &http_client,
            &snowflake_base_url,
            &snowflake_token,
            &snowflake_role,
            record,
        )
        .await
        .with_context(|| {
            format!(
                "failed to insert deployment '{}' (model_version_id={})",
                record.experiment_name, record.model_version_id
            )
        })?;

        existing.insert(
            record.model_version_id.clone(),
            ExistingDeployment {
                experiment_name: record.experiment_name.clone(),
                environment: record.environment.clone(),
            },
        );
    }

    for (i, record) in updates.iter().enumerate() {
        let existing_deployment = existing
            .get(&record.model_version_id)
            .context("update record missing from existing map")?;
        eprintln!(
            "  UPDATE [{}/{}] version_id={}: environment '{}' -> '{}', experiment '{}' -> '{}'",
            i + 1,
            updates.len(),
            record.model_version_id,
            existing_deployment.environment,
            record.environment,
            existing_deployment.experiment_name,
            record.experiment_name,
        );
        update_deployment(
            &http_client,
            &snowflake_base_url,
            &snowflake_token,
            &snowflake_role,
            record,
        )
        .await
        .with_context(|| {
            format!(
                "failed to update deployment '{}' (model_version_id={})",
                record.experiment_name, record.model_version_id
            )
        })?;

        existing.insert(
            record.model_version_id.clone(),
            ExistingDeployment {
                experiment_name: record.experiment_name.clone(),
                environment: record.environment.clone(),
            },
        );
    }

    if inserts.is_empty() && updates.is_empty() {
        eprintln!("All deployments up to date, no writes needed.");
    }

    display_deployments(&existing);

    Ok(())
}
