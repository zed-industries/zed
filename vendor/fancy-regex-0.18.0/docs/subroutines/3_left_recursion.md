## Compile-time rejection of left-recursive patterns

fancy-regex's support of subroutines unlocks powerful features such as recursion.
With that power comes the risk of defining patterns that can recurse forever.

To guarantee termination and predictable behavior, fancy-regex rejects left-recursive patterns at compile time.

This check is conservative: if a pattern could recurse without consuming input, it is rejected - even if a particular input would not trigger that behavior.

### What is left recursion?

A pattern is left-recursive if it can re-enter itself without consuming any input.

In other words, the engine can make recursive calls while staying at the same input position.

A simplified example looks like this:

```regexp
(?<expr>\g<expr>a|a)
```

Here, the subroutine expr can immediately call itself before matching anything.
No matter what the input is, this definition allows infinite recursion.

### Why fancy-regex rejects these patterns

Left recursion is problematic because:

- It can cause infinite recursion or unbounded backtracking
- It cannot be made safe by input inspection alone

Even if a specific input would not trigger the recursion, the pattern itself is unsafe.

Rather than attempting to detect or recover from such cases at runtime, fancy-regex enforces a stronger rule:

**Every recursive call must consume input before it can recurse again.**

This guarantees that evaluation always makes progress.

### Conservative by design

The left-recursion check is intentionally conservative.

Consider the following pattern:

```regexp
(?<expr>ab|\g<expr>a)
```

For the input "ab", this pattern would terminate successfully.
However, fancy-regex still rejects it.

Why?

Because the second alternative allows recursion before any input is consumed.
The engine cannot rely on runtime input to guarantee termination.

This is a deliberate design choice:

fancy-regex validates the structure of the pattern, not the behavior of a particular input.

### What is allowed

Recursive patterns are allowed as long as they consume input before recursing.

For example:

```regexp
(?<paren>\((?:[^()]*|\g<paren>)\))
```

Here:

- Each recursive call to `paren` is preceded by a literal '('
- Input is always consumed before recursion
- Termination is guaranteed

This kind of recursion is safe and fully supported.

### How to restructure left-recursive patterns

Left-recursive definitions can often be rewritten in a right-recursive or iterative form.

For example, instead of:

```regexp
(?<expr>\g<expr>a|a)
```

You can write:

```regexp
a+
```

Or, when recursion is genuinely required:

```regexp
(?<expr>a\g<expr>?)
```

In this version, input is consumed before the recursive call, satisfying fancy-regex's safety rules.
