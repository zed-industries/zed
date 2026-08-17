#![cfg(feature = "std")]

use {
    combine::{
        parser::{
            byte::take_until_bytes,
            combinator::{any_send_sync_partial_state, recognize, AnySendSyncPartialState},
        },
        Parser, RangeStream,
    },
    criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion},
    partial_io::{PartialOp, PartialRead},
    std::io::Cursor,
};

fn test_data() -> Vec<u8> {
    let mut input = vec![b' '; 5_000_000];
    input.push(b'1');

    input
}

fn parser<'a, I>() -> impl combine::Parser<I, Output = usize, PartialState = AnySendSyncPartialState>
where
    I: RangeStream<Token = u8, Range = &'a [u8]>,
    I::Error: combine::ParseError<u8, &'a [u8], I::Position>,
{
    any_send_sync_partial_state(
        recognize(take_until_bytes(&b"1"[..])).map(|spaces: Vec<u8>| spaces.len()),
    )
}

fn bench_small_buf(bencher: &mut Bencher<'_>) {
    let input = test_data();
    let mut decoder = combine::stream::decoder::Decoder::new();

    bencher.iter(|| {
        let cursor = Cursor::new(&input);
        let mut partial_read =
            PartialRead::new(cursor, std::iter::repeat(PartialOp::Limited(1000)));
        let mut ref_decoder = &mut decoder;

        let result = combine::decode!(ref_decoder, partial_read, parser(), |input, _position| {
            combine::easy::Stream::from(input)
        },);

        match result {
            Ok(usize) => black_box(usize),
            Err(err) => {
                println!("{:?}", err);
                panic!();
            }
        };
    });
}

fn bench_big_buf(bencher: &mut Bencher<'_>) {
    let input = test_data();
    let mut decoder = combine::stream::decoder::Decoder::new();

    bencher.iter(|| {
        let cursor = Cursor::new(&input);
        let mut partial_read = PartialRead::new(cursor, std::iter::repeat(PartialOp::Unlimited));
        let mut ref_decoder = &mut decoder;

        let result = combine::decode!(ref_decoder, partial_read, parser(), |input, _position| {
            combine::easy::Stream::from(input)
        },);

        match result {
            Ok(usize) => black_box(usize),
            Err(err) => {
                println!("{:?}", err);
                panic!();
            }
        };
    });
}

fn bench(c: &mut Criterion) {
    c.bench_function("buffers_small", bench_small_buf);
    c.bench_function("buffers_big", bench_big_buf);
}

criterion_group!(buffers, bench);
criterion_main!(buffers);
