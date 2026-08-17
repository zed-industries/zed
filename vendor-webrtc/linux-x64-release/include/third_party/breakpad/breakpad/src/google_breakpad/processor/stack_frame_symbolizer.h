// -*- mode: C++ -*-

// Copyright 2012 Google LLC
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

// Helper class that encapsulates the logic of how symbol supplier interacts
// with source line resolver to fill stack frame information.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_STACK_FRAME_SYMBOLIZER_H__
#define GOOGLE_BREAKPAD_PROCESSOR_STACK_FRAME_SYMBOLIZER_H__

#include <deque>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/code_module.h"

namespace google_breakpad {
class CFIFrameInfo;
class CodeModules;
class SymbolSupplier;
class SourceLineResolverInterface;
struct StackFrame;
struct SystemInfo;
struct WindowsFrameInfo;

class StackFrameSymbolizer {
 public:
  enum SymbolizerResult {
    // Symbol data was found and successfully loaded in resolver.
    // This does NOT guarantee source line info is found within symbol file.
    kNoError,
    // This indicates non-critical error, such as, no code module found for
    // frame's instruction, no symbol file, or resolver failed to load symbol.
    kError,
    // This indicates error for which stack walk should be interrupted
    // and retried in future.
    kInterrupt,
    // Symbol data was found and loaded in resolver however some corruptions
    // were detected.
    kWarningCorruptSymbols,
  };

  StackFrameSymbolizer(SymbolSupplier* supplier,
                       SourceLineResolverInterface* resolver);

  virtual ~StackFrameSymbolizer() { }

  // Encapsulate the step of resolving source line info for a stack frame.
  // "frame" must not be NULL.
  virtual SymbolizerResult FillSourceLineInfo(
      const CodeModules* modules,
      const CodeModules* unloaded_modules,
      const SystemInfo* system_info,
      StackFrame* stack_frame,
      std::deque<std::unique_ptr<StackFrame>>* inlined_frames);

  virtual WindowsFrameInfo* FindWindowsFrameInfo(const StackFrame* frame);

  virtual CFIFrameInfo* FindCFIFrameInfo(const StackFrame* frame);

  // Reset internal (locally owned) data as if the helper is re-instantiated.
  // A typical case is to call Reset() after processing an individual report
  // before start to process next one, in order to reset internal information
  // about missing symbols found so far.
  virtual void Reset() { no_symbol_modules_.clear(); }

  // Returns true if there is valid implementation for stack symbolization.
  virtual bool HasImplementation() { return resolver_ && supplier_; }

  SourceLineResolverInterface* resolver() { return resolver_; }
  SymbolSupplier* supplier() { return supplier_; }

 protected:
  SymbolSupplier* supplier_;
  SourceLineResolverInterface* resolver_;
  // A list of modules known to have symbols missing. This helps avoid
  // repeated lookups for the missing symbols within one minidump.
  std::set<string> no_symbol_modules_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_STACK_FRAME_SYMBOLIZER_H__
