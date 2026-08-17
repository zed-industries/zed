// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_OCMOCK_GTEST_SUPPORT_H_
#define THIRD_PARTY_OCMOCK_GTEST_SUPPORT_H_

#include "testing/gtest/include/gtest/gtest.h"

@class OCMockObject;

namespace testing {
namespace internal {
bool VerifyOCMock(OCMockObject* mock, const char* file, int line);
}  // namespace internal
}  // namespace testing

// Calls -verify of the mock and traps the Objective-C exception that is
// generated, adding it to the gtest failures and returning true/false
// for if there was an exception.  The result should be used in normal
// gtest EXECPT_TRUE/ASSERT_TRUE fashion.
//
// So code that would do:
//
//     id mockFoo = [OCMockObject mockForClass:[Foo class]];
//     ...
//     [mockFoo verify];
//
// Should instead do:
//
//     id mockFoo = [OCMockObject mockForClass:[Foo class]];
//     ...
//     EXPECT_OCMOCK_VERIFY(mockFoo);
//
#define EXPECT_OCMOCK_VERIFY(m) \
    EXPECT_TRUE(testing::internal::VerifyOCMock((m), __FILE__, __LINE__))
#define ASSERT_OCMOCK_VERIFY(m) \
    ASSERT_TRUE(testing::internal::VerifyOCMock((m), __FILE__, __LINE__))

#endif  // THIRD_PARTY_OCMOCK_GTEST_SUPPORT_H_
