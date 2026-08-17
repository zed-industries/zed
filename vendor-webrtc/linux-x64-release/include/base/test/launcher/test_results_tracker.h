// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_LAUNCHER_TEST_RESULTS_TRACKER_H_
#define BASE_TEST_LAUNCHER_TEST_RESULTS_TRACKER_H_

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/test/launcher/test_result.h"
#include "base/threading/thread_checker.h"

namespace base {

class CommandLine;
class FilePath;

// A helper class to output results.
// Note: as currently XML is the only supported format by gtest, we don't
// check output format (e.g. "xml:" prefix) here and output an XML file
// unconditionally.
// Note: we don't output per-test-case or total summary info like
// total failed_test_count, disabled_test_count, elapsed_time and so on.
// Only each test (testcase element in the XML) will have the correct
// failed/disabled/elapsed_time information. Each test won't include
// detailed failure messages either.
class TestResultsTracker {
 public:
  TestResultsTracker();

  TestResultsTracker(const TestResultsTracker&) = delete;
  TestResultsTracker& operator=(const TestResultsTracker&) = delete;

  ~TestResultsTracker();

  // Initialize the result tracker. Must be called exactly once before
  // calling any other methods. Returns true on success.
  [[nodiscard]] bool Init(const CommandLine& command_line);

  // Called when a test iteration is starting.
  void OnTestIterationStarting();

  // Adds |test_name| to the set of discovered tests (this includes all tests
  // present in the executable, not necessarily run).
  void AddTest(const std::string& test_name);

  // Adds |test_name| to the set of disabled tests.
  void AddDisabledTest(const std::string& test_name);

  // Adds location for the |test_name|. Locations are required for all tests run
  // in a given shard, by both the TestLauncher and its delegate.
  void AddTestLocation(const std::string& test_name,
                       const std::string& file,
                       int line);

  // Adds placeholder for the |test_name|. Placeholders are required for all
  // tests that are expected to produce results in a given shard.
  void AddTestPlaceholder(const std::string& test_name);

  // Adds |result| to the stored test results.
  void AddTestResult(const TestResult& result);

  // Adds to the current iteration the fact that |count| items were leaked by
  // one or more tests in |test_names| in its temporary directory.
  void AddLeakedItems(int count, const std::vector<std::string>& test_names);

  // Even when no iterations have occurred, we still want to generate output
  // data with "NOTRUN" status for each test. This method generates a
  // placeholder iteration. The first iteration will overwrite the data in the
  // placeholder iteration.
  void GeneratePlaceholderIteration();

  // Prints a summary of current test iteration to stdout.
  void PrintSummaryOfCurrentIteration() const;

  // Prints a summary of all test iterations (not just the last one) to stdout.
  void PrintSummaryOfAllIterations() const;

  // Adds a string tag to the JSON summary. This is intended to indicate
  // conditions that affect the entire test run, as opposed to individual tests.
  void AddGlobalTag(const std::string& tag);

  // Saves a JSON summary of all test iterations results to |path|. Adds
  // |additional_tags| to the summary (just for this invocation). Returns
  // true on success.
  [[nodiscard]] bool SaveSummaryAsJSON(
      const FilePath& path,
      const std::vector<std::string>& additional_tags) const;

  // Map where keys are test result statuses, and values are sets of tests
  // which finished with that status.
  typedef std::map<TestResult::Status, std::set<std::string>> TestStatusMap;

  // Returns a test status map (see above) for current test iteration.
  TestStatusMap GetTestStatusMapForCurrentIteration() const;

  // Returns a test status map (see above) for all test iterations.
  TestStatusMap GetTestStatusMapForAllIterations() const;

 private:
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithLinkInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithTagInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithMultiTagsInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithMultiTagsSameNameInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithPropertyInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithOutTimestampInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithTimestampInResult);
  FRIEND_TEST_ALL_PREFIXES(TestResultsTrackerTest,
                           SaveSummaryAsJSONWithSubTestResult);
  void GetTestStatusForIteration(int iteration, TestStatusMap* map) const;

  template <typename InputIterator>
  void PrintTests(InputIterator first,
                  InputIterator last,
                  const std::string& description) const;
  void PrintLeaks(int count, const std::vector<std::string>& test_names) const;

  struct AggregateTestResult {
    AggregateTestResult();
    AggregateTestResult(const AggregateTestResult& other);
    ~AggregateTestResult();

    std::vector<TestResult> test_results;
  };

  struct PerIterationData {
    PerIterationData();
    PerIterationData(const PerIterationData& other);
    ~PerIterationData();

    // Aggregate test results grouped by full test name.
    typedef std::map<std::string, AggregateTestResult> ResultsMap;
    ResultsMap results;

    // A sequence of tests that leaked files/dirs in their temp directory.
    std::vector<std::pair<int, std::vector<std::string>>> leaked_temp_items;
  };

  struct CodeLocation {
    CodeLocation(const std::string& f, int l) : file(f), line(l) {}

    std::string file;
    int line;
  };

  ThreadChecker thread_checker_;

  // Print tests that leak files and/or directories in their temp dir.
  bool print_temp_leaks_ = false;

  // Set of global tags, i.e. strings indicating conditions that apply to
  // the entire test run.
  std::set<std::string> global_tags_;

  // Set of all test names discovered in the current executable.
  std::set<std::string> all_tests_;

  // CodeLocation for all tests that will be run as a part of this shard.
  std::map<std::string, CodeLocation> test_locations_;

  // Name of tests that will run and produce results.
  std::set<std::string> test_placeholders_;

  // Set of all disabled tests in the current executable.
  std::set<std::string> disabled_tests_;

  // Store test results for each iteration.
  std::vector<PerIterationData> per_iteration_data_;

  // Index of current iteration (starting from 0). -1 before the first
  // iteration.
  int iteration_;

  // File handle of output file (can be NULL if no file).
  raw_ptr<FILE> out_;
};

}  // namespace base

#endif  // BASE_TEST_LAUNCHER_TEST_RESULTS_TRACKER_H_
