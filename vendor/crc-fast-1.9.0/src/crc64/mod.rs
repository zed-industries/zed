// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides CRC-64 support.

pub mod algorithm;
pub mod consts;
pub mod utils;

#[cfg(test)]
mod property_tests {
    use crate::test::miri_compatible_proptest_config;
    use crate::{checksum, CrcAlgorithm};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(miri_compatible_proptest_config())]

        /// Feature: crc16-hardware-acceleration, Property 4: CRC-32 and CRC-64 backwards compatibility
        /// *For any* existing CRC-32 or CRC-64 configuration and any input byte sequence, the
        /// computed checksum SHALL match the result from the `crc` crate reference implementation,
        /// ensuring no regression from CRC-16 changes.
        /// **Validates: Requirements 3.1, 3.2, 3.4**
        #[test]
        fn prop_crc64_compatibility(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            use crate::test::consts::{RUST_CRC64_NVME, RUST_CRC64_XZ, RUST_CRC64_ECMA_182, RUST_CRC64_GO_ISO};

            // Test CRC-64/NVME (reflected)
            let our_nvme = checksum(CrcAlgorithm::Crc64Nvme, &data);
            let ref_nvme = RUST_CRC64_NVME.checksum(&data);
            prop_assert_eq!(
                our_nvme, ref_nvme,
                "CRC-64/NVME mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_nvme, ref_nvme
            );

            // Test CRC-64/XZ (reflected)
            let our_xz = checksum(CrcAlgorithm::Crc64Xz, &data);
            let ref_xz = RUST_CRC64_XZ.checksum(&data);
            prop_assert_eq!(
                our_xz, ref_xz,
                "CRC-64/XZ mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_xz, ref_xz
            );

            // Test CRC-64/ECMA-182 (forward/non-reflected)
            let our_ecma = checksum(CrcAlgorithm::Crc64Ecma182, &data);
            let ref_ecma = RUST_CRC64_ECMA_182.checksum(&data);
            prop_assert_eq!(
                our_ecma, ref_ecma,
                "CRC-64/ECMA-182 mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_ecma, ref_ecma
            );

            // Test CRC-64/GO-ISO (reflected)
            let our_go_iso = checksum(CrcAlgorithm::Crc64GoIso, &data);
            let ref_go_iso = RUST_CRC64_GO_ISO.checksum(&data);
            prop_assert_eq!(
                our_go_iso, ref_go_iso,
                "CRC-64/GO-ISO mismatch for {} bytes: our=0x{:016X}, ref=0x{:016X}",
                data.len(), our_go_iso, ref_go_iso
            );
        }
    }
}
