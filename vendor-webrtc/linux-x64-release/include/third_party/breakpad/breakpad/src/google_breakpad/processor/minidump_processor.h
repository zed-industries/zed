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

#ifndef GOOGLE_BREAKPAD_PROCESSOR_MINIDUMP_PROCESSOR_H__
#define GOOGLE_BREAKPAD_PROCESSOR_MINIDUMP_PROCESSOR_H__

#include <assert.h>
#include <string>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/process_result.h"

namespace google_breakpad {

class Minidump;
class ProcessState;
class StackFrameSymbolizer;
class SourceLineResolverInterface;
class SymbolSupplier;
struct SystemInfo;

class MinidumpProcessor {
 public:
  // Initializes this MinidumpProcessor.  supplier should be an
  // implementation of the SymbolSupplier abstract base class.
  MinidumpProcessor(SymbolSupplier* supplier,
                    SourceLineResolverInterface* resolver);

  // Initializes the MinidumpProcessor with the option of
  // enabling the exploitability framework to analyze dumps
  // for probable security relevance.
  MinidumpProcessor(SymbolSupplier* supplier,
                    SourceLineResolverInterface* resolver,
                    bool enable_exploitability);

  // Initializes the MinidumpProcessor with source line resolver helper, and
  // the option of enabling the exploitability framework to analyze dumps
  // for probable security relevance.
  // Does not take ownership of resolver_helper, which must NOT be NULL.
  MinidumpProcessor(StackFrameSymbolizer* stack_frame_symbolizer,
                    bool enable_exploitability);

  ~MinidumpProcessor();

  // Processes the minidump file and fills process_state with the result.
  ProcessResult Process(const string& minidump_file,
                        ProcessState* process_state);

  // Processes the minidump structure and fills process_state with the
  // result.
  ProcessResult Process(Minidump* minidump,
                        ProcessState* process_state);
  // Populates the cpu_* fields of the |info| parameter with textual
  // representations of the CPU type that the minidump in |dump| was
  // produced on.  Returns false if this information is not available in
  // the minidump.
  static bool GetCPUInfo(Minidump* dump, SystemInfo* info);

  // Populates the os_* fields of the |info| parameter with textual
  // representations of the operating system that the minidump in |dump|
  // was produced on.  Returns false if this information is not available in
  // the minidump.
  static bool GetOSInfo(Minidump* dump, SystemInfo* info);

  // Populates the |process_create_time| parameter with the create time of the
  // crashed process.  Returns false if this information is not available in
  // the minidump |dump|.
  static bool GetProcessCreateTime(Minidump* dump,
                                   uint32_t* process_create_time);

  // Returns a textual representation of the reason that a crash occurred,
  // if the minidump in dump was produced as a result of a crash.  Returns
  // an empty string if this information cannot be determined.  If address
  // is non-NULL, it will be set to contain the address that caused the
  // exception, if this information is available.  This will be a code
  // address when the crash was caused by problems such as illegal
  // instructions or divisions by zero, or a data address when the crash
  // was caused by a memory access violation. If enable_objdump is set, this
  // may use disassembly to compute the faulting address.
  static string GetCrashReason(Minidump* dump, uint64_t* address,
                               bool enable_objdump);

  // This function returns true if the passed-in error code is
  // something unrecoverable(i.e. retry should not happen).  For
  // instance, if the minidump is corrupt, then it makes no sense to
  // retry as we won't be able to glean additional information.
  // However, as an example of the other case, the symbol supplier can
  // return an error code indicating it was 'interrupted', which can
  // happen of the symbols are fetched from a remote store, and a
  // retry might be successful later on.
  // You should not call this method with PROCESS_OK! Test for
  // that separately before calling this.
  static bool IsErrorUnrecoverable(ProcessResult p) {
    assert(p !=  PROCESS_OK);
    return (p != PROCESS_SYMBOL_SUPPLIER_INTERRUPTED);
  }

  // Returns a textual representation of an assertion included
  // in the minidump.  Returns an empty string if this information
  // does not exist or cannot be determined.
  static string GetAssertion(Minidump* dump);

  // Sets the flag to enable/disable use of objdump during normal crash
  // processing. This is independent from the flag for use of objdump during
  // exploitability analysis.
  void set_enable_objdump(bool enabled) { enable_objdump_ = enabled; }

  // Sets the flag to enable/disable use of objdump during exploitability
  // analysis. This is independent from the flag for use of objdump during
  // normal crash processing.
  void set_enable_objdump_for_exploitability(bool enabled) {
    enable_objdump_for_exploitability_ = enabled;
  }

  // Sets the maximum number of threads to process.
  void set_max_thread_count(int max_thread_count) {
    max_thread_count_ = max_thread_count;
  }

 private:
  StackFrameSymbolizer* frame_symbolizer_;
  // Indicate whether resolver_helper_ is owned by this instance.
  bool own_frame_symbolizer_;

  // This flag enables the exploitability scanner which attempts to
  // guess how likely it is that the crash represents an exploitable
  // memory corruption issue.
  bool enable_exploitability_;

  // This flag permits the processor to shell out to objdump for purposes of
  // disassembly during normal crash processing, but not during exploitability
  // analysis.
  bool enable_objdump_;

  // This flag permits the exploitability scanner to shell out to objdump for
  // purposes of disassembly. This results in significantly more overhead than
  // the enable_objdump_ flag.
  bool enable_objdump_for_exploitability_;

  // The maximum number of threads to process. This can be exceeded if the
  // requesting thread comes after the limit. Setting this to -1 means no limit.
  int max_thread_count_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_MINIDUMP_PROCESSOR_H__
