// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_BARRIER_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_BARRIER_CALLBACK_H_

#include <concepts>
#include <memory>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/check_op.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/notreached.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

namespace internal {

template <typename T, typename DoneArg>
class BarrierCallbackInfo {
 public:
  BarrierCallbackInfo(wtf_size_t num_callbacks,
                      base::OnceCallback<void(DoneArg)> done_callback)
      : num_callbacks_left_(num_callbacks),
        results_(MakeGarbageCollected<GCedHeapVector<Member<T>>>()),
        done_callback_(std::move(done_callback)) {
    results_->reserve(num_callbacks);
  }

  void Run(T* t) {
    DCHECK_NE(num_callbacks_left_, 0U);
    results_->push_back(std::move(t));
    --num_callbacks_left_;

    if (num_callbacks_left_ == 0) {
      std::move(done_callback_).Run(*results_.Get());
    }
  }

 private:
  wtf_size_t num_callbacks_left_;
  Persistent<GCedHeapVector<Member<T>>> results_;
  base::OnceCallback<void(DoneArg)> done_callback_;
};

template <typename T>
void ShouldNeverRun(T t) {
  NOTREACHED();
}

}  // namespace internal

// This is a near-copy of base/barrier_callback.h, except that it is stores
// the result in a GCedHeapVector so that the results can be GarbageCollected
// objects.
template <typename T,
          typename RawArg = std::remove_cvref_t<T>,
          typename DoneArg = GCedHeapVector<Member<RawArg>>,
          template <typename> class CallbackType>
  requires(std::same_as<GCedHeapVector<Member<RawArg>>,
                        std::remove_cvref_t<DoneArg>>)
base::RepeatingCallback<void(T*)> HeapBarrierCallback(
    wtf_size_t num_callbacks,
    CallbackType<void(DoneArg)> done_callback) {
  if (num_callbacks == 0) {
    std::move(done_callback).Run(GCedHeapVector<Member<RawArg>>());
    return base::BindRepeating(&internal::ShouldNeverRun<T*>);
  }

  return base::BindRepeating(
      &internal::BarrierCallbackInfo<T, DoneArg>::Run,
      std::make_unique<internal::BarrierCallbackInfo<T, DoneArg>>(
          num_callbacks, std::move(done_callback)));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_BARRIER_CALLBACK_H_
