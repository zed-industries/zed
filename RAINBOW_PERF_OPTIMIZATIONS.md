# Rainbow Brackets Performance Optimizations

## Problem
The rainbow brackets feature was causing severe CPU usage and UI hangs when enabled, particularly noticeable during cursor movement and text selection. The main thread was experiencing suspected hangs, with the bottleneck appearing in the `update_active_pair` function.

## Root Causes Identified

1. **Expensive anchor-to-offset conversions**: The `update_active_pair` function was calling `to_offset()` multiple times in loops, which is computationally expensive.

2. **Full buffer scanning**: The code was scanning ALL brackets in the entire buffer on every change, regardless of what was visible.

3. **Too frequent updates**: `refresh_rainbow_brackets` was called on every selection change, including during mouse drags and rapid cursor movements.

4. **No caching**: The active pair detection was recalculating on every cursor movement, even when the cursor hadn't actually moved.

5. **Inefficient active pair detection**: The code was looping through all brackets to find the active pair instead of using more efficient methods.

## Optimizations Implemented

### 1. Debouncing and Caching
- Added cursor position caching to avoid redundant updates when cursor hasn't moved
- Implemented time-based throttling (10ms) for active pair updates during rapid cursor movement
- Added `last_cursor_offset` and `last_active_pair_update` fields to track state

### 2. Reduced Anchor-to-Offset Conversions
- Cache cursor offset to avoid repeated conversions
- Only check immediate neighbors (cursor position ± 1) instead of all brackets
- Use range checks instead of individual position checks where possible

### 3. Smart Range Processing
- Added visible range tracking with padding for smoother scrolling
- Limit processing to 100K characters for large files
- Skip rainbow brackets entirely for extremely large ranges
- Added early exit conditions when disabled

### 4. Improved Active Pair Detection
- First try `innermost_enclosing_bracket_ranges` (most common case)
- Only check adjacent positions as fallback
- Eliminate full bracket list iteration

### 5. Better State Management
- Clear all state when feature is disabled
- Track buffer edit count to avoid recalculating on scrolls
- Cache visible range to avoid redundant updates

### 6. Size Limits
- Process only first 50K characters for files larger than 100K
- Limit total number of bracket pairs processed using `max_brackets` setting
- Completely skip processing for extremely large ranges

## Performance Impact

These optimizations should significantly reduce CPU usage by:
- Eliminating most redundant calculations
- Reducing the scope of bracket processing
- Avoiding expensive operations during rapid cursor movement
- Processing only what's necessary for the current view

## Future Improvements

When better APIs become available in the editor:
1. Implement proper visible range calculation using viewport information
2. Use incremental updates instead of full recalculation
3. Consider background processing for initial bracket analysis
4. Add user-configurable performance settings (processing limits, update frequency)

## Testing Recommendations

1. Test with large files (>100K lines) to ensure no hangs
2. Test rapid cursor movement and selection changes
3. Monitor CPU usage during normal editing operations
4. Verify rainbow brackets still work correctly for nested structures
5. Test with different language files (deeply nested JSON, complex Rust code, etc.)