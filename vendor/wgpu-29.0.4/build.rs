fn main() {
    cfg_aliases::cfg_aliases! {
        native: { not(target_arch = "wasm32") },
        Emscripten: { all(target_arch = "wasm32", target_os = "emscripten") },
        web: { all(target_arch = "wasm32", not(Emscripten), feature = "web") },

        send_sync: { any(
            native,
            all(feature = "fragile-send-sync-non-atomic-wasm", not(target_feature = "atomics"))
        ) },

        // Backends - keep this in sync with `wgpu-core/Cargo.toml` & docs in `wgpu/Cargo.toml`
        webgpu: { all(not(native), not(Emscripten), feature = "webgpu") },
        webgl: { all(not(native), not(Emscripten), feature = "webgl") },
        dx12: { all(target_os = "windows", feature = "dx12") },
        metal: { all(target_vendor = "apple", feature = "metal") },
        vulkan: { any(
            // The `vulkan` feature enables the Vulkan backend only on "native Vulkan" platforms, i.e. Windows/Linux/Android
            all(any(windows, target_os = "linux", target_os = "android", target_os = "freebsd"), feature = "vulkan"),
            // On Apple platforms, however, we require the `vulkan-portability` feature
            // to explicitly opt-in to Vulkan since it's meant to be used with MoltenVK.
            all(target_vendor = "apple", feature = "vulkan-portability")
        ) },
        gles: { any(
            // The `gles` feature enables the OpenGL/GLES backend only on "native OpenGL" platforms, i.e. Windows, Linux, Android, and Emscripten.
            // (Note that WebGL is also not included here!)
            all(any(windows, target_os = "linux", target_os = "android", target_os = "freebsd", Emscripten), feature = "gles"),
            // On Apple platforms, however, we require the `angle` feature to explicitly opt-in to OpenGL
            // since its meant to be used with ANGLE.
            all(target_vendor = "apple", feature = "angle")
        ) },
        noop: { feature = "noop" },

        wgpu_core: {
            any(
                // On native, wgpu_core is currently always enabled, even if there's no backend enabled at all.
                native,
                // `wgpu_core` is implied if any backend other than WebGPU is enabled.
                // (this is redundant except for `gles` and `noop`)
                webgl, dx12, metal, vulkan, gles, noop
            )
        },

        // This alias is _only_ if _we_ need naga in the wrapper. wgpu-core provides
        // its own re-export of naga, which can be used in other situations
        naga: { any(feature = "naga-ir", feature = "spirv", feature = "glsl") },
        // ⚠️ Keep in sync with target.cfg() definition in wgpu-hal/Cargo.toml and cfg_alias in `wgpu-hal` crate ⚠️
        static_dxc: { all(target_os = "windows", feature = "static-dxc", not(target_arch = "aarch64"), target_env = "msvc") },
        supports_64bit_atomics: { target_has_atomic = "64" },
        custom: {any(feature = "custom")},
        std: { any(
            feature = "std",
            // TODO: Remove this when an alternative Mutex implementation is available for `no_std`.
            // send_sync requires an appropriate Mutex implementation, which is only currently
            // possible with `std` enabled.
            send_sync,
            // Unwinding panics necessitate access to `std` to determine if a thread is panicking
            panic = "unwind"
        ) },
        no_std: { not(std) }
    }
}
