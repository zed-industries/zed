// Copyright (c) 2021, Google Inc.
// SPDX-License-Identifier: ISC

#include <gtest/gtest.h>

#include <openssl/cipher.h>
#include <openssl/digest.h>
#include <openssl/evp.h>


// Node.js assumes every cipher in |EVP_CIPHER_do_all_sorted| is accessible via
// |EVP_get_cipherby*|.
TEST(EVPTest, CipherDoAll) {
  EVP_CIPHER_do_all_sorted(
      [](const EVP_CIPHER *cipher, const char *name, const char *unused,
         void *arg) {
        SCOPED_TRACE(name);
        EXPECT_EQ(cipher, EVP_get_cipherbyname(name));
        EXPECT_EQ(cipher, EVP_get_cipherbynid(EVP_CIPHER_nid(cipher)));
      },
      nullptr);
}

// Node.js assumes every digest in |EVP_MD_do_all_sorted| is accessible via
// |EVP_get_digestby*|.
TEST(EVPTest, MDDoAll) {
  EVP_MD_do_all_sorted(
      [](const EVP_MD *md, const char *name, const char *unused, void *arg) {
        SCOPED_TRACE(name);
        EXPECT_EQ(md, EVP_get_digestbyname(name));
        EXPECT_EQ(md, EVP_get_digestbynid(EVP_MD_nid(md)));
      },
      nullptr);
}
