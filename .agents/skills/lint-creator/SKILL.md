---
name: lint-creator
description: An auxiliary skill to add more dylints to `tooling/lints`
disable-model-invocation: false
---

# Lint RULES

1. Every lint MUST have accompanying `ui` tests
2. `ui` tests MUST be in the `ui` folder
3. Every lint MUST be in a separate module
4. Every lint MUST have negative `ui` tests
5. Every lint-specific `ui/*.rs` fixture MUST put cases expected to trigger the lint first, followed by cases expected not to trigger it, using exactly these headings:
   ```rust
   // ==================== SHOULD FIRE ====================
   // ================== SHOULD NOT FIRE ==================
   ```
6. Lints should be as simple as possible.
7. Reporting is fine if it's simple, it does not need to be elaborate or lengthy code.
8. Do NOT suggest how to fix the lint, only flag it.
9. Do NOT make lints machine applicable.
10. Detect if lints are redundant vs clippy's capabilities.
