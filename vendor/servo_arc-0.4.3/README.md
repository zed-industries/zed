Stylo
=====

**High-Performance CSS Style Engine**

[![Build Status](https://github.com/servo/stylo/actions/workflows/main.yml/badge.svg)](https://github.com/servo/stylo/actions)
[![Crates.io](https://img.shields.io/crates/v/stylo.svg)](https://crates.io/crates/stylo)
[![Docs](https://docs.rs/stylo/badge.svg)](https://docs.rs/stylo)
![Crates.io License](https://img.shields.io/crates/l/stylo)

Stylo is a high-performance, browser-grade CSS style engine written in Rust that powers [Servo](https://servo.org) and [Firefox](https://firefox.com). This repo contains Servo’s downstream version of Stylo. The upstream version lives in mozilla-central with the rest of the Gecko/Firefox codebase.

Coordination of Stylo development happens:

- Here in Github Issues
- In the [#stylo](https://servo.zulipchat.com/#narrow/channel/417109-stylo) channel of the [Servo Zulip](https://servo.zulipchat.com/)
- In the [#layout](https://chat.mozilla.org/#/room/#layout:mozilla.org) room of the Mozilla Matrix instance (matrix.mozilla.org)

## High-Level Documentation

- This [Mozilla Hacks article](https://hacks.mozilla.org/2017/08/inside-a-super-fast-css-engine-quantum-css-aka-stylo) contains a high-level overview of the Stylo architecture.
- There is a [chapter](https://book.servo.org/architecture/style.html) in the Servo Book (although it is a little out of date).

## Branches

The branches are as follows:

- [**upstream**](https://github.com/servo/style/tree/upstream) has upstream [mozilla-central](https://searchfox.org/mozilla-central/source/servo) filtered to the paths we care about ([style.paths](style.paths)), but is otherwise unmodified.
- [**main**](https://github.com/servo/style/tree/ci) adds our downstream patches, plus the scripts and workflows for syncing with mozilla-central on top of **upstream**.

> [!WARNING]
> This repo syncs from upstream by creating a new branch and then rebasing our changes on top of it. This means that `git pull` will not work across syncs (you will need to use `git fetch`, `git reset` and `git rebase`).

More information on the syncing process is available in [SYNCING.md](SYNCING.md)

## Crates

A guide to the crates contained within this repo

### Stylo Crates

These crates are largely implementation details of Stylo, although you may need to use some of them directly if you use Stylo.

| Directory          | Crate                                                                                                               | Notes                                                                                                             |
| ---                | ---                                                                                                                 | ---                                                                                                               |
| style              | [![Crates.io](https://img.shields.io/crates/v/stylo.svg)](https://crates.io/crates/stylo)                           | The main Stylo crate containing the entire CSS engine                                                             |
| style_traits       | [![Crates.io](https://img.shields.io/crates/v/stylo_traits.svg)](https://crates.io/crates/stylo_traits)             | Types and traits which allow other code to interoperate with Stylo without depending on the main crate directly.  |
| stylo_dom          | [![Crates.io](https://img.shields.io/crates/v/stylo_dom.svg)](https://crates.io/crates/stylo_dom)                   | Similar to stylo_traits (but much smaller)                                                                        |
| stylo_atoms        | [![Crates.io](https://img.shields.io/crates/v/stylo_atoms.svg)](https://crates.io/crates/stylo_atoms)               | [Atoms](https://docs.rs/string_cache/latest/string_cache/struct.Atom.html) for CSS and HTML event related strings |
| stylo_config       | [![Crates.io](https://img.shields.io/crates/v/stylo_config.svg)](https://crates.io/crates/stylo_config)             | Configuration for Stylo. Can be used to set runtime preferences (enabling/disabling properties, etc)              |
| stylo_static_prefs | [![Crates.io](https://img.shields.io/crates/v/stylo_static_prefs.svg)](https://crates.io/crates/stylo_static_prefs) | Static configuration for Stylo. Config be overridden by patching in a replacement crate.                          |
| style_derive       | [![Crates.io](https://img.shields.io/crates/v/stylo_derive.svg)](https://crates.io/crates/stylo_derive)             | Internal derive macro for stylo crate                                                                             |

### Standalone Crates

These crates form part of Stylo but are also be useful standalone.

| Directory | Crate                                                                                             | Notes                   |
| ---       | ---                                                                                               | ---                     |
| selectors | [![Crates.io](https://img.shields.io/crates/v/selectors.svg)](https://crates.io/crates/selectors) | CSS Selector matching   |
| servo_arc | [![Crates.io](https://img.shields.io/crates/v/servo_arc.svg)](https://crates.io/crates/servo_arc) | A variant on `std::Arc` |

You may also be interested in the `cssparser` crate which lives in the [servo/rust-cssparser](https://github.com/servo/rust-cssparser) repo.

### Support Crates

Low-level crates which could technically be used standalone but are unlikely to be generally useful in practice.

| Directory       | Crate                                                                                                                   | Notes                                                       |
| ---             | ---                                                                                                                     | ---                                                         |
| malloc_size_of  | [![Crates.io](https://img.shields.io/crates/v/stylo_malloc_size_of.svg)](https://crates.io/crates/stylo_malloc_size_of) | Heap size measurement for Stylo values                      |
| to_shmem        | [![Crates.io](https://img.shields.io/crates/v/to_shmem.svg)](https://crates.io/crates/to_shmem)                         | Internal utility crate for sharing memory across processes. |
| to_shmem_derive | [![Crates.io](https://img.shields.io/crates/v/to_shmem_derive.svg)](https://crates.io/crates/to_shmem_derive)           | Internal derive macro for to_shmem crate                    |

## Building Servo Against a Local Copy of Stylo

Assuming your local `servo` and `stylo` directories are siblings, you can build `servo` against `stylo` by adding the following to `servo/Cargo.toml`:

```toml
[patch."https://github.com/servo/stylo"]
selectors = { path = "../stylo/selectors" }
servo_arc = { path = "../stylo/servo_arc" }
stylo_atoms = { path = "../stylo/stylo_atoms" }
stylo = { path = "../stylo/style" }
stylo_config = { path = "../stylo/stylo_config" }
stylo_dom = { path = "../stylo/stylo_dom" }
stylo_malloc_size_of = { path = "../stylo/malloc_size_of" }
stylo_traits = { path = "../stylo/style_traits" }
```

## Releases

Releases are made every time this repository rebases its changes on top of the latest version of upstream Stylo. There are a lot of crates here. In order to publish them, they must be done in order. One order that works is:

- selectors
- stylo_static_prefs
- stylo_config
- stylo_atoms
- stylo_malloc_size_of
- stylo_dom
- stylo_derive
- stylo_traits
- stylo

## License

Stylo is licensed under MPL 2.0
