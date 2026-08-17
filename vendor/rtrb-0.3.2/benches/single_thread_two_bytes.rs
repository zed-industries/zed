//! Single-threaded benchmarks, writing and reading two bytes using a three-element queue.
//!
//! This is *not* a typical use case but it should nevertheless be useful
//! for comparing the overhead of different methods.
//! Writing two elements to a three-element queue makes sure
//! that there is a ring buffer wrap-around every second time.

use std::io::{Read, Write};

use criterion::{black_box, criterion_group, criterion_main};
use criterion::{AxisScale, PlotConfiguration};

use rtrb::{CopyToUninit, RingBuffer};

fn add_function<F, M>(group: &mut criterion::BenchmarkGroup<M>, id: impl Into<String>, mut f: F)
where
    F: FnMut(&[u8]) -> [u8; 2],
    M: criterion::measurement::Measurement,
{
    group.bench_function(id, |b| {
        let mut i: u8 = 0;
        b.iter_batched(
            || {
                let mut data = [i, 0];
                i = i.wrapping_add(1);
                data[1] = i;
                i = i.wrapping_add(1);
                data
            },
            |data| {
                assert_eq!(f(black_box(&data)), black_box(data));
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn criterion_benchmark(criterion: &mut criterion::Criterion) {
    let mut group = criterion.benchmark_group("single-thread-two-bytes");
    group.throughput(criterion::Throughput::Bytes(2));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let (mut p, mut c) = RingBuffer::<u8>::new(3);

    add_function(&mut group, "1-push-pop", |data| {
        let mut result = [0; 2];
        for &i in data.iter() {
            p.push(i).unwrap();
        }
        for i in &mut result {
            *i = c.pop().unwrap();
        }
        result
    });

    add_function(&mut group, "2-slice-read", |data| {
        let mut result = [0; 2];
        for &i in data.iter() {
            p.push(i).unwrap();
        }
        let chunk = c.read_chunk(data.len()).unwrap();
        let (first, second) = chunk.as_slices();
        let mid = first.len();
        result[..mid].copy_from_slice(first);
        result[mid..].copy_from_slice(second);
        chunk.commit_all();
        result
    });

    add_function(&mut group, "2-slice-write", |data| {
        let mut result = [0; 2];
        let mut chunk = p.write_chunk(data.len()).unwrap();
        let (first, second) = chunk.as_mut_slices();
        let mid = first.len();
        first.copy_from_slice(&data[..mid]);
        second.copy_from_slice(&data[mid..]);
        chunk.commit_all();
        for i in &mut result {
            *i = c.pop().unwrap();
        }
        result
    });

    add_function(&mut group, "2-slice-write-uninit", |data| {
        let mut result = [0; 2];
        let mut chunk = p.write_chunk_uninit(data.len()).unwrap();
        let (first, second) = chunk.as_mut_slices();
        let mid = first.len();
        data[..mid].copy_to_uninit(first);
        data[mid..].copy_to_uninit(second);
        unsafe {
            chunk.commit_all();
        }
        for i in &mut result {
            *i = c.pop().unwrap();
        }
        result
    });

    add_function(&mut group, "3-iterate-read", |data| {
        let mut result = [0; 2];
        for &i in data.iter() {
            p.push(i).unwrap();
        }
        let chunk = c.read_chunk(data.len()).unwrap();
        for (dst, src) in result.iter_mut().zip(chunk) {
            *dst = src;
        }
        result
    });

    add_function(&mut group, "3-iterate-write", |data| {
        let mut result = [0; 2];
        let chunk = p.write_chunk_uninit(data.len()).unwrap();
        chunk.fill_from_iter(&mut data.iter().copied());
        for i in &mut result {
            *i = c.pop().unwrap();
        }
        result
    });

    add_function(&mut group, "4-read", |data| {
        let mut result = [0; 2];
        for &i in data.iter() {
            p.push(i).unwrap();
        }
        let _ = c.read(&mut result).unwrap();
        result
    });

    add_function(&mut group, "4-write", |data| {
        let mut result = [0; 2];
        let _ = p.write(data).unwrap();
        for i in &mut result {
            *i = c.pop().unwrap();
        }
        result
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
