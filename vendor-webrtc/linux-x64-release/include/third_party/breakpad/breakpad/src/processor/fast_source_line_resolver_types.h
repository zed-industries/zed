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
// fast_source_line_resolver_types.h: definition of nested classes/structs in
// FastSourceLineResolver.  It moves the definitions out of
// fast_source_line_resolver.cc, so that other classes could have access
// to these private nested types without including fast_source_line_resolver.cc
//
// Author: lambxsy@google.com (Siyang Xie)

#ifndef PROCESSOR_FAST_SOURCE_LINE_RESOLVER_TYPES_H__
#define PROCESSOR_FAST_SOURCE_LINE_RESOLVER_TYPES_H__

#include <stdint.h>

#include <map>
#include <string>

#include "google_breakpad/processor/fast_source_line_resolver.h"
#include "google_breakpad/processor/stack_frame.h"
#include "processor/cfi_frame_info.h"
#include "processor/contained_range_map.h"
#include "processor/simple_serializer-inl.h"
#include "processor/source_line_resolver_base_types.h"
#include "processor/static_address_map-inl.h"
#include "processor/static_contained_range_map-inl.h"
#include "processor/static_map.h"
#include "processor/static_range_map-inl.h"
#include "processor/windows_frame_info.h"

namespace google_breakpad {

#define DESERIALIZE(raw_ptr, field)                             \
  field = *(reinterpret_cast<const decltype(field)*>(raw_ptr)); \
  raw_ptr += sizeof(field);

struct FastSourceLineResolver::Line : public SourceLineResolverBase::Line {
  void CopyFrom(const Line* line_ptr) {
    const char* raw = reinterpret_cast<const char*>(line_ptr);
    CopyFrom(raw);
  }

  // De-serialize the memory data of a Line.
  void CopyFrom(const char* raw) {
    DESERIALIZE(raw, address);
    DESERIALIZE(raw, size);
    DESERIALIZE(raw, source_file_id);
    DESERIALIZE(raw, line);
  }
};

struct FastSourceLineResolver::Function :
public SourceLineResolverBase::Function {
  void CopyFrom(const Function* func_ptr) {
    const char* raw = reinterpret_cast<const char*>(func_ptr);
    CopyFrom(raw);
  }

  // De-serialize the memory data of a Function.
  void CopyFrom(const char* raw) {
    size_t name_size = strlen(raw) + 1;
    name = raw;
    raw += name_size;
    DESERIALIZE(raw, address);
    DESERIALIZE(raw, size);
    DESERIALIZE(raw, parameter_size);
    raw = SimpleSerializer<bool>::Read(raw, &is_multiple);
    int32_t inline_size;
    DESERIALIZE(raw, inline_size);
    inlines = StaticContainedRangeMap<MemAddr, char>(raw);
    lines = StaticRangeMap<MemAddr, Line>(raw + inline_size);
  }

  StaticContainedRangeMap<MemAddr, char> inlines;
  StaticRangeMap<MemAddr, Line> lines;
};

struct FastSourceLineResolver::Inline : public SourceLineResolverBase::Inline {
  void CopyFrom(const Inline* inline_ptr) {
    const char* raw = reinterpret_cast<const char*>(inline_ptr);
    CopyFrom(raw);
  }

  // De-serialize the memory data of a Inline.
  void CopyFrom(const char* raw) {
    raw = SimpleSerializer<bool>::Read(raw, &has_call_site_file_id);
    DESERIALIZE(raw, inline_nest_level);
    DESERIALIZE(raw, call_site_line);
    DESERIALIZE(raw, call_site_file_id);
    DESERIALIZE(raw, origin_id);
    uint32_t inline_range_size;
    DESERIALIZE(raw, inline_range_size);
    for (size_t i = 0; i < inline_range_size; i += 2) {
      std::pair<MemAddr, MemAddr> range;
      DESERIALIZE(raw, range.first);
      DESERIALIZE(raw, range.second);
      inline_ranges.push_back(range);
    }
  }
};

struct FastSourceLineResolver::InlineOrigin
    : public SourceLineResolverBase::InlineOrigin {
  void CopyFrom(const InlineOrigin* origin_ptr) {
    const char* raw = reinterpret_cast<const char*>(origin_ptr);
    CopyFrom(raw);
  }

  // De-serialize the memory data of a Line.
  void CopyFrom(const char* raw) {
    DESERIALIZE(raw, has_file_id);
    DESERIALIZE(raw, source_file_id);
    name = raw;
  }
};

struct FastSourceLineResolver::PublicSymbol :
public SourceLineResolverBase::PublicSymbol {
  void CopyFrom(const PublicSymbol* public_symbol_ptr) {
    const char* raw = reinterpret_cast<const char*>(public_symbol_ptr);
    CopyFrom(raw);
  }

  // De-serialize the memory data of a PublicSymbol.
  void CopyFrom(const char* raw) {
    size_t name_size = strlen(raw) + 1;
    name = raw;
    raw += name_size;
    DESERIALIZE(raw, address);
    DESERIALIZE(raw, parameter_size);
    raw = SimpleSerializer<bool>::Read(raw, &is_multiple);
  }
};

#undef DESERIALIZE

class FastSourceLineResolver::Module: public SourceLineResolverBase::Module {
 public:
  explicit Module(const string& name) : name_(name), is_corrupt_(false) { }
  virtual ~Module() { }

  // Looks up the given relative address, and fills the StackFrame struct
  // with the result.
  virtual void LookupAddress(
      StackFrame* frame,
      std::deque<std::unique_ptr<StackFrame>>* inlined_frames) const;

  // Construct inlined frames for |frame| and store them in |inline_frames|.
  // |frame|'s source line and source file name may be updated if an inlined
  // frame is found inside |frame|. As a result, the innermost inlined frame
  // will be the first one in |inline_frames|.
  virtual void ConstructInlineFrames(
      StackFrame* frame,
      MemAddr address,
      const StaticContainedRangeMap<MemAddr, char>& inline_map,
      std::deque<std::unique_ptr<StackFrame>>* inlined_frames) const;

  // Loads a map from the given buffer in char* type.
  virtual bool LoadMapFromMemory(char* memory_buffer,
                                 size_t memory_buffer_size);

  // Tells whether the loaded symbol data is corrupt.  Return value is
  // undefined, if the symbol data hasn't been loaded yet.
  virtual bool IsCorrupt() const { return is_corrupt_; }

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

  // Number of serialized map components of Module.
  static const int kNumberMaps_ = 6 + WindowsFrameInfo::STACK_INFO_LAST;

 private:
  friend class FastSourceLineResolver;
  friend class ModuleComparer;
  typedef StaticMap<int, char> FileMap;

  string name_;
  StaticMap<int, char> files_;
  StaticRangeMap<MemAddr, Function> functions_;
  StaticAddressMap<MemAddr, PublicSymbol> public_symbols_;
  bool is_corrupt_;

  // Each element in the array is a ContainedRangeMap for a type
  // listed in WindowsFrameInfoTypes. These are split by type because
  // there may be overlaps between maps of different types, but some
  // information is only available as certain types.
  StaticContainedRangeMap<MemAddr, char>
    windows_frame_info_[WindowsFrameInfo::STACK_INFO_LAST];

  // DWARF CFI stack walking data. The Module stores the initial rule sets
  // and rule deltas as strings, just as they appear in the symbol file:
  // although the file may contain hundreds of thousands of STACK CFI
  // records, walking a stack will only ever use a few of them, so it's
  // best to delay parsing a record until it's actually needed.
  //
  // STACK CFI INIT records: for each range, an initial set of register
  // recovery rules. The RangeMap's itself gives the starting and ending
  // addresses.
  StaticRangeMap<MemAddr, char> cfi_initial_rules_;

  // STACK CFI records: at a given address, the changes to the register
  // recovery rules that take effect at that address. The map key is the
  // starting address; the ending address is the key of the next entry in
  // this map, or the end of the range as given by the cfi_initial_rules_
  // entry (which FindCFIFrameInfo looks up first).
  StaticMap<MemAddr, char> cfi_delta_rules_;

  // INLINE_ORIGIN records: used as a function name string pool for INLINE
  // records.
  StaticMap<int, char> inline_origins_;
};

}  // namespace google_breakpad

#endif  // PROCESSOR_FAST_SOURCE_LINE_RESOLVER_TYPES_H__
