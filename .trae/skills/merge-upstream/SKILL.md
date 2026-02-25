---
name: "merge-upstream"
description: "Documents breaking changes in this fork. Invoke when merging from upstream Zed or when build errors occur after a merge."
---

# Merge Upstream Guide

This skill documents breaking changes made in this fork that may conflict with upstream Zed. When merging from upstream, refer to this guide to resolve conflicts.

## Breaking Changes

### 1. `DocumentationAside` render callback signature change

**Location:** `crates/ui/src/components/context_menu.rs`

**Change:** The `DocumentationAside` struct's render callback now takes two arguments instead of one:
- Old: `Fn(&mut App) -> AnyElement`
- New: `Fn(&mut Window, &mut App) -> AnyElement`

**Why:** The provider selector submenu for OpenRouter models needed to calculate its max height based on window viewport size using `vh(0.75, window)`. This prevents the submenu from overflowing the window when there are many providers. The `Window` parameter is required to access viewport dimensions.

**Affected APIs:**
- `DocumentationAside::new(side, render)` - the `render` closure now requires `|window, cx|` signature
- `ContextMenuEntry::documentation_aside(side, render)` - same signature change
- `ModelHoverInfo::render(&self, cx)` → `render(&self, window, cx)`

**Merge hint:** When upstream code calls `documentation_aside()` with a single-argument closure like `|cx| { ... }`, update it to `|_, cx| { ... }` or `|window, cx| { ... }` if window access is needed.

---

*Add new breaking changes above this line following the same format.*
