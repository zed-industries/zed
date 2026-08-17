# fontdb
[![Build Status](https://github.com/RazrFalcon/fontdb/actions/workflows/main.yml/badge.svg)](https://github.com/RazrFalcon/fontdb/actions/workflows/main.yml)
[![Crates.io](https://img.shields.io/crates/v/fontdb.svg)](https://crates.io/crates/fontdb)
[![Documentation](https://docs.rs/fontdb/badge.svg)](https://docs.rs/fontdb)
[![Rust 1.60+](https://img.shields.io/badge/rust-1.60+-orange.svg)](https://www.rust-lang.org)

`fontdb` is a simple, in-memory font database with CSS-like queries.

## Features

- The database can load fonts from files, directories and raw data (`Vec<u8>`).
- The database can match a font using CSS-like queries. See `Database::query`.
- The database can try to load system fonts.
  Currently, this is implemented by scanning predefined directories.
  The library does not interact with the system API.
- Provides a unique ID for each font face.

## Non-goals

- Advanced font properties querying.<br>
  The database provides only storage and matching capabilities.
  For font properties querying you can use [ttf-parser].

- A font fallback mechanism.<br>
  This library can be used to implement a font fallback mechanism, but it doesn't implement one.

- Application's global database.<br>
  The database doesn't use `static`, therefore it's up to the caller where it should be stored.

- Font types support other than TrueType.

## Font vs Face

A font is a collection of font faces. Therefore, a font face is a subset of a font.
A simple font (\*.ttf/\*.otf) usually contains a single font face,
but a font collection (\*.ttc) can contain multiple font faces.

`fontdb` stores and matches font faces, not fonts.
Therefore, after loading a font collection with 5 faces (for example), the database will be
populated with 5 `FaceInfo` objects, all of which will be pointing to the same file or binary data.

## Performance

The database performance is largely limited by the storage itself.
We are using [ttf-parser], so the parsing should not be a bottleneck.

For example, on Mac Book Pro 14 with M1 Pro, it takes just ~24ms to load 1361 font faces.

## Safety

The library relies on memory-mapped files, which is inherently unsafe.
But since we do not keep the files open it should be perfectly safe.

If you would like to use a persistent memory mapping of the font files,
then you can use the unsafe `Database::make_shared_face_data` function.

## License

MIT

[ttf-parser]: https://github.com/RazrFalcon/ttf-parser
