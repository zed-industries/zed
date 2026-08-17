// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/ssl.h>
#include "../crypto/test/test_util.h"
#include "ssl_common_test.h"

BSSL_NAMESPACE_BEGIN

struct HybridHandshakeTest {
  // The curves rule string to apply to the client
  const char *client_rule;
  // TLS version that the client is configured with
  uint16_t client_version;
  // The curves rule string to apply to the server
  const char *server_rule;
  // TLS version that the server is configured with
  uint16_t server_version;
  // The group that is expected to be negotiated
  uint16_t expected_group;
  // Is a HelloRetryRequest expected?
  bool is_hrr_expected;
};


static const HybridHandshakeTest kHybridHandshakeTests[] = {
    // The corresponding hybrid group should be negotiated when client
    // and server support only that group
    {
        "X25519MLKEM768",
        TLS1_3_VERSION,
        "X25519MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        false,
    },

    {
        "SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "SecP256r1MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1_MLKEM768,
        false,
    },

    {
        "SecP384r1MLKEM1024",
        TLS1_3_VERSION,
        "SecP384r1MLKEM1024",
        TLS1_3_VERSION,
        SSL_GROUP_SECP384R1_MLKEM1024,
        false,
    },

    // Pure MLKEM at all key sizes
    {
      "MLKEM512",
      TLS1_3_VERSION,
      "MLKEM512",
      TLS1_3_VERSION,
      SSL_GROUP_MLKEM512,
      false,
    },

    {
      "MLKEM768",
      TLS1_3_VERSION,
      "MLKEM768",
      TLS1_3_VERSION,
      SSL_GROUP_MLKEM768,
      false,
    },

    {
      "MLKEM1024",
      TLS1_3_VERSION,
      "MLKEM1024",
      TLS1_3_VERSION,
      SSL_GROUP_MLKEM1024,
      false,
    },

    // The client's preferred hybrid group should be negotiated when also
    // supported by the server, even if the server "prefers"/supports other
    // groups.
    {
        "X25519MLKEM768:x25519",
        TLS1_3_VERSION,
        "x25519:prime256v1:X25519MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        false,
    },

    {
        "X25519MLKEM768:x25519",
        TLS1_3_VERSION,
        "X25519MLKEM768:x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        false,
    },

    {
        "SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "X25519MLKEM768:secp384r1:x25519:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1_MLKEM768,
        false,
    },

    {
        "X25519MLKEM768:SecP256r1MLKEM768:SecP384r1MLKEM1024",
        TLS1_3_VERSION,
        "SecP384r1MLKEM1024:SecP256r1MLKEM768:X25519MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        false,
    },

    {
        "SecP384r1MLKEM1024:SecP256r1MLKEM768:X25519MLKEM768",
        TLS1_3_VERSION,
        "X25519MLKEM768:SecP256r1MLKEM768:SecP384r1MLKEM1024",
        TLS1_3_VERSION,
        SSL_GROUP_SECP384R1_MLKEM1024,
        false,
    },

    // The client lists PQ/hybrid groups as both first and second preferences.
    // The key share logic is implemented such that the client will always
    // attempt to send one hybrid key share and one classical key share.
    // Therefore, the client will send key shares [SecP256r1MLKEM768, x25519],
    // skipping X25519MLKEM768, and the server will choose to negotiate
    // x25519 since it is the only mutually supported group.
    {
        "SecP256r1MLKEM768:X25519MLKEM768:x25519",
        TLS1_3_VERSION,
        "secp384r1:x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519,
        false,
    },

    // The client will send key shares [x25519, SecP256r1MLKEM768].
    // The server will negotiate SecP256r1MLKEM768 since it is the only
    // mutually supported group.
    {
        "x25519:secp384r1:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "SecP256r1MLKEM768:prime256v1",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1_MLKEM768,
        false,
    },

    // The client will send key shares [x25519, SecP256r1MLKEM768]. The
    // server will negotiate x25519 since the client listed it as its first
    // preference, even though it supports SecP256r1MLKEM768.
    {
        "x25519:prime256v1:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "prime256v1:x25519:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_X25519,
        false,
    },

    // The client will send key shares [SecP256r1MLKEM768, x25519].
    // The server will negotiate SecP256r1MLKEM768 since the client listed
    // it as its first preference.
    {
        "SecP256r1MLKEM768:x25519:prime256v1",
        TLS1_3_VERSION,
        "prime256v1:x25519:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1_MLKEM768,
        false,
    },

    // In the supported_groups extension, the client will indicate its
    // preferences, in order, as [SecP256r1MLKEM768, X25519MLKEM768,
    // x25519, prime256v1]. From those groups, it will send key shares
    // [SecP256r1MLKEM768, x25519]. The server supports, and receives a
    // key share for, x25519. However, when selecting a mutually supported group
    // to negotiate, the server recognizes that the client prefers
    // X25519MLKEM768 over x25519. Since the server also supports
    // X25519MLKEM768, but did not receive a key share for it, it will
    // select it and send an HRR. This ensures that the client's highest
    // preference group will be negotiated, even at the expense of an additional
    // round-trip.
    //
    // In our SSL implementation, this situation is unique to the case where the
    // client supports both ECC and hybrid/PQ. When sending key shares, the
    // client will send at most two key shares in one of the following ways:

    // (a) one ECC key share - if the client supports only ECC;
    // (b) one PQ key share - if the client supports only PQ;
    // (c) one ECC and one PQ key share - if the client supports ECC and PQ.
    //
    // One of the above cases will be true irrespective of how many groups
    // the client supports. If, say, the client supports four ECC groups
    // and zero PQ groups, it will still only send a single ECC share. In cases
    // (a) and (b), either the server supports that group and chooses to
    // negotiate it, or it doesn't support it and sends an HRR. Case (c) is the
    // only case where the server might receive a key share for a mutually
    // supported group, but chooses to respect the client's preference order
    // defined in the supported_groups extension at the expense of an additional
    // round-trip.
    {
        "SecP256r1MLKEM768:X25519MLKEM768:x25519:prime256v1",
        TLS1_3_VERSION,
        "X25519MLKEM768:prime256v1:x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        true,
    },

    // Like the previous case, but the client's prioritization of ECC and PQ
    // is inverted.
    {
        "x25519:prime256v1:SecP256r1MLKEM768:X25519MLKEM768",
        TLS1_3_VERSION,
        "X25519MLKEM768:prime256v1",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1,
        true,
    },

    // The client will send key shares [SecP256r1MLKEM768, x25519]. The
    // server will negotiate X25519MLKEM768 after an HRR.
    {
        "SecP256r1MLKEM768:X25519MLKEM768:x25519:prime256v1",
        TLS1_3_VERSION,
        "X25519MLKEM768:prime256v1",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        true,
    },

    // EC should be negotiated when client prefers EC, or server does not
    // support hybrid
    {
        "X25519MLKEM768:x25519",
        TLS1_3_VERSION,
        "x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519,
        false,
    },
    {
        "x25519:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519,
        false,
    },
    {
        "prime256v1:X25519MLKEM768",
        TLS1_3_VERSION,
        "X25519MLKEM768:prime256v1",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1,
        false,
    },
    {
        "prime256v1:x25519:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "x25519:prime256v1:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1,
        false,
    },

    // EC should be negotiated, after a HelloRetryRequest, if the server
    // supports only curves for which it did not initially receive a key share
    {
        "X25519MLKEM768:x25519:SecP256r1MLKEM768:prime256v1",
        TLS1_3_VERSION,
        "prime256v1",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1,
        true,
    },
    {
        "X25519MLKEM768:SecP256r1MLKEM768:prime256v1:x25519",
        TLS1_3_VERSION,
        "secp224r1:secp384r1:secp521r1:x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519,
        true,
    },

    // Hybrid should be negotiated, after a HelloRetryRequest, if the server
    // supports only curves for which it did not initially receive a key share
    {
        "x25519:prime256v1:SecP256r1MLKEM768:X25519MLKEM768",
        TLS1_3_VERSION,
        "secp224r1:X25519MLKEM768:secp521r1",
        TLS1_3_VERSION,
        SSL_GROUP_X25519_MLKEM768,
        true,
    },
    {
        "X25519MLKEM768:x25519:prime256v1:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "SecP256r1MLKEM768",
        TLS1_3_VERSION,
        SSL_GROUP_SECP256R1_MLKEM768,
        true,
    },

    // If there is no overlap between client and server groups,
    // the handshake should fail
    {
        "SecP256r1MLKEM768:X25519MLKEM768:secp384r1",
        TLS1_3_VERSION,
        "prime256v1:x25519",
        TLS1_3_VERSION,
        0,
        false,
    },
    {
        "secp384r1:SecP256r1MLKEM768:X25519MLKEM768",
        TLS1_3_VERSION,
        "prime256v1:x25519",
        TLS1_3_VERSION,
        0,
        false,
    },
    {
        "secp384r1:SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "prime256v1:x25519:X25519MLKEM768",
        TLS1_3_VERSION,
        0,
        false,
    },
    {
        "SecP256r1MLKEM768",
        TLS1_3_VERSION,
        "X25519MLKEM768",
        TLS1_3_VERSION,
        0,
        false,
    },

    // If the client supports hybrid TLS 1.3, but the server
    // only supports TLS 1.2, then TLS 1.2 EC should be negotiated.
    {
        "SecP256r1MLKEM768:prime256v1",
        TLS1_3_VERSION,
        "prime256v1:x25519",
        TLS1_2_VERSION,
        SSL_GROUP_SECP256R1,
        false,
    },

    // Same as above, but server also has SecP256r1MLKEM768 in it's
    // supported list, but can't use it since TLS 1.3 is the minimum version
    // that
    // supports PQ.
    {
        "SecP256r1MLKEM768:prime256v1",
        TLS1_3_VERSION,
        "SecP256r1MLKEM768:prime256v1:x25519",
        TLS1_2_VERSION,
        SSL_GROUP_SECP256R1,
        false,
    },

    // If the client configures the curve list to include a hybrid
    // curve, then initiates a 1.2 handshake, it will not advertise
    // hybrid groups because hybrid is not supported for 1.2. So
    // a 1.2 EC handshake will be negotiated (even if the server
    // supports 1.3 with corresponding hybrid group).
    {
        "SecP256r1MLKEM768:x25519",
        TLS1_2_VERSION,
        "SecP256r1MLKEM768:x25519",
        TLS1_3_VERSION,
        SSL_GROUP_X25519,
        false,
    },
    {
        "SecP256r1MLKEM768:prime256v1",
        TLS1_2_VERSION,
        "prime256v1:x25519",
        TLS1_2_VERSION,
        SSL_GROUP_SECP256R1,
        false,
    },
};

class PerformHybridHandshakeTest
    : public testing::TestWithParam<HybridHandshakeTest> {};
INSTANTIATE_TEST_SUITE_P(PerformHybridHandshakeTests,
                         PerformHybridHandshakeTest,
                         testing::ValuesIn(kHybridHandshakeTests));

// This test runs through an overall handshake flow for all of the cases
// defined in kHybridHandshakeTests. This test runs through both positive and
// negative cases; refer to the comments inline in kHybridHandshakeTests for
// specifics about each test case.
TEST_P(PerformHybridHandshakeTest, PerformHybridHandshake) {
  HybridHandshakeTest t = GetParam();
  // Set up client and server with test case parameters
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(SSL_CTX_set1_curves_list(client_ctx.get(), t.client_rule));
  ASSERT_TRUE(
      SSL_CTX_set_max_proto_version(client_ctx.get(), t.client_version));

  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(server_ctx);
  ASSERT_TRUE(SSL_CTX_set1_curves_list(server_ctx.get(), t.server_rule));
  ASSERT_TRUE(
      SSL_CTX_set_max_proto_version(server_ctx.get(), t.server_version));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  if (t.expected_group != 0) {
    // In this case, assert that the handshake completes as expected.
    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

    SSL_SESSION *client_session = SSL_get_session(client.get());
    ASSERT_TRUE(client_session);
    EXPECT_EQ(t.expected_group, client_session->group_id);
    EXPECT_EQ(t.is_hrr_expected, SSL_used_hello_retry_request(client.get()));
    EXPECT_EQ(std::min(t.client_version, t.server_version),
              client_session->ssl_version);

    SSL_SESSION *server_session = SSL_get_session(server.get());
    ASSERT_TRUE(server_session);
    EXPECT_EQ(t.expected_group, server_session->group_id);
    EXPECT_EQ(t.is_hrr_expected, SSL_used_hello_retry_request(server.get()));
    EXPECT_EQ(std::min(t.client_version, t.server_version),
              server_session->ssl_version);
  } else {
    // In this case, we expect the handshake to fail because client and
    // server configurations are not compatible.
    ASSERT_FALSE(CompleteHandshakes(client.get(), server.get()));

    ASSERT_FALSE(client.get()->s3->initial_handshake_complete);
    EXPECT_EQ(t.is_hrr_expected, SSL_used_hello_retry_request(client.get()));

    ASSERT_FALSE(server.get()->s3->initial_handshake_complete);
    EXPECT_EQ(t.is_hrr_expected, SSL_used_hello_retry_request(server.get()));
  }
}

BSSL_NAMESPACE_END
