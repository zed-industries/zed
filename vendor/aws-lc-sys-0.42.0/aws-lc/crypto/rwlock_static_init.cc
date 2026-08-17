// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// This test attempts to setup a potential race condition that can cause the
// first use of an rwlock to fail when initialized using
// PTHREAD_RWLOCK_INITIALIZER.
// See: https://sourceforge.net/p/mingw-w64/bugs/883/

#include <openssl/rand.h>

#include <cstdint>
#include <iostream>
#include <thread>
#include <cassert>
#include <cstdlib>

static void thread_task_rand(bool *myFlag) {
  uint8_t buf[16];
  if(1 == RAND_bytes(buf, sizeof(buf))) {
    *myFlag = true;
  }
}

int main(int _argc, char** _argv) {
  constexpr size_t kNumThreads = 16;
  bool myFlags[kNumThreads] = {};
  std::thread myThreads[kNumThreads];

  for (size_t i = 0; i < kNumThreads; i++) {
    bool* myFlag = &myFlags[i];
    myThreads[i] = std::thread(thread_task_rand, myFlag);
  }
  for (size_t i = 0; i < kNumThreads; i++) {
    myThreads[i].join();
    if(!myFlags[i]) {
      std::cerr << "Thread " << i << " failed." << std::endl;
      exit(1);
      return 1;
    }
  }
  std::cout << "PASS" << std::endl;
  return 0;
}
