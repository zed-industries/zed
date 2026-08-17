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
//
// source_line_resolver_base.h: SourceLineResolverBase, an (incomplete)
// implementation of SourceLineResolverInterface.  It serves as a common base
// class for concrete implementations: FastSourceLineResolver and
// BasicSourceLineResolver.  It is designed for refactoring that removes
// code redundancy in the two concrete source line resolver classes.
//
// See "google_breakpad/processor/source_line_resolver_interface.h" for more
// documentation.

// Author: Siyang Xie (lambxsy@google.com)

#ifndef GOOGLE_BREAKPAD_PROCESSOR_SOURCE_LINE_RESOLVER_BASE_H__
#define GOOGLE_BREAKPAD_PROCESSOR_SOURCE_LINE_RESOLVER_BASE_H__

#include <deque>
#include <map>
#include <set>
#include <string>

#include "google_breakpad/processor/source_line_resolver_interface.h"

namespace google_breakpad {

using std::map;
using std::set;

// Forward declaration.
// ModuleFactory is a simple factory interface for creating a Module instance
// at run-time.
class ModuleFactory;

class SourceLineResolverBase : public SourceLineResolverInterface {
 public:
  // Read the symbol_data from a file with given file_name.
  // The part of code was originally in BasicSourceLineResolver::Module's
  // LoadMap() method.
  // Place dynamically allocated heap buffer in symbol_data. Caller has the
  // ownership of the buffer, and should call delete [] to free the buffer.
  static bool ReadSymbolFile(const string& file_name,
                             char** symbol_data,
                             size_t* symbol_data_size);

 protected:
  // Users are not allowed create SourceLineResolverBase instance directly.
  SourceLineResolverBase(ModuleFactory* module_factory);
  virtual ~SourceLineResolverBase();

  // Virtual methods inherited from SourceLineResolverInterface.
  virtual bool LoadModule(const CodeModule* module, const string& map_file);
  virtual bool LoadModuleUsingMapBuffer(const CodeModule* module,
                                        const string& map_buffer);
  virtual bool LoadModuleUsingMemoryBuffer(const CodeModule* module,
                                           char* memory_buffer,
                                           size_t memory_buffer_size);
  virtual bool ShouldDeleteMemoryBufferAfterLoadModule();
  virtual void UnloadModule(const CodeModule* module);
  virtual bool HasModule(const CodeModule* module);
  virtual bool IsModuleCorrupt(const CodeModule* module);
  virtual void FillSourceLineInfo(
      StackFrame* frame,
      std::deque<std::unique_ptr<StackFrame>>* inlined_frames);
  virtual WindowsFrameInfo* FindWindowsFrameInfo(const StackFrame* frame);
  virtual CFIFrameInfo* FindCFIFrameInfo(const StackFrame* frame);

  // Nested structs and classes.
  struct InlineOrigin;
  struct Inline;
  struct Line;
  struct Function;
  struct PublicSymbol;
  struct CompareString {
    bool operator()(const string& s1, const string& s2) const;
  };
  // Module is an interface for an in-memory symbol file.
  class Module;
  class AutoFileCloser;

  // All of the modules that are loaded.
  typedef map<string, Module*, CompareString> ModuleMap;
  ModuleMap* modules_;

  // The loaded modules that were detecting to be corrupt during load.
  typedef set<string, CompareString> ModuleSet;
  ModuleSet* corrupt_modules_;

  // All of heap-allocated buffers that are owned locally by resolver.
  typedef std::map<string, char*, CompareString> MemoryMap;
  MemoryMap* memory_buffers_;

  // Creates a concrete module at run-time.
  ModuleFactory* module_factory_;

 private:
  // ModuleFactory needs to have access to protected type Module.
  friend class ModuleFactory;

  // Disallow unwanted copy ctor and assignment operator
  SourceLineResolverBase(const SourceLineResolverBase&);
  void operator=(const SourceLineResolverBase&);
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_SOURCE_LINE_RESOLVER_BASE_H__
