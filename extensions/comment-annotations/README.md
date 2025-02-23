# Comment Annotations Extension for Zed

This extension adds syntax highlighting for common comment annotations in source code files. It highlights annotations like:

- TODO
- NOTE/NOTES
- FIXME
- BUG/BUGS
- HACK
- XXX
- REVIEW
- OPTIMIZE
- QUESTION
- INFO

The highlighting works in both regular comments and documentation comments across different programming languages.

## Installation

This extension is bundled with Zed and is enabled by default.

## Usage

Simply write any of the supported annotations in your comments, and they will be highlighted automatically:

```rust
// TODO: Implement error handling
/* NOTE: This is an important section */
/// FIXME: Update documentation
/** BUG: Memory leak in this function */
```