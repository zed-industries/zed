#[macro_use]
extern crate bencher;

use bencher::Bencher;
use lebe::prelude::*;
use byteorder::{ReadBytesExt, LittleEndian, BigEndian, WriteBytesExt};
use std::io::{Read, Write, Cursor};

const COUNT_8:  usize = 2048;
const COUNT_16: usize = COUNT_8 / 2;
const COUNT_32: usize = COUNT_8 / 4;
const COUNT_64: usize = COUNT_8 / 8;


fn bytes(count: usize) -> Cursor<Vec<u8>> {
    let vec: Vec<u8> = (0..count).map(|i| (i % 256) as u8).collect();
    Cursor::new(vec)
}

fn floats(count: usize) -> Vec<f32> {
    (0..count).map(|i| i as f32).collect()
}

fn read_slice_f32_le_crate(bench: &mut Bencher) {
    bench.iter(move ||{
        let mut target = vec![ 0_f32; COUNT_32 ];
        bencher::black_box(bytes(COUNT_8).read_from_little_endian_into(target.as_mut_slice())).unwrap();
        bencher::black_box(target);
    })
}

fn read_slice_f32_le_byteorder(bench: &mut Bencher) {
    bench.iter(move ||{
        let mut target = vec![ 0_f32; COUNT_32 ];
        bencher::black_box(bytes(COUNT_8).read_f32_into::<LittleEndian>(target.as_mut_slice())).unwrap();
        bencher::black_box(target);
    })
}

fn read_slice_f32_be_crate(bench: &mut Bencher) {
    bench.iter(move ||{
        let mut target = vec![ 0_f32; COUNT_32 ];
        bencher::black_box(bytes(COUNT_8).read_from_big_endian_into(target.as_mut_slice())).unwrap();
        bencher::black_box(target);
    })
}

fn read_slice_f32_be_byteorder(bench: &mut Bencher) {
    bench.iter(move ||{
        let mut target = vec![ 0_f32; COUNT_32 ];
        bencher::black_box(bytes(COUNT_8).read_f32_into::<BigEndian>(target.as_mut_slice())).unwrap();
        bencher::black_box(target);
    })
}

// FIXME faster than baseline?!?!!
fn write_slice_f32_le_crate(bench: &mut Bencher) {
    bench.iter(move ||{
        let data = floats(COUNT_32);
        let mut output = Vec::with_capacity(COUNT_8);

        bencher::black_box(output.write_as_little_endian(data.as_slice())).unwrap();
        assert_eq!(output.len(), COUNT_8);
        bencher::black_box(output);
    })
}

fn write_slice_f32_le_byteorder(bench: &mut Bencher) {
    bench.iter(move ||{
        let data = floats(COUNT_32);
        let mut output = Vec::with_capacity(COUNT_8);

        for number in data {
            bencher::black_box(output.write_f32::<LittleEndian>(number)).unwrap();
        }

        assert_eq!(output.len(), COUNT_8);
        bencher::black_box(output);
    })
}


fn write_slice_f32_be_crate(bench: &mut Bencher) {
    bench.iter(move ||{
        let data = floats(COUNT_32);
        let mut output = Vec::with_capacity(COUNT_8);

        bencher::black_box(output.write_as_big_endian(data.as_slice())).unwrap();
        assert_eq!(output.len(), COUNT_8);
        bencher::black_box(output);
    })
}

fn write_slice_f32_be_byteorder(bench: &mut Bencher) {
    bench.iter(move ||{
        let data = floats(COUNT_32);
        let mut output = Vec::with_capacity(COUNT_8);

        for number in data {
            bencher::black_box(output.write_f32::<BigEndian>(number)).unwrap();
        }

        assert_eq!(output.len(), COUNT_8);
        bencher::black_box(output);
    })
}



fn read_slice_baseline(bench: &mut Bencher) {
    bench.iter(move ||{
        let mut target = vec![ 0_u8; COUNT_8 ];
        bencher::black_box(bytes(COUNT_8).read_exact(target.as_mut_slice())).unwrap();
        bencher::black_box(target);
    })
}


fn write_slice_baseline(bench: &mut Bencher) {
    bench.iter(move ||{
        let data = bytes(COUNT_8).into_inner();
        let mut output = Vec::with_capacity(COUNT_8);

        bencher::black_box(output.write_all(data.as_slice())).unwrap();
        bencher::black_box(output);
    })
}

benchmark_group!(
    benches,
    read_slice_f32_be_byteorder, read_slice_f32_be_crate, read_slice_f32_le_byteorder,
    read_slice_f32_le_crate, write_slice_f32_le_byteorder, write_slice_f32_le_crate,
    write_slice_f32_be_byteorder, write_slice_f32_be_crate,
    read_slice_baseline, write_slice_baseline
);

benchmark_main!(benches);