// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PROCESS_INFO_H_
#define PROCESS_INFO_H_

#include <map>

#include "process_memory_stats.h"

struct ThreadInfo {
  int tid;
  char name[16];
};

struct ProcessInfo {
  int pid;
  bool in_kernel;
  bool is_app;
  char name[256];
  char exe[256];
  std::map<int, ThreadInfo> threads;
};

struct ProcessSnapshot {
  int pid;
  ProcessMemoryStats memory;
  // OOM badness and tolerance (oom_adj is deprecated).
  int oom_score;
  int oom_score_adj;
  // Page faults.
  unsigned long minor_faults;
  unsigned long major_faults;
  // Time spent in userspace and in the kernel.
  unsigned long utime;
  unsigned long stime;
};

#endif  // PROCESS_INFO_H_
