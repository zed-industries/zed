// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <memory>
#include <string>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/digest.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>

#include "../../../crypto/fipsmodule/hmac/internal.h"
#include "../test/file_test.h"
#include "../test/test_util.h"
#include "../test/wycheproof_util.h"


static const EVP_MD *GetDigest(const std::string &name) {
  if (name == "MD5") {
    return EVP_md5();
  } else if (name == "SHA1") {
    return EVP_sha1();
  } else if (name == "SHA224") {
    return EVP_sha224();
  } else if (name == "SHA256") {
    return EVP_sha256();
  } else if (name == "SHA384") {
    return EVP_sha384();
  } else if (name == "SHA512") {
    return EVP_sha512();
  } else if (name == "SHA512/224") {
    return EVP_sha512_224();
  } else if (name == "SHA512/256") {
    return EVP_sha512_256();
  } else if (name == "SHA3-224") {
    return EVP_sha3_224();
  } else if (name == "SHA3-256") {
    return EVP_sha3_256();
  } else if (name == "SHA3-384") {
    return EVP_sha3_384();
  } else if (name == "SHA3-512") {
    return EVP_sha3_512();
  }
  return nullptr;
}

static size_t GetPrecomputedKeySize(const std::string &name) {
  if (name == "MD5") {
    return HMAC_MD5_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA1") {
    return HMAC_SHA1_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA224") {
    return HMAC_SHA224_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA256") {
    return HMAC_SHA256_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA384") {
    return HMAC_SHA384_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA512") {
    return HMAC_SHA512_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA512/224") {
    return HMAC_SHA512_224_PRECOMPUTED_KEY_SIZE;
  } else if (name == "SHA512/256") {
    return HMAC_SHA512_256_PRECOMPUTED_KEY_SIZE;
  }
  return 0;
}

static void RunHMACTestEVP(const std::vector<uint8_t> &key,
                           const std::vector<uint8_t> &msg,
                           const std::vector<uint8_t> &tag, const EVP_MD *md) {

  bssl::UniquePtr<EVP_PKEY> pkey_mac(
      EVP_PKEY_new_mac_key(EVP_PKEY_HMAC, nullptr, key.data(), key.size()));
  ASSERT_TRUE(pkey_mac);

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_HMAC, NULL));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
  auto hexkey = EncodeHex(key);
  ASSERT_TRUE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexkey", hexkey.data()));
  EVP_PKEY* my_pkey = NULL;
  ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &my_pkey));
  bssl::UniquePtr<EVP_PKEY> pkey_gen(my_pkey);
  ASSERT_TRUE(pkey_gen);
  for (const auto pkey : {pkey_mac.get(), pkey_gen.get()}) {

    bssl::ScopedEVP_MD_CTX copy, mctx;
    size_t len;
    std::vector<uint8_t> actual;
    ASSERT_TRUE(EVP_DigestSignInit(mctx.get(), nullptr, md, nullptr, pkey));
    // Make a copy we can test against later.
    ASSERT_TRUE(EVP_MD_CTX_copy_ex(copy.get(), mctx.get()));
    ASSERT_TRUE(EVP_DigestSignUpdate(mctx.get(), msg.data(), msg.size()));
    ASSERT_TRUE(EVP_DigestSignFinal(mctx.get(), nullptr, &len));
    actual.resize(len);
    ASSERT_TRUE(EVP_DigestSignFinal(mctx.get(), actual.data(), &len));
    actual.resize(len);
    // Wycheproof tests truncate the tags down to |tagSize|. Expected outputs in
    // hmac_tests.txt have the length of the entire tag.
    EXPECT_EQ(Bytes(tag), Bytes(actual.data(), tag.size()));

    // Repeat the test with |copy|, to check |EVP_MD_CTX_copy_ex| duplicated
    // everything.
    len = 0;
    actual.clear();
    ASSERT_TRUE(EVP_DigestSignUpdate(copy.get(), msg.data(), msg.size()));
    ASSERT_TRUE(EVP_DigestSignFinal(copy.get(), nullptr, &len));
    actual.resize(len);
    ASSERT_TRUE(EVP_DigestSignFinal(copy.get(), actual.data(), &len));
    actual.resize(len);
    EXPECT_EQ(Bytes(tag), Bytes(actual.data(), tag.size()));

    // Test using the one-shot API.
    mctx.Reset();
    copy.Reset();
    len = 0;
    actual.clear();
    ASSERT_TRUE(EVP_DigestSignInit(mctx.get(), nullptr, md, nullptr, pkey));
    ASSERT_TRUE(EVP_MD_CTX_copy_ex(copy.get(), mctx.get()));
    ASSERT_TRUE(
        EVP_DigestSign(mctx.get(), nullptr, &len, msg.data(), msg.size()));
    actual.resize(len);
    ASSERT_TRUE(
        EVP_DigestSign(mctx.get(), actual.data(), &len, msg.data(), msg.size()));
    actual.resize(len);
    EXPECT_EQ(Bytes(tag), Bytes(actual.data(), tag.size()));

    // Repeat the test with |copy|, to check |EVP_MD_CTX_copy_ex| duplicated
    // everything.
    len = 0;
    actual.clear();
    ASSERT_TRUE(EVP_DigestSignUpdate(copy.get(), msg.data(), msg.size()));
    ASSERT_TRUE(EVP_DigestSignFinal(copy.get(), nullptr, &len));
    actual.resize(len);
    ASSERT_TRUE(EVP_DigestSignFinal(copy.get(), actual.data(), &len));
    actual.resize(len);
    EXPECT_EQ(Bytes(tag), Bytes(actual.data(), tag.size()));

    // Test feeding the input in byte by byte.
    mctx.Reset();
    ASSERT_TRUE(EVP_DigestSignInit(mctx.get(), nullptr, md, nullptr, pkey));
    for (const unsigned char &i : msg) {
      ASSERT_TRUE(EVP_DigestSignUpdate(mctx.get(), &i, 1));
    }
    ASSERT_TRUE(EVP_DigestSignFinal(mctx.get(), actual.data(), &len));
    EXPECT_EQ(Bytes(tag), Bytes(actual.data(), tag.size()));


    // Test |EVP_PKEY| key creation with |EVP_PKEY_new_raw_private_key|.
    bssl::UniquePtr<EVP_PKEY> raw_pkey(EVP_PKEY_new_raw_private_key(
        EVP_PKEY_HMAC, nullptr, key.data(), key.size()));
    mctx.Reset();
    len = 0;
    actual.clear();
    EXPECT_TRUE(
        EVP_DigestSignInit(mctx.get(), nullptr, md, nullptr, raw_pkey.get()));
    EXPECT_TRUE(EVP_DigestSignUpdate(mctx.get(), msg.data(), msg.size()));
    EXPECT_TRUE(EVP_DigestSignFinal(mctx.get(), nullptr, &len));
    actual.resize(len);
    EXPECT_TRUE(EVP_DigestSignFinal(mctx.get(), actual.data(), &len));
    actual.resize(len);
    EXPECT_EQ(Bytes(tag), Bytes(actual.data(), tag.size()));

    // Test retrieving key passed into |raw_pkey| with
    // |EVP_PKEY_get_raw_private_key|.
    std::vector<uint8_t> retrieved_key;
    size_t retrieved_key_len;
    EXPECT_TRUE(EVP_PKEY_get_raw_private_key(raw_pkey.get(), nullptr,
                                             &retrieved_key_len));
    EXPECT_EQ(key.size(), retrieved_key_len);
    retrieved_key.resize(retrieved_key_len);
    EXPECT_TRUE(EVP_PKEY_get_raw_private_key(raw_pkey.get(), retrieved_key.data(),
                                             &retrieved_key_len));
    retrieved_key.resize(retrieved_key_len);
    EXPECT_EQ(Bytes(retrieved_key), Bytes(key));

    // Test retrieving key with a buffer length that's too small. This should fail
    if (!key.empty()) {
      size_t short_key_len = retrieved_key_len - 1;
      EXPECT_FALSE(EVP_PKEY_get_raw_private_key(
          raw_pkey.get(), retrieved_key.data(), &short_key_len));
    }
  }
}


TEST(HMACTest, TestVectors) {
  FileTestGTest("crypto/hmac_extra/hmac_tests.txt", [](FileTest *t) {
    std::string digest_str;
    ASSERT_TRUE(t->GetAttribute(&digest_str, "HMAC"));
    const EVP_MD *digest = GetDigest(digest_str);
    ASSERT_TRUE(digest) << "Unknown digest: " << digest_str;

    std::vector<uint8_t> key, input, output;
    ASSERT_TRUE(t->GetBytes(&key, "Key"));
    ASSERT_TRUE(t->GetBytes(&input, "Input"));
    ASSERT_TRUE(t->GetBytes(&output, "Output"));
    ASSERT_EQ(EVP_MD_size(digest), output.size());

    // Test using the one-shot API.
    const unsigned expected_mac_len = EVP_MD_size(digest);
    std::unique_ptr<uint8_t[]> mac(new uint8_t[expected_mac_len]);
    unsigned mac_len;
    ASSERT_TRUE(HMAC(digest, key.data(), key.size(), input.data(), input.size(),
                     mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Test using HMAC_CTX.
    bssl::ScopedHMAC_CTX ctx;
    ASSERT_TRUE(
        HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Test that an HMAC_CTX may be reset with the same key.
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, digest, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Test that an HMAC_CTX may be reset with the same key and a null md
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, nullptr, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Some callers will call init multiple times and we need to ensure that doesn't break anything
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, nullptr, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Test feeding the input in byte by byte.
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, nullptr, nullptr));
    for (size_t i = 0; i < input.size(); i++) {
      ASSERT_TRUE(HMAC_Update(ctx.get(), &input[i], 1));
    }
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));

    // Test consuming HMAC through the |EVP_PKEY_HMAC| interface.
    RunHMACTestEVP(key, input, output, digest);
  });
}

TEST(HMACTest, TestVectorsPrecomputedKey) {
  FileTestGTest("crypto/hmac_extra/hmac_tests.txt", [](FileTest *t) {
    std::string digest_str;
    ASSERT_TRUE(t->GetAttribute(&digest_str, "HMAC"));
    const EVP_MD *digest = GetDigest(digest_str);
    ASSERT_TRUE(digest) << "Unknown digest: " << digest_str;

    std::vector<uint8_t> key, input, output;
    ASSERT_TRUE(t->GetBytes(&key, "Key"));
    ASSERT_TRUE(t->GetBytes(&input, "Input"));
    ASSERT_TRUE(t->GetBytes(&output, "Output"));
    ASSERT_EQ(EVP_MD_size(digest), output.size());

    // Test using the one-shot API.
    const unsigned expected_mac_len = EVP_MD_size(digest);
    std::unique_ptr<uint8_t[]> mac(new uint8_t[expected_mac_len]);
    unsigned mac_len;
    ASSERT_TRUE(HMAC(digest, key.data(), key.size(), input.data(), input.size(),
                     mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Digests that do not support pre-computed keys will have a non-positive
    // pre-computed key size. In this case, assert that we can't successfully
    // call precomputed-key functions.
    bssl::ScopedHMAC_CTX ctx;
    if (GetPrecomputedKeySize(digest_str) <= 0) {
        ASSERT_TRUE(
            HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
        ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
        ASSERT_FALSE(HMAC_set_precomputed_key_export(ctx.get()));
        size_t len;
        ASSERT_FALSE(HMAC_get_precomputed_key(ctx.get(), key.data(), &len));
        ASSERT_FALSE(HMAC_Init_from_precomputed_key(ctx.get(), key.data(), key.size(), digest));
        return;
    }

    // Test using the one-shot API with precompute
    ASSERT_TRUE(HMAC_with_precompute(digest, key.data(), key.size(),
                                     input.data(), input.size(), mac.get(),
                                     &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Test using HMAC_CTX.
    ASSERT_TRUE(
        HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    uint8_t precomputed_key[HMAC_MAX_PRECOMPUTED_KEY_SIZE];

    // Test that the precomputed key cannot be exported without calling
    // HMAC_set_precomputed_key_export
    size_t precomputed_key_len_out = HMAC_MAX_PRECOMPUTED_KEY_SIZE;
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
    ASSERT_FALSE(HMAC_get_precomputed_key(ctx.get(), precomputed_key, &precomputed_key_len_out));

    // Test that the precomputed key cannot be exported if ctx not initialized
    // and the precomputed_key_export flag cannot be set
    bssl::ScopedHMAC_CTX ctx2;
    ASSERT_FALSE(HMAC_set_precomputed_key_export(ctx2.get()));
    precomputed_key_len_out = HMAC_MAX_PRECOMPUTED_KEY_SIZE;
    ASSERT_FALSE(HMAC_get_precomputed_key(ctx2.get(), precomputed_key, &precomputed_key_len_out));

    // Get the precomputed key length for later use
    // And test the precomputed key size is at most HMAC_MAX_PRECOMPUTED_KEY_SIZE
    // and is equal to HMAC_xxx_PRECOMPUTED_KEY_SIZE, where xxx is the digest name
    ASSERT_TRUE(HMAC_set_precomputed_key_export(ctx.get()));
    size_t precomputed_key_len;
    HMAC_get_precomputed_key(ctx.get(), nullptr, &precomputed_key_len);
    ASSERT_LE(precomputed_key_len, (size_t) HMAC_MAX_PRECOMPUTED_KEY_SIZE);
    ASSERT_EQ(GetPrecomputedKeySize(digest_str), precomputed_key_len);

    // Test that at this point, the context cannot be used with HMAC_Update
    ASSERT_FALSE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_FALSE(HMAC_Final(ctx.get(), mac.get(), &mac_len));

    // Export the precomputed key for later use
    precomputed_key_len_out = HMAC_MAX_PRECOMPUTED_KEY_SIZE;
    ASSERT_TRUE(HMAC_get_precomputed_key(ctx.get(), precomputed_key, &precomputed_key_len_out));
    ASSERT_EQ(precomputed_key_len, precomputed_key_len_out);

    // Test that at this point, the context can be used with HMAC_Update
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Test that an HMAC_CTX may be reset with the same key but with HMAC_Init_from_precomputed_key
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, 0, digest));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len); // Clear the prior correct answer

    // Test that an HMAC_CTX may be reset with the same key and a null md but using the Init_from_precomputed_key instead
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, 0, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Some callers will call init multiple times and we need to ensure that doesn't break anything but using a mix of Init_ex and Init_from_precomputed_key
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, 0, nullptr));
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, nullptr, nullptr));
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, 0, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Test that the HMAC_CTX can be reset using the precomputed key
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), precomputed_key, precomputed_key_len, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Same test but starting from an empty context
    ctx.Reset();
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), precomputed_key, precomputed_key_len, digest));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Some callers will call init_from_precomputed_key multiple times and we need to ensure that doesn't break anything
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), precomputed_key, precomputed_key_len, nullptr));
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, 0, nullptr));
    ASSERT_TRUE(HMAC_Update(ctx.get(), input.data(), input.size()));
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));
    OPENSSL_memset(mac.get(), 0, expected_mac_len);  // Clear the prior correct answer

    // Test that we get an error if the out_len is not large enough or is null
    uint8_t precomputed_key2[HMAC_MAX_PRECOMPUTED_KEY_SIZE];
    size_t precomputed_key_len_out2;
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), key.data(), key.size(), digest, nullptr));
    ASSERT_TRUE(HMAC_set_precomputed_key_export(ctx.get()));
    ASSERT_TRUE(HMAC_set_precomputed_key_export(ctx.get())); // testing we can set it twice
    precomputed_key_len_out2 = precomputed_key_len - 1; // slightly too short
    ASSERT_FALSE(HMAC_get_precomputed_key(ctx.get(), precomputed_key2, &precomputed_key_len_out2));
    precomputed_key_len_out2 = 0; // 0-size should also fail
    ASSERT_FALSE(HMAC_get_precomputed_key(ctx.get(), precomputed_key2, &precomputed_key_len_out2));
    ASSERT_FALSE(HMAC_get_precomputed_key(ctx.get(), precomputed_key2, nullptr));

    // Test that we get the same precompute_key the second time we correctly call HMAC_get_precomputed_key
    precomputed_key_len_out2 = precomputed_key_len; // testing with the out_len is the exact value
    ASSERT_TRUE(HMAC_get_precomputed_key(ctx.get(), precomputed_key2, &precomputed_key_len_out2));
    ASSERT_EQ(precomputed_key_len, precomputed_key_len_out2);
    ASSERT_EQ(Bytes(precomputed_key, precomputed_key_len), Bytes(precomputed_key2, precomputed_key_len));
    OPENSSL_memset(precomputed_key2, 0, HMAC_MAX_PRECOMPUTED_KEY_SIZE);  // Clear the prior correct answer

    // Test that at this point, the context cannot be used to re-export the precomputed key
    precomputed_key_len_out2 = HMAC_MAX_PRECOMPUTED_KEY_SIZE;
    ASSERT_FALSE(HMAC_get_precomputed_key(ctx.get(), precomputed_key2, &precomputed_key_len_out2));
    // Check that precomputed_key_len_out2 and precomputed_key2 were not modified and are still their original value
    uint8_t zero_precomputed_key[HMAC_MAX_PRECOMPUTED_KEY_SIZE];
    OPENSSL_memset(zero_precomputed_key, 0, HMAC_MAX_PRECOMPUTED_KEY_SIZE);
    ASSERT_EQ((size_t)HMAC_MAX_PRECOMPUTED_KEY_SIZE, precomputed_key_len_out2);
    ASSERT_EQ(Bytes(zero_precomputed_key, HMAC_MAX_PRECOMPUTED_KEY_SIZE), Bytes(precomputed_key2, HMAC_MAX_PRECOMPUTED_KEY_SIZE));

    // Same but initializing the ctx using the precompute key in the first place
    ctx.Reset();
    ASSERT_TRUE(HMAC_Init_from_precomputed_key(ctx.get(), precomputed_key, precomputed_key_len, digest));
    ASSERT_TRUE(HMAC_set_precomputed_key_export(ctx.get()));
    ASSERT_TRUE(HMAC_get_precomputed_key(ctx.get(), precomputed_key2, &precomputed_key_len_out2));
    ASSERT_EQ(precomputed_key_len, precomputed_key_len_out2);
    ASSERT_EQ(Bytes(precomputed_key, precomputed_key_len), Bytes(precomputed_key2, precomputed_key_len));

    // Test feeding the input in byte by byte after initializing from precomputed key
    ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, nullptr, nullptr));
    for (size_t i = 0; i < input.size(); i++) {
      ASSERT_TRUE(HMAC_Update(ctx.get(), &input[i], 1));
    }
    ASSERT_TRUE(HMAC_Final(ctx.get(), mac.get(), &mac_len));
    EXPECT_EQ(Bytes(output), Bytes(mac.get(), mac_len));

    // Test consuming HMAC through the |EVP_PKEY_HMAC| interface.
    RunHMACTestEVP(key, input, output, digest);

    // Test that initializing without the precomputed_key does not work
    ctx.Reset();
    ASSERT_FALSE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, precomputed_key_len, digest));

    // Test that initializing with the wrong precomputed_key_len does not work
    ctx.Reset();
    ASSERT_FALSE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, 1, digest));
    ASSERT_FALSE(HMAC_Init_from_precomputed_key(ctx.get(), nullptr, precomputed_key_len+1, digest));
  });
}

static void RunWycheproofTest(const char *path, const EVP_MD *md) {
  SCOPED_TRACE(path);
  FileTestGTest(path, [&](FileTest *t) {
    t->IgnoreInstruction("keySize");
    t->IgnoreInstruction("tagSize");
    std::vector<uint8_t> key, msg, tag;
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_TRUE(t->GetBytes(&tag, "tag"));
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    if (!result.IsValid()) {
      // Wycheproof tests assume the HMAC implementation checks the MAC. Ours
      // simply computes the HMAC, so skip the tests with invalid outputs.
      return;
    }

    uint8_t out[EVP_MAX_MD_SIZE];
    unsigned out_len;
    ASSERT_TRUE(HMAC(md, key.data(), key.size(), msg.data(), msg.size(), out,
                     &out_len));
    // Wycheproof tests truncate the tags down to |tagSize|.
    ASSERT_LE(tag.size(), out_len);
    EXPECT_EQ(Bytes(out, tag.size()), Bytes(tag));

    // Run Wycheproof tests through the |EVP_PKEY_HMAC| interface.
    RunHMACTestEVP(key, msg, tag, md);
  });
}

TEST(HMACTest, WycheproofSHA1) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha1_test.txt",
                    EVP_sha1());
}

TEST(HMACTest, WycheproofSHA224) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha224_test.txt",
                    EVP_sha224());
}

TEST(HMACTest, WycheproofSHA256) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha256_test.txt",
                    EVP_sha256());
}

TEST(HMACTest, WycheproofSHA384) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha384_test.txt",
                    EVP_sha384());
}

TEST(HMACTest, WycheproofSHA512) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha512_test.txt",
                    EVP_sha512());
}

TEST(HMACTest, WycheproofSHA512_224) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha512_224_test.txt",
                    EVP_sha512_224());
}

TEST(HMACTest, WycheproofSHA512_256) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha512_256_test.txt",
                    EVP_sha512_256());
}

TEST(HMACTest, WycheproofSHA3_224) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha3_224_test.txt",
                    EVP_sha3_224());
}

TEST(HMACTest, WycheproofSHA3_256) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha3_256_test.txt",
                    EVP_sha3_256());
}

TEST(HMACTest, WycheproofSHA3_384) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha3_384_test.txt",
                    EVP_sha3_384());
}

TEST(HMACTest, WycheproofSHA3_512) {
  RunWycheproofTest("third_party/wycheproof_testvectors/hmac_sha3_512_test.txt",
                    EVP_sha3_512());
}

TEST(HMACTest, EVP_DigestVerify) {
  bssl::UniquePtr<EVP_PKEY> pkey(
      EVP_PKEY_new_mac_key(EVP_PKEY_HMAC, nullptr, nullptr, 0));
  ASSERT_TRUE(pkey);

  bssl::ScopedEVP_MD_CTX mctx;
  EXPECT_FALSE(EVP_DigestVerifyInit(mctx.get(), nullptr, EVP_sha256(), nullptr,
                                    pkey.get()));
  EXPECT_EQ(EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE,
            ERR_GET_REASON(ERR_get_error()));

  EXPECT_FALSE(EVP_DigestVerifyUpdate(mctx.get(), nullptr, 0));
  EXPECT_EQ(EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE,
            ERR_GET_REASON(ERR_get_error()));

  EXPECT_FALSE(EVP_DigestVerifyFinal(mctx.get(), nullptr, 0));
  EXPECT_EQ(EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE,
            ERR_GET_REASON(ERR_get_error()));

  EXPECT_FALSE(EVP_DigestVerify(mctx.get(), nullptr, 0, nullptr, 0));
  EXPECT_EQ(EVP_R_OPERATION_NOT_SUPPORTED_FOR_THIS_KEYTYPE,
            ERR_GET_REASON(ERR_get_error()));
}

TEST(HMACTest, HandlesNullOutputParameters) {
  bssl::ScopedHMAC_CTX ctx;
  const EVP_MD *digest = EVP_sha256();
  // make key and input valid
  const uint8_t key[32] = {0};
  const uint8_t input[16] = {0};
  
  // Test one-shot API with out and out_len as NULL
  ASSERT_FALSE(HMAC(digest, &key[0], sizeof(key), &input[0], sizeof(input),
                  nullptr,nullptr));
  unsigned mac_len;
  // Test one-shot API with only out as NULL
  ASSERT_FALSE(HMAC(digest, &key[0], sizeof(key), &input[0], sizeof(input),
                  nullptr, &mac_len));

  // Test HMAC_ctx
  ASSERT_TRUE(HMAC_Init_ex(ctx.get(), &key[0], sizeof(key), digest, nullptr));
  ASSERT_TRUE(HMAC_Update(ctx.get(), &input[0], sizeof(input)));
  ASSERT_FALSE(HMAC_Final(ctx.get(), nullptr, nullptr));

  ASSERT_TRUE(HMAC_Init_ex(ctx.get(), &key[0], sizeof(key), digest, nullptr));
  ASSERT_TRUE(HMAC_Update(ctx.get(), &input[0], sizeof(input)));
  ASSERT_FALSE(HMAC_Final(ctx.get(), nullptr, &mac_len));
}

TEST(HMACTest, InitExResetsContext) {
  bssl::ScopedHMAC_CTX ctx;
  bssl::ScopedHMAC_CTX ctx_copy;

  const EVP_MD *digest = EVP_sha256();

  const uint8_t key1[] = "test_key_1";
  const uint8_t data1[] = "first_message";

  uint8_t mac1[EVP_MAX_MD_SIZE];
  uint8_t mac2[EVP_MAX_MD_SIZE];
  uint8_t mac3[EVP_MAX_MD_SIZE];
  unsigned mac_len;

  // First HMAC computation with |key1| and |data1|.
  ASSERT_TRUE(HMAC_Init_ex(ctx.get(), key1, sizeof(key1), digest, nullptr));
  ASSERT_TRUE(HMAC_Update(ctx.get(), data1, sizeof(data1)));
  // Copy to |ctx_copy| and finalize that to preserve |HMAC_STATE_IN_PROGRESS|
  // in |ctx|.
  ASSERT_TRUE(HMAC_CTX_copy(ctx_copy.get(), ctx.get()));
  ASSERT_TRUE(HMAC_Final(ctx_copy.get(), mac1, &mac_len));

  // Reset context with NULL |key|/|digest| - should reuse previous key and
  // reset state. This tests that |HMAC_Init_ex| properly resets even after
  // Update/Final.
  ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, nullptr, nullptr));
  ASSERT_TRUE(HMAC_Update(ctx.get(), data1, sizeof(data1)));
  ASSERT_TRUE(HMAC_Final(ctx.get(), mac2, &mac_len));

  // |mac1| and |mac2| should be the same (same key reused, context reset).
  EXPECT_EQ(Bytes(mac1, mac_len), Bytes(mac2, mac_len));

  // Reset |ctx| with NULL |key| but explicit digest - should reuse previous
  // key and reset state.
  ASSERT_TRUE(HMAC_Init_ex(ctx.get(), nullptr, 0, digest, nullptr));
  ASSERT_TRUE(HMAC_Update(ctx.get(), data1, sizeof(data1)));
  // Copy to |ctx_copy| and finalize that to preserve |HMAC_STATE_IN_PROGRESS|
  // in |ctx|.
  ASSERT_TRUE(HMAC_CTX_copy(ctx_copy.get(), ctx.get()));
  ASSERT_TRUE(HMAC_Final(ctx_copy.get(), mac3, &mac_len));

  // |mac1| and |mac3| should be the same (same key reused, context reset).
  EXPECT_EQ(Bytes(mac1, mac_len), Bytes(mac3, mac_len));
}
