//! Reads how much of each account's subscription quota is used.
//!
//! Ported from Superset's usage readers. Tokens are only ever read, never
//! refreshed: refreshing a token behind the CLI's back can trip the
//! provider's token-reuse protection and sign the CLI out. A lapsed token is
//! reported, and the CLI refreshes it the next time it runs.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, TimeZone as _, Utc};
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, Method, Request};
use serde::Deserialize;
use sha2::{Digest as _, Sha256};

use crate::{AccountProvider, AgentAccount};

const CLAUDE_USAGE_URL: &str = "https://api.anthropic.com/api/oauth/usage";
const CLAUDE_OAUTH_BETA: &str = "oauth-2025-04-20";
const CLAUDE_KEYCHAIN_SERVICE: &str = "Claude Code-credentials";
const CODEX_USAGE_URL: &str = "https://chatgpt.com/backend-api/wham/usage";
const GROK_USAGE_URL: &str = "https://grok.com/grok_api_v2.GrokBuildBilling/GetGrokCreditsConfig";

#[derive(Debug, Clone, PartialEq)]
pub struct QuotaWindow {
    pub id: String,
    pub label: String,
    pub used_percent: u32,
    pub resets_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuotaStatus {
    Ok,
    /// The access token lapsed but the CLI can still refresh it.
    TokenStale,
    /// The CLI needs a new login.
    TokenExpired,
    /// A profile with an identity but no readable login.
    SignedOut,
    Unavailable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountQuota {
    pub status: QuotaStatus,
    pub windows: Vec<QuotaWindow>,
    pub plan: Option<String>,
}

impl AccountQuota {
    fn with_status(status: QuotaStatus) -> Self {
        Self {
            status,
            windows: Vec::new(),
            plan: None,
        }
    }

    /// The most used window, if any.
    pub fn max_used_percent(&self) -> Option<u32> {
        self.windows.iter().map(|window| window.used_percent).max()
    }

    /// Whether any window reached the threshold.
    pub fn is_exhausted(&self, threshold_percent: f32) -> bool {
        self.max_used_percent()
            .is_some_and(|used| used as f32 >= threshold_percent)
    }

    /// A short summary such as "42% · Weekly 80%".
    pub fn summary(&self) -> Option<String> {
        match &self.status {
            QuotaStatus::Ok if !self.windows.is_empty() => Some(
                self.windows
                    .iter()
                    .map(|window| format!("{} {}%", window.label, window.used_percent))
                    .collect::<Vec<_>>()
                    .join(" · "),
            ),
            QuotaStatus::Ok | QuotaStatus::Unavailable(_) => None,
            QuotaStatus::TokenStale => Some("login refreshes on next run".into()),
            QuotaStatus::TokenExpired => Some("login expired".into()),
            QuotaStatus::SignedOut => Some("signed out".into()),
        }
    }
}

/// Fetches the quota of an account. Blocking file and Keychain reads happen
/// inline, so call it off the main thread.
pub async fn fetch_quota(account: &AgentAccount, http: Arc<dyn HttpClient>) -> AccountQuota {
    let result = match account.provider {
        AccountProvider::Claude => fetch_claude(account, http).await,
        AccountProvider::Codex => fetch_codex(&account.home, http).await,
        AccountProvider::Grok => fetch_grok(&account.home, http).await,
        AccountProvider::Cursor => {
            return AccountQuota::with_status(QuotaStatus::Unavailable(
                "Cursor does not report quota".into(),
            ));
        }
    };
    result.unwrap_or_else(|error| {
        AccountQuota::with_status(QuotaStatus::Unavailable(format!("{error:#}")))
    })
}

// ---------------------------------------------------------------- Claude

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeOAuth {
    access_token: Option<String>,
    expires_at: Option<i64>,
    refresh_token: Option<String>,
    refresh_token_expires_at: Option<i64>,
    subscription_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeCredentialFile {
    claude_ai_oauth: Option<ClaudeOAuth>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TokenFreshness {
    Expired,
    Stale,
    Live,
}

impl ClaudeOAuth {
    fn freshness(&self, now_ms: i64) -> TokenFreshness {
        if self.expires_at.is_none_or(|at| at > now_ms) {
            return TokenFreshness::Live;
        }
        let refresh_valid = self
            .refresh_token
            .as_deref()
            .is_some_and(|token| !token.is_empty())
            && self.refresh_token_expires_at.is_none_or(|at| at > now_ms);
        if refresh_valid {
            TokenFreshness::Stale
        } else {
            TokenFreshness::Expired
        }
    }
}

fn parse_claude_credential(json: &str) -> Option<ClaudeOAuth> {
    let oauth = serde_json::from_str::<ClaudeCredentialFile>(json)
        .ok()?
        .claude_ai_oauth?;
    oauth.access_token.as_ref()?;
    Some(oauth)
}

/// `/login` leaves stale copies behind, so the freshest credential wins.
fn pick_freshest(credentials: Vec<ClaudeOAuth>, now_ms: i64) -> Option<ClaudeOAuth> {
    credentials.into_iter().max_by_key(|credential| {
        (
            credential.freshness(now_ms),
            credential.expires_at.unwrap_or(i64::MAX),
        )
    })
}

/// Every Keychain service name Claude Code may have used for a config dir:
/// it hashes the literal `CLAUDE_CONFIG_DIR` value.
fn claude_keychain_services(config_dir: &Path, home_dir: &Path) -> Vec<String> {
    let absolute = config_dir.to_string_lossy().into_owned();
    let mut spellings = vec![absolute];
    if let Ok(relative) = config_dir.strip_prefix(home_dir) {
        spellings.push(format!("~/{}", relative.display()));
        spellings.push(format!("$HOME/{}", relative.display()));
    }
    for spelling in spellings.clone() {
        match spelling.strip_suffix('/') {
            Some(trimmed) => spellings.push(trimmed.to_string()),
            None => spellings.push(format!("{spelling}/")),
        }
    }
    spellings
        .into_iter()
        .map(|spelling| {
            let digest = Sha256::digest(spelling.as_bytes());
            let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
            format!("{CLAUDE_KEYCHAIN_SERVICE}-{}", &hex[..8])
        })
        .collect()
}

async fn read_keychain_secrets(service: &str) -> Vec<String> {
    if !cfg!(target_os = "macos") {
        return Vec::new();
    }
    let mut scopes: Vec<Vec<String>> = Vec::new();
    if let Ok(user) = std::env::var("USER") {
        scopes.push(vec!["-a".into(), user]);
    }
    scopes.push(Vec::new());
    let mut secrets = Vec::new();
    for scope in scopes {
        let output = util::command::new_command("/usr/bin/security")
            .arg("find-generic-password")
            .args(&scope)
            .args(["-s", service, "-w"])
            .output()
            .await;
        if let Ok(output) = output
            && output.status.success()
        {
            let secret = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !secret.is_empty() && !secrets.contains(&secret) {
                secrets.push(secret);
            }
        }
    }
    secrets
}

async fn read_claude_credentials(account: &AgentAccount, home_dir: &Path) -> Vec<ClaudeOAuth> {
    let mut files: Vec<PathBuf> = vec![account.home.join(".credentials.json")];
    let services = if account.is_default {
        files.push(home_dir.join(".config/claude/credentials.json"));
        vec![CLAUDE_KEYCHAIN_SERVICE.to_string()]
    } else {
        claude_keychain_services(&account.home, home_dir)
    };
    let mut secrets: Vec<String> = files
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .collect();
    for service in &services {
        secrets.extend(read_keychain_secrets(service).await);
    }
    secrets
        .iter()
        .filter_map(|json| parse_claude_credential(json))
        .collect()
}

async fn fetch_claude(account: &AgentAccount, http: Arc<dyn HttpClient>) -> Result<AccountQuota> {
    let now_ms = Utc::now().timestamp_millis();
    let credentials = read_claude_credentials(account, util::paths::home_dir()).await;
    let Some(credential) = pick_freshest(credentials, now_ms) else {
        return Ok(AccountQuota::with_status(QuotaStatus::SignedOut));
    };
    match credential.freshness(now_ms) {
        TokenFreshness::Live => {}
        TokenFreshness::Stale => return Ok(AccountQuota::with_status(QuotaStatus::TokenStale)),
        TokenFreshness::Expired => {
            return Ok(AccountQuota::with_status(QuotaStatus::TokenExpired));
        }
    }
    let token = credential.access_token.clone().unwrap_or_default();
    let request = Request::builder()
        .method(Method::GET)
        .uri(CLAUDE_USAGE_URL)
        .header("Authorization", format!("Bearer {token}"))
        .header("anthropic-beta", CLAUDE_OAUTH_BETA)
        .body(AsyncBody::default())?;
    let (status, body) = send(&http, request).await?;
    if status == 401 || status == 403 {
        return Ok(AccountQuota::with_status(QuotaStatus::TokenExpired));
    }
    if !(200..300).contains(&status) {
        return Ok(AccountQuota::with_status(QuotaStatus::Unavailable(
            format!("usage endpoint returned {status}"),
        )));
    }
    let windows = parse_claude_usage(&body)?;
    Ok(finish(windows, credential.subscription_type))
}

#[derive(Deserialize)]
struct ClaudeWindow {
    utilization: Option<f64>,
    resets_at: Option<String>,
}

#[derive(Deserialize)]
struct ClaudeLimit {
    kind: Option<String>,
    percent: Option<f64>,
    resets_at: Option<String>,
    scope: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    five_hour: Option<ClaudeWindow>,
    seven_day: Option<ClaudeWindow>,
    seven_day_sonnet: Option<ClaudeWindow>,
    #[serde(default)]
    limits: Vec<ClaudeLimit>,
}

fn parse_claude_usage(body: &str) -> Result<Vec<QuotaWindow>> {
    let usage: ClaudeUsage = serde_json::from_str(body).context("parsing Claude usage")?;
    let mut windows = Vec::new();
    for (id, label, window) in [
        ("five_hour", "Session (5h)", usage.five_hour),
        ("seven_day", "Weekly", usage.seven_day),
        (
            "seven_day_sonnet",
            "Weekly · Sonnet",
            usage.seven_day_sonnet,
        ),
    ] {
        if let Some(window) = window
            && let Some(utilization) = window.utilization
        {
            windows.push(QuotaWindow {
                id: id.into(),
                label: label.into(),
                used_percent: percent(utilization),
                resets_at: window.resets_at.as_deref().and_then(parse_iso),
            });
        }
    }
    for limit in usage.limits {
        let name = limit
            .scope
            .as_ref()
            .and_then(|scope| scope.pointer("/model/display_name"))
            .and_then(|name| name.as_str());
        if limit.kind.as_deref() != Some("weekly_scoped") {
            continue;
        }
        let (Some(used), Some(name)) = (limit.percent, name) else {
            continue;
        };
        let label = format!("Weekly · {name}");
        if windows.iter().any(|window| window.label == label) {
            continue;
        }
        windows.push(QuotaWindow {
            id: format!("weekly_scoped:{name}"),
            label,
            used_percent: percent(used),
            resets_at: limit.resets_at.as_deref().and_then(parse_iso),
        });
    }
    Ok(windows)
}

// ----------------------------------------------------------------- Codex

#[derive(Deserialize)]
struct CodexAuthFile {
    tokens: Option<CodexTokens>,
}

#[derive(Deserialize)]
struct CodexTokens {
    access_token: Option<String>,
    account_id: Option<String>,
}

async fn fetch_codex(home: &Path, http: Arc<dyn HttpClient>) -> Result<AccountQuota> {
    let Ok(auth) = std::fs::read_to_string(home.join("auth.json")) else {
        return Ok(AccountQuota::with_status(QuotaStatus::SignedOut));
    };
    let tokens = serde_json::from_str::<CodexAuthFile>(&auth)
        .ok()
        .and_then(|auth| auth.tokens);
    let Some((token, account_id)) =
        tokens.and_then(|tokens| Some((tokens.access_token?, tokens.account_id)))
    else {
        // API-key logins have no subscription quota.
        return Ok(AccountQuota::with_status(QuotaStatus::Ok));
    };
    let mut request = Request::builder()
        .method(Method::GET)
        .uri(CODEX_USAGE_URL)
        .header("Authorization", format!("Bearer {token}"));
    if let Some(account_id) = account_id {
        request = request.header("chatgpt-account-id", account_id);
    }
    let (status, body) = send(&http, request.body(AsyncBody::default())?).await?;
    if status == 401 || status == 403 {
        return Ok(AccountQuota::with_status(QuotaStatus::TokenExpired));
    }
    if !(200..300).contains(&status) {
        return Ok(AccountQuota::with_status(QuotaStatus::Unavailable(
            format!("usage endpoint returned {status}"),
        )));
    }
    let (windows, plan) = parse_codex_usage(&body, Utc::now())?;
    Ok(finish(windows, plan))
}

#[derive(Deserialize)]
struct CodexWindow {
    used_percent: Option<f64>,
    limit_window_seconds: Option<f64>,
    reset_at: Option<i64>,
    reset_after_seconds: Option<i64>,
}

#[derive(Deserialize)]
struct CodexRateLimit {
    primary_window: Option<CodexWindow>,
    secondary_window: Option<CodexWindow>,
}

#[derive(Deserialize)]
struct CodexAdditionalLimit {
    limit_name: Option<String>,
    rate_limit: Option<CodexRateLimit>,
}

#[derive(Deserialize)]
struct CodexUsage {
    plan_type: Option<String>,
    rate_limit: Option<CodexRateLimit>,
    #[serde(default)]
    additional_rate_limits: Vec<CodexAdditionalLimit>,
}

fn codex_window_label(window: &CodexWindow) -> String {
    let Some(seconds) = window.limit_window_seconds else {
        return "Limit".into();
    };
    let hours = (seconds / 3600.0).round() as i64;
    if hours <= 5 {
        format!("Session ({hours}h)")
    } else if hours == 168 {
        "Weekly".into()
    } else if hours % 24 == 0 {
        format!("{}d", hours / 24)
    } else {
        format!("{hours}h")
    }
}

fn codex_window(
    id: String,
    label: String,
    window: &CodexWindow,
    now: DateTime<Utc>,
) -> Option<QuotaWindow> {
    Some(QuotaWindow {
        id,
        label,
        used_percent: percent(window.used_percent?),
        resets_at: window
            .reset_at
            .and_then(|at| Utc.timestamp_opt(at, 0).single())
            .or_else(|| {
                window
                    .reset_after_seconds
                    .map(|after| now + chrono::Duration::seconds(after))
            }),
    })
}

fn parse_codex_usage(body: &str, now: DateTime<Utc>) -> Result<(Vec<QuotaWindow>, Option<String>)> {
    let usage: CodexUsage = serde_json::from_str(body).context("parsing Codex usage")?;
    let mut windows = Vec::new();
    if let Some(rate_limit) = &usage.rate_limit {
        for (id, window) in [
            ("primary", &rate_limit.primary_window),
            ("secondary", &rate_limit.secondary_window),
        ] {
            if let Some(window) = window
                && let Some(window) =
                    codex_window(id.into(), codex_window_label(window), window, now)
            {
                windows.push(window);
            }
        }
    }
    for (index, limit) in usage.additional_rate_limits.iter().enumerate() {
        let name = limit
            .limit_name
            .clone()
            .unwrap_or_else(|| format!("limit_{index}"));
        if let Some(window) = limit
            .rate_limit
            .as_ref()
            .and_then(|rate_limit| rate_limit.primary_window.as_ref())
            && let Some(window) = codex_window(
                format!("additional:{name}"),
                format!("{} · {name}", codex_window_label(window)),
                window,
                now,
            )
        {
            windows.push(window);
        }
    }
    Ok((windows, usage.plan_type))
}

// ------------------------------------------------------------------ Grok

async fn fetch_grok(home: &Path, http: Arc<dyn HttpClient>) -> Result<AccountQuota> {
    let Ok(auth) = std::fs::read_to_string(home.join("auth.json")) else {
        return Ok(AccountQuota::with_status(QuotaStatus::SignedOut));
    };
    let auth: serde_json::Value = serde_json::from_str(&auth).context("parsing Grok auth")?;
    let Some(entry) = auth
        .as_object()
        .and_then(|entries| entries.values().find(|entry| entry.get("key").is_some()))
    else {
        return Ok(AccountQuota::with_status(QuotaStatus::SignedOut));
    };
    if let Some(expires_at) = entry.get("expires_at").and_then(parse_flexible_time)
        && expires_at <= Utc::now()
    {
        return Ok(AccountQuota::with_status(QuotaStatus::TokenExpired));
    }
    let key = entry
        .get("key")
        .and_then(|key| key.as_str())
        .ok_or_else(|| anyhow!("Grok login has no key"))?;
    let plan = entry
        .get("subscription_tier")
        .and_then(|tier| tier.as_str())
        .map(str::to_string);
    let request = Request::builder()
        .method(Method::POST)
        .uri(GROK_USAGE_URL)
        .header("Authorization", format!("Bearer {key}"))
        .header("Origin", "https://grok.com")
        .header("Referer", "https://grok.com/?_s=usage")
        .header("Accept", "*/*")
        .header("Content-Type", "application/grpc-web+proto")
        .header("x-grpc-web", "1")
        .header("x-user-agent", "connect-es/2.1.1")
        // An empty gRPC-web frame.
        .body(AsyncBody::from(vec![0u8; 5]))?;
    let mut response = http.send(request).await?;
    let status = response.status().as_u16();
    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;
    if status == 401 || status == 403 {
        return Ok(AccountQuota::with_status(QuotaStatus::TokenExpired));
    }
    if !(200..300).contains(&status) {
        return Ok(AccountQuota::with_status(QuotaStatus::Unavailable(
            format!("usage endpoint returned {status}"),
        )));
    }
    let window = parse_grok_quota(&body, Utc::now())?;
    Ok(finish(vec![window], plan))
}

fn parse_flexible_time(value: &serde_json::Value) -> Option<DateTime<Utc>> {
    if let Some(number) = value.as_f64() {
        let millis = if number < 1e12 {
            number * 1000.0
        } else {
            number
        };
        return Utc.timestamp_millis_opt(millis as i64).single();
    }
    value.as_str().and_then(parse_iso)
}

#[derive(Debug)]
enum ProtoValue {
    Fixed32(f32),
    Varint(u64),
}

fn read_varint(bytes: &[u8], position: &mut usize) -> Option<u64> {
    let mut value = 0u64;
    for shift in (0..64).step_by(7) {
        let byte = *bytes.get(*position)?;
        *position += 1;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}

/// Collects scalar fields with their field-number paths, descending into
/// length-delimited fields that parse as messages.
fn scan_proto(
    bytes: &[u8],
    path: &mut Vec<u64>,
    depth: usize,
    out: &mut Vec<(Vec<u64>, ProtoValue)>,
) -> bool {
    let mut position = 0;
    while position < bytes.len() {
        let Some(key) = read_varint(bytes, &mut position) else {
            return false;
        };
        let field = key >> 3;
        if field == 0 {
            return false;
        }
        path.push(field);
        match key & 7 {
            0 => {
                let Some(value) = read_varint(bytes, &mut position) else {
                    return false;
                };
                out.push((path.clone(), ProtoValue::Varint(value)));
            }
            1 => position += 8,
            2 => {
                let Some(len) = read_varint(bytes, &mut position) else {
                    return false;
                };
                let end = position + len as usize;
                if end > bytes.len() {
                    return false;
                }
                if depth < 4 {
                    let mut nested = Vec::new();
                    if scan_proto(&bytes[position..end], path, depth + 1, &mut nested) {
                        out.extend(nested);
                    }
                }
                position = end;
            }
            5 => {
                let Some(raw) = bytes.get(position..position + 4) else {
                    return false;
                };
                let value = f32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
                out.push((path.clone(), ProtoValue::Fixed32(value)));
                position += 4;
            }
            _ => return false,
        }
        path.pop();
    }
    position == bytes.len()
}

/// Grok's billing response has no published schema; like Superset, take the
/// first 0–100 float in a field 1 as the weekly percentage and a future unix
/// timestamp as the reset time.
fn parse_grok_quota(body: &[u8], now: DateTime<Utc>) -> Result<QuotaWindow> {
    let mut payloads = Vec::new();
    let mut position = 0;
    while position + 5 <= body.len() {
        let flags = body[position];
        let len = u32::from_be_bytes([
            body[position + 1],
            body[position + 2],
            body[position + 3],
            body[position + 4],
        ]) as usize;
        let start = position + 5;
        let Some(frame) = body.get(start..start + len) else {
            break;
        };
        if flags & 0x80 == 0 {
            payloads.push(frame);
        }
        position = start + len;
    }
    if payloads.is_empty() {
        payloads.push(body);
    }
    let mut values = Vec::new();
    for payload in payloads {
        scan_proto(payload, &mut Vec::new(), 0, &mut values);
    }

    let mut percents: Vec<_> = values
        .iter()
        .filter_map(|(path, value)| match value {
            ProtoValue::Fixed32(value)
                if path.last() == Some(&1)
                    && value.is_finite()
                    && (0.0..=100.0).contains(value) =>
            {
                Some((path.len(), *value))
            }
            _ => None,
        })
        .collect();
    percents.sort_by_key(|(depth, _)| *depth);
    let used = percents
        .first()
        .map(|(_, value)| *value)
        .ok_or_else(|| anyhow!("no weekly quota in Grok's response"))?;

    let now_seconds = now.timestamp() as u64;
    let resets: Vec<_> = values
        .iter()
        .filter_map(|(path, value)| match value {
            ProtoValue::Varint(seconds)
                if (1_700_000_000..2_100_000_000).contains(seconds) && *seconds > now_seconds =>
            {
                Some((path.clone(), *seconds))
            }
            _ => None,
        })
        .collect();
    let reset = resets
        .iter()
        .find(|(path, _)| path == &[1, 5, 1])
        .or_else(|| resets.first())
        .and_then(|(_, seconds)| Utc.timestamp_opt(*seconds as i64, 0).single());

    Ok(QuotaWindow {
        id: "weekly".into(),
        label: "Weekly".into(),
        used_percent: used.round().clamp(0.0, 100.0) as u32,
        resets_at: reset,
    })
}

// --------------------------------------------------------------- helpers

async fn send(http: &Arc<dyn HttpClient>, request: Request<AsyncBody>) -> Result<(u16, String)> {
    let mut response = http.send(request).await?;
    let status = response.status().as_u16();
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    Ok((status, body))
}

fn finish(windows: Vec<QuotaWindow>, plan: Option<String>) -> AccountQuota {
    if windows.is_empty() {
        return AccountQuota::with_status(QuotaStatus::Unavailable(
            "no quota reported for this plan".into(),
        ));
    }
    AccountQuota {
        status: QuotaStatus::Ok,
        windows,
        plan,
    }
}

fn percent(value: f64) -> u32 {
    value.round().max(0.0) as u32
}

fn parse_iso(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|date| date.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_claude_windows() {
        let body = r#"{
            "five_hour": {"utilization": 42.4, "resets_at": "2026-10-04T12:00:00Z"},
            "seven_day": {"utilization": 80, "resets_at": null},
            "seven_day_sonnet": null,
            "limits": [
                {"kind": "weekly_scoped", "percent": 12, "resets_at": null,
                 "scope": {"model": {"display_name": "Opus"}}},
                {"kind": "other", "percent": 99, "scope": {"model": {"display_name": "X"}}}
            ]
        }"#;
        let windows = parse_claude_usage(body).unwrap();
        let summary: Vec<_> = windows
            .iter()
            .map(|window| {
                (
                    window.id.as_str(),
                    window.label.as_str(),
                    window.used_percent,
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                ("five_hour", "Session (5h)", 42),
                ("seven_day", "Weekly", 80),
                ("weekly_scoped:Opus", "Weekly · Opus", 12),
            ]
        );
        assert_eq!(windows[0].resets_at, parse_iso("2026-10-04T12:00:00Z"));
    }

    #[test]
    fn claude_token_freshness_and_choice() {
        let now = 1_000_000;
        let live = ClaudeOAuth {
            access_token: Some("a".into()),
            expires_at: Some(now + 10),
            refresh_token: None,
            refresh_token_expires_at: None,
            subscription_type: None,
        };
        let stale = ClaudeOAuth {
            expires_at: Some(now - 10),
            refresh_token: Some("r".into()),
            ..live.clone()
        };
        let expired = ClaudeOAuth {
            refresh_token: None,
            ..stale.clone()
        };
        assert_eq!(live.freshness(now), TokenFreshness::Live);
        assert_eq!(stale.freshness(now), TokenFreshness::Stale);
        assert_eq!(expired.freshness(now), TokenFreshness::Expired);
        let chosen = pick_freshest(vec![expired, live.clone(), stale], now).unwrap();
        assert_eq!(chosen.expires_at, live.expires_at);
        assert!(parse_claude_credential(r#"{"claudeAiOauth":{"expiresAt":1}}"#).is_none());
    }

    #[test]
    fn claude_keychain_service_names_cover_spellings() {
        let services =
            claude_keychain_services(Path::new("/Users/me/.claude-work"), Path::new("/Users/me"));
        assert_eq!(services.len(), 6);
        assert!(
            services
                .iter()
                .all(|service| service.starts_with("Claude Code-credentials-")
                    && service.len() == "Claude Code-credentials-".len() + 8)
        );
    }

    #[test]
    fn parses_codex_windows() {
        let now = parse_iso("2026-10-04T00:00:00Z").unwrap();
        let body = r#"{
            "plan_type": "plus",
            "rate_limit": {
                "primary_window": {"used_percent": 30.6, "limit_window_seconds": 18000, "reset_after_seconds": 60},
                "secondary_window": {"used_percent": 70, "limit_window_seconds": 604800, "reset_at": 1790000000}
            },
            "additional_rate_limits": [
                {"limit_name": "codex-max", "rate_limit": {"primary_window": {"used_percent": 5, "limit_window_seconds": 172800}}}
            ]
        }"#;
        let (windows, plan) = parse_codex_usage(body, now).unwrap();
        assert_eq!(plan.as_deref(), Some("plus"));
        let summary: Vec<_> = windows
            .iter()
            .map(|window| {
                (
                    window.id.as_str(),
                    window.label.as_str(),
                    window.used_percent,
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                ("primary", "Session (5h)", 31),
                ("secondary", "Weekly", 70),
                ("additional:codex-max", "2d · codex-max", 5),
            ]
        );
        assert_eq!(
            windows[0].resets_at,
            Some(now + chrono::Duration::seconds(60))
        );
    }

    #[test]
    fn parses_grok_grpc_web_frame() {
        // message { 1: { 1: float 37.5, 5: { 1: varint 2000000000 } } }
        let mut inner = vec![0x0d];
        inner.extend_from_slice(&37.5f32.to_le_bytes());
        let mut reset = vec![0x08];
        let mut value = 2_000_000_000u64;
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                reset.push(byte);
                break;
            }
            reset.push(byte | 0x80);
        }
        inner.push(0x2a);
        inner.push(reset.len() as u8);
        inner.extend_from_slice(&reset);
        let mut message = vec![0x0a, inner.len() as u8];
        message.extend_from_slice(&inner);
        let mut body = vec![0u8];
        body.extend_from_slice(&(message.len() as u32).to_be_bytes());
        body.extend_from_slice(&message);
        // A trailer frame is ignored.
        body.extend_from_slice(&[0x80, 0, 0, 0, 0]);

        let now = parse_iso("2026-10-04T00:00:00Z").unwrap();
        let window = parse_grok_quota(&body, now).unwrap();
        assert_eq!(window.used_percent, 38);
        assert_eq!(
            window.resets_at,
            Utc.timestamp_opt(2_000_000_000, 0).single()
        );
    }

    #[test]
    fn exhaustion_uses_the_most_used_window() {
        let quota = AccountQuota {
            status: QuotaStatus::Ok,
            windows: vec![
                QuotaWindow {
                    id: "a".into(),
                    label: "A".into(),
                    used_percent: 20,
                    resets_at: None,
                },
                QuotaWindow {
                    id: "b".into(),
                    label: "B".into(),
                    used_percent: 96,
                    resets_at: None,
                },
            ],
            plan: None,
        };
        assert!(quota.is_exhausted(95.0));
        assert!(!quota.is_exhausted(97.0));
        assert_eq!(quota.summary().as_deref(), Some("A 20% · B 96%"));
    }
}
