// -*- mode: C++ -*-

// Copyright 2013 Google LLC
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

// stackwalker_arm64.h: arm64-specific stackwalker.
//
// Provides stack frames given arm64 register context and a memory region
// corresponding to an arm64 stack.
//
// Author: Mark Mentovai, Ted Mielczarek, Colin Blundell


#ifndef PROCESSOR_STACKWALKER_ARM64_H__
#define PROCESSOR_STACKWALKER_ARM64_H__

#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/common/minidump_format.h"
#include "google_breakpad/processor/stack_frame_cpu.h"
#include "google_breakpad/processor/stackwalker.h"

namespace google_breakpad {

class CodeModules;

class StackwalkerARM64 : public Stackwalker {
 public:
  // context is an arm64 context object that gives access to arm64-specific
  // register state corresponding to the innermost called frame to be
  // included in the stack.  The other arguments are passed directly through
  // to the base Stackwalker constructor.
  StackwalkerARM64(const SystemInfo* system_info,
                   const MDRawContextARM64* context,
                   MemoryRegion* memory,
                   const CodeModules* modules,
                   StackFrameSymbolizer* frame_symbolizer);

  // Change the context validity mask of the frame returned by
  // GetContextFrame to VALID. This is only for use by unit tests; the
  // default behavior is correct for all application code.
  void SetContextFrameValidity(uint64_t valid) {
    context_frame_validity_ = valid;
  }

 private:
  // Strip pointer authentication codes from an address.
  uint64_t PtrauthStrip(uint64_t ptr);

  // Implementation of Stackwalker, using arm64 context and stack conventions.
  virtual StackFrame* GetContextFrame();
  virtual StackFrame* GetCallerFrame(const CallStack* stack,
                                     bool stack_scan_allowed);

  // Use cfi_frame_info (derived from STACK CFI records) to construct
  // the frame that called frames.back(). The caller takes ownership
  // of the returned frame. Return NULL on failure.
  StackFrameARM64* GetCallerByCFIFrameInfo(const vector<StackFrame*>& frames,
                                           CFIFrameInfo* cfi_frame_info);

  // Use the frame pointer. The caller takes ownership of the returned frame.
  // Return NULL on failure.
  StackFrameARM64* GetCallerByFramePointer(const vector<StackFrame*>& frames);

  // Scan the stack for plausible return addresses. The caller takes ownership
  // of the returned frame. Return NULL on failure.
  StackFrameARM64* GetCallerByStackScan(const vector<StackFrame*>& frames);

  // GetCallerByFramePointer() depends on the previous frame having recovered
  // x30($LR) which may not have been done when using CFI.
  // This function recovers $LR in the previous frame by using the frame-pointer
  // two frames back to read it from the stack.
  void CorrectRegLRByFramePointer(const vector<StackFrame*>& frames,
                                  StackFrameARM64* last_frame);

  // Stores the CPU context corresponding to the youngest stack frame, to
  // be returned by GetContextFrame.
  const MDRawContextARM64* context_;

  // Validity mask for youngest stack frame. This is always
  // CONTEXT_VALID_ALL in real use; it is only changeable for the sake of
  // unit tests.
  uint64_t context_frame_validity_;

  // A mask of the valid address bits, determined from the address range of
  // modules_.
  uint64_t address_range_mask_;
};


}  // namespace google_breakpad


#endif  // PROCESSOR_STACKWALKER_ARM64_H__
