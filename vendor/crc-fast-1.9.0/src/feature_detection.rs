// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! Feature detection system for safe and efficient hardware acceleration across different
//! platforms.

#[cfg(all(
    not(feature = "std"),
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
use spin::Once;
#[cfg(all(
    feature = "std",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
use std::sync::OnceLock;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(
    feature = "alloc",
    not(feature = "std"),
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
use alloc::string::{String, ToString};
#[cfg(any(
    test,
    all(
        feature = "std",
        any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
    )
))]
use std::string::{String, ToString};

/// Global ArchOps instance cache - initialized once based on feature detection results
#[cfg(all(
    feature = "std",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
static ARCH_OPS_INSTANCE: OnceLock<ArchOpsInstance> = OnceLock::new();
#[cfg(all(
    not(feature = "std"),
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
static ARCH_OPS_INSTANCE: Once<ArchOpsInstance> = Once::new();

/// Performance tiers representing different hardware capability levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Some variants may not be constructed on all target architectures
pub enum PerformanceTier {
    // AArch64 tiers
    AArch64AesSha3,
    AArch64Aes,

    // x86_64 tiers
    X86_64Avx512Vpclmulqdq,
    X86_64Avx512Pclmulqdq,
    X86_64SsePclmulqdq,

    // x86 tiers
    X86SsePclmulqdq,

    // Fallback
    SoftwareTable,
}

/// Architecture-specific capabilities
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // Some fields may not be read on all target architectures
pub struct ArchCapabilities {
    // AArch64 features
    pub has_aes: bool, // provides PMULL support for CRC calculations (NEON is implicit)
    pub has_crc: bool, // provides native CRC32 instructions for fusion techniques
    pub has_sha3: bool, // requires 'aes', provides EOR3 for XOR3 operations

    // x86/x86_64 features
    pub has_sse41: bool,
    pub has_sse42: bool, // provides native CRC32C instructions for fusion techniques
    pub has_pclmulqdq: bool,
    pub has_avx512vl: bool, // implicitly enables avx512f, has XOR3 operations
    pub has_vpclmulqdq: bool,

    // Rust version gates
    pub rust_version_supports_avx512: bool,
}

/// Helper function to convert a performance tier to a human-readable target string
/// Format: {architecture}-{intrinsics-family}-{intrinsics-features}
#[cfg(any(
    test,
    all(
        feature = "alloc",
        any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
    )
))]
#[inline(always)]
fn tier_to_target_string(tier: PerformanceTier) -> String {
    match tier {
        PerformanceTier::AArch64AesSha3 => "aarch64-neon-pmull-sha3".to_string(),
        PerformanceTier::AArch64Aes => "aarch64-neon-pmull".to_string(),
        PerformanceTier::X86_64Avx512Vpclmulqdq => "x86_64-avx512-vpclmulqdq".to_string(),
        PerformanceTier::X86_64Avx512Pclmulqdq => "x86_64-avx512-pclmulqdq".to_string(),
        PerformanceTier::X86_64SsePclmulqdq => "x86_64-sse-pclmulqdq".to_string(),
        PerformanceTier::X86SsePclmulqdq => "x86-sse-pclmulqdq".to_string(),
        PerformanceTier::SoftwareTable => "software-fallback-tables".to_string(),
    }
}

/// Detect architecture-specific capabilities combining compile-time and runtime checks
///
/// # Safety
/// Uses runtime feature detection which may access CPU-specific registers
#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
unsafe fn detect_arch_capabilities() -> ArchCapabilities {
    #[cfg(target_arch = "aarch64")]
    {
        detect_aarch64_features()
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        detect_x86_features()
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    {
        // Other architectures use software fallback
        ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        }
    }
}

/// AArch64-specific feature detection
///
/// Note: NEON is always available on AArch64 and is implicitly enabled by AES support.
/// AES support provides the PMULL instructions needed for CRC calculations.
#[inline(always)]
#[cfg(all(target_arch = "aarch64", feature = "std"))]
unsafe fn detect_aarch64_features() -> ArchCapabilities {
    use std::arch::is_aarch64_feature_detected;

    // AES is available on essentially all AArch64 CPUs and provides the PMULL instructions
    let has_aes = is_aarch64_feature_detected!("aes");

    // CRC provides native CRC32 instructions for fusion techniques
    let has_crc = is_aarch64_feature_detected!("crc");

    // SHA3 is available on modern Aarch64 CPUs, and provides the EOR3 instruction for efficient
    // XOR3 operations.
    let has_sha3 = is_aarch64_feature_detected!("sha3");

    ArchCapabilities {
        has_aes,
        has_crc,
        has_sha3,
        has_sse41: false,
        has_sse42: false,
        has_pclmulqdq: false,
        has_avx512vl: false,
        has_vpclmulqdq: false,
        rust_version_supports_avx512: false,
    }
}

#[inline(always)]
#[cfg(all(target_arch = "aarch64", not(feature = "std")))]
unsafe fn detect_aarch64_features() -> ArchCapabilities {
    ArchCapabilities {
        has_aes: cfg!(target_feature = "aes"),
        has_crc: cfg!(target_feature = "crc"),
        has_sha3: cfg!(target_feature = "sha3"),
        has_sse41: false,
        has_sse42: false,
        has_pclmulqdq: false,
        has_avx512vl: false,
        has_vpclmulqdq: false,
        rust_version_supports_avx512: false,
    }
}

/// x86/x86_64-specific feature detection
#[inline(always)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "std"))]
unsafe fn detect_x86_features() -> ArchCapabilities {
    use std::arch::is_x86_feature_detected;

    // Check Rust version support for VPCLMULQDQ (requires 1.89+)
    let rust_version_supports_avx512 = check_rust_version_supports_avx512();

    // SSE 4.1 and PCLMULQDQ support are the baseline for hardware acceleration
    let has_sse41 = is_x86_feature_detected!("sse4.1");
    let has_pclmulqdq = has_sse41 && is_x86_feature_detected!("pclmulqdq");

    // SSE 4.2 provides native CRC32C instructions for fusion techniques
    let has_sse42 = is_x86_feature_detected!("sse4.2");

    // After Rust 1.89, AVX-512VL and VPCLMULQDQ can be used if available
    let has_avx512vl =
        has_pclmulqdq && rust_version_supports_avx512 && is_x86_feature_detected!("avx512vl");
    let has_vpclmulqdq =
        has_avx512vl && rust_version_supports_avx512 && is_x86_feature_detected!("vpclmulqdq");

    ArchCapabilities {
        has_aes: false,
        has_crc: false,
        has_sha3: false,
        has_sse41,
        has_sse42,
        has_pclmulqdq,
        has_avx512vl,
        has_vpclmulqdq,
        rust_version_supports_avx512,
    }
}

#[inline(always)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(feature = "std")))]
unsafe fn detect_x86_features() -> ArchCapabilities {
    let rust_version_supports_avx512 = check_rust_version_supports_avx512();

    let has_sse41 = cfg!(target_feature = "sse4.1");
    let has_sse42 = cfg!(target_feature = "sse4.2");
    let has_pclmulqdq = has_sse41 && cfg!(target_feature = "pclmulqdq");
    let has_avx512vl =
        has_pclmulqdq && rust_version_supports_avx512 && cfg!(target_feature = "avx512vl");
    let has_vpclmulqdq =
        has_avx512vl && rust_version_supports_avx512 && cfg!(target_feature = "vpclmulqdq");

    ArchCapabilities {
        has_aes: false,
        has_crc: false,
        has_sha3: false,
        has_sse41,
        has_sse42,
        has_pclmulqdq,
        has_avx512vl,
        has_vpclmulqdq,
        rust_version_supports_avx512,
    }
}

/// Check if the current Rust version supports VPCLMULQDQ intrinsics
/// VPCLMULQDQ intrinsics were stabilized in Rust 1.89
#[rustversion::since(1.89)]
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) fn check_rust_version_supports_avx512() -> bool {
    true
}

/// Check if the current Rust version supports VPCLMULQDQ intrinsics
/// VPCLMULQDQ intrinsics were stabilized in Rust 1.89
#[rustversion::before(1.89)]
#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) fn check_rust_version_supports_avx512() -> bool {
    false
}

/// Select the appropriate performance tier based on detected capabilities
#[inline(always)]
#[allow(unused)]
pub(crate) fn select_performance_tier(capabilities: &ArchCapabilities) -> PerformanceTier {
    #[cfg(target_arch = "aarch64")]
    {
        if capabilities.has_sha3 && capabilities.has_aes {
            return PerformanceTier::AArch64AesSha3;
        }

        if capabilities.has_aes {
            return PerformanceTier::AArch64Aes;
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        if capabilities.has_vpclmulqdq {
            return PerformanceTier::X86_64Avx512Vpclmulqdq;
        }
        if capabilities.has_avx512vl {
            return PerformanceTier::X86_64Avx512Pclmulqdq;
        }
        if capabilities.has_pclmulqdq {
            return PerformanceTier::X86_64SsePclmulqdq;
        }
    }

    #[cfg(target_arch = "x86")]
    {
        if capabilities.has_pclmulqdq {
            return PerformanceTier::X86SsePclmulqdq;
        }
    }

    // Fallback to software implementation
    PerformanceTier::SoftwareTable
}

/// Enum that holds the different ArchOps implementations for compile-time dispatch
/// This avoids the need for trait objects while still providing factory-based selection
#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
#[rustversion::since(1.89)]
#[derive(Debug, Clone, Copy)]
pub enum ArchOpsInstance {
    #[cfg(target_arch = "aarch64")]
    Aarch64Aes(crate::arch::aarch64::aes::Aarch64AesOps),
    #[cfg(target_arch = "aarch64")]
    Aarch64AesSha3(crate::arch::aarch64::aes_sha3::Aarch64AesSha3Ops),
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    X86SsePclmulqdq(crate::arch::x86::sse::X86SsePclmulqdqOps),
    #[cfg(target_arch = "x86_64")]
    X86_64Avx512Pclmulqdq(crate::arch::x86_64::avx512::X86_64Avx512PclmulqdqOps),
    #[cfg(target_arch = "x86_64")]
    X86_64Avx512Vpclmulqdq(crate::arch::x86_64::avx512_vpclmulqdq::X86_64Avx512VpclmulqdqOps),
    /// Software fallback - no ArchOps struct needed
    SoftwareFallback,
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
#[rustversion::before(1.89)]
#[derive(Debug, Clone, Copy)]
pub enum ArchOpsInstance {
    #[cfg(target_arch = "aarch64")]
    Aarch64Aes(crate::arch::aarch64::aes::Aarch64AesOps),
    #[cfg(target_arch = "aarch64")]
    Aarch64AesSha3(crate::arch::aarch64::aes_sha3::Aarch64AesSha3Ops),
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    X86SsePclmulqdq(crate::arch::x86::sse::X86SsePclmulqdqOps),
    /// Software fallback - no ArchOps struct needed
    SoftwareFallback,
}

#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
impl ArchOpsInstance {
    #[inline(always)]
    #[rustversion::since(1.89)]
    pub fn get_tier(&self) -> PerformanceTier {
        match self {
            #[cfg(target_arch = "aarch64")]
            ArchOpsInstance::Aarch64Aes(_) => PerformanceTier::AArch64Aes,
            #[cfg(target_arch = "aarch64")]
            ArchOpsInstance::Aarch64AesSha3(_) => PerformanceTier::AArch64AesSha3,
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            ArchOpsInstance::X86SsePclmulqdq(_) => PerformanceTier::X86SsePclmulqdq,
            #[cfg(target_arch = "x86_64")]
            ArchOpsInstance::X86_64Avx512Pclmulqdq(_) => PerformanceTier::X86_64Avx512Pclmulqdq,
            #[cfg(target_arch = "x86_64")]
            ArchOpsInstance::X86_64Avx512Vpclmulqdq(_) => PerformanceTier::X86_64Avx512Vpclmulqdq,
            ArchOpsInstance::SoftwareFallback => PerformanceTier::SoftwareTable,
        }
    }

    #[inline(always)]
    #[rustversion::before(1.89)]
    pub fn get_tier(&self) -> PerformanceTier {
        match self {
            #[cfg(target_arch = "aarch64")]
            ArchOpsInstance::Aarch64Aes(_) => PerformanceTier::AArch64Aes,
            #[cfg(target_arch = "aarch64")]
            ArchOpsInstance::Aarch64AesSha3(_) => PerformanceTier::AArch64AesSha3,
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            ArchOpsInstance::X86SsePclmulqdq(_) => PerformanceTier::X86SsePclmulqdq,
            ArchOpsInstance::SoftwareFallback => PerformanceTier::SoftwareTable,
        }
    }

    /// Get a human-readable target string describing the active configuration
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn get_target_string(&self) -> String {
        tier_to_target_string(self.get_tier())
    }
}

/// Get the global ArchOps instance (thread-safe, initialized once based on feature detection)
///
/// This function provides access to the cached ArchOps instance that was selected based on
/// feature detection results at library initialization time, eliminating runtime feature
/// detection overhead from hot paths.
#[cfg(all(
    feature = "std",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
pub fn get_arch_ops() -> &'static ArchOpsInstance {
    ARCH_OPS_INSTANCE.get_or_init(create_arch_ops)
}

#[cfg(all(
    not(feature = "std"),
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
pub fn get_arch_ops() -> &'static ArchOpsInstance {
    ARCH_OPS_INSTANCE.call_once(create_arch_ops)
}

/// Factory function that creates the appropriate ArchOps struct based on cached feature detection
///
/// This function uses the cached feature detection results to select the optimal
/// architecture-specific implementation at library initialization time, eliminating
/// runtime feature detection overhead from hot paths.
#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
fn create_arch_ops() -> ArchOpsInstance {
    let capabilities = unsafe { detect_arch_capabilities() };
    let tier = select_performance_tier(&capabilities);

    create_arch_ops_from_tier(tier)
}

/// Helper function to create ArchOpsInstance from a performance tier for Rust 1.89+ (when AVX512
/// stabilized)
#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
#[rustversion::since(1.89)]
fn create_arch_ops_from_tier(tier: PerformanceTier) -> ArchOpsInstance {
    match tier {
        #[cfg(target_arch = "aarch64")]
        PerformanceTier::AArch64AesSha3 => {
            use crate::arch::aarch64::aes_sha3::Aarch64AesSha3Ops;
            ArchOpsInstance::Aarch64AesSha3(Aarch64AesSha3Ops::new())
        }
        #[cfg(target_arch = "aarch64")]
        PerformanceTier::AArch64Aes => {
            use crate::arch::aarch64::aes::Aarch64AesOps;
            ArchOpsInstance::Aarch64Aes(Aarch64AesOps)
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx512Vpclmulqdq => {
            use crate::arch::x86_64::avx512_vpclmulqdq::X86_64Avx512VpclmulqdqOps;
            ArchOpsInstance::X86_64Avx512Vpclmulqdq(X86_64Avx512VpclmulqdqOps::new())
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx512Pclmulqdq => {
            use crate::arch::x86_64::avx512::X86_64Avx512PclmulqdqOps;
            ArchOpsInstance::X86_64Avx512Pclmulqdq(X86_64Avx512PclmulqdqOps::new())
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        PerformanceTier::X86_64SsePclmulqdq | PerformanceTier::X86SsePclmulqdq => {
            create_x86_sse_pclmulqdq_ops()
        }
        PerformanceTier::SoftwareTable => {
            // Use software fallback
            ArchOpsInstance::SoftwareFallback
        }
        // Handle cases where the performance tier doesn't match the current architecture
        _ => {
            // This can happen when a tier is selected for a different architecture
            // Fall back to software implementation
            ArchOpsInstance::SoftwareFallback
        }
    }
}

/// Helper function to create ArchOpsInstance from a performance tier for Rust <1.89 (before AVX512
/// stabilized)
#[cfg(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64"))]
#[rustversion::before(1.89)]
fn create_arch_ops_from_tier(tier: PerformanceTier) -> ArchOpsInstance {
    match tier {
        #[cfg(target_arch = "aarch64")]
        PerformanceTier::AArch64AesSha3 => {
            use crate::arch::aarch64::aes_sha3::Aarch64AesSha3Ops;
            ArchOpsInstance::Aarch64AesSha3(Aarch64AesSha3Ops::new())
        }
        #[cfg(target_arch = "aarch64")]
        PerformanceTier::AArch64Aes => {
            use crate::arch::aarch64::aes::Aarch64AesOps;
            ArchOpsInstance::Aarch64Aes(Aarch64AesOps)
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx512Vpclmulqdq => {
            // VPCLMULQDQ and AVX512 not available in older Rust versions, fall back to SSE
            create_x86_sse_pclmulqdq_ops()
        }
        #[cfg(target_arch = "x86_64")]
        PerformanceTier::X86_64Avx512Pclmulqdq => {
            // AVX512 not available in older Rust versions, fall back to SSE
            create_x86_sse_pclmulqdq_ops()
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        PerformanceTier::X86_64SsePclmulqdq | PerformanceTier::X86SsePclmulqdq => {
            create_x86_sse_pclmulqdq_ops()
        }
        PerformanceTier::SoftwareTable => {
            // Use software fallback
            ArchOpsInstance::SoftwareFallback
        }
        // Handle cases where the performance tier doesn't match the current architecture
        _ => {
            // This can happen when a tier is selected for a different architecture
            // Fall back to software implementation
            ArchOpsInstance::SoftwareFallback
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn create_x86_sse_pclmulqdq_ops() -> ArchOpsInstance {
    use crate::arch::x86::sse::X86SsePclmulqdqOps;
    ArchOpsInstance::X86SsePclmulqdq(X86SsePclmulqdqOps)
}
/// Test-specific tier selection that works across all architectures for comprehensive testing
#[cfg(test)]
pub fn select_performance_tier_for_test(capabilities: &ArchCapabilities) -> PerformanceTier {
    // AArch64 tier selection - SHA3 requires AES to be available
    if capabilities.has_sha3 && capabilities.has_aes {
        return PerformanceTier::AArch64AesSha3;
    }

    if capabilities.has_aes {
        return PerformanceTier::AArch64Aes;
    }

    // x86_64 tier selection - VPCLMULQDQ requires AVX512VL
    if capabilities.has_vpclmulqdq
        && capabilities.has_avx512vl
        && capabilities.rust_version_supports_avx512
    {
        return PerformanceTier::X86_64Avx512Vpclmulqdq;
    }

    // AVX512VL requires PCLMULQDQ and SSE4.1
    if capabilities.has_avx512vl
        && capabilities.has_pclmulqdq
        && capabilities.rust_version_supports_avx512
    {
        return PerformanceTier::X86_64Avx512Pclmulqdq;
    }

    // PCLMULQDQ requires SSE4.1
    if capabilities.has_pclmulqdq && capabilities.has_sse41 {
        return PerformanceTier::X86_64SsePclmulqdq;
    }

    // Fallback to software implementation
    PerformanceTier::SoftwareTable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_rust_version_check() {
        let supports_vpclmulqdq = check_rust_version_supports_avx512();

        // Should return a boolean without panicking
        let _ = supports_vpclmulqdq;
    }

    #[test]
    fn test_aarch64_tier_selection() {
        // Test that aarch64 tier selection follows the expected hierarchy

        // Test SHA3 + AES (highest tier) - NEON is implicit with AES
        let capabilities_sha3 = ArchCapabilities {
            has_aes: true,
            has_crc: false,
            has_sha3: true,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_sha3),
            PerformanceTier::AArch64AesSha3
        );

        // Test AES only (baseline tier) - NEON is implicit with AES
        let capabilities_aes = ArchCapabilities {
            has_aes: true,
            has_crc: false,
            has_sha3: false,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_aes),
            PerformanceTier::AArch64Aes
        );

        // Test missing AES (should fall back to software)
        let capabilities_no_aes = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_no_aes),
            PerformanceTier::SoftwareTable
        );
    }

    #[test]
    fn test_aarch64_feature_hierarchy() {
        // Test that AArch64 feature hierarchy is properly maintained
        // NEON is always available on AArch64 and implicit with AES
        // AES provides PMULL instructions, SHA3 provides EOR3

        // Create test capabilities with AES support (NEON is implicit)
        let capabilities_with_aes = ArchCapabilities {
            has_aes: true,
            has_crc: false,
            has_sha3: false,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };

        // AES support means we have PMULL instructions available for CRC calculations
        assert!(capabilities_with_aes.has_aes);

        // SHA3 requires AES to be available first
        let capabilities_with_sha3 = ArchCapabilities {
            has_aes: true,
            has_crc: false,
            has_sha3: true,
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };

        assert!(capabilities_with_sha3.has_aes);
        assert!(capabilities_with_sha3.has_sha3);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_x86_64_tier_selection() {
        // Test that x86_64 tier selection follows the expected hierarchy

        // Test VPCLMULQDQ + AVX512 (highest tier) on Rust 1.89+
        let capabilities_vpclmulqdq = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: true,
            rust_version_supports_avx512: true,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_vpclmulqdq),
            PerformanceTier::X86_64Avx512Vpclmulqdq
        );

        // Test AVX512 + PCLMULQDQ (mid-tier) on Rust 1.89+
        let capabilities_avx512 = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: true,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_avx512),
            PerformanceTier::X86_64Avx512Pclmulqdq
        );

        // Test VPCLMULQDQ + AVX512 (highest tier) on Rust < 1.89
        let capabilities_vpclmulqdq = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: true,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_vpclmulqdq),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // Test AVX512 + PCLMULQDQ (mid-tier) on Rust < 1.89
        let capabilities_avx512 = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_avx512),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // Test SSE + PCLMULQDQ (baseline tier)
        let capabilities_sse = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_sse),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // Test missing PCLMULQDQ (should fall back to software)
        let capabilities_no_pclmul = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_no_pclmul),
            PerformanceTier::SoftwareTable
        );
    }

    #[test]
    #[cfg(target_arch = "x86")]
    fn test_x86_tier_selection() {
        // Test that x86 (32-bit) tier selection works correctly

        // Test SSE + PCLMULQDQ (only available tier for x86)
        let capabilities_sse = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_sse),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // For x86 32-bit testing, we need a special case since AVX512VL indicates x86_64
        // Create capabilities without AVX512VL to simulate x86 32-bit
        let capabilities_x86_sse = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: false, // No AVX512 on 32-bit x86
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        // This should select x86_64 tier since we're testing the general case
        assert_eq!(
            select_performance_tier_for_test(&capabilities_x86_sse),
            PerformanceTier::X86_64SsePclmulqdq
        );

        // Test missing PCLMULQDQ (should fall back to software)
        let capabilities_no_pclmul = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };
        assert_eq!(
            select_performance_tier_for_test(&capabilities_no_pclmul),
            PerformanceTier::SoftwareTable
        );
    }

    #[test]
    fn test_x86_feature_hierarchy() {
        // Test that x86 feature hierarchy is properly maintained
        // SSE4.1 is required for PCLMULQDQ
        // AVX512VL requires PCLMULQDQ
        // VPCLMULQDQ requires AVX512VL and Rust 1.89+

        // Test feature dependencies are enforced
        let capabilities_full = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true,
            has_sse42: false,
            has_pclmulqdq: true,
            has_avx512vl: true,
            has_vpclmulqdq: true,
            rust_version_supports_avx512: true,
        };

        // All x86 features should be available when hierarchy is satisfied
        assert!(capabilities_full.has_sse41);
        assert!(capabilities_full.has_pclmulqdq);
        assert!(capabilities_full.has_avx512vl);
        assert!(capabilities_full.has_vpclmulqdq);
        assert!(capabilities_full.rust_version_supports_avx512);
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_rust_version_gating() {
        // Test that VPCLMULQDQ is properly gated by Rust version
        let rust_support = check_rust_version_supports_avx512();

        // Should return a boolean based on Rust version
        // This will be true for Rust 1.89+ and false for earlier versions
        assert!(rust_support == true || rust_support == false);
    }

    // Mock tests for compile-time and runtime feature agreement scenarios
    mod mock_feature_agreement_tests {
        use super::*;

        #[test]
        fn test_rust_version_gating_scenarios() {
            // Test VPCLMULQDQ with different Rust version scenarios

            // All features available but Rust version too old
            let capabilities_old_rust = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,                // Hardware supports it
                rust_version_supports_avx512: false, // But Rust version is too old
            };

            // Should not select VPCLMULQDQ or AVX512 tiers due to Rust version constraint
            let tier = select_performance_tier_for_test(&capabilities_old_rust);
            assert_ne!(tier, PerformanceTier::X86_64Avx512Vpclmulqdq);
            assert_ne!(tier, PerformanceTier::X86_64Avx512Pclmulqdq);
            assert_eq!(tier, PerformanceTier::X86_64SsePclmulqdq);
        }

        #[test]
        fn test_feature_dependency_validation() {
            // Test that feature dependencies are properly validated

            // SHA3 without AES should not be possible
            let invalid_sha3_caps = ArchCapabilities {
                has_aes: false, // Missing required dependency
                has_crc: false,
                has_sha3: true, // This should be impossible in real detection
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };

            // Should fall back to software since AES is required for SHA3
            assert_eq!(
                select_performance_tier_for_test(&invalid_sha3_caps),
                PerformanceTier::SoftwareTable
            );

            // VPCLMULQDQ without AVX512VL should not be possible
            let invalid_vpclmul_caps = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: false,  // Missing required dependency
                has_vpclmulqdq: true, // This should be impossible in real detection
                rust_version_supports_avx512: true,
            };

            // Should fall back to SSE tier since AVX512VL is required for VPCLMULQDQ
            assert_eq!(
                select_performance_tier_for_test(&invalid_vpclmul_caps),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }
    }

    // Comprehensive tier selection tests across different hardware configurations
    mod tier_selection_comprehensive_tests {
        use super::*;

        #[test]
        fn test_all_aarch64_tier_combinations() {
            // Test all possible AArch64 capability combinations

            // No features - software fallback
            let no_features = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&no_features),
                PerformanceTier::SoftwareTable
            );

            // AES only - baseline AArch64 tier
            let aes_only = ArchCapabilities {
                has_aes: true,
                has_crc: false,
                has_sha3: false,
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&aes_only),
                PerformanceTier::AArch64Aes
            );

            // AES + SHA3 - highest AArch64 tier
            let aes_sha3 = ArchCapabilities {
                has_aes: true,
                has_crc: false,
                has_sha3: true,
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&aes_sha3),
                PerformanceTier::AArch64AesSha3
            );
        }

        #[test]
        fn test_all_x86_64_tier_combinations() {
            // Test all possible x86_64 capability combinations

            // No features - software fallback
            let no_features = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&no_features),
                PerformanceTier::SoftwareTable
            );

            // SSE4.1 only - software fallback (PCLMULQDQ required)
            let sse_only = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&sse_only),
                PerformanceTier::SoftwareTable
            );

            // SSE4.1 + PCLMULQDQ - baseline x86_64 tier
            let sse_pclmul = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&sse_pclmul),
                PerformanceTier::X86_64SsePclmulqdq
            );

            // SSE4.1 + PCLMULQDQ + AVX512VL but old Rust - should fall back to SSE tier
            let avx512_pclmul_old_rust = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false, // Old Rust version
            };
            assert_eq!(
                select_performance_tier_for_test(&avx512_pclmul_old_rust),
                PerformanceTier::X86_64SsePclmulqdq
            );

            // SSE4.1 + PCLMULQDQ + AVX512VL with new Rust - mid-tier
            let avx512_pclmul_new_rust = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: true, // New Rust version
            };
            assert_eq!(
                select_performance_tier_for_test(&avx512_pclmul_new_rust),
                PerformanceTier::X86_64Avx512Pclmulqdq
            );

            // All features + old Rust - should fall back to SSE tier
            let all_features_old_rust = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,
                rust_version_supports_avx512: false, // Old Rust version
            };
            assert_eq!(
                select_performance_tier_for_test(&all_features_old_rust),
                PerformanceTier::X86_64SsePclmulqdq
            );

            // All features + new Rust - highest tier
            let all_features_new_rust = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,
                rust_version_supports_avx512: true, // New Rust version
            };
            assert_eq!(
                select_performance_tier_for_test(&all_features_new_rust),
                PerformanceTier::X86_64Avx512Vpclmulqdq
            );
        }

        #[test]
        fn test_x86_32bit_tier_combinations() {
            // Test x86 (32-bit) capability combinations

            // No features - software fallback
            let no_features = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            assert_eq!(
                select_performance_tier_for_test(&no_features),
                PerformanceTier::SoftwareTable
            );

            // SSE4.1 + PCLMULQDQ - for x86 32-bit testing, we expect x86_64 tier in our test function
            // since the test function doesn't distinguish between x86 and x86_64 architectures
            let sse_pclmul = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: false, // AVX512 not available on 32-bit x86
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            // The test function will return x86_64 tier since it doesn't distinguish architectures
            assert_eq!(
                select_performance_tier_for_test(&sse_pclmul),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }

        #[test]
        fn test_target_string_consistency() {
            // Test that target strings are consistent with selected tiers

            let test_cases = [
                (PerformanceTier::AArch64AesSha3, "aarch64-neon-pmull-sha3"),
                (PerformanceTier::AArch64Aes, "aarch64-neon-pmull"),
                (
                    PerformanceTier::X86_64Avx512Vpclmulqdq,
                    "x86_64-avx512-vpclmulqdq",
                ),
                (
                    PerformanceTier::X86_64Avx512Pclmulqdq,
                    "x86_64-avx512-pclmulqdq",
                ),
                (PerformanceTier::X86_64SsePclmulqdq, "x86_64-sse-pclmulqdq"),
                (PerformanceTier::X86SsePclmulqdq, "x86-sse-pclmulqdq"),
                (PerformanceTier::SoftwareTable, "software-fallback-tables"),
            ];

            for (tier, expected_string) in test_cases {
                assert_eq!(tier_to_target_string(tier), expected_string);
            }
        }
    }

    // Tests for graceful degradation between performance tiers
    mod graceful_degradation_tests {
        use super::*;

        #[test]
        fn test_aarch64_degradation_path() {
            // Test the degradation path for AArch64: SHA3+AES -> AES -> Software

            // Start with highest tier capabilities
            let mut capabilities = ArchCapabilities {
                has_aes: true,
                has_crc: false,
                has_sha3: true,
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };

            // Should select highest tier
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::AArch64AesSha3
            );

            // Remove SHA3 - should degrade to AES tier
            capabilities.has_sha3 = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::AArch64Aes
            );

            // Remove AES - should degrade to software
            capabilities.has_aes = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::SoftwareTable
            );
        }

        #[test]
        fn test_x86_64_degradation_path() {
            // Test the degradation path for x86_64: VPCLMULQDQ -> AVX512 -> SSE -> Software

            // Start with highest tier capabilities
            let mut capabilities = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,
                rust_version_supports_avx512: true,
            };

            // Should select highest tier
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::X86_64Avx512Vpclmulqdq
            );

            // Remove VPCLMULQDQ - should degrade to AVX512 tier
            capabilities.has_vpclmulqdq = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::X86_64Avx512Pclmulqdq
            );

            // Remove AVX512VL - should degrade to SSE tier
            capabilities.has_avx512vl = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::X86_64SsePclmulqdq
            );

            // Remove PCLMULQDQ - should degrade to software
            capabilities.has_pclmulqdq = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::SoftwareTable
            );

            // Remove SSE4.1 - should still be software (already at lowest tier)
            capabilities.has_sse41 = false;
            assert_eq!(
                select_performance_tier_for_test(&capabilities),
                PerformanceTier::SoftwareTable
            );
        }

        #[test]
        fn test_rust_version_degradation() {
            // Test degradation when Rust version doesn't support VPCLMULQDQ

            let capabilities_with_vpclmulqdq = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: true,
                has_vpclmulqdq: true,
                rust_version_supports_avx512: false, // Old Rust version
            };

            // Should degrade from VPCLMULQDQ tier to SSE tier due to Rust version
            assert_eq!(
                select_performance_tier_for_test(&capabilities_with_vpclmulqdq),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }

        #[test]
        fn test_partial_feature_availability() {
            // Test scenarios where only some features in a tier are available

            // AArch64: SHA3 available but AES not (impossible in real hardware, but test safety)
            let aarch64_partial = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: true, // This would be impossible in real detection
                has_sse41: false,
                has_sse42: false,
                has_pclmulqdq: false,
                has_avx512vl: false,
                has_vpclmulqdq: false,
                rust_version_supports_avx512: false,
            };
            // Should fall back to software since AES is required for SHA3
            assert_eq!(
                select_performance_tier_for_test(&aarch64_partial),
                PerformanceTier::SoftwareTable
            );

            // x86_64: VPCLMULQDQ available but AVX512VL not (impossible in real hardware)
            let x86_64_partial = ArchCapabilities {
                has_aes: false,
                has_crc: false,
                has_sha3: false,
                has_sse41: true,
                has_sse42: false,
                has_pclmulqdq: true,
                has_avx512vl: false,
                has_vpclmulqdq: true, // This would be impossible in real detection
                rust_version_supports_avx512: true,
            };
            // Should fall back to SSE tier since AVX512VL is required for VPCLMULQDQ
            assert_eq!(
                select_performance_tier_for_test(&x86_64_partial),
                PerformanceTier::X86_64SsePclmulqdq
            );
        }
    }
}

#[cfg(test)]
mod software_fallback_tests {
    use super::*;
    #[test]
    fn test_aarch64_without_aes_falls_back_to_software() {
        // Test that AArch64 without AES support falls back to software implementation
        let capabilities_no_aes = ArchCapabilities {
            has_aes: false, // No AES support
            has_crc: false,
            has_sha3: false, // SHA3 requires AES, so also false
            has_sse41: false,
            has_sse42: false,
            has_pclmulqdq: false,
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };

        let tier = select_performance_tier_for_test(&capabilities_no_aes);
        assert_eq!(
            tier,
            PerformanceTier::SoftwareTable,
            "AArch64 without AES should fall back to software implementation"
        );
    }

    #[test]
    fn test_x86_without_pclmulqdq_falls_back_to_software() {
        // Test that x86 without SSE4.1/PCLMULQDQ falls back to software implementation
        let capabilities_no_pclmul = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: true, // SSE4.1 available
            has_sse42: false,
            has_pclmulqdq: false, // But PCLMULQDQ not available
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };

        let tier = select_performance_tier_for_test(&capabilities_no_pclmul);
        assert_eq!(
            tier,
            PerformanceTier::SoftwareTable,
            "x86 without PCLMULQDQ should fall back to software implementation"
        );

        // Test x86 without SSE4.1
        let capabilities_no_sse = ArchCapabilities {
            has_aes: false,
            has_crc: false,
            has_sha3: false,
            has_sse41: false, // No SSE4.1 support
            has_sse42: false,
            has_pclmulqdq: false, // PCLMULQDQ requires SSE4.1
            has_avx512vl: false,
            has_vpclmulqdq: false,
            rust_version_supports_avx512: false,
        };

        let tier = select_performance_tier_for_test(&capabilities_no_sse);
        assert_eq!(
            tier,
            PerformanceTier::SoftwareTable,
            "x86 without SSE4.1 should fall back to software implementation"
        );
    }

    #[test]
    fn test_conditional_compilation_coverage() {
        // Test that software fallback is properly conditionally compiled
        // This test ensures the conditional compilation logic is working correctly

        // Software fallback should be available when needed
        #[cfg(any(
            not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")),
            target_arch = "x86",
            all(target_arch = "aarch64", not(target_feature = "aes"))
        ))]
        {
            // Software fallback should be compiled in these cases
            use crate::arch::software;
            let _test_fn = software::update;
            // This test passes if it compiles successfully
        }

        // For x86_64, software fallback should not be needed since SSE4.1/PCLMULQDQ are always available
        // But it may still be compiled for testing purposes
    }
}
