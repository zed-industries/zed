// Copyright 2011 Google LLC
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

// crash_generator.h: Define the google_breakpad::CrashGenerator class,
// which is used to generate a crash (and a core dump file) for testing.

#ifndef COMMON_LINUX_TESTS_CRASH_GENERATOR_H_
#define COMMON_LINUX_TESTS_CRASH_GENERATOR_H_

#include <sys/resource.h>

#include <string>

#include "common/tests/auto_tempdir.h"
#include "common/using_std_string.h"

namespace google_breakpad {

// A utility class for generating a crash (and a core dump file) for
// testing. It creates a child process with the specified number of
// threads, which is then termainated by the specified signal. A core
// dump file is expected to be created upon the termination of the child
// process, which can then be used for testing code that processes core
// dump files.
class CrashGenerator {
 public:
  CrashGenerator();

  ~CrashGenerator();

  // Returns true if a core dump file named 'core' will be generated in
  // the current directory for a test that produces a crash by checking
  // if /proc/sys/kernel/core_pattern has the default value 'core'.
  bool HasDefaultCorePattern() const;

  // Returns the expected path of the core dump file.
  string GetCoreFilePath() const;

  // Returns the directory of a copy of proc files of the child process.
  string GetDirectoryOfProcFilesCopy() const;

  // Returns whether current resource limits would prevent `CreateChildCrash`
  // from operating.
  bool HasResourceLimitsAmenableToCrashCollection() const;

  // Creates a crash (and a core dump file) by creating a child process with
  // |num_threads| threads, and the terminating the child process by sending
  // a signal with number |crash_signal| to the |crash_thread|-th thread.
  // Returns true on success.
  bool CreateChildCrash(unsigned num_threads, unsigned crash_thread,
                        int crash_signal, pid_t* child_pid);

  // Returns the thread ID of the |index|-th thread in the child process.
  // This method does not validate |index|.
  pid_t GetThreadId(unsigned index) const;

 private:
  // Copies the following proc files of the process with |pid| to the directory
  // at |path|: auxv, cmdline, environ, maps, status
  // The directory must have been created. Returns true on success.
  bool CopyProcFiles(pid_t pid, const char* path) const;

  // Creates |num_threads| threads in the child process.
  void CreateThreadsInChildProcess(unsigned num_threads);

  // Sets the maximum size of core dump file (both the soft and hard limit)
  // to |limit| bytes. Returns true on success.
  bool SetCoreFileSizeLimit(rlim_t limit) const;

  // Creates a shared memory of |memory_size| bytes for communicating thread
  // IDs between the parent and child process. Returns true on success.
  bool MapSharedMemory(size_t memory_size);

  // Releases any shared memory created by MapSharedMemory(). Returns true on
  // success.
  bool UnmapSharedMemory();

  // Returns the pointer to the thread ID of the |index|-th thread in the child
  // process. This method does not validate |index|.
  pid_t* GetThreadIdPointer(unsigned index);

  // Temporary directory in which a core file is generated.
  AutoTempDir temp_dir_;

  // Shared memory for communicating thread IDs between the parent and
  // child process.
  void* shared_memory_;

  // Number of bytes mapped for |shared_memory_|.
  size_t shared_memory_size_;
};

}  // namespace google_breakpad

#endif  // COMMON_LINUX_TESTS_CRASH_GENERATOR_H_
