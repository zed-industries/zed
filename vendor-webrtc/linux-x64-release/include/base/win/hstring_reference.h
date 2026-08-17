// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_HSTRING_REFERENCE_H_
#define BASE_WIN_HSTRING_REFERENCE_H_

#include <hstring.h>

#include "base/base_export.h"

namespace base::win {

// HStringReference is an HSTRING representation of a null terminated
// string backed by memory that outlives the HStringReference instance.
//
// If you need an HSTRING class that manages its own memory, you should
// use ScopedHString instead.
//
// Example use:
//
//   HStringReference string(L"abc");
//
class BASE_EXPORT HStringReference {
 public:
  // Creates an HStringReference from `str`, which must be null terminated.
  explicit HStringReference(const wchar_t* str);

  HSTRING Get() const { return hstring_; }

  // HSTRING_HEADER is a structure that contains a pointer to the string
  // passed into the constructor, along with its length.

  // Since HSTRING is a pointer to HSTRING_HEADER, HStringReference
  // cannot be copyable, moveable or assignable, as that would invalidate
  // the HSTRING we're passing out to clients.

  // In the future, we can consider implementing these methods by storing
  // the string passed in the constructor and re-creating the HSTRING and
  // HSTRING_HEADER datastructures. For now, we'll keep things simple and
  // forbid these operations.
  HStringReference(const HStringReference&) = delete;
  HStringReference& operator=(const HStringReference&) = delete;

 private:
  HSTRING hstring_ = nullptr;
  HSTRING_HEADER hstring_header_;
};

}  // namespace base::win

#endif  // BASE_WIN_HSTRING_REFERENCE_H_
