// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#ifndef OPENSSL_HEADER_CRYPTO_TEST_UBE_TEST_H
#define OPENSSL_HEADER_CRYPTO_TEST_UBE_TEST_H

#include <gtest/gtest.h>

#include "../ube/internal.h"

class UbeBase {
  private:
    bool ube_detection_supported_ = false;

  public:
    void SetUp() {
      uint64_t current_generation_number = 0;
      if (CRYPTO_get_ube_generation_number(&current_generation_number) == 1) {
        ube_detection_supported_ = true;
      }
    }

    void TearDown() {
      disable_mocked_ube_detection_FOR_TESTING();
    }

    bool UbeIsSupported() const {
      return ube_detection_supported_;
    }

    void allowMockedUbe() const {
      allow_mocked_ube_detection_FOR_TESTING();
    }
};

#endif // OPENSSL_HEADER_CRYPTO_TEST_UBE_TEST_H
