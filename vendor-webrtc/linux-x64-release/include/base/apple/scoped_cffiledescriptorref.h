// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_SCOPED_CFFILEDESCRIPTORREF_H_
#define BASE_APPLE_SCOPED_CFFILEDESCRIPTORREF_H_

#include <CoreFoundation/CoreFoundation.h>

#include "base/scoped_generic.h"

namespace base::apple {

namespace internal {

struct ScopedCFFileDescriptorRefTraits {
  static CFFileDescriptorRef InvalidValue() { return nullptr; }
  static void Free(CFFileDescriptorRef ref) {
    CFFileDescriptorInvalidate(ref);
    CFRelease(ref);
  }
};

}  // namespace internal

// ScopedCFFileDescriptorRef is designed after ScopedCFTypeRef<>. On
// destruction, it will invalidate the file descriptor.
// ScopedCFFileDescriptorRef (unlike ScopedCFTypeRef<>) does not support RETAIN
// semantics, copying, or assignment, as doing so would increase the chances
// that a file descriptor is invalidated while still in use.
using ScopedCFFileDescriptorRef =
    ScopedGeneric<CFFileDescriptorRef,
                  internal::ScopedCFFileDescriptorRefTraits>;

}  // namespace base::apple

#endif  // BASE_APPLE_SCOPED_CFFILEDESCRIPTORREF_H_
