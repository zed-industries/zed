# Syntax

The regex syntax is based on the [regex] crate's and on Oniguruma, with some additional supported syntax.
Where the two conflict, there is a flag to prefer Oniguruma parsing rules. (By default `regex` crate compatible parsing is used.)

Escapes:

`\h`
: hex digit (`[0-9A-Fa-f]`) \
`\H`
: not hex digit (`[^0-9A-Fa-f]`) \
`\e`
: escape control character (`\x1B`) \
`\K`
: keep text matched so far out of the overall match ([docs](https://www.regular-expressions.info/keep.html))\
`\G`
: anchor to where the previous match ended ([docs](https://www.regular-expressions.info/continue.html))\
`\Z`
: anchor to the end of the text before any trailing newlines\
`\O`
: any character including newline\
`\N`
: any character except newline\
`\R`
: general newline - matches all common line break characters: \n, \v, \f, \r, treating \r\n as an atomic unit

Backreferences:

`\1`
: match the exact string that the first capture group matched \
`\2`
: backref to the second capture group, etc. \
`\k<name>`
: match the exact string that the capture group named *name* matched \
`(?P=name)`
: same as `\k<name>` for compatibility with Python, etc. \
`\g<name>`
: call the subroutine defined in capture group named *name* \
`\g<1>`
: call the subroutine defined in capture group 1. Subroutines can be recursive up to 20 levels deep.

Named capture groups:

`(?<name>exp)`
: match *exp*, creating capture group named *name* \
`(?P<name>exp)`
: same as `(?<name>exp)` for compatibility with Python, etc.

Look-around assertions for matching without changing the current position:

`(?=exp)`
: look-ahead, succeeds if *exp* matches to the right of the current position \
`(?!exp)`
: negative look-ahead, succeeds if *exp* doesn't match to the right \
`(?<=exp)`
: look-behind, succeeds if *exp* matches to the left of the current position \
`(?<!exp)`
: negative look-behind, succeeds if *exp* doesn't match to the left

**Note**: Look-behind assertions with variable length (e.g., `(?<=a+)`) are supported with the
`variable-lookbehinds` feature (enabled by default). Without this feature, only constant-length
look-behinds are supported. Variable-length look-behinds can include word boundaries and other
zero-width assertions (e.g., `(?<=\ba+)`) as long as the rest of the pattern doesn't use
backreferences or other "fancy" features that require backtracking within the lookbehind.

Atomic groups using `(?>exp)` to prevent backtracking within `exp`, e.g.:

```rust
# use fancy_regex::Regex;
let re = Regex::new(r"^a(?>bc|b)c$").unwrap();
assert!(re.is_match("abcc").unwrap());
// Doesn't match because `|b` is never tried because of the atomic group
assert!(!re.is_match("abc").unwrap());
```

Conditionals - if/then/else:

`(?(1))`
: continue only if first capture group matched \
`(?(<name>))` or `(?('name'))`
: continue only if capture group named *name* matched \
`(?(1)true_branch|false_branch)`
: if the first capture group matched then execute the true_branch regex expression, else execute false_branch ([docs](https://www.regular-expressions.info/conditional.html)) \
`(?(condition)true_branch|false_branch)`
: if the condition matches then execute the true_branch regex expression, else execute false_branch from the point just before the condition was evaluated \
`(?(DEFINE)(capture group)(?<named_group>another)`
: define capture groups for later use in subroutine calls

Backtracking control verbs:

`(*FAIL)`
: fail the current backtracking branch

Absent repeater:

`(?~abc)`
: match anything until `abc` would match or until the end of the haystack if no match
