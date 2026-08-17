# Setup

On this page, you will find all the information needed to run and test your
own version of the Logos crate, locally.

We assume you have basic knowledge with git and GitHub. If that is not the
case, please refer to the link mentioned in [Contributing](./contributing.md).

## Prerequisites

You need to have both git and Rust installed on your computer,
see installation procedures:

+ for [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git);
+ and [Rust](https://www.rust-lang.org/tools/install).

Once it's done, clone the Logos repository on your computer:

```bash
git clone https://github.com/maciejhirsz/logos.git
```

If you have a fork of this repository, make sure to clone it instead.

Finally, launch a terminal (i.e., command-line) session and go to the
`logos` directory.

## Checking the code compiles

A good way to see if you code can compile is to use the eponym command:

```bash
cargo check --workspace
```

## Formatting and linting your code

Prior to suggesting changes in a pull request, it is important to both
format your code:

```bash
cargo fmt
```

and check against Rust's linter:

```bash
cargo clippy
```

Make sure to run those frequently, otherwise your pull request will probably
fail to pass the automated tests.

## Testing your code

A code that compiles isn't necessarily correct, and testing it against known
cases is of good practice:

```bash
cargo test --workspace
```

You can also run benchmarks:

```bash
cargo bench --workspace --benches
```

## Building the documentation

Logos' documentation needs to be built with Rust's nightly toolchain.

You can install the latest nightly channel with:

```bash
rustup install nightly
```

Then, use the following command to build the documentation with a similar
configuration to the one used by [docs.rs](https://docs.rs/logos/latest/logos/):

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc \
    --features debug \
    -Zunstable-options \
    -Zrustdoc-scrape-examples \
    --no-deps \
    --open \
```


## Building the book

Logos' book can be built with mbBook.

This tool can be installed with `cargo`:

```bash
cargo install mdbook
```

You also need to install `mdbook-admonish` and its assets:

```bash
cargo install mdbook admonish
cd book/  # You must run the next command from the book/ directory
mdbook-admonish install
```

Then, you can build the book with:

```bash
mbook serve book --open
```

Any change in the `./book` folder will automatically trigger a new build,
and the pages will be live-reloaded.
