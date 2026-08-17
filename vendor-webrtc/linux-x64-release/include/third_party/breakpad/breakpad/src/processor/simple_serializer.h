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
// simple_serializer.h: SimpleSerializer is a template for calculating size and
// writing to specific memory location for objects of primitive types, C-style
// string, string, breakpad types/structs etc.
// All specializations of SimpleSerializer template are defined in the
// "simple_serializer-inl.h" file.
//
// Author: Siyang Xie (lambxsy@google.com)

#ifndef PROCESSOR_SIMPLE_SERIALIZER_H__
#define PROCESSOR_SIMPLE_SERIALIZER_H__

#include <stddef.h>

#include "google_breakpad/common/breakpad_types.h"

namespace google_breakpad {

typedef uint64_t MemAddr;

// Default implementation of SimpleSerializer template.
// Specializations are defined in "simple_serializer-inl.h".
template<class Type> class SimpleSerializer {
 public:
  // Calculate and return the size of the 'item'.
  static size_t SizeOf(const Type& item) { return sizeof(item); }
  // Write 'item' to memory location 'dest', and return to the "end" address of
  // data written, i.e., the address after the final byte written.
  static char* Write(const Type& item, char* dest) {
    new (dest) Type(item);
    return dest + SizeOf(item);
  }
};

}  // namespace google_breakpad

#endif  // PROCESSOR_SIMPLE_SERIALIZER_H__
