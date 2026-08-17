// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_GDI_OBJECT_H_
#define BASE_WIN_SCOPED_GDI_OBJECT_H_

#include "base/base_export.h"
#include "base/scoped_generic.h"
#include "base/types/always_false.h"
#include "base/win/win_handle_types.h"

// Defines `ScopedGDIObject`, an RAII helper for GDI objects. Use like
// ```
//   ScopedGDIObject<HBITMAP> scoped_bitmap(ReturnsHBITMAP(...));
//   DoSomething(scoped_bitmap.get());
//   // At end of scope, scoper auto-calls ::DeleteObject().
// ```
//
// For full API documentation, see the docs for ScopedGeneric.
//
// This is specialized for the following types:
//   HBITMAP
//   HBRUSH
//   HFONT
//   HICON - Calls ::DestroyIcon() instead of ::DeleteObject()
//   HPEN
//   HRGN
// To add more types, add to the DECLARE_TRAIT_SPECIALIZATIONs below and the
// corresponding DEFINE_TRAIT_SPECIALIZATIONs in the .cc file.

namespace base::win {

namespace internal {

template <typename T>
struct BASE_EXPORT ScopedGDIObjectTraits {
  static T InvalidValue() { return nullptr; }
  static void Free(T object) {
    static_assert(base::AlwaysFalse<T>, "Explicitly forward-declare this T");
  }
};

// Forward-declare all used specializations and define them in the .cc file.
// This avoids pulling `<windows.h>` transiently into every file that
// `#include`s this one.
#define DECLARE_TRAIT_SPECIALIZATION(T) \
  template <>                           \
  void ScopedGDIObjectTraits<T>::Free(T object);
DECLARE_TRAIT_SPECIALIZATION(HBITMAP)
DECLARE_TRAIT_SPECIALIZATION(HBRUSH)
DECLARE_TRAIT_SPECIALIZATION(HFONT)
DECLARE_TRAIT_SPECIALIZATION(HICON)
DECLARE_TRAIT_SPECIALIZATION(HPEN)
DECLARE_TRAIT_SPECIALIZATION(HRGN)
#undef DECLARE_TRAIT_SPECIALIZATION

}  // namespace internal

template <class T>
using ScopedGDIObject = ScopedGeneric<T, internal::ScopedGDIObjectTraits<T>>;

}  // namespace base::win

#endif  // BASE_WIN_SCOPED_GDI_OBJECT_H_
