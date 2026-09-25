use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use streaming_diff::StreamingDiff;

const SEED: u64 = 0x5EED_5EED;
const CHUNK_SIZE: usize = 512;

#[derive(Clone)]
struct EditFixture {
    name: &'static str,
    old_text: String,
    new_text: String,
}

fn streaming_diff_push_new(criterion: &mut Criterion) {
    let fixtures = fixtures();
    let mut group = criterion.benchmark_group("streaming_diff_push_new");
    group.sample_size(10);

    for fixture in fixtures {
        group.throughput(Throughput::Bytes(fixture.new_text.len() as u64));
        group.bench_with_input(
            BenchmarkId::new(fixture.name, fixture.old_text.len()),
            &fixture,
            |bench, fixture| {
                bench.iter_batched(
                    || StreamingDiff::new(fixture.old_text.clone()),
                    |mut diff| {
                        let mut operation_count = 0;
                        for chunk in chunk_text(&fixture.new_text, CHUNK_SIZE) {
                            operation_count += black_box(diff.push_new(chunk)).len();
                        }
                        black_box(operation_count);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn streaming_diff_finish(criterion: &mut Criterion) {
    let fixtures = fixtures();
    let mut group = criterion.benchmark_group("streaming_diff_finish");
    group.sample_size(10);

    for fixture in fixtures {
        group.throughput(Throughput::Bytes(fixture.new_text.len() as u64));
        group.bench_with_input(
            BenchmarkId::new(fixture.name, fixture.old_text.len()),
            &fixture,
            |bench, fixture| {
                bench.iter_batched(
                    || {
                        let mut diff = StreamingDiff::new(fixture.old_text.clone());
                        for chunk in chunk_text(&fixture.new_text, CHUNK_SIZE) {
                            black_box(diff.push_new(chunk));
                        }
                        diff
                    },
                    |diff| {
                        black_box(diff.finish());
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

fn fixtures() -> Vec<EditFixture> {
    // Keep fixtures modest because `StreamingDiff` is intentionally stressed here and
    // can become very slow on tens of kilobytes of replacement text. These sizes still
    // represent realistic `edit_file` old/new text blocks and are large enough to cross
    // frame-budget-sized CPU work.
    vec![
        make_fixture(
            "tiny_function_rewrite",
            2,
            EditPattern::LocalizedRewrite {
                start_line: 12,
                line_count: 6,
            },
            SEED,
        ),
        make_fixture(
            "small_function_rewrite",
            5,
            EditPattern::LocalizedRewrite {
                start_line: 22,
                line_count: 12,
            },
            SEED + 1,
        ),
        make_fixture(
            "medium_many_small_changes",
            8,
            EditPattern::ManySmallChanges { every_nth_line: 7 },
            SEED + 2,
        ),
        make_fixture(
            "medium_insertions",
            8,
            EditPattern::InsertHelperBlocks { every_nth_line: 9 },
            SEED + 3,
        ),
    ]
}

enum EditPattern {
    LocalizedRewrite {
        start_line: usize,
        line_count: usize,
    },
    ManySmallChanges {
        every_nth_line: usize,
    },
    InsertHelperBlocks {
        every_nth_line: usize,
    },
}

fn make_fixture(
    name: &'static str,
    function_count: usize,
    pattern: EditPattern,
    seed: u64,
) -> EditFixture {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut lines = random_rust_module(&mut rng, function_count);
    let old_text = lines.join("\n");

    match pattern {
        EditPattern::LocalizedRewrite {
            start_line,
            line_count,
        } => rewrite_local_block(&mut lines, start_line, line_count, &mut rng),
        EditPattern::ManySmallChanges { every_nth_line } => {
            rewrite_many_small_lines(&mut lines, every_nth_line, &mut rng)
        }
        EditPattern::InsertHelperBlocks { every_nth_line } => {
            insert_helper_blocks(&mut lines, every_nth_line, &mut rng)
        }
    }

    EditFixture {
        name,
        old_text,
        new_text: lines.join("\n"),
    }
}

fn random_rust_module(rng: &mut StdRng, function_count: usize) -> Vec<String> {
    let mut lines = vec![
        "use anyhow::{Context as _, Result};".to_string(),
        "use collections::HashMap;".to_string(),
        "".to_string(),
        "#[derive(Clone, Debug)]".to_string(),
        "pub struct WorkspaceSnapshot {".to_string(),
        "    buffers: HashMap<String, usize>,".to_string(),
        "    version: usize,".to_string(),
        "}".to_string(),
        "".to_string(),
        "impl WorkspaceSnapshot {".to_string(),
    ];

    for function_index in 0..function_count {
        let function_name = identifier(rng, function_index);
        let argument_name = identifier(rng, function_index + 1_000);
        let local_name = identifier(rng, function_index + 2_000);
        let branch_name = identifier(rng, function_index + 3_000);
        let multiplier = rng.random_range(2..17);
        let offset = rng.random_range(1..128);

        lines.extend([
            format!(
                "    pub fn {function_name}(&mut self, {argument_name}: usize) -> Result<usize> {{"
            ),
            format!("        let mut {local_name} = {argument_name}.saturating_mul({multiplier});"),
            format!("        if {local_name} % 2 == 0 {{"),
            format!(
                "            {local_name} = {local_name}.saturating_add(self.version + {offset});"
            ),
            "        } else {".to_string(),
            format!("            {local_name} = {local_name}.saturating_sub({offset});"),
            "        }".to_string(),
            format!("        let {branch_name} = self.buffers.len().saturating_add({local_name});"),
            format!("        self.version = self.version.saturating_add({branch_name});"),
            format!("        Ok({branch_name})"),
            "    }".to_string(),
            "".to_string(),
        ]);
    }

    lines.push("}".to_string());
    lines.push("".to_string());
    lines.push("pub fn normalize_path(path: &str) -> String {".to_string());
    lines.push("    path.replace('\\\\', \"/\")".to_string());
    lines.push("}".to_string());
    lines
}

fn rewrite_local_block(
    lines: &mut [String],
    start_line: usize,
    line_count: usize,
    rng: &mut StdRng,
) {
    let end_line = (start_line + line_count).min(lines.len());
    for (relative_index, line) in lines[start_line..end_line].iter_mut().enumerate() {
        let suffix = identifier(rng, relative_index + 10_000);
        if line.contains("saturating_add") {
            *line = format!(
                "        let {suffix} = self.version.checked_add({relative_index}).context(\"version overflow\")?;"
            );
        } else if line.contains("saturating_sub") {
            *line = format!(
                "            {suffix}.saturating_sub({});",
                rng.random_range(8..256)
            );
        } else if line.trim().is_empty() {
            *line = format!(
                "        tracing::trace!(target: \"agent_bench\", value = {relative_index});"
            );
        } else {
            *line = format!("{line} // updated {suffix}");
        }
    }
}

fn rewrite_many_small_lines(lines: &mut [String], every_nth_line: usize, rng: &mut StdRng) {
    for (line_index, line) in lines.iter_mut().enumerate() {
        if line_index % every_nth_line != 0 || line.trim().is_empty() {
            continue;
        }

        if line.contains("let mut") {
            *line = line.replace("let mut", "let mut updated");
        } else if line.contains("Ok(") {
            *line = line.replace("Ok(", "Ok(black_box_value(");
        } else if line.ends_with('{') {
            *line = format!("{line} // scenario {}", identifier(rng, line_index));
        } else {
            *line = format!("{line} // touched {}", identifier(rng, line_index));
        }
    }
}

fn insert_helper_blocks(lines: &mut Vec<String>, every_nth_line: usize, rng: &mut StdRng) {
    let mut line_index = every_nth_line;
    while line_index < lines.len() {
        if lines[line_index].trim() == "}" {
            let helper_name = identifier(rng, line_index + 20_000);
            lines.splice(
                line_index..line_index,
                [
                    format!("        let {helper_name} = self.buffers.len();"),
                    format!("        tracing::trace!(target: \"agent_bench\", {helper_name});"),
                ],
            );
            line_index += 2;
        }
        line_index += every_nth_line;
    }
}

fn identifier(rng: &mut StdRng, salt: usize) -> String {
    const WORDS: &[&str] = &[
        "buffer",
        "workspace",
        "snapshot",
        "version",
        "project",
        "entry",
        "path",
        "cursor",
        "anchor",
        "edit",
        "thread",
        "message",
        "context",
        "store",
        "diff",
        "range",
        "token",
        "parser",
        "semantic",
        "format",
        "completion",
        "diagnostic",
        "terminal",
        "channel",
    ];

    let first = WORDS[(rng.random_range(0..WORDS.len()) + salt) % WORDS.len()];
    let second = WORDS[(rng.random_range(0..WORDS.len()) + salt / 3) % WORDS.len()];
    format!("{first}_{second}_{salt}")
}

fn chunk_text(text: &str, max_chunk_size: usize) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let mut end = (start + max_chunk_size).min(text.len());
        while end < text.len() && !text.is_char_boundary(end) {
            end += 1;
        }
        chunks.push(&text[start..end]);
        start = end;
    }
    chunks
}

criterion_group!(benches, streaming_diff_push_new, streaming_diff_finish);
criterion_main!(benches);
