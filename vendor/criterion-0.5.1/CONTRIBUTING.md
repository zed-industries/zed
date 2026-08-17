# Contributing to Criterion.<span></span>rs

## Ideas, Experiences and Questions

The easiest way to contribute to Criterion.<span></span>rs is to use it and report your experiences, ask questions and contribute ideas. We'd love to hear your thoughts on how to make Criterion.<span></span>rs better, or your comments on why you are or are not currently using it.

Issues, ideas, requests and questions should be posted on the issue tracker at:

https://github.com/bheisler/criterion.rs/issues

## A Note on Dependency Updates

Criterion.<span></span>rs does not accept pull requests to update dependencies unless specifically
requested by the maintaner(s). Dependencies are updated manually by the maintainer(s) before each
new release.

## Code

Pull requests are welcome, though please raise an issue for discussion first if none exists. We're happy to assist new contributors.

If you're not sure what to work on, try checking the [Beginner label](https://github.com/bheisler/criterion.rs/issues?q=is%3Aissue+is%3Aopen+label%3ABeginner)

To make changes to the code, fork the repo and clone it:

`git clone git@github.com:your-username/criterion.rs.git`

You'll probably want to install [gnuplot](http://www.gnuplot.info/) as well. See the gnuplot website for installation instructions.

Then make your changes to the code. When you're done, run the tests:

```
cargo test --all
cargo bench
```

It's a good idea to run clippy and fix any warnings as well:

```
rustup component add clippy-preview
cargo clippy --all
```

Finally, run Rustfmt to maintain a common code style:

```
rustup component add rustfmt-preview
cargo fmt --all
```

Don't forget to update the CHANGELOG.md file and any appropriate documentation. Once you're finished, push to your fork and submit a pull request. We try to respond to new issues and pull requests quickly, so if there hasn't been any response for more than a few days feel free to ping @bheisler.

Some things that will increase the chance that your pull request is accepted:

* Write tests
* Clearly document public methods
* Write a good commit message

## Branches

*   PRs with breaking changes are made against the unreleased branch. e.g. branch version-0.4
*   PRs without breaking changes are made against the master branch.

If you're not sure which branch to use just start with master, as this can be changed during review.

When it is time to release the unreleased branch, a PR is made from the unreleased branch to master. e.g. https://github.com/bheisler/criterion.rs/pull/496

## Github Labels

Criterion.<span></span>rs uses a simple set of labels to track issues. Most important are the 
difficulty labels:

* Beginner - Suitable for people new to Criterion.rs, or even new to Rust in general
* Intermediate - More challenging, likely involves some non-trivial design decisions and/or knowledge
  of Criterion.<span></span>rs' internals
* Bigger Project - Large and/or complex project such as designing a complex new feature

Additionally, there are a few other noteworthy labels:

* Breaking Change - Fixing this will have to wait until the next breaking-change release
* Bug - Something isn't working right
* Enhancement - Request to add a new feature or otherwise improve Criterion.<span></span>rs in some way

## Code of Conduct

We follow the [Rust Code of Conduct](http://www.rust-lang.org/conduct.html).
