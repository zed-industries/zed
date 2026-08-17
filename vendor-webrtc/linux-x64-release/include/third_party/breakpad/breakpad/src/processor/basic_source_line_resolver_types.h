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
// basic_source_line_types.h: definition of nested classes/structs in
// BasicSourceLineResolver.  It moves the definitions out of
// basic_source_line_resolver.cc, so that other classes could have access
// to these private nested types without including basic_source_line_resolver.cc
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_BASIC_SOURCE_LINE_RESOLVER_TYPES_H__
#define PROCESSOR_BASIC_SOURCE_LINE_RESOLVER_TYPES_H__

#include <map>
#include <string>

#include "google_breakpad/processor/basic_source_line_resolver.h"
#include "processor/source_line_resolver_base_types.h"

#include "processor/address_map-inl.h"
#include "processor/range_map-inl.h"
#include "processor/contained_range_map-inl.h"

#include "processor/linked_ptr.h"
#include "google_breakpad/processor/stack_frame.h"
#include "processor/cfi_frame_info.h"
#include "processor/windows_frame_info.h"

namespace google_breakpad {

struct
BasicSourceLineResolver::Function : public SourceLineResolverBase::Function {
  Function(const string& function_name,
           MemAddr function_address,
           MemAddr code_size,
           int set_parameter_size,
           bool is_mutiple)
      : Base(function_name,
             function_address,
             code_size,
             set_parameter_size,
             is_mutiple),
        inlines(true),
        last_added_inline_nest_level(0) {}

  // Append inline into corresponding RangeMap.
  // This function assumes it's called in the order of reading INLINE records.
  bool AppendInline(linked_ptr<Inline> in);

  ContainedRangeMap<MemAddr, linked_ptr<Inline>> inlines;
  RangeMap<MemAddr, linked_ptr<Line>> lines;

 private:
  typedef SourceLineResolverBase::Function Base;

  // The last added inline_nest_level from INLINE record.
  int last_added_inline_nest_level;
};


class BasicSourceLineResolver::Module : public SourceLineResolverBase::Module {
 public:
  explicit Module(const string& name) : name_(name), is_corrupt_(false) { }
  virtual ~Module() { }

  // Loads a map from the given buffer in char* type.
  // Does NOT have ownership of memory_buffer.
  // The passed in |memory buffer| is of size |memory_buffer_size|.  If it is
  // not null terminated, LoadMapFromMemory() will null terminate it by
  // modifying the passed in buffer.
  virtual bool LoadMapFromMemory(char* memory_buffer,
                                 size_t memory_buffer_size);

  // Tells whether the loaded symbol data is corrupt.  Return value is
  // undefined, if the symbol data hasn't been loaded yet.
  virtual bool IsCorrupt() const { return is_corrupt_; }

  // Looks up the given relative address, and fills the StackFrame struct
  // with the result.
  virtual void LookupAddress(
      StackFrame* frame,
      std::deque<std::unique_ptr<StackFrame>>* inlined_frame) const;

  // Construct inlined frames for |frame| and store them in |inline_frames|.
  // |frame|'s source line and source file name may be updated if an inlined
  // frame is found inside |frame|. As a result, the innermost inlined frame
  // will be the first one in |inline_frames|.
  virtual void ConstructInlineFrames(
      StackFrame* frame,
      MemAddr address,
      const ContainedRangeMap<uint64_t, linked_ptr<Inline>>& inline_map,
      std::deque<std::unique_ptr<StackFrame>>* inline_frames) const;

  // If Windows stack walking information is available covering ADDRESS,
  // return a WindowsFrameInfo structure describing it. If the information
  // is not available, returns NULL. A NULL return value does not indicate
  // an error. The caller takes ownership of any returned WindowsFrameInfo
  // object.
  virtual WindowsFrameInfo* FindWindowsFrameInfo(const StackFrame* frame) const;

  // If CFI stack walking information is available covering ADDRESS,
  // return a CFIFrameInfo structure describing it. If the information
  // is not available, return NULL. The caller takes ownership of any
  // returned CFIFrameInfo object.
  virtual CFIFrameInfo* FindCFIFrameInfo(const StackFrame* frame) const;

 private:
  // Friend declarations.
  friend class BasicSourceLineResolver;
  friend class ModuleComparer;
  friend class ModuleSerializer;

  typedef std::map<int, string> FileMap;

  // Logs parse errors.  |*num_errors| is increased every time LogParseError is
  // called.
  static void LogParseError(
      const string& message,
      int line_number,
      int* num_errors);

  // Parses a file declaration
  bool ParseFile(char* file_line);

  // Parses an inline origin declaration.
  bool ParseInlineOrigin(char* inline_origin_line);

  // Parses an inline declaration.
  linked_ptr<Inline> ParseInline(char* inline_line);

  // Parses a function declaration, returning a new Function object.
  Function* ParseFunction(char* function_line);

  // Parses a line declaration, returning a new Line object.
  Line* ParseLine(char* line_line);

  // Parses a PUBLIC symbol declaration, storing it in public_symbols_.
  // Returns false if an error occurs.
  bool ParsePublicSymbol(char* public_line);

  // Parses a STACK WIN or STACK CFI frame info declaration, storing
  // it in the appropriate table.
  bool ParseStackInfo(char* stack_info_line);

  // Parses a STACK CFI record, storing it in cfi_frame_info_.
  bool ParseCFIFrameInfo(char* stack_info_line);

  string name_;
  FileMap files_;
  std::map<int, linked_ptr<InlineOrigin>> inline_origins_;
  RangeMap< MemAddr, linked_ptr<Function> > functions_;
  AddressMap< MemAddr, linked_ptr<PublicSymbol> > public_symbols_;
  bool is_corrupt_;

  // Each element in the array is a ContainedRangeMap for a type
  // listed in WindowsFrameInfoTypes. These are split by type because
  // there may be overlaps between maps of different types, but some
  // information is only available as certain types.
  ContainedRangeMap< MemAddr, linked_ptr<WindowsFrameInfo> >
    windows_frame_info_[WindowsFrameInfo::STACK_INFO_LAST];

  // DWARF CFI stack walking data. The Module stores the initial rule sets
  // and rule deltas as strings, just as they appear in the symbol file:
  // although the file may contain hundreds of thousands of STACK CFI
  // records, walking a stack will only ever use a few of them, so it's
  // best to delay parsing a record until it's actually needed.

  // STACK CFI INIT records: for each range, an initial set of register
  // recovery rules. The RangeMap's itself gives the starting and ending
  // addresses.
  RangeMap<MemAddr, string> cfi_initial_rules_;

  // STACK CFI records: at a given address, the changes to the register
  // recovery rules that take effect at that address. The map key is the
  // starting address; the ending address is the key of the next entry in
  // this map, or the end of the range as given by the cfi_initial_rules_
  // entry (which FindCFIFrameInfo looks up first).
  std::map<MemAddr, string> cfi_delta_rules_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_BASIC_SOURCE_LINE_RESOLVER_TYPES_H__
