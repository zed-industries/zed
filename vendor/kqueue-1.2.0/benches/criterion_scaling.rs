use std::os::unix::io::AsRawFd;
use std::{fs::File, path::PathBuf};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kqueue::Watcher;
use tempfile::{tempdir, TempDir};

const BENCH_CASES: [usize; 3] = [1, 50, 5000];

fn create_files() -> (TempDir, Vec<PathBuf>) {
    let tmp = tempdir().unwrap();
    let mut fils = Vec::new();

    for i in 0..*BENCH_CASES.last().unwrap() {
        let fil = tmp.path().join(format!("temp{i}"));
        fils.push(fil.clone());
        File::create(fil).unwrap();
    }

    (tmp, fils)
}

fn bench_add_filename(c: &mut Criterion) {
    let (tmp, fils) = create_files();

    let mut group = c.benchmark_group("add");
    for nofiles in BENCH_CASES {
        group.throughput(criterion::Throughput::Elements(nofiles as u64));
        group.bench_with_input(BenchmarkId::new("filename", nofiles), &fils, |b, fils| {
            b.iter_batched_ref(
                || {
                    (
                        Watcher::new().unwrap(),
                        fils.iter().take(nofiles).collect::<Vec<_>>(),
                    )
                },
                |(w, fils)| {
                    for fil in fils {
                        w.add_filename(
                            fil,
                            kqueue::EventFilter::EVFILT_VNODE,
                            kqueue::FilterFlag::empty(),
                        )
                        .unwrap();
                    }
                },
                criterion::BatchSize::PerIteration,
            )
        });
    }

    group.finish();
    drop(tmp);
}

fn bench_del_filename(c: &mut Criterion) {
    let (tmp, fils) = create_files();

    let mut group = c.benchmark_group("del");
    for nofiles in BENCH_CASES {
        group.throughput(criterion::Throughput::Elements(nofiles as u64));
        group.bench_with_input(BenchmarkId::new("filename", nofiles), &fils, |b, fils| {
            b.iter_batched(
                || {
                    let mut w = Watcher::new().unwrap();
                    for fil in fils.iter().take(nofiles) {
                        w.add_filename(
                            fil,
                            kqueue::EventFilter::EVFILT_VNODE,
                            kqueue::FilterFlag::empty(),
                        )
                        .unwrap();
                    }
                    w.watch().unwrap();
                    (w, fils.iter().take(nofiles).rev().collect::<Vec<_>>())
                },
                |(mut w, fils)| {
                    for fil in fils {
                        w.remove_filename(fil, kqueue::EventFilter::EVFILT_VNODE)
                            .unwrap();
                    }
                },
                criterion::BatchSize::PerIteration,
            )
        });
    }

    group.finish();
    drop(tmp);
}

fn bench_watch_filename(c: &mut Criterion) {
    let (tmp, fils) = create_files();

    let mut group = c.benchmark_group("watch");
    for nofiles in BENCH_CASES {
        group.throughput(criterion::Throughput::Elements(nofiles as u64));
        group.bench_with_input(BenchmarkId::new("filename", nofiles), &fils, |b, fils| {
            b.iter_batched_ref(
                || {
                    let mut w = Watcher::new().unwrap();
                    for fil in fils.iter().take(nofiles) {
                        w.add_filename(
                            fil,
                            kqueue::EventFilter::EVFILT_VNODE,
                            kqueue::FilterFlag::empty(),
                        )
                        .unwrap();
                    }
                    w
                },
                |w| {
                    w.watch().unwrap();
                },
                criterion::BatchSize::PerIteration,
            )
        });
    }

    group.finish();
    drop(tmp);
}

fn bench_add_fd(c: &mut Criterion) {
    let (tmp, fils) = create_files();

    let mut group = c.benchmark_group("add");
    for nofiles in BENCH_CASES {
        group.throughput(criterion::Throughput::Elements(nofiles as u64));
        group.bench_with_input(BenchmarkId::new("fd", nofiles), &fils, |b, fils| {
            b.iter_batched_ref(
                || {
                    (
                        Watcher::new().unwrap(),
                        fils.iter()
                            .take(nofiles)
                            .map(|nam| File::open(nam).unwrap())
                            .collect::<Vec<_>>(),
                    )
                },
                |(w, fils)| {
                    for fil in fils {
                        w.add_fd(
                            fil.as_raw_fd(),
                            kqueue::EventFilter::EVFILT_VNODE,
                            kqueue::FilterFlag::empty(),
                        )
                        .unwrap();
                    }
                },
                criterion::BatchSize::PerIteration,
            )
        });
    }

    group.finish();
    drop(tmp);
}

fn bench_del_fd(c: &mut Criterion) {
    let (tmp, fils) = create_files();

    let mut group = c.benchmark_group("del");
    for nofiles in BENCH_CASES {
        group.throughput(criterion::Throughput::Elements(nofiles as u64));
        group.bench_with_input(BenchmarkId::new("fd", nofiles), &fils, |b, fils| {
            b.iter_batched_ref(
                || {
                    let mut w = Watcher::new().unwrap();
                    let fils = fils
                        .iter()
                        .take(nofiles)
                        .rev()
                        .map(|nam| File::open(nam).unwrap())
                        .collect::<Vec<_>>();
                    for fil in &fils {
                        w.add_fd(
                            fil.as_raw_fd(),
                            kqueue::EventFilter::EVFILT_VNODE,
                            kqueue::FilterFlag::empty(),
                        )
                        .unwrap();
                    }
                    w.watch().unwrap();
                    (w, fils)
                },
                |(w, fils)| {
                    for fil in fils {
                        w.remove_fd(fil.as_raw_fd(), kqueue::EventFilter::EVFILT_VNODE)
                            .unwrap();
                    }
                },
                criterion::BatchSize::PerIteration,
            )
        });
    }

    group.finish();
    drop(tmp);
}

criterion_group!(
    lots,
    bench_add_filename,
    bench_del_filename,
    bench_watch_filename,
    bench_add_fd,
    bench_del_fd
);
criterion_main!(lots);
