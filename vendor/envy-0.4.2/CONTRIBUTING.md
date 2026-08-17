# Contributing

## Filing an Issue

If you are trying to use `envy` and run into an issue- please file an
issue! We'd love to get you up and running, even if the issue you have might
not be directly related to the code in `envy`. This library seeks to make
it easy for developers to get going, so there's a good chance we can do
something to alleviate the issue by making `envy` better documented or
more robust to different developer environments.

When filing an issue, do your best to be as specific as possible
 The faster was can reproduce your issue, the faster we
can fix it for you!

## Submitting a PR

If you are considering filing a pull request, make sure that there's an issue
filed for the work you'd like to do. There might be some discussion required!
Filing an issue first will help ensure that the work you put into your pull
request will get merged :)

Before you submit your pull request, check that you have completed all of the
steps mentioned in the pull request template. Link the issue that your pull
request is responding to, and format your code using [rustfmt][rustfmt].

### Configuring rustfmt

Before submitting code in a PR, make sure that you have formatted the codebase
using [rustfmt][rustfmt]. `rustfmt` is a tool for formatting Rust code, which
helps keep style consistent across the project. If you have not used `rustfmt`
before, it is not too difficult.

If you have not already configured `rustfmt` for the
nightly toolchain, it can be done using the following steps:

**1. Use Nightly Toolchain**

Install the nightly toolchain. This will only be necessary as long as rustfmt produces different results on stable and nightly.

```sh
$ rustup toolchain install nightly
```

**2. Add the rustfmt component**

Install the most recent version of `rustfmt` using this command:

```sh
$ rustup component add rustfmt-preview --toolchain nightly
```

**3. Running rustfmt**

To run `rustfmt`, use this command:

```sh
cargo +nightly fmt
```

[rustfmt]: https://github.com/rust-lang-nursery/rustfmt

### IDE Configuration files
Machine specific configuration files may be generaged by your IDE while working on the project. Please make sure to add these files to a global .gitignore so they are kept from accidentally being commited to the project and causing issues for other contributors.

Some examples of these files are the `.idea` folder created by JetBrains products (WebStorm, IntelliJ, etc) as well as `.vscode` created by Visual Studio Code for workspace specific settings.

For help setting up a global .gitignore check out this [GitHub article]!

[GitHub article]: https://help.github.com/articles/ignoring-files/#create-a-global-gitignore

## Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html)