# Expo SDK 54 - Key Features & Changes

You are working with **Expo SDK 54** (React Native 0.81, React 19.1)

## Critical New Features

### 1. iOS 26 Liquid Glass Support
- Use `expo-glass-effect` package with `<GlassView>` component
- Built on UIVisualEffectView
- Fallback to BlurView on iOS < 26
- See existing guide in main CLAUDE.md

### 2. Precompiled React Native for iOS
- **10x faster clean builds** (120s → 10s on M4 Max)
- Ships as XCFrameworks instead of source
- **NOT compatible with `use_frameworks!`** (builds from source if enabled)
- Can opt-out via `expo-build-properties`: `ios.buildReactNativeFromSource: false`

### 3. Android 16 Edge-to-Edge (ALWAYS ENABLED)
- **Cannot be disabled** in Android 16+
- `react-native-edge-to-edge` functionality built into React Native
- Use `androidNavigationBar.enforceContrast` in app.json instead of plugin
- All screens must handle edge-to-edge layouts

### 4. New expo-file-system API
- Old API now at `expo-file-system/legacy`
- New API is default export
- Object-oriented API for files/directories
- Supports SAF URIs (Android) and bundled assets

### 5. Expo Router 6 Features
- **Native tabs** (beta): `unstable_NativeTabs` - liquid glass tabs, auto-scroll
- Link previews, transitions, context menus (iOS)
- Server middleware (experimental)
- Web modals now emulate iPad/iPhone behavior

## Breaking Changes

### Reanimated v4
- **Only supports New Architecture**
- Requires `react-native-worklets`
- For Legacy Architecture: use Reanimated v3
- NativeWind users: STAY ON v3 (v4 not supported yet)

### Legacy Architecture Deprecation
- **SDK 54 = LAST SDK with Legacy Architecture support**
- SDK 55+ will be New Architecture only
- Start migration now!

### expo-av Removal
- **Will be removed in SDK 55**
- Migrate to `expo-audio` and `expo-video` NOW

### SafeAreaView Deprecation
- React Native's `<SafeAreaView>` deprecated
- Use `react-native-safe-area-context` instead (already in project)

### Metro Import Changes
- Internal imports changed from `metro/src/..` to `metro/private/..`
- Most apps won't be affected
- Update libraries if you see Metro import errors

## New Packages

### expo-app-integrity
- Verify app integrity via DeviceCheck (iOS) / Play Integrity API (Android)
- Confirms genuine App Store/Play Store install

### expo-blob (beta)
- W3C-compliant Blob API for native
- NOT in Expo Go yet

### expo-sqlite Enhancements
- localStorage API implementation (web-compatible)
- `loadExtensionAsync()` for SQLite extensions
- sqlite-vec bundled for vector/RAG AI work

## Expo Updates Improvements

### Runtime Header Overrides
```typescript
Updates.setUpdateRequestHeadersOverride({ channel: 'employee' })
```
- Safe for production (no anti-bricking flag needed)
- Immediate effect (no restart required)

### Download Progress
```typescript
const { downloadProgress } = useUpdates()
```
- Track update download progress
- Show progress bars

### Custom Reload Screens
```typescript
Updates.reloadAsync({
  reloadScreenOptions: {
    backgroundColor: '#7C3AED',
    image: require('./reload.jpg'),
    fade: true
  }
})
```

## Tool Requirements

- **Node.js**: ≥ 20.19.4 (LTS)
- **Xcode**: 16.1+ (Xcode 26 recommended for iOS 26 features)
- **Android**: Targets API 36 (Android 16)

## Autolinking Changes

- **Transitive dependencies now autolinked** (React Native modules)
- Links based on package.json dependencies, not node_modules scanning
- Unified behavior across Expo & React Native modules

**Opt-out if needed:**
```json
{
  "expo": {
    "autolinking": {
      "legacy_shallowReactNativeLinking": true,
      "searchPaths": ["node_modules"]
    }
  }
}
```

## Common Migration Issues

1. **Unhandled promise rejections now logged as errors** - fix any async error handling
2. **Precompiled RN issues** - set `ios.buildReactNativeFromSource: false` in expo-build-properties
3. **Edge-to-edge layout issues** - test all Android screens
4. **Reanimated v4 breakage** - downgrade to v3 or migrate to New Architecture
5. **expo-file-system imports** - change to `/legacy` for old API

## Using MCP for Latest Info

Always use Expo MCP for up-to-date docs:

```typescript
mcp__expo-mcp__search_documentation({
  query: "expo router native tabs"
})
```
