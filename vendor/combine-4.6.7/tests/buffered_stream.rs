#![cfg(feature = "std")]

use combine::{
    attempt, choice, many, many1,
    parser::{
        char::{char, digit, spaces, string},
        combinator::recognize,
    },
    sep_by, skip_many1,
    stream::{
        buffered,
        easy::{self, Error, Errors},
        position, IteratorStream,
    },
    Parser, Positioned,
};

#[test]
fn shared_stream_buffer() {
    // Iterator that can't be cloned
    let text = "10,222,3,44".chars().map(|c| {
        if c.is_digit(10) {
            (c as u8 + 1) as char
        } else {
            c
        }
    });
    let buffer = buffered::Stream::new(position::Stream::new(IteratorStream::new(text)), 1);
    let int: &mut dyn Parser<_, Output = _, PartialState = _> =
        &mut many(digit()).map(|s: String| s.parse::<i64>().unwrap());
    let result = sep_by(int, char(',')).parse(buffer).map(|t| t.0);
    assert_eq!(result, Ok(vec![21, 333, 4, 55]));
}

#[test]
fn shared_stream_backtrack() {
    let text = "apple,apple,ananas,orangeblah";
    let mut iter = text.chars();
    // Iterator that can't be cloned
    let stream = buffered::Stream::new(position::Stream::new(IteratorStream::new(&mut iter)), 2);

    let value: &mut dyn Parser<_, Output = _, PartialState = _> = &mut choice([
        attempt(string("apple")),
        attempt(string("orange")),
        attempt(string("ananas")),
    ]);
    let mut parser = sep_by(value, char(','));
    let result = parser.parse(stream).map(|t| t.0);
    assert_eq!(result, Ok(vec!["apple", "apple", "ananas", "orange"]));
}

#[test]
fn shared_stream_insufficent_backtrack() {
    let text = "apple,apple,ananas,orangeblah";
    let mut iter = text.chars();
    // Iterator that can't be cloned
    let stream = buffered::Stream::new(
        easy::Stream(position::Stream::new(IteratorStream::new(&mut iter))),
        1,
    );

    let value: &mut dyn Parser<_, Output = _, PartialState = _> = &mut choice([
        attempt(string("apple")),
        attempt(string("orange")),
        attempt(string("ananas")),
    ]);
    let mut parser = sep_by(value, char(','));
    let result: Result<Vec<&str>, _> = parser.parse(stream).map(|t| t.0);
    assert!(result.is_err());
    assert!(
        result
            .as_ref()
            .unwrap_err()
            .errors
            .iter()
            .any(|err| *err == Error::Message("Backtracked to far".into())),
        "{}",
        result.unwrap_err()
    );
}

/// Test which checks that a stream which has ended does not repeat the last token in some cases in
/// which case this test would loop forever
#[test]
fn always_output_end_of_input_after_end_of_input() {
    let text = "10".chars();
    let buffer = buffered::Stream::new(position::Stream::new(IteratorStream::new(text)), 1);
    let int = many1(digit()).map(|s: String| s.parse::<i64>().unwrap());
    let result = many(spaces().with(int)).parse(buffer).map(|t| t.0);
    assert_eq!(result, Ok(vec![10]));
}

#[test]
fn position() {
    let text = "10abc".chars();
    let stream = buffered::Stream::new(position::Stream::new(IteratorStream::new(text)), 3);
    assert_eq!(stream.position(), 0);
    let result = many1::<Vec<_>, _, _>(digit()).parse(stream);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().1.position(), 2);
}

#[test]
fn buffered_stream_recognize_issue_256() {
    let mut parser = recognize::<String, _, _>(skip_many1(digit()));
    let input = "12 ";
    assert_eq!(
        parser
            .parse(buffered::Stream::new(easy::Stream(input), 1))
            .map_err(|err| err.map_position(|pos| pos.translate_position(input))),
        Err(Errors {
            position: 2,
            errors: vec![easy::Error::Message("Backtracked to far".into())]
        })
    );
}
