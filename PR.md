# Rainbow Brackets for Zed Editor

## Overview

This PR implements rainbow bracket highlighting for Zed, addressing issue #5259. The implementation is inspired by the recent Helix PR ([helix-editor/helix#13530](https://github.com/helix-editor/helix/pull/13530)) and provides similar functionality using tree-sitter queries.

## Context

- **Parent Issue:** https://github.com/zed-industries/zed/issues/5259
- **Inspiration:** https://github.com/helix-editor/helix/pull/13530
- **Implementation:** Heavily assisted by Claude Code as I'm not proficient in Rust or familiar with the Zed codebase
- **Motivation:** I'm missing this feature badly and decided to implement it myself

## Implementation Details

The implementation works conceptually the same way as in Helix:
- Uses `rainbow.scm` files to define bracket and scope patterns for each language
- Leverages tree-sitter for syntax-aware bracket matching
- Maintains a scope stack to track nesting levels
- Colors brackets based on their nesting depth

### Key Components

1. **Rainbow Query Files** (`crates/languages/src/*/rainbow.scm`):
   - Define which syntax nodes are scopes (`@rainbow.scope`)
   - Define which tokens are brackets (`@rainbow.bracket`)
   - Support `rainbow.include-children` property for fine-grained control

2. **Core Implementation** (`crates/editor/src/rainbow_brackets.rs`):
   - Uses `BufferSnapshot::matches()` for proper tree-sitter query matching
   - Implements scope tracking algorithm similar to Helix
   - Maps nesting levels to colors (cycling through 10 levels)

## Current Limitations & Help Needed

### 1. Text Highlighting vs Background Highlighting
**Current:** Using `editor.highlight_background()` which colors the background behind brackets
**Desired:** Need to highlight the actual text color of the brackets themselves

I tried using `editor.highlight_text` but couldn't get it to work. I need help understanding:
- How to properly use text highlighting in Zed
- Whether there's a different API I should be using
- If text highlighting requires a different approach than background highlighting

### 2. TODO Items

- [ ] **Create tests** - Need to add comprehensive tests for the rainbow bracket functionality
- [ ] **Use text highlight instead of background highlight** - Main blocker, need help with Zed's text highlighting API
- [ ] **Make colors configurable from settings** - Currently colors are hardcoded in the implementation

## Code Example

Currently working for Rust:
```rust
fn main() {
    let vec = vec![1, 2, 3];  // Different colors for [ ]
    if true {                  // Different color for { }
        println!("Hello");     // Different color for ( )
    }
}
```

## Request for Guidance

As someone unfamiliar with Rust and the Zed codebase, I particularly need help with:
1. Understanding how to use Zed's text highlighting API correctly
2. Best practices for adding configuration options to Zed's settings
3. Guidance on the testing approach for this feature

Any assistance or code examples would be greatly appreciated!

## Release Notes

- Added rainbow bracket highlighting support for better visualization of nested code structures
- Fixed bracket matching to be syntax-aware using tree-sitter queries
- Improved code readability by assigning different colors to brackets based on their nesting depth