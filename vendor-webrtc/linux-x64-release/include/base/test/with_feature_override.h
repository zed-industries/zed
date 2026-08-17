// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_WITH_FEATURE_OVERRIDE_H_
#define BASE_TEST_WITH_FEATURE_OVERRIDE_H_

#include "base/feature_list.h"
#include "base/test/scoped_feature_list.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {
namespace test {

#define INSTANTIATE_FEATURE_OVERRIDE_TEST_SUITE(test_name) \
  INSTANTIATE_TEST_SUITE_P(All, test_name, testing::Values(false, true))

// Base class for a test fixture that enables running tests twice, once with a
// feature enabled and once with it disabled. Must be the first base class of
// the test fixture to take effect during its construction. If
// WithFeatureOverride is added as a parent to an existing test fixture
// all of its existing tests need to be migrated to TEST_P.
//
// Example usage:
//
//  class MyTest : public base::test::WithFeatureOverride, public testing::Test
//  {
//   public:
//    MyTest() : base::test::WithFeatureOverride(kMyFeature){}
//  };
//
//  TEST_P(MyTest, FooBar) {
//    This will run with both the kMyFeature enabled and disabled.
//  }
//
//  INSTANTIATE_FEATURE_OVERRIDE_TEST_SUITE(MyTest);

class WithFeatureOverride : public testing::WithParamInterface<bool> {
 public:
  explicit WithFeatureOverride(const base::Feature& feature);
  ~WithFeatureOverride();

  WithFeatureOverride(const WithFeatureOverride&) = delete;
  WithFeatureOverride& operator=(const WithFeatureOverride&) = delete;

  // Use to know if the configured feature provided in the constructor is
  // enabled or not.
  bool IsParamFeatureEnabled() const;

 private:
  base::test::ScopedFeatureList scoped_feature_list_;
};

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_WITH_FEATURE_OVERRIDE_H_
