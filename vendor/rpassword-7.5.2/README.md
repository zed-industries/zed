# Rustastic Password

`rpassword` makes it easy to read passwords from Rust code in console applications on all platforms, Unix, Windows, WASM and more.
It's similar to Linux's C function `getpass()` or Python's `getpass` module.

![rpassword logo and headline](rpassword.png)

## Usage

Add `rpassword` as a dependency in Cargo.toml:

```toml
[dependencies]
rpassword = "7.5"
```

Then use it in your code:

```rust
use std::io::{Cursor, Write};
use rpassword::{ConfigBuilder};

fn main() {
    // By default, reads and writes to the console, hides password as it is typed 
    let password = rpassword::prompt_password("Your password: ").unwrap();
    println!("Your password is {}", password);
    
    // Behavior is customizable to accommodate custom use-cases and testing
    // See documentation for more details
     let config = rpassword::ConfigBuilder::new()
         .input_data("my-password\n")
         .output_discard()
         .password_feedback_mask('*')
         .build();
    
     let password = rpassword::read_password_with_config(config).unwrap();
     println!("Your password is {}", password);
}
```

See examples and docs at [https://docs.rs/rpassword](https://docs.rs/rpassword).

See the upgrade path in [UPGRADE.md](UPGRADE.md).

## License

The source code is released under the Apache 2.0 license.
