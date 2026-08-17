//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_TEST_HELPERS_H
#define TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_TEST_HELPERS_H

#include <cassert>
#include <cerrno>
#include <concepts>
#include <cstdio>
#include <fstream>
#include <filesystem>
#include <type_traits>
#include <utility>

#if defined(_WIN32)
#  include <io.h>
#  include <windows.h>
#else
#  include <fcntl.h>
#endif

#include "platform_support.h"
#include "types.h"

#if TEST_STD_VER >= 26

inline bool is_handle_valid(NativeHandleT handle) {
#  if defined(_WIN32)
  BY_HANDLE_FILE_INFORMATION fileInformation;
  return GetFileInformationByHandle(handle, &fileInformation);
#  elif __has_include(<unistd.h>) // POSIX
  return fcntl(handle, F_GETFL) != -1 || errno != EBADF;
#  else
#    error "Provide a native file handle!"
#  endif
}

template <typename CharT, typename StreamT>
inline void test_native_handle() {
  static_assert(
      std::is_same_v<typename std::basic_filebuf<CharT>::native_handle_type, typename StreamT::native_handle_type>);

  StreamT f;
  std::filesystem::path p = get_temp_file_name();

  // non-const
  {
    f.open(p);
    std::same_as<NativeHandleT> decltype(auto) handle = f.native_handle();
    assert(is_handle_valid(handle));
    assert(f.rdbuf()->native_handle() == handle);
    assert(std::as_const(f).rdbuf()->native_handle() == handle);
    f.close();
    assert(!is_handle_valid(handle));
    static_assert(noexcept(f.native_handle()));
  }
  // const
  {
    f.open(p);
    std::same_as<NativeHandleT> decltype(auto) const_handle = std::as_const(f).native_handle();
    assert(is_handle_valid(const_handle));
    assert(f.rdbuf()->native_handle() == const_handle);
    assert(std::as_const(f).rdbuf()->native_handle() == const_handle);
    f.close();
    assert(!is_handle_valid(const_handle));
    static_assert(noexcept(std::as_const(f).native_handle()));
  }
}

template <typename StreamT>
inline void test_native_handle_type() {
  static_assert(std::is_trivially_copyable_v<typename StreamT::native_handle_type>);
  static_assert(std::semiregular<typename StreamT::native_handle_type>);
  static_assert(std::is_same_v<typename StreamT::native_handle_type, NativeHandleT>);
}

#endif // #if TEST_STD_VER >= 26

#endif // TEST_STD_INPUT_OUTPUT_FILE_STREAMS_FSTREAMS_TEST_HELPERS_H
