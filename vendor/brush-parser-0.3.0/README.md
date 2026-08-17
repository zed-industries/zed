<div align="center">
  <img src="https://github.com/user-attachments/assets/19351a8e-7b03-4338-81be-dd5b6d7e5abc"/>
</div>

<br/>

<!-- Primary badges -->
<p align="center">
  <!-- crates.io version badge -->
  <a href="https://crates.io/crates/brush-shell"><img src="https://img.shields.io/crates/v/brush-shell?style=flat-square"/></a>
  <!-- msrv badge -->
  <img src="https://img.shields.io/crates/msrv/brush-shell"/>
  <!-- LoC badge: badge generation seems broken; temporarily disabled -->
  <!-- <img src="https://tokei.rs/b1/github/reubeno/brush?category=code"/> -->
  <!-- license badge -->
  <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square"/>
  <!-- CI status badge -->
  <a href="https://github.com/reubeno/brush/actions/workflows/ci.yaml"><img src="https://github.com/reubeno/brush/actions/workflows/ci.yaml/badge.svg"/></a>
  <br/>
  <!-- crates.io download badge -->
  <a href="https://crates.io/crates/brush-shell"><img src="https://img.shields.io/crates/d/brush-shell?style=flat-square"/></a>
  <!-- Packaging badges -->
  <a href="https://repology.org/project/brush/versions">
    <img src="https://repology.org/badge/tiny-repos/brush.svg" alt="Packaging status"/>
  </a>
  <!-- Dependencies badges -->
  <a href="https://deps.rs/repo/github/reubeno/brush"><img src="https://deps.rs/repo/github/reubeno/brush/status.svg" alt="Dependency status"/></a>
  <!-- Social badges -->
  <a href="https://discord.gg/kPRgC9j3Tj">
    <img src="https://dcbadge.limes.pink/api/server/https://discord.gg/kPRgC9j3Tj?compact=true&style=flat" alt="Discord invite"/>
  </a>
</p>

<a href="https://repology.org/project/brush/versions">
</a>

</p> 

<hr/>

`brush` (**B**o(u)rn(e) **RU**sty **SH**ell) is a [POSIX-](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html) and [bash-](https://www.gnu.org/software/bash/)compatible shell,
implemented in Rust. At its core is an embeddable shell interpreter published for reuse
in other Rust projects. It's built and tested on Linux, macOS, and WSL. Native Windows
support is experimental.

<p align="center">
  <img src="https://github.com/user-attachments/assets/0e64d1b9-7e4e-43be-8593-6c1b9607ac52" width="80%"/>
</p>

`brush` is functional for interactive use as a daily driver! It executes most `sh` and `bash` scripts we've
encountered. Known limitations are tracked with filed issues. Out of an abundance of caution,
we wouldn't recommend using it yet in _production_ scenarios in case it doesn't behave identically
to your existing stable shell. (If you do find any behavioral differences, though, please report them with an
issue!)

Contributions and feedback of all kinds are welcome! For more guidance, please consult our
[contribution guidelines](CONTRIBUTING.md). For more technical details, please consult the
[documentation](docs/README.md) in this repo.

This project was originally borne out of curiosity and a desire to learn. We're doing our best to keep that
attitude :).

<br/>

## 📝 License

Available for use and distribution under the [MIT license](LICENSE).

## ⌨️ Installation

_When you run `brush`, it should look exactly as `bash` does on your system: it processes your `.bashrc` and
other standard configuration. If you'd like to distinguish the look of `brush` from the other shells
on your system, you may author a `~/.brushrc` file._

<details open>
<summary>🚀 <b>Installing prebuilt binaries from GitHub</b></summary>

We publish prebuilt binaries of `brush` for Linux (x86_64, aarch64) and macOS (aarch64) to GitHub for official [releases](https://github.com/reubeno/brush/releases). You can manually download and extract the `brush` binary from one of the archives published there, or otherwise use the GitHub CLI to download it, e.g.:

```bash
gh release download --repo reubeno/brush --pattern "brush-x86_64-unknown-linux-gnu.*"
```

After downloading the archive for your platform, you may verify its authenticity using the [GitHub CLI](https://cli.github.com/), e.g.:

```bash
gh attestation verify brush-x86_64-unknown-linux-gnu.tar.gz --repo reubeno/brush
```
</details>

<details open>
<summary>🚀 <b>Installing prebuilt binaries via `cargo binstall`</b></summary>

You may use [cargo binstall](https://github.com/cargo-bins/cargo-binstall) to install pre-built `brush` binaries. Once you've installed `cargo-binstall` you can run:

```bash
cargo binstall brush-shell
```

</details>

<details>
<summary> 🔨 <b>Installing from sources</b></summary>

To build from sources, first install a working (and recent) `rust` toolchain; we recommend installing it via [`rustup`](https://rustup.rs/). Then run:

```bash
cargo install --locked brush-shell
```
</details>

<details>
<summary>🐧 <b>Installing using Nix</b></summary>

If you are a Nix user, you can use the registered version:

```bash
nix run 'github:NixOS/nixpkgs/nixpkgs-unstable#brush' -- --version
```
</details>

<details>
<summary>🐧 <b>Installing on Arch Linux</b></summary>

Arch Linux users can install `brush` from the official [extra repository](https://archlinux.org/packages/extra/x86_64/brush/):

```bash
pacman -S brush
```
</details>

<details>
<summary>🍺 <b>Installing using Homebrew</b></summary>

Homebrew users can install using [the `brush` formula](https://formulae.brew.sh/formula/brush):

```bash
brew install brush
```
</details>

## 👥 Community

`brush` has a community Discord server, available [here](https://discord.gg/kPRgC9j3Tj).

## 🔍 Known limitations

There are some known gaps in compatibility. Most notably:

* **Some `set` and `shopt` options.**
  The `set` builtin is implemented, as is `set -x` and many frequently used `set`/`shopt` options, but a number aren't fully implemented. For example, `set -e` will execute but its semantics aren't applied across execution.

If you're interested, we'd love contributions to improve compatibility, broaden test coverage, or really any other opportunities you can find to help us make this project better.

## 🧪 Testing strategy

This project is primarily tested by comparing its behavior with other existing shells, leveraging the latter as test oracles. The integration tests implemented in this repo include [850+ test cases](brush-shell/tests/cases) run on both this shell and an oracle, comparing standard output and exit codes.

For more details, please consult the [reference documentation on integration testing](docs/reference/integration-testing.md).

## 🙏 Credits

There's a long list of OSS crates whose shoulders this project rests on. Notably, the following crates are directly relied on for major portions of shell functionality:

* [`reedline`](https://github.com/nushell/reedline) - for readline-like input and interactive usage
* [`clap`](https://github.com/clap-rs/clap) - command-line parsing, used both by the top-level brush CLI as well as built-in commands
* [`fancy-regex`](https://github.com/fancy-regex/fancy-regex) - relied on for everything regex
* [`tokio`](https://github.com/tokio-rs/tokio) - async, well, everything
* [`nix` rust crate](https://github.com/nix-rust/nix) - higher-level APIs for Unix/POSIX system APIs

For testing, performance benchmarking, and other important engineering support, we use and love:

* [`pprof-rs`](https://github.com/tikv/pprof-rs) - for sampling-based CPU profiling
* [`criterion.rs`](https://github.com/bheisler/criterion.rs) - for statistics-based benchmarking
* [`bash-completion`](https://github.com/scop/bash-completion) - for its completion test suite and general completion support!

## 🔗 Links: other shell implementations

There are a number of other POSIX-ish shells implemented in a non-C/C++ implementation language. Some inspirational examples include:

* [`nushell`](https://www.nushell.sh/) - modern Rust-implemented shell (which also provides the `reedline` crate we use!)
* [`rusty_bash`](https://github.com/shellgei/rusty_bash)
* [`mvdan/sh`](https://github.com/mvdan/sh)
* [`Oils`](https://github.com/oils-for-unix/oils)
* [`fish`](https://fishshell.com) ([as of 4.0](https://fishshell.com/blog/rustport/))

We're sure there are plenty more; we're happy to include links to them as well.
