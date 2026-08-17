// Copyright 2010 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// Utility functions for spawning a helper process using a different
// CPU architecture.

#ifndef GOOGLE_BREAKPAD_CLIENT_MAC_TESTS_SPAWN_CHILD_PROCESS
#define GOOGLE_BREAKPAD_CLIENT_MAC_TESTS_SPAWN_CHILD_PROCESS

#include <AvailabilityMacros.h>
#ifndef MAC_OS_X_VERSION_10_6
#define MAC_OS_X_VERSION_10_6 1060
#endif
#include <crt_externs.h>
#include <mach-o/dyld.h>
#if MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_6
#include <spawn.h>
#endif

#include <string>
#include <vector>

#include "google_breakpad/common/minidump_format.h"

namespace google_breakpad_test {

using std::string;
using std::vector;

const MDCPUArchitecture kNativeArchitecture =
#if defined(__i386__)
  MD_CPU_ARCHITECTURE_X86
#elif defined(__x86_64__)
  MD_CPU_ARCHITECTURE_AMD64
#elif defined(__ppc__) || defined(__ppc64__)
  MD_CPU_ARCHITECTURE_PPC
#else
#error "This file has not been ported to this CPU architecture."
#endif
  ;

const uint32_t kNativeContext =
#if defined(__i386__)
  MD_CONTEXT_X86
#elif defined(__x86_64__)
  MD_CONTEXT_AMD64
#elif defined(__ppc__) || defined(__ppc64__)
  MD_CONTEXT_PPC
#else
#error "This file has not been ported to this CPU architecture."
#endif
  ;

string GetExecutablePath() {
  char self_path[PATH_MAX];
  uint32_t size = sizeof(self_path);
  if (_NSGetExecutablePath(self_path, &size) != 0)
    return "";
  return self_path;
}

string GetHelperPath() {
  string helper_path(GetExecutablePath());
  size_t pos = helper_path.rfind('/');
  if (pos == string::npos)
    return "";

  helper_path.erase(pos + 1);
  helper_path += "minidump_generator_test_helper";
  return helper_path;
}

#if MAC_OS_X_VERSION_MIN_REQUIRED >= MAC_OS_X_VERSION_10_6

pid_t spawn_child_process(const char** argv) {
  posix_spawnattr_t spawnattr;
  if (posix_spawnattr_init(&spawnattr) != 0)
    return (pid_t)-1;

  cpu_type_t pref_cpu_types[2] = {
#if defined(__x86_64__)
    CPU_TYPE_X86,
#elif defined(__i386__)
    CPU_TYPE_X86_64,
#endif
    CPU_TYPE_ANY
  };

  // Set spawn attributes.
  size_t attr_count = sizeof(pref_cpu_types) / sizeof(pref_cpu_types[0]);
  size_t attr_ocount = 0;
  if (posix_spawnattr_setbinpref_np(&spawnattr,
                                    attr_count,
                                    pref_cpu_types,
                                    &attr_ocount) != 0 ||
      attr_ocount != attr_count) {
    posix_spawnattr_destroy(&spawnattr);
    return (pid_t)-1;
  }

  // Create an argv array.
  vector<char*> argv_v;
  while (*argv) {
    argv_v.push_back(strdup(*argv));
    argv++;
  }
  argv_v.push_back(nullptr);
  pid_t new_pid = 0;
  int result = posix_spawnp(&new_pid, argv_v[0], nullptr, &spawnattr,
                            &argv_v[0], *_NSGetEnviron());
  posix_spawnattr_destroy(&spawnattr);
  
  for (unsigned i = 0; i < argv_v.size(); i++) {
    free(argv_v[i]);
  }

  return result == 0 ? new_pid : -1;
}
#endif

}  // namespace google_breakpad_test

#endif  // GOOGLE_BREAKPAD_CLIENT_MAC_TESTS_SPAWN_CHILD_PROCESS
