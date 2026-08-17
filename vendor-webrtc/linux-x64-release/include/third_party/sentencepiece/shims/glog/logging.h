// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_SENTENCEPIECE_SHIMS_GLOG_LOGGING_H_
#define THIRD_PARTY_SENTENCEPIECE_SHIMS_GLOG_LOGGING_H_

#ifndef LOG

#include "third_party/abseil-cpp/absl/log/check.h"
#include "third_party/abseil-cpp/absl/log/log.h"

#define VLOG(severity) LOG(INFO)

#define VLOG_IS_ON(severity) false

#endif

namespace google {

void AddLogSink(void* unused);
void RemoveLogSink(void* unused);

class LogSink {};

typedef int LogSeverity;

int SetVLOGLevel(const char* module_pattern, int log_level);

}  // namespace google

#endif  // THIRD_PARTY_SENTENCEPIECE_SHIMS_GLOG_LOGGING_H_
