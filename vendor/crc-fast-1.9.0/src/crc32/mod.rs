// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides CRC-32 support.

pub mod algorithm;
pub mod consts;
pub(crate) mod width32_ops;

#[cfg(all(
    feature = "std",
    any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")
))]
pub(crate) mod fusion;

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
        fn prop_crc32_compatibility(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            use crate::test::consts::{RUST_CRC32_ISCSI, RUST_CRC32_ISO_HDLC, RUST_CRC32_BZIP2, RUST_CRC32_MPEG_2};

            // Test CRC-32/ISCSI (reflected)
            let our_iscsi = checksum(CrcAlgorithm::Crc32Iscsi, &data);
            let ref_iscsi = RUST_CRC32_ISCSI.checksum(&data) as u64;
            prop_assert_eq!(
                our_iscsi, ref_iscsi,
                "CRC-32/ISCSI mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_iscsi, ref_iscsi
            );

            // Test CRC-32/ISO-HDLC (reflected)
            let our_hdlc = checksum(CrcAlgorithm::Crc32IsoHdlc, &data);
            let ref_hdlc = RUST_CRC32_ISO_HDLC.checksum(&data) as u64;
            prop_assert_eq!(
                our_hdlc, ref_hdlc,
                "CRC-32/ISO-HDLC mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_hdlc, ref_hdlc
            );

            // Test CRC-32/BZIP2 (forward/non-reflected)
            let our_bzip2 = checksum(CrcAlgorithm::Crc32Bzip2, &data);
            let ref_bzip2 = RUST_CRC32_BZIP2.checksum(&data) as u64;
            prop_assert_eq!(
                our_bzip2, ref_bzip2,
                "CRC-32/BZIP2 mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_bzip2, ref_bzip2
            );

            // Test CRC-32/MPEG-2 (forward/non-reflected)
            let our_mpeg2 = checksum(CrcAlgorithm::Crc32Mpeg2, &data);
            let ref_mpeg2 = RUST_CRC32_MPEG_2.checksum(&data) as u64;
            prop_assert_eq!(
                our_mpeg2, ref_mpeg2,
                "CRC-32/MPEG-2 mismatch for {} bytes: our=0x{:08X}, ref=0x{:08X}",
                data.len(), our_mpeg2, ref_mpeg2
            );
        }
    }
}
