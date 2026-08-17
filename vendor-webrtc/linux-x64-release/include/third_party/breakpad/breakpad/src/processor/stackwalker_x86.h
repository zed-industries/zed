// -*- mode: c++ -*-

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

// stackwalker_x86.h: x86-specific stackwalker.
//
// Provides stack frames given x86 register context and a memory region
// corresponding to an x86 stack.
//
// Author: Mark Mentovai


#ifndef PROCESSOR_STACKWALKER_X86_H__
#define PROCESSOR_STACKWALKER_X86_H__

#include <vector>

#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/common/minidump_format.h"
#include "google_breakpad/processor/stackwalker.h"
#include "google_breakpad/processor/stack_frame_cpu.h"
#include "processor/cfi_frame_info.h"

namespace google_breakpad {

class CodeModules;


class StackwalkerX86 : public Stackwalker {
 public:
  // context is an x86 context object that gives access to x86-specific
  // register state corresponding to the innermost called frame to be
  // included in the stack.  The other arguments are passed directly through
  // to the base Stackwalker constructor.
  StackwalkerX86(const SystemInfo* system_info,
                 const MDRawContextX86* context,
                 MemoryRegion* memory,
                 const CodeModules* modules,
                 StackFrameSymbolizer* frame_symbolizer);

 private:
  // A STACK CFI-driven frame walker for the X86.
  typedef SimpleCFIWalker<uint32_t, MDRawContextX86> CFIWalker;

  // Implementation of Stackwalker, using x86 context (%ebp, %esp, %eip) and
  // stack conventions (saved %ebp at [%ebp], saved %eip at 4[%ebp], or
  // alternate conventions as guided by any WindowsFrameInfo available for the
  // code in question.).
  virtual StackFrame* GetContextFrame();
  virtual StackFrame* GetCallerFrame(const CallStack* stack,
                                     bool stack_scan_allowed);

  // Use windows_frame_info (derived from STACK WIN and FUNC records)
  // to construct the frame that called frames.back(). The caller
  // takes ownership of the returned frame. Return NULL on failure.
  StackFrameX86* GetCallerByWindowsFrameInfo(
      const vector<StackFrame*>& frames,
      WindowsFrameInfo* windows_frame_info,
      bool stack_scan_allowed);

  // Use cfi_frame_info (derived from STACK CFI records) to construct
  // the frame that called frames.back(). The caller takes ownership
  // of the returned frame. Return NULL on failure.
  StackFrameX86* GetCallerByCFIFrameInfo(const vector<StackFrame*>& frames,
                                         CFIFrameInfo* cfi_frame_info);

  // Assuming a traditional frame layout --- where the caller's %ebp
  // has been pushed just after the return address and the callee's
  // %ebp points to the saved %ebp --- construct the frame that called
  // frames.back(). The caller takes ownership of the returned frame.
  // Return NULL on failure.
  StackFrameX86* GetCallerByEBPAtBase(const vector<StackFrame*>& frames,
                                      bool stack_scan_allowed);

  // Stores the CPU context corresponding to the innermost stack frame to
  // be returned by GetContextFrame.
  const MDRawContextX86* context_;

  // Our register map, for cfi_walker_.
  static const CFIWalker::RegisterSet cfi_register_map_[];

  // Our CFI frame walker.
  const CFIWalker cfi_walker_;
};


}  // namespace google_breakpad


#endif  // PROCESSOR_STACKWALKER_X86_H__
