// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_PERF_LUCI_TEST_RESULT_H_
#define TESTING_PERF_LUCI_TEST_RESULT_H_

#include <optional>
#include <string>
#include <vector>

#include "base/containers/flat_map.h"
#include "base/files/file_path.h"
#include "base/time/time.h"

namespace perf_test {

// Generates TestResultEntry dict in LUCI Test Results format.
// See: go/luci-test-results-design
//      //infra/go/src/go.chromium.org/luci/results/proto/v1/test_result.proto
class LuciTestResult {
 public:
  // Represents a test result status.
  enum class Status {
    // The test status is unspecified.
    kUnspecified,
    // The test has passed.
    kPass,
    // The test has failed.
    kFail,
    // The test did not complete because it crashed.
    kCrash,
    // The test did not complete because it was interrupted, e.g. timeout.
    kAbort,
    // The test or test framework decided not to run the test, or the test was
    // not run due to previous tests timing out.
    kSkip
  };

  // Represents an artifact.
  struct Artifact {
    Artifact();
    Artifact(const Artifact& other);
    Artifact(const base::FilePath file_path, const std::string& content_type);
    Artifact(const std::string& contents, const std::string& content_type);
    ~Artifact();

    // Use only one of the two fields below.
    // Absolute path on the same machine running the test.
    std::optional<base::FilePath> file_path;
    // The data of the artifact.
    std::optional<std::string> contents;

    std::string content_type;
  };

  // Represents a tag.
  struct Tag {
    std::string key;
    std::string value;
  };

  LuciTestResult();
  LuciTestResult(const LuciTestResult& other);
  LuciTestResult(LuciTestResult&& other);
  ~LuciTestResult();

  // Helper to create a LuciTestResult and fill in info for the current gtest.
  static LuciTestResult CreateForGTest();

  // Adds a variant key-value pair to |extra_variant_pairs_|. See VariantDef in
  //   //infra/go/src/go.chromium.org/luci/resultdb/proto/v1/common.proto
  // for more details.
  void AddVariant(const std::string& key, const std::string& value);

  // Adds an output artifact.
  void AddOutputArtifactFile(const std::string& artifact_name,
                             const base::FilePath& file_path,
                             const std::string& content_type);
  void AddOutputArtifactContents(const std::string& artifact_name,
                                 const std::string& contents,
                                 const std::string& content_type);

  // Adds a tag.
  void AddTag(const std::string& key, const std::string& value);

  // Writes to |result_file|.
  void WriteToFile(const base::FilePath& result_file) const;

  // Getters and setters.
  const std::string& test_path() const { return test_path_; }
  void set_test_path(const std::string& test_path) { test_path_ = test_path; }

  const base::flat_map<std::string, std::string>& extra_variant_pairs() const {
    return extra_variant_pairs_;
  }

  Status status() const { return status_; }
  void set_status(Status status) { status_ = status; }

  bool is_expected() const { return is_expected_; }
  void set_is_expected(bool is_expcted) { is_expected_ = is_expcted; }

  base::Time start_time() const { return start_time_; }
  void set_start_time(base::Time start_time) { start_time_ = start_time; }

  base::TimeDelta duration() const { return duration_; }
  void set_duration(base::TimeDelta duration) { duration_ = duration; }

  const base::flat_map<std::string, Artifact>& output_artifacts() const {
    return output_artifacts_;
  }

  const std::vector<Tag>& tags() const { return tags_; }

 private:
  // For gtest, |test_path_| is <test_suite_name>.<test_case_name>, without
  // the param annotations. E.g. "InstantiationName/SuiteName.CaseName/0"
  // will have "/0" stripped and be just "InstantiationName/SuiteName.CaseName".
  std::string test_path_;
  // For gtest, |extra_variant_pairs_| holds info about the type param and
  // value param for typed/parameterized tests.
  base::flat_map<std::string, std::string> extra_variant_pairs_;
  // Status of the test result.
  Status status_ = Status::kUnspecified;
  // Whether |status| is expected.
  bool is_expected_ = false;
  // Test start time.
  base::Time start_time_;
  // Duration of the test.
  base::TimeDelta duration_;
  // Artifacts of the test run.
  base::flat_map<std::string, Artifact> output_artifacts_;
  // Tags of the test run.
  std::vector<Tag> tags_;
};

}  // namespace perf_test

#endif  // TESTING_PERF_LUCI_TEST_RESULT_H_
