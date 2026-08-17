// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_SCOPED_IONOTIFICATIONPORTREF_H_
#define BASE_MAC_SCOPED_IONOTIFICATIONPORTREF_H_

#include <IOKit/IOKitLib.h>

#include "base/scoped_generic.h"

namespace base::mac {

namespace internal {

struct ScopedIONotificationPortRefTraits {
  static IONotificationPortRef InvalidValue() { return nullptr; }
  static void Free(IONotificationPortRef object) {
    IONotificationPortDestroy(object);
  }
};

}  // namespace internal

using ScopedIONotificationPortRef =
    ScopedGeneric<IONotificationPortRef,
                  internal::ScopedIONotificationPortRefTraits>;

}  // namespace base::mac

#endif  // BASE_MAC_SCOPED_IONOTIFICATIONPORTREF_H_
