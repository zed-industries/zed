### Version 0.2.6

- fix: ON UPDATE with two many blank formatted incorrectly (#46)
- fix: `EXCEPT` not handled well
- fix: REFERENCES xyz ON UPDATE .. causes formatter to treat the remaining as an UPDATE statement
- fix: Escaped strings formatted incorrectly
- fix: RETURNING is not placed on a new line
- fix: fix the issue of misaligned comments after formatting (#40)

### Version 0.2.4

- Remove `itertools` dependency [#34](https://github.com/shssoichiro/sqlformat-rs/pull/34)

### Version 0.2.3

- Allow alphanumeric characters in SQLite style parameters [#32](https://github.com/shssoichiro/sqlformat-rs/pull/32)
- Format "begin" and "declare" for PLPgSql [#30](https://github.com/shssoichiro/sqlformat-rs/pull/30)
- Allow scientific notation with or without "+"/"-" [#31](https://github.com/shssoichiro/sqlformat-rs/pull/31)
- Treat "$$" as a reserved token that sits on its own line [#29](https://github.com/shssoichiro/sqlformat-rs/pull/29)
- Bump itertools to version 0.12 [#28](https://github.com/shssoichiro/sqlformat-rs/pull/28)

### Version 0.2.2

- Fix a performance issue where the tokenizer would run in O^2
  time [#24](https://github.com/shssoichiro/sqlformat-rs/pull/24)

### Version 0.2.1

- Fix extra spaces inside of scientific notation [#16](https://github.com/shssoichiro/sqlformat-rs/pull/16)
- Remove unnecessary space in BETWEEN clause [#17](https://github.com/shssoichiro/sqlformat-rs/pull/17)
- Denote the minimum Rust version in Cargo.toml

### Version 0.2.0

- Fix extra spaces in string escaping [#13](https://github.com/shssoichiro/sqlformat-rs/pull/13)
- Fix panic on overflowing integer [#14](https://github.com/shssoichiro/sqlformat-rs/pull/14)
- Bump Rust edition to 2021
  - This is technically a breaking change as it bumps the minimum Rust version to 1.56

### Version 0.1.8

- Remove regex dependency
- Remove unused maplit dependency

### Version 0.1.7

- Bump nom to 7.0, which reportedly also fixes some build issues

### Version 0.1.6

- Fix compatibility with Rust 1.44 which was broken in 0.1.5

### Version 0.1.5

- Fix a possible panic on multibyte unicode strings

### Version 0.1.4

- Attempt again to fix the issue some users experience where this crate would fail to compile

### Version 0.1.3

- Fix an issue some users experienced where this crate would fail to compile

### Version 0.1.2

- Rewrite the parser in nom, providing significant performance improvements across the board
- Other significant performance improvement on pathological queries

### Version 0.1.1

- Significant performance improvements

### Version 0.1.0

- Initial release
