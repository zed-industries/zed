// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_COMMAND_LINE_H_
#define BASE_TEST_SCOPED_COMMAND_LINE_H_

#include "base/command_line.h"

namespace base {
namespace test {

// Helper class to restore the original command line at the end of the scope.
// NOTE: In most unit tests, the command line is automatically restored per
//       test, so this class is not necessary if the command line applies to
//       the entire single test.
class ScopedCommandLine final {
 public:
  ScopedCommandLine();
  ~ScopedCommandLine();

  // Gets the command line for the current process.
  // NOTE: Do not name this GetCommandLine as this will conflict with Windows's
  //       GetCommandLine and get renamed to GetCommandLineW.
  CommandLine* GetProcessCommandLine();

 private:
  const CommandLine original_command_line_;
};

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_SCOPED_COMMAND_LINE_H_
