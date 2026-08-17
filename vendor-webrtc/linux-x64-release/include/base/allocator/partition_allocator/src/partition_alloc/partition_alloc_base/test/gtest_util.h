// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_TEST_GTEST_UTIL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_TEST_GTEST_UTIL_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/check.h"
#include "testing/gtest/include/gtest/gtest.h"

// EXPECT/ASSERT_DCHECK_DEATH is intended to replace EXPECT/ASSERT_DEBUG_DEATH
// when the death is expected to be caused by a DCHECK. Contrary to
// EXPECT/ASSERT_DEBUG_DEATH however, it doesn't execute the statement in non-
// dcheck builds as DCHECKs are intended to catch things that should never
// happen and as such executing the statement results in undefined behavior
// (|statement| is compiled in unsupported configurations nonetheless).
// DCHECK_IS_CONFIGURABLE is excluded from DCHECK_DEATH because it's non-FATAL
// by default and there are no known tests that configure a FATAL level. If this
// gets used from FATAL contexts under DCHECK_IS_CONFIGURABLE this may need to
// be updated to look at LOGGING_DCHECK's current severity level.
// Death tests misbehave on Android.
#if PA_BUILDFLAG(DCHECKS_ARE_ON) && defined(GTEST_HAS_DEATH_TEST) && \
    !PA_BUILDFLAG(DCHECK_IS_CONFIGURABLE) && !PA_BUILDFLAG(IS_ANDROID)

// EXPECT/ASSERT_DCHECK_DEATH tests verify that a DCHECK is hit ("Check failed"
// is part of the error message). Optionally you may specify part of the message
// to verify which DCHECK (or LOG(DFATAL)) is being hit.
#define PA_EXPECT_DCHECK_DEATH(statement) \
  EXPECT_DEATH(statement, "Check failed")
#define PA_EXPECT_DCHECK_DEATH_WITH(statement, msg) EXPECT_DEATH(statement, msg)
#define PA_ASSERT_DCHECK_DEATH(statement) \
  ASSERT_DEATH(statement, "Check failed")
#define PA_ASSERT_DCHECK_DEATH_WITH(statement, msg) ASSERT_DEATH(statement, msg)

#else

#define PA_EXPECT_DCHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "Check failed", )
#define PA_EXPECT_DCHECK_DEATH_WITH(statement, msg) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, msg, )
#define PA_ASSERT_DCHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "Check failed", return)
#define PA_ASSERT_DCHECK_DEATH_WITH(statement, msg) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, msg, return)

#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON) && defined(GTEST_HAS_DEATH_TEST) &&
        // !PA_BUILDFLAG(DCHECK_IS_CONFIGURABLE) && !PA_BUILDFLAG(IS_ANDROID)

// As above, but for CHECK().
#if defined(GTEST_HAS_DEATH_TEST) && !PA_BUILDFLAG(IS_ANDROID)

#if PA_BASE_CHECK_WILL_STREAM()
#define PA_EXPECT_CHECK_DEATH(statement) EXPECT_DEATH(statement, "Check failed")
#define PA_EXPECT_CHECK_DEATH_WITH(statement, msg) EXPECT_DEATH(statement, msg)
#define PA_ASSERT_CHECK_DEATH(statement) ASSERT_DEATH(statement, "Check failed")
#define PA_EXPECT_NOTREACHED_DEATH(statement) \
  EXPECT_DEATH(statement, "NOTREACHED hit")
#define PA_ASSERT_NOTREACHED_DEATH(statement) \
  ASSERT_DEATH(statement, "NOTREACHED hit")
#else
#define PA_EXPECT_CHECK_DEATH(statement) EXPECT_DEATH(statement, "")
#define PA_EXPECT_CHECK_DEATH_WITH(statement, msg) EXPECT_DEATH(statement, "")
#define PA_ASSERT_CHECK_DEATH(statement) ASSERT_DEATH(statement, "")
#define PA_EXPECT_NOTREACHED_DEATH(statement) EXPECT_DEATH(statement, "")
#define PA_ASSERT_NOTREACHED_DEATH(statement) ASSERT_DEATH(statement, "")
#endif  // PA_BASE_CHECK_WILL_STREAM()

#else  // defined(GTEST_HAS_DEATH_TEST) && !PA_BUILDFLAG(IS_ANDROID)

// Note GTEST_UNSUPPORTED_DEATH_TEST takes a |regex| only to see whether it is a
// valid regex. It is never evaluated.
#define PA_EXPECT_CHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )
#define PA_EXPECT_CHECK_DEATH_WITH(statement, msg) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )
#define PA_ASSERT_CHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", return)
#define PA_EXPECT_NOTREACHED_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )
#define PA_ASSERT_NOTREACHED_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", return)

#endif  // defined(GTEST_HAS_DEATH_TEST) && !PA_BUILDFLAG(IS_ANDROID)

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_TEST_GTEST_UTIL_H_
