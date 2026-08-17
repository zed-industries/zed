#![allow(unused)]

use core::fmt::{self, Write};
use std::io::{self, BufReader, BufRead};

struct Manifest {
    text: String,
    buffer: String,
    input: io::Lines<BufReader<std::fs::File>>,
    line: usize,
}

impl Manifest {
    fn new(input_file: &str) -> Self {
        let text = std::fs::read_to_string("tests/manifesto.txt").expect("To read");
        assert_eq!(text.len(), 5158);
        let input = std::fs::File::open(input_file).expect("To open input");
        let input = BufReader::new(input).lines();

        Self {
            text,
            buffer: String::with_capacity(20),
            input,
            line: 1,
        }
    }

    #[inline(always)]
    fn get(&self, len: usize) -> &str {
        &self.text[..len]
    }

    #[inline(always)]
    fn format<T: fmt::LowerHex>(&mut self, data: T) -> &str {
        self.buffer.clear();
        let _ = write!(&mut self.buffer, "{:08x}", data);
        self.buffer.as_str()
    }

    //returns (manifesto len, expected output)
    fn next_input(&mut self) -> Option<(usize, String)> {
        match self.input.next() {
            Some(input) => {
                let input = input.expect("Cannot read test input");
                let mut input = input.split(',');
                let len = input.next().unwrap();
                let expected = match input.next() {
                    Some(expected) => expected.trim().to_owned(),
                    None => panic!("test file is missing <expected> at line={}", self.line),
                };
                if let Some(unexpected) = input.next() {
                    panic!("test file contains unexpected input='{}' at line={}", unexpected, self.line);
                }
                let len = match len.parse() {
                    Ok(len) => len,
                    Err(error) => panic!("test file contains invalid len='{}' at line = {}", len, self.line),
                };
                self.line += 1;
                Some((len, expected))
            },
            None => None,
        }
    }
}


#[cfg(feature = "xxh3")]
#[test]
fn test_vectors_xxh3() {
    use xxhash_rust::xxh3::{Xxh3, xxh3_64};

    let mut hasher = Xxh3::new();
    let mut fixture = Manifest::new("tests/xxh3_64_test_inputs.txt");
    while let Some((len, expected)) = fixture.next_input() {
        let manifest = fixture.get(len);
        hasher.update(manifest.as_bytes());
        let digest = hasher.digest();
        assert_eq!(xxh3_64(manifest.as_bytes()), digest, "Streaming variant contradict oneshot function");
        let digest = fixture.format(digest);
        assert_eq!(digest, expected);

        hasher.reset();
    }

    assert_eq!(fixture.line, 5158 + 1);
}
