use std::sync::Arc;

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use edit_prediction::metrics::count_tree_sitter_errors;
use fs::FakeFs;
use gpui::{AppContext as _, TestAppContext};
use language::{Buffer, BufferSnapshot, LanguageRegistry};
use languages::init as init_languages;
use node_runtime::NodeRuntime;
use settings::SettingsStore;

struct ParsedCase {
    label: String,
    bytes: usize,
    error_count: usize,
    snapshot: BufferSnapshot,
}

fn replace_nth_occurrences(
    source: &mut String,
    needle: &str,
    replacement: &str,
    every: usize,
    max_replacements: usize,
) {
    let mut rebuilt = String::with_capacity(source.len());
    let mut cursor = 0;
    let mut seen = 0;
    let mut replaced = 0;

    while let Some(relative_index) = source[cursor..].find(needle) {
        let start = cursor + relative_index;
        let end = start + needle.len();
        rebuilt.push_str(&source[cursor..start]);

        if seen % every == 0 && replaced < max_replacements {
            rebuilt.push_str(replacement);
            replaced += 1;
        } else {
            rebuilt.push_str(needle);
        }

        seen += 1;
        cursor = end;
    }

    rebuilt.push_str(&source[cursor..]);
    *source = rebuilt;
}

fn rust_source(function_count: usize) -> String {
    let mut source = String::from(
        "pub struct Counter {\n    value: usize,\n}\n\nimpl Counter {\n    pub fn new() -> Self {\n        Self { value: 0 }\n    }\n}\n\n",
    );
    for index in 0..function_count {
        source.push_str(&format!(
            "pub fn compute_value_{index}(input: usize) -> usize {{\n    let mut total = input;\n    for offset in 0..32 {{\n        total += offset + {index};\n    }}\n    if total % 2 == 0 {{\n        total / 2\n    }} else {{\n        total * 3 + 1\n    }}\n}}\n\n"
        ));
    }
    source
}

fn rust_source_with_errors(function_count: usize) -> String {
    let mut source = rust_source(function_count);
    replace_nth_occurrences(
        &mut source,
        "    if total % 2 == 0 {\n",
        "    if total % 2 == 0 \n",
        17,
        48,
    );
    source
}

fn python_source(function_count: usize) -> String {
    let mut source = String::from(
        "class Counter:\n    def __init__(self) -> None:\n        self.value = 0\n\n\n",
    );
    for index in 0..function_count {
        source.push_str(&format!(
            "def compute_value_{index}(input_value: int) -> int:\n    total = input_value\n    for offset in range(32):\n        total += offset + {index}\n    if total % 2 == 0:\n        return total // 2\n    return total * 3 + 1\n\n"
        ));
    }
    source
}

fn python_source_with_errors(function_count: usize) -> String {
    let mut source = python_source(function_count);
    replace_nth_occurrences(
        &mut source,
        "    if total % 2 == 0:\n",
        "    if total % 2 == 0\n",
        19,
        48,
    );
    source
}

fn go_source(function_count: usize) -> String {
    let mut source = String::from(
        "package bench\n\ntype Counter struct {\n\tvalue int\n}\n\nfunc NewCounter() Counter {\n\treturn Counter{value: 0}\n}\n\n",
    );
    for index in 0..function_count {
        source.push_str(&format!(
            "func ComputeValue{index}(inputValue int) int {{\n\ttotal := inputValue\n\tfor offset := 0; offset < 32; offset++ {{\n\t\ttotal += offset + {index}\n\t}}\n\tif total%2 == 0 {{\n\t\treturn total / 2\n\t}}\n\treturn total*3 + 1\n}}\n\n"
        ));
    }
    source
}

fn go_source_with_errors(function_count: usize) -> String {
    let mut source = go_source(function_count);
    replace_nth_occurrences(
        &mut source,
        "\tfor offset := 0; offset < 32; offset++ {\n",
        "\tfor offset := 0; offset < 32; offset++ \n",
        17,
        48,
    );
    source
}

fn typescript_source(function_count: usize) -> String {
    let mut source = String::from(
        "export type Counter = { value: number };\n\nexport function newCounter(): Counter {\n  return { value: 0 };\n}\n\n",
    );
    for index in 0..function_count {
        source.push_str(&format!(
            "export function computeValue{index}(inputValue: number): number {{\n  let total = inputValue;\n  for (let offset = 0; offset < 32; offset += 1) {{\n    total += offset + {index};\n  }}\n  return total % 2 === 0 ? total / 2 : total * 3 + 1;\n}}\n\n"
        ));
    }
    source
}

fn typescript_source_with_errors(function_count: usize) -> String {
    let mut source = typescript_source(function_count);
    replace_nth_occurrences(
        &mut source,
        "  return total % 2 === 0 ? total / 2 : total * 3 + 1;\n",
        "  return total % 2 === 0 ? total / 2 : ;\n",
        17,
        64,
    );
    source
}

fn tsx_source(component_count: usize) -> String {
    let mut source = String::from(
        "type ItemProps = { index: number; label: string };\n\nfunction Item({ index, label }: ItemProps) {\n  return <li data-index={index}>{label}</li>;\n}\n\nexport function App() {\n  return <section><ul>{[0, 1, 2].map((value) => <Item key={value} index={value} label={`item-${value}`} />)}</ul></section>;\n}\n\n",
    );
    for index in 0..component_count {
        source.push_str(&format!(
            "export function Widget{index}(): JSX.Element {{\n  const items = Array.from({{ length: 16 }}, (_, value) => value + {index});\n  return (\n    <div className=\"widget-{index}\">\n      <h2>Widget {index}</h2>\n      <ul>\n        {{items.map((value) => (\n          <Item key={{value}} index={{value}} label={{`widget-{index}-${{value}}`}} />\n        ))}}\n      </ul>\n    </div>\n  );\n}}\n\n"
        ));
    }
    source
}

fn tsx_source_with_errors(component_count: usize) -> String {
    let mut source = tsx_source(component_count);
    replace_nth_occurrences(
        &mut source,
        "  const items = Array.from({ length: 16 }, (_, value) => value + ",
        "  const items = Array.from({ length: 16 }, (_, value) => ); // ",
        11,
        32,
    );
    source
}

fn json_source(object_count: usize) -> String {
    let mut source = String::from("{\n  \"items\": [\n");
    for index in 0..object_count {
        let suffix = if index + 1 == object_count { "" } else { "," };
        source.push_str(&format!(
            "    {{\n      \"id\": {index},\n      \"name\": \"item-{index}\",\n      \"enabled\": true,\n      \"tags\": [\"alpha\", \"beta\", \"gamma\"],\n      \"metrics\": {{ \"count\": {}, \"ratio\": {} }}\n    }}{suffix}\n",
            index * 3 + 1,
            index as f64 / 10.0,
        ));
    }
    source.push_str("  ]\n}\n");
    source
}

fn json_source_with_errors(object_count: usize) -> String {
    let mut source = json_source(object_count);
    replace_nth_occurrences(
        &mut source,
        "      \"enabled\": true,\n",
        "      \"enabled\": ,\n",
        23,
        64,
    );
    source
}

fn yaml_source(document_count: usize) -> String {
    let mut source = String::new();
    for index in 0..document_count {
        source.push_str(&format!(
            "- id: {index}\n  name: item-{index}\n  enabled: true\n  tags:\n    - alpha\n    - beta\n    - gamma\n  metrics:\n    count: {}\n    ratio: {}\n",
            index * 3 + 1,
            index as f64 / 10.0,
        ));
    }
    source
}

fn yaml_source_with_errors(document_count: usize) -> String {
    let mut source = yaml_source(document_count);
    replace_nth_occurrences(&mut source, "    count: ", "    count ", 23, 64);
    source
}

fn css_source(rule_count: usize) -> String {
    let mut source = String::new();
    for index in 0..rule_count {
        source.push_str(&format!(
            ".widget-{index} {{\n  display: grid;\n  grid-template-columns: repeat(4, minmax(0, 1fr));\n  gap: 12px;\n  padding: 8px;\n  color: rgb({}, {}, {});\n}}\n\n.widget-{index} > .item-{index} {{\n  border: 1px solid rgba(0, 0, 0, 0.15);\n  background: linear-gradient(90deg, #fff, #eef);\n}}\n\n",
            (index * 17) % 255,
            (index * 31) % 255,
            (index * 47) % 255,
        ));
    }
    source
}

fn css_source_with_errors(rule_count: usize) -> String {
    let mut source = css_source(rule_count);
    replace_nth_occurrences(&mut source, "  gap: 12px;\n", "  gap 12px;\n", 29, 64);
    source
}

fn build_case(
    context: &mut TestAppContext,
    languages: &Arc<LanguageRegistry>,
    language_name: &'static str,
    variant_name: &'static str,
    source: String,
    expect_errors: bool,
) -> ParsedCase {
    let language_task = context.background_spawn({
        let languages = languages.clone();
        async move { languages.language_for_name(language_name).await }
    });
    while !language_task.is_ready() {
        context.run_until_parked();
    }
    let language = futures::executor::block_on(language_task)
        .unwrap_or_else(|error| panic!("failed to load {language_name}: {error}"));

    let buffer = context.new(|cx| Buffer::local(source, cx).with_language(language, cx));
    context.run_until_parked();
    while buffer.read_with(context, |buffer, _| buffer.is_parsing()) {
        context.run_until_parked();
    }

    let snapshot = buffer.read_with(context, |buffer, _| buffer.snapshot());
    let full_range = 0..snapshot.text.len();
    let error_count = count_tree_sitter_errors(snapshot.syntax_layers());
    if expect_errors {
        assert!(
            error_count > 0,
            "expected tree-sitter errors for {language_name}/{variant_name}",
        );
    } else {
        assert_eq!(
            error_count, 0,
            "expected no tree-sitter errors for {language_name}/{variant_name}",
        );
    }

    let label = format!(
        "{}/{}_{}kb_{}e",
        language_name.to_lowercase(),
        variant_name,
        full_range.end / 1024,
        error_count,
    );
    ParsedCase {
        label,
        bytes: full_range.end,
        error_count,
        snapshot,
    }
}

fn parsed_cases() -> Vec<ParsedCase> {
    let mut context = TestAppContext::single();
    context.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });

    let languages = Arc::new(LanguageRegistry::new(context.executor()));
    let fs = FakeFs::new(context.executor());
    let node_runtime = NodeRuntime::unavailable();
    context.update(|cx| init_languages(languages.clone(), fs, node_runtime, cx));

    vec![
        build_case(
            &mut context,
            &languages,
            "Rust",
            "valid",
            rust_source(900),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "Rust",
            "error_heavy",
            rust_source_with_errors(900),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "Python",
            "valid",
            python_source(1100),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "Python",
            "error_heavy",
            python_source_with_errors(1100),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "Go",
            "valid",
            go_source(1000),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "Go",
            "error_heavy",
            go_source_with_errors(1000),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "TypeScript",
            "valid",
            typescript_source(1000),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "TypeScript",
            "error_heavy",
            typescript_source_with_errors(1000),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "TSX",
            "valid",
            tsx_source(350),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "TSX",
            "error_heavy",
            tsx_source_with_errors(350),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "JSON",
            "valid",
            json_source(2200),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "JSON",
            "error_heavy",
            json_source_with_errors(2200),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "YAML",
            "valid",
            yaml_source(2200),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "YAML",
            "error_heavy",
            yaml_source_with_errors(2200),
            true,
        ),
        build_case(
            &mut context,
            &languages,
            "CSS",
            "valid",
            css_source(2400),
            false,
        ),
        build_case(
            &mut context,
            &languages,
            "CSS",
            "error_heavy",
            css_source_with_errors(2400),
            true,
        ),
    ]
}

fn ts_error_count_benchmark(c: &mut Criterion) {
    let cases = parsed_cases();
    let mut group = c.benchmark_group("ts_error_count/full_file");

    for case in &cases {
        group.bench_with_input(
            BenchmarkId::from_parameter(&case.label),
            case,
            |bench, case| {
                bench.iter(|| {
                    black_box(case.bytes);
                    black_box(case.error_count);
                    black_box(count_tree_sitter_errors(case.snapshot.syntax_layers()))
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, ts_error_count_benchmark);
criterion_main!(benches);
