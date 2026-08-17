# Missing field source / IntoError is not implemented

This error is encountered in multi-module / multi-file projects when
the error type is defined in one module and constructed in another.

## Failing Example

**project_error.rs**

```rust,ignore
use snafu::prelude::*;
use std::io;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ProjectError {
    #[snafu(display("Unable to read configuration from {path}"))]
    IOConfigError {
        path: &'static str,
        source: io::Error,
    },
}
```

**main.rs**

```rust,ignore
mod project_error;

use project_error::ProjectError;
use snafu::prelude::*;
use std::fs;

const CONFIG_PATH: &str = "/etc/example/conf.conf";

pub fn read_config() -> Result<String, ProjectError> {
    fs::read_to_string(CONFIG_PATH).context(ProjectError::IOConfigError { path: CONFIG_PATH })
}

pub fn main() {
    println!("{}", read_config().unwrap());
}
```

## Errors

```text
error[E0063]: missing field `source` in initializer of `ProjectError`
  --> examples/scratch.rs:23:45
   |
23 |     fs::read_to_string(CONFIG_PATH).context(ProjectError::IOConfigError { path: CONFIG_PATH })
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `source`
```

and

```text
error[E0277]: the trait bound `ProjectError: IntoError<_>` is not satisfied
  --> examples/scratch.rs:23:45
   |
23 |     fs::read_to_string(CONFIG_PATH).context(ProjectError::IOConfigError { path: CONFIG_PATH })
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `IntoError<_>` is not implemented for `ProjectError`
```

## Solution

Replace the `ProjectError::IOConfigError` in the `read_config()`
function with `project_error::IOConfigSnafu`.

## Explanation

This works because the `#[derive(Snafu)]` macro creates the *context
selector* type `IoConfigSnafu`:

```rust,ignore
#[derive(Debug, Snafu)]
pub enum ProjectError {
    IOConfigError {
        source: io::Error,
        path: &'static str,
    },
}

// some details removed
struct IOConfigSnafu<P> {
    path: P,
}

// Some impls for the IOConfigError struct
```

See [what the `Snafu` macro generates
section](guide::what_code_is_generated) of the guide for more details.

When you use `ProjectError::IOConfigError`, you're referencing the
enum variant, not the struct that you need. Replacing
`ProjectError::IOConfigError` with `project_error::IOConfigSnafu`
fixes this problem.
