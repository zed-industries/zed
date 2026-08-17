/* Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0 OR ISC
 */

#ifndef HEADER_SSL_TRANSFER
#define HEADER_SSL_TRANSFER

#include <string>

#include <openssl/ssl.h>

#include "test_config.h"

struct SSLTransfer {
 public:
  SSLTransfer();

  // MarkTest generate a mark(err msg) if a test case of runner.go can be used to test
  // SSL transfer when |config->check_ssl_transfer| is true.
  void MarkTest(const TestConfig *config, const SSL *ssl);

  // ResetSSL resets |in| with a newly allocated SSL when |config->do_ssl_transfer| is true.
  // The newly allocated SSL has states transferred from the previous one hold by |in|.
  bool ResetSSL(const TestConfig *config, bssl::UniquePtr<SSL> *in);

  // MarkOrReset wraps |MarkTest| and |ResetSSL|.
  bool MarkOrReset(const TestConfig *config, bssl::UniquePtr<SSL> *in);

  // IsSupported returns true when |in| can be transferred. Otherwise, returns false.
  bool IsSupported(const SSL *in);
};

#endif  // HEADER_SSL_TRANSFER
