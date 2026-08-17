// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC


// ssl_test_ticket_aead_failure_mode enumerates the possible ways in which the
// test |SSL_TICKET_AEAD_METHOD| can fail.

#include <gtest/gtest.h>

#include "../crypto/test/file_util.h"
#include "../crypto/test/test_util.h"
#include "internal.h"
#include "ssl_common_test.h"

BSSL_NAMESPACE_BEGIN


struct ExpectedCipher {
  unsigned long id;
  int in_group_flag;
};

struct CipherTest {
  // The rule string to apply.
  const char *rule;
  // The list of expected ciphers, in order.
  std::vector<ExpectedCipher> expected;
  // True if this cipher list should fail in strict mode.
  bool strict_fail;
};

struct CurveTest {
  // The rule string to apply.
  const char *rule;
  // The list of expected curves, in order.
  std::vector<uint16_t> expected;
};


static const CipherTest kCipherTests[] = {
    // Selecting individual ciphers should work.
    {
        "ECDHE-ECDSA-CHACHA20-POLY1305:"
        "ECDHE-RSA-CHACHA20-POLY1305:"
        "ECDHE-ECDSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES128-SHA256:"
        "ECDHE-RSA-AES128-GCM-SHA256",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // + reorders selected ciphers to the end, keeping their relative order.
    {
        "ECDHE-ECDSA-CHACHA20-POLY1305:"
        "ECDHE-RSA-CHACHA20-POLY1305:"
        "ECDHE-ECDSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES128-GCM-SHA256:"
        "+aRSA",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // ! banishes ciphers from future selections.
    {
        "!aRSA:"
        "ECDHE-ECDSA-CHACHA20-POLY1305:"
        "ECDHE-RSA-CHACHA20-POLY1305:"
        "ECDHE-ECDSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES128-GCM-SHA256",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // Multiple masks can be ANDed in a single rule.
    {
        "kRSA+AESGCM+AES128",
        {
            {TLS1_CK_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // - removes selected ciphers, but preserves their order for future
    // selections. Select AES_128_GCM, but order the key exchanges RSA,
    // ECDHE_RSA.
    {
        "ALL:-kECDHE:"
        "-kRSA:-ALL:"
        "AESGCM+AES128+aRSA",
        {
            {TLS1_CK_RSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // Unknown selectors are no-ops, except in strict mode.
    {
        "ECDHE-ECDSA-CHACHA20-POLY1305:"
        "ECDHE-RSA-CHACHA20-POLY1305:"
        "ECDHE-ECDSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES128-GCM-SHA256:"
        "BOGUS1",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        true,
    },
    // Unknown selectors are no-ops, except in strict mode.
    {
        "ECDHE-ECDSA-CHACHA20-POLY1305:"
        "ECDHE-RSA-CHACHA20-POLY1305:"
        "ECDHE-ECDSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES128-GCM-SHA256:"
        "-BOGUS2:+BOGUS3:!BOGUS4",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        true,
    },
    // Square brackets specify equi-preference groups.
    {
        "[ECDHE-ECDSA-CHACHA20-POLY1305|ECDHE-ECDSA-AES128-GCM-SHA256]:"
        "[ECDHE-RSA-CHACHA20-POLY1305]:"
        "ECDHE-RSA-AES128-GCM-SHA256",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 1},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // Standard names may be used instead of OpenSSL names.
    {
        "[TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256|"
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256]:"
        "[TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256]:"
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256, 1},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // @STRENGTH performs a stable strength-sort of the selected ciphers and
    // only the selected ciphers.
    {
        // To simplify things, banish all but {ECDHE_RSA,RSA} x
        // {CHACHA20,AES_256_CBC,AES_128_CBC} x SHA1.
        "!AESGCM:!3DES:"
        // Order some ciphers backwards by strength.
        "ALL:-CHACHA20:-AES256:-AES128:-ALL:"
        // Select ECDHE ones and sort them by strength. Ties should resolve
        // based on the order above.
        "kECDHE:@STRENGTH:-ALL:"
        // Now bring back everything uses RSA. ECDHE_RSA should be first, sorted
        // by strength. Then RSA, backwards by strength.
        "aRSA",
        {
            {TLS1_CK_ECDHE_RSA_WITH_AES_256_CBC_SHA, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_256_SHA384, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_CBC_SHA, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_SHA256, 0},
            {TLS1_CK_RSA_WITH_AES_128_SHA, 0},
            {TLS1_CK_RSA_WITH_AES_128_SHA256, 0},
            {TLS1_CK_RSA_WITH_AES_256_SHA, 0},
        },
        false,
    },
    // Additional masks after @STRENGTH get silently discarded.
    //
    // TODO(davidben): Make this an error. If not silently discarded, they get
    // interpreted as + opcodes which are very different.
    {
        "ECDHE-RSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES256-GCM-SHA384:"
        "@STRENGTH+AES256",
        {
            {TLS1_CK_ECDHE_RSA_WITH_AES_256_GCM_SHA384, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    {
        "ECDHE-RSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES256-GCM-SHA384:"
        "@STRENGTH+AES256:"
        "ECDHE-RSA-CHACHA20-POLY1305",
        {
            {TLS1_CK_ECDHE_RSA_WITH_AES_256_GCM_SHA384, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256, 0},
        },
        false,
    },
    // Exact ciphers may not be used in multi-part rules; they are treated
    // as unknown aliases.
    {
        "ECDHE-ECDSA-AES128-GCM-SHA256:"
        "ECDHE-RSA-AES128-GCM-SHA256:"
        "!ECDHE-RSA-AES128-GCM-SHA256+RSA:"
        "!ECDSA+ECDHE-ECDSA-AES128-GCM-SHA256",
        {
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        true,
    },
    // SSLv3 matches everything that existed before TLS 1.2.
    {
        "AES128-SHA:AES128-SHA256:ECDHE-RSA-AES128-GCM-SHA256:!SSLv3",
        {
            {TLS1_CK_RSA_WITH_AES_128_SHA256, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // TLSv1.2 matches everything added in TLS 1.2.
    {
        "AES128-SHA:AES128-SHA256:ECDHE-RSA-AES128-GCM-SHA256:!TLSv1.2",
        {
            {TLS1_CK_RSA_WITH_AES_128_SHA, 0},
        },
        false,
    },
    // The two directives have no intersection.  But each component is valid, so
    // even in strict mode it is accepted.
    {
        "AES128-SHA:ECDHE-RSA-AES128-GCM-SHA256:!TLSv1.2+SSLv3",
        {
            {TLS1_CK_RSA_WITH_AES_128_SHA, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // Spaces, semi-colons and commas are separators.
    {
        "AES128-SHA: ECDHE-RSA-AES128-GCM-SHA256 AES256-SHA "
        ",ECDHE-ECDSA-AES128-GCM-SHA256 ; AES128-GCM-SHA256",
        {
            {TLS1_CK_RSA_WITH_AES_128_SHA, 0},
            {TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_RSA_WITH_AES_256_SHA, 0},
            {TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0},
            {TLS1_CK_RSA_WITH_AES_128_GCM_SHA256, 0},
        },
        // …but not in strict mode.
        true,
    },
};

// In kTLSv13RuleOnly, |rule| of each CipherTest can only match TLSv1.3 ciphers.
// e.g. kTLSv13RuleOnly does not have rule "AESGCM+AES128", which can match
// some TLSv1.2 ciphers.
static const CipherTest kTLSv13RuleOnly[] = {
    // Selecting individual ciphers should work.
    {
        "TLS_AES_256_GCM_SHA384:"
        "TLS_CHACHA20_POLY1305_SHA256:"
        "TLS_AES_128_GCM_SHA256",
        {
            {TLS1_CK_AES_256_GCM_SHA384, 0},
            {TLS1_CK_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // + reorders selected ciphers to the end, keeping their relative order.
    {
        "TLS_AES_256_GCM_SHA384:"
        "TLS_CHACHA20_POLY1305_SHA256:"
        "TLS_AES_128_GCM_SHA256:"
        "+AES",
        {
            {TLS1_CK_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_AES_256_GCM_SHA384, 0},
            {TLS1_CK_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // ! banishes ciphers from future selections.
    {
        "!CHACHA20:"
        "TLS_AES_256_GCM_SHA384:"
        "TLS_CHACHA20_POLY1305_SHA256:"
        "TLS_AES_128_GCM_SHA256:"
        "TLS_CHACHA20_POLY1305_SHA256",
        {
            {TLS1_CK_AES_256_GCM_SHA384, 0},
            {TLS1_CK_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // Square brackets specify equi-preference groups.
    {
        "[TLS_AES_128_GCM_SHA256|TLS_AES_256_GCM_SHA384]:"
        "TLS_CHACHA20_POLY1305_SHA256",
        {
            {TLS1_CK_AES_128_GCM_SHA256, 1},
            {TLS1_CK_AES_256_GCM_SHA384, 0},
            {TLS1_CK_CHACHA20_POLY1305_SHA256, 0},
        },
        false,
    },
    // Spaces, semi-colons and commas are separators.
    {
        "TLS_AES_256_GCM_SHA384 ; "
        "TLS_CHACHA20_POLY1305_SHA256 , "
        "TLS_AES_128_GCM_SHA256",
        {
            {TLS1_CK_AES_256_GCM_SHA384, 0},
            {TLS1_CK_CHACHA20_POLY1305_SHA256, 0},
            {TLS1_CK_AES_128_GCM_SHA256, 0},
        },
        false,
    },
};

// Besides kTLSv13RuleOnly, additional rules can match TLSv1.3 ciphers.
static const CipherTest kTLSv13CipherTests[] = {
    // Multiple masks can be ANDed in a single rule.
    {
        "AESGCM+AES128",
        {
            {TLS1_CK_AES_128_GCM_SHA256, 0},
        },
        false,
    },
    // - removes selected ciphers, but preserves their order for future
    // selections. Select AES_128_GCM, but order the key exchanges RSA,
    // ECDHE_RSA.
    {
        "ALL:-AES",
        {
            {TLS1_CK_CHACHA20_POLY1305_SHA256, 0},
        },
        false,
    },
    // Unknown selectors or TLSv1.2 ciphers are no-ops.
    {
        "TLS_CHACHA20_POLY1305_SHA256:"
        "ECDHE-RSA-CHACHA20-POLY1305:"
        "BOGUS1",
        {
            {TLS1_CK_CHACHA20_POLY1305_SHA256, 0},
        },
        true,
    },
};

static const char *kBadRules[] = {
    // Invalid brackets.
    "[ECDHE-RSA-CHACHA20-POLY1305|ECDHE-RSA-AES128-GCM-SHA256",
    "RSA]",
    "[[RSA]]",
    // Operators inside brackets.
    "[+RSA]",
    // Unknown directive.
    "@BOGUS",
    "BOGUS",
    // COMPLEMENTOFDEFAULT is empty.
    "COMPLEMENTOFDEFAULT",
    // Invalid command.
    "?BAR",
    // Special operators are not allowed if equi-preference groups are used.
    "[ECDHE-RSA-CHACHA20-POLY1305|ECDHE-RSA-AES128-GCM-SHA256]:+FOO",
    "[ECDHE-RSA-CHACHA20-POLY1305|ECDHE-RSA-AES128-GCM-SHA256]:!FOO",
    "[ECDHE-RSA-CHACHA20-POLY1305|ECDHE-RSA-AES128-GCM-SHA256]:-FOO",
    "[ECDHE-RSA-CHACHA20-POLY1305|ECDHE-RSA-AES128-GCM-SHA256]:@STRENGTH",
    // Opcode supplied, but missing selector.
    "+",
    // Spaces are forbidden in equal-preference groups.
    "[AES128-SHA | AES128-SHA256]",
};

static const char *kMustNotIncludeNull[] = {
    "ALL",  "DEFAULT", "HIGH",  "FIPS",  "SHA",
    "SHA1", "RSA",     "SSLv3", "TLSv1", "TLSv1.2",
};

static const char *kTLSv13MustNotIncludeNull[] = {
    "ALL",
    "DEFAULT",
    "HIGH",
    "FIPS",
};

static const char *kMustNotInclude3DES[] = {
    "ALL", "DEFAULT", "HIGH", "FIPS", "SSLv3", "TLSv1", "TLSv1.2",
};

static const CurveTest kCurveTests[] = {
    {
        "P-256",
        {SSL_GROUP_SECP256R1},
    },
    {
        "P-256:P-384:P-521:X25519",
        {
            SSL_GROUP_SECP256R1,
            SSL_GROUP_SECP384R1,
            SSL_GROUP_SECP521R1,
            SSL_GROUP_X25519,
        },
    },
    {
        "prime256v1:secp384r1:secp521r1:x25519",
        {
            SSL_GROUP_SECP256R1,
            SSL_GROUP_SECP384R1,
            SSL_GROUP_SECP521R1,
            SSL_GROUP_X25519,
        },
    },
    {
        "SecP256r1MLKEM768:prime256v1:secp384r1:secp521r1:x25519",
        {
            SSL_GROUP_SECP256R1_MLKEM768,
            SSL_GROUP_SECP256R1,
            SSL_GROUP_SECP384R1,
            SSL_GROUP_SECP521R1,
            SSL_GROUP_X25519,
        },
    },
    {
        "X25519MLKEM768:prime256v1:secp384r1",
        {
            SSL_GROUP_X25519_MLKEM768,
            SSL_GROUP_SECP256R1,
            SSL_GROUP_SECP384R1,
        },
    },
    {
        "X25519:X25519MLKEM768",
        {
            SSL_GROUP_X25519,
            SSL_GROUP_X25519_MLKEM768,
        },
    },
    {
        "X25519:SecP256r1MLKEM768:prime256v1",
        {
            SSL_GROUP_X25519,
            SSL_GROUP_SECP256R1_MLKEM768,
            SSL_GROUP_SECP256R1,
        },
    },
    {
        "SecP384r1MLKEM1024:X25519MLKEM768:SecP256r1MLKEM768",
        {
            SSL_GROUP_SECP384R1_MLKEM1024,
            SSL_GROUP_X25519_MLKEM768,
            SSL_GROUP_SECP256R1_MLKEM768,
        },
    },
    {
      "MLKEM512:MLKEM768:MLKEM1024",
      {
        SSL_GROUP_MLKEM512,
        SSL_GROUP_MLKEM768,
        SSL_GROUP_MLKEM1024,
      },
    },
    {
      "ML-KEM-512:ML-KEM-768:ML-KEM-1024",
      {
        SSL_GROUP_MLKEM512,
        SSL_GROUP_MLKEM768,
        SSL_GROUP_MLKEM1024,
      },
    },
};



static const char *kBadCurvesLists[] = {
    "",
    ":",
    "::",
    "P-256::X25519",
    "RSA:P-256",
    "P-256:RSA",
    "X25519:P-256:",
    ":X25519:P-256",
    "mlkem768",
    "x25519_mlkem768:prime256v1",
};

static STACK_OF(SSL_CIPHER) *tls13_ciphers(const SSL_CTX *ctx) {
  return ctx->tls13_cipher_list->ciphers.get();
}

static std::string CipherListToString(SSL_CTX *ctx, bool tlsv13_ciphers) {
  bool in_group = false;
  std::string ret;
  const STACK_OF(SSL_CIPHER) *ciphers =
      tlsv13_ciphers ? tls13_ciphers(ctx) : SSL_CTX_get_ciphers(ctx);
  for (size_t i = 0; i < sk_SSL_CIPHER_num(ciphers); i++) {
    const SSL_CIPHER *cipher = sk_SSL_CIPHER_value(ciphers, i);
    if (!in_group && SSL_CTX_cipher_in_group(ctx, i)) {
      ret += "\t[\n";
      in_group = true;
    }
    ret += "\t";
    if (in_group) {
      ret += "  ";
    }
    ret += SSL_CIPHER_get_name(cipher);
    ret += "\n";
    if (in_group && !SSL_CTX_cipher_in_group(ctx, i)) {
      ret += "\t]\n";
      in_group = false;
    }
  }
  return ret;
}

static bool CipherListsEqual(SSL_CTX *ctx,
                             const std::vector<ExpectedCipher> &expected,
                             bool tlsv13_ciphers) {
  STACK_OF(SSL_CIPHER) *ciphers =
      tlsv13_ciphers ? tls13_ciphers(ctx) : SSL_CTX_get_ciphers(ctx);
  if (sk_SSL_CIPHER_num(ciphers) != expected.size()) {
    return false;
  }

  for (size_t i = 0; i < expected.size(); i++) {
    const SSL_CIPHER *cipher = sk_SSL_CIPHER_value(ciphers, i);
    if (expected[i].id != SSL_CIPHER_get_id(cipher) ||
        expected[i].in_group_flag != !!SSL_CTX_cipher_in_group(ctx, i)) {
      return false;
    }
  }

  return true;
}

TEST(SSLTest, CipherRules) {
  for (const CipherTest &t : kCipherTests) {
    SCOPED_TRACE(t.rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    // We configure default TLS 1.3 ciphersuites in |SSL_CTX| which pollute
    // |ctx->cipher_list|. Set it to an empty list so we can test TLS 1.2
    // configurations.
    ASSERT_TRUE(SSL_CTX_set_ciphersuites(ctx.get(), ""));

    // Test lax mode.
    ASSERT_TRUE(SSL_CTX_set_cipher_list(ctx.get(), t.rule));
    EXPECT_TRUE(
        CipherListsEqual(ctx.get(), t.expected, false /* not TLSv1.3 only */))
        << "Cipher rule evaluated to:\n"
        << CipherListToString(ctx.get(), false /* not TLSv1.3 only */);

    // Test strict mode.
    if (t.strict_fail) {
      EXPECT_FALSE(SSL_CTX_set_strict_cipher_list(ctx.get(), t.rule));
    } else {
      ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(ctx.get(), t.rule));
      EXPECT_TRUE(
          CipherListsEqual(ctx.get(), t.expected, false /* not TLSv1.3 only */))
          << "Cipher rule evaluated to:\n"
          << CipherListToString(ctx.get(), false /* not TLSv1.3 only */);
    }
  }

  for (const char *rule : kBadRules) {
    SCOPED_TRACE(rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    EXPECT_FALSE(SSL_CTX_set_cipher_list(ctx.get(), rule));
    ERR_clear_error();
  }

  // kTLSv13RuleOnly are valid test cases for |SSL_CTX_set_ciphersuites|,
  // which configures only TLSv1.3 ciphers.
  // |SSL_CTX_set_cipher_list| only supports TLSv1.2 and below ciphers
  // configuration. Accordingly, kTLSv13RuleOnly result in
  // |SSL_R_NO_CIPHER_MATCH|.
  for (const CipherTest &t : kTLSv13RuleOnly) {
    SCOPED_TRACE(t.rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    // We configure default TLS 1.3 ciphersuites in |SSL_CTX| which pollute
    // |ctx->cipher_list|. Set it to an empty list so we can test TLS 1.2
    // configurations.
    ASSERT_TRUE(SSL_CTX_set_ciphersuites(ctx.get(), ""));

    EXPECT_FALSE(SSL_CTX_set_cipher_list(ctx.get(), t.rule));
    ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_CIPHER_MATCH);
    ERR_clear_error();
  }

  for (const char *rule : kMustNotIncludeNull) {
    SCOPED_TRACE(rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(ctx.get(), rule));
    for (const SSL_CIPHER *cipher : SSL_CTX_get_ciphers(ctx.get())) {
      EXPECT_NE(NID_undef, SSL_CIPHER_get_cipher_nid(cipher));
    }
  }

  for (const char *rule : kMustNotInclude3DES) {
    SCOPED_TRACE(rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(ctx.get(), rule));
    for (const SSL_CIPHER *cipher : SSL_CTX_get_ciphers(ctx.get())) {
      EXPECT_NE(NID_des_ede3_cbc, SSL_CIPHER_get_cipher_nid(cipher));
    }
  }
}

static std::vector<CipherTest> combine_tests(const CipherTest *c1,
                                             size_t size_1,
                                             const CipherTest *c2,
                                             size_t size_2) {
  std::vector<CipherTest> ret(size_1 + size_2);
  size_t j = 0;
  for (size_t i = 0; i < size_1; i++) {
    ret[j++] = c1[i];
  }
  for (size_t i = 0; i < size_2; i++) {
    ret[j++] = c2[i];
  }
  return ret;
}

TEST(SSLTest, TLSv13CipherRules) {
  std::vector<CipherTest> cipherRules =
      combine_tests(kTLSv13RuleOnly, OPENSSL_ARRAY_SIZE(kTLSv13RuleOnly),
                    kTLSv13CipherTests, OPENSSL_ARRAY_SIZE(kTLSv13CipherTests));
  for (const CipherTest &t : cipherRules) {
    SCOPED_TRACE(t.rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);
    bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
    ASSERT_TRUE(ssl);

    // Test lax mode.
    ASSERT_TRUE(SSL_CTX_set_ciphersuites(ctx.get(), t.rule));
    ASSERT_TRUE(SSL_set_ciphersuites(ssl.get(), t.rule));
    EXPECT_TRUE(
        CipherListsEqual(ctx.get(), t.expected, true /* TLSv1.3 only */))
        << "Cipher rule evaluated to:\n"
        << CipherListToString(ctx.get(), true /* TLSv1.3 only */);

    // TODO: add |SSL_CTX_set_strict_ciphersuites| and test strict mode.
  }

  for (const char *rule : kBadRules) {
    SCOPED_TRACE(rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);
    bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
    ASSERT_TRUE(ssl);

    EXPECT_FALSE(SSL_CTX_set_ciphersuites(ctx.get(), rule));
    EXPECT_FALSE(SSL_set_ciphersuites(ssl.get(), rule));
    ERR_clear_error();
  }

  for (const char *rule : kTLSv13MustNotIncludeNull) {
    SCOPED_TRACE(rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);
    bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
    ASSERT_TRUE(ssl);

    ASSERT_TRUE(SSL_CTX_set_ciphersuites(ctx.get(), rule));
    ASSERT_TRUE(SSL_set_ciphersuites(ssl.get(), rule));
    // Currenly, only three TLSv1.3 ciphers are supported.
    EXPECT_EQ(3u, sk_SSL_CIPHER_num(tls13_ciphers(ctx.get())));
    for (const SSL_CIPHER *cipher : tls13_ciphers(ctx.get())) {
      EXPECT_NE(NID_undef, SSL_CIPHER_get_cipher_nid(cipher));
    }
  }

  // kCipherTests are valid test cases for |SSL_CTX_set_cipher_list|,
  // which configures only TLSv1.2 and below ciphers.
  // |SSL_CTX_set_ciphersuites| only supports TLSv1.3 ciphers configuration.
  // Accordingly, kCipherTests result in |SSL_R_NO_CIPHER_MATCH|.
  // If kCipherTests starts to include rules that can match TLSv1.3 ciphers,
  // kCipherTests should get splitted.
  for (const CipherTest &t : kCipherTests) {
    SCOPED_TRACE(t.rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);
    bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
    ASSERT_TRUE(ssl);

    EXPECT_FALSE(SSL_CTX_set_ciphersuites(ctx.get(), t.rule));
    EXPECT_FALSE(SSL_set_ciphersuites(ssl.get(), t.rule));
    ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_CIPHER_MATCH);
    ERR_clear_error();
  }
}

TEST(SSLTest, CurveRules) {
  for (const CurveTest &t : kCurveTests) {
    SCOPED_TRACE(t.rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    ASSERT_TRUE(SSL_CTX_set1_groups_list(ctx.get(), t.rule));
    ASSERT_EQ(t.expected.size(), ctx->supported_group_list.size());
    for (size_t i = 0; i < t.expected.size(); i++) {
      EXPECT_EQ(t.expected[i], ctx->supported_group_list[i]);
    }
  }

  for (const char *rule : kBadCurvesLists) {
    SCOPED_TRACE(rule);
    bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(ctx);

    EXPECT_FALSE(SSL_CTX_set1_groups_list(ctx.get(), rule));
    ERR_clear_error();
  }
}



static void ExpectDefaultVersion(uint16_t min_version, uint16_t max_version,
                                 const SSL_METHOD *(*method)(void)) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(method()));
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
  ASSERT_TRUE(ssl);
  EXPECT_EQ(0, SSL_CTX_get_min_proto_version(ctx.get()));
  EXPECT_EQ(0, SSL_CTX_get_max_proto_version(ctx.get()));
  EXPECT_EQ(0, SSL_get_min_proto_version(ssl.get()));
  EXPECT_EQ(0, SSL_get_max_proto_version(ssl.get()));

  EXPECT_TRUE(SSL_CTX_set_min_proto_version(ctx.get(), min_version));
  EXPECT_TRUE(SSL_CTX_set_max_proto_version(ctx.get(), max_version));
  EXPECT_EQ(min_version, SSL_CTX_get_min_proto_version(ctx.get()));
  EXPECT_EQ(max_version, SSL_CTX_get_max_proto_version(ctx.get()));
  EXPECT_TRUE(SSL_set_min_proto_version(ssl.get(), min_version));
  EXPECT_TRUE(SSL_set_max_proto_version(ssl.get(), max_version));
  EXPECT_EQ(min_version, SSL_get_min_proto_version(ssl.get()));
  EXPECT_EQ(max_version, SSL_get_max_proto_version(ssl.get()));
}

TEST(SSLTest, DefaultVersion) {
  ExpectDefaultVersion(TLS1_VERSION, TLS1_3_VERSION, &TLS_method);
  ExpectDefaultVersion(TLS1_VERSION, TLS1_VERSION, &TLSv1_method);
  ExpectDefaultVersion(TLS1_1_VERSION, TLS1_1_VERSION, &TLSv1_1_method);
  ExpectDefaultVersion(TLS1_2_VERSION, TLS1_2_VERSION, &TLSv1_2_method);
  ExpectDefaultVersion(DTLS1_VERSION, DTLS1_2_VERSION, &DTLS_method);
  ExpectDefaultVersion(DTLS1_VERSION, DTLS1_VERSION, &DTLSv1_method);
  ExpectDefaultVersion(DTLS1_2_VERSION, DTLS1_2_VERSION, &DTLSv1_2_method);
}

TEST(SSLTest, CipherProperties) {
  static const struct {
    int id;
    const char *standard_name;
    int cipher_nid;
    int digest_nid;
    int kx_nid;
    int auth_nid;
    int prf_nid;
  } kTests[] = {
      {
          SSL3_CK_RSA_DES_192_CBC3_SHA,
          "TLS_RSA_WITH_3DES_EDE_CBC_SHA",
          NID_des_ede3_cbc,
          NID_sha1,
          NID_kx_rsa,
          NID_auth_rsa,
          NID_md5_sha1,
      },
      {
          TLS1_CK_RSA_WITH_AES_128_SHA,
          "TLS_RSA_WITH_AES_128_CBC_SHA",
          NID_aes_128_cbc,
          NID_sha1,
          NID_kx_rsa,
          NID_auth_rsa,
          NID_md5_sha1,
      },
      {
          TLS1_CK_RSA_WITH_AES_128_SHA256,
          "TLS_RSA_WITH_AES_128_CBC_SHA256",
          NID_aes_128_cbc,
          NID_sha256,
          NID_kx_rsa,
          NID_auth_rsa,
          NID_sha256,
      },
      {
          TLS1_CK_PSK_WITH_AES_256_CBC_SHA,
          "TLS_PSK_WITH_AES_256_CBC_SHA",
          NID_aes_256_cbc,
          NID_sha1,
          NID_kx_psk,
          NID_auth_psk,
          NID_md5_sha1,
      },
      {
          TLS1_CK_ECDHE_RSA_WITH_AES_128_CBC_SHA,
          "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
          NID_aes_128_cbc,
          NID_sha1,
          NID_kx_ecdhe,
          NID_auth_rsa,
          NID_md5_sha1,
      },
      {
          TLS1_CK_ECDHE_RSA_WITH_AES_128_SHA256,
          "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256",
          NID_aes_128_cbc,
          NID_sha256,
          NID_kx_ecdhe,
          NID_auth_rsa,
          NID_sha256,
      },
      {
          TLS1_CK_ECDHE_RSA_WITH_AES_256_CBC_SHA,
          "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
          NID_aes_256_cbc,
          NID_sha1,
          NID_kx_ecdhe,
          NID_auth_rsa,
          NID_md5_sha1,
      },
      {
          TLS1_CK_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
          "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
          NID_aes_128_gcm,
          NID_undef,
          NID_kx_ecdhe,
          NID_auth_rsa,
          NID_sha256,
      },
      {
          TLS1_CK_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
          "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
          NID_aes_128_gcm,
          NID_undef,
          NID_kx_ecdhe,
          NID_auth_ecdsa,
          NID_sha256,
      },
      {
          TLS1_CK_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
          "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
          NID_aes_256_gcm,
          NID_undef,
          NID_kx_ecdhe,
          NID_auth_ecdsa,
          NID_sha384,
      },
      {
          TLS1_CK_ECDHE_PSK_WITH_AES_128_CBC_SHA,
          "TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA",
          NID_aes_128_cbc,
          NID_sha1,
          NID_kx_ecdhe,
          NID_auth_psk,
          NID_md5_sha1,
      },
      {
          TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
          "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
          NID_chacha20_poly1305,
          NID_undef,
          NID_kx_ecdhe,
          NID_auth_rsa,
          NID_sha256,
      },
      {
          TLS1_3_CK_AES_256_GCM_SHA384,
          "TLS_AES_256_GCM_SHA384",
          NID_aes_256_gcm,
          NID_undef,
          NID_kx_any,
          NID_auth_any,
          NID_sha384,
      },
      {
          TLS1_3_CK_AES_128_GCM_SHA256,
          "TLS_AES_128_GCM_SHA256",
          NID_aes_128_gcm,
          NID_undef,
          NID_kx_any,
          NID_auth_any,
          NID_sha256,
      },
      {
          TLS1_3_CK_CHACHA20_POLY1305_SHA256,
          "TLS_CHACHA20_POLY1305_SHA256",
          NID_chacha20_poly1305,
          NID_undef,
          NID_kx_any,
          NID_auth_any,
          NID_sha256,
      },
  };

  for (const auto &t : kTests) {
    SCOPED_TRACE(t.standard_name);

    const SSL_CIPHER *cipher = SSL_get_cipher_by_value(t.id & 0xffff);
    ASSERT_TRUE(cipher);
    EXPECT_STREQ(t.standard_name, SSL_CIPHER_standard_name(cipher));

    EXPECT_EQ(t.cipher_nid, SSL_CIPHER_get_cipher_nid(cipher));
    EXPECT_EQ(t.digest_nid, SSL_CIPHER_get_digest_nid(cipher));
    EXPECT_EQ(t.kx_nid, SSL_CIPHER_get_kx_nid(cipher));
    EXPECT_EQ(t.auth_nid, SSL_CIPHER_get_auth_nid(cipher));
    const EVP_MD *md = SSL_CIPHER_get_handshake_digest(cipher);
    ASSERT_TRUE(md);
    EXPECT_EQ(t.prf_nid, EVP_MD_nid(md));
    EXPECT_EQ(t.prf_nid, SSL_CIPHER_get_prf_nid(cipher));
  }
}

// Correct ID and name
#define TLS13_AES_128_GCM_SHA256_BYTES ((const unsigned char *)"\x13\x01")
#define TLS13_CHACHA20_POLY1305_SHA256_BYTES ((const unsigned char *)"\x13\x03")
// Invalid ID
#define TLS13_AES_256_GCM_SHA384_BYTES ((const unsigned char *)"\x13\x13")
TEST(SSLTest, FindingCipher) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  // Configure only TLS 1.3.
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  SCOPED_TRACE("TLS_AES_128_GCM_SHA256");
  const SSL_CIPHER *cipher1 =
      SSL_CIPHER_find(server.get(), TLS13_AES_128_GCM_SHA256_BYTES);
  ASSERT_TRUE(cipher1);
  EXPECT_STREQ("TLS_AES_128_GCM_SHA256", SSL_CIPHER_standard_name(cipher1));

  SCOPED_TRACE("TLS_CHACHA20_POLY1305_SHA256");
  const SSL_CIPHER *cipher2 =
      SSL_CIPHER_find(server.get(), TLS13_CHACHA20_POLY1305_SHA256_BYTES);
  ASSERT_TRUE(cipher2);
  EXPECT_STREQ("TLS_CHACHA20_POLY1305_SHA256",
               SSL_CIPHER_standard_name(cipher2));

  SCOPED_TRACE("TLS_AES_256_GCM_SHA384");
  const SSL_CIPHER *cipher3 =
      SSL_CIPHER_find(client.get(), TLS13_AES_256_GCM_SHA384_BYTES);
  ASSERT_FALSE(cipher3);
}

TEST(SSLTest, CheckSSLCipherInheritance) {
  // This configures SSL_CTX objects with default TLS 1.2 and 1.3 ciphersuites
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(client_ctx.get(), "TLS_AES_128_GCM_SHA256"));
  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(server_ctx.get(), "TLS_AES_128_GCM_SHA256"));
  ASSERT_TRUE(SSL_CTX_set_cipher_list(client_ctx.get(),
                                      "TLS_RSA_WITH_AES_128_CBC_SHA"));
  ASSERT_TRUE(SSL_CTX_set_cipher_list(server_ctx.get(),
                                      "TLS_RSA_WITH_AES_128_CBC_SHA"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  // Modify CTX ciphersuites
  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(client_ctx.get(), "TLS_AES_256_GCM_SHA384"));
  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(server_ctx.get(), "TLS_AES_256_GCM_SHA384"));
  ASSERT_TRUE(SSL_CTX_set_cipher_list(client_ctx.get(),
                                      "TLS_RSA_WITH_AES_256_CBC_SHA"));
  ASSERT_TRUE(SSL_CTX_set_cipher_list(server_ctx.get(),
                                      "TLS_RSA_WITH_AES_256_CBC_SHA"));

  // Ensure SSL object inherits initial CTX cipher suites
  STACK_OF(SSL_CIPHER) *client_ciphers = SSL_get_ciphers(client.get());
  STACK_OF(SSL_CIPHER) *server_ciphers = SSL_get_ciphers(server.get());
  ASSERT_TRUE(client_ciphers);
  ASSERT_TRUE(server_ciphers);
  ASSERT_EQ(sk_SSL_CIPHER_num(client_ciphers), 2u);
  ASSERT_EQ(sk_SSL_CIPHER_num(server_ciphers), 2u);
  const SSL_CIPHER *tls13_cipher =
      SSL_get_cipher_by_value(TLS1_3_CK_AES_128_GCM_SHA256 & 0xFFFF);
  const SSL_CIPHER *tls12_cipher =
      SSL_get_cipher_by_value(TLS1_CK_RSA_WITH_AES_128_SHA & 0xFFFF);
  ASSERT_TRUE(tls13_cipher);
  ASSERT_TRUE(tls12_cipher);

  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, tls12_cipher));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, tls13_cipher));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, tls12_cipher));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, tls13_cipher));

  SSL_set_shed_handshake_config(client.get(), 1);
  SSL_set_shed_handshake_config(server.get(), 1);

  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  // Ensure we fall back to ctx once config is shed
  client_ciphers = SSL_get_ciphers(client.get());
  server_ciphers = SSL_get_ciphers(server.get());

  tls13_cipher = SSL_get_cipher_by_value(TLS1_3_CK_AES_256_GCM_SHA384 & 0xFFFF);
  tls12_cipher = SSL_get_cipher_by_value(TLS1_CK_RSA_WITH_AES_256_SHA & 0xFFFF);

  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, tls13_cipher));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, tls12_cipher));
}

TEST(SSLTest, SSLGetCiphersReturnsTLS13Default) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  // Configure only TLS 1.3.
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  // Have to ensure config is not shed per current implementation of
  // SSL_get_ciphers.
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get(), ClientConfig(), false));

  // Ensure default TLS 1.3 Ciphersuites are present
  const SSL_CIPHER *cipher1 =
      SSL_get_cipher_by_value(TLS1_3_CK_AES_128_GCM_SHA256 & 0xFFFF);
  ASSERT_TRUE(cipher1);
  const SSL_CIPHER *cipher2 =
      SSL_get_cipher_by_value(TLS1_3_CK_AES_256_GCM_SHA384 & 0xFFFF);
  ASSERT_TRUE(cipher2);
  const SSL_CIPHER *cipher3 =
      SSL_get_cipher_by_value(TLS1_3_CK_CHACHA20_POLY1305_SHA256 & 0xFFFF);
  ASSERT_TRUE(cipher3);

  STACK_OF(SSL_CIPHER) *client_ciphers = SSL_get_ciphers(client.get());
  STACK_OF(SSL_CIPHER) *server_ciphers = SSL_get_ciphers(server.get());

  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, cipher1));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, cipher2));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, cipher3));

  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, cipher1));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, cipher2));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, cipher3));
}

TEST(SSLTest, TLS13ConfigCiphers) {
  // This configures SSL_CTX objects with default TLS 1.2 and 1.3 ciphersuites
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  // Restrict TLS 1.3 ciphersuite
  ASSERT_TRUE(SSL_CTX_set_ciphersuites(
      client_ctx.get(), "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384"));
  ASSERT_TRUE(SSL_CTX_set_ciphersuites(
      server_ctx.get(), "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  // Modify ciphersuites on the SSL object, this modifies ssl->config
  ASSERT_TRUE(SSL_set_ciphersuites(client.get(), "TLS_AES_256_GCM_SHA384"));
  ASSERT_TRUE(SSL_set_ciphersuites(server.get(), "TLS_AES_128_GCM_SHA256"));

  // Handshake should fail as config objects have no shared cipher.
  ASSERT_FALSE(CompleteHandshakes(client.get(), server.get()));
  ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_SHARED_CIPHER);

  bssl::UniquePtr<SSL> client2, server2;
  ASSERT_TRUE(CreateClientAndServer(&client2, &server2, client_ctx.get(),
                                    server_ctx.get()));

  // Modify ciphersuites on the SSL object, this modifies ssl->config
  ASSERT_TRUE(
      SSL_set_ciphersuites(client2.get(), "TLS_CHACHA20_POLY1305_SHA256"));
  ASSERT_TRUE(
      SSL_set_ciphersuites(server2.get(), "TLS_CHACHA20_POLY1305_SHA256"));

  ASSERT_TRUE(CompleteHandshakes(client2.get(), server2.get()));
  ASSERT_EQ(SSL_CIPHER_get_id(SSL_get_current_cipher(client2.get())),
            (uint32_t)TLS1_3_CK_CHACHA20_POLY1305_SHA256);
  ASSERT_EQ(SSL_CIPHER_get_id(SSL_get_current_cipher(server2.get())),
            (uint32_t)TLS1_3_CK_CHACHA20_POLY1305_SHA256);
}

TEST(SSLTest, TLS13ConfigCtxInteraction) {
  // This configures SSL_CTX objects with default TLS 1.2 and 1.3 ciphersuites
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  // Restrict TLS 1.3 ciphersuite on the SSL_CTX objects
  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(client_ctx.get(), "TLS_AES_128_GCM_SHA256"));
  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(server_ctx.get(), "TLS_AES_128_GCM_SHA256"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  // Modify TLS 1.3 ciphersuites for client's SSL object, but not server
  ASSERT_TRUE(SSL_set_ciphersuites(client.get(), "TLS_AES_256_GCM_SHA384"));

  // Handshake should fail as client SSL config and server CTX objects have no
  // shared TLS 1.3 cipher.
  ASSERT_FALSE(CompleteHandshakes(client.get(), server.get()));
  ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_SHARED_CIPHER);

  ERR_clear_error();

  bssl::UniquePtr<SSL> client2, server2;
  ASSERT_TRUE(CreateClientAndServer(&client2, &server2, client_ctx.get(),
                                    server_ctx.get()));

  // Modify TLS 1.3 ciphersuites for server2 SSL object, but not client
  ASSERT_TRUE(SSL_set_ciphersuites(server2.get(), "TLS_AES_256_GCM_SHA384"));

  // Handshake should fail as server SSL config and client CTX objects have no
  // shared TLS 1.3 cipher.
  ASSERT_FALSE(CompleteHandshakes(client2.get(), server2.get()));
  ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_SHARED_CIPHER);
}

TEST(SSLTest, TLS12ConfigCiphers) {
  // This configures SSL_CTX objects with default TLS 1.2 and 1.3 ciphersuites
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_2_VERSION));

  // Restrict TLS 1.2 ciphersuite
  ASSERT_TRUE(SSL_CTX_set_cipher_list(
      client_ctx.get(),
      "TLS_RSA_WITH_AES_256_CBC_SHA:TLS_RSA_WITH_AES_256_GCM_SHA384"));
  ASSERT_TRUE(SSL_CTX_set_cipher_list(
      server_ctx.get(),
      "TLS_RSA_WITH_AES_256_CBC_SHA:TLS_RSA_WITH_AES_256_GCM_SHA384"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  // Modify ciphersuites on the SSL object and introduce mismatch, this modifies
  // ssl->config
  ASSERT_TRUE(
      SSL_set_cipher_list(client.get(), "TLS_RSA_WITH_AES_256_CBC_SHA"));
  ASSERT_TRUE(
      SSL_set_cipher_list(server.get(), "TLS_RSA_WITH_AES_256_GCM_SHA384"));

  // Handshake should fail as config objects have no shared cipher.
  ASSERT_FALSE(CompleteHandshakes(client.get(), server.get()));
  ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_SHARED_CIPHER);

  ERR_clear_error();

  bssl::UniquePtr<SSL> client2, server2;
  ASSERT_TRUE(CreateClientAndServer(&client2, &server2, client_ctx.get(),
                                    server_ctx.get()));

  // Modify ciphersuites on the SSL object with a new third cipher, this
  // modifies ssl->config
  ASSERT_TRUE(
      SSL_set_cipher_list(client2.get(), "TLS_RSA_WITH_AES_128_CBC_SHA"));
  ASSERT_TRUE(
      SSL_set_cipher_list(server2.get(), "TLS_RSA_WITH_AES_128_CBC_SHA"));

  ASSERT_TRUE(CompleteHandshakes(client2.get(), server2.get()));
  ASSERT_EQ(SSL_CIPHER_get_id(SSL_get_current_cipher(client2.get())),
            (uint32_t)TLS1_CK_RSA_WITH_AES_128_SHA);
  ASSERT_EQ(SSL_CIPHER_get_id(SSL_get_current_cipher(server2.get())),
            (uint32_t)TLS1_CK_RSA_WITH_AES_128_SHA);
}

TEST(SSLTest, TLS12ConfigCtxInteraction) {
  // This configures SSL_CTX objects with default TLS 1.2 and 1.3 ciphersuites
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_2_VERSION));

  // Restrict TLS 1.2 ciphersuites on the SSL_CTX objects
  ASSERT_TRUE(SSL_CTX_set_cipher_list(client_ctx.get(),
                                      "TLS_RSA_WITH_AES_256_CBC_SHA"));
  ASSERT_TRUE(SSL_CTX_set_cipher_list(server_ctx.get(),
                                      "TLS_RSA_WITH_AES_256_CBC_SHA"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  // Modify TLS 1.2 ciphersuite for client's SSL object, but not server
  ASSERT_TRUE(
      SSL_set_cipher_list(client.get(), "TLS_RSA_WITH_AES_256_GCM_SHA384"));

  // Handshake should fail as client SSL config and server CTX objects have no
  // shared TLS 1.2 cipher.
  ASSERT_FALSE(CompleteHandshakes(client.get(), server.get()));
  ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_SHARED_CIPHER);

  ERR_clear_error();

  bssl::UniquePtr<SSL> client2, server2;
  ASSERT_TRUE(CreateClientAndServer(&client2, &server2, client_ctx.get(),
                                    server_ctx.get()));

  // Modify TLS 1.2 ciphersuites for server2 SSL object, but not client
  ASSERT_TRUE(
      SSL_set_cipher_list(server2.get(), "TLS_RSA_WITH_AES_256_GCM_SHA384"));

  // Handshake should fail as server SSL config and client CTX objects have no
  // shared TLS 1.2 cipher.
  ASSERT_FALSE(CompleteHandshakes(client2.get(), server2.get()));
  ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), SSL_R_NO_SHARED_CIPHER);
}

TEST(SSLTest, SSLGetCiphersReturnsTLS13Custom) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  // Configure custom TLS 1.3 Ciphersuites
  SSL_CTX_set_ciphersuites(server_ctx.get(), "TLS_AES_128_GCM_SHA256");
  SSL_CTX_set_ciphersuites(client_ctx.get(),
                           "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384");

  // Configure only TLS 1.3.
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  // Have to ensure config is not shed per current implementation of
  // SSL_get_ciphers.
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get(), ClientConfig(), false));

  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));
  // Ensure default TLS 1.3 Ciphersuites are present
  const SSL_CIPHER *cipher1 =
      SSL_get_cipher_by_value(TLS1_3_CK_AES_128_GCM_SHA256 & 0xFFFF);
  ASSERT_TRUE(cipher1);
  const SSL_CIPHER *cipher2 =
      SSL_get_cipher_by_value(TLS1_3_CK_AES_256_GCM_SHA384 & 0xFFFF);
  ASSERT_TRUE(cipher2);
  const SSL_CIPHER *cipher3 =
      SSL_get_cipher_by_value(TLS1_3_CK_CHACHA20_POLY1305_SHA256 & 0xFFFF);
  ASSERT_TRUE(cipher3);

  STACK_OF(SSL_CIPHER) *client_ciphers = SSL_get_ciphers(client.get());
  STACK_OF(SSL_CIPHER) *server_ciphers = SSL_get_ciphers(server.get());

  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, cipher1));
  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, cipher2));
  ASSERT_FALSE(sk_SSL_CIPHER_find_awslc(client_ciphers, NULL, cipher3));

  ASSERT_TRUE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, cipher1));
  ASSERT_FALSE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, cipher2));
  ASSERT_FALSE(sk_SSL_CIPHER_find_awslc(server_ciphers, NULL, cipher3));
}

TEST(SSLTest, GetClientCiphersAfterHandshakeFailure1_3) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());

  // configure client to add fake ciphersuite
  SSL_CTX_set_grease_enabled(client_ctx.get(), 1);

  // There will be no cipher match, handshake will not succeed.
  ASSERT_TRUE(SSL_CTX_set_ciphersuites(
      client_ctx.get(), "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256"));
  ASSERT_TRUE(
      SSL_CTX_set_ciphersuites(server_ctx.get(), "TLS_AES_128_GCM_SHA256"));

  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  const unsigned char *tmp = nullptr;
  // Handshake not completed, getting ciphers should fail
  ASSERT_FALSE(SSL_client_hello_get0_ciphers(client.get(), &tmp));
  ASSERT_FALSE(SSL_client_hello_get0_ciphers(server.get(), &tmp));
  ASSERT_FALSE(tmp);

  // This should fail, but should be able to inspect client ciphers still
  ASSERT_FALSE(CompleteHandshakes(client.get(), server.get()));

  ASSERT_EQ(SSL_client_hello_get0_ciphers(client.get(), nullptr), (size_t)0);

  const unsigned char expected_cipher_bytes[] = {0x13, 0x02, 0x13, 0x03};
  const unsigned char *p = nullptr;

  // Expected size is 2 bytes more than |expected_cipher_bytes| to account for
  // grease value
  ASSERT_EQ(SSL_client_hello_get0_ciphers(server.get(), &p),
            sizeof(expected_cipher_bytes) + 2);

  // Grab the first 2 bytes and check grease value
  uint16_t grease_val = CRYPTO_load_u16_be(p);
  ASSERT_FALSE(SSL_get_cipher_by_value(grease_val));

  // Sanity check for first cipher ID after grease value
  uint16_t cipher_val = CRYPTO_load_u16_be(p + 2);
  ASSERT_TRUE(SSL_get_cipher_by_value((cipher_val)));

  // Check order and validity of the rest of the client cipher suites,
  // excluding the grease value (2nd byte onwards)
  ASSERT_EQ(Bytes(expected_cipher_bytes, sizeof(expected_cipher_bytes)),
            Bytes(p + 2, sizeof(expected_cipher_bytes)));

  // Parsed ciphersuite list should only have 2 valid ciphersuites as configured
  // (grease value should not be included). Even though the handshake fails,
  // client cipher data should be available through the server SSL object.
  ASSERT_TRUE(sk_SSL_CIPHER_num(server.get()->client_cipher_suites.get()) == 2);
}

TEST(SSLTest, GetClientCiphers1_3) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());

  // configure client to add fake ciphersuite
  SSL_CTX_set_grease_enabled(client_ctx.get(), 1);

  ASSERT_TRUE(SSL_CTX_set_ciphersuites(client_ctx.get(),
                                       "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_"
                                       "SHA384:TLS_CHACHA20_POLY1305_SHA256"));

  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  const unsigned char *tmp = nullptr;
  // Handshake not completed, getting ciphers should fail
  ASSERT_FALSE(SSL_client_hello_get0_ciphers(client.get(), &tmp));
  ASSERT_FALSE(SSL_client_hello_get0_ciphers(server.get(), &tmp));
  ASSERT_FALSE(tmp);

  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  ASSERT_EQ(SSL_client_hello_get0_ciphers(client.get(), nullptr), (size_t)0);

  const unsigned char expected_cipher_bytes[] = {0x13, 0x01, 0x13,
                                                 0x02, 0x13, 0x03};
  const unsigned char *p = nullptr;

  // Expected size is 2 bytes more than |expected_cipher_bytes| to account for
  // grease value
  ASSERT_EQ(SSL_client_hello_get0_ciphers(server.get(), &p),
            sizeof(expected_cipher_bytes) + 2);

  // Grab the first 2 bytes and check grease value
  uint16_t grease_val = CRYPTO_load_u16_be(p);
  ASSERT_FALSE(SSL_get_cipher_by_value(grease_val));

  // Sanity check for first cipher ID after grease value
  uint16_t cipher_val = CRYPTO_load_u16_be(p + 2);
  ASSERT_TRUE(SSL_get_cipher_by_value((cipher_val)));

  // Check order and validity of the rest of the client cipher suites,
  // excluding the grease value (2nd byte onwards)
  ASSERT_EQ(Bytes(expected_cipher_bytes, sizeof(expected_cipher_bytes)),
            Bytes(p + 2, sizeof(expected_cipher_bytes)));

  // Parsed ciphersuite list should only have 3 valid ciphersuites as configured
  // (grease value should not be included)
  ASSERT_TRUE(sk_SSL_CIPHER_num(server.get()->client_cipher_suites.get()) == 3);
}

TEST(SSLTest, GetClientCiphers1_2) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());

  // configure client to add fake ciphersuite
  SSL_CTX_set_grease_enabled(client_ctx.get(), 1);

  ASSERT_TRUE(SSL_CTX_set_cipher_list(client_ctx.get(),
                                      "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384:"
                                      "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA"));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_2_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  const unsigned char *tmp = nullptr;
  // Handshake not completed, getting ciphers should fail
  ASSERT_FALSE(SSL_client_hello_get0_ciphers(client.get(), &tmp));
  ASSERT_FALSE(SSL_client_hello_get0_ciphers(server.get(), &tmp));
  ASSERT_FALSE(tmp);

  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  ASSERT_EQ(SSL_client_hello_get0_ciphers(client.get(), nullptr), (size_t)0);

  const unsigned char expected_cipher_bytes[] = {0xC0, 0x2C, 0xC0, 0x13};
  const unsigned char *p = nullptr;

  // Expected size is 2 bytes more than |expected_cipher_bytes| to account for
  // grease value
  ASSERT_EQ(SSL_client_hello_get0_ciphers(server.get(), &p),
            sizeof(expected_cipher_bytes) + 2);

  // Grab the first 2 bytes and check grease value
  uint16_t grease_val = CRYPTO_load_u16_be(p);
  ASSERT_FALSE(SSL_get_cipher_by_value(grease_val));

  // Sanity check for first cipher ID after grease value
  uint16_t cipher_val = CRYPTO_load_u16_be(p + 2);
  ASSERT_TRUE(SSL_get_cipher_by_value((cipher_val)));

  // Check order and validity of the rest of the client cipher suites,
  // excluding the grease value (2nd byte onwards)
  ASSERT_EQ(Bytes(expected_cipher_bytes, sizeof(expected_cipher_bytes)),
            Bytes(p + 2, sizeof(expected_cipher_bytes)));

  // Parsed ciphersuite list should only have 2 valid ciphersuites as configured
  // (grease value should not be included)
  ASSERT_TRUE(sk_SSL_CIPHER_num(server.get()->client_cipher_suites.get()) == 2);
}


TEST(SSLTest, DISABLED_ApplyHandoffRemovesUnsupportedCiphers) {
  bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL> server(SSL_new(server_ctx.get()));
  ASSERT_TRUE(server);

  // handoff is a handoff message that has been artificially modified to pretend
  // that only cipher 0x0A is supported.  When it is applied to |server|, all
  // ciphers but that one should be removed.
  //
  // To make a new one of these, try sticking this in the |Handoff| test above:
  //
  // hexdump(stderr, "", handoff.data(), handoff.size());
  // sed -e 's/\(..\)/0x\1, /g'
  //
  // and modify serialize_features() to emit only cipher 0x0A.

  uint8_t handoff[] = {
      0x30, 0x81, 0x9a, 0x02, 0x01, 0x00, 0x04, 0x00, 0x04, 0x81, 0x82, 0x01,
      0x00, 0x00, 0x7e, 0x03, 0x03, 0x30, 0x8e, 0x8f, 0x79, 0xd2, 0x87, 0x39,
      0xc2, 0x23, 0x23, 0x13, 0xca, 0x3c, 0x80, 0x44, 0xfd, 0x80, 0x83, 0x62,
      0x3c, 0xcc, 0xf8, 0x76, 0xd3, 0x62, 0xbb, 0x54, 0xe3, 0xc4, 0x39, 0x24,
      0xa5, 0x00, 0x00, 0x1e, 0xc0, 0x2b, 0xc0, 0x2f, 0xc0, 0x2c, 0xc0, 0x30,
      0xcc, 0xa9, 0xcc, 0xa8, 0xc0, 0x09, 0xc0, 0x13, 0xc0, 0x0a, 0xc0, 0x14,
      0x00, 0x9c, 0x00, 0x9d, 0x00, 0x2f, 0x00, 0x35, 0x00, 0x0a, 0x01, 0x00,
      0x00, 0x37, 0x00, 0x17, 0x00, 0x00, 0xff, 0x01, 0x00, 0x01, 0x00, 0x00,
      0x0a, 0x00, 0x08, 0x00, 0x06, 0x00, 0x1d, 0x00, 0x17, 0x00, 0x18, 0x00,
      0x0b, 0x00, 0x02, 0x01, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x0d, 0x00,
      0x14, 0x00, 0x12, 0x04, 0x03, 0x08, 0x04, 0x04, 0x01, 0x05, 0x03, 0x08,
      0x05, 0x05, 0x01, 0x08, 0x06, 0x06, 0x01, 0x02, 0x01, 0x04, 0x02, 0x00,
      0x0a, 0x04, 0x0a, 0x00, 0x15, 0x00, 0x17, 0x00, 0x18, 0x00, 0x19, 0x00,
      0x1d,
  };

  EXPECT_EQ(22u, sk_SSL_CIPHER_num(SSL_get_ciphers(server.get())));
  ASSERT_TRUE(
      SSL_apply_handoff(server.get(), {handoff, OPENSSL_ARRAY_SIZE(handoff)}));
  EXPECT_EQ(1u, sk_SSL_CIPHER_num(SSL_get_ciphers(server.get())));
}


TEST(SSLTest, ApplyHandoffRemovesUnsupportedCurves) {
  bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL> server(SSL_new(server_ctx.get()));
  ASSERT_TRUE(server);

  // handoff is a handoff message that has been artificially modified to pretend
  // that only one ECDH group is supported.  When it is applied to |server|, all
  // groups but that one should be removed.
  //
  // See |ApplyHandoffRemovesUnsupportedCiphers| for how to make a new one of
  // these.
  uint8_t handoff[] = {
      0x30, 0x81, 0xc0, 0x02, 0x01, 0x00, 0x04, 0x00, 0x04, 0x81, 0x82, 0x01,
      0x00, 0x00, 0x7e, 0x03, 0x03, 0x98, 0x30, 0xce, 0xd9, 0xb0, 0xdf, 0x5f,
      0x82, 0x05, 0x4a, 0x43, 0x67, 0x7e, 0xdb, 0x6a, 0x4f, 0x21, 0x18, 0x4e,
      0x0d, 0x94, 0x63, 0x18, 0x8b, 0x54, 0x89, 0xdb, 0x8b, 0x1d, 0x84, 0xbc,
      0x09, 0x00, 0x00, 0x1e, 0xc0, 0x2b, 0xc0, 0x2f, 0xc0, 0x2c, 0xc0, 0x30,
      0xcc, 0xa9, 0xcc, 0xa8, 0xc0, 0x09, 0xc0, 0x13, 0xc0, 0x0a, 0xc0, 0x14,
      0x00, 0x9c, 0x00, 0x9d, 0x00, 0x2f, 0x00, 0x35, 0x00, 0x0a, 0x01, 0x00,
      0x00, 0x37, 0x00, 0x17, 0x00, 0x00, 0xff, 0x01, 0x00, 0x01, 0x00, 0x00,
      0x0a, 0x00, 0x08, 0x00, 0x06, 0x00, 0x1d, 0x00, 0x17, 0x00, 0x18, 0x00,
      0x0b, 0x00, 0x02, 0x01, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x0d, 0x00,
      0x14, 0x00, 0x12, 0x04, 0x03, 0x08, 0x04, 0x04, 0x01, 0x05, 0x03, 0x08,
      0x05, 0x05, 0x01, 0x08, 0x06, 0x06, 0x01, 0x02, 0x01, 0x04, 0x30, 0x00,
      0x02, 0x00, 0x0a, 0x00, 0x2f, 0x00, 0x35, 0x00, 0x8c, 0x00, 0x8d, 0x00,
      0x9c, 0x00, 0x9d, 0x13, 0x01, 0x13, 0x02, 0x13, 0x03, 0xc0, 0x09, 0xc0,
      0x0a, 0xc0, 0x13, 0xc0, 0x14, 0xc0, 0x2b, 0xc0, 0x2c, 0xc0, 0x2f, 0xc0,
      0x30, 0xc0, 0x35, 0xc0, 0x36, 0xcc, 0xa8, 0xcc, 0xa9, 0xcc, 0xac, 0x04,
      0x02, 0x00, 0x17,
  };

  // The zero length means that the default list of groups is used.
  EXPECT_EQ(0u, server->config->supported_group_list.size());
  ASSERT_TRUE(
      SSL_apply_handoff(server.get(), {handoff, OPENSSL_ARRAY_SIZE(handoff)}));
  EXPECT_EQ(1u, server->config->supported_group_list.size());
}

BSSL_NAMESPACE_END
