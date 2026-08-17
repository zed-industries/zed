fn main() {
    cfg_aliases::cfg_aliases! {
        dot_out: { feature = "dot-out" },
        glsl_out: { feature = "glsl-out" },
        hlsl_out: { any(feature = "hlsl-out", all(target_os = "windows", feature = "hlsl-out-if-target-windows")) },
        msl_out: { any(feature = "msl-out", all(target_vendor = "apple", feature = "msl-out-if-target-apple")) },
        spv_out: { feature = "spv-out" },
        wgsl_out: { feature = "wgsl-out" },
        std: { any(test, feature = "wgsl-in", feature = "stderr", feature = "fs") },
        no_std: { not(std) },
    }
}
