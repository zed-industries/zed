<div align="center">
    <img alt="Wax" src="https://raw.githubusercontent.com/olson-sean-k/wax/master/doc/wax.svg?sanitize=true" width="320"/>
</div>
<br/>

**Wax** is a Rust library that provides opinionated and portable globs that can
be matched against file paths and directory trees. Globs use a familiar syntax
and support expressive features with semantics that emphasize component
boundaries.

[![GitHub](https://img.shields.io/badge/GitHub-olson--sean--k/wax-8da0cb?logo=github&style=for-the-badge)](https://github.com/olson-sean-k/wax)
[![docs.rs](https://img.shields.io/badge/docs.rs-wax-66c2a5?logo=rust&style=for-the-badge)](https://docs.rs/wax)
[![crates.io](https://img.shields.io/crates/v/wax.svg?logo=rust&style=for-the-badge)](https://crates.io/crates/wax)

## Basic Usage

Match a path against a glob:

```rust
use wax::{Glob, Program};

let glob = Glob::new("*.png").unwrap();
assert!(glob.is_match("logo.png"));
```

Match a path against a glob with matched text (captures):

```rust
use wax::{CandidatePath, Glob, Program};

let glob = Glob::new("**/{*.{go,rs}}").unwrap();

let path = CandidatePath::from("src/main.go");
let matched = glob.matched(&path).unwrap();

assert_eq!("main.go", matched.get(2).unwrap());
```

Match a directory tree against a glob:

```rust
use wax::Glob;

let glob = Glob::new("**/*.{md,txt}").unwrap();
for entry in glob.walk("doc") {
    let entry = entry.unwrap();
    // ...
}
```

Match a directory tree against a glob with negations:

```rust
use wax::walk::{FileIterator, LinkBehavior};
use wax::Glob;

let glob = Glob::new("**/*.{md,txt}").unwrap();
for entry in glob
    .walk_with_behavior("doc", LinkBehavior::ReadTarget)
    .not("**/secret/**")
    .unwrap()
{
    let entry = entry.unwrap();
    // ...
}
```

Match a path against multiple globs:

```rust
use wax::{Glob, Program};

let any = wax::any([
    "src/**/*.rs",
    "tests/**/*.rs",
    "doc/**/*.md",
    "pkg/**/PKGBUILD",
]).unwrap();
assert!(any.is_match("src/token/mod.rs"));
```

See more details below.

## Construction

Globs are encoded as UTF-8 strings called glob expressions that resemble Unix
paths consisting of nominal components delimited by separators. The most
fundamental type in the Wax API is `Glob`, which is constructed from a glob
expression via inherent functions or standard conversion traits. Data is
borrowed where possible in most APIs, but can be copied into owned instances
using an `into_owned` method with most types.

```rust
use wax::Glob;

let glob = Glob::new("site/img/logo.svg").unwrap();
```

Not only are APIs designed for portability, **but so too are glob expressions**.
Regardless of platform or operating system, globs support the same features and
use the same syntax. **Glob expressions are distinct from paths**, which [differ
in syntax and features](#schemes-and-prefixes) on each platform.

In glob expressions, forward slash `/` is the only path component separator and
back slashes `\` are forbidden (back slash is used for escape sequences, but the
literal sequence `\\` is not supported). This means that it is impossible to
represent `\` in nominal path components, but this character is generally
forbidden as such and its disuse avoids confusion.

Globs enforce various rules regarding meta-characters, patterns, and component
boundaries that reject [nonsense expressions](#errors-and-diagnostics). While
these rules can sometimes make glob expressions a bit more difficult to compose,
they also make glob expressions more consistent, easier to reason about, and
less prone to errors.

## Patterns

Globs resemble Unix paths, but additionally support patterns that can be matched
against paths and directory trees. Patterns use a syntax that resembles globbing
in Unix shells and tools like `git`, though there are some important
differences.

```rust
use wax::Glob;

let glob = Glob::new("**/*.{go,rs}").unwrap();
assert!(glob.is_match("src/lib.rs"));
```

Patterns form captures that can be used to extract matched text (as seen in many
regular expression engines). In the above example, there are three patterns that
can be queried for matched text: `**/`, `*`, and `{go,rs}`. Every glob
expression has an implicit capture for the complete matched text.

Globs use a consistent and opinionated format and patterns are **not**
configurable; the semantics of a particular glob are always the same. For
example, `*` **never** matches across component boundaries. Components are an
important part of paths and file system trees, and only the tree wildcard `**`
(see below) implicitly matches across them.

### Wildcards

Wildcards match some amount of arbitrary text in paths and are the most
fundamental pattern provided by globs (and likely the most familiar).

The zero-or-more wildcards `*` and `$` match zero or more of any character
within a component (**never path separators**). Zero-or-more wildcards cannot be
adjacent to other zero-or-more wildcards. The `*` wildcard is eager and will
match the longest possible text while the `$` wildcard is lazy and will match
the shortest possible text. When followed by a literal, `*` stops at the last
occurrence of that literal while `$` stops at the first occurence.

The exactly-one wildcard `?` matches any single character within a component
(**never path separators**). Exactly-one wildcards do not group automatically,
so a pattern of contiguous wildcards such as `???` form distinct captures for
each `?` wildcard. [An alternation](#alternations) can be used to group
exactly-one wildcards into a single capture, such as `{???}`.

The tree wildcard `**` matches any characters across zero or more components.
**This is the only pattern that implicitly matches across arbitrary component
boundaries**; all other patterns do **not** implicitly match across component
boundaries. When a tree wildcard participates in a match and does not terminate
the pattern, its captured text includes the trailing separator. If a tree
wildcard does not participate in a match, then its captured text is an empty
string.

Tree wildcards must be delimited by forward slashes or terminations (the
beginning and/or end of an expression). **Tree wildcards and path separators are
distinct** and any adjacent forward slashes that form a tree wildcard are parsed
together. Rooting forward slashes in tree wildcards are meaningful and the glob
expressions `**/*.txt` and `/**/*.txt` differ in that the former is relative
(has no root) and the latter has a root.

If a glob expression consists solely of a tree wildcard, then it matches any and
all paths and the complete contents of any and all directory trees, including
the root.

### Character Classes

Character classes match any single character from a group of literals and ranges
within a component (**never path separators**). Classes are delimited by square
brackets `[...]`. Individual character literals are specified as is, such as
`[ab]` to match either `a` or `b`. Character ranges are formed from two
characters separated by a hyphen, such as `[x-z]` to match `x`, `y`, or `z`.
Character classes match characters exactly and are always case-sensitive, so the
expressions `[ab]` and `{a,b}` are not necessarily the same.

Any number of character literals and ranges can be used within a single
character class. For example, `[qa-cX-Z]` matches any of `q`, `a`, `b`, `c`,
`X`, `Y`, or `Z`.

Character classes may be negated by including an exclamation mark `!` at the
beginning of the class pattern. For example, `[!a]` matches any character except
for `a`. **These are the only patterns that support negation.**

It is possible to escape meta-characters like `*`, `$`, etc., using character
classes though globs also support escaping via a backslash `\`. To match the
control characters `[`, `]`, and `-` within a character class, they must be
escaped via a backslash, such as `[a\-]` to match `a` or `-`.

Character classes have notable platform-specific behavior, because they match
arbitrary characters in native paths but never match path separators. This means
that if a character class consists of **only** path separators on a given
platform, then the character class is considered empty and matches nothing. For
example, in the expression `a[/]b` the character class `[/]` matches nothing on
Unix and Windows. Such character classes are not rejected, because the role of
arbitrary characters depends on the platform. In practice, this is rarely a
concern, but **such patterns should be avoided**.

Character classes have limited utility on their own, but compose well with
[repetitions](#repetitions).

### Alternations

Alternations match an arbitrary sequence of one or more comma separated
sub-globs delimited by curly braces `{...,...}`. For example, `{a?c,x?z,foo}`
matches any of the alternative globs `a?c`, `x?z`, or `foo`. Alternations may be
arbitrarily nested and composed with [repetitions](#repetitions).

Alternations form a single capture group regardless of the contents of their
sub-globs. This capture is formed from the complete match of the sub-glob, so if
the alternation `{a?c,x?z}` matches the path `abc`, then the captured text will
be `abc` (**not** `b`). Alternations can be used to group captures using a
single sub-glob, such as `{*.{go,rs}}` to capture an entire file name with a
particular extension or `{???}` to group a sequence of exactly-one wildcards.

Alternations must consider adjacency rules and neighboring patterns. For
example, `*{a,b*}` is allowed but `*{a,*b}` is not. Additionally, they may not
contain a sub-glob consisting of a singular tree wildcard `**` and cannot root a
glob expression as this could cause the expression to match or walk overlapping
trees.

### Repetitions

Repetitions match a sub-glob a specified number of times. Repetitions are
delimited by angle brackets with a separating colon `<...:...>` where a sub-glob
precedes the colon and an optional bounds specification follows it. For example,
`<a*/:0,>` matches the sub-glob `a*/` zero or more times. Though not implicit
like tree [wildcards](#wildcards), **repetitions can match across component
boundaries** (and can themselves include tree wildcards). Repetitions may be
arbitrarily nested and composed with [alternations](#alternations).

Bound specifications are formed from inclusive lower and upper bounds separated
by a comma `,`, such as `:1,4` to match between one and four times. The upper
bound is optional and may be omitted. For example, `:1,` matches one or more
times (note the trailing comma `,`). A singular bound is convergent, so `:3`
matches exactly three times (both the lower and upper bounds are three). If no
lower or upper bound is specified, then the sub-glob matches one or more times,
so `<a:>` and `<a:1,>` are equivalent. Similarly, if the colon `:` is also
omitted, then the sub-glob matches zero or more times, so `<a>` and `<a:0,>` are
equivalent.

Repetitions form a singular capture group regardless of the contents of their
sub-glob. The capture is formed from the complete match of the sub-glob. If the
repetition `<abc/>` matches `abc/abc/`, then the captured text will be
`abc/abc/`.

Repetitions compose well with [character classes](#character-classes). Most
often, a glob expression like `{????}` is sufficient, but the more specific
expression `<[0-9]:4>` further constrains the matched characters to digits, for
example. Repetitions may also be more terse, such as `<?:8>`. Furthermore,
repetitions can form tree expressions that further constrain components, such as
`<[!.]*/>[!.]*` to match paths that contain no leading dots `.` in any
component.

Repetitions must consider adjacency rules and neighboring patterns. For example,
`a/<b/**:1,>` is allowed but `<a/**:1,>/b` is not. Additionally, they may not
contain a sub-glob consisting of a singular separator `/`, a singular
zero-or-more wildcard `*` or `$`, nor a singular tree wildcard `**`. Repetitions
with a lower bound of zero may not root a glob expression, as this could cause
the expression to match or walk overlapping trees.

## Combinators

Glob patterns can be combined and matched together using the `any` combinator.
`any` accepts an `IntoIterator` of `Pattern`s, such as compiled `Program`s like
`Glob` or pattern text like `str` slices. The output is an `Any`, which
implements `Program` and efficiently matches any of its input patterns.

```rust
use wax::{Glob, Program};

let any = wax::any(["**/*.txt", "src/**/*.rs"]).unwrap();
assert!(any.is_match("src/lib.rs"));
```

`Any` and the `any` combinator can be used anywhere a `Pattern` or `Program`
can.

```rust
use wax::walk::FileIterator;
use wax::Glob;

let glob = Glob::new("**/*.{md,rs,toml,txt,yaml,yml}").unwrap();
for entry in glob
    .walk("projects")
    .not(wax::any([
        "**/{.git,.github,target}/**",
        "**/{lib,main,mod}.rs",
    ]))
    .unwrap()
{
    let entry = entry.unwrap();
    // ...
}
```

Unlike `Glob`, an `Any` cannot be matched against a directory tree (as with
`Glob::walk`). However, `Any` supports features that alternations do not and
`Glob`s cannot represent, such as overlapping trees.

## Flags and Case Sensitivity

Flags toggle the matching behavior of globs. Importantly, flags are a part of a
glob expression rather than an API. Behaviors are toggled immediately following
flags in the order in which they appear in glob expressions. Flags are delimited
by parenthesis with a leading question mark `(?...)` and may appear anywhere
within a glob expression so long as they do not split tree wildcards (e.g.,
`a/*(?i)*` is not allowed). Each flag is represented by a single character and
can be negated by preceding the corresponding character with a minus `-`. Flags
are toggled in the order in which they appear within `(?...)`.

The only supported flag is the case-insensitivty flag `i`. By default, glob
expressions use the same case sensitivity as the target platforms's file system
APIs (case-sensitive on Unix and case-insensitive on Windows), but `i` can be
used to toggle this explicitly as needed. For example,
`(?-i)photos/**/*.(?i){jpg,jpeg}` matches file paths beneath a `photos`
directory with a case-**sensitive** base and a case-**insensitive** extension
`jpg` or `jpeg`.

Wax considers literals, their configured case sensitivity, and the case
sensitivity of the target platform's file system APIs [when partitioning glob
expressions](#partitioning-and-semantic-literals) with `Glob::partition`.
Partitioning is unaffected in glob expressions with no flags.

## Errors and Diagnostics

The `GlobError` type represents error conditions that can occur when building a
pattern or walking a directory tree. `GlobError` and its sub-errors implement
the standard `Error` and `Display` traits via [`thiserror`][thiserror].

Wax optionally integrates with the [`miette`][miette] crate, which can be used
to capture and display diagnostics. This can be useful for reporting errors to
users that provide glob expressions. When enabled, error types implement the
`Diagnostic` trait.

```
Error: wax::glob::adjacent_zero_or_more

  x malformed glob expression: adjacent zero-or-more wildcards `*` or `$`
   ,----
 1 | doc/**/*{.md,.tex,*.txt}
   :        |^^^^^^^^|^^^^^^^
   :        |        | `-- here
   :        |        `-- in this alternation
   :        `-- here
   `----
```

Wax also provides inspection APIs that allow code to query glob metadata, such
as captures and variance.

```rust
use wax::Glob;

let glob = Glob::new("videos/**/{*.{mp4,webm}}").unwrap();
assert_eq!(2, glob.captures().count());
```

## Cargo Features

Wax provides some optional integrations and features that can be toggled via
the Cargo features described below.

| Feature  | Default | Dependencies       | Description                                                                   |
|----------|---------|--------------------|-------------------------------------------------------------------------------|
| `miette` | No      | `miette`, `tardar` | Integrates with `miette` and provides `Diagnostic` error types and reporting. |
| `walk`   | Yes     | `walkdir`          | Provides APIs for matching globs against directory trees.                     |

Features can be configured in a crate's `Cargo.toml` manifest.

```toml
[dependency.wax]
version = "^0.x.0"
default-features = false
features = [
    "miette",
    "walk"
]
```

## Unsupported Path Features

Any components not recognized as separators nor patterns are interpreted as
literals. In combination with strict rules, this means **some platform-specific
path features cannot be used directly in globs**. This limitation is by design
and additional code may be necessary to bridge this gap for some use cases.

### Partitioning and Semantic Literals

Globs support no notion of a current or parent directory. The path components
`.` and `..` are interpreted as literals and only match paths with the
corresponding components (even on Unix and Windows). For example, the glob
`src/../*.rs` matches the path `src/../lib.rs` but does **not** match the
semantically equivalent path `lib.rs`.

Parent directory components have unclear meaning and far less utility when they
follow patterns in a glob. However, such components are intuitive and are often
important for escaping a working directory when they precede variant patterns
(i.e., as a prefix). For example, the glob `../src/**/*.rs` has more obvious
intended meaning than the glob `src/**/../*.rs`. As seen above though, the first
glob would only match the literal path component `..` and not paths that replace
this with a parent directory.

`Glob::partition` can be used to isolate semantic components that precede
patterns and apply semantic path operations to them (namely `..`).
`Glob::partition` partitions a glob into an invariant `PathBuf` prefix and a
variant `Glob` postfix. Here, invariant means that the partition contains no
glob patterns that resolve differently than an equivalent native path using the
target platform's file system APIs. The prefix can be used as needed in
combination with the glob.

```rust
use dunce; // Avoids UNC paths on Windows.
use std::path::Path;
use wax::{Glob, Program};

let path: &Path = /* ... */ // Candidate path.

let directory = Path::new("."); // Working directory.
let (prefix, glob) = Glob::new("../../src/**").unwrap().partition();
let prefix = dunce::canonicalize(directory.join(&prefix)).unwrap();
if dunce::canonicalize(path)
    .unwrap()
    .strip_prefix(&prefix)
    .map(|path| glob.is_match(path))
    .unwrap_or(false)
{
    // ...
}
```

Additionally, `Glob::has_semantic_literals` can be used to detect literal
components in a glob that have special semantics on the target platform. When
the `miette` feature is enabled, such literals are reported as warnings.

```rust
use wax::Glob;

let glob = Glob::new("../**/src/**/main.rs").unwrap();
assert!(glob.has_semantic_literals());
```

### Schemes and Prefixes

While globs can be rooted, they cannot include schemes nor Windows path
prefixes. For example, the Windows UNC share path `\\server\share\src` cannot be
represented directly as a glob.

This can be limiting, but the design of Wax explicitly forbids this: Windows
prefixes and other volume components are not portable. Instead, when this is
needed, an additional native path or working directory must be used, such as
[the `--tree` option provided by Nym][nym]. In most contexts, globs are applied
relative to some such working directory.

### Non-nominal Constraints

Globs are strictly nominal and do not support any non-nominal constraints. It is
not possible to directly filter or otherwise select paths or files based on
additional metadata (such as a modification timestamp) in a glob expression.
However, it is possible for user code to query any such metadata for a matching
path or effeciently apply such filtering when matching directory trees using
`FileIterator::filter_tree`.

For such additional features, including metadata filters and transformations
using matched text, see [Nym][nym].

### Encoding

Globs operate exclusively on UTF-8 encoded text. However, this encoding is not
used for paths on all platforms. Wax uses the `CandidatePath` type to re-encode
native paths via lossy conversions that use Unicode replacement codepoints
whenever a part of a path cannot be represented as valid UTF-8. In practice,
most paths can be losslessly encoded in UTF-8, but this means that Wax cannot
match nor capture some literal byte strings.

## Stability

At the time of writing, Wax is experimental and unstable. It is possible that
glob expression syntax and semantics may change between versions in the `0.y.z`
series without warning nor deprecation.

[miette]: https://github.com/zkat/miette
[nym]: https://github.com/olson-sean-k/nym
[thiserror]: https://github.com/dtolnay/thiserror
