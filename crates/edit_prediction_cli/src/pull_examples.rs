use anyhow::{Context as _, Result};
use flate2::read::GzDecoder;
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request};
use indoc::indoc;
use serde::Deserialize;
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use telemetry_events::EditPredictionRating;

use zeta_prompt::{ZetaFormat, ZetaPromptInput, excerpt_range_for_format};

use crate::PredictionProvider;
use crate::example::{Example, ExamplePrompt};
use crate::progress::{InfoStyle, Progress, Step};
use edit_prediction::example_spec::{ExampleSpec, TelemetrySource};

pub(crate) const SNOWFLAKE_SUCCESS_CODE: &str = "090001";
pub(crate) const SNOWFLAKE_ASYNC_IN_PROGRESS_CODE: &str = "333334";
const SNOWFLAKE_TIMEOUT_CODE: &str = "000630";

/// Minimum Zed version for filtering captured examples.
/// For example, `MinCaptureVersion { minor: 224, patch: 1 }` means only pull examples
/// where `zed_version >= 0.224.1`.
#[derive(Clone, Copy, Debug)]
pub struct MinCaptureVersion {
    pub minor: u32,
    pub patch: u32,
}

pub(crate) const POLL_INTERVAL: Duration = Duration::from_secs(2);
const PARTITION_FETCH_MAX_RETRIES: usize = 3;
const PARTITION_FETCH_RETRY_DELAYS: [Duration; PARTITION_FETCH_MAX_RETRIES] = [
    Duration::from_millis(500),
    Duration::from_secs(1),
    Duration::from_secs(2),
];

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

/// Parse an input token of the form `settled-after:{timestamp}`.
pub fn parse_settled_after_input(input: &str) -> Option<&str> {
    input.strip_prefix("settled-after:")
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SnowflakeStatementResponse {
    #[serde(default)]
    pub(crate) data: Vec<Vec<JsonValue>>,
    #[serde(default)]
    pub(crate) result_set_meta_data: Option<SnowflakeResultSetMetaData>,
    #[serde(default)]
    pub(crate) code: Option<String>,
    #[serde(default)]
    pub(crate) message: Option<String>,
    #[serde(default)]
    pub(crate) statement_handle: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SnowflakeResultSetMetaData {
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

        for attempt in 0.. {
            step_progress.set_substatus(format!("polling ({attempt})"));

            background_executor.timer(POLL_INTERVAL).await;

            response = fetch_partition_with_retries(
                http_client.clone(),
                base_url,
                token,
                &statement_handle,
                0,
                background_executor.clone(),
            )
            .await?;

            if response.code.as_deref() != Some(SNOWFLAKE_ASYNC_IN_PROGRESS_CODE) {
                break;
            }
        }
    }

    Ok(response)
}

struct SnowflakeConfig {
    token: String,
    base_url: String,
    role: Option<String>,
}

#[derive(Clone)]
struct QueryRetryState {
    resume_after: String,
    remaining_limit: Option<usize>,
    offset: usize,
}

async fn fetch_examples_with_query<MakeBindings>(
    http_client: Arc<dyn HttpClient>,
    step_progress: &crate::progress::StepProgress,
    background_executor: BackgroundExecutor,
    statement: &str,
    initial_retry_state: QueryRetryState,
    make_bindings: MakeBindings,
    required_columns: &[&str],
    parse_response: for<'a> fn(
        &'a SnowflakeStatementResponse,
        &'a HashMap<String, usize>,
    ) -> Result<Box<dyn Iterator<Item = Example> + 'a>>,
) -> Result<Vec<Example>>
where
    MakeBindings: Fn(&QueryRetryState) -> JsonValue,
{
    let snowflake = SnowflakeConfig {
        token: std::env::var("EP_SNOWFLAKE_API_KEY")
            .context("missing required environment variable EP_SNOWFLAKE_API_KEY")?,
        base_url: std::env::var("EP_SNOWFLAKE_BASE_URL").context(
            "missing required environment variable EP_SNOWFLAKE_BASE_URL (e.g. https://<account>.snowflakecomputing.com)",
        )?,
        role: std::env::var("EP_SNOWFLAKE_ROLE").ok(),
    };

    let mut requested_columns = required_columns.to_vec();
    if !requested_columns.contains(&"continuation_time") {
        requested_columns.push("continuation_time");
    }

    let mut parsed_examples = Vec::new();
    let mut retry_state = initial_retry_state;
    let mut retry_count = 0usize;

    loop {
        let bindings = make_bindings(&retry_state);
        let request = json!({
            "statement": statement,
            "database": "EVENTS",
            "schema": "PUBLIC",
            "warehouse": "DBT",
            "role": snowflake.role.as_deref(),
            "bindings": bindings
        });

        let response = match run_sql_with_polling(
            http_client.clone(),
            &snowflake.base_url,
            &snowflake.token,
            &request,
            step_progress,
            background_executor.clone(),
        )
        .await
        {
            Ok(response) => response,
            Err(error) => {
                if is_snowflake_timeout_error(&error) && !parsed_examples.is_empty() {
                    retry_count += 1;
                    step_progress.set_substatus(format!(
                        "retrying from {} ({retry_count})",
                        retry_state.resume_after
                    ));
                    continue;
                }

                return Err(error);
            }
        };

        let total_rows = response
            .result_set_meta_data
            .as_ref()
            .and_then(|meta| meta.num_rows)
            .unwrap_or(response.data.len() as i64);
        let partition_count = response
            .result_set_meta_data
            .as_ref()
            .map(|meta| meta.partition_info.len())
            .unwrap_or(1)
            .max(1);

        step_progress.set_info(format!("{} rows", total_rows), InfoStyle::Normal);
        step_progress.set_substatus("parsing");

        let column_indices = get_column_indices(&response.result_set_meta_data, &requested_columns);
        let mut rows_fetched_this_attempt = 0usize;
        let mut timed_out_fetching_partition = false;

        parsed_examples.extend(parse_response(&response, &column_indices)?);
        rows_fetched_this_attempt += response.data.len();
        let mut last_continuation_time_this_attempt =
            last_continuation_timestamp_from_response(&response, &column_indices);

        if partition_count > 1 {
            let statement_handle = response
                .statement_handle
                .as_ref()
                .context("response has multiple partitions but no statementHandle")?;

            for partition in 1..partition_count {
                step_progress.set_substatus(format!(
                    "fetching partition {}/{}",
                    partition + 1,
                    partition_count
                ));

                let partition_response = match fetch_partition_with_retries(
                    http_client.clone(),
                    &snowflake.base_url,
                    &snowflake.token,
                    statement_handle,
                    partition,
                    background_executor.clone(),
                )
                .await
                {
                    Ok(response) => response,
                    Err(error) => {
                        if is_snowflake_timeout_error(&error) && rows_fetched_this_attempt > 0 {
                            timed_out_fetching_partition = true;
                            break;
                        }

                        return Err(error);
                    }
                };

                parsed_examples.extend(parse_response(&partition_response, &column_indices)?);
                rows_fetched_this_attempt += partition_response.data.len();

                if let Some(partition_continuation_time) =
                    last_continuation_timestamp_from_response(&partition_response, &column_indices)
                {
                    last_continuation_time_this_attempt = Some(partition_continuation_time);
                }
            }
        }

        if rows_fetched_this_attempt == 0 {
            step_progress.set_substatus("done");
            return Ok(parsed_examples);
        }

        if let Some(remaining_limit_value) = &mut retry_state.remaining_limit {
            *remaining_limit_value =
                remaining_limit_value.saturating_sub(rows_fetched_this_attempt);
            if *remaining_limit_value == 0 {
                step_progress.set_substatus("done");
                return Ok(parsed_examples);
            }
        }

        if !timed_out_fetching_partition {
            step_progress.set_substatus("done");
            return Ok(parsed_examples);
        }

        let Some(last_continuation_time_this_attempt) = last_continuation_time_this_attempt else {
            step_progress.set_substatus("done");
            return Ok(parsed_examples);
        };

        retry_state.resume_after = last_continuation_time_this_attempt;
        retry_state.offset = 0;
        retry_count += 1;
        step_progress.set_substatus(format!(
            "retrying from {} ({retry_count})",
            retry_state.resume_after
        ));
    }
}

pub(crate) async fn fetch_partition(
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

async fn fetch_partition_with_retries(
    http_client: Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    statement_handle: &str,
    partition: usize,
    background_executor: BackgroundExecutor,
) -> Result<SnowflakeStatementResponse> {
    let mut last_error = None;

    for retry_attempt in 0..=PARTITION_FETCH_MAX_RETRIES {
        match fetch_partition(
            http_client.clone(),
            base_url,
            token,
            statement_handle,
            partition,
        )
        .await
        {
            Ok(response) => return Ok(response),
            Err(error) => {
                if retry_attempt == PARTITION_FETCH_MAX_RETRIES
                    || !is_transient_partition_fetch_error(&error)
                {
                    return Err(error);
                }

                last_error = Some(error);
                background_executor
                    .timer(PARTITION_FETCH_RETRY_DELAYS[retry_attempt])
                    .await;
            }
        }
    }

    match last_error {
        Some(error) => Err(error),
        None => anyhow::bail!("partition fetch retry loop exited without a result"),
    }
}

fn is_transient_partition_fetch_error(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        let message = cause.to_string();
        message.contains("failed to read Snowflake SQL API partition response body")
            || message.contains("unexpected EOF")
            || message.contains("peer closed connection without sending TLS close_notify")
    })
}

pub(crate) async fn run_sql(
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

    let snowflake_response = serde_json::from_slice::<SnowflakeStatementResponse>(&body_bytes)
        .context("failed to parse Snowflake SQL API response JSON")?;

    if !status.is_success() && status.as_u16() != 202 && !is_timeout_response(&snowflake_response) {
        let body_text = String::from_utf8_lossy(&body_bytes);
        anyhow::bail!("snowflake sql api http {}: {}", status.as_u16(), body_text);
    }

    if is_timeout_response(&snowflake_response) {
        anyhow::bail!(
            "snowflake sql api timed out code={} message={}",
            snowflake_response.code.as_deref().unwrap_or("<no code>"),
            snowflake_response
                .message
                .as_deref()
                .unwrap_or("<no message>")
        );
    }

    Ok(snowflake_response)
}

pub async fn fetch_rejected_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: Option<usize>,
    offset: usize,
    background_executor: BackgroundExecutor,
    min_capture_version: Option<MinCaptureVersion>,
) -> Result<Vec<Example>> {
    if after_timestamps.is_empty() {
        return Ok(Vec::new());
    }

    let progress = Progress::global();

    let mut all_examples = Vec::new();

    for after_date in after_timestamps.iter() {
        let step_progress_name = format!("rejected>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let min_minor_str = min_capture_version.map(|version| version.minor.to_string());
        let min_patch_str = min_capture_version.map(|version| version.patch.to_string());
        let min_minor_str_ref = min_minor_str.as_deref();
        let min_patch_str_ref = min_patch_str.as_deref();

        let statement = indoc! {r#"
            SELECT
                ep_request_id AS request_id,
                device_id AS device_id,
                requested_at::string AS continuation_time,
                requested_at::string AS time,
                input_payload AS input,
                prompt AS prompt,
                requested_output AS output,
                is_ep_shown_before_rejected AS was_shown,
                ep_rejected_reason AS reason,
                zed_version AS zed_version
            FROM ZED_DBT.DBT_PROD.fct_edit_prediction_examples
            WHERE ep_outcome LIKE 'Rejected%'
                AND is_ep_shown_before_rejected = true
                AND requested_at > TRY_TO_TIMESTAMP_NTZ(?)
                AND (? IS NULL OR (
                    TRY_CAST(SPLIT_PART(zed_version, '.', 2) AS INTEGER) > ?
                    OR (
                        TRY_CAST(SPLIT_PART(zed_version, '.', 2) AS INTEGER) = ?
                        AND TRY_CAST(SPLIT_PART(SPLIT_PART(zed_version, '.', 3), '+', 1) AS INTEGER) >= ?
                    )
                ))
            ORDER BY requested_at ASC
            LIMIT ?
            OFFSET ?
        "#};

        let examples = fetch_examples_with_query(
            http_client.clone(),
            &step_progress,
            background_executor.clone(),
            statement,
            QueryRetryState {
                resume_after: after_date.clone(),
                remaining_limit: max_rows_per_timestamp,
                offset,
            },
            |retry_state| {
                json!({
                    "1": { "type": "TEXT", "value": retry_state.resume_after },
                    "2": { "type": "FIXED", "value": min_minor_str_ref },
                    "3": { "type": "FIXED", "value": min_minor_str_ref },
                    "4": { "type": "FIXED", "value": min_minor_str_ref },
                    "5": { "type": "FIXED", "value": min_patch_str_ref },
                    "6": { "type": "FIXED", "value": format_limit(retry_state.remaining_limit) },
                    "7": { "type": "FIXED", "value": retry_state.offset.to_string() }
                })
            },
            &[
                "request_id",
                "device_id",
                "time",
                "input",
                "prompt",
                "output",
                "was_shown",
                "reason",
                "zed_version",
            ],
            rejected_examples_from_response,
        )
        .await?;

        all_examples.extend(examples);
    }

    Ok(all_examples)
}

fn format_limit(limit: Option<usize>) -> String {
    return limit.map(|l| l.to_string()).unwrap_or("NULL".to_string());
}

pub async fn fetch_requested_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: Option<usize>,
    offset: usize,
    background_executor: BackgroundExecutor,
    min_capture_version: Option<MinCaptureVersion>,
) -> Result<Vec<Example>> {
    if after_timestamps.is_empty() {
        return Ok(Vec::new());
    }

    let progress = Progress::global();

    let mut all_examples = Vec::new();

    for after_date in after_timestamps.iter() {
        let step_progress_name = format!("requested>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let min_minor_str = min_capture_version.map(|version| version.minor.to_string());
        let min_patch_str = min_capture_version.map(|version| version.patch.to_string());
        let min_minor_str_ref = min_minor_str.as_deref();
        let min_patch_str_ref = min_patch_str.as_deref();

        let statement = indoc! {r#"
            SELECT
                ep_request_id AS request_id,
                device_id AS device_id,
                requested_at::string AS continuation_time,
                requested_at::string AS time,
                input_payload AS input,
                zed_version AS zed_version
            FROM ZED_DBT.DBT_PROD.fct_edit_prediction_examples
            WHERE requested_at > TRY_TO_TIMESTAMP_NTZ(?)
                AND (? IS NULL OR (
                    TRY_CAST(SPLIT_PART(zed_version, '.', 2) AS INTEGER) > ?
                    OR (
                        TRY_CAST(SPLIT_PART(zed_version, '.', 2) AS INTEGER) = ?
                        AND TRY_CAST(SPLIT_PART(SPLIT_PART(zed_version, '.', 3), '+', 1) AS INTEGER) >= ?
                    )
                ))
            ORDER BY requested_at ASC
            LIMIT ?
            OFFSET ?
        "#};

        let examples = fetch_examples_with_query(
            http_client.clone(),
            &step_progress,
            background_executor.clone(),
            statement,
            QueryRetryState {
                resume_after: after_date.clone(),
                remaining_limit: max_rows_per_timestamp,
                offset,
            },
            |retry_state| {
                json!({
                    "1": { "type": "TEXT", "value": retry_state.resume_after },
                    "2": { "type": "FIXED", "value": min_minor_str_ref },
                    "3": { "type": "FIXED", "value": min_minor_str_ref },
                    "4": { "type": "FIXED", "value": min_minor_str_ref },
                    "5": { "type": "FIXED", "value": min_patch_str_ref },
                    "6": { "type": "FIXED", "value": format_limit(retry_state.remaining_limit) },
                    "7": { "type": "FIXED", "value": retry_state.offset.to_string() }
                })
            },
            &["request_id", "device_id", "time", "input", "zed_version"],
            requested_examples_from_response,
        )
        .await?;

        all_examples.extend(examples);
    }

    Ok(all_examples)
}

pub async fn fetch_captured_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: Option<usize>,
    offset: usize,
    background_executor: BackgroundExecutor,
    min_capture_version: Option<MinCaptureVersion>,
) -> Result<Vec<Example>> {
    if after_timestamps.is_empty() {
        return Ok(Vec::new());
    }

    let progress = Progress::global();

    let mut all_examples = Vec::new();

    for after_date in after_timestamps.iter() {
        let step_progress_name = format!("captured>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let min_minor_str = min_capture_version.map(|version| version.minor.to_string());
        let min_patch_str = min_capture_version.map(|version| version.patch.to_string());
        let min_minor_str_ref = min_minor_str.as_deref();
        let min_patch_str_ref = min_patch_str.as_deref();

        let statement = indoc! {r#"
            SELECT
                ep_request_id AS request_id,
                device_id AS device_id,
                requested_at::string AS continuation_time,
                requested_at::string AS time,
                input_payload AS input,
                settled_editable_region AS settled_editable_region,
                example_payload AS example,
                zed_version AS zed_version
            FROM ZED_DBT.DBT_PROD.fct_edit_prediction_examples
            WHERE settled_editable_region IS NOT NULL
                AND example_payload IS NOT NULL
                AND requested_at > TRY_TO_TIMESTAMP_NTZ(?)
                AND (? IS NULL OR (
                    TRY_CAST(SPLIT_PART(zed_version, '.', 2) AS INTEGER) > ?
                    OR (
                        TRY_CAST(SPLIT_PART(zed_version, '.', 2) AS INTEGER) = ?
                        AND TRY_CAST(SPLIT_PART(SPLIT_PART(zed_version, '.', 3), '+', 1) AS INTEGER) >= ?
                    )
                ))
            ORDER BY requested_at ASC
            LIMIT ?
            OFFSET ?
        "#};

        let examples = fetch_examples_with_query(
            http_client.clone(),
            &step_progress,
            background_executor.clone(),
            statement,
            QueryRetryState {
                resume_after: after_date.clone(),
                remaining_limit: max_rows_per_timestamp,
                offset,
            },
            |retry_state| {
                json!({
                    "1": { "type": "TEXT", "value": retry_state.resume_after },
                    "2": { "type": "FIXED", "value": min_minor_str_ref },
                    "3": { "type": "FIXED", "value": min_minor_str_ref },
                    "4": { "type": "FIXED", "value": min_minor_str_ref },
                    "5": { "type": "FIXED", "value": min_patch_str_ref },
                    "6": { "type": "FIXED", "value": format_limit(retry_state.remaining_limit) },
                    "7": { "type": "FIXED", "value": retry_state.offset.to_string() }
                })
            },
            &[
                "request_id",
                "device_id",
                "time",
                "input",
                "settled_editable_region",
                "example",
                "zed_version",
            ],
            captured_examples_from_response,
        )
        .await?;

        all_examples.extend(examples);
    }

    Ok(all_examples)
}

pub async fn fetch_settled_examples_after(
    http_client: Arc<dyn HttpClient>,
    after_timestamps: &[String],
    max_rows_per_timestamp: Option<usize>,
    offset: usize,
    background_executor: BackgroundExecutor,
    min_capture_version: Option<MinCaptureVersion>,
) -> Result<Vec<Example>> {
    if after_timestamps.is_empty() {
        return Ok(Vec::new());
    }

    let progress = Progress::global();

    let mut all_examples = Vec::new();

    for after_date in after_timestamps.iter() {
        let step_progress_name = format!("settled>{after_date}");
        let step_progress = progress.start(Step::PullExamples, &step_progress_name);
        step_progress.set_substatus("querying");

        let _ = min_capture_version;

        let statement = indoc! {r#"
            SELECT
                ep_request_id AS request_id,
                device_id AS device_id,
                requested_at::string AS continuation_time,
                requested_at::string AS time,
                input_payload AS input,
                requested_output AS requested_output,
                settled_editable_region AS settled_editable_region,
                requested_format AS requested_format,
                zed_version AS zed_version
            FROM ZED_DBT.DBT_PROD.fct_edit_prediction_examples
            WHERE settled_editable_region IS NOT NULL
                AND requested_at > TRY_TO_TIMESTAMP_NTZ(?)
            ORDER BY requested_at ASC
            LIMIT ?
            OFFSET ?
        "#};

        let examples = fetch_examples_with_query(
            http_client.clone(),
            &step_progress,
            background_executor.clone(),
            statement,
            QueryRetryState {
                resume_after: after_date.clone(),
                remaining_limit: max_rows_per_timestamp,
                offset,
            },
            |retry_state| {
                json!({
                    "1": { "type": "TEXT", "value": retry_state.resume_after },
                    "2": { "type": "FIXED", "value": format_limit(retry_state.remaining_limit) },
                    "3": { "type": "FIXED", "value": retry_state.offset.to_string() }
                })
            },
            &[
                "request_id",
                "device_id",
                "time",
                "input",
                "requested_output",
                "settled_editable_region",
                "requested_format",
                "zed_version",
            ],
            settled_examples_from_response,
        )
        .await?;

        all_examples.extend(examples);
    }

    Ok(all_examples)
}

pub async fn fetch_rated_examples_after(
    http_client: Arc<dyn HttpClient>,
    inputs: &[(String, Option<EditPredictionRating>)],
    max_rows_per_timestamp: Option<usize>,
    offset: usize,
    background_executor: BackgroundExecutor,
    _min_capture_version: Option<MinCaptureVersion>,
) -> Result<Vec<Example>> {
    if inputs.is_empty() {
        return Ok(Vec::new());
    }

    let progress = Progress::global();

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

        let rating_value = rating_filter.as_ref().map(|rating| match rating {
            EditPredictionRating::Positive => "Positive",
            EditPredictionRating::Negative => "Negative",
        });

        let statement = indoc! {r#"
            SELECT
                ep_request_id AS request_id,
                rated_inputs AS inputs,
                rated_output AS output,
                rating AS rating,
                feedback AS feedback,
                device_id AS device_id,
                requested_at::string AS continuation_time,
                requested_at::string AS time,
                NULL AS experiment_name,
                NULL AS environment,
                zed_version AS zed_version
            FROM ZED_DBT.DBT_PROD.fct_edit_prediction_examples
            WHERE rating IS NOT NULL
                AND (? IS NULL OR rating = ?)
                AND requested_at > TRY_TO_TIMESTAMP_NTZ(?)
                AND rated_inputs IS NOT NULL
                AND rated_inputs:cursor_excerpt IS NOT NULL
                AND rated_output IS NOT NULL
            ORDER BY requested_at ASC
            LIMIT ?
            OFFSET ?
        "#};

        let examples = fetch_examples_with_query(
            http_client.clone(),
            &step_progress,
            background_executor.clone(),
            statement,
            QueryRetryState {
                resume_after: after_date.clone(),
                remaining_limit: max_rows_per_timestamp,
                offset,
            },
            |retry_state| {
                json!({
                    "1": { "type": "TEXT", "value": rating_value },
                    "2": { "type": "TEXT", "value": rating_value },
                    "3": { "type": "TEXT", "value": retry_state.resume_after },
                    "4": { "type": "FIXED", "value": format_limit(retry_state.remaining_limit) },
                    "5": { "type": "FIXED", "value": retry_state.offset.to_string() }
                })
            },
            &[
                "request_id",
                "inputs",
                "output",
                "rating",
                "feedback",
                "device_id",
                "time",
                "experiment_name",
                "environment",
                "zed_version",
            ],
            rated_examples_from_response,
        )
        .await?;

        all_examples.extend(examples);
    }

    Ok(all_examples)
}

fn rated_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<Box<dyn Iterator<Item = Example> + 'a>> {
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

            let request_id = get_string("request_id");
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
            let experiment_name = get_string("experiment_name");
            let environment = get_string("environment");
            let zed_version = get_string("zed_version");

            match (inputs, output.clone(), rating.clone(), time.clone()) {
                (Some(inputs), Some(output), Some(rating), Some(time)) => {
                    Some(build_rated_example(
                        request_id,
                        device_id.unwrap_or_default(),
                        time,
                        inputs,
                        output,
                        rating,
                        feedback,
                        experiment_name,
                        environment,
                        zed_version,
                    ))
                }
                _ => {
                    log::warn!(
                        "skipping row {row_index}: missing fields - inputs={:?} output={:?} rating={:?} time={:?}",
                        inputs_json.is_some(),
                        output.is_some(),
                        rating.is_some(),
                        time.is_some(),
                    );
                    None
                }
            }
        });

    Ok(Box::new(iter))
}

fn build_rated_example(
    request_id: Option<String>,
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    output: String,
    rating: String,
    feedback: String,
    experiment_name: Option<String>,
    environment: Option<String>,
    zed_version: Option<String>,
) -> Example {
    let parsed_rating = if rating == "Positive" {
        EditPredictionRating::Positive
    } else {
        EditPredictionRating::Negative
    };
    let is_positive = parsed_rating == EditPredictionRating::Positive;
    let request_id = request_id.unwrap_or_else(|| format!("rated-{}-{}", device_id, time));

    let mut tags = Vec::with_capacity(3);
    tags.push(if is_positive {
        "rated:positive".to_string()
    } else {
        "rated:negative".to_string()
    });
    if let Some(experiment) = experiment_name {
        tags.push(format!("experiment:{experiment}"));
    }
    if let Some(env) = environment {
        tags.push(format!("environment:{env}"));
    }

    let mut example =
        build_example_from_snowflake(request_id, device_id, time, input, tags, None, zed_version);

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
) -> Result<Box<dyn Iterator<Item = Example> + 'a>> {
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
            let zed_version = get_string("zed_version");

            match (request_id_str.clone(), device_id.clone(), time.clone(), input) {
                (Some(request_id), Some(device_id), Some(time), Some(input)) => {
                    Some(build_example_from_snowflake(
                        request_id,
                        device_id,
                        time,
                        input,
                        vec!["requested".to_string()],
                        None,
                        zed_version,
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

    Ok(Box::new(iter))
}

fn settled_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<Box<dyn Iterator<Item = Example> + 'a>> {
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
            let get_value = |name: &str| -> Option<JsonValue> {
                let index = column_indices.get(name).copied()?;
                let value = data_row.get(index)?;
                if value.is_null() {
                    None
                } else {
                    Some(value.clone())
                }
            };

            let get_string = |name: &str| -> Option<String> {
                match get_value(name)? {
                    JsonValue::String(s) => Some(s),
                    other => Some(other.to_string()),
                }
            };

            let parse_json_value = |raw: Option<&JsonValue>| -> Option<JsonValue> {
                let value = raw?;
                match value {
                    JsonValue::String(s) => serde_json::from_str::<JsonValue>(s).ok(),
                    other => Some(other.clone()),
                }
            };

            let request_id_str = get_string("request_id");
            let device_id = get_string("device_id");
            let time = get_string("time");
            let input_raw = get_value("input");
            let input_json = parse_json_value(input_raw.as_ref());
            let input: Option<ZetaPromptInput> = input_json
                .as_ref()
                .and_then(|parsed| serde_json::from_value(parsed.clone()).ok());
            let requested_output = get_string("requested_output");
            let settled_editable_region = get_string("settled_editable_region");
            let requested_format =
                get_string("requested_format").and_then(|s| ZetaFormat::parse(&s).ok());
            let zed_version = get_string("zed_version");

            match (
                request_id_str.clone(),
                device_id.clone(),
                time.clone(),
                input.clone(),
                requested_output.clone(),
                settled_editable_region.clone(),
                requested_format,
            ) {
                (
                    Some(request_id),
                    Some(device_id),
                    Some(time),
                    Some(input),
                    Some(requested_output),
                    Some(settled_editable_region),
                    Some(requested_format),
                ) => Some(build_settled_example(
                    request_id,
                    device_id,
                    time,
                    input,
                    requested_output,
                    settled_editable_region,
                    requested_format,
                    zed_version,
                )),
                _ => {
                    let mut missing_fields = Vec::new();

                    if request_id_str.is_none() {
                        missing_fields.push("request_id");
                    }
                    if device_id.is_none() {
                        missing_fields.push("device_id");
                    }
                    if time.is_none() {
                        missing_fields.push("time");
                    }
                    if input_raw.is_none() || input_json.is_none() || input.is_none() {
                        missing_fields.push("input");
                    }
                    if requested_output.is_none() {
                        missing_fields.push("requested_output");
                    }
                    if settled_editable_region.is_none() {
                        missing_fields.push("settled_editable_region");
                    }
                    if requested_format.is_none() {
                        missing_fields.push("requested_format");
                    }

                    log::warn!(
                        "skipping settled row {row_index}: [{}]",
                        missing_fields.join(", "),
                    );
                    None
                }
            }
        });

    Ok(Box::new(iter))
}

fn captured_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<Box<dyn Iterator<Item = Example> + 'a>> {
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
            let get_value = |name: &str| -> Option<JsonValue> {
                let index = column_indices.get(name).copied()?;
                let value = data_row.get(index)?;
                if value.is_null() {
                    None
                } else {
                    Some(value.clone())
                }
            };

            let get_string = |name: &str| -> Option<String> {
                match get_value(name)? {
                    JsonValue::String(s) => Some(s),
                    other => Some(other.to_string()),
                }
            };

            let parse_json_value = |raw: Option<&JsonValue>| -> Option<JsonValue> {
                let value = raw?;
                match value {
                    JsonValue::String(s) => serde_json::from_str::<JsonValue>(s).ok(),
                    other => Some(other.clone()),
                }
            };

            let request_id = get_string("request_id");
            let device_id = get_string("device_id");
            let time = get_string("time");
            let input_raw = get_value("input");
            let input_json = parse_json_value(input_raw.as_ref());
            let input: Option<ZetaPromptInput> = input_json
                .as_ref()
                .and_then(|parsed| serde_json::from_value(parsed.clone()).ok());
            let example_raw = get_value("example");
            let example_json = parse_json_value(example_raw.as_ref());
            let example_spec: Option<ExampleSpec> = example_json.as_ref().and_then(|parsed| {
                serde_json::from_value(parsed.clone())
                    .or_else(|_| {
                        parsed
                            .as_str()
                            .and_then(|markdown| ExampleSpec::from_markdown(markdown).ok())
                            .ok_or_else(|| {
                                serde_json::Error::io(std::io::Error::other("not markdown"))
                            })
                    })
                    .ok()
            });
            let has_example_spec = example_spec.is_some();
            let settled_editable_region = get_string("settled_editable_region");
            let zed_version = get_string("zed_version");

            match (
                request_id.clone(),
                device_id.clone(),
                time.clone(),
                input.clone(),
                example_spec,
                settled_editable_region.clone(),
            ) {
                (
                    Some(request_id),
                    Some(device_id),
                    Some(time),
                    Some(input),
                    Some(example_spec),
                    Some(settled_editable_region),
                ) => Some(build_captured_example(
                    request_id,
                    device_id,
                    time,
                    input,
                    example_spec,
                    settled_editable_region,
                    zed_version,
                )),
                _ => {
                    let mut missing_fields = Vec::new();

                    if request_id.is_none() {
                        missing_fields.push("request_id");
                    }
                    if device_id.is_none() {
                        missing_fields.push("device_id");
                    }
                    if time.is_none() {
                        missing_fields.push("time");
                    }
                    if input_raw.is_none() || input_json.is_none() || input.is_none() {
                        missing_fields.push("input");
                    }
                    if example_raw.is_none() || !has_example_spec {
                        missing_fields.push("example");
                    }
                    if settled_editable_region.is_none() {
                        missing_fields.push("settled_editable_region");
                    }

                    log::warn!(
                        "skipping captured row {row_index}: [{}]",
                        missing_fields.join(", "),
                    );
                    None
                }
            }
        });

    Ok(Box::new(iter))
}

fn build_settled_example(
    request_id: String,
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    requested_output: String,
    settled_editable_region: String,
    requested_format: ZetaFormat,
    zed_version: Option<String>,
) -> Example {
    let requested_editable_range =
        excerpt_range_for_format(requested_format, &input.excerpt_ranges).0;

    let base_cursor_excerpt = input.cursor_excerpt.to_string();

    let requested_range_is_valid = requested_editable_range.start <= requested_editable_range.end
        && requested_editable_range.end <= base_cursor_excerpt.len();
    let mut example = build_example_from_snowflake(
        request_id.clone(),
        device_id,
        time,
        input,
        vec!["settled".to_string()],
        None,
        zed_version,
    );

    if !requested_range_is_valid {
        log::warn!(
            "skipping malformed requested range for request {}: requested={:?} (base_len={})",
            request_id,
            requested_editable_range,
            base_cursor_excerpt.len(),
        );
        return example;
    }

    let settled_replacement = settled_editable_region.as_str();
    let rejected_patch = build_output_patch(
        &example.spec.cursor_path,
        &base_cursor_excerpt,
        &requested_editable_range,
        &requested_output,
    );
    let expected_patch = build_output_patch(
        &example.spec.cursor_path,
        &base_cursor_excerpt,
        &requested_editable_range,
        settled_replacement,
    );

    example.spec.expected_patches = vec![expected_patch];
    example.spec.rejected_patch = Some(rejected_patch);
    example
}

fn build_captured_example(
    request_id: String,
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    mut example_spec: ExampleSpec,
    settled_editable_region: String,
    zed_version: Option<String>,
) -> Example {
    let expected_patch = build_output_patch(
        &input.cursor_path,
        input.cursor_excerpt.as_ref(),
        &input.excerpt_ranges.editable_350,
        settled_editable_region.as_str(),
    );

    example_spec.expected_patches = vec![expected_patch];
    example_spec.telemetry = Some(TelemetrySource {
        request_id,
        device_id,
        time,
        rejection_reason: String::new(),
        was_shown: false,
    });

    Example {
        spec: example_spec,
        zed_version,
        prompt_inputs: Some(input),
        prompt: None,
        predictions: Vec::new(),
        score: Vec::new(),
        qa: Vec::new(),
        state: None,
    }
}

fn rejected_examples_from_response<'a>(
    response: &'a SnowflakeStatementResponse,
    column_indices: &'a std::collections::HashMap<String, usize>,
) -> Result<Box<dyn Iterator<Item = Example> + 'a>> {
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
            let prompt = get_string("prompt");
            let output = get_string("output");
            let was_shown = get_bool("was_shown");
            let reason = get_string("reason");
            let zed_version = get_string("zed_version");

            match (request_id_str.clone(), device_id.clone(), time.clone(), input, output.clone(), was_shown, reason.clone()) {
                (Some(request_id), Some(device_id), Some(time), Some(input), Some(output), Some(was_shown), Some(reason)) => {
                    Some(build_rejected_example(
                        request_id,
                        device_id,
                        time,
                        input,
                        prompt,
                        output,
                        was_shown,
                        reason,
                        zed_version,
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

    Ok(Box::new(iter))
}

fn build_rejected_example(
    request_id: String,
    device_id: String,
    time: String,
    input: ZetaPromptInput,
    prompt: Option<String>,
    output: String,
    was_shown: bool,
    reason: String,
    zed_version: Option<String>,
) -> Example {
    let rejected_patch = build_output_patch(
        &input.cursor_path,
        input.cursor_excerpt.as_ref(),
        &input.excerpt_ranges.editable_350,
        &output,
    );
    let mut example = build_example_from_snowflake(
        request_id,
        device_id,
        time,
        input,
        vec![format!("rejection:{}", reason.to_lowercase())],
        Some(RejectionInfo { reason, was_shown }),
        zed_version,
    );
    example.spec.rejected_patch = Some(rejected_patch);
    example.prompt = prompt.map(|prompt| ExamplePrompt {
        input: prompt,
        expected_output: String::new(),
        rejected_output: Some(output),
        prefill: None,
        provider: PredictionProvider::default(),
    });
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
    zed_version: Option<String>,
) -> Example {
    let cursor_excerpt = input.cursor_excerpt.as_ref();
    let cursor_offset = input.cursor_offset_in_excerpt;

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
        zed_version,
        prompt_inputs: Some(input),
        prompt: None,
        predictions: Vec::new(),
        score: Vec::new(),
        qa: Vec::new(),
        state: None,
    }
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

fn is_timeout_response(response: &SnowflakeStatementResponse) -> bool {
    response.code.as_deref() == Some(SNOWFLAKE_TIMEOUT_CODE)
        && response
            .message
            .as_deref()
            .map(|message| message.to_ascii_lowercase().contains("timeout"))
            .unwrap_or(false)
}

fn is_snowflake_timeout_error(error: &anyhow::Error) -> bool {
    error
        .chain()
        .any(|cause| cause.to_string().contains(SNOWFLAKE_TIMEOUT_CODE))
}

fn last_continuation_timestamp_from_response(
    response: &SnowflakeStatementResponse,
    column_indices: &HashMap<String, usize>,
) -> Option<String> {
    let continuation_time_index = column_indices.get("continuation_time").copied()?;
    response
        .data
        .iter()
        .rev()
        .find_map(|row| match row.get(continuation_time_index)? {
            JsonValue::String(value) => Some(value.clone()),
            JsonValue::Null => None,
            other => Some(other.to_string()),
        })
}

pub(crate) fn get_column_indices(
    meta: &Option<SnowflakeResultSetMetaData>,
    names: &[&str],
) -> HashMap<String, usize> {
    let mut indices = HashMap::new();
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
