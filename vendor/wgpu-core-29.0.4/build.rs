fn main() {
    cfg_aliases::cfg_aliases! {
        windows_linux_android: { any(windows, target_os = "linux", target_os = "android", target_os = "freebsd") },
        send_sync: { all(
            feature = "std",
            any(
                not(target_arch = "wasm32"),
                all(feature = "fragile-send-sync-non-atomic-wasm", not(target_feature = "atomics"))
            )
        ) },
        dx12: { all(target_os = "windows", feature = "dx12") },
        webgl: { all(target_arch = "wasm32", not(target_os = "emscripten"), feature = "webgl") },
        gles: { any(
            all(windows_linux_android, feature = "gles"), // Regular GLES
            all(webgl), // WebGL
            all(target_os = "emscripten", feature = "gles"), // Emscripten GLES
            all(target_vendor = "apple", feature = "angle") // ANGLE on Apple
        ) },
        vulkan: { any(
            all(windows_linux_android, feature = "vulkan"), // Regular Vulkan
            all(target_vendor = "apple", feature = "vulkan-portability") // Vulkan Portability on Apple
        ) },
        metal: { all(target_vendor = "apple", feature = "metal") },

        supports_64bit_atomics: { target_has_atomic = "64" }
    }
}
