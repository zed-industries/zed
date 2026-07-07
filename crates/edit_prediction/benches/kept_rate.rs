use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use edit_prediction::metrics::compute_kept_rate;

fn repeated_function_lines(line_count: usize) -> String {
    let mut text = String::with_capacity(line_count * 32);
    for index in 0..line_count {
        text.push_str("fn helper_");
        text.push_str(&(index % 16).to_string());
        text.push_str("() { value += old_name + 1; }\n");
    }
    text
}

fn localized_rename_inputs(line_count: usize) -> (String, String, String) {
    let base = repeated_function_lines(line_count);
    let mut predicted = base.clone();
    let mut final_text = base.clone();

    let needle = "value += old_name + 1;";
    let prediction = "value += very_long_predicted_name + 1;";
    let accepted = "value += new_name + 1;";

    let offset = base
        .rfind(needle)
        .expect("expected needle in synthetic input");
    let end = offset + needle.len();

    predicted.replace_range(offset..end, prediction);
    final_text.replace_range(offset..end, accepted);

    (base, predicted, final_text)
}

fn identical_new_content_inputs(line_count: usize) -> (String, String, String) {
    let predicted = repeated_function_lines(line_count);
    (String::new(), predicted.clone(), predicted)
}

fn repetitive_token_inputs(token_repetitions: usize) -> (String, String, String) {
    let repeated_old = "foo + foo + foo + foo + foo\n".repeat(token_repetitions);
    let repeated_predicted = "foo + foo + prediction_token + foo + foo\n".repeat(token_repetitions);
    let repeated_final = "foo + foo + kept_token + foo + foo\n".repeat(token_repetitions);
    (repeated_old, repeated_predicted, repeated_final)
}

fn kept_rate_benchmark(c: &mut Criterion) {
    let mut no_change_group = c.benchmark_group("kept_rate/no_change");
    for line_count in [128usize, 512, 2048] {
        let text = repeated_function_lines(line_count);
        no_change_group.bench_with_input(
            BenchmarkId::new("lines", line_count),
            &text,
            |bench, text| {
                bench.iter(|| {
                    black_box(compute_kept_rate(
                        black_box(text),
                        black_box(text),
                        black_box(text),
                    ));
                });
            },
        );
    }
    no_change_group.finish();

    let mut localized_group = c.benchmark_group("kept_rate/localized_rename");
    for line_count in [128usize, 512, 2048] {
        let inputs = localized_rename_inputs(line_count);
        localized_group.bench_with_input(
            BenchmarkId::new("lines", line_count),
            &inputs,
            |bench, inputs| {
                let (base, predicted, final_text) = inputs;
                bench.iter(|| {
                    black_box(compute_kept_rate(
                        black_box(base),
                        black_box(predicted),
                        black_box(final_text),
                    ));
                });
            },
        );
    }
    localized_group.finish();

    let mut addition_group = c.benchmark_group("kept_rate/identical_addition");
    for line_count in [128usize, 512, 2048] {
        let inputs = identical_new_content_inputs(line_count);
        addition_group.bench_with_input(
            BenchmarkId::new("lines", line_count),
            &inputs,
            |bench, inputs| {
                let (base, predicted, final_text) = inputs;
                bench.iter(|| {
                    black_box(compute_kept_rate(
                        black_box(base),
                        black_box(predicted),
                        black_box(final_text),
                    ));
                });
            },
        );
    }
    addition_group.finish();

    let mut repetitive_group = c.benchmark_group("kept_rate/repetitive_tokens");
    for token_repetitions in [64usize, 256, 1024] {
        let inputs = repetitive_token_inputs(token_repetitions);
        repetitive_group.bench_with_input(
            BenchmarkId::new("repetitions", token_repetitions),
            &inputs,
            |bench, inputs| {
                let (base, predicted, final_text) = inputs;
                bench.iter(|| {
                    black_box(compute_kept_rate(
                        black_box(base),
                        black_box(predicted),
                        black_box(final_text),
                    ));
                });
            },
        );
    }
    repetitive_group.finish();
}

criterion_group!(benches, kept_rate_benchmark);
criterion_main!(benches);
