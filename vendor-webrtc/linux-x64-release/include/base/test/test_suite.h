// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_SUITE_H_
#define BASE_TEST_TEST_SUITE_H_

// Defines a basic test suite framework for running gtest based tests.  You can
// instantiate this class in your main function and call its Run method to run
// any gtest based tests that are linked into your executable.

#include <memory>
#include <string_view>

#include "base/at_exit.h"
#include "base/check.h"
#include "base/memory/raw_ptr.h"
#include "base/tracing_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(ENABLE_BASE_TRACING)
#include "base/test/trace_to_file.h"
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

#if BUILDFLAG(IS_WIN)
#include <vector>

#include "base/memory/raw_ptr_exclusion.h"
#endif

namespace logging {
class ScopedLogAssertHandler;
}

namespace testing {
class TestInfo;
}

namespace base {

class XmlUnitTestResultPrinter;

// Instantiates TestSuite, runs it and returns exit code.
int RunUnitTestsUsingBaseTestSuite(int argc, char** argv);

class TestSuite {
 public:
  // Match function used by the GetTestCount method.
  typedef bool (*TestMatch)(const testing::TestInfo&);

  TestSuite(int argc, char** argv);
#if BUILDFLAG(IS_WIN)
  TestSuite(int argc, wchar_t** argv);
#endif  // BUILDFLAG(IS_WIN)

  TestSuite(const TestSuite&) = delete;
  TestSuite& operator=(const TestSuite&) = delete;

  virtual ~TestSuite();

  int Run();

  // Disables checks for thread and process priority at the beginning and end of
  // each test. Most tests should not use this.
  void DisableCheckForThreadAndProcessPriority();

  // Disables checks for certain global objects being leaked across tests.
  void DisableCheckForLeakedGlobals();

 protected:
  // By default fatal log messages (e.g. from DCHECKs) result in error dialogs
  // which gum up buildbots. Use a minimalistic assert handler which just
  // terminates the process.
  void UnitTestAssertHandler(const char* file,
                             int line,
                             std::string_view summary,
                             std::string_view stack_trace);

  // Disable crash dialogs so that it doesn't gum up the buildbot
  virtual void SuppressErrorDialogs();

  // Override these for custom test handling. Use these instead of putting
  // complex code in your constructor/destructor.
  virtual void Initialize();
  virtual void InitializeFromCommandLine(int* argc, char** argv);
  virtual int RunAllTests();
  virtual void Shutdown();

  // Make sure that we setup an AtExitManager so Singleton objects will be
  // destroyed.
  std::unique_ptr<base::AtExitManager> at_exit_manager_;

 private:
  // Basic initialization for the test suite happens here.
  void PreInitialize();

  void AddTestLauncherResultPrinter();

#if BUILDFLAG(ENABLE_BASE_TRACING)
  test::TraceToFile trace_to_file_;
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

  raw_ptr<XmlUnitTestResultPrinter, DanglingUntriaged> printer_ = nullptr;

  std::unique_ptr<logging::ScopedLogAssertHandler> assert_handler_;

  bool initialized_command_line_ = false;
  bool check_for_leaked_globals_ = true;
  bool check_for_thread_and_process_priority_ = true;
  bool is_initialized_ = false;
  int argc_;
#if BUILDFLAG(IS_WIN)
  // We need argv_as_pointers_.data() to have type char**, so we can't use
  // raw_ptr here.
  RAW_PTR_EXCLUSION std::vector<char*> argv_as_pointers_;
  std::vector<std::string> argv_as_strings_;
#endif
  raw_ptr<char*> argv_;
};

}  // namespace base

#endif  // BASE_TEST_TEST_SUITE_H_
