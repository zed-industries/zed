# SNAFU

**S**ituation **N**ormal: **A**ll **F**ouled **U**p

[![crates.io][Crate Logo]][Crate]
[![Documentation][Doc Logo]][Doc]
[![Build Status][CI Logo]][CI]

SNAFU is a library to easily assign underlying errors into
domain-specific errors while adding context.

```rust
use snafu::prelude::*;
use std::{fs, io, path::PathBuf};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Unable to read configuration from {}", path.display()))]
    ReadConfiguration { source: io::Error, path: PathBuf },

    #[snafu(display("Unable to write result to {}", path.display()))]
    WriteResult { source: io::Error, path: PathBuf },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn process_data() -> Result<()> {
    let path = "config.toml";
    let configuration = fs::read_to_string(path).context(ReadConfigurationSnafu { path })?;
    let path = unpack_config(&configuration);
    fs::write(&path, b"My complex calculation").context(WriteResultSnafu { path })?;
    Ok(())
}

fn unpack_config(data: &str) -> &str {
    "/some/path/that/does/not/exist"
}
```

Please see [the documentation][Doc] and the [user's guide][Guide] for
a full description.

[Crate]: https://crates.io/crates/snafu
[Crate Logo]: https://img.shields.io/crates/v/snafu.svg

[Doc]: https://docs.rs/snafu
[Doc Logo]: https://docs.rs/snafu/badge.svg
[Guide]: https://docs.rs/snafu/*/snafu/guide/index.html

[CI]: https://cirrus-ci.com/github/shepmaster/snafu
[CI Logo]: https://api.cirrus-ci.com/github/shepmaster/snafu.svg
