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

// stackwalker.h: Generic stackwalker.
//
// The Stackwalker class is an abstract base class providing common generic
// methods that apply to stacks from all systems.  Specific implementations
// will extend this class by providing GetContextFrame and GetCallerFrame
// methods to fill in system-specific data in a StackFrame structure.
// Stackwalker assembles these StackFrame strucutres into a CallStack.
//
// Author: Mark Mentovai


#ifndef GOOGLE_BREAKPAD_PROCESSOR_STACKWALKER_H__
#define GOOGLE_BREAKPAD_PROCESSOR_STACKWALKER_H__

#include <set>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/code_modules.h"
#include "google_breakpad/processor/memory_region.h"
#include "google_breakpad/processor/stack_frame_symbolizer.h"

namespace google_breakpad {

class CallStack;
class DumpContext;
class StackFrameSymbolizer;

using std::set;
using std::vector;

class Stackwalker {
 public:
  virtual ~Stackwalker() {}

  // Populates the given CallStack by calling GetContextFrame and
  // GetCallerFrame.  The frames are further processed to fill all available
  // data.  Returns true if the stackwalk completed, or false if it was
  // interrupted by SymbolSupplier::GetSymbolFile().
  // Upon return, |modules_without_symbols| will be populated with pointers to
  // the code modules (CodeModule*) that DON'T have symbols.
  // |modules_with_corrupt_symbols| will be populated with pointers to the
  // modules which have corrupt symbols.  |modules_without_symbols| and
  // |modules_with_corrupt_symbols| DO NOT take ownership of the code modules.
  // The lifetime of these code modules is the same as the lifetime of the
  // CodeModules passed to the StackWalker constructor (which currently
  // happens to be the lifetime of the Breakpad's ProcessingState object).
  // There is a check for duplicate modules so no duplicates are expected.
  bool Walk(CallStack* stack,
            vector<const CodeModule*>* modules_without_symbols,
            vector<const CodeModule*>* modules_with_corrupt_symbols);

  // Returns a new concrete subclass suitable for the CPU that a stack was
  // generated on, according to the CPU type indicated by the context
  // argument.  If no suitable concrete subclass exists, returns NULL.
  static Stackwalker* StackwalkerForCPU(
     const SystemInfo* system_info,
     DumpContext* context,
     MemoryRegion* memory,
     const CodeModules* modules,
     const CodeModules* unloaded_modules,
     StackFrameSymbolizer* resolver_helper);


  static void set_max_frames(uint32_t max_frames) {
    max_frames_ = max_frames;
    max_frames_set_ = true;
  }
  static uint32_t max_frames() { return max_frames_; }

  static void set_max_frames_scanned(uint32_t max_frames_scanned) {
    max_frames_scanned_ = max_frames_scanned;
  }

 protected:
  // system_info identifies the operating system, NULL or empty if unknown.
  // memory identifies a MemoryRegion that provides the stack memory
  // for the stack to walk.  modules, if non-NULL, is a CodeModules
  // object that is used to look up which code module each stack frame is
  // associated with.  frame_symbolizer is a StackFrameSymbolizer object that
  // encapsulates the logic of how source line resolver interacts with symbol
  // supplier to symbolize stack frame and look up caller frame information
  // (see stack_frame_symbolizer.h).
  // frame_symbolizer MUST NOT be NULL (asserted).
  Stackwalker(const SystemInfo* system_info,
              MemoryRegion* memory,
              const CodeModules* modules,
              StackFrameSymbolizer* frame_symbolizer);

  // This can be used to filter out potential return addresses when
  // the stack walker resorts to stack scanning.
  // Returns true if any of:
  // * This address is within a loaded module, but we don't have symbols
  //   for that module.
  // * This address is within a loaded module for which we have symbols,
  //   and falls inside a function in that module.
  // Returns false otherwise.
  bool InstructionAddressSeemsValid(uint64_t address) const;

  // Checks whether we should stop the stack trace.
  // (either we reached the end-of-stack or we detected a
  //  broken callstack invariant)
  bool TerminateWalk(uint64_t caller_ip,
                     uint64_t caller_sp,
                     uint64_t callee_sp,
                     bool first_unwind) const;

  // The default number of words to search through on the stack
  // for a return address.
  static const int kRASearchWords;

  template<typename InstructionType>
  bool ScanForReturnAddress(InstructionType location_start,
                            InstructionType* location_found,
                            InstructionType* ip_found,
                            bool is_context_frame) {
    // When searching for the caller of the context frame,
    // allow the scanner to look farther down the stack.
    const int search_words = is_context_frame ?
      kRASearchWords * 4 :
      kRASearchWords;

    return ScanForReturnAddress(location_start, location_found, ip_found,
                                search_words);
  }

  // Scan the stack starting at location_start, looking for an address
  // that looks like a valid instruction pointer. Addresses must
  // 1) be contained in the current stack memory
  // 2) pass the checks in InstructionAddressSeemsValid
  //
  // Returns true if a valid-looking instruction pointer was found.
  // When returning true, sets location_found to the address at which
  // the value was found, and ip_found to the value contained at that
  // location in memory.
  template<typename InstructionType>
  bool ScanForReturnAddress(InstructionType location_start,
                            InstructionType* location_found,
                            InstructionType* ip_found,
                            int searchwords) {
    for (InstructionType location = location_start;
         location <= location_start + searchwords * sizeof(InstructionType);
         location += sizeof(InstructionType)) {
      InstructionType ip;
      if (!memory_->GetMemoryAtAddress(location, &ip))
        break;

      // The return address points to the instruction after a call. If the
      // caller was a no return function, this might point past the end of the
      // function. Subtract one from the instruction pointer so it points into
      // the call instruction instead.
      if (modules_ && modules_->GetModuleForAddress(ip  - 1) &&
          InstructionAddressSeemsValid(ip - 1)) {
        *ip_found = ip;
        *location_found = location;
        return true;
      }
    }
    // nothing found
    return false;
  }

  // Information about the system that produced the minidump.  Subclasses
  // and the SymbolSupplier may find this information useful.
  const SystemInfo* system_info_;

  // The stack memory to walk.  Subclasses will require this region to
  // get information from the stack.
  MemoryRegion* memory_;

  // A list of modules, for populating each StackFrame's module information.
  // This field is optional and may be NULL.
  const CodeModules* modules_;

  // A list of unloaded modules, for populating frames which aren't matched
  // to any loaded modules.
  // This field is optional and may be NULL.
  const CodeModules* unloaded_modules_;

 protected:
  // The StackFrameSymbolizer implementation.
  StackFrameSymbolizer* frame_symbolizer_;

 private:
  // Obtains the context frame, the innermost called procedure in a stack
  // trace.  Returns NULL on failure.  GetContextFrame allocates a new
  // StackFrame (or StackFrame subclass), ownership of which is taken by
  // the caller.
  virtual StackFrame* GetContextFrame() = 0;

  // Obtains a caller frame.  Each call to GetCallerFrame should return the
  // frame that called the last frame returned by GetContextFrame or
  // GetCallerFrame.  To aid this purpose, stack contains the CallStack
  // made of frames that have already been walked.  GetCallerFrame should
  // return NULL on failure or when there are no more caller frames (when
  // the end of the stack has been reached).  GetCallerFrame allocates a new
  // StackFrame (or StackFrame subclass), ownership of which is taken by
  // the caller.  |stack_scan_allowed| controls whether stack scanning is
  // an allowable frame-recovery method, since it is desirable to be able to
  // disable stack scanning in performance-critical use cases.
  //
  // CONSIDER: a way to differentiate between:
  //  - full stack traces
  //  - explicitly truncated traces (max_frames_)
  //  - stopping after max scanned frames
  //  - failed stack walk (breaking one of the stack walk invariants)
  //
  virtual StackFrame* GetCallerFrame(const CallStack* stack,
                                     bool stack_scan_allowed) = 0;

  // The maximum number of frames Stackwalker will walk through.
  // This defaults to 1024 to prevent infinite loops.
  static uint32_t max_frames_;

  // Keep track of whether max_frames_ has been set by the user, since
  // it affects whether or not an error message is printed in the case
  // where an unwind got stopped by the limit.
  static bool max_frames_set_;

  // The maximum number of stack-scanned and otherwise untrustworthy
  // frames allowed.  Stack-scanning can be expensive, so the option to
  // disable or limit it is helpful in cases where unwind performance is
  // important.  This defaults to 1024, the same as max_frames_.
  static uint32_t max_frames_scanned_;
};

}  // namespace google_breakpad


#endif  // GOOGLE_BREAKPAD_PROCESSOR_STACKWALKER_H__
