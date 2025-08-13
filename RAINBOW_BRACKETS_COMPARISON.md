# Rainbow Brackets Implementation: Zed vs Helix Comparison

## Key Differences

### 1. **Query Matching Approach**

**Helix:**
- Uses proper tree-sitter query matching via `query_iter` and `QueryIterEvent::Match`
- Directly processes query matches in order
- Has access to pattern indices and capture information from queries

**Zed (Updated Implementation):**
- Now uses proper tree-sitter query matching via `BufferSnapshot::matches()`
- Directly processes query matches in order
- Has access to pattern indices and capture information from queries
- Fully respects the `rainbow.scm` query files

### 2. **Scope Tracking**

Both implementations use a similar scope stack approach:
- Pop scopes that have ended before the current node
- Track whether to include children based on pattern properties
- Use pattern indices to determine if `rainbow.include-children` applies

Both implementations now properly respect the `rainbow.include-children` property from the query.

### 3. **Node Type Detection**

Both Helix and Zed now:
- Use tree-sitter queries to determine which nodes are scopes/brackets
- Fully respect the `rainbow.scm` query files
- Don't require any hardcoded node type checks
- Support new languages simply by adding appropriate `rainbow.scm` files

### 4. **Implementation Complexity**

Both implementations now have similar complexity:
- Clean, maintainable implementation
- Language-specific behavior defined entirely in `rainbow.scm` files
- Helix: ~50 lines of core logic
- Zed: ~80 lines of core logic (slightly more due to Zed's highlight API requiring level-specific types)

## Technical Limitations Resolved

**Update:** We discovered that `BufferSnapshot::matches()` is actually public! This allowed us to rewrite the implementation to use proper tree-sitter query matching.

Previously identified limitations that are now resolved:
1. ✅ **API Access:** `BufferSnapshot::matches()` provides the needed query matching functionality
2. ✅ **Query Infrastructure:** Rainbow queries are pre-parsed in the grammar, no runtime creation needed
3. ✅ **Pattern Matching:** We can access pattern indices through `mat.pattern_index`

## Current Status

The implementation now:
- Works correctly for all languages with `rainbow.scm` files
- Properly respects all query properties including `rainbow.include-children`
- Has similar maintainability to the Helix approach
- Supports new languages by simply adding appropriate `rainbow.scm` files

## Key Implementation Details

1. **Scope Stack Algorithm:** Tracks nesting levels and scope boundaries
2. **Pattern Index Handling:** Uses pattern indices to determine `include-children` behavior
3. **Direct Child Detection:** Uses `node.parent()` to check if brackets are direct children
4. **Level-based Highlighting:** Maps nesting levels to color levels (modulo 10)