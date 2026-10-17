use agent_client_protocol::schema::v1 as acp_v1;
use chrono::{DateTime, Utc};
use collections::HashMap;
use serde::Deserialize;

/// `_meta` key under which the Claude Agent adapter forwards the
/// `rate_limit_event` emitted by Claude Code on `usage_update` notifications.
pub const CLAUDE_RATE_LIMIT_META_KEY: &str = "_claude/rateLimit";

pub const RATE_LIMIT_WARNING_THRESHOLD: f32 = 0.8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitStatus {
    Allowed,
    AllowedWarning,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitWindowKind {
    FiveHour,
    SevenDay,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RateLimitWindow {
    pub status: RateLimitStatus,
    /// Fraction of the window consumed, in `0.0..=1.0`.
    pub utilization: Option<f32>,
    pub resets_at: Option<DateTime<Utc>>,
}

impl RateLimitWindow {
    pub fn is_warning(&self) -> bool {
        match self.status {
            RateLimitStatus::AllowedWarning | RateLimitStatus::Rejected => true,
            RateLimitStatus::Allowed => self
                .utilization
                .is_some_and(|utilization| utilization >= RATE_LIMIT_WARNING_THRESHOLD),
        }
    }
}

/// Plan usage windows reported by the agent (session window and weekly cap).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RateLimits {
    pub five_hour: Option<RateLimitWindow>,
    pub seven_day: Option<RateLimitWindow>,
}

impl RateLimits {
    pub fn is_empty(&self) -> bool {
        self.five_hour.is_none() && self.seven_day.is_none()
    }

    pub fn window(&self, kind: RateLimitWindowKind) -> Option<&RateLimitWindow> {
        match kind {
            RateLimitWindowKind::FiveHour => self.five_hour.as_ref(),
            RateLimitWindowKind::SevenDay => self.seven_day.as_ref(),
        }
    }

    /// Merges the rate limits carried by a `usage_update` `_meta`, if any.
    /// Returns whether a window was updated.
    pub fn apply_meta(&mut self, meta: Option<&acp_v1::Meta>) -> bool {
        let Some(windows) = meta
            .and_then(|meta| meta.get(CLAUDE_RATE_LIMIT_META_KEY))
            .and_then(parse_claude_rate_limit)
        else {
            return false;
        };
        let mut updated = false;
        for (kind, window) in windows {
            let slot = match kind {
                RateLimitWindowKind::FiveHour => &mut self.five_hour,
                RateLimitWindowKind::SevenDay => &mut self.seven_day,
            };
            if slot.as_ref() != Some(&window) {
                *slot = Some(window);
                updated = true;
            }
        }
        updated
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeRateLimitInfo {
    status: RateLimitStatus,
    resets_at: Option<i64>,
    rate_limit_type: Option<String>,
    utilization: Option<f32>,
    /// Utilization of every plan window, keyed by window name. Unlike the
    /// top-level fields, which only describe the most constrained window,
    /// these are reported even when the window is far from its limit.
    #[serde(default)]
    unified_windows: HashMap<String, ClaudeUnifiedWindow>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeUnifiedWindow {
    utilization: Option<f32>,
    resets_at: Option<i64>,
}

fn parse_claude_rate_limit(
    value: &serde_json::Value,
) -> Option<Vec<(RateLimitWindowKind, RateLimitWindow)>> {
    let info = ClaudeRateLimitInfo::deserialize(value)
        .inspect_err(|error| log::debug!("Ignoring malformed Claude rate limit info: {error}"))
        .ok()?;
    let primary_kind = match info.rate_limit_type.as_deref() {
        Some("five_hour") => Some(RateLimitWindowKind::FiveHour),
        Some("seven_day") => Some(RateLimitWindowKind::SevenDay),
        _ => None,
    };

    let windows = [
        (RateLimitWindowKind::FiveHour, "five_hour"),
        (RateLimitWindowKind::SevenDay, "seven_day"),
    ]
    .into_iter()
    .filter_map(|(kind, name)| {
        let is_primary = primary_kind == Some(kind);
        // The status only describes the most constrained window.
        let status = if is_primary {
            info.status
        } else {
            RateLimitStatus::Allowed
        };
        let (utilization, resets_at) = match info.unified_windows.get(name) {
            Some(window) => (window.utilization, window.resets_at),
            None if is_primary => (info.utilization, info.resets_at),
            None => return None,
        };
        Some((
            kind,
            RateLimitWindow {
                status,
                utilization: utilization
                    .filter(|utilization| utilization.is_finite())
                    .map(|utilization| utilization.clamp(0.0, 1.0)),
                resets_at: resets_at.and_then(|seconds| DateTime::from_timestamp(seconds, 0)),
            },
        ))
    })
    .collect::<Vec<_>>();

    (!windows.is_empty()).then_some(windows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn meta(value: serde_json::Value) -> acp_v1::Meta {
        serde_json::from_value(json!({ CLAUDE_RATE_LIMIT_META_KEY: value })).unwrap()
    }

    #[test]
    fn test_applies_five_hour_and_weekly_windows() {
        let mut limits = RateLimits::default();

        assert!(limits.apply_meta(Some(&meta(json!({
            "status": "allowed",
            "resetsAt": 1_700_000_000,
            "rateLimitType": "five_hour",
            "utilization": 0.27,
        })))));
        assert!(limits.apply_meta(Some(&meta(json!({
            "status": "allowed_warning",
            "resetsAt": 1_700_500_000,
            "rateLimitType": "seven_day",
            "utilization": 0.91,
        })))));

        let five_hour = limits.five_hour.as_ref().unwrap();
        assert_eq!(five_hour.status, RateLimitStatus::Allowed);
        assert_eq!(five_hour.utilization, Some(0.27));
        assert_eq!(
            five_hour.resets_at,
            DateTime::from_timestamp(1_700_000_000, 0)
        );
        assert!(!five_hour.is_warning());

        let seven_day = limits.seven_day.as_ref().unwrap();
        assert_eq!(seven_day.status, RateLimitStatus::AllowedWarning);
        assert!(seven_day.is_warning());
    }

    #[test]
    fn test_reads_every_window_from_unified_windows() {
        let mut limits = RateLimits::default();

        assert!(limits.apply_meta(Some(&meta(json!({
            "status": "allowed",
            "resetsAt": 1_700_000_000,
            "rateLimitType": "five_hour",
            "overageStatus": "allowed",
            "isUsingOverage": false,
            "unifiedWindows": {
                "five_hour": { "utilization": 0.47, "resetsAt": 1_700_000_000 },
                "seven_day": { "utilization": 0.25, "resetsAt": 1_700_500_000 },
            },
        })))));

        let five_hour = limits.five_hour.as_ref().unwrap();
        assert_eq!(five_hour.status, RateLimitStatus::Allowed);
        assert_eq!(five_hour.utilization, Some(0.47));
        assert_eq!(
            five_hour.resets_at,
            DateTime::from_timestamp(1_700_000_000, 0)
        );

        let seven_day = limits.seven_day.as_ref().unwrap();
        assert_eq!(seven_day.utilization, Some(0.25));
        assert_eq!(
            seven_day.resets_at,
            DateTime::from_timestamp(1_700_500_000, 0)
        );
    }

    #[test]
    fn test_primary_status_only_applies_to_its_window() {
        let mut limits = RateLimits::default();

        limits.apply_meta(Some(&meta(json!({
            "status": "allowed_warning",
            "rateLimitType": "seven_day",
            "unifiedWindows": {
                "five_hour": { "utilization": 0.1 },
                "seven_day": { "utilization": 0.75 },
            },
        }))));

        assert!(!limits.five_hour.as_ref().unwrap().is_warning());
        let seven_day = limits.seven_day.as_ref().unwrap();
        assert_eq!(seven_day.status, RateLimitStatus::AllowedWarning);
        assert!(seven_day.is_warning());
    }

    #[test]
    fn test_ignores_unrelated_or_malformed_meta() {
        let mut limits = RateLimits::default();

        assert!(!limits.apply_meta(None));
        assert!(!limits.apply_meta(Some(&acp_v1::Meta::new())));
        assert!(!limits.apply_meta(Some(&meta(json!({ "status": "nope" })))));
        assert!(!limits.apply_meta(Some(&meta(json!({
            "status": "allowed",
            "utilization": 0.5,
        })))));
        assert!(!limits.apply_meta(Some(&meta(json!({
            "status": "allowed",
            "rateLimitType": "overage",
            "utilization": 0.5,
        })))));
        assert!(limits.is_empty());
    }

    #[test]
    fn test_clamps_utilization_and_skips_unchanged_updates() {
        let mut limits = RateLimits::default();
        let update = meta(json!({
            "status": "rejected",
            "rateLimitType": "five_hour",
            "utilization": 1.4,
        }));

        assert!(limits.apply_meta(Some(&update)));
        assert!(!limits.apply_meta(Some(&update)));
        assert_eq!(limits.five_hour.as_ref().unwrap().utilization, Some(1.0));
        assert!(limits.five_hour.as_ref().unwrap().is_warning());
    }
}
