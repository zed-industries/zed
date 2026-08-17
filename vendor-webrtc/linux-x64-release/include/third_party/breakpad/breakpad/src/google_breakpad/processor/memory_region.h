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

// memory_region.h: Access to memory regions.
//
// A MemoryRegion provides virtual access to a range of memory.  It is an
// abstraction allowing the actual source of memory to be independent of
// methods which need to access a virtual memory space.
//
// Author: Mark Mentovai

#ifndef GOOGLE_BREAKPAD_PROCESSOR_MEMORY_REGION_H__
#define GOOGLE_BREAKPAD_PROCESSOR_MEMORY_REGION_H__


#include "google_breakpad/common/breakpad_types.h"


namespace google_breakpad {


class MemoryRegion {
 public:
  virtual ~MemoryRegion() {}

  // The base address of this memory region.
  virtual uint64_t GetBase() const = 0;

  // The size of this memory region.
  virtual uint32_t GetSize() const = 0;

  // Access to data of various sizes within the memory region.  address
  // is a pointer to read, and it must lie within the memory region as
  // defined by its base address and size.  The location pointed to by
  // value is set to the value at address.  Byte-swapping is performed
  // if necessary so that the value is appropriate for the running
  // program.  Returns true on success.  Fails and returns false if address
  // is out of the region's bounds (after considering the width of value),
  // or for other types of errors.
  virtual bool GetMemoryAtAddress(uint64_t address, uint8_t*  value) const = 0;
  virtual bool GetMemoryAtAddress(uint64_t address, uint16_t* value) const = 0;
  virtual bool GetMemoryAtAddress(uint64_t address, uint32_t* value) const = 0;
  virtual bool GetMemoryAtAddress(uint64_t address, uint64_t* value) const = 0;

  // Print a human-readable representation of the object to stdout.
  virtual void Print() const = 0;
};


}  // namespace google_breakpad


#endif  // GOOGLE_BREAKPAD_PROCESSOR_MEMORY_REGION_H__
