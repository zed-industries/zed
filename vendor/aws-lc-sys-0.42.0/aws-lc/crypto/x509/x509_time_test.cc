// Copyright 2017 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Tests for X509 time functions.

#include <openssl/x509.h>

#include <string.h>
#include <time.h>

#include <gtest/gtest.h>
#include <openssl/asn1.h>

struct TestData {
  const char *data;
  int type;
  int64_t cmp_time;
  // -1 if asn1_time <= cmp_time, 1 if asn1_time > cmp_time, 0 if error.
  int expected;
};

static TestData kX509CmpTests[] = {
    {
        "20170217180154Z",
        V_ASN1_GENERALIZEDTIME,
        // The same in seconds since epoch.
        1487354514,
        -1,
    },
    {
        "20170217180154Z",
        V_ASN1_GENERALIZEDTIME,
        // One second more.
        1487354515,
        -1,
    },
    {
        "20170217180154Z",
        V_ASN1_GENERALIZEDTIME,
        // One second less.
        1487354513,
        1,
    },
    // Same as UTC time.
    {
        "170217180154Z",
        V_ASN1_UTCTIME,
        // The same in seconds since epoch.
        1487354514,
        -1,
    },
    {
        "170217180154Z",
        V_ASN1_UTCTIME,
        // One second more.
        1487354515,
        -1,
    },
    {
        "170217180154Z",
        V_ASN1_UTCTIME,
        // One second less.
        1487354513,
        1,
    },
    // UTCTime from the 20th century.
    {
        "990217180154Z",
        V_ASN1_UTCTIME,
        // The same in seconds since epoch.
        919274514,
        -1,
    },
    {
        "990217180154Z",
        V_ASN1_UTCTIME,
        // One second more.
        919274515,
        -1,
    },
    {
        "990217180154Z",
        V_ASN1_UTCTIME,
        // One second less.
        919274513,
        1,
    },
    // Various invalid formats.
    {
        // No trailing Z.
        "20170217180154",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // No trailing Z, UTCTime.
        "170217180154",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // No seconds.
        "201702171801Z",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // No seconds, UTCTime.
        "1702171801Z",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Fractional seconds.
        "20170217180154.001Z",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // Fractional seconds, UTCTime.
        "170217180154.001Z",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Timezone offset.
        "20170217180154+0100",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // Timezone offset, UTCTime.
        "170217180154+0100",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Extra digits.
        "2017021718015400Z",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // Extra digits, UTCTime.
        "17021718015400Z",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Non-digits.
        "2017021718015aZ",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // Non-digits, UTCTime.
        "17021718015aZ",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Trailing garbage.
        "20170217180154Zlongtrailinggarbage",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // Trailing garbage, UTCTime.
        "170217180154Zlongtrailinggarbage",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Swapped type.
        "20170217180154Z",
        V_ASN1_UTCTIME,
        0,
        0,
    },
    {
        // Swapped type.
        "170217180154Z",
        V_ASN1_GENERALIZEDTIME,
        0,
        0,
    },
    {
        // Bad type.
        "20170217180154Z",
        V_ASN1_OCTET_STRING,
        0,
        0,
    },
    // Test limits and unusual cases.
    {
        "99991231235959Z", V_ASN1_GENERALIZEDTIME,
        // Test a very large positive time with the largest representable time
        253402300799,
        -1,  // TODO(bbe): This is *technically* wrong by rfc5280.
    },
    {
        "99991231235959Z", V_ASN1_GENERALIZEDTIME,
        // one second after the largest possible time should still compare
        // correctly
        253402300800,
        -1,  // TODO(bbe): This is *technically* wrong by rfc5280.
    },
    {
        "99991231235959Z",
        V_ASN1_GENERALIZEDTIME,
        // Test one second before the largest time
        253402300798,
        1,
    },
    {
        "700101000000Z",
        V_ASN1_UTCTIME,
        // The epoch, which should not fail. a time of 0 must be valid.
        0,
        -1,
    },
    {
        "700101000000Z",
        V_ASN1_UTCTIME,
        // One second before the epoch should compare correctly.
        -1,
        1,
    },
    {
        "700101000000Z",
        V_ASN1_UTCTIME,
        // One second after the epoch should compare correctly.
        1,
        -1,
    },
    {
        "690621025615Z",
        V_ASN1_UTCTIME,
        // Test a negative time, we use a time from NASA, close to but not quite
        // at the epoch.
        -16751025,
        -1,
    },
    {
        "690621025615Z",
        V_ASN1_UTCTIME,
        // Test one small second before our negative time.
        -16751026,
        1,
    },
    {
        "690621025615Z",
        V_ASN1_UTCTIME,
        // Test one giant second after our negative time.
        -16751024,
        -1,
    },
    {
        "00000101000000Z",
        V_ASN1_GENERALIZEDTIME,
        // Test a very large negative time with the earliest representable time
        -62167219200,
        -1,
    },
    {
        "00000101000000Z",
        V_ASN1_GENERALIZEDTIME,
        // Test one second after the earliest time.
        -62167219199,
        -1,
    },

};

TEST(X509TimeTest, TestCmpTime) {
  for (auto &test : kX509CmpTests) {
    SCOPED_TRACE(test.data);

    bssl::UniquePtr<ASN1_STRING> t(ASN1_STRING_type_new(test.type));
    ASSERT_TRUE(t);
    ASSERT_TRUE(ASN1_STRING_set(t.get(), test.data, strlen(test.data)));

    EXPECT_EQ(test.expected, X509_cmp_time_posix(t.get(), test.cmp_time));
  }
}

TEST(X509TimeTest, TestCmpTimeCurrent) {
  time_t now = time(NULL);
  // Pick a day earlier and later, relative to any system clock.
  bssl::UniquePtr<ASN1_TIME> asn1_before(ASN1_TIME_adj(NULL, now, -1, 0));
  bssl::UniquePtr<ASN1_TIME> asn1_after(ASN1_TIME_adj(NULL, now, 1, 0));

  ASSERT_EQ(-1, X509_cmp_time(asn1_before.get(), NULL));
  ASSERT_EQ(1, X509_cmp_time(asn1_after.get(), NULL));
}
