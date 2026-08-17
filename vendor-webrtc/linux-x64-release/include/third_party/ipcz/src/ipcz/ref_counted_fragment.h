// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_REF_COUNTED_FRAGMENT_H_
#define IPCZ_SRC_IPCZ_REF_COUNTED_FRAGMENT_H_

#include <atomic>

#include "ipcz/ipcz.h"
#include "util/ref_counted.h"

namespace ipcz {

// A RefCountedFragment is an object allocated within a shared Fragment from
// NodeLinkMemory, and which is automatially freed when its last reference is
// released. Consumers can hold onto references to RefCountedFragment objects
// by holding a FragmentRef.
struct IPCZ_ALIGN(4) RefCountedFragment {
  enum { kUnmanagedRef };

  RefCountedFragment();

  int32_t ref_count_for_testing() const { return ref_count_; }

  // Increments the reference count for this object.
  void AddRef();

  // Releases a reference and returns the previous reference count. If this
  // returns 1, the underlying Fragment can be safely freed.
  int32_t ReleaseRef();

 private:
  std::atomic<int32_t> ref_count_{1};
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_REF_COUNTED_FRAGMENT_H_
