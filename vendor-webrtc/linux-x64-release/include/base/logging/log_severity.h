// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_LOGGING_LOG_SEVERITY_H_
#define BASE_LOGGING_LOG_SEVERITY_H_

#include "base/dcheck_is_on.h"

namespace logging {

using LogSeverity = int;

inline constexpr LogSeverity LOGGING_VERBOSE = -1;  // This is level 1 verbosity
// Note: the log severities are used to index into the array of names,
// see log_severity_names.
inline constexpr LogSeverity LOGGING_INFO = 0;
inline constexpr LogSeverity LOGGING_WARNING = 1;
inline constexpr LogSeverity LOGGING_ERROR = 2;
inline constexpr LogSeverity LOGGING_FATAL = 3;
inline constexpr LogSeverity LOGGING_NUM_SEVERITIES = 4;

// LOGGING_DFATAL is LOGGING_FATAL in DCHECK-enabled builds, ERROR in normal
// mode.
#if DCHECK_IS_ON()
inline constexpr LogSeverity LOGGING_DFATAL = LOGGING_FATAL;
#else
inline constexpr LogSeverity LOGGING_DFATAL = LOGGING_ERROR;
#endif

}  // namespace logging

#endif  // BASE_LOGGING_LOG_SEVERITY_H_
