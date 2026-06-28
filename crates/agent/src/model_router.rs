use settings::AgentModelRegistryEntry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IntelligenceTier {
    Boot,
    Mid,
    Heavy,
}

impl IntelligenceTier {
    pub(crate) fn for_score(score: f32) -> Self {
        if score <= 40.0 {
            Self::Boot
        } else if score <= 70.0 {
            Self::Mid
        } else {
            Self::Heavy
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Boot => "boot",
            Self::Mid => "mid",
            Self::Heavy => "heavy",
        }
    }

    fn contains_model_score(self, score: f32) -> bool {
        match self {
            Self::Boot => (0.0..=40.0).contains(&score),
            Self::Mid => score > 40.0 && score <= 70.0,
            Self::Heavy => score > 70.0 && score <= 100.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct TaskComplexitySignals {
    pub(crate) cyclomatic_complexity: Option<u32>,
    pub(crate) dependent_count: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SubagentTaskProfile {
    complexity_score: f32,
}

impl SubagentTaskProfile {
    pub(crate) fn for_spawn_agent(label: &str, message: &str) -> Self {
        Self::new(score_task(label, message, TaskComplexitySignals::default()))
    }

    pub(crate) fn from_complexity_score(complexity_score: f32) -> Self {
        Self::new(complexity_score)
    }

    fn new(complexity_score: f32) -> Self {
        Self {
            complexity_score: complexity_score.clamp(0.0, 100.0),
        }
    }

    pub(crate) fn complexity_score(self) -> f32 {
        self.complexity_score
    }

    pub(crate) fn tier(self) -> IntelligenceTier {
        IntelligenceTier::for_score(self.complexity_score)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ModelRoutingDecision {
    pub(crate) provider: String,
    pub(crate) model: String,
    pub(crate) task_complexity_score: f32,
    pub(crate) tier: IntelligenceTier,
    pub(crate) model_intelligence_score: f32,
    pub(crate) cost_per_1m_tokens: f32,
}

pub(crate) fn route_model_for_task(
    registry: &[AgentModelRegistryEntry],
    task_profile: SubagentTaskProfile,
) -> Option<ModelRoutingDecision> {
    let tier = task_profile.tier();

    registry
        .iter()
        .filter_map(|entry| {
            let model_intelligence_score = validated_score(entry.intelligence_score)?;
            if !tier.contains_model_score(model_intelligence_score) {
                return None;
            }

            let cost_per_1m_tokens = total_cost_per_1m_tokens(entry)?;
            Some(ModelCandidate {
                entry,
                model_intelligence_score,
                cost_per_1m_tokens,
            })
        })
        .min_by(|left, right| {
            left.cost_per_1m_tokens
                .total_cmp(&right.cost_per_1m_tokens)
                .then_with(|| {
                    left.model_intelligence_score
                        .total_cmp(&right.model_intelligence_score)
                })
                .then_with(|| left.entry.provider.0.cmp(&right.entry.provider.0))
                .then_with(|| left.entry.model.cmp(&right.entry.model))
        })
        .map(|candidate| ModelRoutingDecision {
            provider: candidate.entry.provider.0.clone(),
            model: candidate.entry.model.clone(),
            task_complexity_score: task_profile.complexity_score(),
            tier,
            model_intelligence_score: candidate.model_intelligence_score,
            cost_per_1m_tokens: candidate.cost_per_1m_tokens,
        })
}

struct ModelCandidate<'a> {
    entry: &'a AgentModelRegistryEntry,
    model_intelligence_score: f32,
    cost_per_1m_tokens: f32,
}

fn score_task(label: &str, message: &str, complexity_signals: TaskComplexitySignals) -> f32 {
    let task_text = format!("{label}\n{message}");

    if contains_tag(&task_text, "!heavy") {
        return 90.0;
    }

    if contains_tag(&task_text, "!boot") {
        return 20.0;
    }

    let base_score = intent_base_score(&task_text);
    let complexity_delta = complexity_delta(complexity_signals);

    (base_score + complexity_delta).min(100.0)
}

fn intent_base_score(task_text: &str) -> f32 {
    let task_text = task_text.to_ascii_lowercase();

    if contains_any(
        &task_text,
        &[
            "architecture",
            "architectural",
            "system design",
            "design decision",
        ],
    ) {
        90.0
    } else if contains_any(&task_text, &["algorithm", "data structure"]) {
        80.0
    } else if contains_any(
        &task_text,
        &[
            "bug",
            "fix",
            "debug",
            "crash",
            "panic",
            "error",
            "failing",
            "failure",
            "regression",
            "test fail",
            "review",
            "test",
        ],
    ) {
        50.0
    } else if contains_any(&task_text, &["refactor", "rewrite", "cleanup", "clean up"]) {
        30.0
    } else if contains_any(
        &task_text,
        &[
            "format",
            "formatting",
            "import",
            "imports",
            "rename",
            "boilerplate",
        ],
    ) {
        20.0
    } else {
        50.0
    }
}

fn complexity_delta(signals: TaskComplexitySignals) -> f32 {
    let cyclomatic_delta = match signals.cyclomatic_complexity {
        Some(complexity) if complexity <= 5 => 0.0,
        Some(complexity) if complexity <= 15 => 10.0,
        Some(_) => 20.0,
        None => 0.0,
    };

    let dependent_delta = signals
        .dependent_count
        .map(|dependents| (dependents / 50) as f32 * 10.0)
        .unwrap_or(0.0);

    cyclomatic_delta + dependent_delta
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn contains_tag(text: &str, tag: &str) -> bool {
    text.split(|character: char| {
        character.is_whitespace() || (character.is_ascii_punctuation() && character != '!')
    })
    .any(|token| token.eq_ignore_ascii_case(tag))
}

fn validated_score(score: f32) -> Option<f32> {
    if score.is_finite() && (0.0..=100.0).contains(&score) {
        Some(score)
    } else {
        None
    }
}

fn total_cost_per_1m_tokens(entry: &AgentModelRegistryEntry) -> Option<f32> {
    if entry.cost_per_1m_input_tokens.is_finite()
        && entry.cost_per_1m_output_tokens.is_finite()
        && entry.cost_per_1m_input_tokens >= 0.0
        && entry.cost_per_1m_output_tokens >= 0.0
    {
        Some(entry.cost_per_1m_input_tokens + entry.cost_per_1m_output_tokens)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings::LanguageModelProviderSetting;

    #[test]
    fn maps_scores_to_wide_tiers() {
        assert_eq!(IntelligenceTier::for_score(40.0), IntelligenceTier::Boot);
        assert_eq!(IntelligenceTier::for_score(41.0), IntelligenceTier::Mid);
        assert_eq!(IntelligenceTier::for_score(70.0), IntelligenceTier::Mid);
        assert_eq!(IntelligenceTier::for_score(71.0), IntelligenceTier::Heavy);
    }

    #[test]
    fn scores_tasks_from_intent_and_overrides() {
        assert_eq!(
            score_task(
                "Refactor auth",
                "Clean up this module",
                TaskComplexitySignals::default()
            ),
            30.0
        );
        assert_eq!(
            score_task(
                "Debug parser",
                "Fix failing tests",
                TaskComplexitySignals {
                    cyclomatic_complexity: Some(16),
                    dependent_count: Some(100),
                },
            ),
            90.0
        );
        assert_eq!(
            score_task(
                "Format imports",
                "Run the simple pass !heavy",
                TaskComplexitySignals::default()
            ),
            90.0
        );
        assert_eq!(
            score_task(
                "Architecture",
                "Draft it with !boot",
                TaskComplexitySignals::default()
            ),
            20.0
        );
    }

    #[test]
    fn routes_to_cheapest_model_in_task_tier() {
        let registry = vec![
            registry_entry("local", "expensive-boot", 30.0, 2.0, 2.0),
            registry_entry("local", "cheap-boot", 35.0, 0.2, 0.3),
            registry_entry("cloud", "mid", 65.0, 1.0, 2.0),
        ];

        let decision = route_model_for_task(&registry, SubagentTaskProfile::new(20.0)).unwrap();

        assert_eq!(decision.provider, "local");
        assert_eq!(decision.model, "cheap-boot");
        assert_eq!(decision.tier, IntelligenceTier::Boot);
        assert_eq!(decision.model_intelligence_score, 35.0);
    }

    #[test]
    fn ignores_invalid_registry_entries() {
        let registry = vec![
            registry_entry("local", "invalid-score", 120.0, 0.1, 0.1),
            registry_entry("local", "invalid-cost", 30.0, -1.0, 0.1),
            registry_entry("cloud", "mid", 65.0, 1.0, 2.0),
        ];

        assert!(route_model_for_task(&registry, SubagentTaskProfile::new(20.0)).is_none());
    }

    fn registry_entry(
        provider: &str,
        model: &str,
        intelligence_score: f32,
        input_cost: f32,
        output_cost: f32,
    ) -> AgentModelRegistryEntry {
        AgentModelRegistryEntry {
            provider: LanguageModelProviderSetting(provider.to_string()),
            model: model.to_string(),
            intelligence_score,
            cost_per_1m_input_tokens: input_cost,
            cost_per_1m_output_tokens: output_cost,
        }
    }
}
