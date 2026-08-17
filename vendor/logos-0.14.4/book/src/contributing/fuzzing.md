# Fuzzing


Fuzzing is a technique to test a piece of software by injecting randomly generated inputs. This can be pretty useful to discover bugs, as pointed out in [#407](https://github.com/maciejhirsz/logos/pull/407).

**Logos**' fuzzing crate is powered by [afl.rs](https://github.com/rust-fuzz/afl.rs) that
finds panics in **Logos**' methods.

## Usage

First, make sure you have `cargo-afl` installed,
[see the rust-fuzz afl setup guide for installation information](https://rust-fuzz.github.io/book/afl/setup.html).

Next, change your current working directory to be the `fuzz` folder.

### Building

Before fuzzing, you need to build the target with:

```bash
cargo afl build
```

### Fuzzy testing

The recommended way the run tests is with:

```bash
cargo afl fuzz -i in -o out ../target/debug/logos-fuzz
```

Note that it may run for a (very) long time before
it encounter any bug.

## Replaying a Crash

If you happen to find a bug that crashes the program,
you can reply it with

```bash
cargo afl run logos-fuzz < out/default/crashes/crash_file
```

### Reporting a Bug

If you encounter a crash and you feel the error message
is not appropriate,
please report it by opening
[an issue](https://github.com/maciejhirsz/logos/issues/new).
Don't forget to include your crash file so we can later
reproduce it.
