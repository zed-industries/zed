// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_VIDEO_RVFC_VIDEO_FRAME_REQUEST_CALLBACK_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_VIDEO_RVFC_VIDEO_FRAME_REQUEST_CALLBACK_COLLECTION_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_video_frame_callback_metadata.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_frame_request_callback.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExecutionContext;

// Class that allows the registration and unregistration of generic
// VideoFrameCallbacks. Used to store to pending video.requestVideoFrameCallback
// requests, and to propagate the results of the request once it completes.
class MODULES_EXPORT VideoFrameRequestCallbackCollection final
    : public GarbageCollected<VideoFrameRequestCallbackCollection>,
      public NameClient {
 public:
  explicit VideoFrameRequestCallbackCollection(ExecutionContext*);
  ~VideoFrameRequestCallbackCollection() final = default;

  using CallbackId = int;

  // Abstract class that generalizes a video.rVFC callback.
  class MODULES_EXPORT VideoFrameCallback
      : public GarbageCollected<VideoFrameCallback>,
        public NameClient {
   public:
    virtual void Trace(Visitor*) const {}
    const char* NameInHeapSnapshot() const override {
      return "VideoFrameCallback";
    }
    ~VideoFrameCallback() override = default;

    virtual void Invoke(double, const VideoFrameCallbackMetadata*) = 0;

    int Id() const { return id_; }
    bool IsCancelled() const { return is_cancelled_; }
    void SetId(int id) { id_ = id; }
    void SetIsCancelled(bool is_cancelled) { is_cancelled_ = is_cancelled; }

   protected:
    VideoFrameCallback() = default;

   private:
    int id_ = 0;
    bool is_cancelled_ = false;
  };

  // Wrapper class that bridges the script-based V8 callback into a
  // VideoFrameCallback that can be stored and executed by this collection.
  class MODULES_EXPORT V8VideoFrameCallback : public VideoFrameCallback {
   public:
    void Trace(Visitor*) const override;
    const char* NameInHeapSnapshot() const override {
      return "V8VideoFrameCallback";
    }

    explicit V8VideoFrameCallback(V8VideoFrameRequestCallback*);
    ~V8VideoFrameCallback() override = default;

    void Invoke(double, const VideoFrameCallbackMetadata* metadata) override;

   private:
    Member<V8VideoFrameRequestCallback> callback_;
  };

  // Registers a callback and returns its ID.
  // NOTE: Callbacks registered during a call to ExecuteFrameCallbacks won't be
  // run until the next call to ExecuteFrameCallbacks.
  CallbackId RegisterFrameCallback(VideoFrameCallback*);

  // Cancels and removes the callback with the corresponding ID from the
  // collection, or noop if the ID isn't registered.
  void CancelFrameCallback(CallbackId);

  // Invokes all callbacks with the provided information.
  void ExecuteFrameCallbacks(double high_res_now_ms,
                             const VideoFrameCallbackMetadata*);

  bool IsEmpty() const { return !frame_callbacks_.size(); }

  void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "VideoFrameRequestCallbackCollection";
  }

 private:
  using CallbackList = HeapVector<Member<VideoFrameCallback>>;

  CallbackList frame_callbacks_;
  // only non-empty while inside ExecuteCallbacks.
  CallbackList callbacks_to_invoke_;

  CallbackId next_callback_id_ = 0;

  Member<ExecutionContext> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_VIDEO_RVFC_VIDEO_FRAME_REQUEST_CALLBACK_COLLECTION_H_
