// Copyright 2006 Google LLC
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

// process_state.h: A snapshot of a process, in a fully-digested state.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_PROCESS_STATE_H__
#define GOOGLE_BREAKPAD_PROCESSOR_PROCESS_STATE_H__

#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/code_modules.h"
#include "google_breakpad/processor/exception_record.h"
#include "google_breakpad/processor/minidump.h"
#include "google_breakpad/processor/system_info.h"
#include "processor/linked_ptr.h"

namespace google_breakpad {

using std::vector;

class CallStack;
class CodeModules;

enum ExploitabilityRating {
  EXPLOITABILITY_HIGH,                 // The crash likely represents
                                       // a exploitable memory corruption
                                       // vulnerability.

  EXPLOITABILITY_MEDIUM,               // The crash appears to corrupt
                                       // memory in a way which may be
                                       // exploitable in some situations.

  EXPLOITABLITY_MEDIUM = EXPLOITABILITY_MEDIUM,  // an old misspelling

  EXPLOITABILITY_LOW,                  // The crash either does not corrupt
                                       // memory directly or control over
                                       // the affected data is limited. The
                                       // issue may still be exploitable
                                       // on certain platforms or situations.

  EXPLOITABILITY_INTERESTING,          // The crash does not appear to be
                                       // directly exploitable. However it
                                       // represents a condition which should
                                       // be further analyzed.

  EXPLOITABILITY_NONE,                 // The crash does not appear to represent
                                       // an exploitable condition.

  EXPLOITABILITY_NOT_ANALYZED,         // The crash was not analyzed for
                                       // exploitability because the engine
                                       // was disabled.

  EXPLOITABILITY_ERR_NOENGINE,         // The supplied minidump's platform does
                                       // not have a exploitability engine
                                       // associated with it.

  EXPLOITABILITY_ERR_PROCESSING        // An error occured within the
                                       // exploitability engine and no rating
                                       // was calculated.
};

class ProcessState {
 public:
  ProcessState() : modules_(nullptr), unloaded_modules_(nullptr) { Clear(); }
  ~ProcessState();

  // Resets the ProcessState to its default values
  void Clear();

  // Accessors.  See the data declarations below.
  uint32_t time_date_stamp() const { return time_date_stamp_; }
  uint32_t process_create_time() const { return process_create_time_; }
  bool crashed() const { return crashed_; }
  string crash_reason() const { return crash_reason_; }
  uint64_t crash_address() const { return crash_address_; }
  string assertion() const { return assertion_; }
  int requesting_thread() const { return requesting_thread_; }
  int original_thread_count() const { return original_thread_count_; }
  const ExceptionRecord* exception_record() const { return &exception_record_; }
  const vector<CallStack*>* threads() const { return &threads_; }
  const vector<MemoryRegion*>* thread_memory_regions() const {
    return &thread_memory_regions_;
  }
  const vector<string>* thread_names() const { return &thread_names_; }
  const SystemInfo* system_info() const { return &system_info_; }
  const CodeModules* modules() const { return modules_; }
  const CodeModules* unloaded_modules() const { return unloaded_modules_; }
  const vector<linked_ptr<const CodeModule> >* shrunk_range_modules() const {
    return &shrunk_range_modules_;
  }
  const vector<const CodeModule*>* modules_without_symbols() const {
    return &modules_without_symbols_;
  }
  const vector<const CodeModule*>* modules_with_corrupt_symbols() const {
    return &modules_with_corrupt_symbols_;
  }
  ExploitabilityRating exploitability() const { return exploitability_; }

 private:
  // MinidumpProcessor and MicrodumpProcessor are responsible for building
  // ProcessState objects.
  friend class MinidumpProcessor;
  friend class MicrodumpProcessor;

  // The time-date stamp of the minidump (time_t format)
  uint32_t time_date_stamp_;

  // The time-date stamp when the process was created (time_t format)
  uint32_t process_create_time_;

  // True if the process crashed, false if the dump was produced outside
  // of an exception handler.
  bool crashed_;

  // If the process crashed, the type of crash.  OS- and possibly CPU-
  // specific.  For example, "EXCEPTION_ACCESS_VIOLATION" (Windows),
  // "EXC_BAD_ACCESS / KERN_INVALID_ADDRESS" (Mac OS X), "SIGSEGV"
  // (other Unix).
  string crash_reason_;

  // If the process crashed, and if crash_reason implicates memory,
  // the memory address that caused the crash.  For data access errors,
  // this will be the data address that caused the fault.  For code errors,
  // this will be the address of the instruction that caused the fault.
  uint64_t crash_address_;

  // If there was an assertion that was hit, a textual representation
  // of that assertion, possibly including the file and line at which
  // it occurred.
  string assertion_;

  // The index of the thread that requested a dump be written in the
  // threads vector.  If a dump was produced as a result of a crash, this
  // will point to the thread that crashed.  If the dump was produced as
  // by user code without crashing, and the dump contains extended Breakpad
  // information, this will point to the thread that requested the dump.
  // If the dump was not produced as a result of an exception and no
  // extended Breakpad information is present, this field will be set to -1,
  // indicating that the dump thread is not available.
  int requesting_thread_;

  // Original thread count. The Processor has limit on how many threads to
  // process, so not all threads are processed. This tells you how many threads
  // were originally in the minudump.
  int original_thread_count_;

  // Exception record details: code, flags, address, parameters.
  ExceptionRecord exception_record_;

  // Stacks for each thread (except possibly the exception handler
  // thread) at the time of the crash.
  vector<CallStack*> threads_;
  vector<MemoryRegion*> thread_memory_regions_;

  // Names of each thread at the time of the crash, one for each entry in
  // threads_. Note that a thread's name might be empty if there was no
  // corresponding ThreadNamesStream in the minidump, or if a particular thread
  // ID was not present in the THREAD_NAME_LIST.
  vector<string> thread_names_;

  // OS and CPU information.
  SystemInfo system_info_;

  // The modules that were loaded into the process represented by the
  // ProcessState.
  const CodeModules *modules_;

  // The modules that have been unloaded from the process represented by the
  // ProcessState.
  const CodeModules *unloaded_modules_;

  // The modules which virtual address ranges were shrunk down due to
  // virtual address conflicts.
  vector<linked_ptr<const CodeModule> > shrunk_range_modules_;

  // The modules that didn't have symbols when the report was processed.
  vector<const CodeModule*> modules_without_symbols_;

  // The modules that had corrupt symbols when the report was processed.
  vector<const CodeModule*> modules_with_corrupt_symbols_;

  // The exploitability rating as determined by the exploitability
  // engine. When the exploitability engine is not enabled this
  // defaults to EXPLOITABILITY_NOT_ANALYZED.
  ExploitabilityRating exploitability_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_PROCESS_STATE_H__
