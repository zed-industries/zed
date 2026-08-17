// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_PROCESS_STARTUP_HELPER_H_
#define BASE_WIN_PROCESS_STARTUP_HELPER_H_

#include "base/base_export.h"

namespace base {

class CommandLine;

namespace win {

// Register the invalid param handler and pure call handler to be able to
// notify breakpad when it happens.
BASE_EXPORT void RegisterInvalidParamHandler();

// Sets up the CRT's debugging macros to output to stdout.
BASE_EXPORT void SetupCRT(const CommandLine& command_line);

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_PROCESS_STARTUP_HELPER_H_
