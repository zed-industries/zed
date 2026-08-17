# Version 1.9.0

- Add `Rng::fill()` (#35, #43)
- Add `#[must_use]` to `Rng::with_seed()` (#46)

# Version 1.8.0

- Add `get_seed()` and `Rng::get_seed()` (#33)

# Version 1.7.0

- Add `char()` and `Rng::char()` (#25)

# Version 1.6.0

- Implement `PartialEq` and `Eq` for `Rng` (#23)

# Version 1.5.0

- Switch to Wyrand (#14)

# Version 1.4.1

- Fix bug when generating a signed integer within a range (#16)

# Version 1.4.0

- Add wasm support.

# Version 1.3.5

- Reword docs.
- Add `Rng::with_seed()`.

# Version 1.3.4

- Implement `Clone` for `Rng`.

# Version 1.3.3

- Forbid unsafe code.

# Version 1.3.2

- Support older Rust versions.

# Version 1.3.1

- Tweak Cargo keywords.

# Version 1.3.0

- Add `f32()` and `f64()`.
- Add `lowercase()`, `uppercase()`, `alphabetic()`, and `digit()`.

# Version 1.2.4

- Switch to PCG XSH RR 64/32.
- Fix a bug in `gen_mod_u128`.
- Fix bias in ranges.

# Version 1.2.3

- Support Rust 1.32.0

# Version 1.2.2

- Use `std::$t::MAX` rather than `$t::MAX` to support older Rust versions.

# Version 1.2.1

- Inline all functions.

# Version 1.2.0

- Add `Rng` struct.

# Version 1.1.0

- Switch to PCG implementation.
- Add `alphanumeric()`.
- Add `seed()`.

# Version 1.0.0

- Initial version
