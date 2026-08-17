#![cfg(feature = "mp4")]
#[macro_use]
extern crate criterion;

use std::{fs::File, io::Read, str::from_utf8};

use {
    combine::{
        parser::{
            byte::num::be_u32,
            range::{range, take},
        },
        stream::easy::ParseError,
        *,
    },
    criterion::{black_box, Bencher, Criterion},
};

#[derive(Clone, PartialEq, Eq, Debug)]
struct FileType<'a> {
    major_brand: &'a str,
    major_brand_version: &'a [u8],
    compatible_brands: Vec<&'a str>,
}

#[derive(Clone, Debug)]
enum MP4Box<'a> {
    Ftyp(FileType<'a>),
    Moov,
    Mdat,
    Free,
    Skip,
    Wide,
    Unknown,
}

fn parse_mp4(data: &[u8]) -> Result<(Vec<MP4Box>, &[u8]), ParseError<&[u8]>> {
    let brand_name = || take(4).and_then(from_utf8);
    let filetype_box = (
        range(&b"ftyp"[..]),
        brand_name(),
        take(4),
        many(brand_name()),
    )
        .map(|(_, m, v, c)| {
            MP4Box::Ftyp(FileType {
                major_brand: m,
                major_brand_version: v,
                compatible_brands: c,
            })
        });

    let mp4_box = be_u32().then(|offset| take(offset as usize - 4));
    let mut box_parser = choice((
        filetype_box,
        range(&b"moov"[..]).map(|_| MP4Box::Moov),
        range(&b"mdat"[..]).map(|_| MP4Box::Mdat),
        range(&b"free"[..]).map(|_| MP4Box::Free),
        range(&b"skip"[..]).map(|_| MP4Box::Skip),
        range(&b"wide"[..]).map(|_| MP4Box::Wide),
        value(MP4Box::Unknown),
    ));
    let data_interpreter =
        mp4_box.flat_map(|box_data| box_parser.easy_parse(box_data).map(|t| t.0));

    many(data_interpreter).easy_parse(data)
}

fn run_test(b: &mut Bencher, data: &[u8]) {
    b.iter(|| match parse_mp4(data) {
        Ok(x) => black_box(x),
        Err(err) => panic!("{}", err.map_range(|bytes| format!("{:?}", bytes))),
    });
}

fn mp4_small_test(c: &mut Criterion) {
    let mut mp4_small = Vec::new();
    File::open("benches/small.mp4")
        .and_then(|mut f| f.read_to_end(&mut mp4_small))
        .expect("Unable to read benches/small.mp4");

    c.bench_function("mp4_small", move |b| run_test(b, &mp4_small));
}

criterion_group!(mp4, mp4_small_test);
criterion_main!(mp4);
