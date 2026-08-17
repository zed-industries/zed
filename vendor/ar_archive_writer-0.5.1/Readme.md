# A writer for object file ar archives

This is a Rust port of LLVM's archive writer (see [the LLVM Reference](reference/Readme.md)
for details) with the following options removed:
* Deterministic: always enabled.
* Symbol tables: always enabled.

## License

Licensed under Apache License v2.0 with LLVM Exceptions
([LICENSE.txt](LICENSE.txt) or https://llvm.org/LICENSE.txt)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
