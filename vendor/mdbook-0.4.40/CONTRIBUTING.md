# Contributing

Welcome stranger!

If you have come here to learn how to contribute to mdBook, we have some tips for you!

First of all, don't hesitate to ask questions!
Use the [issue tracker](https://github.com/rust-lang/mdBook/issues), no question is too simple.

### Issue assignment

**:warning: Important :warning:**

Before working on pull request, please ping us on the corresponding issue.
The current PR backlog is beyond what we can process at this time.
Only issues that have an [`E-Help-wanted`](https://github.com/rust-lang/mdBook/labels/E-Help-wanted) or [`Feature accepted`](https://github.com/rust-lang/mdBook/labels/Feature%20accepted) label will likely receive reviews.
If there isn't already an open issue for what you want to work on, please open one first to see if it is something we would be available to review.

### Issues to work on

If you are starting out, you might be interested in the
[E-Easy issues](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AE-Easy).
Those are issues that are considered more straightforward for beginners to Rust or the codebase itself.
These issues can be a good launching pad for more involved issues.
Easy tasks for a first time contribution include documentation improvements, new tests, examples, updating dependencies, etc.

If you come from a web development background, you might be interested in issues related to web technologies tagged
[A-JavaScript](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-JavaScript),
[A-Style](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-Style),
[A-HTML](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-HTML) or
[A-Mobile](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-Mobile).

When you decide you want to work on a specific issue, and it isn't already assigned to someone else, assign the issue to yourself by leaving a comment with the text `@rustbot claim`.
Again, do not hesitate to ask questions. We will gladly mentor anyone that want to tackle an issue.

Issues on the issue tracker are categorized with the following labels:

- **A**-prefixed labels state which area of the project an issue relates to.
- **E**-prefixed labels show an estimate of the experience necessary to fix the issue.
- **M**-prefixed labels are meta-issues regarding the management of the mdBook project itself
- **S**-prefixed labels show the status of the issue
- **C**-prefixed labels show the category of issue

### Building mdBook

mdBook builds on stable Rust, if you want to build mdBook from source, here are the steps to follow:

1. Navigate to the directory of your choice
0. Clone this repository with git.

   ```
   git clone https://github.com/rust-lang/mdBook.git
   ```
0. Navigate into the newly created `mdBook` directory
0. Run `cargo build`

The resulting binary can be found in `mdBook/target/debug/` under the name `mdbook` or `mdbook.exe`.

### Code Quality

We love code quality and Rust has some excellent tools to assist you with contributions.

#### Formatting Code with rustfmt

Before you make your Pull Request to the project, please run it through the `rustfmt` utility.
This will ensure we have good quality source code that is better for us all to maintain.

[rustfmt](https://github.com/rust-lang/rustfmt) has a lot more information on the project.
The quick guide is

1. Install it (`rustfmt` is usually installed by default via [rustup](https://rustup.rs/)):
    ```
    rustup component add rustfmt
    ```
1. You can now run `rustfmt` on a single file simply by...
    ```
    rustfmt src/path/to/your/file.rs
    ```
   ... or you can format the entire project with
   ```
   cargo fmt
   ```
   When run through `cargo` it will format all bin and lib files in the current package.

For more information, such as running it from your favourite editor, please see the `rustfmt` project. [rustfmt](https://github.com/rust-lang/rustfmt)


#### Finding Issues with Clippy

[Clippy](https://doc.rust-lang.org/clippy/) is a code analyser/linter detecting mistakes, and therefore helps to improve your code.
Like formatting your code with `rustfmt`, running clippy regularly and before your Pull Request will help us maintain awesome code.

1. To install
    ```
    rustup component add clippy
    ```
2. Running clippy
    ```
    cargo clippy
    ```

### Change requirements

Please consider the following when making a change:

* Almost all changes that modify the Rust code must be accompanied with a test.

* Almost all features and changes must update the documentation.
  mdBook has the [mdBook Guide](https://rust-lang.github.io/mdBook/) whose source is at <https://github.com/rust-lang/mdBook/tree/master/guide>.

* Almost all Rust items should be documented with doc comments.
  See the [Rustdoc Book](https://doc.rust-lang.org/rustdoc/) for more information on writing doc comments.

* Breaking the API can only be done in major SemVer releases.
  These are done very infrequently, so it is preferred to avoid these when possible.
  See [SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) for more information on what a SemVer breaking change is.

  (Note: At this time, some SemVer breaking changes are inevitable due to the current code structure.
  An example is adding new fields to the config structures.
  These are intended to be fixed in the next major release.)

* Similarly, the CLI interface is considered to be stable.
  Care should be taken to avoid breaking existing workflows.

* Check out the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for guidelines on designing the API.

### Making a pull-request

When you feel comfortable that your changes could be integrated into mdBook, you can create a pull-request on GitHub.
One of the core maintainers will then approve the changes or request some changes before it gets merged.

That's it, happy contributions! :tada: :tada: :tada:

## Browser compatibility and testing

Currently we don't have a strict browser compatibility matrix due to our limited resources.
We generally strive to keep mdBook compatible with a relatively recent browser on all of the most major platforms.
That is, supporting Chrome, Safari, Firefox, Edge on Windows, macOS, Linux, iOS, and Android.
If possible, do your best to avoid breaking older browser releases.

Any change to the HTML or styling is encouraged to manually check on as many browsers and platforms that you can.
Unfortunately at this time we don't have any automated UI or browser testing, so your assistance in testing is appreciated.

## Updating highlight.js

The following are instructions for updating [highlight.js](https://highlightjs.org/).

1. Clone the repository at <https://github.com/highlightjs/highlight.js>
1. Check out a tagged release (like `10.1.1`).
1. Run `npm install`
1. Run `node tools/build.js :common apache armasm coffeescript d handlebars haskell http julia nginx nim nix properties r scala x86asm yaml`
1. Compare the language list that it spits out to the one in [`syntax-highlighting.md`](https://github.com/camelid/mdBook/blob/master/guide/src/format/theme/syntax-highlighting.md). If any are missing, add them to the list and rebuild (and update these docs). If any are added to the common set, add them to `syntax-highlighting.md`.
1. Copy `build/highlight.min.js` to mdbook's directory [`highlight.js`](https://github.com/rust-lang/mdBook/blob/master/src/theme/highlight.js).
1. Be sure to check the highlight.js [CHANGES](https://github.com/highlightjs/highlight.js/blob/main/CHANGES.md) for any breaking changes. Breaking changes that would affect users will need to wait until the next major release.
1. Build mdbook with the new file and build some books with the new version and compare the output with a variety of languages to see if anything changes. The [test_book](https://github.com/rust-lang/mdBook/tree/master/test_book) contains a chapter with many languages to examine.

## Publishing new releases

Instructions for mdBook maintainers to publish a new release:

1. Create a PR to update the version and update the CHANGELOG:
    1. Update the version in `Cargo.toml`
    2. Run `cargo test` to verify that everything is passing, and to update `Cargo.lock`.
    3. Double-check for any SemVer breaking changes.
       Try [`cargo-semver-checks`](https://crates.io/crates/cargo-semver-checks), though beware that the current version of mdBook isn't properly adhering to SemVer due to the lack of `#[non_exhaustive]` and other issues. See https://github.com/rust-lang/mdBook/issues/1835.
    4. Update `CHANGELOG.md` with any changes that users may be interested in.
    5. Update `continuous-integration.md` to update the version number for the installation instructions.
    6. Commit the changes, and open a PR.
2. After the PR has been merged, create a release in GitHub. This can either be done in the GitHub web UI, or on the command-line:
   ```bash
   MDBOOK_VERS="`cargo read-manifest | jq -r .version`" ; \
    gh release create -R rust-lang/mdbook v$MDBOOK_VERS \
        --title v$MDBOOK_VERS \
        --notes "See https://github.com/rust-lang/mdBook/blob/master/CHANGELOG.md#mdbook-${MDBOOK_VERS//.} for a complete list of changes."
   ```
