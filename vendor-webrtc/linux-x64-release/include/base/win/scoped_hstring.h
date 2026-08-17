// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_HSTRING_H_
#define BASE_WIN_SCOPED_HSTRING_H_

#include <hstring.h>

#include <string>
#include <string_view>

#include "base/scoped_generic.h"

namespace base {

namespace internal {

// Scoped HSTRING class to maintain lifetime of HSTRINGs allocated with
// WindowsCreateString().
struct BASE_EXPORT ScopedHStringTraits {
  static HSTRING InvalidValue() { return nullptr; }
  static void Free(HSTRING hstr);
};

}  // namespace internal

namespace win {

// ScopedHString is a wrapper around an HSTRING.
//
// Example use:
//
//   ScopedHString string = ScopedHString::Create(L"abc");
//
// Also:
//
//   HSTRING win_string;
//   HRESULT hr = WindowsCreateString(..., &win_string);
//   ScopedHString string(win_string);
//
class BASE_EXPORT ScopedHString
    : public ScopedGeneric<HSTRING, base::internal::ScopedHStringTraits> {
 public:
  // Constructs a ScopedHString from an HSTRING, and takes ownership of |hstr|.
  explicit ScopedHString(HSTRING hstr);

  static ScopedHString Create(std::wstring_view str);
  static ScopedHString Create(std::string_view str);

  // Returns a view into the memory buffer managed by the instance. The returned
  // std::string_view is only valid during the lifetime of this ScopedHString
  // instance.
  std::wstring_view Get() const;

  // Returns a copy of the instance as a UTF-8 string.
  std::string GetAsUTF8() const;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_HSTRING_H_
