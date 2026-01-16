# Stack Screen UI Configuration Guide

**For Expo Router Stack Navigation with iOS Liquid Glass Support**

This guide covers complete stack screen configuration for Expo Router, with focus on iOS 26+ liquid glass effects and proper modal/sheet presentations.

---

## Table of Contents

1. [Platform Requirements](#platform-requirements)
2. [Basic Stack Screen Configuration](#basic-stack-screen-configuration)
3. [Liquid Glass Integration](#liquid-glass-integration)
4. [Complete Screen Options Reference](#complete-screen-options-reference)
5. [Implementation Examples](#implementation-examples)
6. [Fallback Patterns](#fallback-patterns)

---

## Platform Requirements

### iOS 26+ Features
The following features **ONLY work on iOS 26.0 and above**:

- **Liquid Glass Effect** (via `expo-glass-effect`)
- **Native iOS Liquid Glass Headers** (automatic in Expo Router)
- **Enhanced FormSheet presentations** with glass backgrounds

### Fallback Requirements
Always provide fallbacks for:
- iOS versions below 26.0 (e.g., iOS 18.3)
- Android (all versions)
- Web

### Installation

```bash
npx expo install expo-glass-effect
```

---

## Basic Stack Screen Configuration

### Minimal Configuration

```tsx
import { Stack } from 'expo-router';

<Stack.Screen
  name="route-name"
  options={{
    title: 'Screen Title',
    presentation: 'card', // default
  }}
/>
```

### iOS FormSheet Modal

```tsx
<Stack.Screen
  name="modal-route"
  options={{
    presentation: 'formSheet',
    sheetGrabberVisible: true,
    sheetAllowedDetents: [0.5, 0.75, 1],
    sheetInitialDetentIndex: 0,
  }}
/>
```

---

## Liquid Glass Integration

### Utility Function for Availability Check

```typescript
import { Platform } from 'react-native';
import { isLiquidGlassAvailable } from 'expo-glass-effect';

// Check if iOS 26+ and liquid glass is available
export const canUseLiquidGlass = (): boolean => {
  if (Platform.OS !== 'ios') return false;

  const version = parseInt(Platform.Version as string, 10);
  if (version < 26) return false;

  return isLiquidGlassAvailable();
};
```

### Stack Screen with Conditional Liquid Glass

```tsx
import { Stack } from 'expo-router';
import { Platform } from 'react-native';
import { isLiquidGlassAvailable } from 'expo-glass-effect';
import * as Device from 'expo-device';

<Stack.Screen
  name="talk/[talkId]"
  options={{
    // Transparent header for glass effect
    headerTransparent: Platform.OS === 'ios' ? true : false,
    headerLargeTitle: false,
    title: '',

    // Presentation mode based on platform and availability
    presentation:
      Platform.OS === 'ios'
        ? isLiquidGlassAvailable() && Device.osName !== 'iPadOS'
          ? 'formSheet'
          : 'modal'
        : 'modal',

    // Sheet configuration
    sheetGrabberVisible: true,
    sheetAllowedDetents: [0.8],
    sheetInitialDetentIndex: 0,

    // Transparent background for liquid glass
    contentStyle: {
      backgroundColor: isLiquidGlassAvailable()
        ? 'transparent'
        : '#FFFFFF', // or your theme background color
    },

    // Header style
    headerStyle: {
      backgroundColor: Platform.OS === 'ios'
        ? 'transparent'
        : '#FFFFFF',
    },
  }}
/>
```

### Using GlassView Component

```tsx
import { GlassView } from 'expo-glass-effect';
import { canUseLiquidGlass } from '@/utils/platform';

// Inside your screen component
{canUseLiquidGlass() ? (
  <GlassView
    style={styles.container}
    glassEffectStyle="clear"
    isInteractive={true}
  >
    {/* Your content */}
  </GlassView>
) : (
  <View style={[styles.container, { backgroundColor: '#FFFFFF' }]}>
    {/* Your content */}
  </View>
)}
```

---

## Complete Screen Options Reference

### Presentation Modes

```typescript
presentation:
  | 'card'                // Default stack behavior
  | 'modal'               // Full-screen modal from bottom
  | 'transparentModal'    // Modal with transparent background
  | 'formSheet'           // iOS sheet (bottom sheet)
  | 'fullScreenModal'     // Full screen, no swipe dismiss
  | 'containedModal'      // Modal within navigation context
  | 'containedTransparentModal' // Transparent contained modal
```

### Sheet Configuration (iOS FormSheet)

```typescript
// Sheet detents (available heights)
sheetAllowedDetents: [0.5, 0.75, 1], // 50%, 75%, 100% of screen height
sheetAllowedDetents: ['large', 'medium'], // Named detents (iOS 16+)

// Initial position
sheetInitialDetentIndex: 0, // Start at first detent (0-indexed)

// Visual customization
sheetGrabberVisible: true,           // Show handle at top
sheetCornerRadius: 20,               // Rounded corners (iOS)
sheetElevation: 24,                  // Shadow depth (Android)
sheetExpandsWhenScrolledToEdge: true, // Auto-expand on scroll

// Sheet ID for programmatic control
sheetId: 'my-sheet-id',

// Prevent dismissal
sheetAllowsDismiss: false, // User cannot dismiss
```

### Gesture Configuration

```typescript
// Swipe direction for dismissal
gestureDirection: 'vertical' | 'horizontal' | 'vertical-inverted' | 'horizontal-inverted'

// Gesture behavior
gestureEnabled: true,           // Enable swipe to dismiss
fullScreenGestureEnabled: true, // Allow swipe from anywhere (iOS 13+)
customAnimationOnGesture: true, // Custom animation timing
```

### Animation Options

```typescript
animation:
  | 'default'           // Platform default
  | 'fade'              // Fade in/out
  | 'flip'              // 3D flip
  | 'slide_from_right'  // Slide from right (default iOS)
  | 'slide_from_left'   // Slide from left
  | 'slide_from_bottom' // Slide from bottom
  | 'none'              // No animation

// Custom duration (iOS only)
animationDuration: 300, // milliseconds
```

### Header Configuration

```typescript
// Title
title: 'Screen Title',
headerTitle: 'Custom Title',
headerTitle: ({ children, tintColor }) => <CustomComponent />,

// Large title (iOS)
headerLargeTitle: true,
headerLargeTitleShadowVisible: false,
headerLargeStyle: { backgroundColor: '#FFFFFF' },

// Transparency and blur
headerTransparent: true,
headerBlurEffect: 'systemMaterial', // iOS blur style

// Search bar (iOS)
headerSearchBarOptions: {
  placeholder: 'Search...',
  onChangeText: (text) => console.log(text),
  autoFocus: false,
  tintColor: '#7C3AED',
},

// Custom buttons
headerLeft: ({ tintColor }) => (
  <TouchableOpacity onPress={() => router.back()}>
    <Ionicons name="close" size={24} color={tintColor} />
  </TouchableOpacity>
),

headerRight: ({ tintColor }) => (
  <TouchableOpacity onPress={handleSave}>
    <Text style={{ color: tintColor }}>Save</Text>
  </TouchableOpacity>
),

// Back button customization (iOS)
headerBackTitle: 'Back',
headerBackTitleVisible: true,
headerBackButtonDisplayMode: 'default' | 'generic' | 'minimal',
headerBackButtonMenuEnabled: true, // Long-press menu (iOS 14+)

// Hide header
headerShown: false,

// Complete custom header
header: ({ navigation, route, options }) => <CustomHeader />,

// Header style
headerStyle: {
  backgroundColor: '#7C3AED',
},
headerTintColor: '#FFFFFF', // Icon/text color
headerTitleStyle: {
  fontWeight: 'bold',
  fontSize: 18,
},

// Shadow
headerShadowVisible: true,
```

### Content Style

```typescript
contentStyle: {
  backgroundColor: 'transparent', // For glass effect
  backgroundColor: '#FFFFFF',     // Solid color
},
```

### Status Bar (iOS)

```typescript
statusBarAnimation: 'fade' | 'slide' | 'none',
statusBarHidden: false,
statusBarStyle: 'auto' | 'inverted' | 'dark' | 'light',
```

### Orientation (iOS)

```typescript
orientation: 'default' | 'portrait' | 'portrait_up' | 'portrait_down'
  | 'landscape' | 'landscape_left' | 'landscape_right' | 'all',
```

---

## Implementation Examples

### Example 1: Standard Modal with Liquid Glass

```tsx
<Stack.Screen
  name="create-story"
  options={{
    presentation: 'formSheet',
    sheetGrabberVisible: true,
    sheetAllowedDetents: [0.5, 0.75, 1],
    sheetInitialDetentIndex: 1, // Start at 75%
    headerTransparent: canUseLiquidGlass(),
    contentStyle: {
      backgroundColor: canUseLiquidGlass() ? 'transparent' : '#FFFFFF',
    },
    headerTitle: 'Create Story',
    headerLeft: ({ tintColor }) => (
      <TouchableOpacity onPress={() => router.back()}>
        <Ionicons name="close" size={24} color={tintColor} />
      </TouchableOpacity>
    ),
  }}
/>
```

### Example 2: Full-Screen Modal (No Liquid Glass)

```tsx
<Stack.Screen
  name="story-reader"
  options={{
    presentation: 'fullScreenModal',
    animation: 'fade',
    headerShown: false,
    contentStyle: {
      backgroundColor: '#1A1C23', // Dark background
    },
    gestureEnabled: false, // Disable swipe dismiss
  }}
/>
```

### Example 3: Settings Screen with Search

```tsx
<Stack.Screen
  name="settings"
  options={{
    title: 'Settings',
    headerLargeTitle: true,
    headerSearchBarOptions: {
      placeholder: 'Search settings...',
      hideWhenScrolling: true,
    },
    headerRight: ({ tintColor }) => (
      <TouchableOpacity onPress={handleReset}>
        <Text style={{ color: tintColor }}>Reset</Text>
      </TouchableOpacity>
    ),
  }}
/>
```

### Example 4: Conditional iPad/iPhone Layout

```tsx
import * as Device from 'expo-device';

<Stack.Screen
  name="detail"
  options={{
    presentation: Device.osName === 'iPadOS' ? 'modal' : 'formSheet',
    sheetGrabberVisible: Device.osName !== 'iPadOS',
    sheetAllowedDetents: Device.osName === 'iPadOS'
      ? undefined
      : [0.8],
  }}
/>
```

---

## Fallback Patterns

### Pattern 1: GlassView with Fallback

```tsx
import { GlassView } from 'expo-glass-effect';
import { BlurView } from 'expo-blur';
import { canUseLiquidGlass } from '@/utils/platform';

const GlassContainer = ({ children, style }) => {
  if (canUseLiquidGlass()) {
    return (
      <GlassView style={style} glassEffectStyle="clear" isInteractive>
        {children}
      </GlassView>
    );
  }

  // Fallback: BlurView for iOS < 26
  if (Platform.OS === 'ios') {
    return (
      <BlurView intensity={40} tint="systemMaterialLight" style={style}>
        {children}
      </BlurView>
    );
  }

  // Fallback: Solid background for Android
  return (
    <View style={[style, { backgroundColor: 'rgba(255, 255, 255, 0.95)' }]}>
      {children}
    </View>
  );
};
```

### Pattern 2: Transparent Background Check

```tsx
const getBackgroundColor = () => {
  if (canUseLiquidGlass()) {
    return 'transparent';
  }

  const colorScheme = useColorScheme();
  return colorScheme === 'dark' ? '#1A1C23' : '#FFFFFF';
};

<Stack.Screen
  name="modal"
  options={{
    contentStyle: {
      backgroundColor: getBackgroundColor(),
    },
  }}
/>
```

### Pattern 3: Header Transparency

```tsx
const getHeaderConfig = () => {
  const hasGlass = canUseLiquidGlass();

  return {
    headerTransparent: hasGlass,
    headerBlurEffect: hasGlass ? undefined : 'systemMaterial',
    headerStyle: {
      backgroundColor: hasGlass ? 'transparent' : '#FFFFFF',
    },
  };
};

<Stack.Screen
  name="screen"
  options={{
    ...getHeaderConfig(),
    title: 'Screen',
  }}
/>
```

---

## GlassView Component API

### Props

```typescript
interface GlassViewProps extends ViewProps {
  // Glass effect style
  glassEffectStyle?: 'clear' | 'regular'; // default: 'regular'

  // Interactive touch effects
  isInteractive?: boolean; // default: false
  // ⚠️ Can only be set on mount, cannot change dynamically

  // Custom tint color
  tintColor?: string;

  // Color scheme
  colorScheme?: 'light' | 'dark' | 'system';
}
```

### Usage

```tsx
import { GlassView, isLiquidGlassAvailable } from 'expo-glass-effect';

// Check availability first
if (isLiquidGlassAvailable()) {
  <GlassView
    style={{
      width: 200,
      height: 100,
      borderRadius: 20,
      padding: 16,
    }}
    glassEffectStyle="clear"
    isInteractive={true}
    tintColor="#7C3AED"
  >
    <Text>Glass Content</Text>
  </GlassView>
}
```

### GlassContainer (Multiple Glass Elements)

```tsx
import { GlassContainer, GlassView } from 'expo-glass-effect';

<GlassContainer spacing={20} style={styles.container}>
  <GlassView glassEffectStyle="clear" style={styles.panel1}>
    <Text>Panel 1</Text>
  </GlassView>

  <GlassView glassEffectStyle="clear" style={styles.panel2}>
    <Text>Panel 2</Text>
  </GlassView>
</GlassContainer>
```

**Note:** `spacing` defines the distance threshold where glass elements start morphing/affecting each other.

---

## Best Practices

### 1. Always Check Platform Support

```tsx
// ✅ Good
if (canUseLiquidGlass()) {
  // Use liquid glass
} else {
  // Use fallback
}

// ❌ Bad
<GlassView style={styles.container}>
  {/* No fallback - will break on Android/old iOS */}
</GlassView>
```

### 2. Provide Proper Fallbacks

Liquid glass should enhance UX, not break it:
- iOS 26+: Liquid glass
- iOS < 26: BlurView
- Android: Semi-transparent solid background
- Web: Solid background with shadow

### 3. Don't Overuse Liquid Glass

Use for:
- Navigation bars/headers
- Action buttons
- Key UI overlays
- Modal sheets

Avoid for:
- Full-screen backgrounds
- Text-heavy content areas
- Rapidly animating elements

### 4. Monitor Performance

Liquid glass is GPU-intensive:
- Limit simultaneous glass views (< 5)
- Avoid animating glass effects rapidly
- Test on older devices (iPhone 15/16)

### 5. Ensure Content Readability

Always check contrast behind glass:
- Use `tintColor` for better contrast
- Test with different wallpapers
- Provide solid backgrounds for text

### 6. Handle Sheet Detents Properly

```tsx
// ✅ Good - Multiple snap points
sheetAllowedDetents: [0.5, 0.75, 1],
sheetInitialDetentIndex: 0,

// ❌ Bad - Single detent (feels rigid)
sheetAllowedDetents: [1],
```

### 7. iPadOS Consideration

FormSheet on iPad looks different:
```tsx
Device.osName === 'iPadOS' ? 'modal' : 'formSheet'
```

---

## Troubleshooting

### Issue: Glass Effect Not Showing

**Check:**
1. iOS version >= 26?
2. `isLiquidGlassAvailable()` returns true?
3. Background is transparent?
4. Device supports feature (not in low power mode)?

### Issue: Content Not Visible Behind Glass

**Solution:**
```tsx
contentStyle: {
  backgroundColor: 'transparent', // Not 'rgba(0,0,0,0)'
}
```

### Issue: Header Overlapping Content

**Solution:**
```tsx
headerTransparent: true,

// Add padding in your component
<View style={{ paddingTop: insets.top + 44 }}>
  {content}
</View>
```

### Issue: Sheet Not Dismissible

**Check:**
```tsx
gestureEnabled: true,        // Enable gesture
sheetAllowsDismiss: true,    // Allow dismissal
```

---

## Additional Resources

- [Expo Stack Documentation](https://docs.expo.dev/router/advanced/stack/)
- [expo-glass-effect Package](https://docs.expo.dev/versions/latest/sdk/glass-effect/)
- [Apple Liquid Glass Guidelines](https://developer.apple.com/documentation/technologyoverviews/adopting-liquid-glass)
- [Callstack Liquid Glass Blog](https://www.callstack.com/blog/how-to-use-liquid-glass-in-react-native)

---

## Quick Reference

### iOS 26+ Only Features
- `GlassView` component
- `isLiquidGlassAvailable()` utility
- Native liquid glass headers
- Enhanced FormSheet with glass

### Always Required
- Platform detection
- Fallback UI for non-iOS-26 platforms
- Proper contrast/readability
- Performance testing

### Common Pitfall
❌ Setting `isInteractive` dynamically (not supported)
✅ Set `isInteractive` on mount only
