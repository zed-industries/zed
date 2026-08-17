// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FRAME_REQUEST_CALLBACK_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FRAME_REQUEST_CALLBACK_COLLECTION_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_frame_request_callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExecutionContext;

// |FrameCallback| is an interface type which generalizes callbacks which are
// invoked when a script-based animation needs to be resampled.
class CORE_EXPORT FrameCallback : public GarbageCollected<FrameCallback>,
                                  public NameClient {
 public:
  virtual void Trace(Visitor* visitor) const {}
  const char* NameInHeapSnapshot() const override { return "FrameCallback"; }
  ~FrameCallback() override = default;
  virtual void Invoke(double) = 0;

  int Id() const { return id_; }
  bool IsCancelled() const { return is_cancelled_; }
  bool GetUseLegacyTimeBase() const { return use_legacy_time_base_; }
  void SetId(int id) { id_ = id; }
  void SetIsCancelled(bool is_cancelled) { is_cancelled_ = is_cancelled; }
  void SetUseLegacyTimeBase(bool use_legacy_time_base) {
    use_legacy_time_base_ = use_legacy_time_base;
  }

  probe::AsyncTaskContext* async_task_context() { return &async_task_context_; }

 protected:
  FrameCallback() = default;

 private:
  int id_ = 0;
  bool is_cancelled_ = false;
  bool use_legacy_time_base_ = false;
  probe::AsyncTaskContext async_task_context_;
};

// |V8FrameCallback| is an adapter class for the conversion from
// |V8FrameRequestCallback| to |Framecallback|.
class CORE_EXPORT V8FrameCallback : public FrameCallback {
 public:
  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override { return "V8FrameCallback"; }

  explicit V8FrameCallback(V8FrameRequestCallback*);
  ~V8FrameCallback() override = default;

  void Invoke(double) override;

 private:
  Member<V8FrameRequestCallback> callback_;
};

class CORE_EXPORT FrameRequestCallbackCollection final : public NameClient {
  DISALLOW_NEW();

 public:
  explicit FrameRequestCallbackCollection(ExecutionContext*);

  using CallbackId = int;

  CallbackId RegisterFrameCallback(FrameCallback*);
  void CancelFrameCallback(CallbackId);
  void ExecuteFrameCallbacks(double high_res_now_ms,
                             double high_res_now_ms_legacy);

  bool HasFrameCallback() const { return frame_callbacks_.size(); }
  bool IsEmpty() const { return !HasFrameCallback(); }

  void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "FrameRequestCallbackCollection";
  }

 private:
  using CallbackList = HeapVector<Member<FrameCallback>>;

  CallbackList frame_callbacks_;
  // only non-empty while inside ExecuteCallbacks.
  CallbackList callbacks_to_invoke_;

  CallbackId next_callback_id_ = 0;

  Member<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FRAME_REQUEST_CALLBACK_COLLECTION_H_
