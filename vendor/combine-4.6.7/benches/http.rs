#![cfg(feature = "std")]

#[macro_use]
extern crate criterion;
#[macro_use]
extern crate combine;

use std::fmt;

use {
    combine::{
        many, many1,
        parser::range::{range, take_while1},
        stream::easy,
        token, ParseError, Parser, RangeStream,
    },
    criterion::{black_box, Bencher, Criterion},
};

#[allow(dead_code)]
#[derive(Debug)]
struct Request<'a> {
    method: &'a [u8],
    uri: &'a [u8],
    version: &'a [u8],
}

#[allow(dead_code)]
#[derive(Debug)]
struct Header<'a> {
    name: &'a [u8],
    value: Vec<&'a [u8]>,
}

fn is_token(c: u8) -> bool {
    !matches!(
        c,
        128..=255
        | 0..=31
        | b'('
        | b')'
        | b'<'
        | b'>'
        | b'@'
        | b','
        | b';'
        | b':'
        | b'\\'
        | b'"'
        | b'/'
        | b'['
        | b']'
        | b'?'
        | b'='
        | b'{'
        | b'}'
        | b' '
    )
}

fn is_horizontal_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}
fn is_space(c: u8) -> bool {
    c == b' '
}
fn is_not_space(c: u8) -> bool {
    c != b' '
}
fn is_http_version(c: u8) -> bool {
    (b'0'..=b'9').contains(&c) || c == b'.'
}

fn end_of_line<'a, Input>() -> impl Parser<Input, Output = u8>
where
    Input: RangeStream<Token = u8, Range = &'a [u8]>,
{
    (token(b'\r'), token(b'\n')).map(|_| b'\r').or(token(b'\n'))
}

fn message_header<'a, Input>() -> impl Parser<Input, Output = Header<'a>>
where
    Input: RangeStream<Token = u8, Range = &'a [u8]>,
{
    let message_header_line = (
        take_while1(is_horizontal_space),
        take_while1(|c| c != b'\r' && c != b'\n'),
        end_of_line(),
    )
        .map(|(_, line, _)| line);

    struct_parser!(Header {
        name: take_while1(is_token),
        _: token(b':'),
        value: many1(message_header_line),
    })
}

type HttpRequest<'a> = (Request<'a>, Vec<Header<'a>>);

fn parse_http_request<'a, Input>(input: Input) -> Result<(HttpRequest<'a>, Input), Input::Error>
where
    Input: RangeStream<Token = u8, Range = &'a [u8]>,
{
    let http_version = range(&b"HTTP/"[..]).with(take_while1(is_http_version));

    let request_line = struct_parser!(Request {
        method: take_while1(is_token),
        _: take_while1(is_space),
        uri: take_while1(is_not_space),
        _: take_while1(is_space),
        version: http_version,
    });

    let mut request = (
        request_line,
        end_of_line(),
        many(message_header()),
        end_of_line(),
    )
        .map(|(request, _, headers, _)| (request, headers));

    request.parse(input)
}

static REQUESTS: &[u8] = include_bytes!("http-requests.txt");

fn http_requests_small(b: &mut Bencher<'_>) {
    http_requests_bench(b, easy::Stream(REQUESTS))
}

fn http_requests_large(b: &mut Bencher<'_>) {
    use std::iter;

    let mut buffer = Vec::with_capacity(REQUESTS.len() * 5);
    for buf in iter::repeat(REQUESTS).take(5) {
        buffer.extend_from_slice(buf);
    }
    http_requests_bench(b, easy::Stream(&buffer[..]))
}

fn http_requests_large_cheap_error(b: &mut Bencher<'_>) {
    use std::iter;

    let mut buffer = Vec::with_capacity(REQUESTS.len() * 5);
    for buf in iter::repeat(REQUESTS).take(5) {
        buffer.extend_from_slice(buf);
    }
    http_requests_bench(b, &buffer[..])
}

fn http_requests_bench<'a, Input>(b: &mut Bencher<'_>, buffer: Input)
where
    Input: RangeStream<Token = u8, Range = &'a [u8]> + Clone,
    Input::Error: fmt::Debug,
{
    b.iter(|| {
        let mut buf = black_box(buffer.clone());

        while buf.clone().uncons().is_ok() {
            match parse_http_request(buf) {
                Ok(((_, _), b)) => {
                    buf = b;
                }
                Err(err) => panic!("{:?}", err),
            }
        }
    });
}

fn http_requests(c: &mut Criterion) {
    c.bench_function("http_requests_small", http_requests_small);
    c.bench_function("http_requests_large", http_requests_large);
    c.bench_function(
        "http_requests_large_cheap_error",
        http_requests_large_cheap_error,
    );
}

criterion_group!(http, http_requests,);
criterion_main!(http);
