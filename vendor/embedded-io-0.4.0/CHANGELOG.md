# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0 - 2022-11-25

- Switch all traits to use [`async_fn_in_trait`](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) (AFIT). Requires `nightly-2022-11-22` or newer.

## 0.3.1 - 2022-10-26

- Fix compilation on recent nightlies (#5)

## 0.3.0 - 2022-05-19

- `FromFutures` adapter now requires `futures` Cargo feature. (breaking change)
- Add `FromTokio` adapter.
- Add blanket impls for `&mut T`, `Box<T>`.
- Add impl `Read`, `BufRead` for `&[u8]`
- Add impl `Write` for `&mut [u8]`
- Add impl `Write` for `Vec<u8>`
- impl `std::error::Error` for `ReadExactError`, `WriteFmtError`.

## 0.2.0 - 2022-05-07

- First release