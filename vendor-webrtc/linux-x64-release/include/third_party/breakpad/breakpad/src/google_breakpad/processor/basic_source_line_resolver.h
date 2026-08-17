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

// basic_source_line_resolver.h: BasicSourceLineResolver is derived from
// SourceLineResolverBase, and is a concrete implementation of
// SourceLineResolverInterface, using address map files produced by a
// compatible writer, e.g. PDBSourceLineWriter.
//
// see "processor/source_line_resolver_base.h"
// and "source_line_resolver_interface.h" for more documentation.

#ifndef GOOGLE_BREAKPAD_PROCESSOR_BASIC_SOURCE_LINE_RESOLVER_H__
#define GOOGLE_BREAKPAD_PROCESSOR_BASIC_SOURCE_LINE_RESOLVER_H__

#include <map>
#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/processor/source_line_resolver_base.h"

namespace google_breakpad {

using std::map;

class BasicSourceLineResolver : public SourceLineResolverBase {
 public:
  BasicSourceLineResolver();
  virtual ~BasicSourceLineResolver() { }

  using SourceLineResolverBase::LoadModule;
  using SourceLineResolverBase::LoadModuleUsingMapBuffer;
  using SourceLineResolverBase::LoadModuleUsingMemoryBuffer;
  using SourceLineResolverBase::ShouldDeleteMemoryBufferAfterLoadModule;
  using SourceLineResolverBase::UnloadModule;
  using SourceLineResolverBase::HasModule;
  using SourceLineResolverBase::IsModuleCorrupt;
  using SourceLineResolverBase::FillSourceLineInfo;
  using SourceLineResolverBase::FindWindowsFrameInfo;
  using SourceLineResolverBase::FindCFIFrameInfo;

 private:
  // friend declarations:
  friend class BasicModuleFactory;
  friend class ModuleComparer;
  friend class ModuleSerializer;
  template<class> friend class SimpleSerializer;

  // Function derives from SourceLineResolverBase::Function.
  struct Function;
  // Module implements SourceLineResolverBase::Module interface.
  class Module;

  // Disallow unwanted copy ctor and assignment operator
  BasicSourceLineResolver(const BasicSourceLineResolver&);
  void operator=(const BasicSourceLineResolver&);
};

// Helper class, containing useful methods for parsing of Breakpad symbol files.
class SymbolParseHelper {
 public:
  using MemAddr = SourceLineResolverInterface::MemAddr;

  // Parses a |file_line| declaration.  Returns true on success.
  // Format: FILE <id> <filename>.
  // Notice, that this method modifies the input |file_line| which is why it
  // can't be const.  On success, <id>, and <filename> are stored in |*index|,
  // and |*filename|.  No allocation is done, |*filename| simply points inside
  // |file_line|.
  static bool ParseFile(char* file_line,   // in
                        long* index,       // out
                        char** filename);  // out

  // Parses a |inline_origin_line| declaration.  Returns true on success.
  // Old Format: INLINE_ORIGIN <origin_id> <file_id> <name>.
  // New Format: INLINE_ORIGIN <origin_id> <name>.
  // Notice, that this method modifies the input |inline_origin_line| which is
  // why it can't be const.  On success, <has_file_id>, <origin_id>, <file_id>
  // and <name> are stored in |*has_file_id*|, |*origin_id|, |*file_id|, and
  // |*name|.  No allocation is done, |*name| simply points inside
  // |inline_origin_line|.
  static bool ParseInlineOrigin(char* inline_origin_line,  // in
                                bool* has_file_id,       // out
                                long* origin_id,           // out
                                long* file_id,             // out
                                char** name);              // out

  // Parses a |inline| declaration.  Returns true on success.
  // Old Format: INLINE <inline_nest_level> <call_site_line> <origin_id>
  // [<address> <size>]+
  // New Format: INLINE <inline_nest_level> <call_site_line> <call_site_file_id>
  // <origin_id> [<address> <size>]+
  // Notice, that this method modifies the input |inline|
  // which is why it can't be const.  On success, <has_call_site_file_id>,
  // <inline_nest_level>, <call_site_line> and <origin_id> are stored in
  // |*has_call_site_file_id*|, |*inline_nest_level|, |*call_site_line|, and
  // |*origin_id|, and all pairs of (<address>, <size>) are added into ranges.
  static bool ParseInline(
      char* inline_line,                                  // in
      bool* has_call_site_file_id,                        // out
      long* inline_nest_level,                            // out
      long* call_site_line,                               // out
      long* call_site_file_id,                            // out
      long* origin_id,                                    // out
      std::vector<std::pair<MemAddr, MemAddr>>* ranges);  // out

  // Parses a |function_line| declaration.  Returns true on success.
  // Format:  FUNC [<multiple>] <address> <size> <stack_param_size> <name>.
  // Notice, that this method modifies the input |function_line| which is why it
  // can't be const.  On success, the presence of <multiple>, <address>, <size>,
  // <stack_param_size>, and <name> are stored in |*is_multiple|, |*address|,
  // |*size|, |*stack_param_size|, and |*name|.  No allocation is done, |*name|
  // simply points inside |function_line|.
  static bool ParseFunction(char* function_line,     // in
                            bool* is_multiple,       // out
                            uint64_t* address,       // out
                            uint64_t* size,          // out
                            long* stack_param_size,  // out
                            char** name);            // out

  // Parses a |line| declaration.  Returns true on success.
  // Format:  <address> <size> <line number> <source file id>
  // Notice, that this method modifies the input |function_line| which is why
  // it can't be const.  On success, <address>, <size>, <line number>, and
  // <source file id> are stored in |*address|, |*size|, |*line_number|, and
  // |*source_file|.
  static bool ParseLine(char* line_line,     // in
                        uint64_t* address,   // out
                        uint64_t* size,      // out
                        long* line_number,   // out
                        long* source_file);  // out

  // Parses a |public_line| declaration.  Returns true on success.
  // Format:  PUBLIC [<multiple>] <address> <stack_param_size> <name>
  // Notice, that this method modifies the input |function_line| which is why
  // it can't be const.  On success, the presence of <multiple>, <address>,
  // <stack_param_size>, <name> are stored in |*is_multiple|, |*address|,
  // |*stack_param_size|, and |*name|.  No allocation is done, |*name| simply
  // points inside |public_line|.
  static bool ParsePublicSymbol(char* public_line,       // in
                                bool* is_multiple,       // out
                                uint64_t* address,       // out
                                long* stack_param_size,  // out
                                char** name);            // out

 private:
  // Used for success checks after strtoull and strtol.
  static bool IsValidAfterNumber(char* after_number);

  // Only allow static methods.
  SymbolParseHelper();
  SymbolParseHelper(const SymbolParseHelper&);
  void operator=(const SymbolParseHelper&);
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_PROCESSOR_BASIC_SOURCE_LINE_RESOLVER_H__
