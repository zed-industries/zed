// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/base.h>

#include <gtest/gtest.h>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/nid.h>

#include "internal.h"

// Below test inputs are created by running 'openssl req -x509',
// and then exacting the bytes of RSASSA-PSS-params from generated results.

// pss_sha1_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha1 (absent)
//    Mask Algorithm: mgf1 with sha1 (absent)
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha1_salt_absent[] = {0x30, 0x00};

// pss_sha224_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha224
//    Mask Algorithm: mgf1 with sha224
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha224_salt_absent[] = {
    0x30, 0x2b, 0xa0, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x04, 0xa1, 0x1a, 0x30, 0x18, 0x06, 0x09, 0x2a,
    0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x08, 0x30, 0x0b, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x04};

// pss_sha256_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha256_salt_absent[] = {
    0x30, 0x2B, 0xA0, 0x0D, 0x30, 0x0B, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x01, 0xA1, 0x1A, 0x30, 0x18, 0x06, 0x09, 0x2A,
    0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x08, 0x30, 0x0B, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01};

// pss_sha384_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha384
//    Mask Algorithm: mgf1 with sha384
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha384_salt_absent[] = {
    0x30, 0x2B, 0xA0, 0x0D, 0x30, 0x0B, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x02, 0xA1, 0x1A, 0x30, 0x18, 0x06, 0x09, 0x2A,
    0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x08, 0x30, 0x0B, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02};

// pss_sha512_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha512
//    Mask Algorithm: mgf1 with sha512
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha512_salt_absent[] = {
    0x30, 0x2B, 0xA0, 0x0D, 0x30, 0x0B, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x03, 0xA1, 0x1A, 0x30, 0x18, 0x06, 0x09, 0x2A,
    0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x08, 0x30, 0x0B, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03};

// pss_sha256_salt_default is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x14 (default)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha256_salt_default[] = {
    0x30, 0x30, 0xa0, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0xa1, 0x1a, 0x30,
    0x18, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01,
    0x01, 0x08, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x01, 0xa2, 0x03, 0x02, 0x01, 0x14};

// pss_sha256_salt_30 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x1e (30)
//    Trailer Field: 0x01 (absent)
static const uint8_t pss_sha256_salt_30[] = {
    0x30, 0x30, 0xa0, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0xa1, 0x1a, 0x30,
    0x18, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01,
    0x01, 0x08, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x01, 0xa2, 0x03, 0x02, 0x01, 0x1e};

// Below test inputs are created by the Java sample code.
// Java uses NULL to encode parameters of Hash func.
// ```JDK11
// Signature signatureSHA256Java = Signature.getInstance("RSASSA-PSS")
// PSSParameterSpec pssParameterSpec = new PSSParameterSpec("SHA-256", "MGF1", MGF1ParameterSpec.SHA256, 0, 1);
// signatureSHA256Java.setParameter(pssParameterSpec);
// byte[] bytes = signatureSHA256Java.getParameters().getEncoded();
// ```

// jdk_pss_sha256_salt_0 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x00 (0)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha256_salt_0[] = {
    0x30, 0x34, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48,
    0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0xA1, 0x1C, 0x30,
    0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01,
    0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
    0x04, 0x02, 0x01, 0x05, 0x00, 0xA2, 0x03, 0x02, 0x01, 0x00};

// jdk_pss_sha256_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha256_salt_absent[] = {
    0x30, 0x2F, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0xA1,
    0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7,
    0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00};

// jdk_pss_sha256_salt_30 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x1e (30)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha256_salt_30[] = {
    0x30, 0x34, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48,
    0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0xA1, 0x1C, 0x30,
    0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01,
    0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03,
    0x04, 0x02, 0x01, 0x05, 0x00, 0xA2, 0x03, 0x02, 0x01, 0x1E};

// jdk_pss_sha224_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha224
//    Mask Algorithm: mgf1 with sha224
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha224_salt_absent[] = {
    0x30, 0x2F, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x04, 0x05, 0x00, 0xA1,
    0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7,
    0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x04, 0x05, 0x00};

// jdk_pss_sha384_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha384
//    Mask Algorithm: mgf1 with sha384
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha384_salt_absent[] = {
    0x30, 0x2F, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05, 0x00, 0xA1,
    0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7,
    0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02, 0x05, 0x00};

// jdk_pss_sha512_salt_absent is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha512
//    Mask Algorithm: mgf1 with sha512
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha512_salt_absent[] = {
    0x30, 0x2F, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00, 0xA1,
    0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7,
    0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00};

// jdk_pss_sha256_mgf1_sha512 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha512
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
// This is to test params decode when different hash funcs are used.
static const uint8_t jdk_pss_sha256_mgf1_sha512[] = {
    0x30, 0x2F, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00, 0xA1,
    0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7,
    0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05, 0x00};

// jdk_pss_sha1_mgf1_sha256 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha1 (absent)
//    Mask Algorithm: mgf1 with sha256
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
// This is to test params decode when different hash funcs are used, and one
// hash func is the default (sha1), which is omitted during encode.
static const uint8_t jdk_pss_sha1_mgf1_sha256[] = {
    0x30, 0x1E, 0xA1, 0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48,
    0x86, 0xF7, 0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60,
    0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00};

// jdk_pss_sha256_mgf1_sha1 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha256
//    Mask Algorithm: mgf1 with sha1 (absent)
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (absent)
// This is to test params decode when different hash funcs are used, and one
// hash func is the default (sha1), which is omitted during encode.
static const uint8_t jdk_pss_sha256_mgf1_sha1[] = {
    0x30, 0x11, 0xA0, 0x0F, 0x30, 0x0D, 0x06, 0x09, 0x60, 0x86,
    0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00};

// jdk_pss_sha1_mgf1_sha1_salt_30 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha1 (absent)
//    Mask Algorithm: mgf1 with sha1 (absent)
//    Minimum Salt Length: 0x1e (30)
//    Trailer Field: 0x01 (absent)
static const uint8_t jdk_pss_sha1_mgf1_sha1_salt_30[] = {
    0x30, 0x05, 0xA2, 0x03, 0x02, 0x01, 0x1E};

// These bytes are manually created for test purpose.
// pss_with_trailer_field_1 is a DER-encoded RSASSA-PSS-params:
//    Hash Algorithm: sha1 (absent)
//    Mask Algorithm: mgf1 with sha1 (absent)
//    Minimum Salt Length: 0x14 (absent)
//    Trailer Field: 0x01 (default, not absent)
static const uint8_t pss_with_trailer_field_1[] = {
    0x30, 0x05, 0xA3, 0x03, 0x02, 0x01, 0x01};

// Invalid test inputs:

// invalid_pss_salt_missing is created by removing salt field from
// pss_sha256_salt_20. This is invalid because "type-length-value" cannot
// reconcile "length" and "value".
static const uint8_t invalid_pss_salt_missing[] = {
    0x30, 0x30, 0xa0, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x01, 0xa1, 0x1a, 0x30, 0x18, 0x06, 0x09, 0x2a,
    0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x08, 0x30, 0x0b, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01};

// pss_with_invalid_sha256_oid is created by changing sha512 oid
// from 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03
// to   0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x05.
static const uint8_t pss_with_invalid_sha256_oid[] = {
    0x30, 0x2b, 0xa0, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x05, 0xa1, 0x1a, 0x30, 0x18, 0x06, 0x09, 0x2a,
    0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x08, 0x30, 0x0b, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x04};

// pss_with_invalid_mgf1_oid is created by changing mgf1 oid
// from 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x08
// to   0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x09.
static const uint8_t pss_with_invalid_mgf1_oid[] = {
    0x30, 0x2b, 0xa0, 0x0d, 0x30, 0x0b, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01,
    0x65, 0x03, 0x04, 0x02, 0x04, 0xa1, 0x1a, 0x30, 0x18, 0x06, 0x09, 0x2a,
    0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x09, 0x30, 0x0b, 0x06, 0x09,
    0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x04};

// pss_with_tag2_before_tag0 has tag [2] before tag [0].
static const uint8_t pss_with_tag2_before_tag0[] = {
    0x30, 0x34, 0xA2, 0x03, 0x02, 0x01, 0x1E, 0xA0, 0x0F, 0x30, 0x0D,
    0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01,
    0x05, 0x00, 0xA1, 0x1C, 0x30, 0x1A, 0x06, 0x09, 0x2A, 0x86, 0x48,
    0x86, 0xF7, 0x0D, 0x01, 0x01, 0x08, 0x30, 0x0D, 0x06, 0x09, 0x60,
    0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05, 0x00
};

// pss_with_tag4 has tag [4].
static const uint8_t pss_with_tag4[] = {
    0x30, 0x05, 0xA4, 0x03, 0x02, 0x01, 0x1E};

// pss_with_double_salt_30 has two tag[2].
static const uint8_t pss_with_double_salt_30[] = {
    0x30, 0x0A, 0xA2, 0x03, 0x02, 0x01, 0x1E, 0xA2, 0x03, 0x02, 0x01, 0x1E};

// pss_with_sequence_length_too_short should use 0x05 as length instead of 0x04.
// This invalid bytes are modified from jdk_pss_sha1_mgf1_sha1_salt_30.
static const uint8_t pss_with_sequence_length_too_short[] = {
    0x30, 0x04, 0xA2, 0x03, 0x02, 0x01, 0x1E};

// pss_with_sequence_length_too_long should use 0x05 as length instead of 0x06.
// This invalid bytes are modified from jdk_pss_sha1_mgf1_sha1_salt_30.
static const uint8_t pss_with_sequence_length_too_long[] = {
    0x30, 0x06, 0xA2, 0x03, 0x02, 0x01, 0x1E};

// pss_with_tag2_length_too_short should use 0x03 as tag2 length instead of 0x02.
// This invalid bytes are modified from jdk_pss_sha1_mgf1_sha1_salt_30.
static const uint8_t pss_with_tag2_length_too_short[] = {
    0x30, 0x05, 0xA2, 0x02, 0x02, 0x01, 0x1E};

// pss_with_tag2_length_too_long should use 0x03 as tag2 length instead of 0x04.
// This invalid bytes are modified from jdk_pss_sha1_mgf1_sha1_salt_30.
static const uint8_t pss_with_tag2_length_too_long[] = {
    0x30, 0x05, 0xA2, 0x04, 0x02, 0x01, 0x1E};

// pss_with_negative_salt_length is modified from jdk_pss_sha1_mgf1_sha1_salt_30.
static const uint8_t pss_with_negative_salt_length[] = {
    0x30, 0x05, 0xA2, 0x03, 0x02, 0x01, 0xFF};

// pss_with_negative_salt_length is modified from jdk_pss_sha1_mgf1_sha1_salt_30.
static const uint8_t pss_with_trailer_field_not_1[] = {
    0x30, 0x05, 0xA3, 0x03, 0x02, 0x01, 0x02};

static const int omit_salt_len = -1;

struct PssParseTestInput {
  const uint8_t *der;
  size_t der_len;
  int expected_hash_nid;
  int expected_mask_gen_nid;
  int expected_mgf1_hash_nid;
  int expected_salt_len;
} kPssParseTestInputs[] = {
    {pss_sha1_salt_absent, sizeof(pss_sha1_salt_absent), NID_undef, NID_undef,
     NID_undef, omit_salt_len},
    {pss_sha224_salt_absent, sizeof(pss_sha224_salt_absent), NID_sha224,
     NID_mgf1, NID_sha224, omit_salt_len},
    {pss_sha256_salt_absent, sizeof(pss_sha256_salt_absent), NID_sha256,
     NID_mgf1, NID_sha256, omit_salt_len},
    {pss_sha384_salt_absent, sizeof(pss_sha384_salt_absent), NID_sha384,
     NID_mgf1, NID_sha384, omit_salt_len},
    {pss_sha512_salt_absent, sizeof(pss_sha512_salt_absent), NID_sha512,
     NID_mgf1, NID_sha512, omit_salt_len},
    {pss_sha256_salt_default, sizeof(pss_sha256_salt_default), NID_sha256,
     NID_mgf1, NID_sha256, PSS_DEFAULT_SALT_LEN},
    {pss_sha256_salt_30, sizeof(pss_sha256_salt_30), NID_sha256, NID_mgf1,
     NID_sha256, 30},
    {jdk_pss_sha256_salt_0, sizeof(jdk_pss_sha256_salt_0), NID_sha256, NID_mgf1,
     NID_sha256, 0},
    {jdk_pss_sha256_salt_absent, sizeof(jdk_pss_sha256_salt_absent), NID_sha256,
     NID_mgf1, NID_sha256, omit_salt_len},
    {jdk_pss_sha256_salt_30, sizeof(jdk_pss_sha256_salt_30), NID_sha256,
     NID_mgf1, NID_sha256, 30},
    {jdk_pss_sha224_salt_absent, sizeof(jdk_pss_sha224_salt_absent), NID_sha224,
     NID_mgf1, NID_sha224, omit_salt_len},
    {jdk_pss_sha384_salt_absent, sizeof(jdk_pss_sha384_salt_absent), NID_sha384,
     NID_mgf1, NID_sha384, omit_salt_len},
    {jdk_pss_sha512_salt_absent, sizeof(jdk_pss_sha512_salt_absent), NID_sha512,
     NID_mgf1, NID_sha512, omit_salt_len},
    {jdk_pss_sha256_mgf1_sha512, sizeof(jdk_pss_sha256_mgf1_sha512), NID_sha256,
     NID_mgf1, NID_sha512, omit_salt_len},
    {jdk_pss_sha1_mgf1_sha256, sizeof(jdk_pss_sha1_mgf1_sha256), NID_undef,
     NID_mgf1, NID_sha256, omit_salt_len},
    {jdk_pss_sha256_mgf1_sha1, sizeof(jdk_pss_sha256_mgf1_sha1), NID_sha256,
     NID_undef, NID_undef, omit_salt_len},
    {jdk_pss_sha1_mgf1_sha1_salt_30, sizeof(jdk_pss_sha1_mgf1_sha1_salt_30), NID_undef,
     NID_undef, NID_undef, 30},
};

class PssParseTest : public testing::TestWithParam<PssParseTestInput> {};

TEST_P(PssParseTest, DecodeParamsDer) {
  const auto &param = GetParam();
  CBS params;
  params.data = param.der;
  params.len = param.der_len;
  RSASSA_PSS_PARAMS *pss = nullptr;
  ASSERT_TRUE(RSASSA_PSS_parse_params(&params, &pss));
  // Holds ownership of heap-allocated RSASSA_PSS_PARAMS.
  bssl::UniquePtr<RSASSA_PSS_PARAMS> pss_ptr(pss);
  // Expect all bytes of params are consumed.
  ASSERT_FALSE(CBS_len(&params));
  // Validate Hash Algorithm of RSASSA-PSS-params.
  if (param.expected_hash_nid != NID_undef) {
    ASSERT_TRUE(pss->hash_algor);
    EXPECT_EQ(pss->hash_algor->nid, param.expected_hash_nid);
  } else {
    // When no expectation, make sure this value is absent.
    EXPECT_FALSE(pss->hash_algor);
  }
  // Validate Mask Algorithm of RSASSA-PSS-params.
  RSA_MGA_IDENTIFIER *mga = pss->mask_gen_algor;
  if (param.expected_mask_gen_nid != NID_undef) {
    ASSERT_TRUE(mga);
    ASSERT_TRUE(mga->mask_gen);
    EXPECT_EQ(mga->mask_gen->nid, param.expected_mask_gen_nid);
    // Validate Hash Algorithm of Mask Gen.
    if (param.expected_mgf1_hash_nid) {
      ASSERT_TRUE(mga->one_way_hash);
      EXPECT_EQ(mga->one_way_hash->nid, param.expected_mgf1_hash_nid);
    } else {
      // When no expectation, make sure this value is absent.
      EXPECT_FALSE(mga->one_way_hash);
    }
  } else {
    // When no expectation, make sure this value is absent.
    EXPECT_FALSE(mga);
  }
  // Validate Minimum Salt Length of RSASSA-PSS-params.
  if (param.expected_salt_len != omit_salt_len) {
    ASSERT_TRUE(pss->salt_len);
    EXPECT_EQ(pss->salt_len->value, param.expected_salt_len);
  } else {
    // When no expectation, make sure this value is absent.
    EXPECT_FALSE(pss->salt_len);
  }
  // Perform signature generation MUST omit the trailerField field.
  // https://tools.ietf.org/html/rfc4055#page-8
  EXPECT_FALSE(pss->trailer_field);
}

INSTANTIATE_TEST_SUITE_P(RsassaPssAll, PssParseTest,
                         testing::ValuesIn(kPssParseTestInputs));

struct InvalidPssParseInput {
  const uint8_t *der;
  size_t der_len;
} kInvalidPssParseInputs[] = {
    {invalid_pss_salt_missing, sizeof(invalid_pss_salt_missing)},
    {pss_with_invalid_sha256_oid, sizeof(pss_with_invalid_sha256_oid)},
    {pss_with_invalid_mgf1_oid, sizeof(pss_with_invalid_mgf1_oid)},
    {pss_with_tag2_before_tag0, sizeof(pss_with_tag2_before_tag0)},
    {pss_with_tag4, sizeof(pss_with_tag4)},
    {pss_with_double_salt_30, sizeof(pss_with_double_salt_30)},
    {pss_with_sequence_length_too_short, sizeof(pss_with_sequence_length_too_short)},
    {pss_with_sequence_length_too_long, sizeof(pss_with_sequence_length_too_long)},
    {pss_with_tag2_length_too_short, sizeof(pss_with_tag2_length_too_short)},
    {pss_with_tag2_length_too_long, sizeof(pss_with_tag2_length_too_long)},
    {pss_with_negative_salt_length, sizeof(pss_with_negative_salt_length)},
    {pss_with_trailer_field_not_1, sizeof(pss_with_trailer_field_not_1)},
};

class InvalidPssParseTest
    : public testing::TestWithParam<InvalidPssParseInput> {};

TEST_P(InvalidPssParseTest, DecodeInvalidDer) {
  const auto &param = GetParam();
  CBS params;
  params.data = param.der;
  params.len = param.der_len;
  RSASSA_PSS_PARAMS *pss = nullptr;
  ASSERT_FALSE(RSASSA_PSS_parse_params(&params, &pss));
  ERR_clear_error();
}

INSTANTIATE_TEST_SUITE_P(RsassaPssAll, InvalidPssParseTest,
                         testing::ValuesIn(kInvalidPssParseInputs));

struct PssConversionTestInput {
  const EVP_MD *md;
  int saltlen;
  int expect_pss_is_null;
  int expect_pss_hash_is_null;
  int expect_pss_saltlen_is_null;
} kPssConversionTestInputs[] = {
    {EVP_sha1(), PSS_DEFAULT_SALT_LEN, 0, 1, 1},
    {EVP_sha224(), 30, 0, 0, 0},
    {EVP_sha256(), 30, 0, 0, 0},
    {EVP_sha384(), 30, 0, 0, 0},
    {EVP_sha512(), 30, 0, 0, 0},
    // This test case happens in |pkey_rsa_init| of |p_rsa.c|.
    {nullptr, -2, 1, 1, 1},
};

static void test_RSASSA_PSS_PARAMS_get(RSASSA_PSS_PARAMS *pss,
                                       const EVP_MD *expect_md,
                                       int expect_saltlen) {
  const EVP_MD *sigmd = nullptr;
  const EVP_MD *mgf1md = nullptr;
  int saltlen = -1;
  ASSERT_TRUE(RSASSA_PSS_PARAMS_get(pss, &sigmd, &mgf1md, &saltlen));
  EXPECT_EQ(sigmd, expect_md);
  EXPECT_EQ(mgf1md, expect_md);
  EXPECT_EQ(saltlen, expect_saltlen);
}

class PssConversionTest : public testing::TestWithParam<PssConversionTestInput> {};

// This test is to check the conversion between |RSASSA_PSS_PARAMS| and
// (|*sigmd|, |*mgf1md| and |saltlen|), which are fields of |RSA_PKEY_CTX|.
// The 1st step is to validate |RSASSA_PSS_PARAMS_create|.
// The 2nd step is to validate |RSASSA_PSS_PARAMS_get|.
TEST_P(PssConversionTest, CreationAndGetSuccess) {
  const auto &param = GetParam();
  RSASSA_PSS_PARAMS *pss = nullptr;
  const EVP_MD *md = param.md;
  int saltlen = param.saltlen;
  // STEP 1: validate |RSASSA_PSS_PARAMS_create|.
  ASSERT_TRUE(RSASSA_PSS_PARAMS_create(md, md, saltlen, &pss));
  // Validate if the pss should be allocated.
  if (param.expect_pss_is_null) {
    ASSERT_FALSE(pss);
    return;
  } else {
    ASSERT_TRUE(pss);
  }
  // Holds ownership of heap-allocated RSASSA_PSS_PARAMS.
  bssl::UniquePtr<RSASSA_PSS_PARAMS> pss_ptr(pss);
  // Validate the hash_algor of pss.
  if (param.expect_pss_hash_is_null) {
    // hash_algor is NULL when the default value (sha1) is passed.
    // Pss params encode expects this default value is omitted.
    // See 3.1. https://tools.ietf.org/html/rfc4055#section-3.1
    // MUST omit the hashAlgorithm field when SHA-1 is used.
    EXPECT_FALSE(pss->hash_algor);
    EXPECT_FALSE(pss->mask_gen_algor);
  } else {
    ASSERT_TRUE(pss->hash_algor);
    ASSERT_TRUE(pss->mask_gen_algor);
    ASSERT_TRUE(pss->mask_gen_algor->one_way_hash);
    EXPECT_EQ(pss->hash_algor->nid, EVP_MD_type(md));
    EXPECT_EQ(pss->mask_gen_algor->one_way_hash->nid, EVP_MD_type(md));
  }
  // Validate the saltlen of pss.
  if (param.expect_pss_saltlen_is_null) {
    // salt_len is NULL when the default value (20) is passed.
    // Pss params encode expects the default value to be omitted.
    // This is not a MUST but it is implemented in OpenSSL.
    EXPECT_FALSE(pss->salt_len);
  } else {
    ASSERT_TRUE(pss->salt_len);
    EXPECT_EQ(pss->salt_len->value, saltlen);
  }
  // Trailer field should not be set because it's for encoding only.
  // See 3.1. https://tools.ietf.org/html/rfc4055#section-3.1
  // MUST omit the trailerField field.
  EXPECT_FALSE(pss->trailer_field);

  // STEP 2: validate |RSASSA_PSS_PARAMS_get|.
  // Validate the conversion from |RSASSA_PSS_PARAMS| to
  // (|*sigmd|, |*mgf1md| and |saltlen|).
  test_RSASSA_PSS_PARAMS_get(pss, md, saltlen);
}

INSTANTIATE_TEST_SUITE_P(RsassaPssAll, PssConversionTest,
                         testing::ValuesIn(kPssConversionTestInputs));

TEST(RsassaPssTest, DecodeParamsDerWithTrailerField) {
  CBS params;
  params.data = pss_with_trailer_field_1;
  params.len = sizeof(pss_with_trailer_field_1);
  RSASSA_PSS_PARAMS *pss = nullptr;
  ASSERT_TRUE(RSASSA_PSS_parse_params(&params, &pss));
  // Holds ownership of heap-allocated RSASSA_PSS_PARAMS.
  bssl::UniquePtr<RSASSA_PSS_PARAMS> pss_ptr(pss);
  // Expect all bytes of params are used.
  ASSERT_FALSE(CBS_len(&params));
  // Validate Hash Algorithm of RSASSA-PSS-params.
  EXPECT_FALSE(pss->hash_algor);
  // Validate Mask Algorithm of RSASSA-PSS-params.
  EXPECT_FALSE(pss->mask_gen_algor);
  // Validate Salt Length of RSASSA-PSS-params.
  EXPECT_FALSE(pss->salt_len);
  // Validate Trailer field of RSASSA-PSS-params.
  ASSERT_TRUE(pss->trailer_field);
  EXPECT_EQ(pss->trailer_field->value, PSS_TRAILER_FIELD_VALUE);
}

struct InvalidPssCreateTestInput {
  const EVP_MD *md;
  int saltlen;
} kInvalidPssCreateTestInputs[] = {
    // Expect test fails because saltlen cannot be negative.
    {EVP_sha256(), -1},
    // Expect test fails because md5 is not supported.
    {EVP_md5(), PSS_DEFAULT_SALT_LEN},
};

class InvalidPssCreateTest
    : public testing::TestWithParam<InvalidPssCreateTestInput> {};

TEST_P(InvalidPssCreateTest, CreationFailure) {
  const auto &param = GetParam();
  RSASSA_PSS_PARAMS *pss = nullptr;
  const EVP_MD *md = param.md;
  int saltlen = param.saltlen;
  ASSERT_FALSE(RSASSA_PSS_PARAMS_create(md, md, saltlen, &pss));
  ASSERT_FALSE(pss);
  ERR_clear_error();
}

INSTANTIATE_TEST_SUITE_P(RsassaPssAll, InvalidPssCreateTest,
                         testing::ValuesIn(kInvalidPssCreateTestInputs));

struct InvalidPssGetTestInput {
  int nid;
  int saltlen;
  int trailerfiled;
} kInvalidPssGetTestInputs[] = {
    // Expect test fails because md5 is not supported.
    {NID_md5, PSS_DEFAULT_SALT_LEN, PSS_TRAILER_FIELD_VALUE},
    // Expect test fails because saltlen cannot be negative.
    {NID_sha256, -1, PSS_TRAILER_FIELD_VALUE},
    // Expect test fails because trailer field MUST be 1.
    {NID_sha256, PSS_DEFAULT_SALT_LEN, 2},
};

class InvalidPssGetTest
    : public testing::TestWithParam<InvalidPssGetTestInput> {};

TEST_P(InvalidPssGetTest, GetFailure) {
  const auto &param = GetParam();
  RSA_ALGOR_IDENTIFIER pss_hash = {param.nid};
  RSA_MGA_IDENTIFIER pss_mga = {nullptr, &pss_hash};
  RSA_INTEGER pss_saltlen = {param.saltlen};
  RSA_INTEGER pss_trailer_field = {param.trailerfiled};
  RSASSA_PSS_PARAMS pss = {
      &pss_hash,
      &pss_mga,
      &pss_saltlen,
      &pss_trailer_field,
  };
  const EVP_MD *sigmd = nullptr;
  const EVP_MD *mgf1md = nullptr;
  int saltlen = -1;
  ASSERT_FALSE(RSASSA_PSS_PARAMS_get(&pss, &sigmd, &mgf1md, &saltlen));
  ERR_clear_error();
}

INSTANTIATE_TEST_SUITE_P(RsassaPssAll, InvalidPssGetTest,
                         testing::ValuesIn(kInvalidPssGetTestInputs));

TEST(InvalidPssGetTest, PssIsNull) {
  const EVP_MD *sigmd = nullptr;
  const EVP_MD *mgf1md = nullptr;
  int saltlen = -1;
  ASSERT_FALSE(RSASSA_PSS_PARAMS_get(nullptr, &sigmd, &mgf1md, &saltlen));
  ERR_clear_error();
}

TEST(PssGetTest, AllParamsAbsent) {
  RSASSA_PSS_PARAMS pss = {
      nullptr,
      nullptr,
      nullptr,
      nullptr,
  };
  test_RSASSA_PSS_PARAMS_get(&pss, EVP_sha1(), PSS_DEFAULT_SALT_LEN);
}
