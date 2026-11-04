use std::{cmp::Reverse, fmt, sync::Arc, time::Duration};

use collections::HashSet;
use fuzzy_nucleo::{Case, LengthPenalty, StringMatch, StringMatchCandidate};
use gpui::{BenchAppContext, SharedString};
use ordered_float::OrderedFloat;

// Compile the production search without pulling in the rest of agent_ui.
#[path = "../../agent_ui/src/model_search.rs"]
mod model_search;

#[derive(Clone)]
struct OptionValue {
    value: SharedString,
    name: String,
    description: Option<String>,
    group: Option<String>,
}

struct SearchInput {
    options: usize,
    description_bytes: usize,
    query_name: &'static str,
    query: &'static str,
}

impl fmt::Display for SearchInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}-options-{}-byte-descriptions-{}",
            self.options, self.description_bytes, self.query_name,
        )
    }
}

fn inputs() -> Vec<SearchInput> {
    [(100, 128), (1_000, 128), (10_000, 128), (10_000, 1_024)]
        .into_iter()
        .flat_map(|(options, description_bytes)| {
            [
                ("broad", "gpt"),
                ("selective", "GPT 5.6 000042"),
                ("combined", "openai reasoning"),
                ("missing", "nonexistent-zzzzzz"),
                ("name-ranking", "high"),
            ]
            .into_iter()
            .map(move |(query_name, query)| SearchInput {
                options,
                description_bytes,
                query_name,
                query,
            })
        })
        .collect()
}

fn options(input: &SearchInput) -> Vec<OptionValue> {
    let description = "Flagship reasoning model for coding, analysis, and complex tasks. ";
    let mut description = description.repeat(input.description_bytes.div_ceil(description.len()));
    description.truncate(input.description_bytes);

    (0..input.options)
        .map(|index| {
            let (name, group, identifier) = match index % 4 {
                0 => ("Claude Sonnet 4", "Anthropic", "anthropic/claude-sonnet-4"),
                1 => ("Gemini 2.5 Pro", "Google", "google/gemini-2.5-pro"),
                2 => ("GPT-5.6 Sol", "OpenAI", "openai/gpt-5.6-sol"),
                _ => ("DeepSeek V3", "OpenRouter", "openrouter/deepseek-v3"),
            };
            let mut option = OptionValue {
                value: format!("{identifier}/{index:06}").into(),
                name: format!("{name} {index:06}"),
                description: Some(description.clone()),
                group: Some(group.to_string()),
            };
            match index {
                0 => {
                    option.name = "High".to_string();
                    option.description =
                        Some("High effort with advanced reasoning for complex tasks.".to_string());
                }
                1 => {
                    option.name = "Extra High".to_string();
                    option.description = None;
                }
                2 => {
                    option.name = "Reasoner".to_string();
                    option.description = Some("High reasoning effort".to_string());
                }
                _ => {}
            }
            option
        })
        .collect()
}

fn candidates(index: usize, option: &OptionValue) -> (StringMatchCandidate, StringMatchCandidate) {
    (
        StringMatchCandidate::new(index, option.name.clone()),
        StringMatchCandidate::new(
            index,
            format!(
                "{} {} {} {}",
                option.name,
                option.group.as_deref().unwrap_or_default(),
                option.description.as_deref().unwrap_or_default(),
                option.value,
            ),
        ),
    )
}

fn expected_values(options: &[OptionValue], query: &str) -> Vec<SharedString> {
    let (names, metadata): (Vec<_>, Vec<_>) = options
        .iter()
        .enumerate()
        .map(|(index, option)| candidates(index, option))
        .unzip();
    let mut names = fuzzy_nucleo::match_strings(
        &names,
        query,
        Case::Ignore,
        LengthPenalty::On,
        options.len(),
    );
    let mut metadata = fuzzy_nucleo::match_strings(
        &metadata,
        query,
        Case::Ignore,
        LengthPenalty::On,
        options.len(),
    );
    let sorting_key =
        |matched: &StringMatch| (Reverse(OrderedFloat(matched.score)), matched.candidate_id);
    names.sort_unstable_by_key(sorting_key);
    metadata.sort_unstable_by_key(sorting_key);
    let name_ids: HashSet<_> = names.iter().map(|matched| matched.candidate_id).collect();
    names
        .into_iter()
        .chain(
            metadata
                .into_iter()
                .filter(|matched| !name_ids.contains(&matched.candidate_id)),
        )
        .map(|matched| options[matched.candidate_id].value.clone())
        .collect()
}

struct ValidatedResults {
    results: Vec<OptionValue>,
    expected: Arc<Vec<SharedString>>,
}

impl Drop for ValidatedResults {
    fn drop(&mut self) {
        // bench_task drops its output after timing, so validation doesn't inflate latency.
        assert!(
            self.results
                .iter()
                .map(|option| &option.value)
                .eq(self.expected.iter()),
            "search dropped, duplicated, or reordered options",
        );
    }
}

#[gpui::bench(
    inputs = inputs(),
    input_name = "case",
    group = "agent_model_search",
    fps = 120,
    sample_size = 15
)]
fn model_search(input: &SearchInput, cx: &mut BenchAppContext) {
    let options = Arc::new(options(input));
    let expected = Arc::new(expected_values(&options, input.query));
    match input.query_name {
        "missing" => assert!(expected.is_empty()),
        "name-ranking" => {
            assert_eq!(
                expected.first(),
                options.first().map(|option| &option.value)
            );
            assert_eq!(expected.get(1), options.get(1).map(|option| &option.value));
        }
        _ => assert!(!expected.is_empty()),
    }
    let query = input.query;
    cx.bench_task(|cx| {
        let options = options.clone();
        let expected = expected.clone();
        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            let results = model_search::search(options.to_vec(), query, executor, candidates).await;
            ValidatedResults { results, expected }
        })
    });
}

gpui::bench_group! {
    name = benches;
    config = criterion::Criterion::default()
        .warm_up_time(Duration::from_millis(200))
        .measurement_time(Duration::from_millis(500));
    targets = model_search
}
gpui::bench_main!(benches);
