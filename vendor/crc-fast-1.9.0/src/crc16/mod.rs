// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

pub(crate) mod algorithm;
pub(crate) mod consts;

#[cfg(test)]
mod property_tests {
    use crate::crc16::consts::{CRC16_IBM_SDLC, CRC16_T10_DIF};
    use crate::test::consts::{RUST_CRC16_IBM_SDLC, RUST_CRC16_T10_DIF};
    use crate::test::miri_compatible_proptest_config;
    use crate::{
        checksum, checksum_combine, checksum_combine_with_params, checksum_with_params,
        CrcAlgorithm, CrcParams,
    };
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(miri_compatible_proptest_config())]

        /// Feature: crc16-hardware-acceleration, Property 3: CRC-16 computation matches reference
        /// *For any* valid CRC-16 parameters (polynomial, init, refin, refout, xorout) and any
        /// input byte sequence, the computed CRC-16 checksum SHALL match the result from the
        /// `crc` crate reference implementation.
        /// **Validates: Requirements 2.1, 2.2, 5.5, 6.1, 6.2, 6.3**
        #[test]
        fn prop_crc16_ibm_sdlc_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum(CrcAlgorithm::Crc16IbmSdlc, &data);
            let reference_result = RUST_CRC16_IBM_SDLC.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16/IBM-SDLC mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 3: CRC-16 computation matches reference
        /// *For any* valid CRC-16 parameters (polynomial, init, refin, refout, xorout) and any
        /// input byte sequence, the computed CRC-16 checksum SHALL match the result from the
        /// `crc` crate reference implementation.
        /// **Validates: Requirements 2.1, 2.2, 5.5, 6.1, 6.2, 6.3**
        #[test]
        fn prop_crc16_t10_dif_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum(CrcAlgorithm::Crc16T10Dif, &data);
            let reference_result = RUST_CRC16_T10_DIF.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16/T10-DIF mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 3: CRC-16 computation matches reference
        /// Tests checksum_with_params for CRC-16/IBM-SDLC (reflected variant)
        /// **Validates: Requirements 5.5, 6.1, 6.2, 6.3**
        #[test]
        fn prop_crc16_ibm_sdlc_with_params_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum_with_params(CRC16_IBM_SDLC, &data);
            let reference_result = RUST_CRC16_IBM_SDLC.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16/IBM-SDLC (with_params) mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 3: CRC-16 computation matches reference
        /// Tests checksum_with_params for CRC-16/T10-DIF (forward variant)
        /// **Validates: Requirements 5.5, 6.1, 6.2, 6.3**
        #[test]
        fn prop_crc16_t10_dif_with_params_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            let our_result = checksum_with_params(CRC16_T10_DIF, &data);
            let reference_result = RUST_CRC16_T10_DIF.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16/T10-DIF (with_params) mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 3: CRC-16 computation matches reference
        /// Tests custom CRC-16 parameters (equivalent to IBM-SDLC) to validate CrcParams::new()
        /// **Validates: Requirements 5.5, 6.1, 6.2, 6.3**
        #[test]
        fn prop_crc16_custom_reflected_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Custom CRC-16 parameters equivalent to CRC-16/IBM-SDLC
            let custom_params = CrcParams::new(
                "CRC-16/CUSTOM-REFLECTED",
                16,
                0x1021,
                0xFFFF,
                true,  // reflected
                0xFFFF,
                0x906E,
            );
            let our_result = checksum_with_params(custom_params, &data);
            let reference_result = RUST_CRC16_IBM_SDLC.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16 custom reflected mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 3: CRC-16 computation matches reference
        /// Tests custom CRC-16 parameters (equivalent to T10-DIF) to validate CrcParams::new()
        /// **Validates: Requirements 5.5, 6.1, 6.2, 6.3**
        #[test]
        fn prop_crc16_custom_forward_matches_reference(data in proptest::collection::vec(any::<u8>(), 0..1024)) {
            // Custom CRC-16 parameters equivalent to CRC-16/T10-DIF
            let custom_params = CrcParams::new(
                "CRC-16/CUSTOM-FORWARD",
                16,
                0x8BB7,
                0x0000,
                false,  // forward (non-reflected)
                0x0000,
                0xD0DB,
            );
            let our_result = checksum_with_params(custom_params, &data);
            let reference_result = RUST_CRC16_T10_DIF.checksum(&data) as u64;
            prop_assert_eq!(
                our_result, reference_result,
                "CRC-16 custom forward mismatch for {} bytes: our=0x{:04X}, ref=0x{:04X}",
                data.len(), our_result, reference_result
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 5: CRC-16 checksum combination round-trip
        /// *For any* valid CRC-16 parameters and any two input byte sequences A and B,
        /// `checksum_combine(checksum(A), checksum(B), len(B))` SHALL equal `checksum(A + B)`.
        /// **Validates: Requirements 6.4**
        #[test]
        fn prop_crc16_ibm_sdlc_checksum_combine_roundtrip(
            data_a in proptest::collection::vec(any::<u8>(), 0..512),
            data_b in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let checksum_a = checksum(CrcAlgorithm::Crc16IbmSdlc, &data_a);
            let checksum_b = checksum(CrcAlgorithm::Crc16IbmSdlc, &data_b);
            let combined = checksum_combine(
                CrcAlgorithm::Crc16IbmSdlc,
                checksum_a,
                checksum_b,
                data_b.len() as u64,
            );

            let mut concatenated = data_a.clone();
            concatenated.extend(&data_b);
            let expected = checksum(CrcAlgorithm::Crc16IbmSdlc, &concatenated);

            prop_assert_eq!(
                combined, expected,
                "CRC-16/IBM-SDLC combine mismatch: combined=0x{:04X}, expected=0x{:04X}, len_a={}, len_b={}",
                combined, expected, data_a.len(), data_b.len()
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 5: CRC-16 checksum combination round-trip
        /// *For any* valid CRC-16 parameters and any two input byte sequences A and B,
        /// `checksum_combine(checksum(A), checksum(B), len(B))` SHALL equal `checksum(A + B)`.
        /// **Validates: Requirements 6.4**
        #[test]
        fn prop_crc16_t10_dif_checksum_combine_roundtrip(
            data_a in proptest::collection::vec(any::<u8>(), 0..512),
            data_b in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let checksum_a = checksum(CrcAlgorithm::Crc16T10Dif, &data_a);
            let checksum_b = checksum(CrcAlgorithm::Crc16T10Dif, &data_b);
            let combined = checksum_combine(
                CrcAlgorithm::Crc16T10Dif,
                checksum_a,
                checksum_b,
                data_b.len() as u64,
            );

            let mut concatenated = data_a.clone();
            concatenated.extend(&data_b);
            let expected = checksum(CrcAlgorithm::Crc16T10Dif, &concatenated);

            prop_assert_eq!(
                combined, expected,
                "CRC-16/T10-DIF combine mismatch: combined=0x{:04X}, expected=0x{:04X}, len_a={}, len_b={}",
                combined, expected, data_a.len(), data_b.len()
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 5: CRC-16 checksum combination round-trip
        /// Tests checksum_combine_with_params for CRC-16/IBM-SDLC
        /// **Validates: Requirements 6.4**
        #[test]
        fn prop_crc16_ibm_sdlc_checksum_combine_with_params_roundtrip(
            data_a in proptest::collection::vec(any::<u8>(), 0..512),
            data_b in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let checksum_a = checksum_with_params(CRC16_IBM_SDLC, &data_a);
            let checksum_b = checksum_with_params(CRC16_IBM_SDLC, &data_b);
            let combined = checksum_combine_with_params(
                CRC16_IBM_SDLC,
                checksum_a,
                checksum_b,
                data_b.len() as u64,
            );

            let mut concatenated = data_a.clone();
            concatenated.extend(&data_b);
            let expected = checksum_with_params(CRC16_IBM_SDLC, &concatenated);

            prop_assert_eq!(
                combined, expected,
                "CRC-16/IBM-SDLC combine_with_params mismatch: combined=0x{:04X}, expected=0x{:04X}, len_a={}, len_b={}",
                combined, expected, data_a.len(), data_b.len()
            );
        }

        /// Feature: crc16-hardware-acceleration, Property 5: CRC-16 checksum combination round-trip
        /// Tests checksum_combine_with_params for CRC-16/T10-DIF
        /// **Validates: Requirements 6.4**
        #[test]
        fn prop_crc16_t10_dif_checksum_combine_with_params_roundtrip(
            data_a in proptest::collection::vec(any::<u8>(), 0..512),
            data_b in proptest::collection::vec(any::<u8>(), 0..512)
        ) {
            let checksum_a = checksum_with_params(CRC16_T10_DIF, &data_a);
            let checksum_b = checksum_with_params(CRC16_T10_DIF, &data_b);
            let combined = checksum_combine_with_params(
                CRC16_T10_DIF,
                checksum_a,
                checksum_b,
                data_b.len() as u64,
            );

            let mut concatenated = data_a.clone();
            concatenated.extend(&data_b);
            let expected = checksum_with_params(CRC16_T10_DIF, &concatenated);

            prop_assert_eq!(
                combined, expected,
                "CRC-16/T10-DIF combine_with_params mismatch: combined=0x{:04X}, expected=0x{:04X}, len_a={}, len_b={}",
                combined, expected, data_a.len(), data_b.len()
            );
        }
    }
}
