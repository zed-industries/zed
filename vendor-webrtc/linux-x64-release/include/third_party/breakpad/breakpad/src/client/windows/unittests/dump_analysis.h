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

#ifndef CLIENT_WINDOWS_UNITTESTS_DUMP_ANALYSIS_H_
#define CLIENT_WINDOWS_UNITTESTS_DUMP_ANALYSIS_H_

#include "client/windows/crash_generation/minidump_generator.h"

// Convenience to get to the PEB pointer in a TEB.
struct FakeTEB {
  char dummy[0x30];
  void* peb;
};

class DumpAnalysis {
 public:
  explicit DumpAnalysis(const std::wstring& file_path)
      : dump_file_(file_path),
        dump_file_view_(nullptr),
        dump_file_mapping_(nullptr),
        dump_file_handle_(nullptr) {
    EnsureDumpMapped();
  }
  ~DumpAnalysis();

  bool HasStream(ULONG stream_number) const;

  // This is template to keep type safety in the front, but we end up casting
  // to void** inside the implementation to pass the pointer to Win32. So
  // casting here is considered safe.
  template <class StreamType>
  size_t GetStream(ULONG stream_number, StreamType** stream) const {
    return GetStreamImpl(stream_number, reinterpret_cast<void**>(stream));
  }

  bool HasTebs() const;
  bool HasPeb() const;
  bool HasMemory(ULONG64 address) const {
    return HasMemory<BYTE>(address, nullptr);
  }

  bool HasMemory(const void* address) const {
    return HasMemory<BYTE>(address, nullptr);
  }

  template <class StructureType>
  bool HasMemory(ULONG64 address, StructureType** structure = nullptr) const {
    // We can't cope with 64 bit addresses for now.
    if (address > 0xFFFFFFFFUL)
      return false;

    return HasMemory(reinterpret_cast<void*>(address), structure);
  }

  template <class StructureType>
  bool HasMemory(const void* addr_in,
                 StructureType** structure = nullptr) const {
    return HasMemoryImpl(addr_in, sizeof(StructureType),
                             reinterpret_cast<void**>(structure));
  }

 protected:
  void EnsureDumpMapped();

  HANDLE dump_file_mapping_;
  HANDLE dump_file_handle_;
  void* dump_file_view_;
  std::wstring dump_file_;

 private:
  // This is the implementation of GetStream<>.
  size_t GetStreamImpl(ULONG stream_number, void** stream) const;

  // This is the implementation of HasMemory<>.
  bool HasMemoryImpl(const void* addr_in, size_t pointersize,
                     void** structure) const;
};

#endif  // CLIENT_WINDOWS_UNITTESTS_DUMP_ANALYSIS_H_
