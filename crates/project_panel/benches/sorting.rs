use criterion::{Criterion, criterion_group, criterion_main};
use project::{Entry, EntryKind, GitEntry, ProjectEntryId};
use project_panel::par_sort_worktree_entries_with_mode;
use settings::ProjectPanelSortMode;
use std::sync::Arc;
use util::rel_path::RelPath;

fn load_linux_repo_snapshot() -> Vec<GitEntry> {
    let file = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benches/linux_repo_snapshot.txt"
    ))
    .expect("Failed to read file");
    file.lines()
        .filter_map(|line| {
            let kind = match line.chars().next() {
                Some('f') => EntryKind::File,
                Some('d') => EntryKind::Dir,
                _ => return None,
            };

            let entry = Entry {
                kind,
                path: Arc::from(RelPath::unix(&(line.trim_end()[2..])).unwrap()),
                id: ProjectEntryId::default(),
                size: 0,
                inode: 0,
                mtime: None,
                canonical_path: None,
                is_ignored: false,
                is_always_included: false,
                is_external: false,
                is_private: false,
                is_hidden: false,
                char_bag: Default::default(),
                is_fifo: false,
            };
            Some(GitEntry {
                entry,
                git_summary: Default::default(),
            })
        })
        .collect()
}
fn criterion_benchmark(c: &mut Criterion) {
    let snapshot = load_linux_repo_snapshot();

    c.bench_function("Sort linux worktree snapshot", |b| {
        b.iter_batched(
            || snapshot.clone(),
            |mut snapshot| {
                par_sort_worktree_entries_with_mode(
                    &mut snapshot,
                    ProjectPanelSortMode::DirectoriesFirst,
                )
            },
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("Sort linux worktree snapshot (Mixed)", |b| {
        b.iter_batched(
            || snapshot.clone(),
            |mut snapshot| {
                par_sort_worktree_entries_with_mode(&mut snapshot, ProjectPanelSortMode::Mixed)
            },
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("Sort linux worktree snapshot (FilesFirst)", |b| {
        b.iter_batched(
            || snapshot.clone(),
            |mut snapshot| {
                par_sort_worktree_entries_with_mode(&mut snapshot, ProjectPanelSortMode::FilesFirst)
            },
            criterion::BatchSize::LargeInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
