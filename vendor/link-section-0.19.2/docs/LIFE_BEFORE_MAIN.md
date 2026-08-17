# Life-Before-Main and Other Link-Time Hazards

The linker is a powerful tool that can be used to perform a number of tricks,
but you, the developer, should be aware of the hazards that come with this
low-level control.

Rust is a systems programming language that is designed to be safe, and
link-time tools may bend or break some safety guarantees.

 - [Rust’s model vs `ctor`/`dtor`](#rusts-model-vs-ctordtor)
    - [Panic handling](#panic-handling)
    - [I/O and the Standard Library](#io-and-the-standard-library)
    - [Aggressive Linker Garbage Collection](#aggressive-linker-garbage-collection)
 - [cdylib lifecycle](#cdylib-lifecycle)

## Rust’s model vs `ctor`/`dtor`

Rust’s usual model is that nothing runs before or after `main`. The `ctor` and
`dtor` crates deliberately subvert that
([*UCG* #598](https://github.com/rust-lang/unsafe-code-guidelines/issues/598)).

Code inside `#[ctor]` and `#[dtor]` should favor **`libc`-level APIs** and logic
that does **not depend on the Rust standard library** having finished
initializing (constructors) or still being valid (destructors).

[*UCG* #598](https://github.com/rust-lang/unsafe-code-guidelines/issues/598) — Rust `unsafe-code-guidelines`; discussion thread on lifecycle and unsafe-code implications around execution outside `main`.

### Panic handling

Panic handlers may not be set up in early code, so a `panic!()` may not be
catchable, or may even result in undefined behavior. Generally, code that runs
before main must take great pains not to panic ([🦀
#97049](https://github.com/rust-lang/rust/issues/97049), [🦀
#107381](https://github.com/rust-lang/rust/issues/107381), [🦀
#86030](https://github.com/rust-lang/rust/issues/86030)).

_References_

[🦀 #97049](https://github.com/rust-lang/rust/issues/97049) — `rust-lang/rust`; Miri and discussion of panicking inside `#[start]` before the runtime can catch unwinding.

[🦀 #107381](https://github.com/rust-lang/rust/issues/107381) — `rust-lang/rust`; unwinding and whether escaping `lang_start` is undefined behavior.

[🦀 #86030](https://github.com/rust-lang/rust/issues/86030) — `rust-lang/rust`; `lang_start` soundness when panic payloads panic during drop (`failed to initiate panic`).

### I/O and the Standard Library

`std::io` is known to be problematic after main
([🦀 #29488](https://github.com/rust-lang/rust/issues/29488),
[SO #35980148](https://stackoverflow.com/questions/35980148/why-does-an-atexit-handler-panic-when-it-accesses-stdout)).
`println!` uses thread-local stdout machinery which may not be initialized yet, or may be
closed/broken after `main` exits. The
[`libc_print`](https://crates.io/crates/libc-print) crate provides a safe
alternative for this that uses `libc` functions directly and will not panic if
used before or after `main`.

The standard library does not make any particular guarantees about the state of
the system after `main` exits or before it starts and code that works in one
version of Rust may or may not work in another.

_References_

[🦀 #29488](https://github.com/rust-lang/rust/issues/29488) — `rust-lang/rust`; `println!` panics from `Drop` / TLS (`cannot access stdout during shutdown`).

[SO #35980148](https://stackoverflow.com/questions/35980148/why-does-an-atexit-handler-panic-when-it-accesses-stdout) — `println!` / stdout access from a libc `atexit` handler vs Rust runtime teardown order.

## Aggressive Linker Garbage Collection

Some linker configurations can **strip** the underlying registration data for
`#[ctor]`, `#[dtor]` and `#[in_section]` registrations from the final binary,
resulting in them not being called
([rust-ctor #280](https://github.com/mmastrac/linktime/issues/280),
[🦀 #99721](https://github.com/rust-lang/rust/issues/99721)).

Building with `--cfg linktime_used_linker` for the `ctor` and `dtor` crates
_may_ help — it applies `used(linker)` to the linker-generated items, but it
requires nightly Rust and `#![feature(used_with_arg)]` on the crate root.

Often a **`use` of the module** that contains the missing registration is enough
for the linker to retain the code.

_References_

[rust-ctor #280](https://github.com/mmastrac/linktime/issues/280) — `mmastrac/linktime`; linker / LTO stripping ctor registrations.

[🦀 #99721](https://github.com/rust-lang/rust/issues/99721) — `rust-lang/rust`; rustc / linkage behavior relevant to similar stripping (`used`, linker GC).

## `cdylib` lifecycle

A `cdylib` is a dynamic library that is loaded (and potentially unloaded) at
runtime, independent of the main executable.

On some platforms, **unloading a shared library may not occur when you expect**;
behavior can be deferred until **process exit**. The rules are described as
arcane. **Thread-local storage on macOS** is called out as influencing this; see
[this comment on 🦀 #28794](https://github.com/rust-lang/rust/issues/28794#issuecomment-368693049).

Care should be taken to ensure that the `#[dtor]` functions are called before
the library is unloaded. While the `#[dtor]` macro supports registering
"termination" functions - which are called when the main binary process
terminates - inside of `cdylib`s, it is not recommended to use them as the code
that will perform the cleanup may have been unloaded and unmapped from memory,
causing random crashes.

_References_

[🦀 #28794 (comment)](https://github.com/rust-lang/rust/issues/28794#issuecomment-368693049) — `rust-lang/rust`; thread-local storage and dynamic-library unload behavior on macOS.
