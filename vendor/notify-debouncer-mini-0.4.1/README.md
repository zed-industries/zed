# Notify debouncer

[![» Docs](https://flat.badgen.net/badge/api/docs.rs/df3600)][docs]

Tiny debouncer for [notify]. Filters incoming events and emits only one event per timeframe per file.

## Features

- `crossbeam` enabled by default, for crossbeam channel support.  
This may create problems used in tokio environments. See [#380](https://github.com/notify-rs/notify/issues/380).  
Use something like the following to disable it.
```toml
notify-debouncer-mini = { version = "*", default-features = false }
```
This also passes through to notify as `crossbeam-channel` feature.
- `serde` for serde support of event types, off by default

[docs]: https://docs.rs/notify-debouncer-mini
[notify]: https://crates.io/crates/notify
