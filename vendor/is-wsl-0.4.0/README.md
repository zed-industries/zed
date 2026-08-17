# is-wsl

> Check if the process is running inside [Windows Subsystem for Linux](https://msdn.microsoft.com/commandline/wsl/about) (Bash on Windows)

Inspired by [sindresorhus/is-wsl](https://github.com/sindresorhus/is-wsl) and made for Rust lang.

Can be useful if you need to work around unimplemented or buggy features in WSL. Supports both WSL 1 and WSL 2.


## Usage 
`$> cargo add is-wsl`

_main.rs_
```rust

use is_wsl::is_wsl

fn main() {
    if is_wsl() {
        // Do some WSL related stuff 🎇
    } else {
        // Do some different things! <3
    }
}
```

