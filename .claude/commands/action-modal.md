# Action Modal Pattern (pageSheet with BlurView Backdrop)

You are creating a route-based action modal using the standardized pattern that provides a native iOS sheet experience with a blurred backdrop that fades in while content slides up.

## When to Use This Pattern

Use this pattern for:
- Action sheets (like story actions, share options, etc.)
- Quick menus with a few options
- Confirmation dialogs
- Any modal that should fit to its content size

**Key Difference from formSheet**: This pattern uses `pageSheet` with `fitToContents` for automatic height sizing, and includes a BlurView backdrop for a polished native feel.

## Complete Implementation Guide

### Step 1: Create the Modal Route File

Create a new file at `app/[modal-name].tsx`:

```tsx
import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  useColorScheme,
} from 'react-native';
import { useRouter, useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { BlurView } from 'expo-blur';
import { Colors, BrandColors, InteractiveColors } from '@/constants/theme';

export default function YourActionModal() {
  const router = useRouter();
  const params = useLocalSearchParams();
  const colorScheme = useColorScheme();
  const colors = Colors[colorScheme ?? 'light'];
  const isDark = colorScheme === 'dark';

  // Get params passed from navigation
  const itemId = params.itemId as string;

  const handleClose = () => {
    router.back();
  };

  const handleAction = () => {
    console.log('Action performed for:', itemId);
    router.back();
  };

  return (
    <View style={styles.modalWrapper}>
      {/* Blurred background overlay - MUST be sibling with absoluteFillObject */}
      <BlurView
        intensity={isDark ? 60 : 40}
        tint={isDark ? 'dark' : 'light'}
        style={styles.blurOverlay}
      />

      {/* Content container */}
      <View style={[styles.container, { backgroundColor: colors.background }]}>
        {/* Grabber handle */}
        <View style={styles.grabberContainer}>
          <View style={[styles.grabber, { backgroundColor: colors.border }]} />
        </View>

        {/* Header */}
        <View style={[styles.header, { borderBottomColor: colors.border }]}>
          <Text style={[styles.headerTitle, { color: colors.text }]}>Actions</Text>
          <TouchableOpacity onPress={handleClose} style={styles.closeButton}>
            <Ionicons name="close" size={28} color={colors.text} />
          </TouchableOpacity>
        </View>

        {/* Menu Items */}
        <View style={styles.menuContent}>
          {/* Example menu item */}
          <TouchableOpacity
            style={[styles.menuItem, { backgroundColor: colors.card }]}
            onPress={handleAction}
            activeOpacity={0.7}
          >
            <View style={[styles.menuIconContainer, { backgroundColor: BrandColors.primary + '15' }]}>
              <Ionicons name="share-outline" size={22} color={BrandColors.primary} />
            </View>
            <View style={styles.menuItemTextContainer}>
              <Text style={[styles.menuItemTitle, { color: colors.text }]}>Action Title</Text>
              <Text style={[styles.menuItemSubtitle, { color: colors.mutedForeground }]}>
                Action description
              </Text>
            </View>
            <Ionicons name="chevron-forward" size={20} color={colors.mutedForeground} />
          </TouchableOpacity>

          {/* Divider before destructive action */}
          <View style={[styles.divider, { backgroundColor: colors.border }]} />

          {/* Destructive action example */}
          <TouchableOpacity
            style={[styles.menuItem, { backgroundColor: InteractiveColors.error + '10' }]}
            onPress={() => { /* handle delete */ router.back(); }}
            activeOpacity={0.7}
          >
            <View style={[styles.menuIconContainer, { backgroundColor: InteractiveColors.error + '15' }]}>
              <Ionicons name="trash-outline" size={22} color={InteractiveColors.error} />
            </View>
            <View style={styles.menuItemTextContainer}>
              <Text style={[styles.menuItemTitle, { color: InteractiveColors.error }]}>Delete</Text>
              <Text style={[styles.menuItemSubtitle, { color: InteractiveColors.error + '99' }]}>
                This action cannot be undone
              </Text>
            </View>
          </TouchableOpacity>
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  modalWrapper: {
    flex: 1,
  },
  blurOverlay: {
    ...StyleSheet.absoluteFillObject,
    zIndex: -1,
  },
  container: {
    flex: 1,
  },
  grabberContainer: {
    alignItems: 'center',
    paddingTop: 12,
    paddingBottom: 8,
  },
  grabber: {
    width: 36,
    height: 5,
    borderRadius: 3,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingTop: 8,
    paddingBottom: 16,
    borderBottomWidth: 1,
  },
  headerTitle: {
    fontSize: 24,
    fontWeight: '700',
  },
  closeButton: {
    padding: 4,
  },
  menuContent: {
    padding: 20,
    gap: 12,
  },
  menuItem: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: 16,
    borderRadius: 16,
    gap: 14,
  },
  menuIconContainer: {
    width: 44,
    height: 44,
    borderRadius: 12,
    justifyContent: 'center',
    alignItems: 'center',
  },
  menuItemTextContainer: {
    flex: 1,
  },
  menuItemTitle: {
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 2,
  },
  menuItemSubtitle: {
    fontSize: 13,
  },
  divider: {
    height: 1,
    marginVertical: 4,
  },
});
```

### Step 2: Add Stack.Screen in _layout.tsx

Add to `app/_layout.tsx` inside your Stack:

```tsx
<Stack.Screen
  name="your-action-modal"
  options={{
    title: 'Actions',
    presentation: 'pageSheet',
    gestureDirection: 'vertical',
    animation: 'slide_from_bottom',
    sheetGrabberVisible: true,
    sheetAllowedDetents: 'fitToContents',
    sheetCornerRadius: 20,
    headerShown: false,
  }}
/>
```

### Step 3: Navigate to the Modal

From any component:

```tsx
import { useRouter } from 'expo-router';

const router = useRouter();

// Navigate with params
router.push({
  pathname: '/your-action-modal',
  params: { itemId: item.id.toString(), from: 'source-screen' },
});
```

## Critical Configuration Details

### `sheetAllowedDetents: 'fitToContents'`
- **Purpose**: Modal automatically sizes to fit content
- **Effect**: No wasted space below content
- **When to use**: Action sheets, menus with fixed number of items

### `presentation: 'pageSheet'`
- **Purpose**: iOS page sheet presentation (different from formSheet)
- **Effect**: Native sheet behavior with proper backdrop handling

### BlurView Structure
```tsx
<View style={styles.modalWrapper}>
  {/* BlurView MUST be sibling, NOT wrapper */}
  <BlurView style={styles.blurOverlay} />  {/* absoluteFillObject, zIndex: -1 */}

  <View style={styles.container}>
    {/* Your content */}
  </View>
</View>
```

**Why this structure?**
- BlurView as absolute sibling fades in with native animation
- Content slides up independently
- Avoids the "backdrop sliding up" problem

## Required Styles

```tsx
blurOverlay: {
  ...StyleSheet.absoluteFillObject,
  zIndex: -1,  // CRITICAL: Must be behind content
},
```

## Dark Theme Support

```tsx
const isDark = colorScheme === 'dark';

<BlurView
  intensity={isDark ? 60 : 40}
  tint={isDark ? 'dark' : 'light'}
  style={styles.blurOverlay}
/>
```

## Menu Item Pattern

For consistent menu items:

```tsx
<TouchableOpacity
  style={[styles.menuItem, { backgroundColor: colors.card }]}
  onPress={handleAction}
  activeOpacity={0.7}
>
  <View style={[styles.menuIconContainer, { backgroundColor: BrandColors.primary + '15' }]}>
    <Ionicons name="icon-name" size={22} color={BrandColors.primary} />
  </View>
  <View style={styles.menuItemTextContainer}>
    <Text style={[styles.menuItemTitle, { color: colors.text }]}>Title</Text>
    <Text style={[styles.menuItemSubtitle, { color: colors.mutedForeground }]}>
      Subtitle
    </Text>
  </View>
  <Ionicons name="chevron-forward" size={20} color={colors.mutedForeground} />
</TouchableOpacity>
```

## Destructive Action Pattern

```tsx
<TouchableOpacity
  style={[styles.menuItem, { backgroundColor: InteractiveColors.error + '10' }]}
  onPress={handleDelete}
>
  <View style={[styles.menuIconContainer, { backgroundColor: InteractiveColors.error + '15' }]}>
    <Ionicons name="trash-outline" size={22} color={InteractiveColors.error} />
  </View>
  <View style={styles.menuItemTextContainer}>
    <Text style={[styles.menuItemTitle, { color: InteractiveColors.error }]}>Delete</Text>
    <Text style={[styles.menuItemSubtitle, { color: InteractiveColors.error + '99' }]}>
      This action cannot be undone
    </Text>
  </View>
</TouchableOpacity>
```

## Checklist for New Action Modals

1. [ ] Create route file at `app/[modal-name].tsx`
2. [ ] Use BlurView as absolute sibling (NOT wrapper)
3. [ ] Add grabber handle
4. [ ] Add header with title and close button
5. [ ] Add Stack.Screen in `_layout.tsx`
6. [ ] Use `sheetAllowedDetents: 'fitToContents'`
7. [ ] Support dark theme with intensity/tint props
8. [ ] Pass necessary params via navigation
9. [ ] Use `router.back()` to close
10. [ ] Test on both phone and tablet

## Example Reference

See `app/story-actions-modal.tsx` for a complete working example.

## Common Mistakes

❌ **DON'T** wrap content in BlurView - backdrop will slide with content
❌ **DON'T** use inline `<Modal>` component - use route-based navigation
❌ **DON'T** forget `zIndex: -1` on blurOverlay
❌ **DON'T** use fixed detent values for action sheets - use `fitToContents`
❌ **DON'T** forget `headerShown: false` in Stack.Screen options

## Tablet vs Phone Behavior

The `pageSheet` presentation adapts to device type automatically:

| Device | Behavior |
|--------|----------|
| **Phone** | Full-width sheet slides up from bottom, backdrop fades in |
| **Tablet (iPad)** | Centered card presentation, narrower width, dimmed backdrop |

**No separate code needed** - iOS handles the device-appropriate presentation automatically. The same route-based modal code works on both devices with native-feeling behavior for each.

## Why This Pattern?

- **Native feel**: Uses iOS pageSheet presentation
- **Proper animations**: Backdrop fades, content slides
- **Auto-sizing**: Fits to content, no wasted space
- **Dark theme**: Automatic blur intensity adjustment
- **Consistent**: Matches iOS system sheets
- **Reusable**: One modal file, navigate from anywhere
- **Universal**: Same code works on phone AND tablet
