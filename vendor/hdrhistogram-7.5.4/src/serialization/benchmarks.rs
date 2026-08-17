use super::deserializer::{varint_read, varint_read_slice};
use super::v2_serializer::varint_write;
use rand::distributions::uniform::Uniform;
use rand::distributions::Distribution;
use rand::SeedableRng;
use std::io::Cursor;
use test::Bencher;

#[bench]
fn varint_write_rand(b: &mut Bencher) {
    do_varint_write_rand(b, Uniform::new(0, u64::max_value()))
}

#[bench]
fn varint_write_rand_1_byte(b: &mut Bencher) {
    do_varint_write_rand(b, Uniform::new(0, 128))
}

#[bench]
fn varint_write_rand_9_bytes(b: &mut Bencher) {
    do_varint_write_rand(b, Uniform::new(1 << 56, u64::max_value()))
}

#[bench]
fn varint_read_rand(b: &mut Bencher) {
    do_varint_read_rand(b, Uniform::new(0, u64::max_value()))
}

#[bench]
fn varint_read_rand_1_byte(b: &mut Bencher) {
    do_varint_read_rand(b, Uniform::new(0, 128))
}

#[bench]
fn varint_read_rand_9_byte(b: &mut Bencher) {
    do_varint_read_rand(b, Uniform::new(1 << 56, u64::max_value()))
}

#[bench]
fn varint_read_slice_rand(b: &mut Bencher) {
    do_varint_read_slice_rand(b, Uniform::new(0, u64::max_value()))
}

#[bench]
fn varint_read_slice_rand_1_byte(b: &mut Bencher) {
    do_varint_read_slice_rand(b, Uniform::new(0, 128))
}

#[bench]
fn varint_read_slice_rand_9_byte(b: &mut Bencher) {
    do_varint_read_slice_rand(b, Uniform::new(1 << 56, u64::max_value()))
}

fn do_varint_write_rand(b: &mut Bencher, range: Uniform<u64>) {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let num = 1000_000;
    let mut vec: Vec<u64> = Vec::new();

    for _ in 0..num {
        vec.push(range.sample(&mut rng));
    }

    let mut buf = [0; 9];
    b.iter(|| {
        for i in vec.iter() {
            let _ = varint_write(*i, &mut buf);
        }
    });
}

fn do_varint_read_rand(b: &mut Bencher, range: Uniform<u64>) {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let num = 1000_000;
    let mut vec = Vec::new();
    vec.resize(9 * num, 0);
    let mut bytes_written = 0;

    for _ in 0..num {
        bytes_written += varint_write(range.sample(&mut rng), &mut vec[bytes_written..]);
    }

    b.iter(|| {
        let mut cursor = Cursor::new(&vec);
        for _ in 0..num {
            let _ = varint_read(&mut cursor);
        }
    });
}

fn do_varint_read_slice_rand(b: &mut Bencher, range: Uniform<u64>) {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let num = 1000_000;
    let mut vec = Vec::new();

    vec.resize(9 * num, 0);
    let mut bytes_written = 0;

    for _ in 0..num {
        bytes_written += varint_write(range.sample(&mut rng), &mut vec[bytes_written..]);
    }

    b.iter(|| {
        let mut input_index = 0;
        // cheat a little bit: this will skip the last couple numbers, but that's why we do a
        // million numbers. Losing the last few won't be measurable.
        while input_index < bytes_written - 9 {
            let (_, bytes_read) = varint_read_slice(&vec[input_index..(input_index + 9)]);
            input_index += bytes_read;
        }
    });
}
