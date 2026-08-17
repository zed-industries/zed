// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_REQUEST_CALLBACK_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_REQUEST_CALLBACK_COLLECTION_H_

#include <memory>

#include "base/check_op.h"
#include "base/rand_util.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExecutionContext;
class V8XRFrameRequestCallback;
class XRFrame;
class XRSession;

namespace probe {
class AsyncTaskContext;
}

class XRFrameRequestCallbackCollection final
    : public GarbageCollected<XRFrameRequestCallbackCollection>,
      public NameClient {
 public:
  explicit XRFrameRequestCallbackCollection(ExecutionContext*);
  ~XRFrameRequestCallbackCollection() override = default;

  using CallbackId = int;
  CallbackId RegisterCallback(V8XRFrameRequestCallback*);
  void CancelCallback(CallbackId);
  void ExecuteCallbacks(XRSession*, double timestamp, XRFrame*);

  bool IsEmpty() const {
    DCHECK_EQ(callback_frame_requests_.size(), callback_async_tasks_.size());
    return !callback_frame_requests_.size();
  }

  void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "XRFrameRequestCallbackCollection";
  }

 private:
  bool IsValidCallbackId(int id) {
    using Traits = HashTraits<CallbackId>;
    return !WTF::IsHashTraitsEmptyOrDeletedValue<Traits, CallbackId>(id);
  }

  using CallbackFrameRequestMap =
      HeapHashMap<CallbackId, Member<V8XRFrameRequestCallback>>;
  using CallbackAsyncTaskMap =
      HashMap<CallbackId, std::unique_ptr<probe::AsyncTaskContext>>;

  CallbackFrameRequestMap callback_frame_requests_;
  CallbackAsyncTaskMap callback_async_tasks_;
  Vector<CallbackId> pending_callbacks_;

  // Only non-empty while inside executeCallbacks.
  CallbackFrameRequestMap current_callback_frame_requests_;
  CallbackAsyncTaskMap current_callback_async_tasks_;

  CallbackId next_callback_id_ = 0;

  // Trace IDs need to be unique for any outstanding frames. While we can only
  // have one immersive session at a time, that is not the case for inline
  // sessions. Since this class is created per-session, we cannot simply
  // use `next_callback_id_`. Instead we generate a random number over
  // a sufficiently large space to provide an offset so that outstanding frames
  // should for all pracitcal purposes never overlap.
  const uint64_t trace_id_base_ = base::RandUint64();

  Member<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_FRAME_REQUEST_CALLBACK_COLLECTION_H_
