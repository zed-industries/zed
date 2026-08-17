# is-docker
Checks if the process is running inside a Docker container. Rust implementation of `sindresorhus/is-docker`

## Usage 
`$> cargo add is-docker`

_main.rs_
```rust

use is_docker::is_docker

fn main() {
    if is_docker() {
        // Do some docker related stuff 🎇
    } else {
        // Do some different things! <3
    }
}
```

