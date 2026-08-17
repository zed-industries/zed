// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//
// Created by Smith, Justin on 6/16/25.
//

#include <openssl/ssl.h>
#include <thread>
#include "../crypto/test/test_util.h"
#include "ssl_common_test.h"

BSSL_NAMESPACE_BEGIN

struct ECHConfigParams {
  uint16_t version = TLSEXT_TYPE_encrypted_client_hello;
  uint16_t config_id = 1;
  std::string public_name = "example.com";
  const EVP_HPKE_KEY *key = nullptr;
  // kem_id, if zero, takes its value from |key|.
  uint16_t kem_id = 0;
  // public_key, if empty takes its value from |key|.
  std::vector<uint8_t> public_key;
  size_t max_name_len = 16;
  // cipher_suites is a list of code points which should contain pairs of KDF
  // and AEAD IDs.
  std::vector<uint16_t> cipher_suites = {EVP_HPKE_HKDF_SHA256,
                                         EVP_HPKE_AES_128_GCM};
  std::vector<uint8_t> extensions;
};

// MakeECHConfig serializes an ECHConfig from |params| and writes it to
// |*out|.
static bool MakeECHConfig(std::vector<uint8_t> *out,
                          const ECHConfigParams &params) {
  uint16_t kem_id = params.kem_id == 0
                        ? EVP_HPKE_KEM_id(EVP_HPKE_KEY_kem(params.key))
                        : params.kem_id;
  std::vector<uint8_t> public_key = params.public_key;
  if (public_key.empty()) {
    public_key.resize(EVP_HPKE_MAX_PUBLIC_KEY_LENGTH);
    size_t len = 0;
    if (!EVP_HPKE_KEY_public_key(params.key, public_key.data(), &len,
                                 public_key.size())) {
      return false;
    }
    public_key.resize(len);
  }

  bssl::ScopedCBB cbb;
  CBB contents, child;
  if (!CBB_init(cbb.get(), 64) || !CBB_add_u16(cbb.get(), params.version) ||
      !CBB_add_u16_length_prefixed(cbb.get(), &contents) ||
      !CBB_add_u8(&contents, params.config_id) ||
      !CBB_add_u16(&contents, kem_id) ||
      !CBB_add_u16_length_prefixed(&contents, &child) ||
      !CBB_add_bytes(&child, public_key.data(), public_key.size()) ||
      !CBB_add_u16_length_prefixed(&contents, &child)) {
    return false;
  }
  for (uint16_t cipher_suite : params.cipher_suites) {
    if (!CBB_add_u16(&child, cipher_suite)) {
      return false;
    }
  }
  if (!CBB_add_u8(&contents, params.max_name_len) ||
      !CBB_add_u8_length_prefixed(&contents, &child) ||
      !CBB_add_bytes(
          &child, reinterpret_cast<const uint8_t *>(params.public_name.data()),
          params.public_name.size()) ||
      !CBB_add_u16_length_prefixed(&contents, &child) ||
      !CBB_add_bytes(&child, params.extensions.data(),
                     params.extensions.size()) ||
      !CBB_flush(cbb.get())) {
    return false;
  }

  out->assign(CBB_data(cbb.get()), CBB_data(cbb.get()) + CBB_len(cbb.get()));
  return true;
}

static bssl::UniquePtr<SSL_ECH_KEYS> MakeTestECHKeys(uint8_t config_id = 1) {
  bssl::ScopedEVP_HPKE_KEY key;
  uint8_t *ech_config = nullptr;
  size_t ech_config_len = 0;
  if (!EVP_HPKE_KEY_generate(key.get(), EVP_hpke_x25519_hkdf_sha256()) ||
      !SSL_marshal_ech_config(&ech_config, &ech_config_len, config_id,
                              key.get(), "public.example", 16)) {
    return nullptr;
  }
  bssl::UniquePtr<uint8_t> free_ech_config(ech_config);

  // Install a non-retry config.
  bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
  if (!keys || !SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1, ech_config,
                                 ech_config_len, key.get())) {
    return nullptr;
  }
  return keys;
}

static bool InstallECHConfigList(SSL *client, const SSL_ECH_KEYS *keys) {
  uint8_t *ech_config_list = nullptr;
  size_t ech_config_list_len = 0;
  if (!SSL_ECH_KEYS_marshal_retry_configs(keys, &ech_config_list,
                                          &ech_config_list_len)) {
    return false;
  }
  bssl::UniquePtr<uint8_t> free_ech_config_list(ech_config_list);
  return SSL_set1_ech_config_list(client, ech_config_list, ech_config_list_len);
}

// Test that |SSL_marshal_ech_config| and |SSL_ECH_KEYS_marshal_retry_configs|
// output values as expected.
TEST(SSLTest, MarshalECHConfig) {
  static const uint8_t kPrivateKey[X25519_PRIVATE_KEY_LEN] = {
      0xbc, 0xb5, 0x51, 0x29, 0x31, 0x10, 0x30, 0xc9, 0xed, 0x26, 0xde,
      0xd4, 0xb3, 0xdf, 0x3a, 0xce, 0x06, 0x8a, 0xee, 0x17, 0xab, 0xce,
      0xd7, 0xdb, 0xf3, 0x11, 0xe5, 0xa8, 0xf3, 0xb1, 0x8e, 0x24};
  bssl::ScopedEVP_HPKE_KEY key;
  ASSERT_TRUE(EVP_HPKE_KEY_init(key.get(), EVP_hpke_x25519_hkdf_sha256(),
                                kPrivateKey, sizeof(kPrivateKey)));

  static const uint8_t kECHConfig[] = {
      // version
      0xfe, 0x0d,
      // length
      0x00, 0x41,
      // contents.config_id
      0x01,
      // contents.kem_id
      0x00, 0x20,
      // contents.public_key
      0x00, 0x20, 0xa6, 0x9a, 0x41, 0x48, 0x5d, 0x32, 0x96, 0xa4, 0xe0, 0xc3,
      0x6a, 0xee, 0xf6, 0x63, 0x0f, 0x59, 0x32, 0x6f, 0xdc, 0xff, 0x81, 0x29,
      0x59, 0xa5, 0x85, 0xd3, 0x9b, 0x3b, 0xde, 0x98, 0x55, 0x5c,
      // contents.cipher_suites
      0x00, 0x08, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
      // contents.maximum_name_length
      0x10,
      // contents.public_name
      0x0e, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x2e, 0x65, 0x78, 0x61, 0x6d,
      0x70, 0x6c, 0x65,
      // contents.extensions
      0x00, 0x00};
  uint8_t *ech_config = nullptr;
  size_t ech_config_len = 0;
  ASSERT_TRUE(SSL_marshal_ech_config(&ech_config, &ech_config_len,
                                     /*config_id=*/1, key.get(),
                                     "public.example", 16));
  bssl::UniquePtr<uint8_t> free_ech_config(ech_config);
  EXPECT_EQ(Bytes(kECHConfig), Bytes(ech_config, ech_config_len));

  // Generate a second ECHConfig.
  bssl::ScopedEVP_HPKE_KEY key2;
  ASSERT_TRUE(EVP_HPKE_KEY_generate(key2.get(), EVP_hpke_x25519_hkdf_sha256()));
  uint8_t *ech_config2 = nullptr;
  size_t ech_config2_len = 0;
  ASSERT_TRUE(SSL_marshal_ech_config(&ech_config2, &ech_config2_len,
                                     /*config_id=*/2, key2.get(),
                                     "public.example", 16));
  bssl::UniquePtr<uint8_t> free_ech_config2(ech_config2);

  // Install both ECHConfigs in an |SSL_ECH_KEYS|.
  bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
  ASSERT_TRUE(keys);
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1, ech_config,
                               ech_config_len, key.get()));
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1, ech_config2,
                               ech_config2_len, key2.get()));

  // The ECHConfigList should be correctly serialized.
  uint8_t *ech_config_list = nullptr;
  size_t ech_config_list_len = 0;
  ASSERT_TRUE(SSL_ECH_KEYS_marshal_retry_configs(keys.get(), &ech_config_list,
                                                 &ech_config_list_len));
  bssl::UniquePtr<uint8_t> free_ech_config_list(ech_config_list);

  // ECHConfigList is just the concatenation with a length prefix.
  size_t len = ech_config_len + ech_config2_len;
  std::vector<uint8_t> expected = {uint8_t(len >> 8), uint8_t(len)};
  expected.insert(expected.end(), ech_config, ech_config + ech_config_len);
  expected.insert(expected.end(), ech_config2, ech_config2 + ech_config2_len);
  EXPECT_EQ(Bytes(expected), Bytes(ech_config_list, ech_config_list_len));
}

TEST(SSLTest, ECHHasDuplicateConfigID) {
  const struct {
    std::vector<uint8_t> ids;
    bool has_duplicate;
  } kTests[] = {
      {{}, false},
      {{1}, false},
      {{1, 2, 3, 255}, false},
      {{1, 2, 3, 1}, true},
  };
  for (const auto &test : kTests) {
    bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
    ASSERT_TRUE(keys);
    for (const uint8_t id : test.ids) {
      bssl::ScopedEVP_HPKE_KEY key;
      ASSERT_TRUE(
          EVP_HPKE_KEY_generate(key.get(), EVP_hpke_x25519_hkdf_sha256()));
      uint8_t *ech_config = nullptr;
      size_t ech_config_len = 0;
      ASSERT_TRUE(SSL_marshal_ech_config(&ech_config, &ech_config_len, id,
                                         key.get(), "public.example", 16));
      bssl::UniquePtr<uint8_t> free_ech_config(ech_config);
      ASSERT_TRUE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                   ech_config, ech_config_len, key.get()));
    }

    EXPECT_EQ(test.has_duplicate ? 1 : 0,
              SSL_ECH_KEYS_has_duplicate_config_id(keys.get()));
  }
}

// Test that |SSL_ECH_KEYS_add| checks consistency between the public and
// private key.
TEST(SSLTest, ECHKeyConsistency) {
  bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
  ASSERT_TRUE(keys);
  bssl::ScopedEVP_HPKE_KEY key;
  ASSERT_TRUE(EVP_HPKE_KEY_generate(key.get(), EVP_hpke_x25519_hkdf_sha256()));
  uint8_t public_key[EVP_HPKE_MAX_PUBLIC_KEY_LENGTH];
  size_t public_key_len = 0;
  ASSERT_TRUE(EVP_HPKE_KEY_public_key(key.get(), public_key, &public_key_len,
                                      sizeof(public_key)));

  // Adding an ECHConfig with the matching public key succeeds.
  ECHConfigParams params;
  params.key = key.get();
  std::vector<uint8_t> ech_config;
  ASSERT_TRUE(MakeECHConfig(&ech_config, params));
  EXPECT_TRUE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                               ech_config.data(), ech_config.size(),
                               key.get()));

  // Adding an ECHConfig with the wrong public key is an error.
  bssl::ScopedEVP_HPKE_KEY wrong_key;
  ASSERT_TRUE(
      EVP_HPKE_KEY_generate(wrong_key.get(), EVP_hpke_x25519_hkdf_sha256()));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                wrong_key.get()));

  // Adding an ECHConfig with a truncated public key is an error.
  ECHConfigParams truncated;
  truncated.key = key.get();
  truncated.public_key.assign(public_key, public_key + public_key_len - 1);
  ASSERT_TRUE(MakeECHConfig(&ech_config, truncated));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));

  // Adding an ECHConfig with the right public key, but wrong KEM ID, is an
  // error.
  ECHConfigParams wrong_kem;
  wrong_kem.key = key.get();
  wrong_kem.kem_id = 0x0010;  // DHKEM(P-256, HKDF-SHA256)
  ASSERT_TRUE(MakeECHConfig(&ech_config, wrong_kem));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));
}

// Test that |SSL_CTX_set1_ech_keys| fails when the config list
// has no retry configs.
TEST(SSLTest, ECHServerConfigsWithoutRetryConfigs) {
  bssl::ScopedEVP_HPKE_KEY key;
  ASSERT_TRUE(EVP_HPKE_KEY_generate(key.get(), EVP_hpke_x25519_hkdf_sha256()));
  uint8_t *ech_config = nullptr;
  size_t ech_config_len = 0;
  ASSERT_TRUE(SSL_marshal_ech_config(&ech_config, &ech_config_len,
                                     /*config_id=*/1, key.get(),
                                     "public.example", 16));
  bssl::UniquePtr<uint8_t> free_ech_config(ech_config);

  // Install a non-retry config.
  bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
  ASSERT_TRUE(keys);
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/0, ech_config,
                               ech_config_len, key.get()));

  // |keys| has no retry configs.
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);
  EXPECT_FALSE(SSL_CTX_set1_ech_keys(ctx.get(), keys.get()));

  // Add the same ECHConfig to the list, but this time mark it as a retry
  // config.
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1, ech_config,
                               ech_config_len, key.get()));
  EXPECT_TRUE(SSL_CTX_set1_ech_keys(ctx.get(), keys.get()));
}

// Test that the server APIs reject ECHConfigs with unsupported features.
TEST(SSLTest, UnsupportedECHConfig) {
  bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
  ASSERT_TRUE(keys);
  bssl::ScopedEVP_HPKE_KEY key;
  ASSERT_TRUE(EVP_HPKE_KEY_generate(key.get(), EVP_hpke_x25519_hkdf_sha256()));

  // Unsupported versions are rejected.
  ECHConfigParams unsupported_version;
  unsupported_version.version = 0xffff;
  unsupported_version.key = key.get();
  std::vector<uint8_t> ech_config;
  ASSERT_TRUE(MakeECHConfig(&ech_config, unsupported_version));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));

  // Unsupported cipher suites are rejected. (We only support HKDF-SHA256.)
  ECHConfigParams unsupported_kdf;
  unsupported_kdf.key = key.get();
  unsupported_kdf.cipher_suites = {0x002 /* HKDF-SHA384 */,
                                   EVP_HPKE_AES_128_GCM};
  ASSERT_TRUE(MakeECHConfig(&ech_config, unsupported_kdf));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));
  ECHConfigParams unsupported_aead;
  unsupported_aead.key = key.get();
  unsupported_aead.cipher_suites = {EVP_HPKE_HKDF_SHA256, 0xffff};
  ASSERT_TRUE(MakeECHConfig(&ech_config, unsupported_aead));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));


  // Unsupported extensions are rejected.
  ECHConfigParams extensions;
  extensions.key = key.get();
  extensions.extensions = {0x00, 0x01, 0x00, 0x00};
  ASSERT_TRUE(MakeECHConfig(&ech_config, extensions));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));

  // Invalid public names are rejected.
  ECHConfigParams invalid_public_name;
  invalid_public_name.key = key.get();
  invalid_public_name.public_name = "dns_names_have_no_underscores.example";
  ASSERT_TRUE(MakeECHConfig(&ech_config, invalid_public_name));
  EXPECT_FALSE(SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1,
                                ech_config.data(), ech_config.size(),
                                key.get()));
}


// Test that |SSL_get_client_random| reports the correct value on both client
// and server in ECH. The client sends two different random values. When ECH is
// accepted, we should report the inner one.
TEST(SSLTest, ECHClientRandomsMatch) {
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL_ECH_KEYS> keys = MakeTestECHKeys();
  ASSERT_TRUE(keys);
  ASSERT_TRUE(SSL_CTX_set1_ech_keys(server_ctx.get(), keys.get()));

  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));
  ASSERT_TRUE(InstallECHConfigList(client.get(), keys.get()));
  ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));

  EXPECT_TRUE(SSL_ech_accepted(client.get()));
  EXPECT_TRUE(SSL_ech_accepted(server.get()));

  // An ECH server will fairly naturally record the inner ClientHello random,
  // but an ECH client may forget to update the random once ClientHelloInner is
  // selected.
  uint8_t client_random1[SSL3_RANDOM_SIZE];
  uint8_t client_random2[SSL3_RANDOM_SIZE];
  ASSERT_EQ(sizeof(client_random1),
            SSL_get_client_random(client.get(), client_random1,
                                  sizeof(client_random1)));
  ASSERT_EQ(sizeof(client_random2),
            SSL_get_client_random(server.get(), client_random2,
                                  sizeof(client_random2)));
  EXPECT_EQ(Bytes(client_random1), Bytes(client_random2));
}


// GetECHLength sets |*out_client_hello_len| and |*out_ech_len| to the lengths
// of the ClientHello and ECH extension, respectively, when a client created
// from |ctx| constructs a ClientHello with name |name| and an ECHConfig with
// maximum name length |max_name_len|.
static bool GetECHLength(SSL_CTX *ctx, size_t *out_client_hello_len,
                         size_t *out_ech_len, size_t max_name_len,
                         const char *name) {
  bssl::ScopedEVP_HPKE_KEY key;
  uint8_t *ech_config = nullptr;
  size_t ech_config_len = 0;
  if (!EVP_HPKE_KEY_generate(key.get(), EVP_hpke_x25519_hkdf_sha256()) ||
      !SSL_marshal_ech_config(&ech_config, &ech_config_len,
                              /*config_id=*/1, key.get(), "public.example",
                              max_name_len)) {
    return false;
  }
  bssl::UniquePtr<uint8_t> free_ech_config(ech_config);

  bssl::UniquePtr<SSL_ECH_KEYS> keys(SSL_ECH_KEYS_new());
  if (!keys || !SSL_ECH_KEYS_add(keys.get(), /*is_retry_config=*/1, ech_config,
                                 ech_config_len, key.get())) {
    return false;
  }

  bssl::UniquePtr<SSL> ssl(SSL_new(ctx));
  if (!ssl || !InstallECHConfigList(ssl.get(), keys.get()) ||
      (name != nullptr && !SSL_set_tlsext_host_name(ssl.get(), name))) {
    return false;
  }
  SSL_set_connect_state(ssl.get());

  std::vector<uint8_t> client_hello;
  SSL_CLIENT_HELLO parsed;
  const uint8_t *unused = nullptr;
  if (!GetClientHello(ssl.get(), &client_hello) ||
      !ssl_client_hello_init(
          ssl.get(), &parsed,
          // Skip record and handshake headers. This assumes the ClientHello
          // fits in one record.
          MakeConstSpan(client_hello)
              .subspan(SSL3_RT_HEADER_LENGTH + SSL3_HM_HEADER_LENGTH)) ||
      !SSL_early_callback_ctx_extension_get(
          &parsed, TLSEXT_TYPE_encrypted_client_hello, &unused, out_ech_len)) {
    return false;
  }
  *out_client_hello_len = client_hello.size();
  return true;
}

TEST(SSLTest, ECHPadding) {
  bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(ctx);

  // Sample lengths with max_name_len = 128 as baseline.
  size_t client_hello_len_baseline = 0, ech_len_baseline = 0;
  ASSERT_TRUE(GetECHLength(ctx.get(), &client_hello_len_baseline,
                           &ech_len_baseline, 128, "example.com"));

  // Check that all name lengths under the server's maximum look the same.
  for (size_t name_len : {1, 2, 32, 64, 127, 128}) {
    SCOPED_TRACE(name_len);
    size_t client_hello_len = 0, ech_len = 0;
    ASSERT_TRUE(GetECHLength(ctx.get(), &client_hello_len, &ech_len, 128,
                             std::string(name_len, 'a').c_str()));
    EXPECT_EQ(client_hello_len, client_hello_len_baseline);
    EXPECT_EQ(ech_len, ech_len_baseline);
  }

  // When sending no SNI, we must still pad as if we are sending one.
  size_t client_hello_len = 0, ech_len = 0;
  ASSERT_TRUE(
      GetECHLength(ctx.get(), &client_hello_len, &ech_len, 128, nullptr));
  EXPECT_EQ(client_hello_len, client_hello_len_baseline);
  EXPECT_EQ(ech_len, ech_len_baseline);

  // Name lengths above the maximum do not get named-based padding, but the
  // overall input is padded to a multiple of 32.
  size_t client_hello_len_baseline2 = 0, ech_len_baseline2 = 0;
  ASSERT_TRUE(GetECHLength(ctx.get(), &client_hello_len_baseline2,
                           &ech_len_baseline2, 128,
                           std::string(128 + 32, 'a').c_str()));
  EXPECT_EQ(ech_len_baseline2, ech_len_baseline + 32);
  // The ClientHello lengths may match if we are still under the threshold for
  // padding extension.
  EXPECT_GE(client_hello_len_baseline2, client_hello_len_baseline);

  for (size_t name_len = 128 + 1; name_len < 128 + 32; name_len++) {
    SCOPED_TRACE(name_len);
    ASSERT_TRUE(GetECHLength(ctx.get(), &client_hello_len, &ech_len, 128,
                             std::string(name_len, 'a').c_str()));
    EXPECT_TRUE(ech_len == ech_len_baseline || ech_len == ech_len_baseline2)
        << ech_len;
    EXPECT_TRUE(client_hello_len == client_hello_len_baseline ||
                client_hello_len == client_hello_len_baseline2)
        << client_hello_len;
  }
}

TEST(SSLTest, ECHPublicName) {
  auto str_to_span = [](const char *str) -> Span<const uint8_t> {
    return MakeConstSpan(reinterpret_cast<const uint8_t *>(str), strlen(str));
  };

  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("")));
  EXPECT_TRUE(ssl_is_valid_ech_public_name(str_to_span("example.com")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span(".example.com")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("example.com.")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("example..com")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("www.-example.com")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("www.example-.com")));
  EXPECT_FALSE(
      ssl_is_valid_ech_public_name(str_to_span("no_underscores.example")));
  EXPECT_FALSE(
      ssl_is_valid_ech_public_name(str_to_span("invalid_chars.\x01.example")));
  EXPECT_FALSE(
      ssl_is_valid_ech_public_name(str_to_span("invalid_chars.\xff.example")));
  static const uint8_t kWithNUL[] = {'t', 'e', 's', 't', 0};
  EXPECT_FALSE(ssl_is_valid_ech_public_name(kWithNUL));

  // Test an LDH label with every character and the maximum length.
  EXPECT_TRUE(ssl_is_valid_ech_public_name(str_to_span(
      "abcdefhijklmnopqrstuvwxyz-ABCDEFGHIJKLMNOPQRSTUVWXYZ-0123456789")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span(
      "abcdefhijklmnopqrstuvwxyz-ABCDEFGHIJKLMNOPQRSTUVWXYZ-01234567899")));

  // Inputs with trailing numeric components are rejected.
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("127.0.0.1")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("example.1")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("example.01")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("example.0x01")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("example.0X01")));
  // Leading zeros and values that overflow |uint32_t| are still rejected.
  EXPECT_FALSE(ssl_is_valid_ech_public_name(
      str_to_span("example.123456789000000000000000")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(
      str_to_span("example.012345678900000000000000")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(
      str_to_span("example.0x123456789abcdefABCDEF0")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(
      str_to_span("example.0x0123456789abcdefABCDEF")));
  // Adding a non-digit or non-hex character makes it a valid DNS name again.
  // Single-component numbers are rejected.
  EXPECT_TRUE(ssl_is_valid_ech_public_name(str_to_span("example.1234567890a")));
  EXPECT_TRUE(
      ssl_is_valid_ech_public_name(str_to_span("example.01234567890a")));
  EXPECT_TRUE(
      ssl_is_valid_ech_public_name(str_to_span("example.0x123456789abcdefg")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("1")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("01")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("0x01")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("0X01")));
  // Numbers with trailing dots are rejected. (They are already rejected by the
  // LDH label rules, but the WHATWG URL parser additionally rejects them.)
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("1.")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("01.")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("0x01.")));
  EXPECT_FALSE(ssl_is_valid_ech_public_name(str_to_span("0X01.")));
}


// When using the built-in verifier, test that |SSL_get0_ech_name_override| is
// applied automatically.
TEST(SSLTest, ECHBuiltinVerifier) {
  // Use different config IDs so that fuzzer mode, which breaks trial
  // decryption, will observe the key mismatch.
  bssl::UniquePtr<SSL_ECH_KEYS> keys = MakeTestECHKeys(/*config_id=*/1);
  ASSERT_TRUE(keys);
  bssl::UniquePtr<SSL_ECH_KEYS> wrong_keys = MakeTestECHKeys(/*config_id=*/2);
  ASSERT_TRUE(wrong_keys);
  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(server_ctx);
  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);

  // Configure the client to verify certificates and expect the secret name.
  // This is the name the client is trying to connect to. If ECH is rejected,
  // BoringSSL will internally override this setting with the public name.
  bssl::UniquePtr<X509_STORE> store(X509_STORE_new());
  ASSERT_TRUE(store);
  ASSERT_TRUE(X509_STORE_add_cert(store.get(), GetLeafRoot().get()));
  SSL_CTX_set_cert_store(client_ctx.get(), store.release());
  SSL_CTX_set_verify(client_ctx.get(), SSL_VERIFY_PEER, nullptr);
  X509_VERIFY_PARAM_set_flags(SSL_CTX_get0_param(client_ctx.get()),
                              X509_V_FLAG_NO_CHECK_TIME);
  static const char kSecretName[] = "secret.example";
  ASSERT_TRUE(X509_VERIFY_PARAM_set1_host(SSL_CTX_get0_param(client_ctx.get()),
                                          kSecretName, strlen(kSecretName)));

  // For simplicity, we only run through a pair of representative scenarios here
  // and rely on runner.go to verify that |SSL_get0_ech_name_override| behaves
  // correctly.
  for (bool accept_ech : {false, true}) {
    SCOPED_TRACE(accept_ech);
    for (bool use_leaf_secret : {false, true}) {
      SCOPED_TRACE(use_leaf_secret);

      // The server will reject ECH when configured with the wrong keys.
      ASSERT_TRUE(SSL_CTX_set1_ech_keys(
          server_ctx.get(), accept_ech ? keys.get() : wrong_keys.get()));

      bssl::UniquePtr<SSL> client, server;
      ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                        server_ctx.get()));
      ASSERT_TRUE(InstallECHConfigList(client.get(), keys.get()));

      // Configure the server with the selected certificate.
      ASSERT_TRUE(SSL_use_certificate(
          server.get(),
          use_leaf_secret ? GetLeafSecret().get() : GetLeafPublic().get()));
      ASSERT_TRUE(SSL_use_PrivateKey(server.get(), GetLeafKey().get()));

      // The handshake may fail due to name mismatch or ECH reject. We check
      // |SSL_get_verify_result| to confirm the handshake got far enough.
      CompleteHandshakes(client.get(), server.get());
      EXPECT_EQ(accept_ech == use_leaf_secret ? X509_V_OK
                                              : X509_V_ERR_HOSTNAME_MISMATCH,
                SSL_get_verify_result(client.get()));
    }
  }
}

#if defined(OPENSSL_THREADS)
// Test that the server ECH config can be swapped out while the |SSL_CTX| is
// in use on other threads. This test is intended to be run with TSan.
TEST(SSLTest, ECHThreads) {
  // Generate a pair of ECHConfigs.
  bssl::ScopedEVP_HPKE_KEY key1;
  ASSERT_TRUE(EVP_HPKE_KEY_generate(key1.get(), EVP_hpke_x25519_hkdf_sha256()));
  uint8_t *ech_config1 = nullptr;
  size_t ech_config1_len = 0;
  ASSERT_TRUE(SSL_marshal_ech_config(&ech_config1, &ech_config1_len,
                                     /*config_id=*/1, key1.get(),
                                     "public.example", 16));
  bssl::UniquePtr<uint8_t> free_ech_config1(ech_config1);
  bssl::ScopedEVP_HPKE_KEY key2;
  ASSERT_TRUE(EVP_HPKE_KEY_generate(key2.get(), EVP_hpke_x25519_hkdf_sha256()));
  uint8_t *ech_config2 = nullptr;
  size_t ech_config2_len = 0;
  ASSERT_TRUE(SSL_marshal_ech_config(&ech_config2, &ech_config2_len,
                                     /*config_id=*/2, key2.get(),
                                     "public.example", 16));
  bssl::UniquePtr<uint8_t> free_ech_config2(ech_config2);

  // |keys1| contains the first config. |keys12| contains both.
  bssl::UniquePtr<SSL_ECH_KEYS> keys1(SSL_ECH_KEYS_new());
  ASSERT_TRUE(keys1);
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys1.get(), /*is_retry_config=*/1, ech_config1,
                               ech_config1_len, key1.get()));
  bssl::UniquePtr<SSL_ECH_KEYS> keys12(SSL_ECH_KEYS_new());
  ASSERT_TRUE(keys12);
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys12.get(), /*is_retry_config=*/1, ech_config2,
                               ech_config2_len, key2.get()));
  ASSERT_TRUE(SSL_ECH_KEYS_add(keys12.get(), /*is_retry_config=*/0, ech_config1,
                               ech_config1_len, key1.get()));

  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  ASSERT_TRUE(server_ctx);
  ASSERT_TRUE(SSL_CTX_set1_ech_keys(server_ctx.get(), keys1.get()));

  bssl::UniquePtr<SSL_CTX> client_ctx(SSL_CTX_new(TLS_method()));
  ASSERT_TRUE(client_ctx);
  bssl::UniquePtr<SSL> client, server;
  ASSERT_TRUE(CreateClientAndServer(&client, &server, client_ctx.get(),
                                    server_ctx.get()));
  ASSERT_TRUE(InstallECHConfigList(client.get(), keys1.get()));

  // In parallel, complete the connection and reconfigure the ECHConfig. Note
  // |keys12| supports all the keys in |keys1|, so the handshake should complete
  // the same whichever the server uses.
  std::vector<std::thread> threads;
  threads.emplace_back([&] {
    ASSERT_TRUE(CompleteHandshakes(client.get(), server.get()));
    EXPECT_TRUE(SSL_ech_accepted(client.get()));
    EXPECT_TRUE(SSL_ech_accepted(server.get()));
  });
  threads.emplace_back([&] {
    EXPECT_TRUE(SSL_CTX_set1_ech_keys(server_ctx.get(), keys12.get()));
  });
  for (auto &thread : threads) {
    thread.join();
  }
}
#endif  // OPENSSL_THREADS

// GetExtensionOrder sets |*out| to the list of extensions a client attached to
// |ctx| will send in the ClientHello. If |ech_keys| is non-null, the client
// will offer ECH with the public component. If |decrypt_ech| is true, |*out|
// will be set to the ClientHelloInner's extensions, rather than
// ClientHelloOuter.
static bool GetExtensionOrder(SSL_CTX *client_ctx, std::vector<uint16_t> *out,
                              SSL_ECH_KEYS *ech_keys, bool decrypt_ech) {
  struct AppData {
    std::vector<uint16_t> *out;
    bool decrypt_ech;
    bool callback_done = false;
  };
  AppData app_data;
  app_data.out = out;
  app_data.decrypt_ech = decrypt_ech;

  bssl::UniquePtr<SSL_CTX> server_ctx =
      CreateContextWithTestCertificate(TLS_method());
  if (!server_ctx ||  //
      !SSL_CTX_set_app_data(server_ctx.get(), &app_data) ||
      (decrypt_ech && !SSL_CTX_set1_ech_keys(server_ctx.get(), ech_keys))) {
    return false;
  }

  // Configure the server to record the ClientHello extension order. We use a
  // server rather than |GetClientHello| so it can decrypt ClientHelloInner.
  SSL_CTX_set_select_certificate_cb(
      server_ctx.get(),
      [](const SSL_CLIENT_HELLO *client_hello) -> ssl_select_cert_result_t {
        AppData *app_data_ptr = static_cast<AppData *>(
            SSL_CTX_get_app_data(SSL_get_SSL_CTX(client_hello->ssl)));
        EXPECT_EQ(app_data_ptr->decrypt_ech ? 1 : 0,
                  SSL_ech_accepted(client_hello->ssl));

        app_data_ptr->out->clear();
        CBS extensions;
        CBS_init(&extensions, client_hello->extensions,
                 client_hello->extensions_len);
        while (CBS_len(&extensions)) {
          uint16_t type = 0;
          CBS body;
          if (!CBS_get_u16(&extensions, &type) ||
              !CBS_get_u16_length_prefixed(&extensions, &body)) {
            return ssl_select_cert_error;
          }
          app_data_ptr->out->push_back(type);
        }

        // Don't bother completing the handshake.
        app_data_ptr->callback_done = true;
        return ssl_select_cert_error;
      });

  bssl::UniquePtr<SSL> client, server;
  if (!CreateClientAndServer(&client, &server, client_ctx, server_ctx.get()) ||
      (ech_keys != nullptr && !InstallECHConfigList(client.get(), ech_keys))) {
    return false;
  }

  // Run the handshake far enough to process the ClientHello.
  SSL_do_handshake(client.get());
  SSL_do_handshake(server.get());
  return app_data.callback_done;
}

// Test that, when extension permutation is enabled, the ClientHello extension
// order changes, both with and without ECH, and in both ClientHelloInner and
// ClientHelloOuter.
TEST(SSLTest, PermuteExtensions) {
  bssl::UniquePtr<SSL_ECH_KEYS> keys = MakeTestECHKeys();
  ASSERT_TRUE(keys);
  for (bool offer_ech : {false, true}) {
    SCOPED_TRACE(offer_ech);
    SSL_ECH_KEYS *maybe_keys = offer_ech ? keys.get() : nullptr;
    for (bool decrypt_ech : {false, true}) {
      SCOPED_TRACE(decrypt_ech);
      if (!offer_ech && decrypt_ech) {
        continue;
      }

      // When extension permutation is disabled, the order should be consistent.
      bssl::UniquePtr<SSL_CTX> ctx(SSL_CTX_new(TLS_method()));
      ASSERT_TRUE(ctx);
      std::vector<uint16_t> order1, order2;
      ASSERT_TRUE(
          GetExtensionOrder(ctx.get(), &order1, maybe_keys, decrypt_ech));
      ASSERT_TRUE(
          GetExtensionOrder(ctx.get(), &order2, maybe_keys, decrypt_ech));
      EXPECT_EQ(order1, order2);

      ctx.reset(SSL_CTX_new(TLS_method()));
      ASSERT_TRUE(ctx);
      SSL_CTX_set_permute_extensions(ctx.get(), 1);

      // When extension permutation is enabled, each ClientHello should have a
      // different order.
      //
      // This test is inherently flaky, so we run it multiple times. We send at
      // least five extensions by default from TLS 1.3: supported_versions,
      // key_share, supported_groups, psk_key_exchange_modes, and
      // signature_algorithms. That means the probability of a false negative is
      // at most 1/120. Repeating the test 14 times lowers false negative rate
      // to under 2^-96.
      ASSERT_TRUE(
          GetExtensionOrder(ctx.get(), &order1, maybe_keys, decrypt_ech));
      EXPECT_GE(order1.size(), 5u);
      static const int kNumIterations = 14;
      bool passed = false;
      for (int i = 0; i < kNumIterations; i++) {
        ASSERT_TRUE(
            GetExtensionOrder(ctx.get(), &order2, maybe_keys, decrypt_ech));
        if (order1 != order2) {
          passed = true;
          break;
        }
      }
      EXPECT_TRUE(passed) << "Extensions were not permuted";
    }
  }
}

BSSL_NAMESPACE_END
