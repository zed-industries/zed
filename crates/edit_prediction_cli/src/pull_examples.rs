use anyhow::{Context as _, Result};
use flate2::read::GzDecoder;
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request};
use indoc::indoc;
use serde::Deserialize;
use serde_json::{Value as JsonValue, json};
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use telemetry_events::EditPredictionRating;

use zeta_prompt::ZetaPromptInput;

use crate::example::Example;
use crate::progress::{InfoStyle, Progress, Step};
use edit_prediction::example_spec::{
    CapturedEvent, CapturedPromptInput, CapturedRelatedExcerpt, CapturedRelatedFile, ExampleSpec,
    TelemetrySource,
};
use std::fmt::Write as _;

const SNOWFLAKE_SUCCESS_CODE: &str = "090001";
const SNOWFLAKE_ASYNC_IN_PROGRESS_CODE: &str = "333334";
const EDIT_PREDICTION_EXAMPLE_CAPTURED_EVENT: &str = "Edit Prediction Example Captured";
const PREDICTIVE_EDIT_REQUESTED_EVENT: &str = "Predictive Edit Requested";
const PREDICTIVE_EDIT_REJECTED_EVENT: &str = "Predictive Edit Rejected";
const EDIT_PREDICTION_RATED_EVENT: &str = "Edit Prediction Rated";

const DEFAULT_STATEMENT_TIMEOUT_SECONDS: u64 = 120;
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const MAX_POLL_ATTEMPTS: usize = 120;

/// Parse an input token of the form `captured-after:{timestamp}`.
pub fn parse_captured_after_input(input: &str) -> Option<&str> {
    input.strip_prefix("captured-after:")
}

/// Parse an input token of the form `rejected-after:{timestamp}`.
pub fn parse_rejected_after_input(input: &str) -> Option<&str> {
    input.strip_prefix("rejected-after:")
}

/// Parse an input token of the form `requested-after:{timestamp}`.
pub fn parse_requested_after_input(input: &str) -> Option<&str> {
    input.strip_prefix("requested-after:")
}

/// Parse an input token of the form `rated-after:{timestamp}`, `rated-positive-after:{timestamp}`,
/// or `rated-negative-after:{timestamp}`.
/// Returns `(timestamp, Option<EditPredictionRating>)` where `None` means all ratings.
pub fn parse_rated_after_input(input: &str) -> Option<(&str, Option<EditPredictionRating>)> {
    if let Some(timestamp) = input.strip_prefix("rated-positive-after:") {
        Some((timestamp, Some(EditPredictionRating::Positive)))
    } else if let Some(timestamp) = input.strip_prefix("rated-negative-after:") {
        Some((timestamp, Some(EditPredictionRating::Negative)))
    } else if let Some(timestamp) = input.strip_prefix("rated-after:") {
        Some((timestamp, None))
    } else {
        None
    }
}

pub async fn fetch_captured_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: usize,
    background_executor: BackgroundExecutor,
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

        let response = run_sql_with_polling(
            http_client.clone(),
            &base_url,
            &token,
            &request,
            &step_progress,
            background_executor.clone(),
        )
        .await?;

        let total_rows = response
            .result_set_meta_data
            .as_ref()
            .and_then(|m| m.num_rows)
            .unwrap_or(response.data.len() as i64);

        let num_partitions = response
            .result_set_meta_data
            .as_ref()
            .map(|m| m.partition_info.len())
            .unwrap_or(1)
            .max(1);

        step_progress.set_info(format!("{} rows", total_rows), InfoStyle::Normal);
        step_progress.set_substatus("parsing");

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

        all_examples.extend(examples_from_response(&response, example_index)?);

        if num_partitions > 1 {
            let statement_handle = response
                .statement_handle
                .as_ref()
                .context("response has multiple partitions but no statementHandle")?;

            for partition in 1..num_partitions {
                step_progress.set_substatus(format!(
                    "fetching partition {}/{}",
                    partition + 1,
                    num_partitions
                ));

                let partition_response = fetch_partition(
                    http_client.clone(),
                    &base_url,
                    &token,
                    statement_handle,
                    partition,
                )
                .await?;

                all_examples.extend(examples_from_response(&partition_response, example_index)?);
            }
        }

        step_progress.set_substatus("done");
    }

    Ok(all_examples)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SnowflakeStatementResponse {
    #[serde(default)]
    data: Vec<Vec<JsonValue>>,
    #[serde(default)]
    result_set_meta_data: Option<SnowflakeResultSetMetaData>,
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    statement_handle: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SnowflakeResultSetMetaData {
    #[serde(default, rename = "rowType")]
    row_type: Vec<SnowflakeColumnMeta>,
    #[serde(default)]
    num_rows: Option<i64>,
    #[serde(default)]
    partition_info: Vec<SnowflakePartitionInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SnowflakePartitionInfo {}

#[derive(Debug, Clone, Deserialize)]
struct SnowflakeColumnMeta {
    #[serde(default)]
    name: String,
}

fn examples_from_response(
    response: &SnowflakeStatementResponse,
    example_index: usize,
) -> Result<impl Iterator<Item = Example> + '_> {
    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake sql api returned error code={code} message={}",
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

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
                prompt_inputs: None,
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                qa: Vec::new(),
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

async fn run_sql_with_polling(
    http_client: Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    request: &serde_json::Value,
    step_progress: &crate::progress::StepProgress,
    background_executor: BackgroundExecutor,
) -> Result<SnowflakeStatementResponse> {
    let mut response = run_sql(http_client.clone(), base_url, token, request).await?;

    if response.code.as_deref() == Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
        let statement_handle = response
            .statement_handle
            .as_ref()
            .context("async query response missing statementHandle")?
            .clone();

        for attempt in 1..=MAX_POLL_ATTEMPTS {
            step_progress.set_substatus(format!("polling ({attempt})"));

            background_executor.timer(POLL_INTERVAL).await;

            response =
                fetch_partition(http_client.clone(), base_url, token, &statement_handle, 0).await?;

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

    Ok(response)
}

async fn fetch_partition(
    http_client: Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    statement_handle: &str,
    partition: usize,
) -> Result<SnowflakeStatementResponse> {
    let url = format!(
        "{}/api/v2/statements/{}?partition={}",
        base_url.trim_end_matches('/'),
        statement_handle,
        partition
    );

    let http_request = Request::builder()
        .method(Method::GET)
        .uri(url.as_str())
        .header("Authorization", format!("Bearer {token}"))
        .header(
            "X-Snowflake-Authorization-Token-Type",
            "PROGRAMMATIC_ACCESS_TOKEN",
        )
        .header("Accept", "application/json")
        .header("Accept-Encoding", "gzip")
        .header("User-Agent", "edit_prediction_cli")
        .body(AsyncBody::empty())?;

    let response = http_client
        .send(http_request)
        .await
        .context("failed to send partition request to Snowflake SQL API")?;

    let status = response.status();
    let content_encoding = response
        .headers()
        .get("content-encoding")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_lowercase());

    let body_bytes = {
        use futures::AsyncReadExt as _;

        let mut body = response.into_body();
        let mut bytes = Vec::new();
        body.read_to_end(&mut bytes)
            .await
            .context("failed to read Snowflake SQL API partition response body")?;
        bytes
    };

    let body_bytes = if content_encoding.as_deref() == Some("gzip") {
        let mut decoder = GzDecoder::new(&body_bytes[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .context("failed to decompress gzip response")?;
        decompressed
    } else {
        body_bytes
    };

    if !status.is_success() && status.as_u16() != 202 {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!(
            "snowflake sql api partition request http {}: {}",
            status.as_u16(),
            body_text
        );
    }

    if body_bytes.is_empty() {
        anyhow::bail!(
            "snowflake sql api partition {} returned empty response body (http {})",
            partition,
            status.as_u16()
        );
    }

    serde_json::from_slice::<SnowflakeStatementResponse>(&body_bytes).with_context(|| {
        let body_preview = String::from_utf8_lossy(&body_bytes[..body_bytes.len().min(500)]);
        format!(
            "failed to parse Snowflake SQL API partition {} response JSON (http {}): {}",
            partition,
            status.as_u16(),
            body_preview
        )
    })
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
        .header("User-Agent", "edit_prediction_cli")
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

    if !status.is_success() && status.as_u16() != 202 {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!("snowflake sql api http {}: {}", status.as_u16(), body_text);
    }

    serde_json::from_slice::<SnowflakeStatementResponse>(&body_bytes)
        .context("failed to parse Snowflake SQL API response JSON")
}

pub async fn fetch_rejected_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: usize,
    background_executor: BackgroundExecutor,
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
        let step_progress_name = format!("rejected>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        // Join rejected events with their corresponding request events to get the full context.
        // We filter for V3 sampling data which contains the structured input we need.
        // We also filter for predictions that were actually shown to the user (was_shown = true)
        // to focus on explicit user rejections rather than implicit cancellations.
        let statement = indoc! {r#"
            SELECT
                req.event_properties:request_id::string AS request_id,
                req.device_id::string AS device_id,
                req.time::string AS time,
                req.event_properties:input AS input,
                req.event_properties:prompt::string AS prompt,
                req.event_properties:output::string AS output,
                rej.event_properties:was_shown::boolean AS was_shown,
                rej.event_properties:reason::string AS reason
            FROM events req
            INNER JOIN events rej
                ON req.event_properties:request_id = rej.event_properties:request_id
            WHERE req.event_type = ?
                AND rej.event_type = ?
                AND req.event_properties:version = 'V3'
                AND rej.event_properties:was_shown = true
                AND req.time > TRY_TO_TIMESTAMP_NTZ(?)
            ORDER BY req.time ASC
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
                "1": { "type": "TEXT", "value": PREDICTIVE_EDIT_REQUESTED_EVENT },
                "2": { "type": "TEXT", "value": PREDICTIVE_EDIT_REJECTED_EVENT },
                "3": { "type": "TEXT", "value": after_date },
                "4": { "type": "FIXED", "value": max_rows_per_timestamp.to_string() }
            }
        });

        let response = run_sql_with_polling(
            http_client.clone(),
            &base_url,
            &token,
            &request,
            &step_progress,
            background_executor.clone(),
        )
        .await?;

        let total_rows = response
            .result_set_meta_data
            .as_ref()
            .and_then(|m| m.num_rows)
            .unwrap_or(response.data.len() as i64);

        let num_partitions = response
            .result_set_meta_data
            .as_ref()
            .map(|m| m.partition_info.len())
            .unwrap_or(1)
            .max(1);

        step_progress.set_info(format!("{} rows", total_rows), InfoStyle::Normal);
        step_progress.set_substatus("parsing");

        let column_indices = get_column_indices(
            &response.result_set_meta_data,
            &[
                "request_id",
                "device_id",
                "time",
                "input",
                "prompt",
                "output",
                "was_shown",
                "reason",
            ],
        );

        all_examples.extend(rejected_examples_from_response(&response, &column_indices)?);

        if num_partitions > 1 {
            let statement_handle = response
                .statement_handle
                .as_ref()
                .context("response has multiple partitions but no statementHandle")?;

            for partition in 1..num_partitions {
                step_progress.set_substatus(format!(
                    "fetching partition {}/{}",
                    partition + 1,
                    num_partitions
                ));

                let partition_response = fetch_partition(
                    http_client.clone(),
                    &base_url,
                    &token,
                    statement_handle,
                    partition,
                )
                .await?;

                all_examples.extend(rejected_examples_from_response(
                    &partition_response,
                    &column_indices,
                )?);
            }
        }

        step_progress.set_substatus("done");
    }

    Ok(all_examples)
}

pub async fn fetch_requested_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: usize,
    background_executor: BackgroundExecutor,
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
        let step_progress_name = format!("requested>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let statement = indoc! {r#"
            SELECT
                req.event_properties:request_id::string AS request_id,
                req.device_id::string AS device_id,
                req.time::string AS time,
                req.event_properties:input AS input
            FROM events req
            WHERE req.event_type = ?
                AND req.event_properties:version = 'V3'
                AND req.time > TRY_TO_TIMESTAMP_NTZ(?)
            ORDER BY req.time ASC
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
                "1": { "type": "TEXT", "value": PREDICTIVE_EDIT_REQUESTED_EVENT },
                "2": { "type": "TEXT", "value": after_date },
                "3": { "type": "FIXED", "value": max_rows_per_timestamp.to_string() }
            }
        });

        let response = run_sql_with_polling(
            http_client.clone(),
            &base_url,
            &token,
            &request,
            &step_progress,
            background_executor.clone(),
        )
        .await?;

        let total_rows = response
            .result_set_meta_data
            .as_ref()
            .and_then(|m| m.num_rows)
            .unwrap_or(response.data.len() as i64);

        let num_partitions = response
            .result_set_meta_data
            .as_ref()
            .map(|m| m.partition_info.len())
            .unwrap_or(1)
            .max(1);

        step_progress.set_info(format!("{} rows", total_rows), InfoStyle::Normal);
        step_progress.set_substatus("parsing");

        let column_indices = get_column_indices(
            &response.result_set_meta_data,
            &["request_id", "device_id", "time", "input"],
        );

        all_examples.extend(requested_examples_from_response(
            &response,
            &column_indices,
        )?);

        if num_partitions > 1 {
            let statement_handle = response
                .statement_handle
                .as_ref()
                .context("response has multiple partitions but no statementHandle")?;

            for partition in 1..num_partitions {
                step_progress.set_substatus(format!(
                    "fetching partition {}/{}",
                    partition + 1,
                    num_partitions
                ));

                let partition_response = fetch_partition(
                    http_client.clone(),
                    &base_url,
                    &token,
                    statement_handle,
                    partition,
                )
                .await?;

                all_examples.extend(requested_examples_from_response(
                    &partition_response,
                    &column_indices,
                )?);
            }
        }

        step_progress.set_substatus("done");
    }

    Ok(all_examples)
}

pub async fn fetch_rated_examples_after(
    http_client: Arc<dyn HttpClient>,
    inputs: &[(String, Option<EditPredictionRating>)],
    max_rows_per_timestamp: usize,
    background_executor: BackgroundExecutor,
) -> Result<Vec<Example>> {
    if inputs.is_empty() {
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

    for (after_date, rating_filter) in inputs.iter() {
        let filter_label = match rating_filter {
            None => "",
            Some(EditPredictionRating::Positive) => ":positive",
            Some(EditPredictionRating::Negative) => ":negative",
        };
        let step_progress_name = format!("rated{filter_label}>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let rating_value = rating_filter.as_ref().map(|r| match r {
            EditPredictionRating::Positive => "Positive",
            EditPredictionRating::Negative => "Negative",
        });

        let statement = indoc! {r#"
            SELECT
                event_properties:inputs AS inputs,
                event_properties:output::string AS output,
                event_properties:rating::string AS rating,
                event_properties:feedback::string AS feedback,
                device_id::string AS device_id,
                time::string AS time
            FROM events
            WHERE event_type = ?
                AND (? IS NULL OR event_properties:rating::string = ?)
                AND time > TRY_TO_TIMESTAMP_NTZ(?)
                AND event_properties:inputs IS NOT NULL
                AND event_properties:inputs:cursor_excerpt IS NOT NULL
                AND event_properties:output IS NOT NULL
            ORDER BY time ASC
            LIMIT ?
        "#};

        let bindings = json!({
            "1": { "type": "TEXT", "value": EDIT_PREDICTION_RATED_EVENT },
            "2": { "type": "TEXT", "value": rating_value },
            "3": { "type": "TEXT", "value": rating_value },
            "4": { "type": "TEXT", "value": after_date },
            "5": { "type": "FIXED", "value": max_rows_per_timestamp.to_string() }
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

        let response = run_sql_with_polling(
            http_client.clone(),
            &base_url,
            &token,
            &request,
            &step_progress,
            background_executor.clone(),
        )
        .await?;

        let total_rows = response
            .result_set_meta_data
            .as_ref()
            .and_then(|m| m.num_rows)
            .unwrap_or(response.data.len() as i64);

        let num_partitions = response
            .result_set_meta_data
            .as_ref()
            .map(|m| m.partition_info.len())
            .unwrap_or(1)
            .max(1);

        step_progress.set_info(format!("{} rows", total_rows), InfoStyle::Normal);
        step_progress.set_substatus("parsing");

        let column_indices = get_column_indices(
            &response.result_set_meta_data,
            &[
                "inputs",
                "output",
                "rating",
                "feedback",
                "device_id",
                "time",
            ],
        );

        all_examples.extend(rated_examples_from_response(&response, &column_indices)?);

        if num_partitions > 1 {
            let statement_handle = response
                .statement_handle
                .as_ref()
                .context("response has multiple partitions but no statementHandle")?;

            for partition in 1..num_partitions {
                step_progress.set_substatus(format!(
                    "fetching partition {}/{}",
                    partition + 1,
                    num_partitions
                ));

                let partition_response = fetch_partition(
                    http_client.clone(),
                    &base_url,
                    &token,
                    statement_handle,
                    partition,
                )
                .await?;

                all_examples.extend(rated_examples_from_response(
                    &partition_response,
                    &column_indices,
                )?);
            }
        }

        step_progress.set_substatus("done");
    }

    Ok(all_examples)
}

fn rated_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<impl Iterator<Item = Example> + 'a> {
    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake sql api returned error code={code} message={}",
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    let iter = response
        .data
        .iter()
        .enumerate()
        .filter_map(move |(row_index, data_row)| {
            let get_string = |name: &str| -> Option<String> {
                let index = column_indices.get(name).copied()?;
                match data_row.get(index)? {
                    JsonValue::String(s) => Some(s.clone()),
                    JsonValue::Null => None,
                    other => Some(other.to_string()),
                }
            };

            let get_json = |name: &str| -> Option<JsonValue> {
                let index = column_indices.get(name).copied()?;
                let value = data_row.get(index)?;
                if value.is_null() {
                    return None;
                }
                match value {
                    JsonValue::String(s) => serde_json::from_str(s).ok(),
                    other => Some(other.clone()),
                }
            };

            let inputs_json = get_json("inputs");
            let inputs: Option<ZetaPromptInput> = match &inputs_json {
                Some(v) => match serde_json::from_value(v.clone()) {
                    Ok(parsed) => Some(parsed),
                    Err(e) => {
                        log::warn!(
                            "skipping row {row_index}: failed to parse inputs - {e}",
                        );
                        return None;
                    }
                },
                None => None,
            };
            let output = get_string("output");
            let rating = get_string("rating");
            let feedback = get_string("feedback").unwrap_or_default();
            let device_id = get_string("device_id");
            let time = get_string("time");

            match (inputs, output.clone(), rating.clone(), device_id.clone(), time.clone()) {
                (Some(inputs), Some(output), Some(rating), Some(device_id), Some(time)) => {
                    Some(build_rated_example(
                        device_id,
                        time,
                        inputs,
                        output,
                        rating,
                        feedback,
                    ))
                }
                _ => {
                    log::warn!(
                        "skipping row {row_index}: missing fields - inputs={:?} output={:?} rating={:?} device_id={:?} time={:?}",
                        inputs_json.is_some(),
                        output.is_some(),
                        rating.is_some(),
                        device_id.is_some(),
                        time.is_some(),
                    );
                    None
                }
            }
        });

    Ok(iter)
}

fn build_rated_example(
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    output: String,
    rating: String,
    feedback: String,
) -> Example {
    let parsed_rating = if rating == "Positive" {
        EditPredictionRating::Positive
    } else {
        EditPredictionRating::Negative
    };
    let is_positive = parsed_rating == EditPredictionRating::Positive;
    let request_id = format!("rated-{}-{}", device_id, time);

    let tags = if is_positive {
        vec!["rated:positive".to_string()]
    } else {
        vec!["rated:negative".to_string()]
    };

    let mut example = build_example_from_snowflake(request_id, device_id, time, input, tags, None);

    example.spec.rating = Some(parsed_rating);

    if !feedback.is_empty() {
        example
            .spec
            .human_feedback
            .push(edit_prediction::example_spec::HumanFeedback { message: feedback });
    }

    if is_positive {
        example.spec.expected_patches = vec![output];
    } else {
        example.spec.rejected_patch = Some(output);
    }

    example
}

fn requested_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<impl Iterator<Item = Example> + 'a> {
    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake sql api returned error code={code} message={}",
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    let iter = response
        .data
        .iter()
        .enumerate()
        .filter_map(move |(row_index, data_row)| {
            let get_string = |name: &str| -> Option<String> {
                let index = column_indices.get(name).copied()?;
                match data_row.get(index)? {
                    JsonValue::String(s) => Some(s.clone()),
                    JsonValue::Null => None,
                    other => Some(other.to_string()),
                }
            };

            let get_json = |name: &str| -> Option<JsonValue> {
                let index = column_indices.get(name).copied()?;
                let value = data_row.get(index)?;
                if value.is_null() {
                    return None;
                }
                match value {
                    JsonValue::String(s) => serde_json::from_str(s).ok(),
                    other => Some(other.clone()),
                }
            };

            let request_id_str = get_string("request_id");
            let device_id = get_string("device_id");
            let time = get_string("time");
            let input_json = get_json("input");
            let input: Option<ZetaPromptInput> =
                input_json.clone().and_then(|v| serde_json::from_value(v).ok());

            match (request_id_str.clone(), device_id.clone(), time.clone(), input) {
                (Some(request_id), Some(device_id), Some(time), Some(input)) => {
                    Some(build_example_from_snowflake(
                        request_id,
                        device_id,
                        time,
                        input,
                        vec!["requested".to_string()],
                        None,
                    ))
                }
                _ => {
                    log::warn!(
                        "skipping row {row_index}: missing fields - request_id={:?} device_id={:?} time={:?} input={:?}",
                        request_id_str.is_some(),
                        device_id.is_some(),
                        time.is_some(),
                        input_json.is_some(),
                    );
                    None
                }
            }
        });

    Ok(iter)
}

fn rejected_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<impl Iterator<Item = Example> + 'a> {
    if let Some(code) = &response.code {
        if code != SNOWFLAKE_SUCCESS_CODE {
            anyhow::bail!(
                "snowflake sql api returned error code={code} message={}",
                response.message.as_deref().unwrap_or("<no message>")
            );
        }
    }

    let iter = response
        .data
        .iter()
        .enumerate()
        .filter_map(move |(row_index, data_row)| {
            let get_string = |name: &str| -> Option<String> {
                let index = column_indices.get(name).copied()?;
                match data_row.get(index)? {
                    JsonValue::String(s) => Some(s.clone()),
                    JsonValue::Null => None,
                    other => Some(other.to_string()),
                }
            };

            let get_json = |name: &str| -> Option<JsonValue> {
                let index = column_indices.get(name).copied()?;
                let value = data_row.get(index)?;
                if value.is_null() {
                    return None;
                }
                match value {
                    JsonValue::String(s) => serde_json::from_str(s).ok(),
                    other => Some(other.clone()),
                }
            };

            let get_bool = |name: &str| -> Option<bool> {
                let index = column_indices.get(name).copied()?;
                match data_row.get(index)? {
                    JsonValue::Bool(b) => Some(*b),
                    JsonValue::String(s) => s.parse().ok(),
                    _ => None,
                }
            };

            let request_id_str = get_string("request_id");
            let device_id = get_string("device_id");
            let time = get_string("time");
            let input_json = get_json("input");
            let input: Option<ZetaPromptInput> =
                input_json.clone().and_then(|v| serde_json::from_value(v).ok());
            let output = get_string("output");
            let was_shown = get_bool("was_shown");
            let reason = get_string("reason");

            match (request_id_str.clone(), device_id.clone(), time.clone(), input, output.clone(), was_shown, reason.clone()) {
                (Some(request_id), Some(device_id), Some(time), Some(input), Some(output), Some(was_shown), Some(reason)) => {
                    Some(build_rejected_example(
                        request_id,
                        device_id,
                        time,
                        input,
                        output,
                        was_shown,
                        reason,
                    ))
                }
                _ => {
                    log::warn!(
                        "skipping row {row_index}: missing fields - request_id={:?} device_id={:?} time={:?} input={:?} output={:?} was_shown={:?} reason={:?}",
                        request_id_str.is_some(),
                        device_id.is_some(),
                        time.is_some(),
                        input_json.is_some(),
                        output.is_some(),
                        was_shown.is_some(),
                        reason.is_some()
                    );
                    None
                }
            }
        });

    Ok(iter)
}

fn build_rejected_example(
    request_id: String,
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    output: String,
    was_shown: bool,
    reason: String,
) -> Example {
    let rejected_patch = build_output_patch(
        &input.cursor_path,
        input.cursor_excerpt.as_ref(),
        &input.editable_range_in_excerpt,
        &output,
    );
    let mut example = build_example_from_snowflake(
        request_id,
        device_id,
        time,
        input,
        vec![format!("rejection:{}", reason.to_lowercase())],
        Some(RejectionInfo { reason, was_shown }),
    );
    example.spec.rejected_patch = Some(rejected_patch);
    example
}

struct RejectionInfo {
    reason: String,
    was_shown: bool,
}

fn build_example_from_snowflake(
    request_id: String,
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    tags: Vec<String>,
    rejection: Option<RejectionInfo>,
) -> Example {
    let events: Vec<CapturedEvent> = input
        .events
        .iter()
        .map(|event| match event.as_ref() {
            zeta_prompt::Event::BufferChange {
                path,
                old_path,
                diff,
                predicted,
                in_open_source_repo,
            } => CapturedEvent {
                path: path.clone(),
                old_path: old_path.clone(),
                diff: diff.clone(),
                predicted: *predicted,
                in_open_source_repo: *in_open_source_repo,
            },
        })
        .collect();

    let related_files: Vec<CapturedRelatedFile> = input
        .related_files
        .iter()
        .map(|rf| CapturedRelatedFile {
            path: rf.path.clone(),
            max_row: rf.max_row,
            excerpts: rf
                .excerpts
                .iter()
                .map(|e| CapturedRelatedExcerpt {
                    row_range: e.row_range.clone(),
                    text: e.text.to_string(),
                })
                .collect(),
        })
        .collect();

    let cursor_excerpt = input.cursor_excerpt.as_ref();
    let cursor_offset = input.cursor_offset_in_excerpt;

    let (cursor_row, cursor_column) = compute_row_column(cursor_excerpt, cursor_offset);

    let mut edit_history = String::new();
    for event in &input.events {
        zeta_prompt::write_event(&mut edit_history, event);
        edit_history.push('\n');
    }

    let (rejection_reason, was_shown) = match &rejection {
        Some(r) => (r.reason.clone(), r.was_shown),
        None => (String::new(), false),
    };

    let spec = ExampleSpec {
        name: request_id.clone(),
        repository_url: String::new(),
        revision: String::new(),
        tags,
        reasoning: None,
        uncommitted_diff: String::new(),
        cursor_path: input.cursor_path.clone(),
        cursor_position: build_cursor_position(cursor_excerpt, cursor_offset),
        edit_history,
        expected_patches: Vec::new(),
        rejected_patch: None,
        captured_prompt_input: Some(CapturedPromptInput {
            cursor_file_content: cursor_excerpt.to_string(),
            cursor_offset,
            cursor_row,
            cursor_column,
            events,
            related_files,
        }),
        telemetry: Some(TelemetrySource {
            request_id,
            device_id,
            time,
            rejection_reason,
            was_shown,
        }),
        human_feedback: Vec::new(),
        rating: None,
    };

    Example {
        spec,
        prompt_inputs: None,
        prompt: None,
        predictions: Vec::new(),
        score: Vec::new(),
        qa: Vec::new(),
        state: None,
    }
}

fn compute_row_column(text: &str, offset: usize) -> (u32, u32) {
    let mut row = 0u32;
    let mut last_newline_offset = 0;
    for (i, c) in text.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            row += 1;
            last_newline_offset = i + 1;
        }
    }
    let column = (offset - last_newline_offset) as u32;
    (row, column)
}

fn build_cursor_position(excerpt: &str, cursor_offset: usize) -> String {
    let before = &excerpt[..cursor_offset.min(excerpt.len())];
    let after = &excerpt[cursor_offset.min(excerpt.len())..];
    format!("{}[CURSOR_POSITION]{}", before, after)
}

fn build_output_patch(
    cursor_path: &std::path::Path,
    cursor_excerpt: &str,
    editable_range: &std::ops::Range<usize>,
    model_output: &str,
) -> String {
    let old_text = &cursor_excerpt[editable_range.clone()];

    let editable_start_row = cursor_excerpt[..editable_range.start]
        .chars()
        .filter(|&c| c == '\n')
        .count() as u32;

    let diff_body = language::unified_diff_with_offsets(
        old_text,
        model_output,
        editable_start_row,
        editable_start_row,
    );

    let mut patch = String::new();
    writeln!(&mut patch, "--- a/{}", cursor_path.display()).ok();
    writeln!(&mut patch, "+++ b/{}", cursor_path.display()).ok();
    patch.push_str(&diff_body);
    patch
}

fn get_column_indices(
    meta: &Option<SnowflakeResultSetMetaData>,
    names: &[&str],
) -> std::collections::HashMap<String, usize> {
    let mut indices = std::collections::HashMap::new();
    if let Some(meta) = meta {
        for (index, col) in meta.row_type.iter().enumerate() {
            for &name in names {
                if col.name.eq_ignore_ascii_case(name) {
                    indices.insert(name.to_string(), index);
                }
            }
        }
    }
    indices
}
