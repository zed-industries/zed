# Version 0.2.13

- Bump MSRV to 1.71. (#55)
- Update to `windows-sys` v0.61. (#55)

# Version 0.2.12

- Update windows-sys to v0.60. (#51)

# Version 0.2.11

- Update rustix to 1.0.7. (#49)

# Version 0.2.10

- Update windows-sys to v0.59. (#41)

# Version 0.2.9

- Fix the `clippy::needless_borrows_for_generic_args` lint in the README
  example. (#38)

# Version 0.2.8

- Update README.md to use a working example. (#35)

# Version 0.2.7

- Remove an invalid category from `Cargo.toml`. (#33)

# Version 0.2.6

- Bump `windows-sys` to 0.52 and `async-io` to 3.3.0. (#27)

# Version 0.2.5

- Bump `async-io` to version 2.0.0. (#25)

# Version 0.2.4

- Add `LICENSE` files to this crate. (#23)

# Version 0.2.3

- Remove the `signalfd` backend, as it offered little to no advantages over the pipe-based backend and it didn't catch signals sometimes. (#20)

# Version 0.2.2

- Fix build error on Android. (#18)

# Version 0.2.1

- Add support for the signalfd mechanism on Linux. (#5)
- Add support for Windows. (#6)
- Bump MSRV to 1.63. (#8)

# Version 0.2.0

- Initial release
