# Contributing

Thanks for contributing!

## Getting Started

### Required software

- [Rust toolchain](https://www.rust-lang.org/en-US/install.html)
- [`cargo-readme`](https://github.com/livioribeiro/cargo-readme) (`cargo install
  cargo-readme`)
- [GNU Make](https://www.gnu.org/software/make/)
- [Docker](https://www.docker.com/) or another Docker-compatible container
  engine
  - The docker command used by the Makefile can be specified with `DOCKER_COMMAND`, e.g.
    ```
    make DOCKER_COMMAND=containerd docker/redis
    ```

## Making Changes

- Before committing changes, make sure to run `make fmt` to format the changes
- Add an entry to the `CHANGELOG.md` file
- The `README.md` is generated using `cargo-readme` from the crate level
  documentation in `src/lib.rs`. This means the `README.md` should never be
  modified by hand. To sync changes made to `src/lib.rs`, run `make docs`
- The CI system will run all tests using `make ci`. Make sure to run all tests
  before submitting your pull request

## Make goals overview

```bash
# The goal used by the CI system
make ci
# Runs all examples
make examples
# Runs all tests
make tests
# Syncs all docs
make sync
# Formats the source code
make fmt
# Performs multiple checks (formatting, clippy and documentation sync)
make check
# Removes all generated artifacts, including docker containers
make clean
```

## Submitting Changes

Pull Requests should be made against master.
Travis CI will run the test suite on all PRs.
Remember to update the changelog!

