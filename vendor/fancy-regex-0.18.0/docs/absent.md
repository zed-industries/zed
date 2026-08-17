# Absent Operators

The absent operators are worth talking about because they are quite uncommon.

## Absent Repeater

An absent repeater node is defined by the syntax `(?~inner_pattern)`, and it will match any text where the inner pattern does not match (i.e. is absent), including across newlines.
This does not add any new abilities to the engine, it just allows to clarify intent and to be more easily optimized under the hood. fancy-regex mainly implements this for Oniguruma compatibility.

It works best or is at least easiest to understand when the inner pattern is a literal.

### Example

Let's imagine you have some Markdown, containing some code fences.

It might look something like this:

````md
# Some Heading

Given a todo list like this:

**Input:**
```json
{
  "todos": [
    {
      "content": "Create `some_helper_func` helper in some_file.rs that takes a closure to check the error",
      "status": "complete",
      "priority": "high"
    },
    {
      "content": "Update error-asserting tests in some_file.rs to use `some_helper_func`",
      "status": "complete",
      "priority": "high"
    },
    {
      "content": "Run `cargo fmt` and `cargo test`",
      "status": "pending",
      "priority": "medium"
    }
  ]
}
```

You might expect this output:

**Output:**
```text
High priority tasks have now been completed.
```

Some more text.
````

Let's say you want to match all input and output codeblocks.

Typically you *could* do it like:

```regexp
[*]{2}(?:In|Out)put:[*]{2}\n```(?:[^`]+|`(?!``))+```
```

This would match everything inside the codeblock which is not a backtick, or backticks which are not followed by another 2 backticks, until it reaches the 3 backticks marking the end of the codeblock.
Generally this type of construct can be quite hard to follow and reason about, to be sure it won't suffer from catastrophic backtracking.

With the absent repeater, the intention becomes a lot easier to understand - match anything that isn't 3 backticks, followed by 3 backticks.
```regexp
[*]{2}(?:In|Out)put:[*]{2}\n```(?~```)```
```

Where it really shines is when you need more complicated expressions, like to match a variable number of backticks from the code fence boundaries, it becomes a lot easier to read than an expanded alternative which would avoid catastrophic backtracking.

```rust
use fancy_regex::Regex;

// Match a code fence: opening backticks (3+), content (absent the same backticks), closing backticks
let re = Regex::new(r"(?<!`)(`{3,}(?!`))\w*\n(?~\1)\n(\1)")?;

// A code fence with 4 backticks, where the inner code contains 3 backticks
let input = "````text\nsome code with ``` backticks\n````";
let captures = re.captures(input)?.expect("should match");

// The overall match spans the entire input
let m = captures.get(0).unwrap();
assert_eq!(m.start(), 0);
assert_eq!(m.end(), input.len());
assert_eq!(m.as_str(), input);

// Group 1: the opening 4 backticks
let open = captures.get(1).unwrap();
assert_eq!(open.as_str(), "````");
assert_eq!(open.start(), 0);
assert_eq!(open.end(), 4);

// Group 2: the closing 4 backticks
let close = captures.get(2).unwrap();
assert_eq!(close.as_str(), "````");
assert_eq!(close.start(), input.len() - 4);
assert_eq!(close.end(), input.len());

# Ok::<(), fancy_regex::Error>(())
```

It also allows the engine to optimize it accordingly.

### Other ways of looking at it

The absent repeater can be considered shorthand for this:
```regexp
(?((?!absent))\O|)*
```

Essentially a conditional, which says when the absent expression doesn't match, match a single character including newlines. When the absent expression does match, match nothing. Repeat greedily.
