// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef ATRACE_PROCESS_DUMP_H_
#define ATRACE_PROCESS_DUMP_H_

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <memory>
#include <set>
#include <string>

#include "logging.h"
#include "process_info.h"
#include "time_utils.h"

// Program that collects processes, thread names, per-process memory stats and
// other minor metrics from /proc filesystem. It's aimed to extend systrace
// with more actionable number to hit performance issues.
class AtraceProcessDump {
 public:
  enum FullDumpMode {
    kDisabled,
    kAllProcesses,
    kAllJavaApps,
    kOnlyWhitelisted,
  };

  AtraceProcessDump();
  ~AtraceProcessDump();

  void RunAndPrintJson(FILE* stream);
  void Stop();

  void SetDumpInterval(int interval_ms);

  // Negative number or zero means unlimited number of dumps.
  void set_dump_count(int count) { dump_count_ = count; }

  void set_full_dump_mode(FullDumpMode mode) { full_dump_mode_ = mode; }
  void set_full_dump_whitelist(const std::set<std::string> &whitelist) {
    CHECK(full_dump_mode_ == FullDumpMode::kOnlyWhitelisted);
    full_dump_whitelist_ = whitelist;
  }
  void enable_graphics_stats() { graphics_stats_ = true; }
  void enable_print_smaps() { print_smaps_ = true; }

 private:
  AtraceProcessDump(const AtraceProcessDump&) = delete;
  void operator=(const AtraceProcessDump&) = delete;

  using ProcessMap = std::map<int, std::unique_ptr<ProcessInfo>>;
  using ProcessSnapshotMap = std::map<int, std::unique_ptr<ProcessSnapshot>>;

  void TakeGlobalSnapshot();
  void TakeAndSerializeMemInfo();
  bool UpdatePersistentProcessInfo(int pid);
  bool ShouldTakeFullDump(const ProcessInfo* process);
  void SerializeSnapshot();
  void SerializePersistentProcessInfo();
  void Cleanup();

  int self_pid_;
  int dump_count_;
  bool graphics_stats_ = false;
  bool print_smaps_ = false;
  FullDumpMode full_dump_mode_ = FullDumpMode::kDisabled;
  std::set<std::string> full_dump_whitelist_;

  FILE* out_;
  ProcessMap processes_;
  ProcessSnapshotMap snapshot_;
  uint64_t snapshot_timestamp_;
  std::set<int> full_dump_whitelisted_pids_;
  std::unique_ptr<time_utils::PeriodicTimer> snapshot_timer_;
  int dump_interval_in_timer_ticks_;
};

#endif  // ATRACE_PROCESS_DUMP_H_
