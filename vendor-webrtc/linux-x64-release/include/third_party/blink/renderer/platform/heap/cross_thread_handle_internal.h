// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_HANDLE_INTERNAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_HANDLE_INTERNAL_H_

#include "base/functional/bind.h"
#include "base/threading/platform_thread.h"
#include "third_party/blink/renderer/platform/heap/heap_buildflags.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "v8/include/cppgc/cross-thread-persistent.h"
#include "v8/include/cppgc/source-location.h"

// Required to optimize away locations for builds that do not need them to avoid
// binary size blowup.
// Same as in `platform/heap/persistent.h`. Avoiding the include to avoid
// exposing other types that are harder to use in concurrent contexts.
#if BUILDFLAG(VERBOSE_PERSISTENT)
#define CROSS_THREAD_HANDLE_LOCATION_FROM_HERE \
  blink::CrossThreadHandleLocation::Current()
#else  // !BUILDFLAG(VERBOSE_PERSISTENT)
#define CROSS_THREAD_HANDLE_LOCATION_FROM_HERE \
  blink::CrossThreadHandleLocation()
#endif  // !BUILDFLAG(VERBOSE_PERSISTENT)

namespace blink {

template <typename T, typename WeaknessPolicy>
class BasicUnwrappingCrossThreadHandle;

// A source location that is used for tracking creation of
// `CrossThreadHandle` and `CrossThreadWeakHandle` in debugging
// configurations.
using CrossThreadHandleLocation = cppgc::SourceLocation;

namespace internal {

struct StrongCrossThreadHandleWeaknessPolicy {
  template <typename T>
  using InternalRefType = cppgc::subtle::CrossThreadPersistent<T>;
};

struct WeakCrossThreadHandleWeaknessPolicy {
  template <typename T>
  using InternalRefType = cppgc::subtle::WeakCrossThreadPersistent<T>;
};

template <typename T, typename WeaknessPolicy>
class BasicCrossThreadHandle {
 public:
  explicit BasicCrossThreadHandle(T* raw,
                                  const CrossThreadHandleLocation& loc =
                                      CROSS_THREAD_HANDLE_LOCATION_FROM_HERE)
      : ref_(raw, loc) {}

  ~BasicCrossThreadHandle() = default;
  BasicCrossThreadHandle(BasicCrossThreadHandle&&) = default;
  // Required by CrossThreadCopier.
  BasicCrossThreadHandle(const BasicCrossThreadHandle&) = default;

  BasicCrossThreadHandle& operator=(BasicCrossThreadHandle&&) = delete;
  BasicCrossThreadHandle* operator=(const BasicCrossThreadHandle&) = delete;

 protected:
  // Returns a pointer to the garbage collected object. Must only be used from
  // the creation thread and will crash if invoked on any other thread.
  T* GetOnCreationThread() const {
    CHECK_EQ(creation_thread_id_, base::PlatformThread::CurrentId());
    return ref_.Get();
  }

  // Clears the stored object.
  void Clear() { ref_.Clear(); }

 private:
  template <typename U, typename V>
  friend class blink::BasicUnwrappingCrossThreadHandle;

  typename WeaknessPolicy::template InternalRefType<T> ref_;
  const base::PlatformThreadId creation_thread_id_ =
      base::PlatformThread::CurrentId();
};

template <typename T, typename WeaknessPolicy>
class BasicUnwrappingCrossThreadHandle final
    : public BasicCrossThreadHandle<T, WeaknessPolicy> {
  using Base = BasicCrossThreadHandle<T, WeaknessPolicy>;

 public:
  explicit BasicUnwrappingCrossThreadHandle(
      BasicCrossThreadHandle<T, WeaknessPolicy>&& handle)
      : Base(std::move(handle)) {}
  explicit BasicUnwrappingCrossThreadHandle(
      const BasicCrossThreadHandle<T, WeaknessPolicy>& handle)
      : Base(handle) {}
  explicit BasicUnwrappingCrossThreadHandle(
      T* raw,
      const CrossThreadHandleLocation& loc =
          CROSS_THREAD_HANDLE_LOCATION_FROM_HERE)
      : Base(raw, loc) {}

  ~BasicUnwrappingCrossThreadHandle() = default;
  BasicUnwrappingCrossThreadHandle(BasicUnwrappingCrossThreadHandle&&) =
      default;
  // Required by CrossThreadCopier.
  BasicUnwrappingCrossThreadHandle(const BasicUnwrappingCrossThreadHandle&) =
      default;

  // Re-expose the actual getter for the underlying object.
  using Base::GetOnCreationThread;

  // Re-expose the clear method for the underlying object.
  using Base::Clear;

  // Returns whether a value is set.  May only be accessed on the thread the
  // original CrossThreadHandle object was created.
  //
  // Exposed so the callback implementation can test if weak handles are still
  // live before trying to run the callback.
  explicit operator bool() const { return Base::GetOnCreationThread(); }

  BasicUnwrappingCrossThreadHandle& operator=(
      BasicUnwrappingCrossThreadHandle&&) = delete;
  BasicUnwrappingCrossThreadHandle* operator=(
      const BasicUnwrappingCrossThreadHandle&) = delete;
};

}  // namespace internal
}  // namespace blink

namespace WTF {

template <typename T, typename WeaknessPolicy>
struct CrossThreadCopier<
    blink::internal::BasicCrossThreadHandle<T, WeaknessPolicy>>
    : public CrossThreadCopierPassThrough<
          blink::internal::BasicCrossThreadHandle<T, WeaknessPolicy>> {};

template <typename T, typename WeaknessPolicy>
struct CrossThreadCopier<
    blink::internal::BasicUnwrappingCrossThreadHandle<T, WeaknessPolicy>>
    : public CrossThreadCopierPassThrough<
          blink::internal::BasicUnwrappingCrossThreadHandle<T,
                                                            WeaknessPolicy>> {};

}  // namespace WTF

namespace base {

template <typename T>
struct IsWeakReceiver<blink::internal::BasicUnwrappingCrossThreadHandle<
    T,
    blink::internal::WeakCrossThreadHandleWeaknessPolicy>> : std::true_type {};

template <typename T, typename WeaknessPolicy>
struct BindUnwrapTraits<
    blink::internal::BasicUnwrappingCrossThreadHandle<T, WeaknessPolicy>> {
  static T* Unwrap(
      const blink::internal::BasicUnwrappingCrossThreadHandle<T,
                                                              WeaknessPolicy>&
          wrapped) {
    return wrapped.GetOnCreationThread();
  }
};

template <typename T, typename WeaknessPolicy>
struct MaybeValidTraits<
    blink::internal::BasicCrossThreadHandle<T, WeaknessPolicy>> {
  static bool MaybeValid(
      const blink::internal::BasicCrossThreadHandle<T, WeaknessPolicy>& p) {
    return true;
  }
};

template <typename T, typename WeaknessPolicy>
struct MaybeValidTraits<
    blink::internal::BasicUnwrappingCrossThreadHandle<T, WeaknessPolicy>> {
  static bool MaybeValid(
      const blink::internal::BasicUnwrappingCrossThreadHandle<T,
                                                              WeaknessPolicy>&
          p) {
    return true;
  }
};

}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_HANDLE_INTERNAL_H_
