// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <string.h>

#include <string>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/base64.h>
#include <openssl/crypto.h>
#include <openssl/err.h>

#include "../internal.h"
#include "../test/test_util.h"


enum encoding_relation {
  // canonical indicates that the encoding is the expected encoding of the
  // input.
  canonical,
  // valid indicates that the encoding is /a/ valid encoding of the input, but
  // need not be the canonical one.
  valid,
  // invalid indicates that the encoded data is valid.
  invalid,
};

struct Base64TestVector {
  enum encoding_relation relation;
  const char *decoded;
  const char *encoded;
};

// Test vectors from RFC 4648.
static const Base64TestVector kTestVectors[] = {
    {canonical, "", ""},
    {canonical, "f", "Zg==\n"},
    {canonical, "fo", "Zm8=\n"},
    {canonical, "foo", "Zm9v\n"},
    {canonical, "foob", "Zm9vYg==\n"},
    {canonical, "fooba", "Zm9vYmE=\n"},
    {canonical, "foobar", "Zm9vYmFy\n"},
    {valid, "foobar", "Zm9vYmFy\n\n"},
    {valid, "foobar", " Zm9vYmFy\n\n"},
    {valid, "foobar", " Z m 9 v Y m F y\n\n"},
    {invalid, "", "Zm9vYmFy=\n"},
    {invalid, "", "Zm9vYmFy==\n"},
    {invalid, "", "Zm9vYmFy===\n"},
    {invalid, "", "Z"},
    {invalid, "", "Z\n"},
    {invalid, "", "ab!c"},
    {invalid, "", "ab=c"},
    {invalid, "", "abc"},

    {canonical, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eA==\n"},
    {valid, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eA\n==\n"},
    {valid, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eA=\n=\n"},
    {invalid, "",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eA=\n==\n"},
    {canonical, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4\neHh4eHh"
     "4eHh4eHh4\n"},
    {canonical,
     "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4\neHh4eHh"
     "4eHh4eHh4eHh4eA==\n"},
    {valid, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh\n4eHh4eHh"
     "4eHh4eHh4eHh4eA==\n"},
    {valid, "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4e"
     "Hh4eHh4eHh4eA==\n"},
    {invalid, "",
     "eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eA=="
     "\neHh4eHh4eHh4eHh4eHh4eHh4\n"},

    // A '-' has traditionally been treated as the end of the data by OpenSSL
    // and anything following would be ignored. BoringSSL does not accept this
    // non-standard extension.
    {invalid, "", "Zm9vYmFy-anythinggoes"},
    {invalid, "", "Zm9vYmFy\n-anythinggoes"},

    // CVE-2015-0292
    {invalid, "",
     "ZW5jb2RlIG1lCg==========================================================="
     "=======\n"},
};

class Base64Test : public testing::TestWithParam<Base64TestVector> {};

INSTANTIATE_TEST_SUITE_P(All, Base64Test, testing::ValuesIn(kTestVectors));

// RemoveNewlines returns a copy of |in| with all '\n' characters removed.
static std::string RemoveNewlines(const char *in) {
  std::string ret;
  const size_t in_len = strlen(in);

  for (size_t i = 0; i < in_len; i++) {
    if (in[i] != '\n') {
      ret.push_back(in[i]);
    }
  }

  return ret;
}

TEST_P(Base64Test, EncodeBlock) {
  const Base64TestVector &t = GetParam();
  if (t.relation != canonical) {
    return;
  }

  const size_t decoded_len = strlen(t.decoded);
  size_t max_encoded_len;
  ASSERT_TRUE(EVP_EncodedLength(&max_encoded_len, decoded_len));

  std::vector<uint8_t> out_vec(max_encoded_len);
  uint8_t *out = out_vec.data();
  size_t len = EVP_EncodeBlock(out, (const uint8_t *)t.decoded, decoded_len);

  std::string encoded(RemoveNewlines(t.encoded));
  EXPECT_EQ(Bytes(encoded), Bytes(out, len));
}

TEST_P(Base64Test, DecodeBase64) {
  const Base64TestVector &t = GetParam();
  if (t.relation == valid) {
    // The non-canonical encodings will generally have odd whitespace etc
    // that |EVP_DecodeBase64| will reject.
    return;
  }

  const std::string encoded(RemoveNewlines(t.encoded));
  std::vector<uint8_t> out_vec(encoded.size());
  uint8_t *out = out_vec.data();

  size_t len;
  int ok = EVP_DecodeBase64(out, &len, out_vec.size(),
                            (const uint8_t *)encoded.data(), encoded.size());

  if (t.relation == invalid) {
    EXPECT_FALSE(ok);
  } else if (t.relation == canonical) {
    ASSERT_TRUE(ok);
    EXPECT_EQ(Bytes(t.decoded), Bytes(out, len));
  }
}

TEST_P(Base64Test, DecodeBlock) {
  const Base64TestVector &t = GetParam();
  if (t.relation != canonical) {
    return;
  }

  std::string encoded(RemoveNewlines(t.encoded));

  std::vector<uint8_t> out_vec(encoded.size());
  uint8_t *out = out_vec.data();

  // Test that the padding behavior of the deprecated API is preserved.
  int ret =
      EVP_DecodeBlock(out, (const uint8_t *)encoded.data(), encoded.size());
  ASSERT_GE(ret, 0);
  // EVP_DecodeBlock should ignore padding.
  ASSERT_EQ(0, ret % 3);
  size_t expected_len = strlen(t.decoded);
  if (expected_len % 3 != 0) {
    ret -= 3 - (expected_len % 3);
  }
  EXPECT_EQ(Bytes(t.decoded), Bytes(out, static_cast<size_t>(ret)));
}

TEST_P(Base64Test, EncodeDecode) {
  const Base64TestVector &t = GetParam();

  EVP_ENCODE_CTX ctx;
  const size_t decoded_len = strlen(t.decoded);

  if (t.relation == canonical) {
    size_t max_encoded_len;
    ASSERT_TRUE(EVP_EncodedLength(&max_encoded_len, decoded_len));

    // EVP_EncodeUpdate will output new lines every 64 bytes of output so we
    // need slightly more than |EVP_EncodedLength| returns. */
    max_encoded_len += (max_encoded_len + 63) >> 6;
    std::vector<uint8_t> out_vec(max_encoded_len);
    uint8_t *out = out_vec.data();

    EVP_EncodeInit(&ctx);

    int out_len;
    int ret = EVP_EncodeUpdate(&ctx, out, &out_len,
                               reinterpret_cast<const uint8_t *>(t.decoded),
                               decoded_len);
    EXPECT_EQ(ret, (strlen(t.encoded) > 0 ? 1 : 0));
    size_t total = out_len;

    EVP_EncodeFinal(&ctx, out + total, &out_len);
    total += out_len;

    EXPECT_EQ(Bytes(t.encoded), Bytes(out, total));
  }

  std::vector<uint8_t> out_vec(strlen(t.encoded));
  uint8_t *out = out_vec.data();

  EVP_DecodeInit(&ctx);
  int out_len;
  size_t total = 0;
  int ret = EVP_DecodeUpdate(&ctx, out, &out_len,
                             reinterpret_cast<const uint8_t *>(t.encoded),
                             strlen(t.encoded));
  if (ret != -1) {
    total = out_len;
    ret = EVP_DecodeFinal(&ctx, out + total, &out_len);
    total += out_len;
  }

  switch (t.relation) {
    case canonical:
    case valid:
      ASSERT_NE(-1, ret);
      EXPECT_EQ(Bytes(t.decoded), Bytes(out, total));
      break;

    case invalid:
      EXPECT_EQ(-1, ret);
      break;
  }
}

TEST_P(Base64Test, DecodeUpdateStreaming) {
  const Base64TestVector &t = GetParam();
  if (t.relation == invalid) {
    return;
  }

  const size_t encoded_len = strlen(t.encoded);

  std::vector<uint8_t> out(encoded_len);

  for (size_t chunk_size = 1; chunk_size <= encoded_len; chunk_size++) {
    SCOPED_TRACE(chunk_size);
    size_t out_len = 0;
    EVP_ENCODE_CTX ctx;
    EVP_DecodeInit(&ctx);

    for (size_t i = 0; i < encoded_len;) {
      size_t todo = encoded_len - i;
      if (todo > chunk_size) {
        todo = chunk_size;
      }

      int bytes_written;
      int ret = EVP_DecodeUpdate(
          &ctx, out.data() + out_len, &bytes_written,
          reinterpret_cast<const uint8_t *>(t.encoded + i), todo);
      i += todo;

      switch (ret) {
        case -1:
          FAIL() << "EVP_DecodeUpdate failed";
        case 0:
          out_len += bytes_written;
          if (i == encoded_len ||
              (i + 1 == encoded_len && t.encoded[i] == '\n') ||
              // If there was an '-' in the input (which means “EOF”) then
              // this loop will continue to test that |EVP_DecodeUpdate| will
              // ignore the remainder of the input.
              strchr(t.encoded, '-') != nullptr) {
            break;
          }

          FAIL()
              << "EVP_DecodeUpdate returned zero before end of encoded data.";
        case 1:
          out_len += bytes_written;
          break;
        default:
          FAIL() << "Invalid return value " << ret;
      }
    }

    int bytes_written;
    int ret = EVP_DecodeFinal(&ctx, out.data() + out_len, &bytes_written);
    ASSERT_NE(ret, -1);
    out_len += bytes_written;

    EXPECT_EQ(Bytes(t.decoded), Bytes(out.data(), out_len));
  }
}
