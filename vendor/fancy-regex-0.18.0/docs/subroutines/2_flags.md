## Flags and the common misconception

When subroutines are first introduced, a very common assumption is:

> *"If I apply flags at the call site, they should affect the subroutine."*

This assumption is reasonable — many regex engines either behave this way or do not specify the behavior clearly.
**fancy-regex does not do this**, and the reason is central to its design.

Let's look at a concrete example.

---

### The pattern

```regex
\A(?<word>[a-z]+)\s+(?i:\g<word>)
```

### The input

```text
hello Mr
```

At first glance, this pattern appears to say:

1. At the beginning of the input string
2. Match a word of lowercase letters and capture it as `word`
3. Match some whitespace
4. Call the subroutine `word`, but case-insensitively

Many users therefore expect this pattern to match the input above.

**It does not.**

---

### What actually happens

The key to understanding this behavior is that **flags belong to the subroutine definition, not the call site**.

Let's walk through the execution step by step.

---

### Execution trace

#### Step 1: Assert position at the beginning of the input string

* Pattern: `\A`
* Active flags: none
* Input position: 0 (start of `"hello Mr"`)

#### Step 2: Enter subroutine definition `word`

* Pattern: `[a-z]+`
* Active flags: none
* Input position: 0 (start of `"hello Mr"`)

The engine greedily matches `[a-z]+` against `"hello"`.
The range 0 - 5 is stored in capture group 1, whose name is `word`.

#### Step 3: Exit subroutine definition `word` and continue matching

* Pattern: `\s+`
* Active flags: none
* Input position: 5 (at the space after `hello`)

The engine matches `\s+` against `" "`.

#### Step 4: Call subroutine `word`

* Pattern: `\g<word>` -> `(?<word>[a-z]+)`
* Active flags: none - because the capture group definition had no flags active
* Input position: 6 (after `" "` at the `'M'` of `"Mr"`)

This fails immediately, because:

* The pattern is case-sensitive
* The first character is `'M'`, which does not fall in the range a-z.

#### Step 5: No alternatives available

* There are no alternations inside `word`
* There are no backtracking points before the failure
* The anchor prevents us from trying other starting positions in the input string

The match fails.

---

### Why the `i` flag did not apply

The `i` flag appears **only at the call site**:

```regex
(?i:\g<word>)
```

However, calling a subroutine does **not** re-evaluate or modify its definition.

The subroutine `word` was compiled once, with these properties:

* Pattern: `[a-z]+`
* Flags: none
* Capture group number: 1

When the subroutine is called, the engine:

* Enters the already-compiled definition
* Executes it exactly as defined
* Ignores any flags applied at the call site

---

### This is a feature, not a limitation

fancy-regex deliberately enforces this rule to guarantee that:

* A subroutine behaves the same everywhere it is used
* Flags cannot silently change the meaning of a reused pattern
* There is no "action at a distance" from call sites

If call-site flags were allowed to affect subroutines, the same subroutine could behave differently depending on where it was called - making patterns harder to reason about and easier to misuse.

---

### Expressing the intended behavior

If the intent is for `word` to be matched case-insensitively, the flag must be applied **at the definition**:

```regex
(?i:(?<word>[a-z]+))\s+\g<word>
```

or
```regex
(?<word>(?i:[a-z]+))\s+\g<word>
```

Now the subroutine is compiled with the `i` flag, and every call to it behaves consistently.

---

### Key takeaway

> **Subroutines in fancy-regex are compiled once, with fixed flags.**
> **Call sites cannot change their behavior.**

This rule enables safe reuse, predictable execution, and clear reasoning - especially in larger and more complex patterns. It also matches Oniguruma behavior, so if you plan to use fancy-regex as a memory-safe alternative, you can!
