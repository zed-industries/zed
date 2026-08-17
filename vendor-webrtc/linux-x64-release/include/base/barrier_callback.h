// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BARRIER_CALLBACK_H_
#define BASE_BARRIER_CALLBACK_H_

#include <concepts>
#include <memory>
#include <type_traits>
#include <utility>
#include <vector>

#include "base/check.h"
#include "base/check_op.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/notreached.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"

namespace base {

namespace internal {

template <typename T, typename DoneArg>
class BarrierCallbackInfo {
 public:
  BarrierCallbackInfo(size_t num_callbacks,
                      OnceCallback<void(DoneArg)> done_callback)
      : num_callbacks_left_(num_callbacks),
        done_callback_(std::move(done_callback)) {
    results_.reserve(num_callbacks);
  }

  void Run(T t) LOCKS_EXCLUDED(mutex_) {
    base::ReleasableAutoLock lock(&mutex_);
    DCHECK_NE(num_callbacks_left_, 0U);
    results_.push_back(std::move(t));
    --num_callbacks_left_;

    if (num_callbacks_left_ == 0) {
      std::vector<std::remove_cvref_t<T>> results = std::move(results_);
      lock.Release();
      std::move(done_callback_).Run(std::move(results));
    }
  }

 private:
  Lock mutex_;
  size_t num_callbacks_left_ GUARDED_BY(mutex_);
  std::vector<std::remove_cvref_t<T>> results_ GUARDED_BY(mutex_);
  OnceCallback<void(DoneArg)> done_callback_;
};

template <typename T>
void ShouldNeverRun(T t) {
  NOTREACHED();
}

}  // namespace internal

// BarrierCallback<T> is an analog of BarrierClosure for which each `Run()`
// invocation takes a `T` as an argument. After `num_callbacks` such
// invocations, BarrierCallback invokes `Run()` on its `done_callback`, passing
// the vector of `T`s as an argument. (The ordering of the vector is
// unspecified.)
//
// `T`s that are movable are moved into the callback's storage; otherwise the T
// is copied. (BarrierCallback does not support `T`s that are neither movable
// nor copyable.) If T is a reference, the reference is removed, and the
// callback moves or copies the underlying value per the previously stated rule.
// Beware of creating dangling references. Types that contain references but are
// not references themselves are not modified for callback storage, e.g.
// `std::pair<int, const Foo&>`. Dangling references will be passed to
// `done_callback` if the referenced `Foo` objects have already been deleted.
//
// If `num_callbacks` is 0, `done_callback` is executed immediately.
//
// `done_callback` may accept a `std::vector<T>`, `const std::vector<T>`, or
// `const std::vector<T>&`.
//
// BarrierCallback is thread-safe - the internals are protected by a
// `base::Lock`. `done_callback` will be run on the thread that calls the final
// Run() on the returned callbacks, or the thread that constructed the
// BarrierCallback (in the case where `num_callbacks` is 0).
//
// BarrierCallback is copyable. Copies share state.
//
// `done_callback` is also cleared on the thread that runs it (by virtue of
// being a OnceCallback).
//
// See also
// https://chromium.googlesource.com/chromium/src/+/HEAD/docs/callback.md
template <typename T,
          typename RawArg = std::remove_cvref_t<T>,
          typename DoneArg = std::vector<RawArg>,
          template <typename>
          class CallbackType>
  requires(std::same_as<std::vector<RawArg>, std::remove_cvref_t<DoneArg>> &&
           IsBaseCallback<CallbackType<void()>>)
RepeatingCallback<void(T)> BarrierCallback(
    size_t num_callbacks,
    CallbackType<void(DoneArg)> done_callback) {
  if (num_callbacks == 0) {
    std::move(done_callback).Run({});
    return BindRepeating(&internal::ShouldNeverRun<T>);
  }

  return BindRepeating(
      &internal::BarrierCallbackInfo<T, DoneArg>::Run,
      std::make_unique<internal::BarrierCallbackInfo<T, DoneArg>>(
          num_callbacks, std::move(done_callback)));
}

}  // namespace base

#endif  // BASE_BARRIER_CALLBACK_H_
