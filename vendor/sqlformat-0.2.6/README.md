# sqlformat

[![Build Status](https://github.com/shssoichiro/sqlformat-rs/workflows/sqlformat/badge.svg)](https://github.com/shssoichiro/sqlformat-rs/actions?query=branch%3Amaster)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Version](https://img.shields.io/crates/v/sqlformat.svg)](https://crates.io/crates/sqlformat)
[![Docs](https://docs.rs/sqlformat/badge.svg)](https://docs.rs/sqlformat)

This crate is a port of https://github.com/kufii/sql-formatter-plus
written in Rust. It is intended to be usable as a pure-Rust library
for formatting SQL queries.

There is currently no binary interface.
This crate was written for formatting queries to logs
within `sqlx`, but it may be useful to other crates
in the Rust ecosystem.
