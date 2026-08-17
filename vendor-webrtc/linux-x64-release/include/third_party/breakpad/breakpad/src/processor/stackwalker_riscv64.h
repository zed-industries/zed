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

/* stackwalker_riscv64.h: riscv64-specific stackwalker.
 *
 * Provides stack frames given riscv64 register context and a memory region
 * corresponding to a riscv64 stack.
 *
 * Author: Iacopo Colonnelli
 */

#ifndef PROCESSOR_STACKWALKER_RISCV64_H__
#define PROCESSOR_STACKWALKER_RISCV64_H__

#include "google_breakpad/common/minidump_format.h"
#include "google_breakpad/processor/stackwalker.h"

namespace google_breakpad {

class CodeModules;

class StackwalkerRISCV64 : public Stackwalker {
public:
  // Context is a riscv context object that gives access to riscv-specific
  // register state corresponding to the innermost called frame to be
  // included in the stack.  The other arguments are passed directly
  // through to the base Stackwalker constructor.
  StackwalkerRISCV64(const SystemInfo* system_info,
                     const MDRawContextRISCV64* context,
                     MemoryRegion* memory,
                     const CodeModules* modules,
                     StackFrameSymbolizer* frame_symbolizer);

  // Change the context validity mask of the frame returned by
  // GetContextFrame to VALID. This is only for use by unit tests; the
  // default behavior is correct for all application code.
  void SetContextFrameValidity(int valid) {
    context_frame_validity_ = valid;
  }

private:
  // Implementation of Stackwalker, using riscv context and stack conventions.
  virtual StackFrame* GetContextFrame();
  virtual StackFrame* GetCallerFrame(
      const CallStack* stack, bool stack_scan_allowed);

  // Use cfi_frame_info (derived from STACK CFI records) to construct
  // the frame that called frames.back(). The caller takes ownership
  // of the returned frame. Return NULL on failure.
  StackFrameRISCV64* GetCallerByCFIFrameInfo(
      const vector<StackFrame*>& frames, CFIFrameInfo* cfi_frame_info);

  // Use the frame pointer. The caller takes ownership of the returned frame.
  // Return NULL on failure.
  StackFrameRISCV64* GetCallerByFramePointer(
      const vector<StackFrame*>& frames);

  // Scan the stack for plausible return addresses. The caller takes ownership
  // of the returned frame. Return NULL on failure.
  StackFrameRISCV64* GetCallerByStackScan(
      const vector<StackFrame*>& frames);

  // Stores the CPU context corresponding to the innermost stack frame to
  // be returned by GetContextFrame.
  const MDRawContextRISCV64* context_;

  // Validity mask for youngest stack frame. This is always
  // CONTEXT_VALID_ALL in real use; it is only changeable for the sake of
  // unit tests.
  int context_frame_validity_;
};

}  // namespace google_breakpad

#endif // PROCESSOR_STACKWALKER_RISCV64_H__
