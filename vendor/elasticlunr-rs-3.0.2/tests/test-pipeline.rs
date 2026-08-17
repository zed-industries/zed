// Input text is excerpted from public domain books on gutenberg.org or wikisource.org

use elasticlunr::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

#[allow(dead_code)]
fn write_output(lang: &dyn Language) {
    let code = lang.code();
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");

    let input = base.join(&format!("{}.in.txt", code));
    let mut input_str = String::new();
    File::open(&input)
        .unwrap()
        .read_to_string(&mut input_str)
        .unwrap();

    let output = base.join(&format!("{}.out.txt", code));
    let mut output = File::create(&output).unwrap();

    let pipeline = lang.make_pipeline();
    let tokens = pipeline.run(lang.tokenize(&input_str));

    for tok in tokens {
        writeln!(&mut output, "{}", tok).unwrap();
    }
}

fn compare_to_fixture(lang: &dyn Language) {
    let code = lang.code();
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data");

    let input = base.join(&format!("{}.in.txt", code));
    let mut input_str = String::new();
    File::open(&input)
        .unwrap()
        .read_to_string(&mut input_str)
        .unwrap();

    let output = base.join(&format!("{}.out.txt", code));
    let mut output = BufReader::new(File::open(&output).unwrap()).lines();

    let pipeline = lang.make_pipeline();
    let tokens = pipeline.run(lang.tokenize(&input_str));

    for tok in tokens {
        assert_eq!(
            tok,
            output.next().unwrap().unwrap(),
            "Comparing pipeline tokens to fixture for {}",
            lang.name()
        );
    }
}

#[test]
fn test_languages() {
    for lang in lang::languages() {
        //write_output(lang.as_ref());
        compare_to_fixture(lang.as_ref());
    }
}
