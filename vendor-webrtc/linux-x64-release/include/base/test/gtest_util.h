// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GTEST_UTIL_H_
#define BASE_TEST_GTEST_UTIL_H_

#include <string>
#include <utility>
#include <vector>

#include "base/check.h"
#include "base/debug/debugging_buildflags.h"
#include "build/build_config.h"
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
#if DCHECK_IS_ON() && defined(GTEST_HAS_DEATH_TEST) && \
    !BUILDFLAG(DCHECK_IS_CONFIGURABLE) && !BUILDFLAG(IS_ANDROID)

// EXPECT/ASSERT_DCHECK_DEATH tests verify that a DCHECK is hit ("Check failed"
// is part of the error message). Optionally you may specify part of the message
// to verify which DCHECK (or LOG(DFATAL)) is being hit.
#define EXPECT_DCHECK_DEATH(statement) EXPECT_DEATH(statement, "DCHECK failed")
#define EXPECT_DCHECK_DEATH_WITH(statement, msg) EXPECT_DEATH(statement, msg)
#define ASSERT_DCHECK_DEATH(statement) ASSERT_DEATH(statement, "DCHECK failed")
#define ASSERT_DCHECK_DEATH_WITH(statement, msg) ASSERT_DEATH(statement, msg)

#else

#define EXPECT_DCHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "DCHECK failed", )
#define EXPECT_DCHECK_DEATH_WITH(statement, msg) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, msg, )
#define ASSERT_DCHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "DCHECK failed", return)
#define ASSERT_DCHECK_DEATH_WITH(statement, msg) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, msg, return)

#endif  // DCHECK_IS_ON() && defined(GTEST_HAS_DEATH_TEST) &&
        // !BUILDFLAG(DCHECK_IS_CONFIGURABLE) && !BUILDFLAG(IS_ANDROID)

// As above, but for CHECK().
#if defined(GTEST_HAS_DEATH_TEST) && !BUILDFLAG(IS_ANDROID)

#if CHECK_WILL_STREAM()
#define EXPECT_CHECK_DEATH(statement) EXPECT_DEATH(statement, "Check failed")
#define EXPECT_CHECK_DEATH_WITH(statement, msg) EXPECT_DEATH(statement, msg)
#define ASSERT_CHECK_DEATH(statement) ASSERT_DEATH(statement, "Check failed")
#define EXPECT_NOTREACHED_DEATH(statement) \
  EXPECT_DEATH(statement, "NOTREACHED hit")
#define ASSERT_NOTREACHED_DEATH(statement) \
  ASSERT_DEATH(statement, "NOTREACHED hit")
#else
#define EXPECT_CHECK_DEATH(statement) EXPECT_DEATH(statement, "")
#define EXPECT_CHECK_DEATH_WITH(statement, msg) EXPECT_DEATH(statement, "")
#define ASSERT_CHECK_DEATH(statement) ASSERT_DEATH(statement, "")
#define EXPECT_NOTREACHED_DEATH(statement) EXPECT_DEATH(statement, "")
#define ASSERT_NOTREACHED_DEATH(statement) ASSERT_DEATH(statement, "")
#endif  // CHECK_WILL_STREAM()

#else  // defined(GTEST_HAS_DEATH_TEST) && !BUILDFLAG(IS_ANDROID)

// Note GTEST_UNSUPPORTED_DEATH_TEST takes a |regex| only to see whether it is a
// valid regex. It is never evaluated.
#define EXPECT_CHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )
#define EXPECT_CHECK_DEATH_WITH(statement, msg) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )
#define ASSERT_CHECK_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", return)
#define EXPECT_NOTREACHED_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )
#define ASSERT_NOTREACHED_DEATH(statement) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", return)

#endif  // defined(GTEST_HAS_DEATH_TEST) && !BUILDFLAG(IS_ANDROID)

// `BASE_EXPECT_DEATH` is similar to gtest's `EXPECT_DEATH_IF_SUPPORTED`. It
// takes into account that Android does not support them.
#if defined(GTEST_HAS_DEATH_TEST) && !BUILDFLAG(IS_ANDROID)

#define BASE_EXPECT_DEATH EXPECT_DEATH

#else  // defined(GTEST_HAS_DEATH_TEST) && !BUILDFLAG(IS_ANDROID)

#define BASE_EXPECT_DEATH(statement, matcher) \
  GTEST_UNSUPPORTED_DEATH_TEST(statement, "", )

#endif  // defined(GTEST_HAS_DEATH_TEST) && !BUILDFLAG(IS_ANDROID)

namespace base {

class FilePath;

struct TestIdentifier {
  TestIdentifier();
  TestIdentifier(const TestIdentifier& other);
  TestIdentifier& operator=(const TestIdentifier& other);

  std::string test_case_name;
  std::string test_name;
  std::string file;
  int line;
};

// Constructs a full test name given a test case name and a test name,
// e.g. for test case "A" and test name "B" returns "A.B".
std::string FormatFullTestName(const std::string& test_case_name,
                               const std::string& test_name);

// Returns the full test name with the "DISABLED_" prefix stripped out.
// e.g. for the full test names "A.DISABLED_B", "DISABLED_A.B", and
// "DISABLED_A.DISABLED_B", returns "A.B".
std::string TestNameWithoutDisabledPrefix(const std::string& full_test_name);

// Returns a vector of gtest-based tests compiled into
// current executable.
std::vector<TestIdentifier> GetCompiledInTests();

// Writes the list of gtest-based tests compiled into
// current executable as a JSON file. Returns true on success.
[[nodiscard]] bool WriteCompiledInTestsToFile(const FilePath& path);

// Reads the list of gtest-based tests from |path| into |output|.
// Returns true on success.
[[nodiscard]] bool ReadTestNamesFromFile(const FilePath& path,
                                         std::vector<TestIdentifier>* output);

}  // namespace base

#endif  // BASE_TEST_GTEST_UTIL_H_
