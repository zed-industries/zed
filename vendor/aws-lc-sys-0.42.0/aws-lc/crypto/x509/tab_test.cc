// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project 1999.
// Copyright (c) 1999 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#if !defined(BORINGSSL_SHARED_LIBRARY)

#include <gtest/gtest.h>

#include <openssl/x509.h>

#include "../internal.h"
#include "ext_dat.h"

// Check ext_data.h is correct.
TEST(X509V3Test, TabTest) {
  EXPECT_EQ(OPENSSL_ARRAY_SIZE(standard_exts), STANDARD_EXTENSION_COUNT);
  for (size_t i = 1; i < OPENSSL_ARRAY_SIZE(standard_exts); i++) {
    SCOPED_TRACE(i);
    EXPECT_LT(standard_exts[i-1]->ext_nid, standard_exts[i]->ext_nid);
  }
}

#endif  // !BORINGSSL_SHARED_LIBRARY
