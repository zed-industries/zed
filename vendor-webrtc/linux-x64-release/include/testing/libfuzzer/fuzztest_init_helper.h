// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_FUZZTEST_INIT_HELPER_H_
#define TESTING_LIBFUZZER_FUZZTEST_INIT_HELPER_H_

namespace fuzztest_init_helper {

extern void (*initialization_function)(int argc, char** argv);
}

// If we're in a test suite which really has fuzztests,
// the above function pointer will have been populated with
// a function that knows how to initialize FuzzTests. Otherwise,
// it won't, to avoid bringing all of FuzzTests's dependencies
// into all the other Chromium test suites.
inline void MaybeInitFuzztest(int argc, char** argv) {
  if (fuzztest_init_helper::initialization_function) {
    fuzztest_init_helper::initialization_function(argc, argv);
  }
}

#endif  // TESTING_LIBFUZZER_FUZZTEST_INIT_HELPER_H_
