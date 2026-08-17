// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_LAUNCHER_TEST_RESULT_H_
#define BASE_TEST_LAUNCHER_TEST_RESULT_H_

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "base/threading/platform_thread.h"
#include "base/time/time.h"

namespace base {

// Structure contains result of a single EXPECT/ASSERT/SUCCESS/SKIP.
struct TestResultPart {
  enum Type {
    kSuccess,          // SUCCESS
    kNonFatalFailure,  // EXPECT
    kFatalFailure,     // ASSERT
    kSkip,             // SKIP
  };
  Type type;

  TestResultPart();
  ~TestResultPart();

  TestResultPart(const TestResultPart& other);
  TestResultPart(TestResultPart&& other);
  TestResultPart& operator=(const TestResultPart& other);
  TestResultPart& operator=(TestResultPart&& other);

  // Convert type to string and back.
  static bool TypeFromString(const std::string& str, Type* type);
  std::string TypeAsString() const;

  // Filename and line of EXPECT/ASSERT.
  std::string file_name;
  int line_number;

  // Message without stacktrace, etc.
  std::string summary;

  // Complete message.
  std::string message;
};

// Manually reported additional test result.
// A TestResult may have any number of SubTestResults.
struct SubTestResult {
  SubTestResult();

  SubTestResult(const SubTestResult& other);
  SubTestResult& operator=(const SubTestResult& other);
  SubTestResult(SubTestResult&& other) noexcept;
  SubTestResult& operator=(SubTestResult&& other) noexcept;

  ~SubTestResult();

  // Fully qualified name containing `classname`, `name`, and `subname`.
  // Constructed to match the regular expression for GTest names.
  std::string FullName() const;

  // GTest test suite name.
  std::string classname;

  // GTest test name.
  std::string name;

  // Custom third name field defined by test author.
  std::string subname;

  // Failure message that is passed along to CI. If nullopt, this SubTestResult
  // is interpreted as successful.
  std::optional<std::string> failure_message;
};

// Structure containing result of a single test.
struct TestResult {
  enum Status {
    TEST_UNKNOWN,           // Status not set.
    TEST_SUCCESS,           // Test passed.
    TEST_FAILURE,           // Assertion failure (e.g. EXPECT_TRUE, not DCHECK).
    TEST_FAILURE_ON_EXIT,   // Passed but executable exit code was non-zero.
    TEST_TIMEOUT,           // Test timed out and was killed.
    TEST_CRASH,             // Test crashed (includes CHECK/DCHECK failures).
    TEST_SKIPPED,           // Test skipped (not run at all).
    TEST_EXCESSIVE_OUTPUT,  // Test exceeded output limit.
    TEST_NOT_RUN,           // Test has not yet been run.
  };

  TestResult();
  ~TestResult();

  TestResult(const TestResult& other);
  TestResult(TestResult&& other);
  TestResult& operator=(const TestResult& other);
  TestResult& operator=(TestResult&& other);

  // Returns the test status as string (e.g. for display).
  std::string StatusAsString() const;

  // Returns the test name (e.g. "B" for "A.B").
  std::string GetTestName() const;

  // Returns the test case name (e.g. "A" for "A.B").
  std::string GetTestCaseName() const;

  // Add link in the xml output.
  // See more in gtest_links.h.
  void AddLink(const std::string& name, const std::string& url);

  // Add tag in the xml output.
  // See more in gtest_tags.h.
  void AddTag(const std::string& name, const std::string& value);

  // Add an additional test result.
  void AddSubTestResult(SubTestResult sub_test_result);

  // Add property in the xml output.
  void AddProperty(const std::string& name, const std::string& value);

  // Returns true if the test has completed (i.e. the test binary exited
  // normally, possibly with an exit code indicating failure, but didn't crash
  // or time out in the middle of the test).
  bool completed() const {
    return status == TEST_SUCCESS || status == TEST_FAILURE ||
           status == TEST_FAILURE_ON_EXIT || status == TEST_EXCESSIVE_OUTPUT;
  }

  // Full name of the test (e.g. "A.B").
  std::string full_name;

  Status status;

  // Start time of child test process, the field is optional the test could be
  // NOT_RUN.
  std::optional<base::Time> timestamp;

  // Thread id of the runner that launching the child process, which is also
  // recorded in TestLauncherTracer.
  std::optional<base::PlatformThreadId> thread_id;

  // The process num of child process launched it's recorded as event name in
  // TestLauncherTracer.
  // It's used instead of process id to distinguish processes that process id
  // might be reused by OS.
  std::optional<int> process_num;

  // Time it took to run the test.
  base::TimeDelta elapsed_time;

  // Output of just this test (optional).
  std::string output_snippet;

  // Information about failed expectations.
  std::vector<TestResultPart> test_result_parts;

  // The key is link name.
  std::map<std::string, std::string> links;

  // The key is property name.
  std::map<std::string, std::string> properties;

  // The key is tag name.
  std::map<std::string, std::vector<std::string>> tags;

  // Collection of SubTestResults reported within this test.
  std::vector<SubTestResult> sub_test_results;
};

}  // namespace base

#endif  // BASE_TEST_LAUNCHER_TEST_RESULT_H_
