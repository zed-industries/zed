use anyhow::{Context as _, Result};
use http_client::{AsyncBody, HttpClient, Method, Request};
use indoc::indoc;
use serde::Deserialize;
use serde_json::{Value as JsonValue, json};
use std::sync::Arc;

use crate::{
    example::Example,
    progress::{InfoStyle, Progress, Step},
};
use edit_prediction::example_spec::ExampleSpec;

const SNOWFLAKE_SUCCESS_CODE: &str = "090001";
const EDIT_PREDICTION_EXAMPLE_CAPTURED_EVENT: &str = "Edit Prediction Example Captured";

const DEFAULT_STATEMENT_TIMEOUT_SECONDS: u64 = 120;

/// Parse an input token of the form `captured-after:{timestamp}`.
pub fn parse_captured_after_input(input: &str) -> Option<&str> {
    input.strip_prefix("captured-after:")
}

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
    let role = std::env::var("EP_SNOWFLAKE_ROLE").ok();

    let mut all_examples = Vec::new();

    for after_date in after_timestamps.iter() {
        let step_progress_name = format!(">{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let statement = indoc! {r#"
            SELECT
                event_properties:example AS example
            FROM events
            WHERE event_type = ?
                AND time > TRY_TO_TIMESTAMP_NTZ(?)
            ORDER BY time ASC
            LIMIT ?
        "#};

        let request = json!({
            "statement": statement,
            "timeout": DEFAULT_STATEMENT_TIMEOUT_SECONDS,
            "database": "EVENTS",
            "schema": "PUBLIC",
            "warehouse": "DBT",
            "role": role,
            "bindings": {
                "1": { "type": "TEXT", "value": EDIT_PREDICTION_EXAMPLE_CAPTURED_EVENT },
                "2": { "type": "TEXT", "value": after_date },
                "3": { "type": "FIXED", "value": max_rows_per_timestamp.to_string() }
            }
        });

        let response = run_sql(http_client.clone(), &base_url, &token, &request).await?;

        step_progress.set_info(format!("{} rows", response.data.len()), InfoStyle::Normal);
        step_progress.set_substatus("parsing");

        all_examples.extend(examples_from_response(&response)?);

        step_progress.set_substatus("done");
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

fn examples_from_response(
    response: &SnowflakeStatementResponse,
) -> Result<impl Iterator<Item = Example>> {
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

    let iter = response.data.iter().enumerate().filter_map(move |(row_index, data_row)| {
        let Some(example_value) = data_row.get(example_index) else {
            return None;
        };
        if example_value.is_null() {
            return None;
        }

        let parse_result = match example_value {
            JsonValue::String(encoded_json) => serde_json::from_str::<ExampleSpec>(encoded_json),
            _ => serde_json::from_value::<ExampleSpec>(example_value.clone()),
        };

        match parse_result {
            Ok(spec) => Some(Example {
                spec,
                buffer: None,
                context: None,
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                state: None,
            }),
            Err(error) => {
                let raw_json = serde_json::to_string_pretty(example_value)
                    .unwrap_or_else(|_| "<failed to serialize json>".to_string());
                log::error!(
                    "failed to parse ExampleSpec for row {row_index}: {error:#}\nraw json:\n{raw_json}"
                );
                None
            }
        }
    });

    Ok(iter)
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
