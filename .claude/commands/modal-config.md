# Expo 54 Modal Configuration Standard

You are configuring an Expo Router modal using the standardized formSheet configuration that provides consistent UX across the StoryBud mobile app.

## Standard Modal Configuration

**ALL modals in this application MUST use the following configuration** to ensure consistent behavior, appearance, and user experience:

```typescript
<Stack.Screen
  name="modal-name"
  options={{
    title: 'Modal Title',
    presentation: 'formSheet',
    gestureDirection: 'vertical',
    animation: 'slide_from_bottom',
    sheetGrabberVisible: true,
    sheetInitialDetentIndex: 0,
    sheetAllowedDetents: [0.5, 0.75, 1],
    sheetCornerRadius: 20,
    sheetExpandsWhenScrolledToEdge: true,
    sheetElevation: 24,
  }}
/>
```

## Required Options (Never Omit These)

### ✅ `presentation: 'formSheet'`
- **Purpose**: Enables iOS-style sheet presentation
- **Effect**: Modal slides up from bottom as a sheet overlaying content
- **Required**: YES - This is the foundation of the modal UX

### ✅ `gestureDirection: 'vertical'`
- **Purpose**: Enables swipe-down-to-dismiss gesture
- **Effect**: Users can drag the sheet down to close it
- **Required**: YES - Essential for modern iOS UX patterns

### ✅ `animation: 'slide_from_bottom'`
- **Purpose**: Controls entrance/exit animation
- **Effect**: Sheet smoothly slides up from bottom edge
- **Required**: YES - Matches iOS system behavior

### ✅ `sheetGrabberVisible: true`
- **Purpose**: Shows drag handle at top of sheet
- **Effect**: Visual affordance indicating sheet can be dragged
- **Required**: YES - Critical for discoverability (see image reference)
- **Appearance**: Small horizontal line centered at top of modal

## Customizable Options (Adjust Per Modal)

### 🔧 `sheetInitialDetentIndex: 0`
- **Purpose**: Controls which height the modal opens at
- **Values**:
  - `0` = Opens at first detent (e.g., 50% height)
  - `1` = Opens at second detent (e.g., 75% height)
  - `2` = Opens at third detent (e.g., 100% height)
- **When to adjust**:
  - Use `0` for simple modals with minimal content
  - Use `1` for settings/forms with moderate content (recommended for settings-modal)
  - Use `2` for full-screen experiences

### 🔧 `sheetAllowedDetents: [0.5, 0.75, 1]`
- **Purpose**: Defines snap points where sheet can rest
- **Values**: Array of decimals between 0 and 1 (percentage of screen height)
- **Default**: `[0.5, 0.75, 1]` = half, three-quarters, full screen
- **Common alternatives**:
  - `[0.7, 1]` - Two detents for taller content
  - `[0.4, 0.8, 1]` - Three detents with different proportions
  - `[1]` - Full screen only (like traditional modal)
- **Tip**: Match detent count to your content's natural sizes

### 🔧 `sheetCornerRadius: 20`
- **Purpose**: Rounds the top corners of the sheet
- **Default**: `20` - Modern, polished appearance
- **Range**: `0` to `40` (typically)
- **When to adjust**: Usually keep at 20 for consistency

### 🔧 `sheetExpandsWhenScrolledToEdge: true`
- **Purpose**: Auto-expands sheet when user scrolls to top of content
- **Effect**: Sheet grows to next detent when scrolling past top
- **Recommendation**: Keep `true` for better UX on content-heavy modals

### 🔧 `sheetElevation: 24`
- **Purpose**: Controls shadow depth (Android primarily)
- **Default**: `24` - Prominent shadow
- **Range**: `0` to `30`
- **Note**: Minimal effect on iOS, important for Android consistency

## Optional Properties

### 🎯 `headerShown: false`
- **Use when**: You want a custom header or no header at all
- **Example**: Settings modal with custom close button
- **Default**: `true` (shows Expo Router's default header)

### 🎯 `title: 'Your Title'`
- **Purpose**: Sets modal title in header bar
- **Required when**: `headerShown` is not `false`
- **Tip**: Keep short and descriptive

## Implementation Examples

### Example 1: Simple Modal (50% initial height)
```typescript
<Stack.Screen
  name="quick-action"
  options={{
    title: 'Quick Action',
    presentation: 'formSheet',
    gestureDirection: 'vertical',
    animation: 'slide_from_bottom',
    sheetGrabberVisible: true,
    sheetInitialDetentIndex: 0,  // Opens at 50%
    sheetAllowedDetents: [0.5, 0.75, 1],
    sheetCornerRadius: 20,
    sheetExpandsWhenScrolledToEdge: true,
    sheetElevation: 24,
  }}
/>
```

### Example 2: Settings Modal (75% initial height, custom header)
```typescript
<Stack.Screen
  name="settings-modal"
  options={{
    title: 'Settings',
    presentation: 'formSheet',
    gestureDirection: 'vertical',
    animation: 'slide_from_bottom',
    sheetGrabberVisible: true,
    sheetInitialDetentIndex: 1,  // Opens at 75%
    sheetAllowedDetents: [0.5, 0.75, 1],
    sheetCornerRadius: 20,
    sheetExpandsWhenScrolledToEdge: true,
    sheetElevation: 24,
    headerShown: false,  // Custom header in component
  }}
/>
```

### Example 3: Full-Screen First Modal
```typescript
<Stack.Screen
  name="editor"
  options={{
    title: 'Story Editor',
    presentation: 'formSheet',
    gestureDirection: 'vertical',
    animation: 'slide_from_bottom',
    sheetGrabberVisible: true,
    sheetInitialDetentIndex: 2,  // Opens at 100%
    sheetAllowedDetents: [0.7, 1],  // Only two detents
    sheetCornerRadius: 20,
    sheetExpandsWhenScrolledToEdge: true,
    sheetElevation: 24,
  }}
/>
```

### Example 4: Taller Content Modal
```typescript
<Stack.Screen
  name="story-preview"
  options={{
    title: 'Preview',
    presentation: 'formSheet',
    gestureDirection: 'vertical',
    animation: 'slide_from_bottom',
    sheetGrabberVisible: true,
    sheetInitialDetentIndex: 0,  // Opens at 70%
    sheetAllowedDetents: [0.7, 1],  // Taller first detent
    sheetCornerRadius: 20,
    sheetExpandsWhenScrolledToEdge: true,
    sheetElevation: 24,
  }}
/>
```

## File Locations

Add modal configurations to your Stack.Screen components in:
- **Root Layout**: `/app/_layout.tsx` - For app-wide modals
- **Tab Layout**: `/app/(tabs)/_layout.tsx` - For tab-specific modals (if needed)

## Common Mistakes to Avoid

❌ **DON'T** use `presentation: 'modal'` - Old style, no detents
❌ **DON'T** omit `sheetGrabberVisible: true` - Users won't know they can drag
❌ **DON'T** forget `gestureDirection: 'vertical'` - No swipe-to-dismiss
❌ **DON'T** use only `[1]` detent unless truly needed - Loses flexibility
❌ **DON'T** set detents without considering content height
❌ **DON'T** skip `animation: 'slide_from_bottom'` - Breaks expected behavior

## Visual Reference

The modal should appear with:
1. ✅ Rounded top corners (20px radius)
2. ✅ Grab handle visible at top center (horizontal line)
3. ✅ Smooth slide-up animation from bottom
4. ✅ Swipe down to dismiss
5. ✅ Multiple snap points (can resize by dragging)
6. ✅ Auto-expand when scrolling to edge
7. ✅ Shadow/elevation on Android

## When Adding a New Modal

Follow this checklist:

1. ✅ Copy the standard configuration from above
2. ✅ Set appropriate `title`
3. ✅ Choose `sheetInitialDetentIndex` (0 for simple, 1 for complex content)
4. ✅ Adjust `sheetAllowedDetents` if content needs different heights
5. ✅ Set `headerShown: false` only if using custom header
6. ✅ Keep all other options at standard values
7. ✅ Test on both iOS and Android
8. ✅ Verify grab handle is visible
9. ✅ Test swipe-to-dismiss gesture
10. ✅ Test detent snapping behavior

## Testing Checklist

After configuring a modal, verify:
- [ ] Modal slides up from bottom smoothly
- [ ] Grab handle is visible at top
- [ ] Can swipe down to dismiss
- [ ] Snaps to each defined detent
- [ ] Auto-expands when scrolling to top
- [ ] Rounded corners appear correctly
- [ ] Works on both iOS and Android
- [ ] Matches design from reference image

## Why This Configuration?

This standardized config ensures:
- **Consistency**: All modals behave the same way
- **Modern UX**: Matches iOS 18+ native sheet behavior
- **Flexibility**: Users can resize via detents
- **Discoverability**: Grab handle shows interactivity
- **Accessibility**: Swipe gestures are intuitive
- **Polish**: Rounded corners and smooth animations

## Related Documentation

- See `CLAUDE.md` section "Expo 54 Modal Configuration Standard" for reference
- Expo Router docs: https://docs.expo.dev/router/advanced/modals/
- React Navigation docs: https://reactnavigation.org/docs/modal/

---

**Remember**: Consistency is key. Always use this configuration for new modals to maintain a cohesive user experience across the entire app.
