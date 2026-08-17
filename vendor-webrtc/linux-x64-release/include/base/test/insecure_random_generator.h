// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_INSECURE_RANDOM_GENERATOR_H_
#define BASE_TEST_INSECURE_RANDOM_GENERATOR_H_

#include "base/rand_util.h"

namespace base::test {

// This class publicly inherits from base::InsecureRandomGenerator,
// whose constructor is private, to allow instantiation in tests.
class InsecureRandomGenerator : public base::InsecureRandomGenerator {
 public:
  InsecureRandomGenerator();
  ~InsecureRandomGenerator();
};

}  // namespace base::test

#endif  // BASE_TEST_INSECURE_RANDOM_GENERATOR_H_
