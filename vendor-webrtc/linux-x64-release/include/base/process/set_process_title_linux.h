// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_SET_PROCESS_TITLE_LINUX_H_
#define BASE_PROCESS_SET_PROCESS_TITLE_LINUX_H_

#include "base/base_export.h"

// Set the process title that will show in "ps" and similar tools. Takes
// printf-style format string and arguments. After calling setproctitle()
// the original main() argv[] array should not be used. By default, the
// original argv[0] is prepended to the format; this can be disabled by
// including a '-' as the first character of the format string.
//
// This signature and naming is to be compatible with most other Unix
// implementations of setproctitle().
BASE_EXPORT void setproctitle(const char* fmt, ...);

// Initialize state needed for setproctitle() on Linux. Pass the argv pointer
// from main() to setproctitle_init() before calling setproctitle().
BASE_EXPORT void setproctitle_init(const char** main_argv);

#endif  // BASE_PROCESS_SET_PROCESS_TITLE_LINUX_H_
