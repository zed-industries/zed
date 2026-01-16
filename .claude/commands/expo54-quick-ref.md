# Quick Reference: Expo SDK 54 Commands & Patterns

## Installation & Upgrade

```bash
# Upgrade to SDK 54
npx expo install expo@^54.0.0 --fix

# Check autolinking
npx expo-modules-autolinking verify -v

# Run Expo Doctor
npx expo-doctor
```

## New Architecture Migration

```bash
# Enable New Architecture (required for Reanimated v4)
# In app.json:
{
  "expo": {
    "newArchEnabled": true
  }
}

# Check if using New Architecture
npx expo config --type introspect | grep newArchEnabled
```

## Liquid Glass (iOS 26+)

```tsx
import { GlassView } from 'expo-glass-effect';
import { canUseLiquidGlass } from '@/utils/platform';

{canUseLiquidGlass() ? (
  <GlassView glassEffectStyle="clear" isInteractive>
    {content}
  </GlassView>
) : (
  <BlurView intensity={40}>{content}</BlurView>
)}
```

## Edge-to-Edge (Android 16)

```json
// app.json
{
  "android": {
    "androidNavigationBar": {
      "enforceContrast": false
    }
  }
}
```

```tsx
import { SafeAreaView } from 'react-native-safe-area-context';

<SafeAreaView edges={['top', 'bottom']}>
  {content}
</SafeAreaView>
```

## expo-file-system Migration

```tsx
// Old way (now legacy)
import * as FileSystem from 'expo-file-system/legacy';

// New way (default in SDK 54)
import { FileSystem } from 'expo-file-system';

const file = FileSystem.documentDirectory.getFile('data.json');
await file.write(JSON.stringify(data));
const content = await file.readAsString();
```

## Expo Router 6 - Native Tabs (Beta)

```tsx
import { Tabs } from '@expo/ui/tabs'; // or unstable_NativeTabs

<Tabs>
  <Tabs.Screen
    name="index"
    options={{
      tabBarIcon: ({ color }) => <Icon name="home" color={color} />
    }}
  />
</Tabs>
```

## Expo Updates

```tsx
import * as Updates from 'expo-updates';

// Override channel at runtime
Updates.setUpdateRequestHeadersOverride({ channel: 'beta' });

// Show reload screen
Updates.reloadAsync({
  reloadScreenOptions: {
    backgroundColor: '#7C3AED',
    image: require('./reload.png'),
    fade: true
  }
});

// Track download progress
const { downloadProgress } = useUpdates();
```

## expo-sqlite localStorage

```tsx
import { openDatabaseSync } from 'expo-sqlite';

const db = openDatabaseSync('mydb.db');

// Use localStorage API
localStorage.setItem('key', 'value');
const value = localStorage.getItem('key');
```

## Build Configuration

### Disable Precompiled React Native (if issues)
```json
// app.json
{
  "plugins": [
    [
      "expo-build-properties",
      {
        "ios": {
          "buildReactNativeFromSource": false
        }
      }
    ]
  ]
}
```

### Enable Predictive Back (Android)
```json
{
  "android": {
    "predictiveBackGestureEnabled": true
  }
}
```

## Debugging

```bash
# Check for Metro import issues
npx expo start --clear

# Verify native modules
npx expo-modules-autolinking verify -v

# Check build errors (iOS)
npx expo run:ios --configuration Release

# Check build errors (Android)
npx expo run:android --variant release
```

## Common Patterns

### Reanimated v3 with SDK 54 (Legacy Arch)
```bash
npx expo install react-native-reanimated@~3.16.3
```

### Check Platform Version
```tsx
import { Platform } from 'react-native';

const iosVersion = parseInt(Platform.Version as string, 10);
if (iosVersion >= 26) {
  // Use iOS 26 features
}
```

### MCP Documentation Search
```bash
# In Claude Code, use:
mcp__expo-mcp__search_documentation({ query: "your question" })
```

## Upgrade Checklist

- [ ] Update to Node 20.19.4+
- [ ] Update to Xcode 16.1+
- [ ] Run `npx expo install expo@^54.0.0 --fix`
- [ ] Check for Reanimated v4 compatibility
- [ ] Test edge-to-edge on Android
- [ ] Update expo-file-system imports
- [ ] Test on iOS 26 for liquid glass
- [ ] Run `npx expo-doctor`
- [ ] Test builds: `npx expo run:ios` / `npx expo run:android`
