/*
 * Copyright (C) 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2009 Jian Li <jianli@chromium.org>
 * Copyright (C) 2012 Patrick Gansterer <paroga@paroga.com>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_THREAD_SPECIFIC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_THREAD_SPECIFIC_H_

#include "base/threading/thread_local_storage.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partition_allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partitions.h"
#include "third_party/blink/renderer/platform/wtf/stack_util.h"
#include "third_party/blink/renderer/platform/wtf/wtf.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

template <typename T>
class ThreadSpecific {
  USING_FAST_MALLOC(ThreadSpecific);

 public:
  ThreadSpecific() : slot_(&Destroy) {}
  ThreadSpecific(const ThreadSpecific&) = delete;
  ThreadSpecific& operator=(const ThreadSpecific&) = delete;
  bool
  IsSet();  // Useful as a fast check to see if this thread has set this value.
  T* operator->();
  operator T*();
  T& operator*();

 private:
  // Not implemented. It's technically possible to destroy a thread specific
  // key, but one would need to make sure that all values have been destroyed
  // already (usually, that all threads that used it have exited). It's
  // unlikely that any user of this call will be in that situation - and having
  // a destructor defined can be confusing, given that it has such strong
  // pre-requisites to work correctly.
  ~ThreadSpecific() = delete;

  T* Get() { return static_cast<T*>(slot_.Get()); }

  void Set(T* ptr) {
    DCHECK(!Get());
    slot_.Set(ptr);
  }

  void static Destroy(void* ptr);

  // This member must only be accessed or modified on the main thread.
  T* main_thread_storage_ = nullptr;
  base::ThreadLocalStorage::Slot slot_;
};

template <typename T>
inline void ThreadSpecific<T>::Destroy(void* ptr) {
  // Never call destructors on the main thread. This is fine because Blink no
  // longer has a graceful shutdown sequence. Be careful to call this function
  // (which can be re-entrant) while the pointer is still set, to avoid lazily
  // allocating Threading after it is destroyed.
  if (IsMainThread())
    return;

  // The memory was allocated via Partitions::FastZeroedMalloc, and then the
  // object was placement-newed. To destroy, we must call the delete expression,
  // and then free the memory manually.
  T* instance = static_cast<T*>(ptr);
  instance->~T();
  Partitions::FastFree(ptr);
}

template <typename T>
inline bool ThreadSpecific<T>::IsSet() {
  return !!Get();
}

template <typename T>
inline ThreadSpecific<T>::operator T*() {
  T* off_thread_ptr;
#if defined(__GLIBC__) || BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_FREEBSD)
  // TLS is fast on these platforms.
  // TODO(csharrison): Qualify this statement for Android.
  const bool kMainThreadAlwaysChecksTLS = true;
  T** ptr = &off_thread_ptr;
  off_thread_ptr = static_cast<T*>(Get());
#else
  const bool kMainThreadAlwaysChecksTLS = false;
  T** ptr = &main_thread_storage_;
  if (MayNotBeMainThread()) [[unlikely]] {
    off_thread_ptr = static_cast<T*>(Get());
    ptr = &off_thread_ptr;
  }
#endif
  // Set up thread-specific value's memory pointer before invoking constructor,
  // in case any function it calls needs to access the value, to avoid
  // recursion.
  if (!*ptr) [[unlikely]] {
    *ptr = static_cast<T*>(Partitions::FastZeroedMalloc(
        sizeof(T), WTF_HEAP_PROFILER_TYPE_NAME(T)));

    // Even if we didn't realize we're on the main thread, we might still be.
    // We need to double-check so that |main_thread_storage_| is populated.
    if (!kMainThreadAlwaysChecksTLS && ptr != &main_thread_storage_ &&
        IsMainThread()) [[unlikely]] {
      main_thread_storage_ = *ptr;
    }

    Set(*ptr);
    ::new (NotNullTag::kNotNull, *ptr) T;
  }
  return *ptr;
}

template <typename T>
inline T* ThreadSpecific<T>::operator->() {
  return operator T*();
}

template <typename T>
inline T& ThreadSpecific<T>::operator*() {
  return *operator T*();
}

}  // namespace WTF

using WTF::ThreadSpecific;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_THREAD_SPECIFIC_H_
