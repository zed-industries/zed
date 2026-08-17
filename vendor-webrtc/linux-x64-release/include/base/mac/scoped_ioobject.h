// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_SCOPED_IOOBJECT_H_
#define BASE_MAC_SCOPED_IOOBJECT_H_

#include <IOKit/IOKitLib.h>

#include "base/apple/scoped_typeref.h"

namespace base::mac {

namespace internal {

template <typename IOT>
struct ScopedIOObjectTraits {
  static IOT InvalidValue() { return IO_OBJECT_NULL; }
  static IOT Retain(IOT iot) {
    IOObjectRetain(iot);
    return iot;
  }
  static void Release(IOT iot) { IOObjectRelease(iot); }
};

}  // namespace internal

// Just like ScopedCFTypeRef but for io_object_t and subclasses.
template <typename IOT>
using ScopedIOObject =
    apple::ScopedTypeRef<IOT, internal::ScopedIOObjectTraits<IOT>>;

}  // namespace base::mac

#endif  // BASE_MAC_SCOPED_IOOBJECT_H_
