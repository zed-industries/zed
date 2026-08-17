# install-rust

A small github action to install `rustup` and a Rust toolchain. This is
generally expressed inline, but it was repeated enough in this repository it
seemed worthwhile to extract.

Some gotchas:

* Can't `--self-update` on Windows due to permission errors (a bug in Github
  Actions)
* `rustup` isn't installed on macOS (a bug in Github Actions)

When the above are fixed we should delete this action and just use this inline:

```yml
- run: rustup update $toolchain && rustup default $toolchain
  shell: bash
```
