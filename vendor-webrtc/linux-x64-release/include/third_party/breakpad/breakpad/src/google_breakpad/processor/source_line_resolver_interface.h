// -*- mode: C++ -*-

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

// Abstract interface to return function/file/line info for a memory address.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_SOURCE_LINE_RESOLVER_INTERFACE_H__
#define GOOGLE_BREAKPAD_PROCESSOR_SOURCE_LINE_RESOLVER_INTERFACE_H__

#include <deque>
#include <memory>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"
#include "google_breakpad/processor/code_module.h"

namespace google_breakpad {

struct StackFrame;
struct WindowsFrameInfo;
class CFIFrameInfo;

class SourceLineResolverInterface {
 public:
  typedef uint64_t MemAddr;

  virtual ~SourceLineResolverInterface() {}

  // Adds a module to this resolver, returning true on success.
  //
  // module should have at least the code_file, debug_file,
  // and debug_identifier members populated.
  //
  // map_file should contain line/address mappings for this module.
  virtual bool LoadModule(const CodeModule* module,
                          const string& map_file) = 0;
  // Same as above, but takes the contents of a pre-read map buffer
  virtual bool LoadModuleUsingMapBuffer(const CodeModule* module,
                                        const string& map_buffer) = 0;

  // Add an interface to load symbol using C-String data instead of string.
  // This is useful in the optimization design for avoiding unnecessary copying
  // of symbol data, in order to improve memory efficiency.
  // LoadModuleUsingMemoryBuffer() does NOT take ownership of memory_buffer.
  // LoadModuleUsingMemoryBuffer() null terminates the passed in buffer, if
  // the last character is not a null terminator.
  virtual bool LoadModuleUsingMemoryBuffer(const CodeModule* module,
                                           char* memory_buffer,
                                           size_t memory_buffer_size) = 0;

  // Return true if the memory buffer should be deleted immediately after
  // LoadModuleUsingMemoryBuffer(). Return false if the memory buffer has to be
  // alive during the lifetime of the corresponding Module.
  virtual bool ShouldDeleteMemoryBufferAfterLoadModule() = 0;

  // Request that the specified module be unloaded from this resolver.
  // A resolver may choose to ignore such a request.
  virtual void UnloadModule(const CodeModule* module) = 0;

  // Returns true if the module has been loaded.
  virtual bool HasModule(const CodeModule* module) = 0;

  // Returns true if the module has been loaded and it is corrupt.
  virtual bool IsModuleCorrupt(const CodeModule* module) = 0;

  // Fills in the function_base, function_name, source_file_name,
  // and source_line fields of the StackFrame.  The instruction and
  // module_name fields must already be filled in. If inlined_frames is not
  // nullptr, it will try to construct inlined frames by adding them into
  // inlined_frames in an order from outermost frame to inner most frame.
  virtual void FillSourceLineInfo(
      StackFrame* frame,
      std::deque<std::unique_ptr<StackFrame>>* inlined_frames) = 0;

  // If Windows stack walking information is available covering
  // FRAME's instruction address, return a WindowsFrameInfo structure
  // describing it. If the information is not available, returns NULL.
  // A NULL return value does not indicate an error. The caller takes
  // ownership of any returned WindowsFrameInfo object.
  virtual WindowsFrameInfo* FindWindowsFrameInfo(const StackFrame* frame) = 0;

  // If CFI stack walking information is available covering ADDRESS,
  // return a CFIFrameInfo structure describing it. If the information
  // is not available, return NULL. The caller takes ownership of any
  // returned CFIFrameInfo object.
  virtual CFIFrameInfo* FindCFIFrameInfo(const StackFrame* frame) = 0;

 protected:
  // SourceLineResolverInterface cannot be instantiated except by subclasses
  SourceLineResolverInterface() {}
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_SOURCE_LINE_RESOLVER_INTERFACE_H__
