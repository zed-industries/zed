//! Token usage reconstructed from the agents' local session logs, priced at
//! API rates. Ported from Superset's usage history.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{BufRead as _, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::{Context as _, Result};
use chrono::{DateTime, Days, Local, NaiveDate, TimeZone as _, Utc};
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, Method, Request};
use serde_json::Value;

use crate::AccountProvider;

const MAX_LINE_BYTES: usize = 32 * 1024 * 1024;
const MAX_GROK_SESSION_DIRS: usize = 4096;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UsageEntry {
    pub provider: Option<AccountProvider>,
    pub model: String,
    pub timestamp: DateTime<Utc>,
    pub uncached_input: u64,
    pub cached_input: u64,
    pub cache_write_5m: u64,
    pub cache_write_1h: u64,
    pub output: u64,
    /// The provider's own charge, when it reports one.
    pub cost_usd: Option<f64>,
}

impl UsageEntry {
    pub fn tokens(&self) -> u64 {
        self.uncached_input
            + self.cached_input
            + self.cache_write_5m
            + self.cache_write_1h
            + self.output
    }
}

/// The first local midnight of a `days`-day range ending today.
pub fn range_start(days: u32, now: DateTime<Local>) -> DateTime<Utc> {
    let first_day = now.date_naive() - Days::new(u64::from(days.saturating_sub(1)));
    local_midnight(first_day).with_timezone(&Utc)
}

fn local_midnight(day: NaiveDate) -> DateTime<Local> {
    let midnight = day.and_hms_opt(0, 0, 0).unwrap_or_default();
    Local
        .from_local_datetime(&midnight)
        .earliest()
        .unwrap_or_else(|| Local.from_utc_datetime(&midnight))
}

/// Reads every usage record newer than `cutoff` from the given homes'
/// logs. Blocking: call it off the main thread.
pub fn collect_local_entries(
    homes: &[(AccountProvider, PathBuf)],
    cutoff: DateTime<Utc>,
    now: DateTime<Utc>,
) -> Vec<UsageEntry> {
    let mut entries = Vec::new();
    let mut claude_by_message: HashMap<String, UsageEntry> = HashMap::new();
    let mut seen_roots = HashSet::new();
    let file_cutoff = SystemTime::from(cutoff - chrono::Duration::days(1));

    for (provider, home) in homes {
        let root = match provider {
            AccountProvider::Claude => home.join("projects"),
            AccountProvider::Codex => home.join("sessions"),
            AccountProvider::Grok => home.join("logs"),
            AccountProvider::Cursor => continue,
        };
        // Profiles may share history by linking it back to the main home.
        let Ok(root) = fs::canonicalize(&root) else {
            continue;
        };
        if !seen_roots.insert((*provider, root.clone())) {
            continue;
        }
        match provider {
            AccountProvider::Claude => {
                for file in jsonl_files(&root, file_cutoff) {
                    parse_claude_file(&file, cutoff, now, &mut claude_by_message, &mut entries);
                }
            }
            AccountProvider::Codex => {
                for file in jsonl_files(&root, file_cutoff) {
                    parse_codex_file(&file, cutoff, now, &mut entries);
                }
            }
            AccountProvider::Grok => {
                parse_grok_home(home, &root.join("unified.jsonl"), cutoff, now, &mut entries);
            }
            AccountProvider::Cursor => {}
        }
    }
    entries.extend(claude_by_message.into_values());
    entries
}

fn jsonl_files(root: &Path, modified_after: SystemTime) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let path = entry.path();
            if file_type.is_dir() {
                stack.push(path);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "jsonl")
                && entry
                    .metadata()
                    .and_then(|metadata| metadata.modified())
                    .is_ok_and(|modified| modified >= modified_after)
            {
                files.push(path);
            }
        }
    }
    files
}

/// Calls `visit` with each line containing `needle`, plus the file's mtime.
fn for_each_line(path: &Path, needles: &[&str], mut visit: impl FnMut(&str, DateTime<Utc>)) {
    let Ok(file) = fs::File::open(path) else {
        return;
    };
    let modified = file
        .metadata()
        .and_then(|metadata| metadata.modified())
        .map(DateTime::<Utc>::from)
        .unwrap_or_else(|_| Utc::now());
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        if line.len() > MAX_LINE_BYTES || !needles.iter().any(|needle| line.contains(needle)) {
            continue;
        }
        visit(line.trim_end_matches(['\n', '\r']), modified);
    }
}

fn count(value: Option<&Value>) -> u64 {
    value
        .and_then(Value::as_f64)
        .filter(|number| number.is_finite() && *number > 0.0)
        .map_or(0, |number| number as u64)
}

fn timestamp(value: Option<&Value>, fallback: DateTime<Utc>, now: DateTime<Utc>) -> DateTime<Utc> {
    let parsed = match value {
        Some(Value::String(text)) => DateTime::parse_from_rfc3339(text)
            .ok()
            .map(|date| date.with_timezone(&Utc)),
        Some(Value::Number(number)) => number.as_f64().and_then(|number| {
            let millis = if number < 1e12 {
                number * 1000.0
            } else {
                number
            };
            Utc.timestamp_millis_opt(millis as i64).single()
        }),
        _ => None,
    };
    parsed
        .filter(|date| *date <= now + chrono::Duration::hours(26))
        .unwrap_or(fallback)
}

fn parse_claude_file(
    path: &Path,
    cutoff: DateTime<Utc>,
    now: DateTime<Utc>,
    by_message: &mut HashMap<String, UsageEntry>,
    entries: &mut Vec<UsageEntry>,
) {
    for_each_line(path, &["\"assistant\""], |line, modified| {
        let Ok(record) = serde_json::from_str::<Value>(line) else {
            return;
        };
        if record.get("type").and_then(Value::as_str) != Some("assistant") {
            return;
        }
        let Some(message) = record.get("message") else {
            return;
        };
        let (Some(usage), Some(model)) = (
            message.get("usage"),
            message.get("model").and_then(Value::as_str),
        ) else {
            return;
        };
        if model == "<synthetic>" {
            return;
        }
        let mut cache_write_5m = count(usage.pointer("/cache_creation/ephemeral_5m_input_tokens"));
        let cache_write_1h = count(usage.pointer("/cache_creation/ephemeral_1h_input_tokens"));
        if cache_write_5m == 0 && cache_write_1h == 0 {
            cache_write_5m = count(usage.get("cache_creation_input_tokens"));
        }
        let entry = UsageEntry {
            provider: Some(AccountProvider::Claude),
            model: model.to_string(),
            timestamp: timestamp(record.get("timestamp"), modified, now),
            uncached_input: count(usage.get("input_tokens")),
            cached_input: count(usage.get("cache_read_input_tokens")),
            cache_write_5m,
            cache_write_1h,
            output: count(usage.get("output_tokens")),
            cost_usd: None,
        };
        if entry.timestamp < cutoff {
            return;
        }
        let message_id = message
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let request_id = record
            .get("requestId")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if message_id.is_empty() && request_id.is_empty() {
            entries.push(entry);
        } else {
            // Streaming rewrites the same message with growing usage.
            by_message.insert(format!("{message_id}|{request_id}"), entry);
        }
    });
}

fn parse_codex_file(
    path: &Path,
    cutoff: DateTime<Utc>,
    now: DateTime<Utc>,
    entries: &mut Vec<UsageEntry>,
) {
    let mut model = String::from("unknown");
    let mut previous_usage: Option<String> = None;
    for_each_line(
        path,
        &["token_count", "turn_context", "session_meta"],
        |line, modified| {
            let Ok(record) = serde_json::from_str::<Value>(line) else {
                return;
            };
            let Some(payload) = record.get("payload") else {
                return;
            };
            if matches!(
                record.get("type").and_then(Value::as_str),
                Some("turn_context" | "session_meta")
            ) {
                if let Some(name) = payload.get("model").and_then(Value::as_str) {
                    model = name.to_string();
                }
                return;
            }
            if payload.get("type").and_then(Value::as_str) != Some("token_count") {
                return;
            }
            let Some(usage) = payload.pointer("/info/last_token_usage") else {
                return;
            };
            let serialized = usage.to_string();
            if previous_usage.as_ref() == Some(&serialized) {
                return;
            }
            previous_usage = Some(serialized);
            let cached = count(usage.get("cached_input_tokens"));
            let entry = UsageEntry {
                provider: Some(AccountProvider::Codex),
                model: model.clone(),
                timestamp: timestamp(record.get("timestamp"), modified, now),
                uncached_input: count(usage.get("input_tokens")).saturating_sub(cached),
                cached_input: cached,
                cache_write_5m: count(usage.get("cache_write_input_tokens")),
                cache_write_1h: 0,
                output: count(usage.get("output_tokens")),
                cost_usd: None,
            };
            if entry.timestamp >= cutoff {
                entries.push(entry);
            }
        },
    );
}

fn parse_grok_home(
    home: &Path,
    log: &Path,
    cutoff: DateTime<Utc>,
    now: DateTime<Utc>,
    entries: &mut Vec<UsageEntry>,
) {
    let mut grok_entries: Vec<(String, UsageEntry)> = Vec::new();
    for_each_line(log, &["shell.turn.inference_done"], |line, modified| {
        let Ok(record) = serde_json::from_str::<Value>(line) else {
            return;
        };
        if record.get("msg").and_then(Value::as_str) != Some("shell.turn.inference_done") {
            return;
        }
        let Some(session_id) = record
            .get("sid")
            .and_then(Value::as_str)
            .filter(|sid| !sid.is_empty())
        else {
            return;
        };
        let context = record.get("ctx").unwrap_or(&Value::Null);
        let cached = count(context.get("cached_prompt_tokens"));
        let entry = UsageEntry {
            provider: Some(AccountProvider::Grok),
            model: String::from("unknown"),
            timestamp: timestamp(record.get("ts"), modified, now),
            uncached_input: count(context.get("prompt_tokens")).saturating_sub(cached),
            cached_input: cached,
            output: count(context.get("completion_tokens")),
            ..UsageEntry::default()
        };
        if entry.timestamp >= cutoff {
            grok_entries.push((session_id.to_string(), entry));
        }
    });
    if grok_entries.is_empty() {
        return;
    }
    let wanted: HashSet<&str> = grok_entries.iter().map(|(sid, _)| sid.as_str()).collect();
    let models = grok_session_models(&home.join("sessions"), &wanted);
    for (session_id, mut entry) in grok_entries {
        if let Some(model) = models.get(&session_id) {
            entry.model = model.clone();
        }
        entries.push(entry);
    }
}

fn grok_session_models(sessions: &Path, wanted: &HashSet<&str>) -> HashMap<String, String> {
    let mut models = HashMap::new();
    let mut visited = 0;
    let Ok(projects) = fs::read_dir(sessions) else {
        return models;
    };
    'projects: for project in projects.flatten() {
        let Ok(session_dirs) = fs::read_dir(project.path()) else {
            continue;
        };
        for session in session_dirs.flatten() {
            visited += 1;
            if visited > MAX_GROK_SESSION_DIRS {
                break 'projects;
            }
            let name = session.file_name().to_string_lossy().into_owned();
            if !wanted.contains(name.as_str()) {
                continue;
            }
            let model = fs::read(session.path().join("summary.json"))
                .ok()
                .and_then(|contents| serde_json::from_slice::<Value>(&contents).ok())
                .and_then(|summary| {
                    summary
                        .get("current_model_id")
                        .and_then(Value::as_str)
                        .map(str::to_string)
                });
            if let Some(model) = model {
                models.insert(name, model);
            }
        }
    }
    models
}

// ----------------------------------------------------------------- cursor

const CURSOR_USAGE_URL: &str =
    "https://api2.cursor.sh/aiserver.v1.DashboardService/GetFilteredUsageEvents";
const CURSOR_PAGE_SIZE: usize = 300;
const CURSOR_MAX_PAGES: u32 = 40;

/// Cursor keeps no token counts on disk; its dashboard API lists usage
/// events. A profile home stands in for `HOME`, so its login is the file
/// store below it; the main login is in the Keychain.
pub async fn read_cursor_token(home: &Path, is_default: bool) -> Option<String> {
    if is_default && cfg!(target_os = "macos") {
        let output = util::command::new_command("/usr/bin/security")
            .args([
                "find-generic-password",
                "-s",
                "cursor-access-token",
                "-a",
                "cursor-user",
                "-w",
            ])
            .output()
            .await
            .ok()
            .filter(|output| output.status.success());
        if let Some(output) = output {
            let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !token.is_empty() {
                return Some(token);
            }
        }
    }
    let contents = fs::read(home.join(".cursor/auth.json")).ok()?;
    let auth: Value = serde_json::from_slice(&contents).ok()?;
    auth.get("accessToken")
        .or_else(|| auth.get("access_token"))
        .and_then(Value::as_str)
        .filter(|token| !token.is_empty())
        .map(str::to_string)
}

pub async fn fetch_cursor_entries(
    token: &str,
    http: Arc<dyn HttpClient>,
    cutoff: DateTime<Utc>,
    now: DateTime<Utc>,
) -> Result<Vec<UsageEntry>> {
    let mut entries = Vec::new();
    let mut seen_events = 0;
    for page in 1..=CURSOR_MAX_PAGES {
        let body = serde_json::json!({
            "teamId": 0,
            "startDate": cutoff.timestamp_millis().to_string(),
            "endDate": now.timestamp_millis().to_string(),
            "page": page,
            "pageSize": CURSOR_PAGE_SIZE,
        });
        let request = Request::builder()
            .method(Method::POST)
            .uri(CURSOR_USAGE_URL)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body.to_string()))?;
        let mut response = http.send(request).await?;
        let status = response.status();
        let mut text = String::new();
        response.body_mut().read_to_string(&mut text).await?;
        anyhow::ensure!(
            status.is_success(),
            "Cursor usage request failed (HTTP {status})"
        );
        let page_body: Value = serde_json::from_str(&text).context("parsing Cursor usage")?;
        let events = page_body
            .get("usageEventsDisplay")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        seen_events += events.len();
        entries.extend(
            events
                .iter()
                .filter_map(|event| parse_cursor_event(event, cutoff)),
        );
        let total = count(page_body.get("totalUsageEventsCount")) as usize;
        if events.len() < CURSOR_PAGE_SIZE || (total > 0 && seen_events >= total) {
            break;
        }
    }
    Ok(entries)
}

fn parse_cursor_event(event: &Value, cutoff: DateTime<Utc>) -> Option<UsageEntry> {
    let millis: i64 = match event.get("timestamp")? {
        Value::String(text) => text.parse().ok()?,
        Value::Number(number) => number.as_f64()? as i64,
        _ => return None,
    };
    let timestamp = Utc.timestamp_millis_opt(millis).single()?;
    if timestamp < cutoff {
        return None;
    }
    let usage = event.get("tokenUsage")?;
    let cents = usage
        .get("totalCents")
        .or_else(|| event.get("chargedCents"))
        .and_then(Value::as_f64)
        .filter(|cents| cents.is_finite() && *cents > 0.0);
    Some(UsageEntry {
        provider: Some(AccountProvider::Cursor),
        model: event
            .get("model")
            .and_then(Value::as_str)
            .filter(|model| !model.is_empty())
            .unwrap_or("unknown")
            .to_string(),
        timestamp,
        uncached_input: count(usage.get("inputTokens")),
        cached_input: count(usage.get("cacheReadTokens")),
        cache_write_5m: count(usage.get("cacheWriteTokens")),
        cache_write_1h: 0,
        output: count(usage.get("outputTokens")),
        cost_usd: cents.map(|cents| cents / 100.0),
    })
}

// ------------------------------------------------------------------ prices

/// USD per million tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Price {
    pub input: f64,
    pub output: f64,
    pub cache_read: Option<f64>,
}

struct PriceRow {
    provider: AccountProvider,
    prefix: &'static str,
    price: Price,
    long_context: Option<(u64, Price)>,
}

const fn price(input: f64, output: f64) -> Price {
    Price {
        input,
        output,
        cache_read: None,
    }
}

const fn price_with_cache(input: f64, output: f64, cache_read: f64) -> Price {
    Price {
        input,
        output,
        cache_read: Some(cache_read),
    }
}

const fn row(provider: AccountProvider, prefix: &'static str, price: Price) -> PriceRow {
    PriceRow {
        provider,
        prefix,
        price,
        long_context: None,
    }
}

/// Superset's table as of 2026-09-30.
const PRICES: &[PriceRow] = {
    use AccountProvider::*;
    &[
        row(Claude, "claude-fable-5-1", price_with_cache(10., 50., 0.25)),
        row(
            Claude,
            "claude-mythos-5-1",
            price_with_cache(10., 50., 0.25),
        ),
        row(Claude, "claude-fable-5", price(10., 50.)),
        row(Claude, "claude-mythos", price(10., 50.)),
        row(Claude, "claude-opus-5-5", price_with_cache(4., 20., 0.2)),
        row(Claude, "claude-opus-5", price(5., 25.)),
        row(Claude, "claude-opus-4-8", price(5., 25.)),
        row(Claude, "claude-opus-4-7", price(5., 25.)),
        row(Claude, "claude-opus-4-6", price(5., 25.)),
        row(Claude, "claude-opus-4-5", price(5., 25.)),
        row(Claude, "claude-opus-4", price(15., 75.)),
        row(Claude, "claude-sonnet-5", price(2., 10.)),
        row(Claude, "claude-sonnet-4", price(3., 15.)),
        row(Claude, "claude-haiku-4-5", price(1., 5.)),
        row(Claude, "claude-3-5-haiku", price(0.8, 4.)),
        PriceRow {
            provider: Codex,
            prefix: "gpt-6.1-sol",
            price: price_with_cache(2., 10., 0.1),
            long_context: Some((272_000, price_with_cache(4., 15., 0.2))),
        },
        row(Codex, "gpt-6-astra", price(10., 50.)),
        row(Codex, "gpt-6-sol", price(2., 10.)),
        row(Codex, "gpt-6-luna", price(0.1, 0.5)),
        row(Codex, "gpt-5.6-sol", price(4., 20.)),
        row(Codex, "gpt-5.6-terra", price(2., 12.)),
        row(Codex, "gpt-5.6-luna", price(0.2, 1.2)),
        row(Codex, "gpt-5.6", price(4., 20.)),
        row(Codex, "gpt-5.3-codex", price(1.75, 14.)),
        row(Codex, "gpt-5.3", price(1.75, 14.)),
        row(Codex, "gpt-5-codex", price(1.25, 10.)),
        row(Codex, "gpt-5", price(1.25, 10.)),
        row(Codex, "gpt-4.1", price(2., 8.)),
        row(Codex, "gpt-4o", price(2.5, 10.)),
        row(Grok, "grok-4.6", price(2., 6.)),
        row(Grok, "grok-4.5", price(2., 6.)),
        row(Grok, "grok-4-fast", price(0.2, 0.5)),
        row(Grok, "grok-4", price(3., 15.)),
        row(Grok, "grok-code", price(0.2, 1.5)),
        row(Grok, "grok-3-mini", price(0.3, 0.5)),
        row(Grok, "grok-3", price(3., 15.)),
        row(Cursor, "composer", price(1.25, 10.)),
    ]
};

/// The price of a model, and whether it is a guess because the model is
/// unknown (then the provider's cheapest rate is used).
pub fn price_for(provider: AccountProvider, model: &str, context_tokens: u64) -> (Price, bool) {
    let model = model.to_lowercase();
    let short = model.rsplit('/').next().unwrap_or(&model).to_string();
    let found = PRICES
        .iter()
        .filter(|row| row.provider == provider)
        .filter(|row| short.starts_with(row.prefix) || model.starts_with(row.prefix))
        .max_by_key(|row| row.prefix.len());
    if let Some(found) = found {
        let price = match found.long_context {
            Some((threshold, long)) if context_tokens > threshold => long,
            _ => found.price,
        };
        return (price, false);
    }
    let cheapest = PRICES
        .iter()
        .filter(|row| row.provider == provider)
        .min_by(|a, b| {
            (a.price.input + a.price.output).total_cmp(&(b.price.input + b.price.output))
        })
        .map_or(price(0., 0.), |row| row.price);
    (cheapest, true)
}

/// The API-rate cost of an entry, what caching saved, and whether the cost
/// is a guess. A cost the provider reported wins over the estimate.
pub fn entry_cost(entry: &UsageEntry) -> (f64, f64, bool) {
    if let Some(cost) = entry.cost_usd {
        return (cost, 0., false);
    }
    let provider = entry.provider.unwrap_or(AccountProvider::Claude);
    let (price, approximate) = price_for(
        provider,
        &entry.model,
        entry.uncached_input + entry.cached_input,
    );
    let cache_read = price.cache_read.unwrap_or(price.input * 0.1);
    let per_million = |tokens: u64, rate: f64| tokens as f64 / 1e6 * rate;
    let cost = per_million(entry.uncached_input, price.input)
        + per_million(entry.output, price.output)
        + per_million(entry.cached_input, cache_read)
        + per_million(entry.cache_write_5m, price.input * 1.25)
        + per_million(entry.cache_write_1h, price.input * 2.);
    let savings = per_million(entry.cached_input, price.input - cache_read);
    (cost, savings, approximate)
}

// ------------------------------------------------------------- aggregation

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Amount {
    pub usd: f64,
    pub tokens: u64,
}

impl Amount {
    fn add(&mut self, usd: f64, tokens: u64) {
        self.usd += usd;
        self.tokens += tokens;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayUsage {
    pub date: NaiveDate,
    pub total: Amount,
    pub by_provider: BTreeMap<AccountProvider, Amount>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelUsage {
    pub provider: AccountProvider,
    pub model: String,
    pub amount: Amount,
    pub approximate: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UsageSummary {
    pub first_day: Option<NaiveDate>,
    pub days: Vec<DayUsage>,
    pub by_provider: BTreeMap<AccountProvider, Amount>,
    pub models: Vec<ModelUsage>,
    pub total: Amount,
    pub uncached_input: u64,
    pub cached_input: u64,
    pub cache_write: u64,
    pub output: u64,
    pub cache_savings_usd: f64,
    pub approximate: bool,
}

/// Buckets entries into `days` local days ending on `today`.
pub fn summarize(entries: &[UsageEntry], days: u32, today: NaiveDate) -> UsageSummary {
    let first_day = today - Days::new(u64::from(days.saturating_sub(1)));
    let mut summary = UsageSummary {
        first_day: Some(first_day),
        days: first_day
            .iter_days()
            .take(days as usize)
            .map(|date| DayUsage {
                date,
                total: Amount::default(),
                by_provider: BTreeMap::new(),
            })
            .collect(),
        ..UsageSummary::default()
    };
    let mut models: HashMap<(AccountProvider, String), ModelUsage> = HashMap::new();
    for entry in entries {
        let Some(provider) = entry.provider else {
            continue;
        };
        let day = entry.timestamp.with_timezone(&Local).date_naive();
        let Some(index) = usize::try_from((day - first_day).num_days())
            .ok()
            .filter(|index| *index < summary.days.len())
        else {
            continue;
        };
        let (usd, savings, approximate) = entry_cost(entry);
        let tokens = entry.tokens();
        let bucket = &mut summary.days[index];
        bucket.total.add(usd, tokens);
        bucket
            .by_provider
            .entry(provider)
            .or_default()
            .add(usd, tokens);
        summary
            .by_provider
            .entry(provider)
            .or_default()
            .add(usd, tokens);
        summary.total.add(usd, tokens);
        summary.uncached_input += entry.uncached_input;
        summary.cached_input += entry.cached_input;
        summary.cache_write += entry.cache_write_5m + entry.cache_write_1h;
        summary.output += entry.output;
        if entry.cost_usd.is_none() {
            summary.cache_savings_usd += savings;
            summary.approximate |= approximate;
        }
        let model = models
            .entry((provider, entry.model.clone()))
            .or_insert_with(|| ModelUsage {
                provider,
                model: entry.model.clone(),
                amount: Amount::default(),
                approximate: false,
            });
        model.amount.add(usd, tokens);
        model.approximate |= approximate && entry.cost_usd.is_none();
    }
    summary.models = models.into_values().collect();
    summary
        .models
        .sort_by(|a, b| b.amount.usd.total_cmp(&a.amount.usd));
    summary
}

/// "$1,802" at or above $100, "$0.75" below.
pub fn format_usd(usd: f64) -> String {
    if usd >= 100. {
        let whole = usd.round() as u64;
        let digits = whole.to_string();
        let mut grouped = String::new();
        for (index, digit) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index).is_multiple_of(3) {
                grouped.push(',');
            }
            grouped.push(digit);
        }
        format!("${grouped}")
    } else {
        format!("${usd:.2}")
    }
}

/// "3.6B", "12.4M", "830K", "512".
pub fn format_tokens(tokens: u64) -> String {
    let value = tokens as f64;
    if value >= 1e12 {
        format!("{:.2}T", value / 1e12)
    } else if value >= 1e9 {
        format!("{:.1}B", value / 1e9)
    } else if value >= 1e6 {
        format!("{:.1}M", value / 1e6)
    } else if value >= 1e3 {
        format!("{:.0}K", value / 1e3)
    } else {
        tokens.to_string()
    }
}

/// How long until a quota window resets: "1d 22h", "1h 43m", "now".
pub fn format_reset_in(resets_at: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let millis = (resets_at - now).num_milliseconds();
    if millis <= 0 {
        return "now".into();
    }
    let minutes = (millis as f64 / 60_000.).ceil() as i64;
    let (days, hours, minutes) = (minutes / 1440, minutes % 1440 / 60, minutes % 60);
    match (days, hours, minutes) {
        (0, 0, minutes) => format!("{minutes}m"),
        (0, hours, 0) => format!("{hours}h"),
        (0, hours, minutes) => format!("{hours}h {minutes}m"),
        (days, 0, _) => format!("{days}d"),
        (days, hours, _) => format!("{days}d {hours}h"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn reads_claude_codex_and_grok_logs() {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path();
        let now = Utc::now();
        let ts = (now - chrono::Duration::hours(1)).to_rfc3339();
        // The same message streamed twice: only the last snapshot counts.
        write(
            &home.join(".claude/projects/p/s.jsonl"),
            &format!(
                "{{\"type\":\"assistant\",\"timestamp\":\"{ts}\",\"requestId\":\"r1\",\"message\":{{\"id\":\"m1\",\"model\":\"claude-opus-5-5\",\"usage\":{{\"input_tokens\":10,\"output_tokens\":1}}}}}}\n\
                 {{\"type\":\"assistant\",\"timestamp\":\"{ts}\",\"requestId\":\"r1\",\"message\":{{\"id\":\"m1\",\"model\":\"claude-opus-5-5\",\"usage\":{{\"input_tokens\":10,\"cache_read_input_tokens\":1000,\"cache_creation_input_tokens\":100,\"output_tokens\":50}}}}}}\n\
                 {{\"type\":\"assistant\",\"timestamp\":\"{ts}\",\"message\":{{\"model\":\"<synthetic>\",\"usage\":{{\"input_tokens\":5}}}}}}\n\
                 {{\"type\":\"user\",\"message\":\"assistant\"}}\n"
            ),
        );
        write(
            &home.join(".codex/sessions/2026/10/04/rollout.jsonl"),
            &format!(
                "{{\"type\":\"turn_context\",\"timestamp\":\"{ts}\",\"payload\":{{\"model\":\"gpt-6.1-sol\"}}}}\n\
                 {{\"type\":\"event_msg\",\"timestamp\":\"{ts}\",\"payload\":{{\"type\":\"token_count\",\"info\":{{\"last_token_usage\":{{\"input_tokens\":300,\"cached_input_tokens\":200,\"output_tokens\":20}}}}}}}}\n\
                 {{\"type\":\"event_msg\",\"timestamp\":\"{ts}\",\"payload\":{{\"type\":\"token_count\",\"info\":{{\"last_token_usage\":{{\"input_tokens\":300,\"cached_input_tokens\":200,\"output_tokens\":20}}}}}}}}\n"
            ),
        );
        write(
            &home.join(".grok/logs/unified.jsonl"),
            &format!(
                "{{\"msg\":\"shell.turn.inference_done\",\"sid\":\"g1\",\"ts\":\"{ts}\",\"ctx\":{{\"prompt_tokens\":40,\"cached_prompt_tokens\":30,\"completion_tokens\":7}}}}\n"
            ),
        );
        write(
            &home.join(".grok/sessions/%2Fp/g1/summary.json"),
            r#"{"current_model_id":"grok-4.6"}"#,
        );

        let homes = [
            (AccountProvider::Claude, home.join(".claude")),
            (AccountProvider::Codex, home.join(".codex")),
            (AccountProvider::Grok, home.join(".grok")),
        ];
        let mut entries = collect_local_entries(&homes, now - chrono::Duration::days(1), now);
        entries.sort_by_key(|entry| entry.provider.map(|provider| provider as u8));
        let summary: Vec<_> = entries
            .iter()
            .map(|entry| {
                (
                    entry.provider,
                    entry.model.as_str(),
                    entry.uncached_input,
                    entry.cached_input,
                    entry.cache_write_5m,
                    entry.output,
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (
                    Some(AccountProvider::Claude),
                    "claude-opus-5-5",
                    10,
                    1000,
                    100,
                    50
                ),
                (Some(AccountProvider::Codex), "gpt-6.1-sol", 100, 200, 0, 20),
                (Some(AccountProvider::Grok), "grok-4.6", 10, 30, 0, 7),
            ]
        );
    }

    #[test]
    fn prices_entries() {
        let entry = UsageEntry {
            provider: Some(AccountProvider::Claude),
            model: "claude-opus-5-5-20260901".into(),
            uncached_input: 1_000_000,
            cached_input: 1_000_000,
            cache_write_5m: 1_000_000,
            output: 1_000_000,
            ..UsageEntry::default()
        };
        let (cost, savings, approximate) = entry_cost(&entry);
        // 4 + 20 + 0.2 + 5
        assert!((cost - 29.2).abs() < 1e-9, "{cost}");
        assert!((savings - 3.8).abs() < 1e-9, "{savings}");
        assert!(!approximate);

        let (fallback, approximate) = price_for(AccountProvider::Codex, "mystery", 0);
        assert!(approximate);
        assert_eq!(fallback, price(0.1, 0.5));
        let (long, _) = price_for(AccountProvider::Codex, "openai/gpt-6.1-sol", 300_000);
        assert_eq!(long, price_with_cache(4., 15., 0.2));
        let reported = UsageEntry {
            cost_usd: Some(1.5),
            ..entry
        };
        assert_eq!(entry_cost(&reported), (1.5, 0., false));
    }

    #[test]
    fn parses_cursor_events() {
        let cutoff = Utc.timestamp_millis_opt(1_787_000_000_000).unwrap();
        let event: Value = serde_json::from_str(
            r#"{"timestamp":"1787004840166","model":"composer-2","conversationId":"c",
                "tokenUsage":{"inputTokens":100,"outputTokens":20,"cacheReadTokens":300,
                "cacheWriteTokens":5,"totalCents":0.4547}}"#,
        )
        .unwrap();
        let entry = parse_cursor_event(&event, cutoff).unwrap();
        assert_eq!(
            (
                entry.model.as_str(),
                entry.uncached_input,
                entry.cached_input,
                entry.cache_write_5m,
                entry.output
            ),
            ("composer-2", 100, 300, 5, 20)
        );
        assert!((entry.cost_usd.unwrap() - 0.004547).abs() < 1e-12);
        let without_tokens: Value =
            serde_json::from_str(r#"{"timestamp":"1787004840166","chargedCents":4}"#).unwrap();
        assert!(parse_cursor_event(&without_tokens, cutoff).is_none());
    }

    #[test]
    fn summarizes_into_local_days() {
        let today = Local::now().date_naive();
        let at = |days_ago: u64| {
            local_midnight(today - Days::new(days_ago)).with_timezone(&Utc)
                + chrono::Duration::hours(12)
        };
        let entry = |provider, days_ago, output| UsageEntry {
            provider: Some(provider),
            model: "grok-4.6".into(),
            timestamp: at(days_ago),
            output,
            ..UsageEntry::default()
        };
        let summary = summarize(
            &[
                entry(AccountProvider::Grok, 0, 1_000_000),
                entry(AccountProvider::Grok, 2, 1_000_000),
                entry(AccountProvider::Codex, 6, 10),
                entry(AccountProvider::Grok, 7, 1_000_000),
            ],
            7,
            today,
        );
        assert_eq!(summary.days.len(), 7);
        assert_eq!(summary.days[6].total.tokens, 1_000_000);
        assert_eq!(summary.days[4].total.tokens, 1_000_000);
        assert_eq!(
            summary.days[0].by_provider[&AccountProvider::Codex].tokens,
            10
        );
        assert_eq!(summary.by_provider[&AccountProvider::Grok].usd, 12.);
        assert_eq!(summary.total.tokens, 2_000_010);
        assert!(summary.approximate);
    }

    #[test]
    fn formats_amounts() {
        assert_eq!(format_usd(1802.4), "$1,802");
        assert_eq!(format_usd(0.75), "$0.75");
        assert_eq!(format_tokens(3_600_000_000), "3.6B");
        assert_eq!(format_tokens(830_400), "830K");
        let now = Utc::now();
        assert_eq!(
            format_reset_in(now + chrono::Duration::minutes(103), now),
            "1h 43m"
        );
        assert_eq!(
            format_reset_in(now + chrono::Duration::hours(46), now),
            "1d 22h"
        );
        assert_eq!(
            format_reset_in(now - chrono::Duration::minutes(1), now),
            "now"
        );
    }
}
