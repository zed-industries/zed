---
paths:
  - "ios/**/*.swift"
---

# iOS Swift Rules

## Architecture
- The Swift layer is a THIN host app — it bootstraps UIKit and calls into Rust via C FFI
- All substantive logic (rendering, input, networking, editor) lives in Rust crates
- Swift handles: UIScene lifecycle, UIKeyCommand registration, UIDocumentPicker, Keychain UI

## FFI bridge pattern
- Rust exports `#[no_mangle] pub extern "C" fn` symbols from `crates/zed-ios/`
- Swift imports via `ZedApp-Bridging-Header.h` (auto-generated C header)
- Pass strings as `*const c_char`, booleans as `Bool`, callbacks as function pointers
- Never pass Rust-owned memory to Swift without a corresponding free function

## UIKit conventions
- Deployment target: iPadOS 17.0
- Use UIScene-based lifecycle (UIWindowSceneDelegate), NOT the old UIApplicationDelegate
  window management — though AppDelegate is still needed for global setup
- UIApplicationSceneManifest in Info.plist with UIApplicationSupportsMultipleScenes = YES
- Each UIWindowScene = one Zed workspace (can connect to different hosts)

## Keyboard input
- Register UIKeyCommand on root UIViewController for the ⌘-hold discoverability HUD
- Override pressesBegan:withEvent: and pressesEnded:withEvent: for raw key capture
- System-reserved shortcuts CANNOT be captured: Cmd+Tab, Cmd+H, Cmd+Space, Globe+key
- Ctrl-based shortcuts ARE fully available on iPadOS (unlike macOS)

## Text input
- Implement full UITextInput protocol on the Metal view (not hidden UITextField trampoline)
- Disable autocorrect, smart quotes, smart dashes, spell checking
- Return nil/empty for methods not relevant to a code editor

## Never do
- Never use Process() / NSTask — iOS prohibits subprocess spawning
- Never access paths outside the sandbox without security-scoped bookmarks
- Never write SSH keys to disk — Keychain only (Security.framework)
- Never import AppKit or reference NSApplication
