// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_SCOPED_CFTYPEREF_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_SCOPED_CFTYPEREF_H_

#include <CoreFoundation/CoreFoundation.h>

#include "partition_alloc/partition_alloc_base/apple/scoped_typeref.h"

namespace partition_alloc::internal::base::apple {

// ScopedCFTypeRef<> is patterned after std::unique_ptr<>, but maintains
// ownership of a CoreFoundation object: any object that can be represented
// as a CFTypeRef.  Style deviations here are solely for compatibility with
// std::unique_ptr<>'s interface, with which everyone is already familiar.
//
// By default, ScopedCFTypeRef<> takes ownership of an object (in the
// constructor or in reset()) by taking over the caller's existing ownership
// claim.  The caller must own the object it gives to ScopedCFTypeRef<>, and
// relinquishes an ownership claim to that object.  ScopedCFTypeRef<> does not
// call CFRetain(). This behavior is parameterized by the |OwnershipPolicy|
// enum. If the value |RETAIN| is passed (in the constructor or in reset()),
// then ScopedCFTypeRef<> will call CFRetain() on the object, and the initial
// ownership is not changed.

namespace internal {

template <typename CFT>
struct ScopedCFTypeRefTraits {
  static CFT InvalidValue() { return nullptr; }
  static CFT Retain(CFT object) {
    CFRetain(object);
    return object;
  }
  static void Release(CFT object) { CFRelease(object); }
};

}  // namespace internal

template <typename CFT>
using ScopedCFTypeRef =
    ScopedTypeRef<CFT, internal::ScopedCFTypeRefTraits<CFT>>;

}  // namespace partition_alloc::internal::base::apple

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_SCOPED_CFTYPEREF_H_
