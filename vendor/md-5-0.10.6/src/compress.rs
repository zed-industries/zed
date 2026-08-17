cfg_if::cfg_if! {
    if #[cfg(feature = "force-soft")] {
        mod soft;
        pub use soft::compress;
    } else if #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))] {
        pub use md5_asm::compress;
    } else if #[cfg(all(feature = "loongarch64_asm", target_arch = "loongarch64"))] {
        mod loongarch64_asm;
        pub use loongarch64_asm::compress;
    } else {
        mod soft;
        pub use soft::compress;
    }
}
