use std::sync::Arc;

use anyhow::{Context as _, Result};
use chrono::{Datelike, Utc};
use client::{Client, UserStore, zed_urls};
use copilot_chat::CopilotChat;
use futures::AsyncReadExt as _;
use gpui::{App, AsyncApp, Entity};
use http_client::{AsyncBody, HttpClient, Method, Request};
use language_model::{
    ANTHROPIC_PROVIDER_ID, LanguageModelProviderId, LanguageModelRegistry, OPEN_AI_PROVIDER_ID,
    ZED_CLOUD_PROVIDER_ID,
};

const COPILOT_CHAT_PROVIDER_ID: LanguageModelProviderId =
    LanguageModelProviderId::new("copilot_chat");
const OPENROUTER_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openrouter");
const MISTRAL_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("mistral");
use project::DisableAiSettings;
use serde::Deserialize;
use settings::Settings;
use theme::ActiveTheme;

#[derive(Debug, Clone, PartialEq)]
pub struct CreditSnapshot {
    pub provider_label: String,
    pub used_ratio: f32,
    pub label: String,
    pub tooltip: String,
    pub account_url: Option<String>,
}

pub fn active_provider_id(cx: &App) -> Option<LanguageModelProviderId> {
    if DisableAiSettings::get_global(cx).disable_ai {
        return None;
    }

    LanguageModelRegistry::read_global(cx)
        .default_model()
        .map(|model| model.model.provider_id())
}

pub async fn fetch_credit_snapshot(
    provider_id: LanguageModelProviderId,
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    monthly_budget_usd: Option<f32>,
    cx: &AsyncApp,
) -> Result<CreditSnapshot> {
    if provider_id == ZED_CLOUD_PROVIDER_ID {
        return fetch_zed_hosted(user_store, client, cx).await;
    }

    if provider_id == COPILOT_CHAT_PROVIDER_ID {
        return fetch_copilot(client.http_client(), cx).await;
    }

    if provider_id == OPENROUTER_PROVIDER_ID {
        return fetch_openrouter(client.http_client()).await;
    }

    if provider_id == OPEN_AI_PROVIDER_ID {
        return fetch_openai(client.http_client(), monthly_budget_usd).await;
    }

    if provider_id == ANTHROPIC_PROVIDER_ID {
        return fetch_anthropic(client.http_client(), monthly_budget_usd).await;
    }

    if provider_id == MISTRAL_PROVIDER_ID {
        return fetch_mistral(client.http_client(), monthly_budget_usd).await;
    }

    anyhow::bail!("Unsupported provider: {}", provider_id)
}

fn zed_hosted_snapshot(
    usage: cloud_llm_client::TokenSpendUsage,
    cx: &App,
) -> CreditSnapshot {
    let limit = usage.limit_cents.max(1) as f32;
    let spent = usage.spent_cents as f32;
    CreditSnapshot {
        provider_label: "Zed Pro".to_string(),
        used_ratio: (spent / limit).clamp(0.0, 1.0),
        label: format!(
            "${:.2}/${:.2}",
            spent / 100.0,
            usage.limit_cents as f32 / 100.0
        ),
        tooltip: format!(
            "Zed Pro token spend: ${:.2} of ${:.2} monthly limit",
            spent / 100.0,
            usage.limit_cents as f32 / 100.0
        ),
        account_url: Some(zed_urls::account_url(cx)),
    }
}

async fn fetch_zed_hosted(
    user_store: Entity<UserStore>,
    _client: Arc<Client>,
    cx: &AsyncApp,
) -> Result<CreditSnapshot> {
    if let Some(snapshot) = cx.update(|cx| {
        user_store
            .read(cx)
            .token_spend_usage()
            .map(|usage| zed_hosted_snapshot(usage, cx))
    }) {
        return Ok(snapshot);
    }

    let refresh = cx.update(|cx| {
        user_store.update(cx, |store, cx| store.refresh_authenticated_user(cx))
    });
    refresh
        .await
        .context("failed to refresh Zed account usage")?;

    cx.update(|cx| {
        user_store
            .read(cx)
            .token_spend_usage()
            .map(|usage| zed_hosted_snapshot(usage, cx))
    })
    .ok_or_else(|| anyhow::anyhow!("Zed Pro token spend is not available from your account yet"))
}

async fn fetch_copilot(http: Arc<dyn HttpClient>, cx: &AsyncApp) -> Result<CreditSnapshot> {
    let oauth_token = cx
        .update(|cx| {
            CopilotChat::global(cx)
                .and_then(|chat| chat.read(cx).oauth_token().map(str::to_string))
                .or_else(resolve_github_token)
        })
        .ok_or_else(|| anyhow::anyhow!("Sign in to GitHub Copilot to view usage"))?;

    #[derive(Debug, Deserialize)]
    struct CopilotUserResponse {
        #[allow(dead_code)]
        copilot_plan: Option<String>,
        quota_reset_date: Option<String>,
        quota_snapshots: Option<QuotaSnapshots>,
    }

    #[derive(Debug, Deserialize)]
    struct QuotaSnapshots {
        premium_interactions: Option<QuotaSnapshot>,
    }

    #[derive(Debug, Deserialize)]
    struct QuotaSnapshot {
        percent_remaining: Option<f64>,
        quota_remaining: Option<f64>,
        entitlement: Option<f64>,
        unlimited: Option<bool>,
    }

    let request = Request::builder()
        .method(Method::GET)
        .uri("https://api.github.com/copilot_internal/user")
        .header("Authorization", format!("Bearer {oauth_token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "zed-ai-credit-status")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .body(AsyncBody::default())?;

    let mut response = http.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub Copilot usage request failed: {}", body);
    }

    let parsed: CopilotUserResponse = serde_json::from_str(&body)?;
    let premium = parsed
        .quota_snapshots
        .and_then(|snapshots| snapshots.premium_interactions)
        .context("No Copilot premium quota data available")?;

    if premium.unlimited.unwrap_or(false) {
        return Ok(CreditSnapshot {
            provider_label: "Copilot".to_string(),
            used_ratio: 0.0,
            label: "Premium included".to_string(),
            tooltip: "GitHub Copilot premium requests are included with your plan".to_string(),
            account_url: Some("https://github.com/settings/copilot".into()),
        });
    }

    let percent_remaining = premium.percent_remaining.unwrap_or(100.0) as f32;
    let used_ratio = ((100.0 - percent_remaining) / 100.0).clamp(0.0, 1.0);
    let label = match (premium.quota_remaining, premium.entitlement) {
        (Some(remaining), Some(total)) if total > 0.0 => {
            format!("{:.0}% ({:.0}/{:.0})", used_ratio * 100.0, total - remaining, total)
        }
        _ => format!("{:.0}%", used_ratio * 100.0),
    };

    let reset = parsed
        .quota_reset_date
        .map(|date| format!("\nResets {date}"))
        .unwrap_or_default();

    Ok(CreditSnapshot {
        provider_label: "Copilot".to_string(),
        used_ratio,
        label,
        tooltip: format!(
            "GitHub Copilot premium requests: {:.0}% used{reset}",
            used_ratio * 100.0
        ),
        account_url: Some("https://github.com/settings/copilot".into()),
    })
}

async fn fetch_openrouter(http: Arc<dyn HttpClient>) -> Result<CreditSnapshot> {
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .context("Set OPENROUTER_API_KEY to view OpenRouter credits")?;

    #[derive(Debug, Deserialize)]
    struct KeyResponse {
        data: KeyData,
    }

    #[derive(Debug, Deserialize)]
    struct KeyData {
        limit: Option<f64>,
        limit_remaining: Option<f64>,
        usage: f64,
    }

    let request = Request::builder()
        .method(Method::GET)
        .uri("https://openrouter.ai/api/v1/key")
        .header("Authorization", format!("Bearer {api_key}"))
        .body(AsyncBody::default())?;

    let mut response = http.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if !response.status().is_success() {
        anyhow::bail!("OpenRouter usage request failed: {}", body);
    }

    let parsed: KeyResponse = serde_json::from_str(&body)?;
    let data = parsed.data;

    if let (Some(limit), Some(remaining)) = (data.limit, data.limit_remaining) {
        if limit > 0.0 {
            let used = (limit - remaining).max(0.0);
            let used_ratio = (used / limit).clamp(0.0, 1.0) as f32;
            return Ok(CreditSnapshot {
                provider_label: "OpenRouter".to_string(),
                used_ratio,
                label: format!("${:.2}/${:.2}", used, limit),
                tooltip: format!(
                    "OpenRouter credits: ${:.2} used of ${:.2} limit (${:.2} total usage)",
                    used, limit, data.usage
                ),
                account_url: Some("https://openrouter.ai/settings/credits".into()),
            });
        }
    }

    Ok(CreditSnapshot {
        provider_label: "OpenRouter".to_string(),
        used_ratio: 0.0,
        label: format!("${:.2} used", data.usage),
        tooltip: format!("OpenRouter total usage: ${:.2}", data.usage),
        account_url: Some("https://openrouter.ai/settings/credits".into()),
    })
}

async fn fetch_openai(
    http: Arc<dyn HttpClient>,
    monthly_budget_usd: Option<f32>,
) -> Result<CreditSnapshot> {
    let api_key =
        std::env::var("OPENAI_API_KEY").context("Set OPENAI_API_KEY to view OpenAI usage")?;
    let budget = monthly_budget_usd
        .filter(|budget| *budget > 0.0)
        .context("Set ai_credit_status.monthly_budget_usd to track OpenAI usage")?;

    let now = Utc::now();
    let start = format!("{}-{:02}-01", now.year(), now.month());

    #[derive(Debug, Deserialize)]
    struct UsageResponse {
        total_usage: Option<f64>,
    }

    let uri = format!(
        "https://api.openai.com/v1/usage?start_date={start}&end_date={start}"
    );
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Authorization", format!("Bearer {api_key}"))
        .body(AsyncBody::default())?;

    let mut response = http.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if !response.status().is_success() {
        anyhow::bail!("OpenAI usage request failed: {}", body);
    }

    let parsed: UsageResponse = serde_json::from_str(&body)?;
    let spent_usd = parsed.total_usage.unwrap_or(0.0) / 100.0;
    let used_ratio = (spent_usd / budget as f64).clamp(0.0, 1.0) as f32;

    Ok(CreditSnapshot {
        provider_label: "OpenAI".to_string(),
        used_ratio,
        label: format!("${:.2}/${:.2}", spent_usd, budget),
        tooltip: format!(
            "OpenAI usage this month: ${:.2} of ${:.2} configured budget",
            spent_usd, budget
        ),
        account_url: Some("https://platform.openai.com/usage".into()),
    })
}

async fn fetch_anthropic(
    http: Arc<dyn HttpClient>,
    monthly_budget_usd: Option<f32>,
) -> Result<CreditSnapshot> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("Set ANTHROPIC_API_KEY to view Anthropic usage")?;
    let budget = monthly_budget_usd
        .filter(|budget| *budget > 0.0)
        .context("Set ai_credit_status.monthly_budget_usd to track Anthropic usage")?;

    let request = Request::builder()
        .method(Method::GET)
        .uri("https://api.anthropic.com/v1/models")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .body(AsyncBody::default())?;

    let mut response = http.send(request).await?;
    if !response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!("Anthropic request failed: {}", body);
    }

    // Anthropic does not expose remaining credits on standard API keys. Surface
    // configured budget guidance until a billing endpoint is available.
    Ok(CreditSnapshot {
        provider_label: "Anthropic".to_string(),
        used_ratio: 0.0,
        label: format!("Budget ${:.2}", budget),
        tooltip: format!(
            "Anthropic does not expose remaining credits via API. \
             Configure ai_credit_status.monthly_budget_usd (${:.2}) and check \
             https://console.anthropic.com/settings/billing for actual spend.",
            budget
        ),
        account_url: Some("https://console.anthropic.com/settings/billing".into()),
    })
}

async fn fetch_mistral(
    http: Arc<dyn HttpClient>,
    monthly_budget_usd: Option<f32>,
) -> Result<CreditSnapshot> {
    let api_key =
        std::env::var("MISTRAL_API_KEY").context("Set MISTRAL_API_KEY to view Mistral usage")?;
    let budget = monthly_budget_usd
        .filter(|budget| *budget > 0.0)
        .context("Set ai_credit_status.monthly_budget_usd to track Mistral usage")?;

    let request = Request::builder()
        .method(Method::GET)
        .uri("https://api.mistral.ai/v1/models")
        .header("Authorization", format!("Bearer {api_key}"))
        .body(AsyncBody::default())?;

    let mut response = http.send(request).await?;
    if !response.status().is_success() {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::bail!("Mistral request failed: {}", body);
    }

    Ok(CreditSnapshot {
        provider_label: "Mistral".to_string(),
        used_ratio: 0.0,
        label: format!("Budget ${:.2}", budget),
        tooltip: format!(
            "Mistral does not expose remaining credits via API. \
             Configure ai_credit_status.monthly_budget_usd (${:.2}) and check \
             https://console.mistral.ai/billing for actual spend.",
            budget
        ),
        account_url: Some("https://console.mistral.ai/billing".into()),
    })
}

fn resolve_github_token() -> Option<String> {
    for key in [
        "GITHUB_TOKEN",
        "COPILOT_USAGE_TOKEN",
        copilot_chat::COPILOT_OAUTH_ENV_VAR,
        copilot_chat::GITHUB_COPILOT_OAUTH_ENV_VAR,
        "GH_TOKEN",
    ] {
        if let Ok(token) = std::env::var(key) {
            let token = token.trim();
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
    }
    None
}

pub fn usage_color(used_ratio: f32, cx: &App) -> gpui::Hsla {
    let ratio = used_ratio.clamp(0.0, 1.0);
    let status = cx.theme().status();
    if ratio < 0.25 {
        status.success
    } else if ratio < 0.50 {
        status.warning
    } else if ratio < 0.75 {
        status.modified
    } else {
        status.error
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    fn usage_color_escalates_with_ratio(cx: &mut TestAppContext) {
        cx.update(|cx| {
            assert_eq!(
                usage_color(0.1, cx),
                cx.theme().status().success
            );
            assert_eq!(
                usage_color(0.4, cx),
                cx.theme().status().warning
            );
            assert_eq!(
                usage_color(0.6, cx),
                cx.theme().status().modified
            );
            assert_eq!(usage_color(0.9, cx), cx.theme().status().error);
        });
    }
}
