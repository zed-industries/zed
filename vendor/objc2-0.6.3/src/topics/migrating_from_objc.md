# Migrating from the `objc` crate

If size of your project is fairly small, it'll probably be easiest to just
jump straight into replacing everything to use the framework crates.

If your project is large, you can consider upgrading in small steps, following
the changelog at each step of the way. For the most common cases, the
changelogs will include a helpful example on how to upgrade.

As an example you'd start by using `objc2` instead of `objc` in your
`Cargo.toml`:
```toml
[dependencies]
objc = { package = "objc2", version = "0.2.7" }
```

Afterwards, you can upgrade to the next release, in this case
`v0.3.0-alpha.0`, and make the required changes to your code following the
changelog. And so on, with every following release.
