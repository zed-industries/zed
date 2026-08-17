// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_MEDIAPIPE_SHIMS_GLOG_LOGGING_H_
#define THIRD_PARTY_MEDIAPIPE_SHIMS_GLOG_LOGGING_H_

#ifndef LOG

#include "third_party/abseil-cpp/absl/log/absl_check.h"
#include "third_party/abseil-cpp/absl/log/absl_log.h"

#define VLOG(severity) ABSL_LOG(INFO)
#define VLOG_IS_ON(severity) false

#define LOG(severity) ABSL_LOG(severity)
#define LOG_IF(severity, condition) ABSL_LOG_IF(severity, condition)
#define LOG_EVERY_N(severity, n) ABSL_LOG_EVERY_N(severity, n)
#define LOG_FIRST_N(severity, n) ABSL_LOG_FIRST_N(severity, n)

#elif !defined(VLOG)

#define VLOG(severity) ABSL_LOG(INFO)
#define VLOG_IS_ON(severity) false

#endif  // !defined(VLOG)

namespace google {

void AddLogSink(void* unused);
void RemoveLogSink(void* unused);

class LogSink {};

typedef int LogSeverity;

int SetVLOGLevel(const char* module_pattern, int log_level);

}  // namespace google

#endif  // THIRD_PARTY_MEDIAPIPE_SHIMS_GLOG_LOGGING_H_
