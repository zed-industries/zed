/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Checksum support for HTTP requests and responses.

use aws_smithy_types::base64;

use crate::Crc64Nvme;
use crate::{
    Checksum, Crc32, Crc32c, Md5, Sha1, Sha256, CRC_32_C_NAME, CRC_32_NAME, CRC_64_NVME_NAME,
    SHA_1_NAME, SHA_256_NAME,
};

pub const CRC_32_HEADER_NAME: &str = "x-amz-checksum-crc32";
pub const CRC_32_C_HEADER_NAME: &str = "x-amz-checksum-crc32c";
pub const SHA_1_HEADER_NAME: &str = "x-amz-checksum-sha1";
pub const SHA_256_HEADER_NAME: &str = "x-amz-checksum-sha256";
pub const CRC_64_NVME_HEADER_NAME: &str = "x-amz-checksum-crc64nvme";

// Preserved for compatibility purposes. This should never be used by users, only within smithy-rs
#[warn(dead_code)]
pub(crate) static MD5_HEADER_NAME: &str = "content-md5";

/// When a response has to be checksum-verified, we have to check possible headers until we find the
/// header with the precalculated checksum. Because a service may send back multiple headers, we have
/// to check them in order based on how fast each checksum is to calculate.
pub const CHECKSUM_ALGORITHMS_IN_PRIORITY_ORDER: [&str; 5] = [
    CRC_64_NVME_NAME,
    CRC_32_C_NAME,
    CRC_32_NAME,
    SHA_1_NAME,
    SHA_256_NAME,
];

/// Checksum algorithms are use to validate the integrity of data. Structs that implement this trait
/// can be used as checksum calculators. This trait requires Send + Sync because these checksums are
/// often used in a threaded context.
pub trait HttpChecksum: Checksum + Send + Sync {
    /// Either return this checksum as a http-02x `HeaderMap` containing one HTTP header, or return an error
    /// describing why checksum calculation failed.
    fn headers(self: Box<Self>) -> http_1x::HeaderMap<http_1x::HeaderValue> {
        let mut header_map = http_1x::HeaderMap::new();
        header_map.insert(self.header_name(), self.header_value());

        header_map
    }

    /// Return the `HeaderName` used to represent this checksum algorithm
    fn header_name(&self) -> &'static str;

    /// Return the calculated checksum as a base64-encoded http-02x `HeaderValue`
    fn header_value(self: Box<Self>) -> http_1x::HeaderValue {
        let hash = self.finalize();
        http_1x::HeaderValue::from_str(&base64::encode(&hash[..]))
            .expect("base64 encoded bytes are always valid header values")
    }

    /// Return the total size of
    /// - The `HeaderName`
    /// - The header name/value separator
    /// - The base64-encoded `HeaderValue`
    fn size(&self) -> u64 {
        let trailer_name_size_in_bytes = self.header_name().len();
        let base64_encoded_checksum_size_in_bytes =
            base64::encoded_length(Checksum::size(self) as usize);

        let size = trailer_name_size_in_bytes
            // HTTP trailer names and values may be separated by either a single colon or a single
            // colon and a whitespace. In the AWS Rust SDK, we use a single colon.
            + ":".len()
            + base64_encoded_checksum_size_in_bytes;

        size as u64
    }
}

impl HttpChecksum for Crc32 {
    fn header_name(&self) -> &'static str {
        CRC_32_HEADER_NAME
    }
}

impl HttpChecksum for Crc32c {
    fn header_name(&self) -> &'static str {
        CRC_32_C_HEADER_NAME
    }
}

impl HttpChecksum for Crc64Nvme {
    fn header_name(&self) -> &'static str {
        CRC_64_NVME_HEADER_NAME
    }
}

impl HttpChecksum for Sha1 {
    fn header_name(&self) -> &'static str {
        SHA_1_HEADER_NAME
    }
}

impl HttpChecksum for Sha256 {
    fn header_name(&self) -> &'static str {
        SHA_256_HEADER_NAME
    }
}

impl HttpChecksum for Md5 {
    fn header_name(&self) -> &'static str {
        MD5_HEADER_NAME
    }
}

#[cfg(test)]
mod tests {
    use aws_smithy_types::base64;
    use bytes::Bytes;

    use crate::{
        ChecksumAlgorithm, CRC_32_C_NAME, CRC_32_NAME, CRC_64_NVME_NAME, SHA_1_NAME, SHA_256_NAME,
    };

    use super::HttpChecksum;

    #[test]
    fn test_trailer_length_of_crc32_checksum_body() {
        let checksum = CRC_32_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        let expected_size = 29;
        let actual_size = HttpChecksum::size(&*checksum);
        assert_eq!(expected_size, actual_size)
    }

    #[test]
    fn test_trailer_value_of_crc32_checksum_body() {
        let checksum = CRC_32_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        // The CRC32 of an empty string is all zeroes
        let expected_value = Bytes::from_static(b"\0\0\0\0");
        let expected_value = base64::encode(&expected_value);
        let actual_value = checksum.header_value();
        assert_eq!(expected_value, actual_value)
    }

    #[test]
    fn test_trailer_length_of_crc32c_checksum_body() {
        let checksum = CRC_32_C_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        let expected_size = 30;
        let actual_size = HttpChecksum::size(&*checksum);
        assert_eq!(expected_size, actual_size)
    }

    #[test]
    fn test_trailer_value_of_crc32c_checksum_body() {
        let checksum = CRC_32_C_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        // The CRC32C of an empty string is all zeroes
        let expected_value = Bytes::from_static(b"\0\0\0\0");
        let expected_value = base64::encode(&expected_value);
        let actual_value = checksum.header_value();
        assert_eq!(expected_value, actual_value)
    }

    #[test]
    fn test_trailer_length_of_crc64nvme_checksum_body() {
        let checksum = CRC_64_NVME_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        let expected_size = 37;
        let actual_size = HttpChecksum::size(&*checksum);
        assert_eq!(expected_size, actual_size)
    }

    #[test]
    fn test_trailer_value_of_crc64nvme_checksum_body() {
        let checksum = CRC_64_NVME_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        // The CRC64NVME of an empty string is all zeroes
        let expected_value = Bytes::from_static(b"\0\0\0\0\0\0\0\0");
        let expected_value = base64::encode(&expected_value);
        let actual_value = checksum.header_value();
        assert_eq!(expected_value, actual_value)
    }

    #[test]
    fn test_trailer_length_of_sha1_checksum_body() {
        let checksum = SHA_1_NAME.parse::<ChecksumAlgorithm>().unwrap().into_impl();
        let expected_size = 48;
        let actual_size = HttpChecksum::size(&*checksum);
        assert_eq!(expected_size, actual_size)
    }

    #[test]
    fn test_trailer_value_of_sha1_checksum_body() {
        let checksum = SHA_1_NAME.parse::<ChecksumAlgorithm>().unwrap().into_impl();
        // The SHA1 of an empty string is da39a3ee5e6b4b0d3255bfef95601890afd80709
        let expected_value = Bytes::from_static(&[
            0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d, 0x32, 0x55, 0xbf, 0xef, 0x95, 0x60,
            0x18, 0x90, 0xaf, 0xd8, 0x07, 0x09,
        ]);
        let expected_value = base64::encode(&expected_value);
        let actual_value = checksum.header_value();
        assert_eq!(expected_value, actual_value)
    }

    #[test]
    fn test_trailer_length_of_sha256_checksum_body() {
        let checksum = SHA_256_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        let expected_size = 66;
        let actual_size = HttpChecksum::size(&*checksum);
        assert_eq!(expected_size, actual_size)
    }

    #[test]
    fn test_trailer_value_of_sha256_checksum_body() {
        let checksum = SHA_256_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        // The SHA256 of an empty string is e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let expected_value = Bytes::from_static(&[
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ]);
        let expected_value = base64::encode(&expected_value);
        let actual_value = checksum.header_value();
        assert_eq!(expected_value, actual_value)
    }
}
