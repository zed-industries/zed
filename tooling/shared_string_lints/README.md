# lints

A [dylint](https://github.com/trailofbits/dylint) library that flags various bad patterns in our codebase.

## Current lints
- `SharedString::new/from etc` where `SharedString::from_static` should be used instead.
TODO!(yara): get another two in with raz

## How to run
Run the `clippy` in `scripts` directory
TODO!(yara) make it so the tooling is installed and the script is a litte more forgiving about missing tools.

## Adding more lints with AI
All issues highlighed go through manual review therefore we feel comfortable vibe coding the lints. At worst we miss cases and have false positives.

In our experience this takes around half an hour.

This is how you can optimally prompt Claude 4.6 as of May 2026 to add a lint, anything that could be detected from the typed AST can work:

TODO!(yara) add that example
