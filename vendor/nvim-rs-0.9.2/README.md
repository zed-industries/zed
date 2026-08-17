# nvim-rs ![CI](https://github.com/KillTheMule/nvim-rs/actions/workflows/ci.yml/badge.svg)  [![(Docs.rs)](https://docs.rs/nvim-rs/badge.svg)](https://docs.rs/nvim-rs/) [![(Crates.io status)](https://img.shields.io/crates/v/nvim-rs.svg)](https://crates.io/crates/nvim-rs)
Rust library for Neovim msgpack-rpc clients. Utilizes async to allow for arbitrary nesting of requests.

## Status

Useable, see the `examples/` and `tests/` folders for examples. The `nvim_rs::examples` submodule contains documentation of the examples.

The **API** is unstable, see the [Roadmap](https://github.com/KillTheMule/nvim-rs/issues/1) for things being planned.

## Contributing

I'd love contributions, comments, praise, criticism... You could open an [issue](https://github.com/KillTheMule/nvim-rs/issues) or a [pull request](https://github.com/KillTheMule/nvim-rs/pulls). I also read the subreddits for [rust](https://www.reddit.com/r/rust/), if that suits you better.

## Running tests

For some tests, neovim needs to be installed. Set the environment variable `NVIMRS_TEST_BIN` to
the path of the binary before running the tests.

Afterwards, you can simply run `cargo test --features="use_tokio"`.
Also run `cargo build --examples --features="use_tokio"` as well as `cargo
bench -- --test --features="use_tokio"` to make sure everything still compiles
(replace `use_tokio` by `use_async-std` to do all the above with `async-std`
instead of `tokio`). 

## License

As this is a fork of [neovim-lib](https://github.com/daa84/neovim-lib), it is licensed under the GNU Lesser General Public License v3.0.

**IMPORTANT**: All commits to this project, including all PRs, are
dual-licensed under the Apache or MIT license. This is to allow the possibility
of relicensing this project later.

## CoC

Wherever applicable, this project follows the [rust code of
conduct](https://www.rust-lang.org/en-US/conduct.html).
