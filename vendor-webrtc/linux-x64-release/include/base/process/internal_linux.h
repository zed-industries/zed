// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/390223051): Remove C-library calls to fix the errors.
#pragma allow_unsafe_libc_calls
#endif

// This file contains internal routines that are called by other files in
// base/process/.

#ifndef BASE_PROCESS_INTERNAL_LINUX_H_
#define BASE_PROCESS_INTERNAL_LINUX_H_

#include <stddef.h>
#include <stdint.h>
#include <unistd.h>

#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/containers/span.h"
#include "base/files/dir_reader_posix.h"
#include "base/files/file_path.h"
#include "base/process/process_handle.h"
#include "base/strings/string_number_conversions.h"
#include "base/strings/string_split.h"
#include "base/threading/platform_thread.h"

namespace base {

class Time;
class TimeDelta;

namespace internal {

// "/proc"
extern const char kProcDir[];

// "stat"
extern const char kStatFile[];

// Returns a FilePath to "/proc/pid".
BASE_EXPORT base::FilePath GetProcPidDir(pid_t pid);

// Reads a file from /proc into a string. This is allowed on any thread as
// reading from /proc does not hit the disk. Returns true if the file can be
// read and is non-empty.
bool ReadProcFile(const FilePath& file, std::string* buffer);

// Take a /proc directory entry named |d_name|, and if it is the directory for
// a process, convert it to a pid_t.
// Returns 0 on failure.
// e.g. /proc/self/ will return 0, whereas /proc/1234 will return 1234.
pid_t ProcDirSlotToPid(std::string_view d_name);

// Read |filename| in /proc/<pid>/, split the entries into key/value pairs, and
// trim the key and value. On success, return true and write the trimmed
// key/value pairs into |key_value_pairs|.
bool ReadProcFileToTrimmedStringPairs(pid_t pid,
                                      std::string_view filename,
                                      StringPairs* key_value_pairs);

// Read /proc/<pid>/status and return the value for |field|, or 0 on failure.
// Only works for fields in the form of "Field: value kB".
size_t ReadProcStatusAndGetKbFieldAsSizeT(pid_t pid, std::string_view field);

// Read /proc/<pid>/status and look for |field|. On success, return true and
// write the value for |field| into |result|.
// Only works for fields in the form of "field    :     uint_value"
bool ReadProcStatusAndGetFieldAsUint64(pid_t pid,
                                       std::string_view field,
                                       uint64_t* result);

// Reads /proc/<pid>/stat into |buffer|. Returns true if the file can be read
// and is non-empty.
bool ReadProcStats(pid_t pid, std::string* buffer);

// Takes |stats_data| and populates |proc_stats| with the values split by
// spaces. Taking into account the 2nd field may, in itself, contain spaces.
// Returns true if successful.
bool ParseProcStats(const std::string& stats_data,
                    std::vector<std::string>* proc_stats);

// Fields from /proc/<pid>/stat, 0-based. See man 5 proc.
// If the ordering ever changes, carefully review functions that use these
// values.
enum ProcStatsFields {
  VM_COMM = 1,         // Filename of executable, without parentheses.
  VM_STATE = 2,        // Letter indicating the state of the process.
  VM_PPID = 3,         // PID of the parent.
  VM_PGRP = 4,         // Process group id.
  VM_MINFLT = 9,       // Minor page fault count excluding children.
  VM_MAJFLT = 11,      // Major page fault count excluding children.
  VM_UTIME = 13,       // Time scheduled in user mode in clock ticks.
  VM_STIME = 14,       // Time scheduled in kernel mode in clock ticks.
  VM_NUMTHREADS = 19,  // Number of threads.
  VM_STARTTIME = 21,   // The time the process started in clock ticks.
  VM_VSIZE = 22,       // Virtual memory size in bytes.
  VM_RSS = 23,         // Resident Set Size in pages.
};

// Reads the |field_num|th field from |proc_stats|. Returns 0 on failure.
// This version does not handle the first 3 values, since the first value is
// simply |pid|, and the next two values are strings.
int64_t GetProcStatsFieldAsInt64(const std::vector<std::string>& proc_stats,
                                 ProcStatsFields field_num);

// Reads the `field_num`th field from `proc_stats`. Asserts that `field_num` is
// a valid index into `proc_stats`. Returns nullopt if the field doesn't contain
// a valid integer.
std::optional<int64_t> GetProcStatsFieldAsOptionalInt64(
    base::span<const std::string> proc_stats,
    ProcStatsFields field_num);

// Same as GetProcStatsFieldAsInt64(), but for size_t values.
size_t GetProcStatsFieldAsSizeT(const std::vector<std::string>& proc_stats,
                                ProcStatsFields field_num);

// Convenience wrappers around GetProcStatsFieldAsInt64(), ParseProcStats() and
// ReadProcStats(). See GetProcStatsFieldAsInt64() for details.
int64_t ReadStatsFilendGetFieldAsInt64(const FilePath& stat_file,
                                       ProcStatsFields field_num);
int64_t ReadProcStatsAndGetFieldAsInt64(pid_t pid, ProcStatsFields field_num);
int64_t ReadProcSelfStatsAndGetFieldAsInt64(ProcStatsFields field_num);

// Same as ReadProcStatsAndGetFieldAsInt64() but for size_t values.
size_t ReadProcStatsAndGetFieldAsSizeT(pid_t pid, ProcStatsFields field_num);

// Returns the time that the OS started. Clock ticks are relative to this.
Time GetBootTime();

// Returns the amount of time spent in user space since boot across all CPUs.
TimeDelta GetUserCpuTimeSinceBoot();

// Converts Linux clock ticks to a wall time delta.
TimeDelta ClockTicksToTimeDelta(int64_t clock_ticks);

// Executes the lambda for every task in the process's /proc/<pid>/task
// directory. The thread id and file path of the task directory are provided as
// arguments to the lambda.
template <typename Lambda>
void ForEachProcessTask(base::ProcessHandle process, Lambda&& lambda) {
  // Iterate through the different threads tracked in /proc/<pid>/task.
  FilePath fd_path = GetProcPidDir(process).Append("task");

  DirReaderPosix dir_reader(fd_path.value().c_str());
  if (!dir_reader.IsValid()) {
    return;
  }

  for (; dir_reader.Next();) {
    const char* tid_str = dir_reader.name();
    if (strcmp(tid_str, ".") == 0 || strcmp(tid_str, "..") == 0) {
      continue;
    }

    PlatformThreadId::UnderlyingType tid_value;
    if (!StringToInt(tid_str, &tid_value)) {
      continue;
    }
    PlatformThreadId tid(tid_value);

    FilePath task_path = fd_path.Append(tid_str);
    lambda(tid, task_path);
  }
}

}  // namespace internal
}  // namespace base

#endif  // BASE_PROCESS_INTERNAL_LINUX_H_
