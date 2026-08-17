# Version 0.2.5

- Fix a bug where the `tokio` runtime would disappear. (#38)

# Version 0.2.4

- Derive `Clone` for `Compat`. (#27)
- Rather than spawning our own `tokio` runtime all of the time, reuse an
  existing runtime if possible. (#30)

# Version 0.2.3

- Enter the `tokio` context while dropping wrapped `tokio` types. (#22)

# Version 0.2.2

- Add `smol-rs` logo to the docs. (#19)

# Version 0.2.1

- Fix a bug in `AsyncSeek` implementation.

# Version 0.2.0

- Update tokio to v1.

# Version 0.1.5

- Add tokio v0.3 support.

# Version 0.1.4

- Add `Compat::get_ref()`.
- Add `Compat::get_mut()`.

# Version 0.1.3

- More elaborate docs.

# Version 0.1.2

- More docs.

# Version 0.1.1

- More docs.

# Version 0.1.0

- Initial version
