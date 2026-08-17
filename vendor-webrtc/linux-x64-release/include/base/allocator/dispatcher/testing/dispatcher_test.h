// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_TESTING_DISPATCHER_TEST_H_
#define BASE_ALLOCATOR_DISPATCHER_TESTING_DISPATCHER_TEST_H_

#include "testing/gtest/include/gtest/gtest.h"

namespace base::allocator::dispatcher::testing {

// DispatcherTest provides some common initialization which most of the
// unittests of the dispatcher require. DispatcherTest should not be used
// directly. Instead, derive your test fixture from it.
struct DispatcherTest : public ::testing::Test {
  // Perform some commonly required initialization, at them moment
  // - Initialize the TLS slot for the ReentryGuard
  DispatcherTest();

 protected:
  // Protected d'tor only to prevent direct usage of this class.
  ~DispatcherTest() override;
};

}  // namespace base::allocator::dispatcher::testing

#endif  // BASE_ALLOCATOR_DISPATCHER_TESTING_DISPATCHER_TEST_H_
