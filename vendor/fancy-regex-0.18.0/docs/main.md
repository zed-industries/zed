An implementation of regexes, supporting a relatively rich set of features, including backreferences
and lookaround. Aims to be compatible with Oniguruma syntax when the relevant flag is set.

It builds on top of the excellent [regex] crate. If you are not
familiar with it, make sure you read its documentation and maybe you don't even need fancy-regex.

If your regex or parts of it does not use any special features, the matching is delegated to the
regex crate. That means it has linear runtime. But if you use "fancy" features such as
backreferences or look-around, an engine with backtracking needs to be used. In that case, the regex
can be slow and take exponential time to run because of what is called "catastrophic backtracking".
This depends on the regex and the input.

# Usage

The API should feel very similar to the regex crate, and involves compiling a regex and then using
it to find matches in text.

## Example: Matching text

An example with backreferences to check if a text consists of two identical words:

```rust
use fancy_regex::Regex;

let re = Regex::new(r"^(\w+) (\1)$").unwrap();
let result = re.is_match("foo foo");

assert!(result.is_ok());
let did_match = result.unwrap();
assert!(did_match);
```

Note that like in the regex crate, the regex needs anchors like `^` and `$` to match against the
entire input text.

## Example: Finding the position of matches

```rust
use fancy_regex::Regex;

let re = Regex::new(r"(\d)\1").unwrap();
let result = re.find("foo 22");

assert!(result.is_ok(), "execution was successful");
let match_option = result.unwrap();

assert!(match_option.is_some(), "found a match");
let m = match_option.unwrap();

assert_eq!(m.start(), 4);
assert_eq!(m.end(), 6);
assert_eq!(m.as_str(), "22");
```

## Example: Capturing groups

```rust
use fancy_regex::Regex;

let re = Regex::new(r"(?<!AU)\$(\d+)").unwrap();
let result = re.captures("AU$10, $20");

let captures = result.expect("Error running regex").expect("No match found");
let group = captures.get(1).expect("No group");
assert_eq!(group.as_str(), "20");
```

## Example: Splitting text

```rust
use fancy_regex::Regex;

let re = Regex::new(r"[ \t]+").unwrap();
let target = "a b \t  c\td    e";
let fields: Vec<&str> = re.split(target).map(|x| x.unwrap()).collect();
assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);

let fields: Vec<&str> = re.splitn(target, 3).map(|x| x.unwrap()).collect();
assert_eq!(fields, vec!["a", "b", "c\td    e"]);
```

[regex]: https://crates.io/crates/regex
