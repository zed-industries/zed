// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <limits.h>
#include <stdio.h>
#include <string.h>

#include <algorithm>
#include <array>
#include <string>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/aead.h>
#include <openssl/base64.h>
#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/hpke.h>
#include <openssl/pem.h>
#include <openssl/sha.h>
#include <openssl/ssl.h>
#include <openssl/x509.h>

#include "../crypto/fipsmodule/ec/internal.h"
#include "../crypto/fipsmodule/ml_kem/ml_kem.h"
#include "../crypto/internal.h"
#include "../crypto/test/test_util.h"
#include "internal.h"

BSSL_NAMESPACE_BEGIN

namespace {

// SECP256R1:     https://datatracker.ietf.org/doc/html/rfc8446#section-4.2.8.2
// X25519:        https://datatracker.ietf.org/doc/html/rfc8446#section-4.2.8.2
const size_t P256_KEYSHARE_SIZE = ((EC_P256R1_FIELD_ELEM_BYTES * 2) + 1);
const size_t P256_SECRET_SIZE = EC_P256R1_FIELD_ELEM_BYTES;
const size_t P384_KEYSHARE_SIZE = ((EC_P384R1_FIELD_ELEM_BYTES * 2) + 1);
const size_t P384_SECRET_SIZE = EC_P384R1_FIELD_ELEM_BYTES;
const size_t X25519_KEYSHARE_SIZE = 32;
const size_t X25519_SECRET_SIZE = 32;

struct GroupTest {
  int nid;
  uint16_t group_id;
  size_t offer_key_share_size;
  size_t accept_key_share_size;
  size_t shared_secret_size;
};

struct HybridGroupTest {
  int nid;
  uint16_t group_id;
  size_t offer_key_share_size;
  size_t accept_key_share_size;
  size_t shared_secret_size;
  size_t offer_share_sizes[NUM_HYBRID_COMPONENTS];
  size_t accept_share_sizes[NUM_HYBRID_COMPONENTS];
};

const HybridGroupTest kHybridGroupTests[] = {
    {
        NID_SecP256r1MLKEM768,
        SSL_GROUP_SECP256R1_MLKEM768,
        P256_KEYSHARE_SIZE + MLKEM768_PUBLIC_KEY_BYTES,
        P256_KEYSHARE_SIZE + MLKEM768_CIPHERTEXT_BYTES,
        P256_SECRET_SIZE + MLKEM768_SHARED_SECRET_LEN,
        {
            P256_KEYSHARE_SIZE,         // offer_share_sizes[0]
            MLKEM768_PUBLIC_KEY_BYTES,  // offer_share_sizes[1]
        },
        {
            P256_KEYSHARE_SIZE,         // accept_share_sizes[0]
            MLKEM768_CIPHERTEXT_BYTES,  // accept_share_sizes[1]
        },
    },
    {
        NID_X25519MLKEM768,
        SSL_GROUP_X25519_MLKEM768,
        X25519_KEYSHARE_SIZE + MLKEM768_PUBLIC_KEY_BYTES,
        X25519_KEYSHARE_SIZE + MLKEM768_CIPHERTEXT_BYTES,
        X25519_SECRET_SIZE + MLKEM768_SHARED_SECRET_LEN,
        {
            // MLKEM768 is sent first for X25519MLKEM768 for FIPS compliance
            // See:
            // https://datatracker.ietf.org/doc/html/draft-kwiatkowski-tls-ecdhe-mlkem.html#section-3
            MLKEM768_PUBLIC_KEY_BYTES,  // offer_share_sizes[0]
            X25519_KEYSHARE_SIZE,       // offer_share_sizes[1]
        },
        {
            // MLKEM768 is sent first for X25519MLKEM768 for FIPS compliance
            // See:
            // https://datatracker.ietf.org/doc/html/draft-kwiatkowski-tls-ecdhe-mlkem.html#section-3
            MLKEM768_CIPHERTEXT_BYTES,  // accept_share_sizes[0]
            X25519_KEYSHARE_SIZE,       // accept_share_sizes[1]
        },
    },
    {
        NID_SecP384r1MLKEM1024,
        SSL_GROUP_SECP384R1_MLKEM1024,
        P384_KEYSHARE_SIZE + MLKEM1024_PUBLIC_KEY_BYTES,
        P384_KEYSHARE_SIZE + MLKEM1024_CIPHERTEXT_BYTES,
        P384_SECRET_SIZE + MLKEM1024_SHARED_SECRET_LEN,
        {
            P384_KEYSHARE_SIZE,          // offer_share_sizes[0]
            MLKEM1024_PUBLIC_KEY_BYTES,  // offer_share_sizes[1]
        },
        {
            P384_KEYSHARE_SIZE,          // accept_share_sizes[0]
            MLKEM1024_CIPHERTEXT_BYTES,  // accept_share_sizes[1]
        },
    },
};

const GroupTest kKemGroupTests[] = {
    {
        NID_MLKEM512,
        SSL_GROUP_MLKEM512,
        MLKEM512_PUBLIC_KEY_BYTES,
        MLKEM512_CIPHERTEXT_BYTES,
        MLKEM512_SHARED_SECRET_LEN,
    },
    {
        NID_MLKEM768,
        SSL_GROUP_MLKEM768,
        MLKEM768_PUBLIC_KEY_BYTES,
        MLKEM768_CIPHERTEXT_BYTES,
        MLKEM768_SHARED_SECRET_LEN,
    },
    {
        NID_MLKEM1024,
        SSL_GROUP_MLKEM1024,
        MLKEM1024_PUBLIC_KEY_BYTES,
        MLKEM1024_CIPHERTEXT_BYTES,
        MLKEM1024_SHARED_SECRET_LEN,
    },
};

class BadKemKeyShareOfferTest : public testing::TestWithParam<GroupTest> {};
INSTANTIATE_TEST_SUITE_P(BadKemKeyShareOfferTests, BadKemKeyShareOfferTest,
                         testing::ValuesIn(kKemGroupTests));

// Test failure cases for KEMKeyShare::Offer()
TEST_P(BadKemKeyShareOfferTest, BadKemKeyShareOffers) {
  GroupTest t = GetParam();
  // Basic nullptr checks
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);

    ASSERT_FALSE(client_key_share->Offer(nullptr));
  }

  // Offer() should fail if |client_out_public_key| has children
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;
    CBB child;

    EXPECT_TRUE(CBB_init(&client_out_public_key, 2));
    client_out_public_key.child = &child;
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    CBB_cleanup(&client_out_public_key);
  }

  // Offer() should succeed on the first call, but fail on all repeated calls
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;

    EXPECT_TRUE(CBB_init(&client_out_public_key, 2));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    CBB_cleanup(&client_out_public_key);
  }

  // Offer() should fail if Accept() was previously called
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    ASSERT_TRUE(server_key_share);
    uint8_t server_alert = 0;
    Array<uint8_t> server_secret;
    CBB client_out_public_key;
    CBB server_out_public_key;
    CBB server_offer_out;

    EXPECT_TRUE(CBB_init(&client_out_public_key, t.offer_key_share_size));
    EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
    EXPECT_TRUE(CBB_init(&server_offer_out, t.offer_key_share_size));

    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    const uint8_t *client_public_key_data = CBB_data(&client_out_public_key);
    Span<const uint8_t> client_public_key =
        MakeConstSpan(client_public_key_data, CBB_len(&client_out_public_key));

    EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                         &server_alert, client_public_key));
    EXPECT_EQ(server_alert, 0);

    EXPECT_FALSE(server_key_share->Offer(&server_offer_out));

    CBB_cleanup(&client_out_public_key);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&server_offer_out);
  }

  // |client_out_public_key| is properly initialized, some zeros are written
  // to it so that it records a non-zero length, then its buffer is
  // invalidated.
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;
    CBB_init(&client_out_public_key, t.offer_key_share_size);
    EXPECT_TRUE(CBB_add_zeros(&client_out_public_key, 2));
    // Keep a pointer to the buffer so we can cleanup correctly
    uint8_t *buf = client_out_public_key.u.base.buf;
    client_out_public_key.u.base.buf = nullptr;
    EXPECT_EQ(CBB_len(&client_out_public_key), (size_t)2);
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    client_out_public_key.u.base.buf = buf;
    CBB_cleanup(&client_out_public_key);
  }
}


class BadKemKeyShareAcceptTest : public testing::TestWithParam<GroupTest> {};
INSTANTIATE_TEST_SUITE_P(BadKemKeyShareAcceptTests, BadKemKeyShareAcceptTest,
                         testing::ValuesIn(kKemGroupTests));

// Test failure cases for KEMKeyShare::Accept()
TEST_P(BadKemKeyShareAcceptTest, BadKemKeyShareAccept) {
  GroupTest t = GetParam();
  // Basic nullptr checks
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    uint8_t server_alert = 0;
    Array<uint8_t> server_secret;
    Span<const uint8_t> client_public_key;
    CBB server_out_public_key;

    EXPECT_FALSE(server_key_share->Accept(nullptr, &server_secret,
                                          &server_alert, client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    server_alert = 0;

    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key, nullptr,
                                          &server_alert, client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    server_alert = 0;

    EXPECT_FALSE(server_key_share->Accept(
        &server_out_public_key, &server_secret, nullptr, client_public_key));
  }

  // |server_out_public_key| is properly initialized, then is assigned a child
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    uint8_t server_alert = 0;
    Array<uint8_t> server_secret;
    Span<const uint8_t> client_public_key;
    CBB server_out_public_key;
    CBB child;

    CBB_init(&server_out_public_key, t.accept_key_share_size);
    server_out_public_key.child = &child;
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    CBB_cleanup(&server_out_public_key);
  }

  // |server_out_public_key| is properly initialized with CBB_init,
  // some zeros are written to it so that it records a non-zero length,
  // then its buffer is invalidated.
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    uint8_t server_alert = 0;
    Array<uint8_t> server_secret;
    Span<const uint8_t> client_public_key;
    CBB server_out_public_key;

    CBB_init(&server_out_public_key, t.accept_key_share_size);
    EXPECT_TRUE(CBB_add_zeros(&server_out_public_key, 2));
    // Keep a pointer to the buffer so we can cleanup correctly
    uint8_t *buf = server_out_public_key.u.base.buf;
    server_out_public_key.u.base.buf = nullptr;
    EXPECT_EQ(CBB_len(&server_out_public_key), (size_t)2);
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    server_out_public_key.u.base.buf = buf;
    CBB_cleanup(&server_out_public_key);
  }

  // KemKeyShare::Accept() should fail if KemKeyShare::Offer() has been
  // previously called by that peer. The server should have no reason to
  // call Offer(); enforcing this case will guard against that type of bug.
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    uint8_t server_alert = 0;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    CBB server_offer_out;

    EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
    EXPECT_TRUE(CBB_init(&server_offer_out, t.offer_key_share_size));
    EXPECT_TRUE(server_key_share->Offer(&server_offer_out));
    const uint8_t *server_offer_out_data = CBB_data(&server_offer_out);
    ASSERT_TRUE(server_offer_out_data);
    Span<const uint8_t> server_offered_pk =
        MakeConstSpan(server_offer_out_data, CBB_len(&server_offer_out));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          server_offered_pk));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&server_offer_out);
  }

  // |client_public_key| is initialized with too little data
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    uint8_t server_alert = 0;

    // Generate a valid |client_public_key|, then truncate the last byte
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    const uint8_t *client_out_public_key_data =
        CBB_data(&client_out_public_key);
    ASSERT_TRUE(client_out_public_key_data);
    client_public_key = MakeConstSpan(client_out_public_key_data,
                                      CBB_len(&client_out_public_key) - 1);

    EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_ILLEGAL_PARAMETER);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
  }

  // |client_public_key| is initialized with too much data
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    uint8_t server_alert = 0;

    // Generate a valid |client_public_key|, then append a byte
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    EXPECT_TRUE(CBB_add_zeros(&client_out_public_key, 1));
    const uint8_t *client_out_public_key_data =
        CBB_data(&client_out_public_key);
    ASSERT_TRUE(client_out_public_key_data);
    client_public_key = MakeConstSpan(client_out_public_key_data,
                                      CBB_len(&client_out_public_key));

    EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_ILLEGAL_PARAMETER);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
  }

  // |client_public_key| has been initialized but is empty
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    uint8_t server_alert = 0;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;

    EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
    const uint8_t empty_client_public_key_buf[] = {0};
    Span<const uint8_t> client_public_key =
        MakeConstSpan(empty_client_public_key_buf, 0);
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_DECODE_ERROR);
    CBB_cleanup(&server_out_public_key);
  }

  // |client_public_key| is initialized with key material that is the correct
  // length, but it doesn't match the corresponding secret key. The exchange
  // will succeed, but the client and the server will end up with different
  // secrets, and the overall handshake will eventually fail.
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> random_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    ASSERT_TRUE(random_key_share);
    uint8_t server_alert = 0;
    uint8_t client_alert = 0;
    Array<uint8_t> server_secret;
    Array<uint8_t> client_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    CBB random_out_public_key;

    // Start by having the client Offer() its public key
    EXPECT_TRUE(CBB_init(&client_out_public_key, t.offer_key_share_size));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));

    // Generate a random public key that is incompatible with client's secret
    // key
    EXPECT_TRUE(CBB_init(&random_out_public_key, t.offer_key_share_size));
    EXPECT_TRUE(random_key_share->Offer(&random_out_public_key));
    const uint8_t *random_out_public_key_data =
        CBB_data(&random_out_public_key);
    ASSERT_TRUE(random_out_public_key_data);
    Span<const uint8_t> client_public_key =
        MakeConstSpan(random_out_public_key_data, t.offer_key_share_size);

    // When the server calls Accept() with the modified public key, it will
    // return success
    EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
    EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                         &server_alert, client_public_key));

    // And when the client calls Finish(), it will also return success
    const uint8_t *server_out_public_key_data =
        CBB_data(&server_out_public_key);
    ASSERT_TRUE(server_out_public_key_data);
    Span<const uint8_t> server_public_key = MakeConstSpan(
        server_out_public_key_data, CBB_len(&server_out_public_key));
    EXPECT_TRUE(client_key_share->Finish(&client_secret, &client_alert,
                                         server_public_key));

    // The shared secrets are of the correct length...
    EXPECT_EQ(client_secret.size(), t.shared_secret_size);
    EXPECT_EQ(server_secret.size(), t.shared_secret_size);

    // ... but they are not equal
    EXPECT_NE(Bytes(client_secret), Bytes(server_secret));

    EXPECT_EQ(server_alert, 0);
    EXPECT_EQ(client_alert, 0);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
    CBB_cleanup(&random_out_public_key);
  }
}

class BadKemKeyShareFinishTest : public testing::TestWithParam<GroupTest> {};
INSTANTIATE_TEST_SUITE_P(BadKemKeyShareFinishTests, BadKemKeyShareFinishTest,
                         testing::ValuesIn(kKemGroupTests));

TEST_P(BadKemKeyShareFinishTest, BadKemKeyShareFinish) {
  GroupTest t = GetParam();

  // Basic nullptr checks
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> server_public_key;
    Array<uint8_t> client_secret;
    uint8_t client_alert = 0;

    EXPECT_FALSE(
        client_key_share->Finish(nullptr, &client_alert, server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);
    client_alert = 0;

    EXPECT_FALSE(
        client_key_share->Finish(&client_secret, nullptr, server_public_key));
  }

  // A call to Finish() should fail if Offer() was not called previously
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> server_public_key;
    Array<uint8_t> client_secret;
    uint8_t client_alert = 0;

    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);
  }

  // Set up the client and server states for the remaining tests
  bssl::UniquePtr<SSLKeyShare> server_key_share =
      bssl::SSLKeyShare::Create(t.group_id);
  bssl::UniquePtr<SSLKeyShare> client_key_share =
      bssl::SSLKeyShare::Create(t.group_id);
  bssl::UniquePtr<SSLKeyShare> random_key_share =
      bssl::SSLKeyShare::Create(t.group_id);
  ASSERT_TRUE(server_key_share);
  ASSERT_TRUE(client_key_share);
  ASSERT_TRUE(random_key_share);
  CBB client_out_public_key;
  CBB server_out_public_key;
  CBB random_out_public_key;
  Array<uint8_t> server_secret;
  Array<uint8_t> client_secret;
  uint8_t client_alert = 0;
  uint8_t server_alert = 0;
  Span<const uint8_t> client_public_key;
  Span<const uint8_t> server_public_key;

  EXPECT_TRUE(CBB_init(&client_out_public_key, t.offer_key_share_size));
  EXPECT_TRUE(CBB_init(&server_out_public_key, t.accept_key_share_size));
  EXPECT_TRUE(CBB_init(&random_out_public_key, t.accept_key_share_size));
  EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
  const uint8_t *client_out_public_key_data = CBB_data(&client_out_public_key);
  ASSERT_TRUE(client_out_public_key_data);
  client_public_key = MakeConstSpan(client_out_public_key_data,
                                    CBB_len(&client_out_public_key));
  EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                       &server_alert, client_public_key));
  EXPECT_EQ(server_alert, 0);

  // |server_public_key| has been initialized with too little data. Here, we
  // initialize |server_public_key| with a fragment of an otherwise valid
  // key. However, it doesn't matter if it is a fragment of a valid key, or
  // complete garbage, the client will reject it all the same.
  {
    const uint8_t *server_out_public_key_data =
        CBB_data(&server_out_public_key);
    ASSERT_TRUE(server_out_public_key_data);
    server_public_key =
        MakeConstSpan(server_out_public_key_data, t.accept_key_share_size - 1);
    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);
    client_alert = 0;
  }

  // |server_public_key| has been initialized with too much data. Here, we
  // initialize |server_public_key| with a valid public key, and over-read
  // the buffer to append a random byte. However, it doesn't matter if it is a
  // valid key with nonsense appended, or complete garbage, the client will
  // reject it all the same.
  {
    const uint8_t *server_out_public_key_data =
        CBB_data(&server_out_public_key);
    ASSERT_TRUE(server_out_public_key_data);
    server_public_key =
        MakeConstSpan(server_out_public_key_data, t.accept_key_share_size + 1);
    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);
    client_alert = 0;
  }

  // |server_public_key| is initialized with a modified key of the correct
  // length. The decapsulation operations will succeed; however, the resulting
  // shared secret will be garbage, and eventually the overall handshake
  // would fail because the client secret does not match the server secret.
  {
    // The server's public key was already correctly generated previously in
    // a call to Accept(). Here we modify it by replacing it with a randomly
    // generated public key that is incompatible with the secret key
    EXPECT_TRUE(random_key_share->Offer(&random_out_public_key));
    const uint8_t *random_out_public_key_data =
        CBB_data(&random_out_public_key);
    ASSERT_TRUE(random_out_public_key_data);
    server_public_key =
        MakeConstSpan(random_out_public_key_data, t.accept_key_share_size);

    // The call to Finish() will return success
    EXPECT_TRUE(client_key_share->Finish(&client_secret, &client_alert,
                                         server_public_key));
    EXPECT_EQ(client_alert, 0);

    // The shared secrets are of the correct length...
    EXPECT_EQ(client_secret.size(), t.shared_secret_size);
    EXPECT_EQ(server_secret.size(), t.shared_secret_size);

    // ... but they are not equal
    EXPECT_NE(Bytes(client_secret), Bytes(server_secret));
  }

  CBB_cleanup(&server_out_public_key);
  CBB_cleanup(&client_out_public_key);
  CBB_cleanup(&random_out_public_key);
}


class HybridKeyShareTest : public testing::TestWithParam<HybridGroupTest> {};
INSTANTIATE_TEST_SUITE_P(HybridKeyShareTests, HybridKeyShareTest,
                         testing::ValuesIn(kHybridGroupTests));

// Test a successful round-trip for HybridKeyShare
TEST_P(HybridKeyShareTest, HybridKeyShares) {
  HybridGroupTest t = GetParam();

  // Set up client and server with test case parameters
  bssl::UniquePtr<SSLKeyShare> client_key_share =
      SSLKeyShare::Create(t.group_id);
  bssl::UniquePtr<SSLKeyShare> server_key_share =
      SSLKeyShare::Create(t.group_id);
  ASSERT_TRUE(client_key_share);
  ASSERT_TRUE(server_key_share);
  EXPECT_EQ(t.group_id, client_key_share->GroupID());
  EXPECT_EQ(t.group_id, server_key_share->GroupID());

  // The client generates its key pair and outputs the public key.
  // We initialize the CBB with a capacity of 2 as a simple sanity check
  // to ensure that the CBB will grow accordingly when necessary.
  CBB client_out_public_key;
  EXPECT_TRUE(CBB_init(&client_out_public_key, 2));
  EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
  EXPECT_EQ(CBB_len(&client_out_public_key), t.offer_key_share_size);

  // The server accepts the public key, generates the shared secret,
  // and outputs the ciphertext. Again, we initialize the CBB with
  // a capacity of 2 to ensure it will grow accordingly.
  CBB server_out_public_key;
  EXPECT_TRUE(CBB_init(&server_out_public_key, 2));
  uint8_t server_alert = 0;
  Array<uint8_t> server_secret;
  const uint8_t *client_out_public_key_data = CBB_data(&client_out_public_key);
  ASSERT_TRUE(client_out_public_key_data);
  Span<const uint8_t> client_public_key = MakeConstSpan(
      client_out_public_key_data, CBB_len(&client_out_public_key));
  EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                       &server_alert, client_public_key));
  EXPECT_EQ(CBB_len(&server_out_public_key), t.accept_key_share_size);
  EXPECT_EQ(server_alert, 0);

  // The client accepts the server's public key and decrypts it to obtain
  // the shared secret.
  uint8_t client_alert = 0;
  Array<uint8_t> client_secret;
  const uint8_t *server_out_public_key_data = CBB_data(&server_out_public_key);
  ASSERT_TRUE(server_out_public_key_data);
  Span<const uint8_t> server_public_key = MakeConstSpan(
      server_out_public_key_data, CBB_len(&server_out_public_key));
  EXPECT_TRUE(client_key_share->Finish(&client_secret, &client_alert,
                                       server_public_key));
  EXPECT_EQ(client_alert, 0);

  // Verify that client and server arrived at the same shared secret.
  EXPECT_EQ(server_secret.size(), t.shared_secret_size);
  EXPECT_EQ(client_secret.size(), t.shared_secret_size);
  EXPECT_EQ(Bytes(client_secret), Bytes(server_secret));

  CBB_cleanup(&client_out_public_key);
  CBB_cleanup(&server_out_public_key);
}

class BadHybridKeyShareOfferTest
    : public testing::TestWithParam<HybridGroupTest> {};
INSTANTIATE_TEST_SUITE_P(BadHybridKeyShareOfferTests,
                         BadHybridKeyShareOfferTest,
                         testing::ValuesIn(kHybridGroupTests));

// Test failure cases for HybridKeyShare::Offer()
TEST_P(BadHybridKeyShareOfferTest, BadHybridKeyShareOffers) {
  HybridGroupTest t = GetParam();
  // Basic nullptr check
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);

    ASSERT_FALSE(client_key_share->Offer(nullptr));
  }

  // Offer() should fail if |client_out| has not been initialized at all.
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;
    CBB_zero(&client_out_public_key);

    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
  }

  // Offer() should fail if the CBB has children
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    CBB child;

    client_out_public_key.child = &child;
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    CBB_cleanup(&client_out_public_key);
  }

  // Offer() should succeed on the first call, but fail on all repeated calls
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        bssl::SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;

    EXPECT_TRUE(CBB_init(&client_out_public_key, 2));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    CBB_cleanup(&client_out_public_key);
  }

  // |client_out| is properly initialized, some zeros are written
  // to it so that it records a non-zero length, then its buffer is
  // invalidated.
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    CBB client_out_public_key;

    CBB_init(&client_out_public_key, t.offer_key_share_size);
    EXPECT_TRUE(CBB_add_zeros(&client_out_public_key, 2));
    // Keep a pointer to the buffer so we can cleanup correctly
    uint8_t *buf = client_out_public_key.u.base.buf;
    client_out_public_key.u.base.buf = nullptr;
    EXPECT_EQ(CBB_len(&client_out_public_key), (size_t)2);
    EXPECT_FALSE(client_key_share->Offer(&client_out_public_key));
    client_out_public_key.u.base.buf = buf;
    CBB_cleanup(&client_out_public_key);
  }
}

class BadHybridKeyShareAcceptTest
    : public testing::TestWithParam<HybridGroupTest> {};
INSTANTIATE_TEST_SUITE_P(BadHybridKeyShareAcceptTests,
                         BadHybridKeyShareAcceptTest,
                         testing::ValuesIn(kHybridGroupTests));

const HybridGroup *GetHybridGroup(uint16_t group_id) {
  for (const HybridGroup &g : HybridGroups()) {
    if (group_id == g.group_id) {
      return &g;
    }
  }

  return NULL;
}

// Test failure cases for HybridKeyShare::Accept()
TEST_P(BadHybridKeyShareAcceptTest, BadHybridKeyShareAccept) {
  HybridGroupTest t = GetParam();
  // Basic nullptr checks
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    uint8_t server_alert = 0;

    EXPECT_FALSE(server_key_share->Accept(nullptr, &server_secret,
                                          &server_alert, client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    server_alert = 0;

    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key, nullptr,
                                          &server_alert, client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    server_alert = 0;

    EXPECT_FALSE(server_key_share->Accept(
        &server_out_public_key, &server_secret, nullptr, client_public_key));
  }

  // |server_out_public_key| has not been initialized
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    uint8_t server_alert = 0;

    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
  }

  // |server_out_public_key| is properly initialized, then is assigned a child
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    uint8_t server_alert = 0;
    CBB child;

    CBB_init(&server_out_public_key, 64);
    server_out_public_key.child = &child;
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    CBB_cleanup(&server_out_public_key);
  }

  // |server_out_public_key| is properly initialized with CBB_init,
  // some zeros are written to it so that it records a non-zero length,
  // then its buffer is invalidated.
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    uint8_t server_alert = 0;

    CBB_init(&server_out_public_key, t.accept_key_share_size);
    EXPECT_TRUE(CBB_add_zeros(&server_out_public_key, 2));
    // Keep a pointer to the buffer so we can cleanup correctly
    uint8_t *buf = server_out_public_key.u.base.buf;
    server_out_public_key.u.base.buf = nullptr;
    EXPECT_EQ(CBB_len(&server_out_public_key), (size_t)2);
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    server_out_public_key.u.base.buf = buf;
    CBB_cleanup(&server_out_public_key);
  }

  // |client_public_key| has not been initialized with anything
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    uint8_t server_alert = 0;

    EXPECT_TRUE(CBB_init(&server_out_public_key, 64));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_INTERNAL_ERROR);
    CBB_cleanup(&server_out_public_key);
  }

  // |client_public_key| has been initialized but is empty
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    uint8_t server_alert = 0;

    const uint8_t empty_buffer[1] = {
        0};  // Arrays must have at least 1 element to compile on Windows
    Span<const uint8_t> client_public_key = MakeConstSpan(empty_buffer, 0);

    EXPECT_TRUE(CBB_init(&server_out_public_key, 64));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_ILLEGAL_PARAMETER);
    CBB_cleanup(&server_out_public_key);
  }

  // |client_public_key| is initialized with too little data
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    uint8_t server_alert = 0;

    // Generate a valid |client_public_key|, then truncate the last byte
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    const uint8_t *client_out_public_key_data =
        CBB_data(&client_out_public_key);
    ASSERT_TRUE(client_out_public_key_data);
    client_public_key = MakeConstSpan(client_out_public_key_data,
                                      CBB_len(&client_out_public_key) - 1);

    EXPECT_TRUE(CBB_init(&server_out_public_key, 64));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_ILLEGAL_PARAMETER);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
  }

  // |client_public_key| is initialized with too much data
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> client_public_key;
    Array<uint8_t> server_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    uint8_t server_alert = 0;

    // Generate a valid |client_public_key|, then append a byte
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    EXPECT_TRUE(CBB_add_zeros(&client_out_public_key, 1));
    const uint8_t *client_out_public_key_data =
        CBB_data(&client_out_public_key);
    ASSERT_TRUE(client_out_public_key_data);
    client_public_key = MakeConstSpan(client_out_public_key_data,
                                      CBB_len(&client_out_public_key));

    EXPECT_TRUE(CBB_init(&server_out_public_key, 64));
    EXPECT_FALSE(server_key_share->Accept(&server_out_public_key,
                                          &server_secret, &server_alert,
                                          client_public_key));
    EXPECT_EQ(server_alert, SSL_AD_DECODE_ERROR);
    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
  }

  // |client_public_key| is initialized with key material that is the correct
  // length, but is not a valid key. We do this iteratively over each
  // component group that makes up the hybrid group so that we can test
  // all Accept() code paths in the hybrid key share.
  {
    size_t client_public_key_index = 0;
    for (size_t i = 0; i < NUM_HYBRID_COMPONENTS; i++) {
      // We'll need the hybrid group to retrieve the component share sizes
      const HybridGroup *hybrid_group = GetHybridGroup(t.group_id);
      ASSERT_TRUE(hybrid_group != NULL);

      // Create the hybrid key shares and generate a valid |client_public_key|
      bssl::UniquePtr<SSLKeyShare> client_key_share =
          SSLKeyShare::Create(t.group_id);
      bssl::UniquePtr<SSLKeyShare> server_key_share =
          SSLKeyShare::Create(t.group_id);
      ASSERT_TRUE(client_key_share);
      ASSERT_TRUE(server_key_share);

      CBB client_out_public_key;
      CBB server_out_public_key;
      EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
      EXPECT_TRUE(CBB_init(&server_out_public_key, 64));

      Array<uint8_t> server_secret;
      Array<uint8_t> client_secret;
      uint8_t client_alert = 0;
      uint8_t server_alert = 0;

      EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));

      // For the current component group, overwrite the bytes of that
      // component's key share (and *only* that component's key share) with
      // arbitrary nonsense; leave all other sections of the key share alone.
      // This ensures:
      // 1. The overall size of the hybrid key share is still correct
      // 2. The sizes of the component key shares are still correct; in other
      //    words, the component key shares are still partitioned correctly
      //    and will be parsed individually, as intended
      // 2. The key share associated with the current component group is invalid
      // 3. All other component key shares are still valid
      //
      // (We have to do this in a roundabout way with malloc'ing another
      // buffer because CBBs cannot be arbitrarily edited.)
      size_t client_out_public_key_len = CBB_len(&client_out_public_key);
      const uint8_t *client_out_public_key_data =
          CBB_data(&client_out_public_key);
      ASSERT_TRUE(client_out_public_key_data);
      uint8_t *buffer = (uint8_t *)OPENSSL_malloc(client_out_public_key_len);
      ASSERT_TRUE(buffer);
      OPENSSL_memcpy(buffer, client_out_public_key_data,
                     client_out_public_key_len);

      for (size_t j = client_public_key_index;
           j < client_public_key_index + t.offer_share_sizes[i]; j++) {
        buffer[j] = 7;  // 7 is arbitrary
      }
      Span<const uint8_t> client_public_key =
          MakeConstSpan(buffer, client_out_public_key_len);

      // The server will Accept() the invalid public key
      bool accepted =
          server_key_share->Accept(&server_out_public_key, &server_secret,
                                   &server_alert, client_public_key);

      if (accepted) {
        // The Accept() functionality for X25519 and all KEM key shares is
        // written so that, even if the given public key is invalid, it will
        // return success, output its own public key, and continue with the
        // handshake. (This is the intended functionality.) So, in this
        // case, we assert that the component group was one of those groups,
        // continue with the handshake, then verify that the client and
        // server ultimately arrived at different shared secrets.
        EXPECT_TRUE(
            hybrid_group->component_group_ids[i] == SSL_GROUP_MLKEM768 ||
            hybrid_group->component_group_ids[i] == SSL_GROUP_MLKEM1024 ||
            hybrid_group->component_group_ids[i] == SSL_GROUP_X25519);

        // The handshake will complete without error...
        EXPECT_EQ(server_alert, 0);
        EXPECT_EQ(server_secret.size(), t.shared_secret_size);

        Span<const uint8_t> server_public_key = MakeConstSpan(
            CBB_data(&server_out_public_key), CBB_len(&server_out_public_key));
        EXPECT_TRUE(client_key_share->Finish(&client_secret, &client_alert,
                                             server_public_key));
        EXPECT_EQ(client_secret.size(), t.shared_secret_size);
        EXPECT_EQ(client_alert, 0);

        // ...but client and server will arrive at different shared secrets
        EXPECT_NE(Bytes(client_secret), Bytes(server_secret));

      } else {
        // The Accept() functionality for the NIST curves (e.g. P256) is
        // written so that it will return failure if the key share is invalid.
        EXPECT_TRUE(
            hybrid_group->component_group_ids[i] == SSL_GROUP_SECP256R1 ||
            hybrid_group->component_group_ids[i] == SSL_GROUP_SECP384R1);
        EXPECT_EQ(server_alert, SSL_AD_ILLEGAL_PARAMETER);
      }

      client_public_key_index += t.offer_share_sizes[i];
      CBB_cleanup(&client_out_public_key);
      CBB_cleanup(&server_out_public_key);
      OPENSSL_free(buffer);
    }
  }
}


class BadHybridKeyShareFinishTest
    : public testing::TestWithParam<HybridGroupTest> {};
INSTANTIATE_TEST_SUITE_P(BadHybridKeyShareFinishTests,
                         BadHybridKeyShareFinishTest,
                         testing::ValuesIn(kHybridGroupTests));

// Test failure cases for HybridKeyShare::Finish()
TEST_P(BadHybridKeyShareFinishTest, BadHybridKeyShareFinish) {
  HybridGroupTest t = GetParam();
  // Basic nullptr checks
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    Span<const uint8_t> server_public_key;
    Array<uint8_t> client_secret;
    uint8_t client_alert = 0;
    CBB client_public_key_out;
    CBB_init(&client_public_key_out, 2);
    EXPECT_TRUE(client_key_share->Offer(&client_public_key_out));

    EXPECT_FALSE(
        client_key_share->Finish(nullptr, &client_alert, server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);
    client_alert = 0;

    EXPECT_FALSE(
        client_key_share->Finish(&client_secret, nullptr, server_public_key));

    CBB_cleanup(&client_public_key_out);
  }

  // It is an error if Finish() is called without there
  // having been a previous call to Offer()
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    Array<uint8_t> client_secret;
    uint8_t client_alert = 0;
    uint8_t *buffer = (uint8_t *)OPENSSL_malloc(t.accept_key_share_size);

    Span<const uint8_t> server_public_key =
        MakeConstSpan(buffer, t.accept_key_share_size);

    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);

    OPENSSL_free(buffer);
  }

  // |server_public_key| has not been initialized with anything
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    Span<const uint8_t> server_public_key;
    Array<uint8_t> client_secret;
    uint8_t client_alert = 0;
    CBB client_public_key_out;
    CBB_init(&client_public_key_out, 2);

    EXPECT_TRUE(client_key_share->Offer(&client_public_key_out));

    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_INTERNAL_ERROR);

    CBB_cleanup(&client_public_key_out);
  }

  // |server_public_key| is initialized but is empty
  {
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(client_key_share);
    Array<uint8_t> client_secret;
    uint8_t client_alert = 0;
    const uint8_t empty_buffer[1] = {
        0};  // Arrays must have at least 1 element to compile on Windows
    Span<const uint8_t> server_public_key = MakeConstSpan(empty_buffer, 0);
    CBB client_public_key_out;
    CBB_init(&client_public_key_out, 2);

    EXPECT_TRUE(client_key_share->Offer(&client_public_key_out));

    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    CBB_cleanup(&client_public_key_out);
    EXPECT_EQ(client_alert, SSL_AD_DECODE_ERROR);
  }

  // |server_public_key| is initialized with too little data
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> client_public_key;
    Span<const uint8_t> server_public_key;
    Array<uint8_t> server_secret;
    Array<uint8_t> client_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    uint8_t server_alert = 0;
    uint8_t client_alert = 0;

    // Generate a valid |client_public_key|
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    const uint8_t *client_out_public_key_data =
        CBB_data(&client_out_public_key);
    ASSERT_TRUE(client_out_public_key_data);
    client_public_key = MakeConstSpan(client_out_public_key_data,
                                      CBB_len(&client_out_public_key));

    // Generate a valid |server_public_key|, then truncate the last byte
    EXPECT_TRUE(CBB_init(&server_out_public_key, 64));
    EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                         &server_alert, client_public_key));
    EXPECT_EQ(server_alert, 0);
    const uint8_t *server_out_public_key_data =
        CBB_data(&server_out_public_key);
    ASSERT_TRUE(server_out_public_key_data);
    server_public_key = MakeConstSpan(server_out_public_key_data,
                                      CBB_len(&server_out_public_key) - 1);

    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_DECODE_ERROR);

    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
  }

  // |server_public_key| is initialized with too much data
  {
    bssl::UniquePtr<SSLKeyShare> server_key_share =
        SSLKeyShare::Create(t.group_id);
    bssl::UniquePtr<SSLKeyShare> client_key_share =
        SSLKeyShare::Create(t.group_id);
    ASSERT_TRUE(server_key_share);
    ASSERT_TRUE(client_key_share);
    Span<const uint8_t> client_public_key;
    Span<const uint8_t> server_public_key;
    Array<uint8_t> server_secret;
    Array<uint8_t> client_secret;
    CBB server_out_public_key;
    CBB client_out_public_key;
    uint8_t server_alert = 0;
    uint8_t client_alert = 0;

    // Generate a valid |client_public_key|
    EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
    EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
    const uint8_t *client_out_public_key_data =
        CBB_data(&client_out_public_key);
    ASSERT_TRUE(client_out_public_key_data);
    client_public_key = MakeConstSpan(client_out_public_key_data,
                                      CBB_len(&client_out_public_key));

    // Generate a valid |server_public_key|, then append a byte
    EXPECT_TRUE(CBB_init(&server_out_public_key, 64));
    EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                         &server_alert, client_public_key));
    EXPECT_EQ(server_alert, 0);
    EXPECT_TRUE(CBB_add_zeros(&server_out_public_key, 1));
    const uint8_t *server_out_public_key_data =
        CBB_data(&server_out_public_key);
    ASSERT_TRUE(server_out_public_key_data);
    server_public_key = MakeConstSpan(server_out_public_key_data,
                                      CBB_len(&server_out_public_key));

    EXPECT_FALSE(client_key_share->Finish(&client_secret, &client_alert,
                                          server_public_key));
    EXPECT_EQ(client_alert, SSL_AD_ILLEGAL_PARAMETER);

    CBB_cleanup(&server_out_public_key);
    CBB_cleanup(&client_out_public_key);
  }

  // |server_public_key| is initialized with key material that is the correct
  // length, but is not a valid key. We do this iteratively over each
  // component group that makes up the hybrid group so that we can test
  // all Finish() code paths in the hybrid key share.
  {
    size_t server_public_key_index = 0;
    for (size_t i = 0; i < NUM_HYBRID_COMPONENTS; i++) {
      // We'll need the hybrid group to retrieve the component share sizes
      const HybridGroup *hybrid_group = GetHybridGroup(t.group_id);
      ASSERT_TRUE(hybrid_group != NULL);

      // Create the hybrid key shares and generate a valid |server_public_key|
      bssl::UniquePtr<SSLKeyShare> client_key_share =
          SSLKeyShare::Create(t.group_id);
      bssl::UniquePtr<SSLKeyShare> server_key_share =
          SSLKeyShare::Create(t.group_id);
      ASSERT_TRUE(client_key_share);
      ASSERT_TRUE(server_key_share);

      CBB client_out_public_key;
      CBB server_out_public_key;
      EXPECT_TRUE(CBB_init(&client_out_public_key, 64));
      EXPECT_TRUE(CBB_init(&server_out_public_key, 64));

      Array<uint8_t> server_secret;
      Array<uint8_t> client_secret;
      uint8_t client_alert = 0;
      uint8_t server_alert = 0;

      EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));

      Span<const uint8_t> client_public_key = MakeConstSpan(
          CBB_data(&client_out_public_key), CBB_len(&client_out_public_key));
      EXPECT_TRUE(server_key_share->Accept(&server_out_public_key,
                                           &server_secret, &server_alert,
                                           client_public_key));
      EXPECT_EQ(server_alert, 0);

      // For the current component group, overwrite the bytes of that
      // component's key share (and *only* that component's key share) with
      // arbitrary nonsense; leave all other sections of the key share alone.
      // This ensures:
      // 1. The overall size of the hybrid key share is still correct
      // 2. The sizes of the component key shares are still correct; in other
      //    words, the component key shares are still partitioned correctly
      //    and will be parsed individually, as intended
      // 2. The key share associated with the current component group is invalid
      // 3. All other component key shares are still valid
      //
      // (We have to do this in a roundabout way with malloc'ing another
      // buffer because CBBs cannot be arbitrarily edited.)
      size_t server_out_public_key_len = CBB_len(&server_out_public_key);
      const uint8_t *server_out_public_key_data =
          CBB_data(&server_out_public_key);
      ASSERT_TRUE(server_out_public_key_data);
      uint8_t *buffer = (uint8_t *)OPENSSL_malloc(server_out_public_key_len);
      ASSERT_TRUE(buffer);
      OPENSSL_memcpy(buffer, server_out_public_key_data,
                     server_out_public_key_len);
      for (size_t j = server_public_key_index;
           j < server_public_key_index + t.accept_share_sizes[i]; j++) {
        buffer[j] = 7;  // 7 is arbitrary
      }
      Span<const uint8_t> server_public_key =
          MakeConstSpan(buffer, server_out_public_key_len);

      // The client will Finish() with the invalid public key
      bool accepted = client_key_share->Finish(&client_secret, &client_alert,
                                               server_public_key);

      if (accepted) {
        // The Finish() functionality for X25519 and all KEM key shares is
        // written so that, even if the given public key is invalid, it will
        // return success, output its own public key, and continue with the
        // handshake. (This is the intended functionality.) So, in this
        // case, we assert that the component group was one of those groups,
        // continue with the handshake, then verify that the client and
        // server ultimately arrived at different shared secrets.
        EXPECT_TRUE(
            hybrid_group->component_group_ids[i] == SSL_GROUP_MLKEM768 ||
            hybrid_group->component_group_ids[i] == SSL_GROUP_MLKEM1024 ||
            hybrid_group->component_group_ids[i] == SSL_GROUP_X25519);

        // The handshake will complete without error...
        EXPECT_EQ(client_alert, 0);
        EXPECT_EQ(client_secret.size(), t.shared_secret_size);

        // ...but client and server will arrive at different shared secrets
        EXPECT_NE(Bytes(client_secret), Bytes(server_secret));

      } else {
        // The Finish() functionality for the NIST curves (e.g. P256) is
        // written so that it will return failure if the key share is invalid.
        EXPECT_TRUE(
            hybrid_group->component_group_ids[i] == SSL_GROUP_SECP256R1 ||
            hybrid_group->component_group_ids[i] == SSL_GROUP_SECP384R1);
        EXPECT_EQ(client_alert, SSL_AD_ILLEGAL_PARAMETER);
      }

      server_public_key_index += t.accept_share_sizes[i];
      CBB_cleanup(&client_out_public_key);
      CBB_cleanup(&server_out_public_key);
      OPENSSL_free(buffer);
    }
  }
}


class KemKeyShareTest : public testing::TestWithParam<GroupTest> {};

INSTANTIATE_TEST_SUITE_P(KemKeyShareTests, KemKeyShareTest,
                         testing::ValuesIn(kKemGroupTests));

// Test a successful round-trip for KemKeyShare
TEST_P(KemKeyShareTest, KemKeyShares) {
  GroupTest t = GetParam();
  bssl::UniquePtr<SSLKeyShare> client_key_share =
      bssl::SSLKeyShare::Create(t.group_id);
  bssl::UniquePtr<SSLKeyShare> server_key_share =
      bssl::SSLKeyShare::Create(t.group_id);
  ASSERT_TRUE(client_key_share);
  ASSERT_TRUE(server_key_share);
  EXPECT_EQ(t.group_id, client_key_share->GroupID());
  EXPECT_EQ(t.group_id, server_key_share->GroupID());

  // The client generates its key pair and outputs the public key.
  // We initialize the CBB with a capacity of 2 as a sanity check to
  // ensure that the CBB will grow accordingly if necessary.
  CBB client_out_public_key;
  EXPECT_TRUE(CBB_init(&client_out_public_key, 2));
  EXPECT_TRUE(client_key_share->Offer(&client_out_public_key));
  EXPECT_EQ(CBB_len(&client_out_public_key), t.offer_key_share_size);

  // The server accepts the public key, generates the shared secret,
  // and outputs the ciphertext. Again, we initialize the CBB with
  // a capacity of 2 to ensure it will grow accordingly.
  CBB server_out_public_key;
  EXPECT_TRUE(CBB_init(&server_out_public_key, 2));
  uint8_t server_alert = 0;
  Array<uint8_t> server_secret;
  const uint8_t *client_out_public_key_data = CBB_data(&client_out_public_key);
  ASSERT_TRUE(client_out_public_key_data);
  Span<const uint8_t> client_public_key = MakeConstSpan(
      client_out_public_key_data, CBB_len(&client_out_public_key));
  EXPECT_TRUE(server_key_share->Accept(&server_out_public_key, &server_secret,
                                       &server_alert, client_public_key));
  EXPECT_EQ(CBB_len(&server_out_public_key), t.accept_key_share_size);
  EXPECT_EQ(server_secret.size(), t.shared_secret_size);
  EXPECT_EQ(server_alert, 0);

  // The client accepts the ciphertext and decrypts it to obtain
  // the shared secret.
  uint8_t client_alert = 0;
  Array<uint8_t> client_secret;
  const uint8_t *server_out_public_key_data = CBB_data(&server_out_public_key);
  ASSERT_TRUE(server_out_public_key_data);
  Span<const uint8_t> server_public_key = MakeConstSpan(
      server_out_public_key_data, CBB_len(&server_out_public_key));
  EXPECT_TRUE(client_key_share->Finish(&client_secret, &client_alert,
                                       server_public_key));
  EXPECT_EQ(client_secret.size(), t.shared_secret_size);
  EXPECT_EQ(client_alert, 0);

  // Verify that client and server arrived at the same shared secret.
  EXPECT_EQ(Bytes(client_secret), Bytes(server_secret));

  CBB_cleanup(&client_out_public_key);
  CBB_cleanup(&server_out_public_key);
}

}  // namespace
BSSL_NAMESPACE_END
