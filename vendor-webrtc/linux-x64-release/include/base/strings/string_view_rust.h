// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRING_VIEW_RUST_H_
#define BASE_STRINGS_STRING_VIEW_RUST_H_

#include <stdint.h>

#include <string_view>

#include "build/build_config.h"
#include "third_party/rust/cxx/v1/cxx.h"

#if BUILDFLAG(IS_NACL)
#error "string_view_rust.h included under IS_NACL"
#endif

namespace base {

// Create a Rust str from a std::string_view. This will call std::abort
// if there is any invalid UTF8. If you're concerned about this, then
// instead use StringViewToRustSlice and convert the data to a string on
// the Rust side (or pass in a std::string).
inline rust::Str StringViewToRustStrUTF8(std::string_view string_piece) {
  return rust::Str(string_piece.data(), string_piece.size());
}

// Create a Rust slice from a std::string_view. No UTF8 check is performed.
inline rust::Slice<const uint8_t> StringViewToRustSlice(
    std::string_view string_piece) {
  return rust::Slice<const uint8_t>(
      reinterpret_cast<const uint8_t*>(string_piece.data()),
      string_piece.length() * sizeof(std::string_view::value_type));
}

// Create a std::string_view from a Rust str.
inline std::string_view RustStrToStringView(rust::Str str) {
  return std::string_view(str.data(), str.size());
}

}  // namespace base

#endif  // BASE_STRINGS_STRING_VIEW_RUST_H_
