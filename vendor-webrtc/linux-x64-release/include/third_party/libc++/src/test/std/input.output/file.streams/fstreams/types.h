//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_TYPES_H
#define TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_TYPES_H

#include <cstddef>
#include <fstream>
#include <vector>

#include "test_macros.h"

struct UserManagedBuffer {
  UserManagedBuffer(std::size_t size) : size_(size) {}

  UserManagedBuffer(UserManagedBuffer const&) = delete;
  UserManagedBuffer(UserManagedBuffer&&)      = default;

  void operator()(std::basic_ofstream<char>& stream) {
    buffers_.emplace_back(new char[size_]);
    stream.rdbuf()->pubsetbuf(buffers_.back(), size_);
  }
  void operator()(std::basic_ifstream<char>& stream) {
    buffers_.emplace_back(new char[size_]);
    stream.rdbuf()->pubsetbuf(buffers_.back(), size_);
  }

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
  void operator()(std::basic_ofstream<wchar_t>& stream) {
    wbuffers_.emplace_back(new wchar_t[size_]);
    stream.rdbuf()->pubsetbuf(wbuffers_.back(), size_);
  }
  void operator()(std::basic_ifstream<wchar_t>& stream) {
    wbuffers_.emplace_back(new wchar_t[size_]);
    stream.rdbuf()->pubsetbuf(wbuffers_.back(), size_);
  }
#endif
  ~UserManagedBuffer() {
    for (char* p : buffers_)
      delete[] p;

#ifndef TEST_HAS_NO_WIDE_CHARACTERS
    for (wchar_t* p : wbuffers_)
      delete[] p;
#endif
  }

private:
  std::size_t size_;
  std::vector<char*> buffers_;
  std::vector<wchar_t*> wbuffers_;
};

struct LibraryManagedBuffer {
  LibraryManagedBuffer(std::size_t size) : size_(size) {}
  template <class CharT>
  void operator()(std::basic_ofstream<CharT>& stream) const {
    stream.rdbuf()->pubsetbuf(nullptr, size_);
  }

  template <class CharT>
  void operator()(std::basic_ifstream<CharT>& stream) const {
    stream.rdbuf()->pubsetbuf(nullptr, size_);
  }

private:
  std::size_t size_;
};

struct LibraryDefaultBuffer {
  LibraryDefaultBuffer() {}
  template <class CharT>
  void operator()(std::basic_ofstream<CharT>&) const {}
  template <class CharT>
  void operator()(std::basic_ifstream<CharT>&) const {}
};

#if TEST_STD_VER >= 26
#  if defined(_WIN32)
using NativeHandleT = void*; // HANDLE
#  elif __has_include(<unistd.h>)
using NativeHandleT = int; // POSIX file descriptor
#  else
#    error "Provide a native file handle!"
#  endif
#endif

#endif // TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_TYPES_H
