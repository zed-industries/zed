// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <string.h>

#include <gtest/gtest.h>

#include <openssl/rand.h>

#include "abi_test.h"
#include "gtest_main.h"
#include "../internal.h"
#include "../ube/vm_ube_detect.h"


int main(int argc, char **argv) {
#if defined(OPENSSL_LINUX) && defined(AWSLC_VM_UBE_TESTING)
  if (1 != HAZMAT_init_sysgenid_file()) {
    abort();
  }
#endif

  testing::InitGoogleTest(&argc, argv);
  bssl::SetupGoogleTest();
  bool unwind_tests = true;
  for (int i = 1; i < argc; i++) {
    if (strcmp(argv[i], "--no_unwind_tests") == 0) {
      unwind_tests = false;
    }
  }

  if (unwind_tests) {
    abi_test::EnableUnwindTests();
  }

  return RUN_ALL_TESTS();
}
