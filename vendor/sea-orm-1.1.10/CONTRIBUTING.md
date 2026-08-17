# Contributing to SeaORM

Thank you for taking the time to read this. First of all, star ALL our repos!

SeaORM is a community driven project. We welcome you to participate, contribute and together build for SeaQL's future.

## Code of Conduct

This project is governed by the [SeaQL Code of Conduct](https://github.com/SeaQL/.github/blob/master/CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## I have a question

If you have a question to ask, please do not open an issue for it. It's quicker to ask us on [SeaQL Discord Server](https://discord.com/invite/uCPdDXzbdv) or open a [GitHub Discussion](https://docs.github.com/en/discussions/quickstart#creating-a-new-discussion) on the corresponding repository.

## I need a feature

Feature requests from anyone is definitely welcomed! Actually, since 0.2, many features are proposed and/or contributed by non-core members, e.g. [#105](https://github.com/SeaQL/sea-orm/issues/105), [#142](https://github.com/SeaQL/sea-orm/issues/142), [#252](https://github.com/SeaQL/sea-orm/issues/252), with various degrees of involvement. We will implement feature proposals if it benefits everyone, but of course code contributions will more likely be accepted.

## I want to support

Awesome! The best way to support us is to recommend it to your classmates/colleagues/friends, write blog posts and tutorials on SeaQL projects and help out other users in the community. It is difficult enough to keep an open source afloat, so every little help matters, especially if it can directly/indirectly lighten the core team's mental load.

## I want to join

We are always looking for long-term contributors. If you want to commit longer-term to SeaQL's open source effort, definitely talk with us! There may be various forms of "grant" to compensate for your devotion. Although at this stage we are not resourceful enough to offer a stable stream of income to contributors.

## I want to sponsor

If you don't have time to contribute but would like to support the organization, a financial contribution via [GitHub sponsor](https://github.com/sponsors/SeaQL) is a great way to support us.

## I want to setup my machine for development and testing

Thanks for the time and effort to compose a PR! You are always welcomed to contact us via [Discord](https://discord.com/invite/uCPdDXzbdv) or GitHub if you need any help when contributing. Feel free to open draft PR and ask for review and guidance.

### Unit Test

Without involving a live database, you can run unit tests on your machine with the command below:

- Unit testing `sea-orm`, `sea-orm-macros`, `sea-orm-codegen`
    ```sh
    cargo test --workspace
    ```
- Unit testing `sea-orm-cli`
    ```sh
    cargo test --manifest-path sea-orm-cli/Cargo.toml
    ```
- Unit testing `sea-orm-rocket`
    ```sh
    cargo test --manifest-path sea-orm-rocket/Cargo.toml
    ```

### Integration Test

Next, if you want to run integration tests on a live database. We recommend using Docker to spawn your database instance, you can refer to [this](build-tools/docker-compose.yml) docker compose file for reference.

Running integration tests on a live database:
- SQLite
    ```sh
    DATABASE_URL="sqlite::memory:" cargo test --all --features default,sqlx-sqlite,runtime-async-std-native-tls
    ```
- MySQL / MariaDB
    ```sh
    DATABASE_URL="mysql://root:root@localhost" cargo test --all --features default,sqlx-mysql,runtime-async-std-rustls
    ```
- PostgreSQL
    ```sh
    DATABASE_URL="postgres://root:root@localhost" cargo test --all --features default,sqlx-postgres,runtime-async-std-native-tls
    ```

### Running `sea-orm-cli` from source code

You can either run the follow command at root:

```sh
cargo run --manifest-path sea-orm-cli/Cargo.toml -- <command & arguments for sea-orm-cli>
# E.g.
cargo run --manifest-path sea-orm-cli/Cargo.toml -- migrate init
```

Or, you `cd` into `sea-orm-cli` directory and simply execute:

```sh
cargo run -- <command & arguments for sea-orm-cli>
# E.g.
cargo run -- migrate init
```

### Installing `sea-orm-cli` from source code

You can either run the follow command at root:

```sh
cargo install --force --path sea-orm-cli
```

Or, you `cd` into `sea-orm-cli` directory and simply execute:

```sh
cargo install --force --path .
```

Or, you install `sea-orm-cli` from GitHub:

```sh
cargo install sea-orm-cli --force --git https://github.com/SeaQL/sea-orm --branch <GIT_BRANCH>
```
