use anyhow::{Context as _, Result};
use http_client::{AsyncBody, HttpClient, Method, Request};
use indoc::indoc;
use serde::Deserialize;
use serde_json::{Value as JsonValue, json};
use std::{collections::BTreeSet, sync::Arc};

use crate::{
    example::Example,
    progress::{InfoStyle, Progress, Step},
};

const SNOWFLAKE_SUCCESS_CODE: &str = "090001";
const EDIT_PREDICTION_EXAMPLE_CAPTURED_EVENT: &str = "Edit Prediction Example Captured";

const DEFAULT_STATEMENT_TIMEOUT_SECONDS: u64 = 120;

/// Parse an input token of the form `captured-after:{timestamp}`.
///
/// Returns the timestamp string if the token matches, otherwise `None`.
pub fn parse_captured_after_input(input: &str) -> Option<&str> {
    input.strip_prefix("captured-after:")
}

/// Fetch captured examples from Snowflake for each `captured-after:{timestamp}` token.
///
/// - Each `captured-after:{timestamp}` input produces examples independently, and results are
///   concatenated.
/// - The caller is responsible for applying any global limit (`--limit`) after all inputs are loaded.
/// - Uses the application's configured HTTP client (do not construct a separate client).
///
/// Required env vars:
/// - `EP_SNOWFLAKE_API_KEY`
/// - `EP_SNOWFLAKE_BASE_URL`
/// - `EP_SNOWFLAKE_EVENTS_TABLE`
/// - `EP_SNOWFLAKE_DATABASE`
/// - `EP_SNOWFLAKE_SCHEMA`
/// - `EP_SNOWFLAKE_WAREHOUSE`
///
/// Optional env vars:
/// - `EP_SNOWFLAKE_ROLE`
pub async fn fetch_captured_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: usize,
) -> Result<Vec<Example>> {
    if after_timestamps.is_empty() {
        return Ok(Vec::new());
    }

    let progress = Progress::global();

    let token = std::env::var("EP_SNOWFLAKE_API_KEY")
        .context("missing required environment variable EP_SNOWFLAKE_API_KEY")?;
    let base_url = std::env::var("EP_SNOWFLAKE_BASE_URL").context(
        "missing required environment variable EP_SNOWFLAKE_BASE_URL (e.g. https://<account>.snowflakecomputing.com)",
    )?;
    let database = std::env::var("EP_SNOWFLAKE_DATABASE")
        .context("missing required environment variable EP_SNOWFLAKE_DATABASE")?;
    let schema = std::env::var("EP_SNOWFLAKE_SCHEMA")
        .context("missing required environment variable EP_SNOWFLAKE_SCHEMA")?;
    let warehouse = std::env::var("EP_SNOWFLAKE_WAREHOUSE")
        .context("missing required environment variable EP_SNOWFLAKE_WAREHOUSE")?;
    let role = std::env::var("EP_SNOWFLAKE_ROLE").ok();
    let events_table = std::env::var("EP_SNOWFLAKE_EVENTS_TABLE")
        .context("missing required environment variable EP_SNOWFLAKE_EVENTS_TABLE")?;

    let mut all_examples = Vec::new();
    let mut seen_specs: BTreeSet<(String, String, String)> = BTreeSet::new();

    for after_date in after_timestamps.iter() {
        let step_progress_name = format!(">{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let statement = format!(
            indoc! {r#"
                SELECT
                  event_properties:example AS example
                FROM {}
                WHERE event_type = ?
                  AND time > TRY_TO_TIMESTAMP_NTZ(?)
                ORDER BY time ASC
                LIMIT ?
            "#},
            events_table
        );

        let request = json!({
            "statement": statement,
            "timeout": DEFAULT_STATEMENT_TIMEOUT_SECONDS,
            "database": database,
            "schema": schema,
            "warehouse": warehouse,
            "role": role,
            "bindings": {
                "1": { "type": "TEXT", "value": EDIT_PREDICTION_EXAMPLE_CAPTURED_EVENT },
                "2": { "type": "TEXT", "value": after_date },
                "3": { "type": "FIXED", "value": max_rows_per_timestamp.to_string() }
            }
        });

        let response = run_sql(http_client.clone(), &base_url, &token, &request).await?;
        let rows = extract_rows(&response)?;

        step_progress.set_info(format!("{} rows", rows.len()), InfoStyle::Normal);

        step_progress.set_substatus(format!("parsing {} rows", rows.len()));

        let mut parsed = 0usize;
        let mut skipped_null = 0usize;
        let mut parse_failed = 0usize;

        for (row_index, row) in rows.into_iter().enumerate() {
            let Some(example_value) = row.example else {
                skipped_null += 1;
                continue;
            };

            let spec = match serde_json::from_value::<edit_prediction::example_spec::ExampleSpec>(
                example_value.clone(),
            ) {
                Ok(spec) => spec,
                Err(error) => {
                    parse_failed += 1;
                    let raw_json = serde_json::to_string_pretty(&example_value)
                        .unwrap_or_else(|_| "<failed to serialize json>".to_string());
                    log::error!(
                        "failed to parse ExampleSpec for row {row_index}: {error:#}\nraw json:\n{raw_json}"
                    );
                    continue;
                }
            };

            let dedupe_key = (
                spec.repository_url.clone(),
                spec.revision.clone(),
                spec.name.clone(),
            );
            if !seen_specs.insert(dedupe_key) {
                continue;
            }

            all_examples.push(Example {
                spec,
                buffer: None,
                context: None,
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                state: None,
            });
            parsed += 1;
        }

        step_progress.set_substatus(format!(
            "done (parsed={}, skipped_null={}, parse_failed={})",
            parsed, skipped_null, parse_failed
        ));
    }

    Ok(all_examples)
}

#[derive(Debug, Clone, Deserialize)]
struct SnowflakeStatementResponse {
    #[serde(default)]
    data: Vec<Vec<JsonValue>>,
    #[serde(default)]
    result_set_meta_data: Option<SnowflakeResultSetMetaData>,
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct SnowflakeResultSetMetaData {
    #[serde(default, rename = "rowType")]
    row_type: Vec<SnowflakeColumnMeta>,
}

#[derive(Debug, Clone, Deserialize)]
struct SnowflakeColumnMeta {
    #[serde(default)]
    name: String,
}

#[derive(Debug, Clone)]
struct SnowflakeRow {
    example: Option<JsonValue>,
}

fn extract_rows(response: &SnowflakeStatementResponse) -> Result<Vec<SnowflakeRow>> {
    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake sql api returned error code={code} message={}",
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    let example_index = response
        .result_set_meta_data
        .as_ref()
        .and_then(|m| {
            m.row_type.iter().enumerate().find_map(|(index, col)| {
                if col.name.eq_ignore_ascii_case("example") {
                    Some(index)
                } else {
                    None
                }
            })
        })
        .unwrap_or(0);

    let mut rows = Vec::with_capacity(response.data.len());
    for data_row in &response.data {
        let example = data_row.get(example_index).cloned();
        let example = match example {
            Some(JsonValue::Null) | None => None,
            Some(value) => Some(value),
        };
        rows.push(SnowflakeRow { example });
    }
    Ok(rows)
}

async fn run_sql(
    http_client: Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    request: &serde_json::Value,
) -> Result<SnowflakeStatementResponse> {
    let url = format!("{}/api/v2/statements", base_url.trim_end_matches('/'));

    let request_body =
        serde_json::to_vec(request).context("failed to serialize Snowflake SQL API request")?;

    let http_request = Request::builder()
        .method(Method::POST)
        .uri(url.as_str())
        .header("Authorization", format!("Bearer {token}"))
        .header(
            "X-Snowflake-Authorization-Token-Type",
            "PROGRAMMATIC_ACCESS_TOKEN",
        )
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(AsyncBody::from(request_body.clone()))?;

    let response = http_client
        .send(http_request)
        .await
        .context("failed to send request to Snowflake SQL API")?;

    let status = response.status();
    let body_bytes = {
        use futures::AsyncReadExt as _;

        let mut body = response.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes)
            .await
            .context("failed to read Snowflake SQL API response body")?;
        bytes
    };

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!("snowflake sql api http {}: {}", status.as_u16(), body_text);
    }

    serde_json::from_slice::<SnowflakeStatementResponse>(&body_bytes)
        .context("failed to parse Snowflake SQL API response JSON")
}
