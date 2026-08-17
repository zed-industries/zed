# redox_users <a href="https://crates.io/crates/redox_users"><img src="https://img.shields.io/crates/v/redox_users.svg"></a>

Redox OS APIs for accessing users and groups information. [Documentation](https://docs.rs/redox_users/0.1.0/redox_users/)

High level APIs for:

- Getting the current process effective user ID.
- Getting the current process user ID.
- Getting the current process effective group ID.
- Getting the current process group ID.
- Manipulating User and Group information (including adding, removing, and modifying groups and users, in addition to other functionality, see docs)

We recommend to use these APIs instead of directly manipulating the
`/etc/group` and `/etc/passwd` as this is an implementation detail and
might change in the future.

Note that redox_users is an API designed only for use on Redox. It compiles on other platforms (for testing), but it will not work and might produce unexpected behavior.

## Hashing
redox_users uses the Argon2 hashing algorithm. The default hashing parameters are as follows:
```Rust
Argon2::new(10, 1, 4096, Variant::Argon2i)
```
