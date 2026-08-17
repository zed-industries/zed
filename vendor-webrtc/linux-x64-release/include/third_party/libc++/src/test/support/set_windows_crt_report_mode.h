// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_SET_WINDOWS_CRT_REPORT_MODE_H
#define SUPPORT_SET_WINDOWS_CRT_REPORT_MODE_H

// A few tests are built in C mode or in C++03 mode. The initialization
// of init_crt_anchor is a C++ feature, and <crtdbg.h> ends up including
// MSVC header code which breaks in C++03 mode. Therefore, only expand
// the body of this header when included in C++ >= 11 mode. As this file
// is included in every single translation unit, we're intentionally not
// including test_macros.h (for TEST_STD_VER) but try to keep it to the
// bare minimum.
#if defined(__cplusplus) && __cplusplus > 199711L
#ifndef _DEBUG
#error _DEBUG must be defined when using this header
#endif

#ifndef _WIN32
#error This header can only be used when targeting Windows
#endif

#include <crtdbg.h>

// On Windows in debug builds the default assertion handler opens a new dialog
// window which must be dismissed manually by the user. This function overrides
// that setting and instead changes the assertion handler to log to stderr
// instead.
inline int init_crt_report_mode() {
  _CrtSetReportMode(_CRT_WARN, _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_WARN, _CRTDBG_FILE_STDERR);
  _CrtSetReportMode(_CRT_ERROR, _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_ERROR, _CRTDBG_FILE_STDERR);
  _CrtSetReportMode(_CRT_ASSERT, _CRTDBG_MODE_FILE);
  _CrtSetReportFile(_CRT_ASSERT, _CRTDBG_FILE_STDERR);
  return 0;
}

static int init_crt_anchor = init_crt_report_mode();
#endif // defined(__cplusplus) && __cplusplus > 199711L

#endif // SUPPORT_SET_WINDOWS_CRT_REPORT_MODE_H
