// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_LAUNCHER_UNIT_TEST_LAUNCHER_H_
#define BASE_TEST_LAUNCHER_UNIT_TEST_LAUNCHER_H_

#include <stddef.h>

#include <string>
#include <string_view>
#include <vector>

#include "base/files/file_path.h"
#include "base/files/scoped_temp_dir.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(USE_BLINK)
#include "base/test/launcher/test_launcher.h"
#endif

namespace base {

// Callback that runs a test suite and returns exit code.
using RunTestSuiteCallback = OnceCallback<int(void)>;

// Launches unit tests in given test suite. Returns exit code.
int LaunchUnitTests(int argc,
                    char** argv,
                    RunTestSuiteCallback run_test_suite,
                    size_t retry_limit = 1U);

// Same as above, but always runs tests serially.
int LaunchUnitTestsSerially(int argc,
                            char** argv,
                            RunTestSuiteCallback run_test_suite);

// The following is not supported in unit_test_launcher_ios.cc, which is used on
// iOS unless Blink is enabled.
#if BUILDFLAG(USE_BLINK)

// Launches unit tests in given test suite. Returns exit code.
// |parallel_jobs| is the number of parallel test jobs.
// |default_batch_limit| is the default size of test batch
// (use 0 to disable batching).
// |use_job_objects| determines whether to use job objects.
// |timeout_callback| is called each time a test batch times out. It can be used
// as a cue to print additional debugging information about the test system,
// such as log files or the names of running processes.
int LaunchUnitTestsWithOptions(int argc,
                               char** argv,
                               size_t parallel_jobs,
                               int default_batch_limit,
                               bool use_job_objects,
                               RepeatingClosure timeout_callback,
                               RunTestSuiteCallback run_test_suite);

#if BUILDFLAG(IS_WIN)
// Launches unit tests in given test suite. Returns exit code.
// |use_job_objects| determines whether to use job objects.
int LaunchUnitTests(int argc,
                    wchar_t** argv,
                    bool use_job_objects,
                    RunTestSuiteCallback run_test_suite);
#endif  // BUILDFLAG(IS_WIN)

// Delegate to abstract away platform differences for unit tests.
class UnitTestPlatformDelegate {
 public:
  // Called to get names of tests available for running. The delegate
  // must put the result in |output| and return true on success.
  virtual bool GetTests(std::vector<TestIdentifier>* output) = 0;

  // Called to create a temporary for storing test results. The delegate
  // must put the resulting path in |path| and return true on success.
  virtual bool CreateResultsFile(const base::FilePath& temp_dir,
                                 base::FilePath* path) = 0;

  // Called to create a new temporary file. The delegate must put the resulting
  // path in |path| and return true on success.
  virtual bool CreateTemporaryFile(const base::FilePath& temp_dir,
                                   base::FilePath* path) = 0;

  // Returns command line for child GTest process based on the command line
  // of current process. |test_names| is a vector of test full names
  // (e.g. "A.B"), |output_file| is path to the GTest XML output file.
  virtual CommandLine GetCommandLineForChildGTestProcess(
      const std::vector<std::string>& test_names,
      const base::FilePath& output_file,
      const base::FilePath& flag_file) = 0;

  // Returns wrapper to use for child GTest process. Empty string means
  // no wrapper.
  virtual std::string GetWrapperForChildGTestProcess() = 0;

 protected:
  ~UnitTestPlatformDelegate() = default;
};

// This default implementation uses gtest_util to get all
// compiled gtests into the binary.
// The delegate will relaunch test in parallel,
// but only use single test per launch.
class DefaultUnitTestPlatformDelegate : public UnitTestPlatformDelegate {
 public:
  DefaultUnitTestPlatformDelegate();

  DefaultUnitTestPlatformDelegate(const DefaultUnitTestPlatformDelegate&) =
      delete;
  DefaultUnitTestPlatformDelegate& operator=(
      const DefaultUnitTestPlatformDelegate&) = delete;

 private:
  // UnitTestPlatformDelegate:

  bool GetTests(std::vector<TestIdentifier>* output) override;

  bool CreateResultsFile(const base::FilePath& temp_dir,
                         base::FilePath* path) override;

  bool CreateTemporaryFile(const base::FilePath& temp_dir,
                           base::FilePath* path) override;

  CommandLine GetCommandLineForChildGTestProcess(
      const std::vector<std::string>& test_names,
      const base::FilePath& output_file,
      const base::FilePath& flag_file) override;

  std::string GetWrapperForChildGTestProcess() override;

  ScopedTempDir temp_dir_;
};

// Test launcher delegate for unit tests (mostly to support batching).
class UnitTestLauncherDelegate : public TestLauncherDelegate {
 public:
  UnitTestLauncherDelegate(UnitTestPlatformDelegate* delegate,
                           size_t batch_limit,
                           bool use_job_objects,
                           RepeatingClosure timeout_callback);

  UnitTestLauncherDelegate(const UnitTestLauncherDelegate&) = delete;
  UnitTestLauncherDelegate& operator=(const UnitTestLauncherDelegate&) = delete;

  ~UnitTestLauncherDelegate() override;

 private:
  // TestLauncherDelegate:
  bool GetTests(std::vector<TestIdentifier>* output) override;

  CommandLine GetCommandLine(const std::vector<std::string>& test_names,
                             const FilePath& temp_dir,
                             FilePath* output_file) override;

  std::string GetWrapper() override;

  int GetLaunchOptions() override;

  TimeDelta GetTimeout() override;

  size_t GetBatchSize() override;

  void OnTestTimedOut(const CommandLine& cmd_line) override;

  ThreadChecker thread_checker_;

  raw_ptr<UnitTestPlatformDelegate> platform_delegate_;

  // Maximum number of tests to run in a single batch.
  size_t batch_limit_;

  // Determines whether we use job objects on Windows.
  bool use_job_objects_;

  // Callback to invoke when a test process times out.
  RepeatingClosure timeout_callback_;
};

// We want to stop throwing away duplicate test filter file flags, but we're
// afraid of changing too much in fear of breaking other use cases.
// If you feel like another flag should be merged instead of overridden,
// feel free to make this into a set of flags in this function,
// or add its own merging code.
//
// out_value contains the existing value and is modified to resolve the
// duplicate
class MergeTestFilterSwitchHandler : public DuplicateSwitchHandler {
 public:
  ~MergeTestFilterSwitchHandler() override;

  void ResolveDuplicate(std::string_view key,
                        CommandLine::StringViewType new_value,
                        CommandLine::StringType& out_value) override;
};

#endif  // BUILDFLAG(USE_BLINK)

}  // namespace base

#endif  // BASE_TEST_LAUNCHER_UNIT_TEST_LAUNCHER_H_
