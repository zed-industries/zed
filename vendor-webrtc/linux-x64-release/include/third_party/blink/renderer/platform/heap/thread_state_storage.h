// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_STATE_STORAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_STATE_STORAGE_H_

#include <cstdint>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/heap/thread_local.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/stack_util.h"

namespace cppgc {
class AllocationHandle;
class HeapHandle;
}  // namespace cppgc

namespace blink {

class ThreadState;
class ThreadStateStorage;

// ThreadAffinity indicates which threads objects can be used on. We
// distinguish between objects that can be used on the main thread
// only and objects that can be used on any thread.
//
// For objects that can only be used on the main thread, we avoid going
// through thread-local storage to get to the thread state. This is
// important for performance.
enum ThreadAffinity {
  kAnyThread,
  kMainThreadOnly,
};

template <typename T, typename = void>
struct ThreadingTrait {
  STATIC_ONLY(ThreadingTrait);
  static constexpr ThreadAffinity kAffinity = kAnyThread;
};

// Storage for all ThreadState objects. This includes the main-thread
// ThreadState as well. Keep it outside the class so that PLATFORM_EXPORT
// doesn't apply to it (otherwise, clang-cl complains).
extern constinit thread_local ThreadStateStorage* g_thread_specific_
    __attribute__((tls_model(BLINK_HEAP_THREAD_LOCAL_MODEL)));

// ThreadStateStorage is the explicitly managed TLS- and global-backed storage
// for ThreadState.
class PLATFORM_EXPORT ThreadStateStorage final {
 public:
  ALWAYS_INLINE static ThreadStateStorage* MainThreadStateStorage() {
    return &main_thread_state_storage_;
  }

  BLINK_HEAP_DECLARE_THREAD_LOCAL_GETTER(Current,
                                         ThreadStateStorage*,
                                         g_thread_specific_)

  ALWAYS_INLINE cppgc::AllocationHandle& allocation_handle() const {
    return *allocation_handle_;
  }

  ALWAYS_INLINE cppgc::HeapHandle& heap_handle() const { return *heap_handle_; }

  ALWAYS_INLINE ThreadState& thread_state() const { return *thread_state_; }

  ALWAYS_INLINE bool IsMainThread() const {
    return this == MainThreadStateStorage();
  }

 private:
  static void AttachMainThread(ThreadState&,
                               cppgc::AllocationHandle&,
                               cppgc::HeapHandle&);
  static void AttachNonMainThread(ThreadState&,
                                  cppgc::AllocationHandle&,
                                  cppgc::HeapHandle&);
  static void DetachNonMainThread(ThreadStateStorage&);

  static ThreadStateStorage main_thread_state_storage_;

  ThreadStateStorage() = default;
  ThreadStateStorage(ThreadState&,
                     cppgc::AllocationHandle&,
                     cppgc::HeapHandle&);

  cppgc::AllocationHandle* allocation_handle_ = nullptr;
  cppgc::HeapHandle* heap_handle_ = nullptr;
  ThreadState* thread_state_ = nullptr;

  friend class ThreadState;
};

template <ThreadAffinity>
class ThreadStateStorageFor;

template <>
class ThreadStateStorageFor<kMainThreadOnly> {
  STATIC_ONLY(ThreadStateStorageFor);

 public:
  ALWAYS_INLINE static ThreadStateStorage* GetState() {
    auto* main_thread_storage = ThreadStateStorage::MainThreadStateStorage();
    DCHECK_EQ(main_thread_storage, ThreadStateStorage::Current());
    return main_thread_storage;
  }
};

template <>
class ThreadStateStorageFor<kAnyThread> {
  STATIC_ONLY(ThreadStateStorageFor);

 public:
  static ThreadStateStorage* GetState() {
#if BUILDFLAG(IS_MAC) || BUILDFLAG(IS_ANDROID)
    // Perform a fast on main thread check on platforms with expensive TLS.
    if (!WTF::MayNotBeMainThread()) {
      return ThreadStateStorage::MainThreadStateStorage();
    }
#endif  // BUILDFLAG(IS_MAC)
    return ThreadStateStorage::Current();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_STATE_STORAGE_H_
