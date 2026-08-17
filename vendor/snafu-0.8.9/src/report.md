Adapts a function to provide user-friendly error output for `main`
functions and tests.

```rust,no_run
use snafu::prelude::*;

#[snafu::report]
fn main() -> Result<()> {
    let _v = frobnicate_the_mumbletypeg()?;

    Ok(())
}

fn frobnicate_the_mumbletypeg() -> Result<u8> {
    api::contact_frobnicate_api().context(FrobnicateSnafu)
}

#[derive(Debug, Snafu)]
#[snafu(display("Unable to frobnicate the mumbletypeg"))]
struct FrobnicateError {
    source: api::ContactFrobnicateApiError,
}

type Result<T, E = FrobnicateError> = std::result::Result<T, E>;

mod api {
    use crate::config;
    use snafu::prelude::*;

    pub fn contact_frobnicate_api() -> Result<u8> {
        config::load_password().context(ContactFrobnicateApiSnafu)
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("Could not contact the mumbletypeg API"))]
    pub struct ContactFrobnicateApiError {
        source: crate::config::MissingPasswordError,
    }

    pub type Result<T, E = ContactFrobnicateApiError> = std::result::Result<T, E>;
}

mod config {
    use snafu::prelude::*;

    pub fn load_password() -> Result<u8> {
        MissingPasswordSnafu.fail()
    }

    #[derive(Debug, Snafu)]
    #[snafu(display("The configuration has no password"))]
    pub struct MissingPasswordError {
        backtrace: snafu::Backtrace,
    }

    pub type Result<T, E = MissingPasswordError> = std::result::Result<T, E>;
}
```

When using `#[snafu::report]`, the output of running this program
may look like (backtrace edited for clarity and brevity):

```text
Error: Unable to frobnicate the mumbletypeg

Caused by these errors (recent errors listed first):
  1: Could not contact the mumbletypeg API
  2: The configuration has no password

Backtrace:
   [... output edited ...]
   3: <std::backtrace::Backtrace as snafu::GenerateImplicitData>::generate
             at crates/snafu/src/lib.rs:1210:9
   4: backtrace_example::config::MissingPasswordSnafu::build
             at ./src/main.rs:48:21
   5: backtrace_example::config::MissingPasswordSnafu::fail
             at ./src/main.rs:48:21
   6: backtrace_example::config::load_password
             at ./src/main.rs:45:9
   7: backtrace_example::api::contact_frobnicate_api
             at ./src/main.rs:29:9
   8: backtrace_example::frobnicate_the_mumbletypeg
             at ./src/main.rs:13:5
   9: backtrace_example::main::{{closure}}
             at ./src/main.rs:7:14
  10: backtrace_example::main
             at ./src/main.rs:5:1
   [... output edited ...]
```

Contrast this to the default output produced when returning a
`Result`:

```text
Error: FrobnicateError { source: ContactFrobnicateApiError { source: MissingPasswordError { backtrace: Backtrace [...2000+ bytes of backtrace...] } } }
```

This macro is syntax sugar for using [`Report`][]; please read
its documentation for detailed information, especially if you wish to
[see backtraces][] in the output.

[see backtraces]: crate::Report#interaction-with-the-provider-api

## Usage with other procedural macros

This macro should work with other common procedural macros. It has been tested with

- `tokio::main`
- `tokio::test`
- `async_std::main`
- `async_std::test`

Depending on the implementation details of each procedural macro, you
may need to experiment by placing `snafu::report` before or after
other macro invocations.
