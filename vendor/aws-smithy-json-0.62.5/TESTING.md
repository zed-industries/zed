How to run JSONTestSuite against aws-smithy-json deserialize
============================================================

When making changes to the `deserialize` module, it is a good idea
to run the changes against the [JSONTestSuite](https://github.com/nst/JSONTestSuite)
and manually examine the test results.

### How to setup the JSONTestSuite

1. Clone the [JSONTestSuite](https://github.com/nst/JSONTestSuite) repository.
2. In `JSONTestSuite/parsers`, create a new Cargo bin project named `test_json-aws_smithy_json`.
3. Add the following dependencies to the `Cargo.toml` (be sure to replace `<local-path-to-smithy-rs>`:

```
aws-smithy-json = { path = "<local-path-to-smithy-rs>/rust-runtime/aws-smithy-json" }
```

4. Replace the code in `main.rs` with:

```rust
use std::fs::File;
use std::io::Read;
use std::env;

use aws_smithy_json::deserialize::{json_token_iter, Token, Error};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} file.json", args[0]);
        std::process::exit(1);
    }

    let ref path = args[1];
    let mut s = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    match f.read_to_string(&mut s) {
        Err(_) => std::process::exit(1),
        Ok(_) => println!("{}", s),
    }

    let result: Result<Vec<Token>, Error> = json_token_iter(s.as_bytes()).collect();
    match result {
        Err(_) => std::process::exit(1),
        Ok(value) => if value.is_empty() {
            std::process::exit(1)
        } else {
            // The test suite includes incomplete objects and arrays (i.e., "[null,").
            // These are completely valid for this parser, so we'll just pretend to have
            // failed to parse these to satisfy the test suite.
            if value.first() == Some(&Token::StartObject) && value.last() != Some(&Token::EndObject) {
                std::process::exit(1)
            }
            if value.first() == Some(&Token::StartArray) && value.last() != Some(&Token::EndArray) {
                std::process::exit(1)
            }
            // Unescape all strings and fail if any of them failed to unescape.
            for token in value {
                if let Token::ValueString(escaped) = token {
                    if escaped.into_unescaped().is_err() {
                        std::process::exit(1)
                    }
                }
            }
            std::process::exit(0)
        }
    }
}
```

5. Compile this program with `cargo build --release`.
6. Modify `JSONTestSuite/run_tests.py` so that the `programs` dictionary only contains this one entry:

```
programs = {
   "Rust aws-smithy-json":
       {
           "url":"dontcare",
           "commands":[os.path.join(PARSERS_DIR, "test_json-aws_smithy_json/target/release/sj")]
       }
}
```

7. Run `run_tests.py` and examine the output with a web browser by opening `JSONTestSuite/results/parsing.html`.

### Examining the results

When looking at `JSONTestSuite/results/parsing.html`, there is a matrix of test cases against their
results with a legend at the top.

Any test result marked with blue or light blue is for a test case where correct behavior isn't specified,
so use your best judgement to decide if it should have succeeded or failed.

The other colors are bad and should be carefully examined. At time of writing, the following test cases
succeed when they should fail, and we intentionally left it that way since we're not currently concerned
about being more lenient in the number parsing:

```
n_number_-01.json                           [-01]
n_number_-2..json                           [-2.]
n_number_0.e1.json                          [0.e1]
n_number_2.e+3.json                         [2.e+3]
n_number_2.e-3.json                         [2.e-3]
n_number_2.e3.json                          [2.e3]
n_number_neg_int_starting_with_zero.json    [-012]
n_number_neg_real_without_int_part.json     [-.123]
n_number_real_without_fractional_part.json  [1.]
n_number_with_leading_zero.json             [012]
```

This test case succeeds with our parser and that's OK since we're
a token streaming parser (multiple values are allowed):
```
n_structure_double_array.json               [][]
```
