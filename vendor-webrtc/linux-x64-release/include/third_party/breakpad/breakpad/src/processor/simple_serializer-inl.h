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
// simple_serializer-inl.h: template specializations for following types:
// bool, const char *(C-string), string,
// Line, Function, PublicSymbol, WindowsFrameInfo and their linked pointers.
//
// See simple_serializer.h for moredocumentation.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_SIMPLE_SERIALIZER_INL_H__
#define PROCESSOR_SIMPLE_SERIALIZER_INL_H__

#include "processor/simple_serializer.h"

#include <stdint.h>

#include <string>

#include "google_breakpad/processor/basic_source_line_resolver.h"
#include "processor/basic_source_line_resolver_types.h"
#include "processor/linked_ptr.h"
#include "processor/map_serializers-inl.h"
#include "processor/windows_frame_info.h"

namespace google_breakpad {

// Specializations of SimpleSerializer: bool
template<>
class SimpleSerializer<bool> {
 public:
  static size_t SizeOf(bool boolean) { return 1; }

  static char* Write(bool boolean, char* dest) {
    *dest = static_cast<char>(boolean? 255 : 0);
    return ++dest;
  }

  static const char* Read(const char* source, bool* value) {
    *value = ((*source) == 0 ? false : true);
    return ++source;
  }
};

// Specializations of SimpleSerializer: string
template<>
class SimpleSerializer<string> {
 public:
  static size_t SizeOf(const string& str) { return str.size() + 1; }

  static char* Write(const string& str, char* dest) {
    strcpy(dest, str.c_str());
    return dest + SizeOf(str);
  }
};

// Specializations of SimpleSerializer: C-string
template<>
class SimpleSerializer<const char*> {
 public:
  static size_t SizeOf(const char* cstring) {
    return strlen(cstring) + 1;
  }

  static char* Write(const char* cstring, char* dest) {
    strcpy(dest, cstring);
    return dest + SizeOf(cstring);
  }
};

// Specializations of SimpleSerializer: Line
template<>
class SimpleSerializer<BasicSourceLineResolver::Line> {
  typedef BasicSourceLineResolver::Line Line;
 public:
  static size_t SizeOf(const Line& line) {
    return SimpleSerializer<MemAddr>::SizeOf(line.address)
         + SimpleSerializer<MemAddr>::SizeOf(line.size)
         + SimpleSerializer<int32_t>::SizeOf(line.source_file_id)
         + SimpleSerializer<int32_t>::SizeOf(line.line);
  }
  static char* Write(const Line& line, char* dest) {
    dest = SimpleSerializer<MemAddr>::Write(line.address, dest);
    dest = SimpleSerializer<MemAddr>::Write(line.size, dest);
    dest = SimpleSerializer<int32_t>::Write(line.source_file_id, dest);
    dest = SimpleSerializer<int32_t>::Write(line.line, dest);
    return dest;
  }
};

// Specializations of SimpleSerializer: InlineOrigin
template <>
class SimpleSerializer<BasicSourceLineResolver::InlineOrigin> {
  typedef BasicSourceLineResolver::InlineOrigin InlineOrigin;

 public:
  static size_t SizeOf(const InlineOrigin& origin) {
    return SimpleSerializer<bool>::SizeOf(origin.has_file_id) +
           SimpleSerializer<int32_t>::SizeOf(origin.source_file_id) +
           SimpleSerializer<string>::SizeOf(origin.name);
  }
  static char* Write(const InlineOrigin& origin, char* dest) {
    dest = SimpleSerializer<bool>::Write(origin.has_file_id, dest);
    dest = SimpleSerializer<int32_t>::Write(origin.source_file_id, dest);
    dest = SimpleSerializer<string>::Write(origin.name, dest);
    return dest;
  }
};

// Specializations of SimpleSerializer: PublicSymbol
template<>
class SimpleSerializer<BasicSourceLineResolver::PublicSymbol> {
  typedef BasicSourceLineResolver::PublicSymbol PublicSymbol;
 public:
  static size_t SizeOf(const PublicSymbol& pubsymbol) {
    return SimpleSerializer<string>::SizeOf(pubsymbol.name)
         + SimpleSerializer<MemAddr>::SizeOf(pubsymbol.address)
         + SimpleSerializer<int32_t>::SizeOf(pubsymbol.parameter_size)
         + SimpleSerializer<bool>::SizeOf(pubsymbol.is_multiple);
  }
  static char* Write(const PublicSymbol& pubsymbol, char* dest) {
    dest = SimpleSerializer<string>::Write(pubsymbol.name, dest);
    dest = SimpleSerializer<MemAddr>::Write(pubsymbol.address, dest);
    dest = SimpleSerializer<int32_t>::Write(pubsymbol.parameter_size, dest);
    dest = SimpleSerializer<bool>::Write(pubsymbol.is_multiple, dest);
    return dest;
  }
};

// Specializations of SimpleSerializer: WindowsFrameInfo
template<>
class SimpleSerializer<WindowsFrameInfo> {
 public:
  static size_t SizeOf(const WindowsFrameInfo& wfi) {
    unsigned int size = 0;
    size += sizeof(int32_t);  // wfi.type_
    size += SimpleSerializer<int32_t>::SizeOf(wfi.valid);
    size += SimpleSerializer<uint32_t>::SizeOf(wfi.prolog_size);
    size += SimpleSerializer<uint32_t>::SizeOf(wfi.epilog_size);
    size += SimpleSerializer<uint32_t>::SizeOf(wfi.parameter_size);
    size += SimpleSerializer<uint32_t>::SizeOf(wfi.saved_register_size);
    size += SimpleSerializer<uint32_t>::SizeOf(wfi.local_size);
    size += SimpleSerializer<uint32_t>::SizeOf(wfi.max_stack_size);
    size += SimpleSerializer<bool>::SizeOf(wfi.allocates_base_pointer);
    size += SimpleSerializer<string>::SizeOf(wfi.program_string);
    return size;
  }
  static char* Write(const WindowsFrameInfo& wfi, char* dest) {
    dest = SimpleSerializer<int32_t>::Write(
        static_cast<const int32_t>(wfi.type_), dest);
    dest = SimpleSerializer<int32_t>::Write(wfi.valid, dest);
    dest = SimpleSerializer<uint32_t>::Write(wfi.prolog_size, dest);
    dest = SimpleSerializer<uint32_t>::Write(wfi.epilog_size, dest);
    dest = SimpleSerializer<uint32_t>::Write(wfi.parameter_size, dest);
    dest = SimpleSerializer<uint32_t>::Write(wfi.saved_register_size, dest);
    dest = SimpleSerializer<uint32_t>::Write(wfi.local_size, dest);
    dest = SimpleSerializer<uint32_t>::Write(wfi.max_stack_size, dest);
    dest = SimpleSerializer<bool>::Write(wfi.allocates_base_pointer, dest);
    return SimpleSerializer<string>::Write(wfi.program_string, dest);
  }
};

// Specializations of SimpleSerializer: Linked_ptr version of
// Line, InlineOrigin, Inline, Function, PublicSymbol, WindowsFrameInfo.
template<>
class SimpleSerializer< linked_ptr<BasicSourceLineResolver::Line> > {
  typedef BasicSourceLineResolver::Line Line;
 public:
  static size_t SizeOf(const linked_ptr<Line>& lineptr) {
    if (lineptr.get() == nullptr) return 0;
    return SimpleSerializer<Line>::SizeOf(*(lineptr.get()));
  }
  static char* Write(const linked_ptr<Line>& lineptr, char* dest) {
    if (lineptr.get())
      dest = SimpleSerializer<Line>::Write(*(lineptr.get()), dest);
    return dest;
  }
};

template <>
class SimpleSerializer<linked_ptr<BasicSourceLineResolver::InlineOrigin>> {
  typedef BasicSourceLineResolver::InlineOrigin InlineOrigin;

 public:
  static size_t SizeOf(const linked_ptr<InlineOrigin>& origin_ptr) {
    if (origin_ptr.get() == nullptr)
      return 0;
    return SimpleSerializer<InlineOrigin>::SizeOf(*(origin_ptr.get()));
  }
  static char* Write(const linked_ptr<InlineOrigin>& origin_ptr, char* dest) {
    if (origin_ptr.get())
      dest = SimpleSerializer<InlineOrigin>::Write(*(origin_ptr.get()), dest);
    return dest;
  }
};

// Specializations of SimpleSerializer: Inline
template <>
class SimpleSerializer<linked_ptr<BasicSourceLineResolver::Inline>>;
template <>
class SimpleSerializer<BasicSourceLineResolver::Inline> {
  typedef BasicSourceLineResolver::Inline Inline;

 public:
  inline static size_t SizeOf(const Inline& in);
  inline static char* Write(const Inline& in, char* dest);
};

template <>
class SimpleSerializer<linked_ptr<BasicSourceLineResolver::Inline>> {
  typedef BasicSourceLineResolver::Inline Inline;

 public:
  static size_t SizeOf(const linked_ptr<Inline>& inline_ptr) {
    if (inline_ptr.get() == nullptr)
      return 0;
    return SimpleSerializer<Inline>::SizeOf(*(inline_ptr.get()));
  }
  static char* Write(const linked_ptr<Inline>& inline_ptr, char* dest) {
    if (inline_ptr.get())
      dest = SimpleSerializer<Inline>::Write(*(inline_ptr.get()), dest);
    return dest;
  }
};

size_t SimpleSerializer<BasicSourceLineResolver::Inline>::SizeOf(
    const Inline& in) {
  return SimpleSerializer<bool>::SizeOf(in.has_call_site_file_id) +
         SimpleSerializer<int32_t>::SizeOf(in.inline_nest_level) +
         SimpleSerializer<int32_t>::SizeOf(in.call_site_line) +
         SimpleSerializer<int32_t>::SizeOf(in.call_site_file_id) +
         SimpleSerializer<int32_t>::SizeOf(in.origin_id) +
         sizeof(uint32_t) +  // This is to store the size of inline_ranges.
         (in.inline_ranges.size() * sizeof(MemAddr) * 2);
}

char* SimpleSerializer<BasicSourceLineResolver::Inline>::Write(const Inline& in,
                                                               char* dest) {
  dest = SimpleSerializer<bool>::Write(in.has_call_site_file_id, dest);
  dest = SimpleSerializer<int32_t>::Write(in.inline_nest_level, dest);
  dest = SimpleSerializer<int32_t>::Write(in.call_site_line, dest);
  dest = SimpleSerializer<int32_t>::Write(in.call_site_file_id, dest);
  dest = SimpleSerializer<int32_t>::Write(in.origin_id, dest);
  // Write the size of inline_ranges.
  dest = SimpleSerializer<int32_t>::Write(in.inline_ranges.size(), dest);
  for (const std::pair<MemAddr, MemAddr>& range : in.inline_ranges) {
    dest = SimpleSerializer<MemAddr>::Write(range.first, dest);
    dest = SimpleSerializer<MemAddr>::Write(range.second, dest);
  }
  return dest;
}

template<>
class SimpleSerializer<BasicSourceLineResolver::Function> {
  // Convenient type names.
  typedef BasicSourceLineResolver::Function Function;
  typedef BasicSourceLineResolver::Line Line;
  typedef BasicSourceLineResolver::Inline Inline;

 public:
  static size_t SizeOf(const Function& func) {
    unsigned int size = 0;
    size += SimpleSerializer<string>::SizeOf(func.name);
    size += SimpleSerializer<MemAddr>::SizeOf(func.address);
    size += SimpleSerializer<MemAddr>::SizeOf(func.size);
    size += SimpleSerializer<int32_t>::SizeOf(func.parameter_size);
    size += SimpleSerializer<bool>::SizeOf(func.is_multiple);
    // This extra size is used to store the size of serialized func.inlines, so
    // we know where to start de-serialize func.lines.
    size += sizeof(int32_t);
    size += inline_range_map_serializer_.SizeOf(&func.inlines);
    size += range_map_serializer_.SizeOf(func.lines);
    return size;
  }

  static char* Write(const Function& func, char* dest) {
    dest = SimpleSerializer<string>::Write(func.name, dest);
    dest = SimpleSerializer<MemAddr>::Write(func.address, dest);
    dest = SimpleSerializer<MemAddr>::Write(func.size, dest);
    dest = SimpleSerializer<int32_t>::Write(func.parameter_size, dest);
    dest = SimpleSerializer<bool>::Write(func.is_multiple, dest);
    char* old_dest = dest;
    dest += sizeof(int32_t);
    dest = inline_range_map_serializer_.Write(&func.inlines, dest);
    // Write the size of serialized func.inlines. The size doesn't include size
    // field itself.
    SimpleSerializer<MemAddr>::Write(dest - old_dest - sizeof(int32_t),
                                     old_dest);
    dest = range_map_serializer_.Write(func.lines, dest);
    return dest;
  }
 private:
  // This static member is defined in module_serializer.cc.
  static RangeMapSerializer<MemAddr, linked_ptr<Line>> range_map_serializer_;
  static ContainedRangeMapSerializer<MemAddr, linked_ptr<Inline>>
      inline_range_map_serializer_;
};

template<>
class SimpleSerializer< linked_ptr<BasicSourceLineResolver::Function> > {
  typedef BasicSourceLineResolver::Function Function;
 public:
  static size_t SizeOf(const linked_ptr<Function>& func) {
    if (!func.get()) return 0;
    return SimpleSerializer<Function>::SizeOf(*(func.get()));
  }

  static char* Write(const linked_ptr<Function>& func, char* dest) {
    if (func.get())
      dest = SimpleSerializer<Function>::Write(*(func.get()), dest);
    return dest;
  }
};

template<>
class SimpleSerializer< linked_ptr<BasicSourceLineResolver::PublicSymbol> > {
  typedef BasicSourceLineResolver::PublicSymbol PublicSymbol;
 public:
  static size_t SizeOf(const linked_ptr<PublicSymbol>& pubsymbol) {
    if (pubsymbol.get() == nullptr) return 0;
    return SimpleSerializer<PublicSymbol>::SizeOf(*(pubsymbol.get()));
  }
  static char* Write(const linked_ptr<PublicSymbol>& pubsymbol, char* dest) {
    if (pubsymbol.get())
      dest = SimpleSerializer<PublicSymbol>::Write(*(pubsymbol.get()), dest);
    return dest;
  }
};

template<>
class SimpleSerializer< linked_ptr<WindowsFrameInfo> > {
 public:
  static size_t SizeOf(const linked_ptr<WindowsFrameInfo>& wfi) {
    if (wfi.get() == nullptr) return 0;
    return SimpleSerializer<WindowsFrameInfo>::SizeOf(*(wfi.get()));
  }
  static char* Write(const linked_ptr<WindowsFrameInfo>& wfi, char* dest) {
    if (wfi.get())
      dest = SimpleSerializer<WindowsFrameInfo>::Write(*(wfi.get()), dest);
    return dest;
  }
};

}  // namespace google_breakpad

#endif  // PROCESSOR_SIMPLE_SERIALIZER_INL_H__
