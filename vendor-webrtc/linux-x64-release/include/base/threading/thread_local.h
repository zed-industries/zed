// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WARNING: Thread local storage is a bit tricky to get right. Please make sure
// that this is really the proper solution for what you're trying to achieve.
// Don't prematurely optimize, most likely you can just use a Lock.

#ifndef BASE_THREADING_THREAD_LOCAL_H_
#define BASE_THREADING_THREAD_LOCAL_H_

#include <memory>

#include "base/dcheck_is_on.h"
#include "base/memory/ptr_util.h"
#include "base/threading/thread_local_internal.h"
#include "base/threading/thread_local_storage.h"

namespace base {

// `thread_local` is only allowed for trivially-destructible types (see
// //styleguide/c++/c++.md#thread_local-variables). This class provides
// thread-scoped management of non-trivially-destructible types. Pointers handed
// to it are owned and automatically deleted during their associated thread's
// exit phase (or when replaced if Set() is invoked multiple times on the same
// thread).
//
// The ThreadLocalOwnedPointer instance itself can only be destroyed when no
// threads, other than the one it is destroyed on, have remaining state set in
// it. Typically this means that ThreadLocalOwnedPointer instances are held in
// static storage or at the very least only recycled in the single-threaded
// phase between tests in the same process.
#if DCHECK_IS_ON()
template <typename T>
using ThreadLocalOwnedPointer = internal::CheckedThreadLocalOwnedPointer<T>;
#else   // DCHECK_IS_ON()
template <typename T>
class ThreadLocalOwnedPointer {
 public:
  ThreadLocalOwnedPointer() = default;

  ThreadLocalOwnedPointer(const ThreadLocalOwnedPointer&) = delete;
  ThreadLocalOwnedPointer& operator=(const ThreadLocalOwnedPointer&) = delete;

  ~ThreadLocalOwnedPointer() {
    // Assume that this thread is the only one with potential state left. This
    // is verified in ~CheckedThreadLocalOwnedPointer().
    Set(nullptr);
  }

  T* Get() const { return static_cast<T*>(slot_.Get()); }

  // Sets a new value, returns the old.
  std::unique_ptr<T> Set(std::unique_ptr<T> ptr) {
    auto existing = WrapUnique(Get());
    slot_.Set(const_cast<void*>(static_cast<const void*>(ptr.release())));
    return existing;
  }

  T& operator*() { return *Get(); }

 private:
  static void DeleteTlsPtr(void* ptr) { delete static_cast<T*>(ptr); }

  ThreadLocalStorage::Slot slot_{&DeleteTlsPtr};
};
#endif  // DCHECK_IS_ON()

}  // namespace base

#endif  // BASE_THREADING_THREAD_LOCAL_H_
