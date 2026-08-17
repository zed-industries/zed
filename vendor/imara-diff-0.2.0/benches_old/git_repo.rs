use std::convert::Infallible;
use std::path::PathBuf;

use criterion::measurement::Measurement;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
};
use git_repository::object::tree::diff::{Action, Change};
use git_repository::Id;
use imara_diff::intern::InternedInput;
use imara_diff::sink::Counter;
use imara_diff::Algorithm;

fn extract_diff(change: &Change) -> Option<(Vec<u8>, Vec<u8>)> {
    use git_repository::object::tree::diff::change::Event::Modification;

    let (previous_id, id) = match change.event {
        Modification {
            previous_entry_mode,
            previous_id,
            entry_mode,
            id,
        } if previous_entry_mode.is_blob() && entry_mode.is_blob() => (previous_id, id),
        _ => return None,
    };

    let old = previous_id.object().ok()?.detach().data;
    let new = id.object().ok()?.detach().data;

    Some((new, old))
}

fn git_tree_diff(from: Id, to: Id, diffs: &mut Vec<(Vec<u8>, Vec<u8>, usize)>) {
    let from_tree = from.object().unwrap().peel_to_tree().unwrap();
    let to_tree = to.object().unwrap().peel_to_tree().unwrap();
    from_tree
        .changes()
        .track_filename()
        .for_each_to_obtain_tree(&to_tree, |change| -> Result<_, Infallible> {
            if let Some((old, new)) = extract_diff(&change) {
                let input = InternedInput::new(&*old, &*new);
                let changes =
                    imara_diff::diff(Algorithm::Myers, &input, Counter::default()).total();
                let complexity = changes * (old.len() + new.len());
                diffs.push((old, new, complexity));
            }
            Ok(Action::Continue)
        })
        .unwrap();
}

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let mut res = PathBuf::from(dir);
    while !res.join("README.md").exists() {
        res = res
            .parent()
            .expect("reached fs root without finding project root")
            .to_owned()
    }
    res
}

fn bench_repo(c: &mut Criterion, name: &str, tag2: &str, tag1: &str, num_commits: usize) {
    let path = project_root().join("bench_data").join("repos").join(name);
    let repo = git_repository::open(path).unwrap();
    let tag1 = repo
        .find_reference(tag1)
        .unwrap()
        .into_fully_peeled_id()
        .unwrap();
    let tag2 = repo
        .find_reference(tag2)
        .unwrap()
        .into_fully_peeled_id()
        .unwrap();
    let mut diffs = Vec::new();
    git_tree_diff(tag1, tag2, &mut diffs);
    let mut last_commit = tag2;
    tag2.object()
        .unwrap()
        .into_commit()
        .ancestors()
        .all()
        .unwrap()
        .take(num_commits as usize)
        .for_each(|parent| {
            let parent = parent.unwrap();
            git_tree_diff(last_commit, parent, &mut diffs);
            last_commit = parent;
        });
    diffs.sort_unstable_by_key(|(_, _, complexity)| *complexity);

    if std::env::var("PLOT").is_ok() {
        let mut group = c.benchmark_group(format!("{name}_plot"));
        group.sample_size(15);
        bench_file_diffs(group, &diffs, 12, true);
    } else {
        bench_file_diffs(c.benchmark_group(name), &diffs, 2, false);
    }
}

fn bench_file_diffs<M: Measurement>(
    mut group: BenchmarkGroup<M>,
    files: &[(Vec<u8>, Vec<u8>, usize)],
    num_chunks: usize,
    compare_to_similar: bool,
) {
    let mut run = |name, f: fn(&[u8], &[u8]) -> usize| {
        let mut i = 0;
        for chunk in files.chunks((files.len() + num_chunks - 1) / num_chunks) {
            let mut average_complexity: usize = chunk.iter().map(|(_, _, it)| *it).sum();
            average_complexity /= chunk.len();
            println!("benchmarking {i}..{}/{}", i + chunk.len(), files.len());
            i += chunk.len();
            group.bench_function(
                BenchmarkId::new(name, format!("{average_complexity}::{}", chunk.len())),
                |b| {
                    b.iter(|| {
                        for (old, new, _) in chunk {
                            // myers algorithm is O(ND) where D is the length of the (minimal) edit script and N the sum of file lengths
                            // we use that as an x axis to find a meaningful way to plot a
                            black_box(f(old, new));
                        }
                    });
                },
            );
        }
    };

    run("imara_diff-histogram", |file1, file2| {
        let input = InternedInput::new(file1, file2);
        imara_diff::diff(Algorithm::Histogram, &input, Counter::default()).total()
    });

    run("imara_diff-myers", |file1, file2| {
        let input = InternedInput::new(file1, file2);
        imara_diff::diff(Algorithm::Myers, &input, Counter::default()).total()
    });

    if compare_to_similar {
        run("similar", |file1, file2| {
            let diff = similar::utils::diff_lines(similar::Algorithm::Myers, file1, file2);
            diff.len()
        });
    }

    group.finish();
}

fn rust(c: &mut Criterion) {
    bench_repo(c, "rust", "1.64.0", "1.50.0", 30);
}

fn vscode(c: &mut Criterion) {
    bench_repo(c, "vscode", "1.72.2", "1.41.0", 30);
}

fn linux(c: &mut Criterion) {
    bench_repo(c, "linux", "v6.0", "v5.7", 30);
}

fn helix(c: &mut Criterion) {
    bench_repo(c, "helix", "22.08.1", "v0.5.0", 30);
}

criterion_group!(realworld_repos, helix, rust, vscode, linux);
criterion_main!(realworld_repos);
