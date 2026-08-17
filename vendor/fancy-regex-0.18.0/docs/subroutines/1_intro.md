# Subroutines: reusable patterns with stable meaning

## What is a subroutine

Subroutines in fancy-regex are *compiled pattern definitions* that can be invoked safely and predictably.
Any capture group can become a subroutine - it just needs to be "called".

```regexp
(?<num>\d*\.\d+|\d+) x \g<num>
```

In the above example, a capture group called `num` is defined, to match numbers with or without decimal places. `\g<num>` executes the capture group again, without the author having to re-type the pattern inside it.
The above pattern would match text like `5.2 x 6` for instance.

Think of a subroutine as:

- defined by a capture group
- executed exactly the way the capture group was originally defined
- reusable from multiple places

## Side effects

A subroutine call has one side-effect - it updates the capture group position, which affects backref matching etc.

## Example

Let's imagine a pattern which will match a digit and capture it into group 1. Then it will call that capture group as a subroutine. Then it will do a backref to group 1.

This will match three consecutive digits. The 2nd and 3rd digits must be identical.

```rust
use fancy_regex::{Error, Regex};

let re = Regex::new(r"(\d)\g<1>\1")?;
let result = re.captures("foo 711")?;

let captures = result.unwrap();

let m = captures.get(0).unwrap();

assert_eq!(m.start(), 4);
assert_eq!(m.end(), 7);
assert_eq!(m.as_str(), "711");

let group = captures.get(1).unwrap();
assert_eq!(group.as_str(), "1");

assert!(!re.is_match("foo 717")?);

# Ok::<(), Error>(())
```

In the above example, 7 was stored in capture group 1. Then it was replaced with 1 by the subroutine call. Then the backreference to group 1 can only match the literal `1`.

## Side effect edge cases

Also, in a lookbehind, a subroutine call would not update the capture group position when the currently stored position for that capture group is further to the right in the haystack. i.e. right-most captures take precedence.
