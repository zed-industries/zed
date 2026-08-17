/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_PROFILING_COMMON_PROC_CMDLINE_H_
#define SRC_PROFILING_COMMON_PROC_CMDLINE_H_

#include <sys/types.h>

#include <string>

namespace perfetto {
namespace profiling {

// TODO(rsavitski): these functions are a reimplementation of those found in
// proc_utils, but with a change in semantics that we intend for all profilers.
// Eventually this should become the single canonical file dealing with proc
// cmdlines. The transition will start with traced_perf and perfetto_hprof, and
// heapprofd will follow later.
namespace glob_aware {

// These functions let the profilers read a /proc/pid/cmdline, find the
// substrings corresponding to the argv0 as well as the binary name (e.g.
// "/bin/echo" and "echo" respectively), and then match it against a set of glob
// patterns.
//
// Example usage:
//   std::string cmdline;
//   bool success = ReadProcCmdlineForPID(42, &cmdline);
//   if (!success) return false;
//   const char* binname = FindBinaryName(cmdline.c_str(), cmdline.size());
//   return MatchGlobPattern("test*", cmdline.c_str(), binname);

bool ReadProcCmdlineForPID(pid_t pid, std::string* cmdline_out);
const char* FindBinaryName(const char* cmdline, size_t cmdline_len);
bool MatchGlobPattern(const char* pattern,
                      const char* cmdline,
                      const char* binname);

}  // namespace glob_aware
}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_COMMON_PROC_CMDLINE_H_
