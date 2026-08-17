# How to contribute

I'm really glad you're reading this, because we need volunteer developers to
help this project continue to grow and improve.

1. file [bugs](../../issues/new?assignees=&labels=bug&template=bug_report.md) and [enhancement requests](../../issues/new?assignees=&labels=enhancement&template=feature_request.md)
2. review the project documentation know if you find are issues, or missing
   content, there
3. Fix or Add something and send us a pull request; you may like to pick up one
   of the issues marked [help wanted](../../labels/help%20wanted) or [good first issue](../../labels/good%20first%20issue) as an introduction.
   Alternatively, [documentation](../../labels/documentation) issues can be a great way to understand the
   project and help improve the developer experience.

## Submitting changes


We love pull requests from everyone. By participating in this project, you agree
to abide by our [code of conduct](./CODE_OF_CONDUCT.md), and [License](./LICENSE).

Fork, then clone the repo:

```
git clone git@github.com:johnstonskj/{{repository-name}}.git
```

Ensure you have a good Rust install, usually managed by [Rustup](https://rustup.rs/).
You can ensure the latest tools with the following:

```
rustup update
```

Make sure the tests pass:

```
cargo test --package {{package-name}} --no-fail-fast -- --exact
cargo test --package {{package-name}} --no-fail-fast --all-features -- --exact
cargo test --package {{package-name}} --no-fail-fast --no-default-features -- --exact
```

Make your change. Add tests, and documentation, for your change. For tests
please add a comment of the form:

```rust
#[test]
// Regression test: GitHub issue #11
// or
// Feature test: GitHub PR: #15
fn test_something() { }
```

Ensure not only that tests pass, but the following all run successfully.

```
cargo doc --all-features --no-deps
cargo fmt
cargo clippy
```

If you have made any changes to `Cargo.toml`, also check:

```
cargo outdated --depth 1
cargo audit
```

Push to your fork and [submit a pull request](../../compare/) using our [template](./pull_request_template.md).

At this point you're waiting on us. We like to at least comment on pull requests
within three business days (and, typically, one business day). We may suggest
some changes or improvements or alternatives.

Some things that will increase the chance that your pull request is accepted:

* Write unit tests.
* Write API documentation.
* Write a [good commit message](https://cbea.ms/git-commit/https://cbea.ms/git-commit/).

## Coding conventions

The primary tool for coding conventions is rustfmt, and specifically `cargo fmt`
is a part of the build process and will cause Actions to fail.

DO NOT create or update any existing `rustfmt.toml` file to change the default
formatting rules.

DO NOT alter any `warn` or `deny` library attributes.

DO NOT add any `feature` attributes that would prohibit building on the stable
channel. In some cases new crate-level features can be used to introduce an
unstable feature dependency but these MUST be clearly documented and optional.
