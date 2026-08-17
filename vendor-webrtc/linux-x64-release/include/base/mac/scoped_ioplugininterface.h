// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_SCOPED_IOPLUGININTERFACE_H_
#define BASE_MAC_SCOPED_IOPLUGININTERFACE_H_

#include <IOKit/IOKitLib.h>

#include "base/apple/scoped_typeref.h"

namespace base::mac {

namespace internal {

template <typename T>
struct ScopedIOPluginInterfaceTraits {
  static T InvalidValue() { return nullptr; }
  static T Retain(T t) {
    (*t)->AddRef(t);
    return t;
  }
  static void Release(T t) { (*t)->Release(t); }
};

}  // namespace internal

// Just like ScopedCFTypeRef but for IOCFPlugInInterface and friends
// (IOUSBInterfaceStruct and IOUSBDeviceStruct320 in particular).
template <typename T>
using ScopedIOPluginInterface =
    apple::ScopedTypeRef<T**, internal::ScopedIOPluginInterfaceTraits<T**>>;

}  // namespace base::mac

#endif  // BASE_MAC_SCOPED_IOPLUGININTERFACE_H_
