//! Replaces occurrences of `:name:` with the actual emoji for `name` in text.
//!
//! # Usage
//!
//! ```sh
//! $ echo "launch :rocket:" | cargo run --example replace
//! launch 🚀
//! ```

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> io::Result<()> {
    let stdin = {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };
    replace(&stdin, BufWriter::new(io::stdout()))
}

fn replace(mut s: &str, mut o: impl Write) -> io::Result<()> {
    // The meaning of the index values is as follows.
    //
    //  : r o c k e t :
    // ^ ^           ^ ^
    // i m           n j
    //
    // i..j gives ":rocket:"
    // m..n gives "rocket"
    while let Some((i, m, n, j)) = s
        .find(':')
        .map(|i| (i, i + 1))
        .and_then(|(i, m)| s[m..].find(':').map(|x| (i, m, m + x, m + x + 1)))
    {
        match emojis::get_by_shortcode(&s[m..n]) {
            Some(emoji) => {
                // Output everything preceding, except the first colon.
                o.write_all(s[..i].as_bytes())?;
                // Output the emoji.
                o.write_all(emoji.as_bytes())?;
                // Update the string to past the last colon.
                s = &s[j..];
            }
            None => {
                // Output everything preceding but not including the colon.
                o.write_all(s[..n].as_bytes())?;
                // Update the string to start with the last colon.
                s = &s[n..];
            }
        }
    }
    o.write_all(s.as_bytes())
}

#[test]
fn smoke() {
    let tests = [
        ("launch nothing", "launch nothing"),
        ("launch :rocket: something", "launch 🚀 something"),
        ("? :unknown: emoji", "? :unknown: emoji"),
        ("::very:naughty::", "::very:naughty::"),
        (":maybe:rocket:", ":maybe🚀"),
        (":rocket::rocket:", "🚀🚀"),
    ];

    for (i, o) in tests {
        let mut v = Vec::new();
        replace(i, &mut v).unwrap();
        assert_eq!(std::str::from_utf8(&v).unwrap(), o);
    }
}
