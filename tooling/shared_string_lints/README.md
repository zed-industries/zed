# lints

A [dylint](https://github.com/trailofbits/dylint) library that flags various bad patterns in our codebase.

## Current lints
- `SharedString::new/from etc` where `SharedString::from_static` should be used instead.

## How to run
Run the `clippy` in `scripts` directory
TODO!(yara) make it so the tooling is installed and the script is a litte more forgiving about missing tools.

## Adding more lints with AI
All issues highlighed go through manual review therefore we feel comfortable vibe coding the lints. At worst we miss cases and have false positives.

This is how you can optimally prompt Claude 4.6 as of may 2026 to add a lint:

TODO!(yara)
