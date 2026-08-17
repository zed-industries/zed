## Recursion

Recursion is when a subroutine calls itself, directly or indirectly.

### Depth limit

fancy-regex supports recursion up to 20 levels deep.

Let's look at a simple example to prove this:
```regexp
(a\g<1>?)
```

Here we have a pattern which defines capture group 1 as consuming the literal `a`, followed by calling itself between 0 and 1 times greedily.

With 22 `a` characters as input, only 20 are matched:

```rust
use fancy_regex::{Error, Regex};

let pattern = r"(a\g<1>?)";
let re = Regex::new(pattern)?;

let haystack = "aaaaaaaaaaaaaaaaaaaaaa"; // 22 a's
let result = re.find(haystack)?;

let found = result.unwrap();
// match is limited to 20 characters due to recursion depth limit
assert_eq!(found.as_str().len(), 20);

# Ok::<(), Error>(())
```

### Unbounded recursion

fancy-regex will return a compile error for patterns which recurse indefinitely.

Let's look at a simple example to prove this:
```regexp
(a\g<1>)
```

Here, capture group 1 consumes the literal `a`, then calls itself unconditionally. After recursion level 20 is reached, there is not a single path which would return a match.

### Side effects

You may remember that it was stated earlier that the side effect of a subroutine call is that the capture group will be updated. It would be more accurate to say that the capture group is updated for non-recursive subroutine calls only.

Why?

Imagine a pattern like:

```regexp
(?<foo>a|\(\g<foo>\))
```

It will match the literal `a`, or any number of balanced parenthesis surrounding `a`.
If the recursive subroutine call would update the capture group start position, the opening parenthesis would not be included in the capture group.
If the recursive subroutine call would update the capture group end position (as well), you'd get the inner most subroutine call's start position and outer most subroutine call's end position, which would then be overridden anyway when the capture group at the root level is exited.
This would produce exceptionally odd and confusing behavior.

### Backreferences
fancy-regex does not yet support relative recursion level backreferences, and attempting to backreference a capture group which is currently being recursed is at present a compile error.

Example (adapting the previous pattern):

```regexp
(?<foo>a|\(\g<foo>\)\k<foo>?)
```

With an input like:

```text
(((a)(a)))
```
Oniguruma would give you two matches - the two `(a)`s.
fancy-regex would give (if the compile error were removed and no other changes made,) a single match of `(a)(a`, which is clearly not what anyone would expect.
fancy-regex prefers correctness and rejects such patterns rather than exhibiting undefined behavior.
