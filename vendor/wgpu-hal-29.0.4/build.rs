fn main() {
    cfg_aliases::cfg_aliases! {
        native: { not(target_arch = "wasm32") },
        send_sync: { any(
            not(target_arch = "wasm32"),
            all(feature = "fragile-send-sync-non-atomic-wasm", not(target_feature = "atomics"))
        ) },
        webgl: { all(target_arch = "wasm32", not(target_os = "emscripten"), gles) },
        Emscripten: { all(target_os = "emscripten", gles) },
        dx12: { all(target_os = "windows", feature = "dx12") },
        gles: { all(feature = "gles") },
        // Within the GL ES backend, use `std` and be Send + Sync only if we are using a target
        // that, among the ones where the GL ES backend is supported, has `std`.
        gles_with_std: { all(
            feature = "gles",
            any(
                not(target_arch = "wasm32"),
                // Accept wasm32-unknown-unknown, which uniquely has a stub `std`
                all(target_vendor = "unknown", target_os = "unknown"),
                // Accept wasm32-unknown-emscripten and similar, which has a real `std`
                target_os = "emscripten"
            )
        ) },
        metal: { all(target_vendor = "apple", feature = "metal") },
        vulkan: { all(not(target_arch = "wasm32"), feature = "vulkan") },
        any_backend: { any(dx12, metal, vulkan, gles) },
        // ⚠️ Keep in sync with target.cfg() definition in Cargo.toml and cfg_alias in `wgpu` crate ⚠️
        static_dxc: { all(target_os = "windows", feature = "static-dxc", not(target_arch = "aarch64"), target_env = "msvc") },
        supports_64bit_atomics: { target_has_atomic = "64" },
        supports_ptr_atomics: { target_has_atomic = "ptr" }
    }
}
