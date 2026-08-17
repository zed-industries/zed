// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>

#include <algorithm>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/cipher.h>
#include <openssl/cmac.h>
#include <openssl/mem.h>

#include "../../test/file_test.h"
#include "../../test/test_util.h"
#include "../../test/wycheproof_util.h"


static void test(const char *name, const uint8_t *key, size_t key_len,
                 const uint8_t *msg, size_t msg_len, const uint8_t *expected) {
  SCOPED_TRACE(name);

  // Test the single-shot API.
  uint8_t out[16];
  ASSERT_TRUE(AES_CMAC(out, key, key_len, msg, msg_len));
  EXPECT_EQ(Bytes(expected, sizeof(out)), Bytes(out));

  bssl::UniquePtr<CMAC_CTX> ctx(CMAC_CTX_new());
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(CMAC_Init(ctx.get(), key, key_len, EVP_aes_128_cbc(), NULL));

  for (unsigned chunk_size = 1; chunk_size <= msg_len; chunk_size++) {
    SCOPED_TRACE(chunk_size);

    ASSERT_TRUE(CMAC_Reset(ctx.get()));

    size_t done = 0;
    while (done < msg_len) {
      size_t todo = std::min(msg_len - done, static_cast<size_t>(chunk_size));
      ASSERT_TRUE(CMAC_Update(ctx.get(), msg + done, todo));
      done += todo;
    }

    size_t out_len;
    ASSERT_TRUE(CMAC_Final(ctx.get(), out, &out_len));
    EXPECT_EQ(Bytes(expected, sizeof(out)), Bytes(out, out_len));
  }

  // Test that |CMAC_CTX_copy| works.
  ASSERT_TRUE(CMAC_Reset(ctx.get()));
  size_t chunk = msg_len / 2;
  ASSERT_TRUE(CMAC_Update(ctx.get(), msg, chunk));
  bssl::UniquePtr<CMAC_CTX> ctx2(CMAC_CTX_new());
  ASSERT_TRUE(ctx2);
  ASSERT_TRUE(CMAC_CTX_copy(ctx2.get(), ctx.get()));
  ASSERT_TRUE(CMAC_Update(ctx2.get(), msg + chunk, msg_len - chunk));
  size_t out_len;
  ASSERT_TRUE(CMAC_Final(ctx2.get(), out, &out_len));
  EXPECT_EQ(Bytes(expected, sizeof(out)), Bytes(out, out_len));
}

TEST(CMACTest, RFC4493TestVectors) {
  static const uint8_t kKey[16] = {
      0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
      0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
  };
  static const uint8_t kOut1[16] = {
      0xbb, 0x1d, 0x69, 0x29, 0xe9, 0x59, 0x37, 0x28,
      0x7f, 0xa3, 0x7d, 0x12, 0x9b, 0x75, 0x67, 0x46,
  };
  static const uint8_t kMsg2[] = {
      0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
      0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
  };
  static const uint8_t kOut2[16] = {
      0x07, 0x0a, 0x16, 0xb4, 0x6b, 0x4d, 0x41, 0x44,
      0xf7, 0x9b, 0xdd, 0x9d, 0xd0, 0x4a, 0x28, 0x7c,
  };
  static const uint8_t kMsg3[] = {
      0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
      0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
      0xae, 0x2d, 0x8a, 0x57, 0x1e, 0x03, 0xac, 0x9c,
      0x9e, 0xb7, 0x6f, 0xac, 0x45, 0xaf, 0x8e, 0x51,
      0x30, 0xc8, 0x1c, 0x46, 0xa3, 0x5c, 0xe4, 0x11,
  };
  static const uint8_t kOut3[16] = {
      0xdf, 0xa6, 0x67, 0x47, 0xde, 0x9a, 0xe6, 0x30,
      0x30, 0xca, 0x32, 0x61, 0x14, 0x97, 0xc8, 0x27,
  };
  static const uint8_t kMsg4[] = {
      0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
      0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
      0xae, 0x2d, 0x8a, 0x57, 0x1e, 0x03, 0xac, 0x9c,
      0x9e, 0xb7, 0x6f, 0xac, 0x45, 0xaf, 0x8e, 0x51,
      0x30, 0xc8, 0x1c, 0x46, 0xa3, 0x5c, 0xe4, 0x11,
      0xe5, 0xfb, 0xc1, 0x19, 0x1a, 0x0a, 0x52, 0xef,
      0xf6, 0x9f, 0x24, 0x45, 0xdf, 0x4f, 0x9b, 0x17,
      0xad, 0x2b, 0x41, 0x7b, 0xe6, 0x6c, 0x37, 0x10,
  };
  static const uint8_t kOut4[16] = {
      0x51, 0xf0, 0xbe, 0xbf, 0x7e, 0x3b, 0x9d, 0x92,
      0xfc, 0x49, 0x74, 0x17, 0x79, 0x36, 0x3c, 0xfe,
  };

  test("RFC 4493 #1", kKey, sizeof(kKey), NULL, 0, kOut1);
  test("RFC 4493 #2", kKey, sizeof(kKey), kMsg2, sizeof(kMsg2), kOut2);
  test("RFC 4493 #3", kKey, sizeof(kKey), kMsg3, sizeof(kMsg3), kOut3);
  test("RFC 4493 #4", kKey, sizeof(kKey), kMsg4, sizeof(kMsg4), kOut4);
}

TEST(CMACTest, Wycheproof) {
  FileTestGTest("third_party/wycheproof_testvectors/aes_cmac_test.txt",
                [](FileTest *t) {
    std::string key_size, tag_size;
    ASSERT_TRUE(t->GetInstruction(&key_size, "keySize"));
    ASSERT_TRUE(t->GetInstruction(&tag_size, "tagSize"));
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));
    std::vector<uint8_t> key, msg, tag;
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_TRUE(t->GetBytes(&tag, "tag"));

    const EVP_CIPHER *cipher;
    switch (atoi(key_size.c_str())) {
      case 128:
        cipher = EVP_aes_128_cbc();
        break;
      case 192:
        cipher = EVP_aes_192_cbc();
        break;
      case 256:
        cipher = EVP_aes_256_cbc();
        break;
      default:
        // Some test vectors intentionally give the wrong key size. Our API
        // requires the caller pick the sized CBC primitive, so these tests
        // aren't useful for us.
        EXPECT_FALSE(result.IsValid());
        return;
    }

    size_t tag_len = static_cast<size_t>(atoi(tag_size.c_str())) / 8;

    uint8_t out[16];
    bssl::UniquePtr<CMAC_CTX> ctx(CMAC_CTX_new());
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(CMAC_Init(ctx.get(), key.data(), key.size(), cipher, NULL));
    ASSERT_TRUE(CMAC_Update(ctx.get(), msg.data(), msg.size()));
    size_t out_len;
    ASSERT_TRUE(CMAC_Final(ctx.get(), out, &out_len));
    // Truncate the tag, if requested.
    out_len = std::min(out_len, tag_len);

    if (result.IsValid()) {
      EXPECT_EQ(Bytes(tag), Bytes(out, out_len));

      // Test the streaming API as well.
      ASSERT_TRUE(CMAC_Reset(ctx.get()));
      for (uint8_t b : msg) {
        ASSERT_TRUE(CMAC_Update(ctx.get(), &b, 1));
      }
      ASSERT_TRUE(CMAC_Final(ctx.get(), out, &out_len));
      out_len = std::min(out_len, tag_len);
      EXPECT_EQ(Bytes(tag), Bytes(out, out_len));
    } else {
      // Wycheproof's invalid tests assume the implementation internally does
      // the comparison, whereas our API only computes the tag. Check that
      // they're not equal, but these tests are mostly not useful for us.
      EXPECT_NE(Bytes(tag), Bytes(out, out_len));
    }
  });
}

static void RunCAVPTest(const char *path, const EVP_CIPHER *cipher,
                        bool is_3des) {
  FileTestGTest(path, [&](FileTest *t) {
    t->IgnoreAttribute("Count");
    t->IgnoreAttribute("Klen");
    std::string t_len, m_len, result;
    ASSERT_TRUE(t->GetAttribute(&t_len, "Tlen"));
    ASSERT_TRUE(t->GetAttribute(&m_len, "Mlen"));
    ASSERT_TRUE(t->GetAttribute(&result, "Result"));
    std::vector<uint8_t> key, msg, mac;
    if (is_3des) {
      std::vector<uint8_t> key2, key3;
      ASSERT_TRUE(t->GetBytes(&key, "Key1"));
      ASSERT_TRUE(t->GetBytes(&key2, "Key2"));
      ASSERT_TRUE(t->GetBytes(&key3, "Key3"));
      key.insert(key.end(), key2.begin(), key2.end());
      key.insert(key.end(), key3.begin(), key3.end());
    } else {
      ASSERT_TRUE(t->GetBytes(&key, "Key"));
    }
    ASSERT_TRUE(t->GetBytes(&msg, "Msg"));
    ASSERT_TRUE(t->GetBytes(&mac, "Mac"));

    // CAVP's uses a non-empty Msg attribute and zero Mlen for the empty string.
    if (atoi(m_len.c_str()) == 0) {
      msg.clear();
    } else {
      EXPECT_EQ(static_cast<size_t>(atoi(m_len.c_str())), msg.size());
    }

    size_t tag_len = static_cast<size_t>(atoi(t_len.c_str()));

    uint8_t out[16];
    bssl::UniquePtr<CMAC_CTX> ctx(CMAC_CTX_new());
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(CMAC_Init(ctx.get(), key.data(), key.size(), cipher, NULL));
    ASSERT_TRUE(CMAC_Update(ctx.get(), msg.data(), msg.size()));
    size_t out_len;
    ASSERT_TRUE(CMAC_Final(ctx.get(), out, &out_len));
    // Truncate the tag, if requested.
    out_len = std::min(out_len, tag_len);

    ASSERT_FALSE(result.empty());
    if (result[0] == 'P') {
      EXPECT_EQ(Bytes(mac), Bytes(out, out_len));

      // Test the streaming API as well.
      ASSERT_TRUE(CMAC_Reset(ctx.get()));
      for (uint8_t b : msg) {
        ASSERT_TRUE(CMAC_Update(ctx.get(), &b, 1));
      }
      ASSERT_TRUE(CMAC_Final(ctx.get(), out, &out_len));
      out_len = std::min(out_len, tag_len);
      EXPECT_EQ(Bytes(mac), Bytes(out, out_len));
    } else {
      // CAVP's invalid tests assume the implementation internally does the
      // comparison, whereas our API only computes the tag. Check that they're
      // not equal, but these tests are mostly not useful for us.
      EXPECT_NE(Bytes(mac), Bytes(out, out_len));
    }
  });
}

TEST(CMACTest, CAVPAES128) {
  RunCAVPTest("crypto/fipsmodule/cmac/cavp_aes128_cmac_tests.txt", EVP_aes_128_cbc(),
              false);
}

TEST(CMACTest, CAVPAES192) {
  RunCAVPTest("crypto/fipsmodule/cmac/cavp_aes192_cmac_tests.txt", EVP_aes_192_cbc(),
              false);
}

TEST(CMACTest, CAVPAES256) {
  RunCAVPTest("crypto/fipsmodule/cmac/cavp_aes256_cmac_tests.txt", EVP_aes_256_cbc(),
              false);
}

TEST(CMACTest, CAVP3DES) {
  RunCAVPTest("crypto/fipsmodule/cmac/cavp_3des_cmac_tests.txt", EVP_des_ede3_cbc(), true);
}
