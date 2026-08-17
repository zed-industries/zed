// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>
#include <thread>

#include "../crypto/test/file_util.h"
#include "../crypto/test/test_util.h"
#include "internal.h"
#include "ssl_common_test.h"

BSSL_NAMESPACE_BEGIN

TEST(SSLTest, SelectNextProto) {
  uint8_t *result = nullptr;
  uint8_t result_len = 0;

  // If there is an overlap, it should be returned.
  EXPECT_EQ(OPENSSL_NPN_NEGOTIATED,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\1x\1y\1a\1z", 8));
  EXPECT_EQ(Bytes("a"), Bytes(result, result_len));

  EXPECT_EQ(OPENSSL_NPN_NEGOTIATED,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\1x\1y\2bb\1z", 9));
  EXPECT_EQ(Bytes("bb"), Bytes(result, result_len));

  EXPECT_EQ(OPENSSL_NPN_NEGOTIATED,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\1x\1y\3ccc\1z", 10));
  EXPECT_EQ(Bytes("ccc"), Bytes(result, result_len));

  // Peer preference order takes precedence over local.
  EXPECT_EQ(OPENSSL_NPN_NEGOTIATED,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\3ccc\2bb\1a", 9));
  EXPECT_EQ(Bytes("a"), Bytes(result, result_len));

  // If there is no overlap, opportunistically select the first local protocol.
  // ALPN callers should ignore this, but NPN callers may use this per
  // draft-agl-tls-nextprotoneg-03, section 6.
  EXPECT_EQ(OPENSSL_NPN_NO_OVERLAP,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\1x\2yy\3zzz", 9));
  EXPECT_EQ(Bytes("x"), Bytes(result, result_len));

  // The peer preference order may be empty in NPN. This should be treated as no
  // overlap and continue to select an opportunistic protocol.
  EXPECT_EQ(OPENSSL_NPN_NO_OVERLAP,
            SSL_select_next_proto(&result, &result_len, nullptr, 0,
                                  (const uint8_t *)"\1x\2yy\3zzz", 9));
  EXPECT_EQ(Bytes("x"), Bytes(result, result_len));

  // Although calling this function with no local protocols is a caller error,
  // it should cleanly return an empty protocol.
  EXPECT_EQ(
      OPENSSL_NPN_NO_OVERLAP,
      SSL_select_next_proto(&result, &result_len,
                            (const uint8_t *)"\1a\2bb\3ccc", 9, nullptr, 0));
  EXPECT_EQ(Bytes(""), Bytes(result, result_len));

  // Syntax errors are similarly caller errors.
  EXPECT_EQ(
      OPENSSL_NPN_NO_OVERLAP,
      SSL_select_next_proto(&result, &result_len, (const uint8_t *)"\4aaa", 4,
                            (const uint8_t *)"\1a\2bb\3ccc", 9));
  EXPECT_EQ(Bytes(""), Bytes(result, result_len));
  EXPECT_EQ(OPENSSL_NPN_NO_OVERLAP,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\4aaa", 4));
  EXPECT_EQ(Bytes(""), Bytes(result, result_len));

  // Protocols in protocol lists may not be empty.
  EXPECT_EQ(OPENSSL_NPN_NO_OVERLAP,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\0\2bb\3ccc", 8,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9));
  EXPECT_EQ(OPENSSL_NPN_NO_OVERLAP,
            SSL_select_next_proto(&result, &result_len,
                                  (const uint8_t *)"\1a\2bb\3ccc", 9,
                                  (const uint8_t *)"\0\2bb\3ccc", 8));
  EXPECT_EQ(Bytes(""), Bytes(result, result_len));
}

// The client should gracefully handle no suitable ciphers being enabled.
TEST(SSLTest, NoCiphersAvailable) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  // Configure |client_ctx| with a cipher list that does not intersect with its
  // version configuration.
  ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(
      ctx.get(), "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(ctx.get(), TLS1_1_VERSION));

  bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
  ASSERT_TRUE(ssl);
  SSL_set_connect_state(ssl.get());

  UniquePtr<BIO> rbio(BIO_new(BIO_s_mem())), wbio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(rbio);
  ASSERT_TRUE(wbio);
  SSL_set0_rbio(ssl.get(), rbio.release());
  SSL_set0_wbio(ssl.get(), wbio.release());

  int ret = SSL_do_handshake(ssl.get());
  EXPECT_EQ(-1, ret);
  EXPECT_EQ(SSL_ERROR_SSL, SSL_get_error(ssl.get(), ret));
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_SSL, SSL_R_NO_CIPHERS_AVAILABLE));
}

// Test that post-handshake tickets consumed by |SSL_shutdown| are ignored.
TEST(SSLTest, ShutdownIgnoresTickets) {
  bssl::UniquePtr<SSL_CTX> ctx(CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(ctx.get(), TLS1_3_VERSION));

  SSL_CTX_set_session_cache_mode(ctx.get(), SSL_SESS_CACHE_BOTH);

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, ctx.get(), ctx.get()));

  SSL_CTX_sess_set_new_cb(ctx.get(), [](SSL *ssl, SSL_SESSION *session) -> int {
    ADD_FAILURE() << "New session callback called during SSL_shutdown";
    return 0;
  });

  // Send close_notify.
  EXPECT_EQ(0, SSL_shutdown(server.get()));
  EXPECT_EQ(0, SSL_shutdown(client.get()));

  // Receive close_notify.
  EXPECT_EQ(1, SSL_shutdown(server.get()));
  EXPECT_EQ(1, SSL_shutdown(client.get()));
}

TEST(SSLTest, SignatureAlgorithmProperties) {
  EXPECT_EQ(EVP_PKEY_NONE, SSL_get_signature_algorithm_key_type(0x1234));
  EXPECT_EQ(nullptr, SSL_get_signature_algorithm_digest(0x1234));
  EXPECT_FALSE(SSL_is_signature_algorithm_rsa_pss(0x1234));

  EXPECT_EQ(EVP_PKEY_RSA,
            SSL_get_signature_algorithm_key_type(SSL_SIGN_RSA_PKCS1_MD5_SHA1));
  EXPECT_EQ(EVP_md5_sha1(),
            SSL_get_signature_algorithm_digest(SSL_SIGN_RSA_PKCS1_MD5_SHA1));
  EXPECT_FALSE(SSL_is_signature_algorithm_rsa_pss(SSL_SIGN_RSA_PKCS1_MD5_SHA1));

  EXPECT_EQ(EVP_PKEY_EC, SSL_get_signature_algorithm_key_type(
                             SSL_SIGN_ECDSA_SECP256R1_SHA256));
  EXPECT_EQ(EVP_sha256(), SSL_get_signature_algorithm_digest(
                              SSL_SIGN_ECDSA_SECP256R1_SHA256));
  EXPECT_FALSE(
      SSL_is_signature_algorithm_rsa_pss(SSL_SIGN_ECDSA_SECP256R1_SHA256));

  EXPECT_EQ(EVP_PKEY_RSA,
            SSL_get_signature_algorithm_key_type(SSL_SIGN_RSA_PSS_RSAE_SHA384));
  EXPECT_EQ(EVP_sha384(),
            SSL_get_signature_algorithm_digest(SSL_SIGN_RSA_PSS_RSAE_SHA384));
  EXPECT_TRUE(SSL_is_signature_algorithm_rsa_pss(SSL_SIGN_RSA_PSS_RSAE_SHA384));
}



TEST(SSLTest, HandoffDeclined) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  SSL_CTX_set_handoff_mode(server_ctx.get(), true);
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_2_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));

  int client_ret = SSL_do_handshake(client.get());
  int client_err = SSL_get_error(client.get(), client_ret);
  ASSERT_EQ(client_err, SSL_ERROR_WANT_READ);

  int server_ret = SSL_do_handshake(server.get());
  int server_err = SSL_get_error(server.get(), server_ret);
  ASSERT_EQ(server_err, SSL_ERROR_HANDOFF);

  ScopedCBB cbb;
  SSL_CLIENT_HELLO hello;
  ASSERT_TRUE(CBB_init(cbb.get(), 256));
  ASSERT_TRUE(SSL_serialize_handoff(server.get(), cbb.get(), &hello));

  ASSERT_TRUE(SSL_decline_handoff(server.get()));

  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  uint8_t byte = 42;
  EXPECT_EQ(SSL_write(client.get(), &byte, 1), 1);
  EXPECT_EQ(SSL_read(server.get(), &byte, 1), 1);
  EXPECT_EQ(42, byte);

  byte = 43;
  EXPECT_EQ(SSL_write(server.get(), &byte, 1), 1);
  EXPECT_EQ(SSL_read(client.get(), &byte, 1), 1);
  EXPECT_EQ(43, byte);
}

static std::string SigAlgsToString(Span<const uint16_t> sigalgs) {
  std::string ret = "{";

  for (uint16_t v : sigalgs) {
    if (ret.size() > 1) {
      ret += ", ";
    }

    char buf[8];
    snprintf(buf, sizeof(buf) - 1, "0x%02x", v);
    buf[sizeof(buf) - 1] = 0;
    ret += std::string(buf);
  }

  ret += "}";
  return ret;
}

static void ExpectSigAlgsEqual(Span<const uint16_t> expected,
                               Span<const uint16_t> actual) {
  bool matches = false;
  if (expected.size() == actual.size()) {
    matches = true;

    for (size_t i = 0; i < expected.size(); i++) {
      if (expected[i] != actual[i]) {
        matches = false;
        break;
      }
    }
  }

  if (!matches) {
    ADD_FAILURE() << "expected: " << SigAlgsToString(expected)
                  << " got: " << SigAlgsToString(actual);
  }
}

TEST(SSLTest, SigAlgs) {
  static const struct {
    std::vector<int> input;
    bool ok;
    std::vector<uint16_t> expected;
  } kTests[] = {
      {{}, true, {}},
      {{1}, false, {}},
      {{1, 2, 3}, false, {}},
      {{NID_sha256, EVP_PKEY_ED25519}, false, {}},
      {{NID_sha256, EVP_PKEY_RSA, NID_sha256, EVP_PKEY_RSA}, false, {}},

      {{NID_sha256, EVP_PKEY_RSA}, true, {SSL_SIGN_RSA_PKCS1_SHA256}},
      {{NID_sha512, EVP_PKEY_RSA}, true, {SSL_SIGN_RSA_PKCS1_SHA512}},
      {{NID_sha256, EVP_PKEY_RSA_PSS}, true, {SSL_SIGN_RSA_PSS_RSAE_SHA256}},
      {{NID_undef, EVP_PKEY_ED25519}, true, {SSL_SIGN_ED25519}},
      {{NID_undef, EVP_PKEY_ED25519, NID_sha384, EVP_PKEY_EC},
       true,
       {SSL_SIGN_ED25519, SSL_SIGN_ECDSA_SECP384R1_SHA384}},
  };

  UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  unsigned n = 1;
  for (const auto &test : kTests) {
    SCOPED_TRACE(n++);

    const bool ok =
        SSL_CTX_set1_sigalgs(ctx.get(), test.input.data(), test.input.size());
    EXPECT_EQ(ok, test.ok);

    if (!ok) {
      ERR_clear_error();
    }

    if (!test.ok) {
      continue;
    }

    ExpectSigAlgsEqual(test.expected, ctx->cert->sigalgs);
  }
}

TEST(SSLTest, SigAlgsList) {
  static const struct {
    const char *input;
    bool ok;
    std::vector<uint16_t> expected;
  } kTests[] = {
      {"", false, {}},
      {":", false, {}},
      {"+", false, {}},
      {"RSA", false, {}},
      {"RSA+", false, {}},
      {"RSA+SHA256:", false, {}},
      {":RSA+SHA256:", false, {}},
      {":RSA+SHA256+:", false, {}},
      {"!", false, {}},
      {"\x01", false, {}},
      {"RSA+SHA256:RSA+SHA384:RSA+SHA256", false, {}},
      {"RSA-PSS+SHA256:rsa_pss_rsae_sha256", false, {}},

      {"RSA+SHA256", true, {SSL_SIGN_RSA_PKCS1_SHA256}},
      {"RSA+SHA256:ed25519",
       true,
       {SSL_SIGN_RSA_PKCS1_SHA256, SSL_SIGN_ED25519}},
      {"ECDSA+SHA256:RSA+SHA512",
       true,
       {SSL_SIGN_ECDSA_SECP256R1_SHA256, SSL_SIGN_RSA_PKCS1_SHA512}},
      {"ecdsa_secp256r1_sha256:rsa_pss_rsae_sha256",
       true,
       {SSL_SIGN_ECDSA_SECP256R1_SHA256, SSL_SIGN_RSA_PSS_RSAE_SHA256}},
      {"RSA-PSS+SHA256", true, {SSL_SIGN_RSA_PSS_RSAE_SHA256}},
      {"PSS+SHA256", true, {SSL_SIGN_RSA_PSS_RSAE_SHA256}},
  };

  UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  unsigned n = 1;
  for (const auto &test : kTests) {
    SCOPED_TRACE(n++);

    const bool ok = SSL_CTX_set1_sigalgs_list(ctx.get(), test.input);
    EXPECT_EQ(ok, test.ok);

    if (!ok) {
      if (test.ok) {
        ERR_print_errors_fp(stderr);
      }
      ERR_clear_error();
    }

    if (!test.ok) {
      continue;
    }

    ExpectSigAlgsEqual(test.expected, ctx->cert->sigalgs);
  }
}

TEST(SSLTest, ZeroSizedWriteFlushesHandshakeMessages) {
  // If there are pending handshake messages, an |SSL_write| of zero bytes
  // should flush them.
  bssl::UniquePtr<SSL_CTX> server_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(server_ctx);
  EXPECT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));
  EXPECT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  EXPECT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  EXPECT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  BIO *client_wbio = SSL_get_wbio(client.get());
  EXPECT_EQ(0u, BIO_wpending(client_wbio));
  EXPECT_TRUE(SSL_key_update(client.get(), SSL_KEY_UPDATE_NOT_REQUESTED));
  EXPECT_EQ(0u, BIO_wpending(client_wbio));
  EXPECT_EQ(0, SSL_write(client.get(), nullptr, 0));
  EXPECT_NE(0u, BIO_wpending(client_wbio));
}

TEST(SSLTest, SSLGetKeyUpdate) {
  bssl::UniquePtr<SSL_CTX> server_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(server_ctx);
  EXPECT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));
  EXPECT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  EXPECT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  EXPECT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));

  // Initial state should be |SSL_KEY_UPDATE_NONE|.
  EXPECT_EQ(SSL_get_key_update_type(client.get()), SSL_KEY_UPDATE_NONE);

  // Test setting |SSL_key_update| with |SSL_KEY_UPDATE_REQUESTED|.
  EXPECT_TRUE(SSL_key_update(client.get(), SSL_KEY_UPDATE_REQUESTED));
  // |SSL_get_key_update_type| is used to determine whether a key update
  // operation has been scheduled but not yet performed.
  EXPECT_EQ(SSL_get_key_update_type(client.get()), SSL_KEY_UPDATE_REQUESTED);
  EXPECT_EQ(0, SSL_write(client.get(), nullptr, 0));
  // Key update operation should have been performed by now.
  EXPECT_EQ(SSL_get_key_update_type(client.get()), SSL_KEY_UPDATE_NONE);

  // Test setting |SSL_key_update| with |SSL_KEY_UPDATE_NOT_REQUESTED|.
  EXPECT_TRUE(SSL_key_update(client.get(), SSL_KEY_UPDATE_NOT_REQUESTED));
  EXPECT_EQ(SSL_get_key_update_type(client.get()),
            SSL_KEY_UPDATE_NOT_REQUESTED);
  EXPECT_EQ(0, SSL_write(client.get(), nullptr, 0));
  // Key update operation should have been performed by now.
  EXPECT_EQ(SSL_get_key_update_type(client.get()), SSL_KEY_UPDATE_NONE);
}
#if defined(OPENSSL_THREADS)
// SSL_CTX_get0_certificate needs to lock internally. Test this works.
TEST(SSLTest, GetCertificateThreads) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<X509> cert = GetTestCertificate();
  ASSERT_TRUE(cert);
  ASSERT_TRUE(SSL_CTX_use_certificate(ctx.get(), cert.get()));

  // Existing code expects |SSL_CTX_get0_certificate| to be callable from two
  // threads concurrently. It originally was an immutable operation. Now we
  // implement it with a thread-safe cache, so it is worth testing.
  X509 *cert2_thread = nullptr;
  std::thread thread(
      [&] { cert2_thread = SSL_CTX_get0_certificate(ctx.get()); });
  X509 *cert2 = SSL_CTX_get0_certificate(ctx.get());
  thread.join();

  ASSERT_TRUE(cert2);
  ASSERT_TRUE(cert2_thread);
  EXPECT_EQ(cert2, cert2_thread);
  EXPECT_EQ(0, X509_cmp(cert.get(), cert2));
}
#endif  // OPENSSL_THREADS

#ifdef OPENSSL_THREADS

static void SetValueOnFree(void *parent, void *ptr, CRYPTO_EX_DATA *ad,
                           int index, long argl, void *argp) {
  if (ptr != nullptr) {
    *static_cast<long *>(ptr) = argl;
  }
}

// Test that one thread can register ex_data while another thread is destroying
// an object that uses it.
TEST(SSLTest, ExDataThreads) {
  static bool already_run = false;
  if (already_run) {
    GTEST_SKIP() << "This test consumes process-global resources and can only "
                    "be run once in a process. It is not compatible with "
                    "--gtest_repeat.";
  }
  already_run = true;

  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  // Register an initial index, so the threads can exercise having any ex_data.
  int first_index =
      SSL_get_ex_new_index(-1, nullptr, nullptr, nullptr, SetValueOnFree);
  ASSERT_GE(first_index, 0);

  // Callers may register indices concurrently with using other indices. This
  // may happen if one part of an application is initializing while another part
  // is already running.
  static constexpr int kNumIndices = 3;
  static constexpr int kNumSSLs = 10;
  int index[kNumIndices];
  long values[kNumSSLs];
  std::fill(std::begin(values), std::end(values), -2);
  std::vector<std::thread> threads;
  for (size_t i = 0; i < kNumIndices; i++) {
    threads.emplace_back([&, i] {
      index[i] = SSL_get_ex_new_index(static_cast<long>(i), nullptr, nullptr,
                                      nullptr, SetValueOnFree);
      ASSERT_GE(index[i], 0);
    });
  }
  for (size_t i = 0; i < kNumSSLs; i++) {
    threads.emplace_back([&, i] {
      bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
      ASSERT_TRUE(ssl);
      ASSERT_TRUE(SSL_set_ex_data(ssl.get(), first_index, &values[i]));
    });
  }
  for (auto &thread : threads) {
    thread.join();
  }

  // Each of the SSL threads should have set their flag via ex_data.
  for (size_t i = 0; i < kNumSSLs; i++) {
    EXPECT_EQ(values[i], -1);
  }

  // Each of the newly-registered indices should be distinct and work correctly.
  static_assert(kNumIndices <= kNumSSLs, "values buffer too small");
  std::fill(std::begin(values), std::end(values), -2);
  bssl::UniquePtr<SSL> ssl(SSL_new(ctx.get()));
  ASSERT_TRUE(ssl);
  for (size_t i = 0; i < kNumIndices; i++) {
    for (size_t j = 0; j < i; j++) {
      EXPECT_NE(index[i], index[j]);
    }
    ASSERT_TRUE(SSL_set_ex_data(ssl.get(), index[i], &values[i]));
  }
  ssl = nullptr;
  for (size_t i = 0; i < kNumIndices; i++) {
    EXPECT_EQ(values[i], static_cast<long>(i));
  }
}
#endif  // OPENSSL_THREADS

extern "C" {
int BORINGSSL_enum_c_type_test(void);
}

TEST(SSLTest, EnumTypes) {
  EXPECT_EQ(sizeof(int), sizeof(ssl_private_key_result_t));
  EXPECT_EQ(1, BORINGSSL_enum_c_type_test());
}

static void WriteHelloRequest(SSL *server) {
  // This function assumes TLS 1.2 with ChaCha20-Poly1305.
  ASSERT_EQ(SSL_version(server), TLS1_2_VERSION);
  ASSERT_EQ(SSL_CIPHER_get_cipher_nid(SSL_get_current_cipher(server)),
            NID_chacha20_poly1305);

  // Encrypt a HelloRequest.
  uint8_t in[] = {SSL3_MT_HELLO_REQUEST, 0, 0, 0};
#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
  // Fuzzer-mode records are unencrypted.
  uint8_t record[5 + sizeof(in)];
  record[0] = SSL3_RT_HANDSHAKE;
  record[1] = 3;
  record[2] = 3;  // TLS 1.2
  record[3] = 0;
  record[4] = sizeof(record) - 5;
  memcpy(record + 5, in, sizeof(in));
#else
  // Extract key material from |server|.
  static const size_t kKeyLen = 32;
  static const size_t kNonceLen = 12;
  ASSERT_EQ(2u * (kKeyLen + kNonceLen), SSL_get_key_block_len(server));
  uint8_t key_block[2u * (kKeyLen + kNonceLen)];
  ASSERT_TRUE(SSL_generate_key_block(server, key_block, sizeof(key_block)));
  Span<uint8_t> key = MakeSpan(key_block + kKeyLen, kKeyLen);
  Span<uint8_t> nonce =
      MakeSpan(key_block + kKeyLen + kKeyLen + kNonceLen, kNonceLen);

  uint8_t ad[13];
  uint64_t seq = SSL_get_write_sequence(server);
  for (size_t i = 0; i < 8; i++) {
    // The nonce is XORed with the sequence number.
    nonce[11 - i] ^= uint8_t(seq);
    ad[7 - i] = uint8_t(seq);
    seq >>= 8;
  }

  ad[8] = SSL3_RT_HANDSHAKE;
  ad[9] = 3;
  ad[10] = 3;  // TLS 1.2
  ad[11] = 0;
  ad[12] = sizeof(in);

  uint8_t record[5 + sizeof(in) + 16];
  record[0] = SSL3_RT_HANDSHAKE;
  record[1] = 3;
  record[2] = 3;  // TLS 1.2
  record[3] = 0;
  record[4] = sizeof(record) - 5;

  ScopedEVP_AEAD_CTX aead;
  ASSERT_TRUE(EVP_AEAD_CTX_init(aead.get(), EVP_aead_chacha20_poly1305(),
                                key.data(), key.size(),
                                EVP_AEAD_DEFAULT_TAG_LENGTH, nullptr));
  size_t len = 0;
  ASSERT_TRUE(EVP_AEAD_CTX_seal(aead.get(), record + 5, &len,
                                sizeof(record) - 5, nonce.data(), nonce.size(),
                                in, sizeof(in), ad, sizeof(ad)));
  ASSERT_EQ(sizeof(record) - 5, len);
#endif  // BORINGSSL_UNSAFE_FUZZER_MODE

  ASSERT_EQ(int(sizeof(record)),
            BIO_write(SSL_get_wbio(server), record, sizeof(record)));
}

TEST_P(SSLTest, WriteWhileExplicitRenegotiate) {
  bssl::UniquePtr<SSL_CTX> ctx(CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(ctx);

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(
      ctx.get(), "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, ctx.get(), ctx.get()));
  SSL_set_renegotiate_mode(client.get(), ssl_renegotiate_explicit);
  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  if (GetParam().transfer_ssl) {
    // |server| is reset to hold the transferred SSL.
    TransferSSL(&server, ctx.get(), nullptr);
  }

  static const uint8_t kInput[] = {'h', 'e', 'l', 'l', 'o'};

  // Write "hello" until the buffer is full, so |client| has a pending write.
  size_t num_writes = 0;
  for (;;) {
    int ret = SSL_write(client.get(), kInput, sizeof(kInput));
    if (ret != int(sizeof(kInput))) {
      ASSERT_EQ(-1, ret);
      ASSERT_EQ(SSL_ERROR_WANT_WRITE, SSL_get_error(client.get(), ret));
      break;
    }
    num_writes++;
  }

  ASSERT_NO_FATAL_FAILURE(WriteHelloRequest(server.get()));

  // |SSL_read| should pick up the HelloRequest.
  uint8_t byte = 0;
  ASSERT_EQ(-1, SSL_read(client.get(), &byte, 1));
  ASSERT_EQ(SSL_ERROR_WANT_RENEGOTIATE, SSL_get_error(client.get(), -1));

  // Drain the data from the |client|.
  uint8_t buf[sizeof(kInput)];
  for (size_t i = 0; i < num_writes; i++) {
    ASSERT_EQ(int(sizeof(buf)), SSL_read(server.get(), buf, sizeof(buf)));
    EXPECT_EQ(Bytes(buf), Bytes(kInput));
  }

  // |client| should be able to finish the pending write and continue to write,
  // despite the paused HelloRequest.
  ASSERT_EQ(int(sizeof(kInput)),
            SSL_write(client.get(), kInput, sizeof(kInput)));
  ASSERT_EQ(int(sizeof(buf)), SSL_read(server.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes(buf), Bytes(kInput));

  ASSERT_EQ(int(sizeof(kInput)),
            SSL_write(client.get(), kInput, sizeof(kInput)));
  ASSERT_EQ(int(sizeof(buf)), SSL_read(server.get(), buf, sizeof(buf)));
  EXPECT_EQ(Bytes(buf), Bytes(kInput));

  // |SSL_read| is stuck until we acknowledge the HelloRequest.
  ASSERT_EQ(-1, SSL_read(client.get(), &byte, 1));
  ASSERT_EQ(SSL_ERROR_WANT_RENEGOTIATE, SSL_get_error(client.get(), -1));

  ASSERT_TRUE(SSL_renegotiate(client.get()));
  ASSERT_EQ(-1, SSL_read(client.get(), &byte, 1));
  ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(client.get(), -1));

  // We never renegotiate as a server.
  ASSERT_EQ(-1, SSL_read(server.get(), buf, sizeof(buf)));
  ASSERT_EQ(SSL_ERROR_SSL, SSL_get_error(server.get(), -1));
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_SSL, SSL_R_NO_RENEGOTIATION));

  EXPECT_EQ(SSL_CTX_sess_connect_renegotiate(ctx.get()), 1);
  EXPECT_EQ(SSL_CTX_sess_accept_renegotiate(ctx.get()), 0);
}

TEST(SSLTest, ConnectionPropertiesDuringRenegotiate) {
  // Configure known connection properties, so we can check against them.
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<X509> cert = GetTestCertificate();
  ASSERT_TRUE(cert);
  bssl::UniquePtr<EVP_PKEY> key = GetTestKey();
  ASSERT_TRUE(key);
  ASSERT_TRUE(SSL_CTX_use_certificate(ctx.get(), cert.get()));
  ASSERT_TRUE(SSL_CTX_use_PrivateKey(ctx.get(), key.get()));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(ctx.get(), TLS1_2_VERSION));
  ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(
      ctx.get(), "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256"));
  ASSERT_TRUE(SSL_CTX_set1_groups_list(ctx.get(), "X25519"));
  ASSERT_TRUE(SSL_CTX_set1_sigalgs_list(ctx.get(), "rsa_pkcs1_sha256"));

  // Connect a client and server that accept renegotiation.
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, ctx.get(), ctx.get()));
  SSL_set_renegotiate_mode(client.get(), ssl_renegotiate_freely);
  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  auto check_properties = [&] {
    EXPECT_EQ(SSL_version(client.get()), TLS1_2_VERSION);
    const SSL_CIPHER *cipher = SSL_get_current_cipher(client.get());
    ASSERT_TRUE(cipher);
    EXPECT_EQ(SSL_CIPHER_get_id(cipher),
              uint32_t{TLS1_CK_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256});
    EXPECT_EQ(SSL_get_group_id(client.get()), SSL_GROUP_X25519);
    EXPECT_EQ(SSL_get_negotiated_group(client.get()), NID_X25519);
    EXPECT_EQ(SSL_get_peer_signature_algorithm(client.get()),
              SSL_SIGN_RSA_PKCS1_SHA256);

    int psig_nid = 0;
    EXPECT_TRUE(SSL_get_peer_signature_type_nid(client.get(), &psig_nid));
    EXPECT_EQ(psig_nid, EVP_PKEY_RSA);
    int digest_nid = 0;
    EXPECT_TRUE(SSL_get_peer_signature_nid(client.get(), &digest_nid));
    EXPECT_EQ(digest_nid, NID_sha256);

    bssl::UniquePtr<X509> peer(SSL_get_peer_certificate(client.get()));
    ASSERT_TRUE(peer);
    EXPECT_EQ(X509_cmp(cert.get(), peer.get()), 0);
  };
  check_properties();

  // Client has not signed any TLS messages yet
  EXPECT_FALSE(SSL_get_peer_signature_type_nid(server.get(), nullptr));
  EXPECT_FALSE(SSL_get_peer_signature_nid(server.get(), nullptr));

  // The server sends a HelloRequest.
  ASSERT_NO_FATAL_FAILURE(WriteHelloRequest(server.get()));

  // Reading from the client will consume the HelloRequest, start a
  // renegotiation, and then block on a ServerHello from the server.
  uint8_t byte = 0;
  ASSERT_EQ(-1, SSL_read(client.get(), &byte, 1));
  ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(client.get(), -1));

  // Connection properties should continue to report values from the original
  // handshake.
  check_properties();
  EXPECT_EQ(SSL_CTX_sess_connect_renegotiate(ctx.get()), 1);
  EXPECT_EQ(SSL_CTX_sess_accept_renegotiate(ctx.get()), 0);

  // Client does not sign any messages in renegotiation either
  EXPECT_FALSE(SSL_get_peer_signature_type_nid(server.get(), nullptr));
  EXPECT_FALSE(SSL_get_peer_signature_nid(server.get(), nullptr));
}

TEST(SSLTest, SSLGetSignatureData) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<X509> cert = GetECDSATestCertificate();
  ASSERT_TRUE(cert);
  bssl::UniquePtr<EVP_PKEY> key = GetECDSATestKey();
  ASSERT_TRUE(key);
  ASSERT_TRUE(SSL_CTX_use_certificate(ctx.get(), cert.get()));
  ASSERT_TRUE(SSL_CTX_use_PrivateKey(ctx.get(), key.get()));

  // Explicitly configure |SSL_VERIFY_PEER| so both the client and server
  // verify each other
  SSL_CTX_set_custom_verify(
      ctx.get(), SSL_VERIFY_PEER,
      [](SSL *ssl, uint8_t *out_alert) { return ssl_verify_ok; });

  ASSERT_TRUE(SSL_CTX_set_min_proto_version(ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set1_sigalgs_list(ctx.get(), "ECDSA+SHA256"));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, ctx.get(), ctx.get()));

  // Before handshake, neither client nor server has signed any messages
  ASSERT_FALSE(SSL_get_peer_signature_nid(client.get(), nullptr));
  ASSERT_FALSE(SSL_get_peer_signature_nid(server.get(), nullptr));
  ASSERT_FALSE(SSL_get_peer_signature_type_nid(client.get(), nullptr));
  ASSERT_FALSE(SSL_get_peer_signature_type_nid(server.get(), nullptr));

  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  // Both client and server verified each other, both have signed TLS messages
  // now
  int client_digest = 0, client_sigtype = 0;
  ASSERT_TRUE(SSL_get_peer_signature_nid(server.get(), &client_digest));
  ASSERT_TRUE(SSL_get_peer_signature_type_nid(server.get(), &client_sigtype));
  ASSERT_EQ(client_sigtype, EVP_PKEY_EC);
  ASSERT_EQ(client_digest, NID_sha256);

  int server_digest = 0, server_sigtype = 0;
  ASSERT_TRUE(SSL_get_peer_signature_nid(client.get(), &server_digest));
  ASSERT_TRUE(SSL_get_peer_signature_type_nid(client.get(), &server_sigtype));
  ASSERT_EQ(server_sigtype, EVP_PKEY_EC);
  ASSERT_EQ(server_digest, NID_sha256);
}

TEST(SSLTest, CopyWithoutEarlyData) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);

  SSL_CTX_set_session_cache_mode(client_ctx.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_session_cache_mode(server_ctx.get(), SSL_SESS_CACHE_BOTH);
  SSL_CTX_set_early_data_enabled(client_ctx.get(), 1);
  SSL_CTX_set_early_data_enabled(server_ctx.get(), 1);

  bssl::UniquePtr<SSL_SESSION> session =
      CreateClientSession(client_ctx.get(), server_ctx.get());
  ASSERT_TRUE(session);

  // The client should attempt early data with |session|.
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));
  SSL_set_session(client.get(), session.get());
  SSL_set_early_data_enabled(client.get(), 1);
  ASSERT_EQ(1, SSL_do_handshake(client.get()));
  EXPECT_TRUE(SSL_in_early_data(client.get()));

  // |SSL_SESSION_copy_without_early_data| should disable early data but
  // still resume the session.
  bssl::UniquePtr<SSL_SESSION> session2(
      SSL_SESSION_copy_without_early_data(session.get()));
  ASSERT_TRUE(session2);
  EXPECT_NE(session.get(), session2.get());
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));
  SSL_set_session(client.get(), session2.get());
  SSL_set_early_data_enabled(client.get(), 1);
  EXPECT_TRUE(CompleteHandshakes(client.get(), server.get()));
  EXPECT_TRUE(SSL_session_reused(client.get()));
  EXPECT_EQ(ssl_early_data_unsupported_for_session,
            SSL_get_early_data_reason(client.get()));

  // |SSL_SESSION_copy_without_early_data| should be a reference count increase
  // when passed an early-data-incapable session.
  bssl::UniquePtr<SSL_SESSION> session3(
      SSL_SESSION_copy_without_early_data(session2.get()));
  EXPECT_EQ(session2.get(), session3.get());
}

TEST(SSLTest, ProcessTLS13NewSessionTicket) {
  // Configure client and server to negotiate TLS 1.3 only.
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx(
      CreateContextWithTestCertificate(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(server_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));

  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get()));
  EXPECT_EQ(TLS1_3_VERSION, SSL_version(client.get()));

  // Process a TLS 1.3 NewSessionTicket.
  static const uint8_t kTicket[] = {
      0x04, 0x00, 0x00, 0xb2, 0x00, 0x02, 0xa3, 0x00, 0x04, 0x03, 0x02, 0x01,
      0x01, 0x00, 0x00, 0xa0, 0x01, 0x06, 0x09, 0x11, 0x16, 0x19, 0x21, 0x26,
      0x29, 0x31, 0x36, 0x39, 0x41, 0x46, 0x49, 0x51, 0x03, 0x06, 0x09, 0x13,
      0x16, 0x19, 0x23, 0x26, 0x29, 0x33, 0x36, 0x39, 0x43, 0x46, 0x49, 0x53,
      0xf7, 0x00, 0x29, 0xec, 0xf2, 0xc4, 0xa4, 0x41, 0xfc, 0x30, 0x17, 0x2e,
      0x9f, 0x7c, 0xa8, 0xaf, 0x75, 0x70, 0xf0, 0x1f, 0xc7, 0x98, 0xf7, 0xcf,
      0x5a, 0x5a, 0x6b, 0x5b, 0xfe, 0xf1, 0xe7, 0x3a, 0xe8, 0xf7, 0x6c, 0xd2,
      0xa8, 0xa6, 0x92, 0x5b, 0x96, 0x8d, 0xde, 0xdb, 0xd3, 0x20, 0x6a, 0xcb,
      0x69, 0x06, 0xf4, 0x91, 0x85, 0x2e, 0xe6, 0x5e, 0x0c, 0x59, 0xf2, 0x9e,
      0x9b, 0x79, 0x91, 0x24, 0x7e, 0x4a, 0x32, 0x3d, 0xbe, 0x4b, 0x80, 0x70,
      0xaf, 0xd0, 0x1d, 0xe2, 0xca, 0x05, 0x35, 0x09, 0x09, 0x05, 0x0f, 0xbb,
      0xc4, 0xae, 0xd7, 0xc4, 0xed, 0xd7, 0xae, 0x35, 0xc8, 0x73, 0x63, 0x78,
      0x64, 0xc9, 0x7a, 0x1f, 0xed, 0x7a, 0x9a, 0x47, 0x44, 0xfd, 0x50, 0xf7,
      0xb7, 0xe0, 0x64, 0xa9, 0x02, 0xc1, 0x5c, 0x23, 0x18, 0x3f, 0xc4, 0xcf,
      0x72, 0x02, 0x59, 0x2d, 0xe1, 0xaa, 0x61, 0x72, 0x00, 0x04, 0x5a, 0x5a,
      0x00, 0x00,
  };
  bssl::UniquePtr<SSL_SESSION> session(SSL_process_tls13_new_session_ticket(
      client.get(), kTicket, sizeof(kTicket)));
  ASSERT_TRUE(session);
  ASSERT_TRUE(SSL_SESSION_has_ticket(session.get()));

  uint8_t *session_buf = nullptr;
  size_t session_length = 0;
  ASSERT_TRUE(
      SSL_SESSION_to_bytes(session.get(), &session_buf, &session_length));
  bssl::UniquePtr<uint8_t> session_buf_free(session_buf);
  ASSERT_TRUE(session_buf);
  ASSERT_GT(session_length, 0u);

  // Servers cannot call |SSL_process_tls13_new_session_ticket|.
  ASSERT_FALSE(SSL_process_tls13_new_session_ticket(server.get(), kTicket,
                                                    sizeof(kTicket)));

  // Clients cannot call |SSL_process_tls13_new_session_ticket| before the
  // handshake completes.
  bssl::UniquePtr<SSL> client2(SSL_new(client_ctx.get()));
  ASSERT_TRUE(client2);
  SSL_set_connect_state(client2.get());
  ASSERT_FALSE(SSL_process_tls13_new_session_ticket(client2.get(), kTicket,
                                                    sizeof(kTicket)));
}


TEST(SSLTest, EmptyWriteBlockedOnHandshakeData) {
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(server_ctx);
  // Configure only TLS 1.3. This test requires post-handshake NewSessionTicket.
  ASSERT_TRUE(SSL_CTX_set_min_proto_version(client_ctx.get(), TLS1_3_VERSION));
  ASSERT_TRUE(SSL_CTX_set_max_proto_version(client_ctx.get(), TLS1_3_VERSION));

  // Connect a client and server with tiny buffer between the two.
  bssl::UniquePtr<SSL> client(SSL_new(client_ctx.get())),
      server(SSL_new(server_ctx.get()));
  ASSERT_TRUE(client);
  ASSERT_TRUE(server);
  SSL_set_connect_state(client.get());
  SSL_set_accept_state(server.get());
  BIO *bio1 = nullptr, *bio2 = nullptr;
  ASSERT_TRUE(BIO_new_bio_pair(&bio1, 1, &bio2, 1));
  SSL_set_bio(client.get(), bio1, bio1);
  SSL_set_bio(server.get(), bio2, bio2);
  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  // We defer NewSessionTicket to the first write, so the server has a pending
  // NewSessionTicket. See https://boringssl-review.googlesource.com/34948. This
  // means an empty write will flush the ticket. However, the transport only
  // allows one byte through, so this will fail with |SSL_ERROR_WANT_WRITE|.
  int ret = SSL_write(server.get(), nullptr, 0);
  ASSERT_EQ(ret, -1);
  ASSERT_EQ(SSL_get_error(server.get(), ret), SSL_ERROR_WANT_WRITE);

  // Attempting to write non-zero data should not trip |SSL_R_BAD_WRITE_RETRY|.
  const uint8_t kData[] = {'h', 'e', 'l', 'l', 'o'};
  ret = SSL_write(server.get(), kData, sizeof(kData));
  ASSERT_EQ(ret, -1);
  ASSERT_EQ(SSL_get_error(server.get(), ret), SSL_ERROR_WANT_WRITE);

  // Byte by byte, the data should eventually get through.
  uint8_t buf[sizeof(kData)];
  for (;;) {
    ret = SSL_read(client.get(), buf, sizeof(buf));
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(SSL_get_error(client.get(), ret), SSL_ERROR_WANT_READ);

    ret = SSL_write(server.get(), kData, sizeof(kData));
    if (ret > 0) {
      ASSERT_EQ(ret, 5);
      break;
    }
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(SSL_get_error(server.get(), ret), SSL_ERROR_WANT_WRITE);
  }

  ret = SSL_read(client.get(), buf, sizeof(buf));
  ASSERT_EQ(ret, static_cast<int>(sizeof(kData)));
  ASSERT_EQ(Bytes(buf, ret), Bytes(kData));
}


TEST(SSLTest, HostMatching) {
  static const char kCertPEM[] = R"(
-----BEGIN CERTIFICATE-----
MIIB9jCCAZ2gAwIBAgIQeudG9R61BOxUvWkeVhU5DTAKBggqhkjOPQQDAjApMRAw
DgYDVQQKEwdBY21lIENvMRUwEwYDVQQDEwxleGFtcGxlMy5jb20wHhcNMjExMjA2
MjA1NjU2WhcNMjIxMjA2MjA1NjU2WjApMRAwDgYDVQQKEwdBY21lIENvMRUwEwYD
VQQDEwxleGFtcGxlMy5jb20wWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAAS7l2VO
Bl2TjVm9WfGk24+hMbVFUNB+RVHWbCvFvNZAoWiIJ2z34RLGInyZvCZ8xLAvsuaW
ULDDaoeDl1M0t4Hmo4GmMIGjMA4GA1UdDwEB/wQEAwIChDATBgNVHSUEDDAKBggr
BgEFBQcDATAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBTTJWurcc1t+VPQBko3
Gsw6cbcWSTBMBgNVHREERTBDggxleGFtcGxlMS5jb22CDGV4YW1wbGUyLmNvbYIP
YSouZXhhbXBsZTQuY29tgg4qLmV4YW1wbGU1LmNvbYcEAQIDBDAKBggqhkjOPQQD
AgNHADBEAiAAv0ljHJGrgyzZDkG6XvNZ5ewxRfnXcZuD0Y7E4giCZgIgNK1qjilu
5DyVbfKeeJhOCtGxqE1dWLXyJBnoRomSYBY=
-----END CERTIFICATE-----
)";
  bssl::UniquePtr<X509> cert(CertFromPEM(kCertPEM));
  ASSERT_TRUE(cert);
  static const char kCertNoSANsPEM[] = R"(
-----BEGIN CERTIFICATE-----
MIIBqzCCAVGgAwIBAgIQeudG9R61BOxUvWkeVhU5DTAKBggqhkjOPQQDAjArMRIw
EAYDVQQKEwlBY21lIENvIDIxFTATBgNVBAMTDGV4YW1wbGUzLmNvbTAeFw0yMTEy
MDYyMDU2NTZaFw0yMjEyMDYyMDU2NTZaMCsxEjAQBgNVBAoTCUFjbWUgQ28gMjEV
MBMGA1UEAxMMZXhhbXBsZTMuY29tMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE
u5dlTgZdk41ZvVnxpNuPoTG1RVDQfkVR1mwrxbzWQKFoiCds9+ESxiJ8mbwmfMSw
L7LmllCww2qHg5dTNLeB5qNXMFUwDgYDVR0PAQH/BAQDAgKEMBMGA1UdJQQMMAoG
CCsGAQUFBwMBMA8GA1UdEwEB/wQFMAMBAf8wHQYDVR0OBBYEFNMla6txzW35U9AG
SjcazDpxtxZJMAoGCCqGSM49BAMCA0gAMEUCIG3YWGWtpVhbcGV7wFKQwTfmvwHW
pw4qCFZlool4hCwsAiEA+2fc6NfSbNpFEtQkDOMJW2ANiScAVEmImNqPfb2klz4=
-----END CERTIFICATE-----
)";
  bssl::UniquePtr<X509> cert_no_sans(CertFromPEM(kCertNoSANsPEM));
  ASSERT_TRUE(cert_no_sans);

  static const char kKeyPEM[] = R"(
-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQghsaSZhUzZAcQlLyJ
MDuy7WPdyqNsAX9rmEP650LF/q2hRANCAAS7l2VOBl2TjVm9WfGk24+hMbVFUNB+
RVHWbCvFvNZAoWiIJ2z34RLGInyZvCZ8xLAvsuaWULDDaoeDl1M0t4Hm
-----END PRIVATE KEY-----
)";
  bssl::UniquePtr<EVP_PKEY> key(KeyFromPEM(kKeyPEM));
  ASSERT_TRUE(key);

  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  ASSERT_TRUE(X509_STORE_add_cert(SSL_CTX_get_cert_store(client_ctx.get()),
                                  cert.get()));
  ASSERT_TRUE(X509_STORE_add_cert(SSL_CTX_get_cert_store(client_ctx.get()),
                                  cert_no_sans.get()));
  SSL_CTX_set_verify(client_ctx.get(),
                     SSL_VERIFY_PEER | SSL_VERIFY_FAIL_IF_NO_PEER_CERT,
                     nullptr);
  X509_VERIFY_PARAM_set_flags(SSL_CTX_get0_param(client_ctx.get()),
                              X509_V_FLAG_NO_CHECK_TIME);

  struct TestCase {
    X509 *cert;
    std::string hostname;
    unsigned flags;
    bool should_match;
  };
  std::vector<TestCase> kTests = {
      // These two names are present as SANs in the certificate.
      {cert.get(), "example1.com", 0, true},
      {cert.get(), "example2.com", 0, true},
      // This is the CN of the certificate, but that shouldn't matter if a SAN
      // extension is present.
      {cert.get(), "example3.com", 0, false},
      // If the SAN is not present, we, for now, look for DNS names in the CN.
      {cert_no_sans.get(), "example3.com", 0, true},
      // ... but this can be turned off.
      {cert_no_sans.get(), "example3.com", X509_CHECK_FLAG_NEVER_CHECK_SUBJECT,
       false},
      // a*.example4.com is a SAN, but is invalid.
      {cert.get(), "abc.example4.com", 0, false},
      // *.example5.com is a SAN in the certificate, which is a normal and valid
      // wildcard.
      {cert.get(), "abc.example5.com", 0, true},
      // This name is not present.
      {cert.get(), "notexample1.com", 0, false},
      // The IPv4 address 1.2.3.4 is a SAN, but that shouldn't match against a
      // hostname that happens to be its textual representation.
      {cert.get(), "1.2.3.4", 0, false},
  };

  for (const TestCase &test : kTests) {
    SCOPED_TRACE(test.hostname);

    bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
    ASSERT_TRUE(server_ctx);
    ASSERT_TRUE(SSL_CTX_use_certificate(server_ctx.get(), test.cert));
    ASSERT_TRUE(SSL_CTX_use_PrivateKey(server_ctx.get(), key.get()));

    ClientConfig config;
    bssl::UniquePtr<SSL> client, server;
    config.verify_hostname = test.hostname;
    config.hostflags = test.flags;
    EXPECT_EQ(test.should_match,
              ConnectClientAndServer(&client, &server, client_ctx.get(),
                                     server_ctx.get(), config));
  }
}

TEST(SSLTest, NumTickets) {
  bssl::UniquePtr<SSL_CTX> server_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  bssl::UniquePtr<X509> cert = GetTestCertificate();
  ASSERT_TRUE(cert);
  bssl::UniquePtr<EVP_PKEY> key = GetTestKey();
  ASSERT_TRUE(key);
  ASSERT_TRUE(SSL_CTX_use_certificate(server_ctx.get(), cert.get()));
  ASSERT_TRUE(SSL_CTX_use_PrivateKey(server_ctx.get(), key.get()));
  SSL_CTX_set_session_cache_mode(server_ctx.get(), SSL_SESS_CACHE_BOTH);

  SSL_CTX_set_session_cache_mode(client_ctx.get(), SSL_SESS_CACHE_BOTH);
  static size_t ticket_count;
  SSL_CTX_sess_set_new_cb(client_ctx.get(), [](SSL *, SSL_SESSION *) -> int {
    ticket_count++;
    return 0;
  });

  auto count_tickets = [&]() -> size_t {
    ticket_count = 0;
    bssl::UniquePtr<SSL> client, server;
    if (!ConnectClientAndServer(&client, &server, client_ctx.get(),
                                server_ctx.get()) ||
        !FlushNewSessionTickets(client.get(), server.get())) {
      ADD_FAILURE() << "Could not run handshake";
      return 0;
    }
    return ticket_count;
  };

  // By default, we should send two tickets.
  EXPECT_EQ(count_tickets(), 2u);

  for (size_t num_tickets : {0, 1, 2, 3, 4, 5}) {
    SCOPED_TRACE(num_tickets);
    ASSERT_TRUE(SSL_CTX_set_num_tickets(server_ctx.get(), num_tickets));
    EXPECT_EQ(SSL_CTX_get_num_tickets(server_ctx.get()), num_tickets);
    EXPECT_EQ(count_tickets(), num_tickets);
  }

  // Configuring too many tickets causes us to stop at some point.
  ASSERT_TRUE(SSL_CTX_set_num_tickets(server_ctx.get(), 100000));
  EXPECT_EQ(SSL_CTX_get_num_tickets(server_ctx.get()), 16u);
  EXPECT_EQ(count_tickets(), 16u);
}

TEST(SSLTest, CertSubjectsToStack) {
  const std::string kCert1 = R"(
-----BEGIN CERTIFICATE-----
MIIBzzCCAXagAwIBAgIJANlMBNpJfb/rMAkGByqGSM49BAEwRTELMAkGA1UEBhMC
QVUxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGEludGVybmV0IFdpZGdp
dHMgUHR5IEx0ZDAeFw0xNDA0MjMyMzIxNTdaFw0xNDA1MjMyMzIxNTdaMEUxCzAJ
BgNVBAYTAkFVMRMwEQYDVQQIDApTb21lLVN0YXRlMSEwHwYDVQQKDBhJbnRlcm5l
dCBXaWRnaXRzIFB0eSBMdGQwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAATmK2ni
v2Wfl74vHg2UikzVl2u3qR4NRvvdqakendy6WgHn1peoChj5w8SjHlbifINI2xYa
HPUdfvGULUvPciLBo1AwTjAdBgNVHQ4EFgQUq4TSrKuV8IJOFngHVVdf5CaNgtEw
HwYDVR0jBBgwFoAUq4TSrKuV8IJOFngHVVdf5CaNgtEwDAYDVR0TBAUwAwEB/zAJ
BgcqhkjOPQQBA0gAMEUCIQDyoDVeUTo2w4J5m+4nUIWOcAZ0lVfSKXQA9L4Vh13E
BwIgfB55FGohg/B6dGh5XxSZmmi08cueFV7mHzJSYV51yRQ=
-----END CERTIFICATE-----
)";
  const std::vector<uint8_t> kName1 = {
      0x30, 0x45, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13,
      0x02, 0x41, 0x55, 0x31, 0x13, 0x30, 0x11, 0x06, 0x03, 0x55, 0x04, 0x08,
      0x0c, 0x0a, 0x53, 0x6f, 0x6d, 0x65, 0x2d, 0x53, 0x74, 0x61, 0x74, 0x65,
      0x31, 0x21, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x0c, 0x18, 0x49,
      0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x20, 0x57, 0x69, 0x64, 0x67,
      0x69, 0x74, 0x73, 0x20, 0x50, 0x74, 0x79, 0x20, 0x4c, 0x74, 0x64};
  const std::string kCert2 = R"(
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIIWjO48ufpunYwDQYJKoZIhvcNAQELBQAwNjEaMBgGA1UE
ChMRQm9yaW5nU1NMIFRFU1RJTkcxGDAWBgNVBAMTD0ludGVybWVkaWF0ZSBDQTAg
Fw0xNTAxMDEwMDAwMDBaGA8yMTAwMDEwMTAwMDAwMFowMjEaMBgGA1UEChMRQm9y
aW5nU1NMIFRFU1RJTkcxFDASBgNVBAMTC2V4YW1wbGUuY29tMIGfMA0GCSqGSIb3
DQEBAQUAA4GNADCBiQKBgQDD0U0ZYgqShJ7oOjsyNKyVXEHqeafmk/bAoPqY/h1c
oPw2E8KmeqiUSoTPjG5IXSblOxcqpbAXgnjPzo8DI3GNMhAf8SYNYsoH7gc7Uy7j
5x8bUrisGnuTHqkqH6d4/e7ETJ7i3CpR8bvK16DggEvQTudLipz8FBHtYhFakfdh
TwIDAQABo3cwdTAOBgNVHQ8BAf8EBAMCBaAwHQYDVR0lBBYwFAYIKwYBBQUHAwEG
CCsGAQUFBwMCMAwGA1UdEwEB/wQCMAAwGQYDVR0OBBIEEKN5pvbur7mlXjeMEYA0
4nUwGwYDVR0jBBQwEoAQjBpoqLV2211Xex+NFLIGozANBgkqhkiG9w0BAQsFAAOB
gQBj/p+JChp//LnXWC1k121LM/ii7hFzQzMrt70bny406SGz9jAjaPOX4S3gt38y
rhjpPukBlSzgQXFg66y6q5qp1nQTD1Cw6NkKBe9WuBlY3iYfmsf7WT8nhlT1CttU
xNCwyMX9mtdXdQicOfNjIGUCD5OLV5PgHFPRKiHHioBAhg==
-----END CERTIFICATE-----
)";
  const std::vector<uint8_t> kName2 = {
      0x30, 0x32, 0x31, 0x1a, 0x30, 0x18, 0x06, 0x03, 0x55, 0x04, 0x0a,
      0x13, 0x11, 0x42, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x53, 0x53, 0x4c,
      0x20, 0x54, 0x45, 0x53, 0x54, 0x49, 0x4e, 0x47, 0x31, 0x14, 0x30,
      0x12, 0x06, 0x03, 0x55, 0x04, 0x03, 0x13, 0x0b, 0x65, 0x78, 0x61,
      0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d};

  const struct {
    std::vector<std::vector<uint8_t>> existing;
    std::string pem;
    std::vector<std::vector<uint8_t>> expected;
  } kTests[] = {
      // Do nothing.
      {{}, "", {}},
      // Append to an empty list, skipping duplicates.
      {{}, kCert1 + kCert2 + kCert1, {kName1, kName2}},
      // One of the names was already present.
      {{kName1}, kCert1 + kCert2, {kName1, kName2}},
      // Both names were already present.
      {{kName1, kName2}, kCert1 + kCert2, {kName1, kName2}},
      // Preserve existing duplicates.
      {{kName1, kName2, kName2}, kCert1 + kCert2, {kName1, kName2, kName2}},
  };
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kTests); i++) {
    SCOPED_TRACE(i);
    const auto &t = kTests[i];

    bssl::UniquePtr<STACK_OF(X509_NAME)> stack(sk_X509_NAME_new_null());
    ASSERT_TRUE(stack);
    for (const auto &name : t.existing) {
      const uint8_t *inp = name.data();
      bssl::UniquePtr<X509_NAME> name_obj(
          d2i_X509_NAME(nullptr, &inp, name.size()));
      ASSERT_TRUE(name_obj);
      EXPECT_EQ(inp, name.data() + name.size());
      ASSERT_TRUE(bssl::PushToStack(stack.get(), std::move(name_obj)));
    }

    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(t.pem.data(), t.pem.size()));
    ASSERT_TRUE(bio);
    ASSERT_TRUE(SSL_add_bio_cert_subjects_to_stack(stack.get(), bio.get()));

    // The function should have left |stack|'s comparison function alone.
    EXPECT_EQ(nullptr, sk_X509_NAME_set_cmp_func(stack.get(), nullptr));

    std::vector<std::vector<uint8_t>> expected = t.expected, result;
    for (X509_NAME *name : stack.get()) {
      uint8_t *der = nullptr;
      int der_len = i2d_X509_NAME(name, &der);
      ASSERT_GE(der_len, 0);
      result.push_back(std::vector<uint8_t>(der, der + der_len));
      OPENSSL_free(der);
    }

    // |SSL_add_bio_cert_subjects_to_stack| does not return the output in a
    // well-defined order.
    std::sort(expected.begin(), expected.end());
    std::sort(result.begin(), result.end());
    EXPECT_EQ(result, expected);
  }
}

TEST(SSLTest, EmptyClientCAList) {
  if (SkipTempFileTests()) {
    GTEST_SKIP();
  }

  TemporaryFile empty;
  ASSERT_TRUE(empty.Init());
  bssl::UniquePtr<STACK_OF(X509_NAME)> names(
      SSL_load_client_CA_file(empty.path().c_str()));
  EXPECT_FALSE(names);
}


// Test that |SSL_can_release_private_key| reports true as early as expected.
// The internal asserts in the library check we do not report true too early.
TEST(SSLTest, CanReleasePrivateKey) {
  bssl::UniquePtr<SSL_CTX> client_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(client_ctx);
  SSL_CTX_set_session_cache_mode(client_ctx.get(), SSL_SESS_CACHE_BOTH);

  // Note this assumes the transport buffer is large enough to fit the client
  // and server first flights. We check this with |SSL_ERROR_WANT_READ|. If the
  // transport buffer was too small it would return |SSL_ERROR_WANT_WRITE|.
  auto check_first_server_round_trip = [&](SSL *client, SSL *server) {
    // Write the ClientHello.
    ASSERT_EQ(-1, SSL_do_handshake(client));
    ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(client, -1));

    // Consume the ClientHello and write the server flight.
    ASSERT_EQ(-1, SSL_do_handshake(server));
    ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(server, -1));

    EXPECT_TRUE(SSL_can_release_private_key(server));
  };

  {
    SCOPED_TRACE("TLS 1.2 ECDHE");
    bssl::UniquePtr<SSL_CTX> server_ctx(
        CreateContextWithTestCertificate(TLS_method()));
    ASSERT_TRUE(server_ctx);
    ASSERT_TRUE(
        SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_2_VERSION));
    ASSERT_TRUE(SSL_CTX_set_strict_cipher_list(
        server_ctx.get(), "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"));
    // Configure the server to request client certificates, so we can also test
    // the client half.
    SSL_CTX_set_custom_verify(
        server_ctx.get(), SSL_VERIFY_PEER,
        [](SSL *ssl, uint8_t *out_alert) { return ssl_verify_ok; });
    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                      server_ctx.get()));
    check_first_server_round_trip(client.get(), server.get());

    // Consume the server flight and write the client response. The client still
    // has a Finished message to consume but can also release its key early.
    ASSERT_EQ(-1, SSL_do_handshake(client.get()));
    ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(client.get(), -1));
    EXPECT_TRUE(SSL_can_release_private_key(client.get()));

    // However, a client that has not disabled renegotiation can never release
    // the key.
    ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                      server_ctx.get()));
    SSL_set_renegotiate_mode(client.get(), ssl_renegotiate_freely);
    check_first_server_round_trip(client.get(), server.get());
    ASSERT_EQ(-1, SSL_do_handshake(client.get()));
    ASSERT_EQ(SSL_ERROR_WANT_READ, SSL_get_error(client.get(), -1));
    EXPECT_FALSE(SSL_can_release_private_key(client.get()));
  }

  {
    SCOPED_TRACE("TLS 1.2 resumption");
    bssl::UniquePtr<SSL_CTX> server_ctx(
        CreateContextWithTestCertificate(TLS_method()));
    ASSERT_TRUE(server_ctx);
    ASSERT_TRUE(
        SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_2_VERSION));
    bssl::UniquePtr<SSL_SESSION> session =
        CreateClientSession(client_ctx.get(), server_ctx.get());
    ASSERT_TRUE(session);
    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                      server_ctx.get()));
    SSL_set_session(client.get(), session.get());
    check_first_server_round_trip(client.get(), server.get());
  }

  {
    SCOPED_TRACE("TLS 1.3 1-RTT");
    bssl::UniquePtr<SSL_CTX> server_ctx(
        CreateContextWithTestCertificate(TLS_method()));
    ASSERT_TRUE(server_ctx);
    ASSERT_TRUE(
        SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));
    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                      server_ctx.get()));
    check_first_server_round_trip(client.get(), server.get());
  }

  {
    SCOPED_TRACE("TLS 1.3 resumption");
    bssl::UniquePtr<SSL_CTX> server_ctx(
        CreateContextWithTestCertificate(TLS_method()));
    ASSERT_TRUE(server_ctx);
    ASSERT_TRUE(
        SSL_CTX_set_max_proto_version(server_ctx.get(), TLS1_3_VERSION));
    bssl::UniquePtr<SSL_SESSION> session =
        CreateClientSession(client_ctx.get(), server_ctx.get());
    ASSERT_TRUE(session);
    bssl::UniquePtr<SSL> client, server;
    ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                      server_ctx.get()));
    SSL_set_session(client.get(), session.get());
    check_first_server_round_trip(client.get(), server.get());
  }
}

BSSL_NAMESPACE_END
