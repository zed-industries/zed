/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_TEST_FUNCTION_EQUIVALENCE_TEST_H_
#define AOM_TEST_FUNCTION_EQUIVALENCE_TEST_H_

#include <ostream>

#include "gtest/gtest.h"
#include "test/acm_random.h"
#include "test/util.h"

using libaom_test::ACMRandom;

namespace libaom_test {
// Base class for tests that compare 2 implementations of the same function
// for equivalence. The template parameter should be pointer to a function
// that is being tested.
//
// The test takes a 3-parameters encapsulating struct 'FuncParam', containing:
//   - Pointer to reference function
//   - Pointer to tested function
//   - Integer bit depth (default to 0).
//
// These values are then accessible in the tests as member of params_:
// params_.ref_func, params_.tst_func, and params_.bit_depth.
//

template <typename T>
struct FuncParam {
  FuncParam(T ref = nullptr, T tst = nullptr, int depth = 0)
      : ref_func(ref), tst_func(tst), bit_depth(depth) {}
  T ref_func;
  T tst_func;
  int bit_depth;
};

template <typename T>
std::ostream &operator<<(std::ostream &os, const FuncParam<T> &p) {
  return os << "bit_depth:" << p.bit_depth
            << " function:" << reinterpret_cast<const void *>(p.ref_func)
            << " function:" << reinterpret_cast<const void *>(p.tst_func);
}

template <typename T>
class FunctionEquivalenceTest : public ::testing::TestWithParam<FuncParam<T> > {
 public:
  FunctionEquivalenceTest() : rng_(ACMRandom::DeterministicSeed()) {}

  ~FunctionEquivalenceTest() override = default;

  void SetUp() override { params_ = this->GetParam(); }

 protected:
  ACMRandom rng_;
  FuncParam<T> params_;
};

}  // namespace libaom_test
#endif  // AOM_TEST_FUNCTION_EQUIVALENCE_TEST_H_
