// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_TEST_GTEST_MAIN_H
#define OPENSSL_HEADER_CRYPTO_TEST_GTEST_MAIN_H

#include <stdio.h>
#include <stdlib.h>

#include <gtest/gtest.h>

#include <openssl/crypto.h>
#include <openssl/err.h>

#if defined(OPENSSL_WINDOWS)
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <winsock2.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#else
#include <signal.h>
#endif


BSSL_NAMESPACE_BEGIN

class ErrorTestEventListener : public testing::EmptyTestEventListener {
 public:
  ErrorTestEventListener() {}
  ~ErrorTestEventListener() override {}

  void OnTestEnd(const testing::TestInfo &test_info) override {
    if (test_info.result()->Failed()) {
      // The test failed. Print any errors left in the error queue.
      ERR_print_errors_fp(stdout);
    } else {
      // The test succeeded, so any failed operations are expected. Clear the
      // error queue without printing.
      ERR_clear_error();
    }
  }
};

// SetupGoogleTest should be called by the test runner after
// testing::InitGoogleTest has been called and before RUN_ALL_TESTS.
inline void SetupGoogleTest() {
  CRYPTO_library_init();

#if defined(OPENSSL_WINDOWS)
  // Initialize Winsock.
  WORD wsa_version = MAKEWORD(2, 2);
  WSADATA wsa_data;
  int wsa_err = WSAStartup(wsa_version, &wsa_data);
  if (wsa_err != 0) {
    fprintf(stderr, "WSAStartup failed: %d\n", wsa_err);
    exit(1);
  }
  if (wsa_data.wVersion != wsa_version) {
    fprintf(stderr, "Didn't get expected version: %x\n", wsa_data.wVersion);
    exit(1);
  }
#else
  // Some tests create pipes. We check return values, so avoid being killed by
  // |SIGPIPE|.
  signal(SIGPIPE, SIG_IGN);
#endif

  testing::UnitTest::GetInstance()->listeners().Append(
      new ErrorTestEventListener);
}

BSSL_NAMESPACE_END


#endif  // OPENSSL_HEADER_CRYPTO_TEST_GTEST_MAIN_H
