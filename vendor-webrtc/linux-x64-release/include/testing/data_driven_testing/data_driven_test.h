// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_DATA_DRIVEN_TESTING_DATA_DRIVEN_TEST_H_
#define TESTING_DATA_DRIVEN_TESTING_DATA_DRIVEN_TEST_H_

#include <string>

#include "base/files/file_path.h"

namespace testing {

// A convenience class for implementing data-driven tests. Subclassers need only
// implement the conversion of serialized input data to serialized output data
// and provide a set of input files. For each input file, on the first run, a
// gold output file is generated; for subsequent runs, the test output is
// compared to this gold output.
class DataDrivenTest {
 public:
  DataDrivenTest(const DataDrivenTest&) = delete;
  DataDrivenTest& operator=(const DataDrivenTest&) = delete;

  // For each file in |input_directory| whose filename matches
  // |file_name_pattern|, slurps in the file contents and calls into
  // |GenerateResults()|. If the corresponding output file already exists in
  // the |output_directory|, verifies that the results match the file contents;
  // otherwise, writes a gold result file to the |output_directory|.
  void RunDataDrivenTest(const base::FilePath& input_directory,
                         const base::FilePath& output_directory,
                         const base::FilePath::StringType& file_name_pattern);

  // As above, but runs a test for a single file, the full path of which is
  // given by |test_file_name|.
  void RunOneDataDrivenTest(const base::FilePath& test_file_name,
                            const base::FilePath& output_directory,
                            bool is_expected_to_pass);

  // Given the |input| data, generates the |output| results. The output results
  // must be stable across runs.
  // Note: The return type is |void| so that googletest |ASSERT_*| macros will
  // compile.
  virtual void GenerateResults(const std::string& input,
                               std::string* output) = 0;

  // Return |base::FilePath|s to the test input and output subdirectories
  // ../|feature_dir|/|test_name|/input and ../|feature_dir|/|test_name|/output.
  base::FilePath GetInputDirectory();
  base::FilePath GetOutputDirectory();

 protected:
  DataDrivenTest(const base::FilePath& test_data_directory,
                 const base::FilePath::StringType& feature_name,
                 const base::FilePath::StringType& test_name);
  virtual ~DataDrivenTest();

 private:
  base::FilePath test_data_directory_;
  base::FilePath::StringType feature_directory_;
  base::FilePath::StringType test_name_;
};

}  // namespace testing

#endif  // TESTING_DATA_DRIVEN_TESTING_DATA_DRIVEN_TEST_H_
