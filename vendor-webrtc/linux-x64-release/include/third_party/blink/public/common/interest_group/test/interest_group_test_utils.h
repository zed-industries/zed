// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_TEST_INTEREST_GROUP_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_TEST_INTEREST_GROUP_TEST_UTILS_H_

#include "third_party/blink/public/common/interest_group/interest_group.h"

namespace blink {

// Equality and non-equality tests.
//
// IgExpectEqualsForTesting() will use EXPECT_EQ() on all the fields and
// subfields (such as ad fields) in the interest group -- this way, detailed
// gtest expectation failures will show which fields don't match.
//
// IgExpectNotEqualsForTesting() will raise a single expectation failure if all
// the fields are the same.
//
// If you'd like to ASSERT_*() on these checks, you can
// ASSERT_FALSE(Test::HasFailure()) after the check.
void IgExpectEqualsForTesting(const blink::InterestGroup& actual,
                              const blink::InterestGroup& expected);
void IgExpectNotEqualsForTesting(const blink::InterestGroup& actual,
                                 const blink::InterestGroup& not_expected);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_TEST_INTEREST_GROUP_TEST_UTILS_H_
