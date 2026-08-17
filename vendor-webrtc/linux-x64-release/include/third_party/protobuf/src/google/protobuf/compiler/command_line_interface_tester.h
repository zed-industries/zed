// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_COMMAND_LINE_INTERFACE_TESTER_H__
#define GOOGLE_PROTOBUF_COMPILER_COMMAND_LINE_INTERFACE_TESTER_H__

#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>

#include <memory>
#include <string>
#include <utility>
#include <vector>

#ifndef _MSC_VER
#include <unistd.h>
#endif

#include <gtest/gtest.h>
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/compiler/command_line_interface.h"
#include "google/protobuf/testing/file.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {

// Provide a base class for testing the protoc CLI and plugins.
class CommandLineInterfaceTester : public testing::Test {
 protected:
  CommandLineInterfaceTester();
  ~CommandLineInterfaceTester() override;

  // Runs the CommandLineInterface with the given command line.  The
  // command is automatically split on spaces, and the string "$tmpdir"
  // is replaced with TestTempDir().
  void RunProtoc(absl::string_view command);
  void RunProtocWithArgs(std::vector<std::string> args);

  // -----------------------------------------------------------------
  // Methods to set up the test (called before Run()).

  // Returns the temporary directory created for testing.
  std::string temp_directory() { return temp_directory_; }

  void AllowPlugins(const std::string& prefix) { cli_.AllowPlugins(prefix); }

  void RegisterGenerator(const std::string& flag_name,
                         std::unique_ptr<CodeGenerator> generator,
                         const std::string& help_text) {
    generators_.emplace_back(std::move(generator));
    cli_.RegisterGenerator(flag_name, generators_.back().get(), help_text);
  }

  void RegisterGenerator(const std::string& flag_name,
                         const std::string& option_flag_name,
                         std::unique_ptr<CodeGenerator> generator,
                         const std::string& help_text) {
    generators_.emplace_back(std::move(generator));
    cli_.RegisterGenerator(flag_name, option_flag_name,
                           generators_.back().get(), help_text);
  }

  // Creates a temp file within temp_directory_ with the given name.
  // The containing directory is also created if necessary.
  void CreateTempFile(absl::string_view name, absl::string_view contents);

  // Creates a subdirectory within temp_directory_.
  void CreateTempDir(absl::string_view name);

  // Changes working directory to temp directory.
  void SwitchToTempDirectory() {
    File::ChangeWorkingDirectory(temp_directory_);
  }

  // -----------------------------------------------------------------
  // Methods to check the test results (called after Run()).

  // Checks that no text was written to stderr during Run(), and Run()
  // returned 0.
  void ExpectNoErrors();

  // Checks that Run() returned non-zero and the stderr output is exactly
  // the text given.  expected_test may contain references to "$tmpdir",
  // which will be replaced by the temporary directory path.
  void ExpectErrorText(absl::string_view expected_text);

  // Checks that Run() returned non-zero and the stderr contains the given
  // substring.
  void ExpectErrorSubstring(absl::string_view expected_substring);

  // Checks that Run() returned zero and the stderr contains the given
  // substring.
  void ExpectWarningSubstring(absl::string_view expected_substring);

  // Checks that the captured stdout is the same as the expected_text.
  void ExpectCapturedStdout(absl::string_view expected_text);

  // Checks that Run() returned zero and the stdout contains the given
  // substring.
  void ExpectCapturedStdoutSubstringWithZeroReturnCode(
      absl::string_view expected_substring);

  // Checks that Run() returned zero and the stderr contains the given
  // substring.
  void ExpectCapturedStderrSubstringWithZeroReturnCode(
      absl::string_view expected_substring);

#if defined(_WIN32) && !defined(__CYGWIN__)
  // Returns true if ExpectErrorSubstring(expected_substring) would pass, but
  // does not fail otherwise.
  bool HasAlternateErrorSubstring(const std::string& expected_substring);
#endif  // _WIN32 && !__CYGWIN__

  void ExpectFileContent(absl::string_view filename, absl::string_view content);

 private:
  // The object we are testing.
  CommandLineInterface cli_;

  // We create a directory within TestTempDir() in order to add extra
  // protection against accidentally deleting user files (since we recursively
  // delete this directory during the test).  This is the full path of that
  // directory.
  std::string temp_directory_;

  // The result of Run().
  int return_code_;

  std::string captured_stderr_;

  std::string captured_stdout_;

  std::vector<std::unique_ptr<CodeGenerator>> generators_;
};

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_COMMAND_LINE_INTERFACE_TEST_UTIL_H__
