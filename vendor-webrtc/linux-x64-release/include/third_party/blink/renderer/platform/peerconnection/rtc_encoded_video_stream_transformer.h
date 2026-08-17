// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_ENCODED_VIDEO_STREAM_TRANSFORMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_ENCODED_VIDEO_STREAM_TRANSFORMER_H_

#include <stdint.h>

#include <memory>
#include <utility>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/metronome/metronome.h"
#include "third_party/webrtc/api/scoped_refptr.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace webrtc {
class FrameTransformerInterface;
class TransformedFrameCallback;
class TransformableVideoFrameInterface;
}  // namespace webrtc

namespace blink {

class PLATFORM_EXPORT RTCEncodedVideoStreamTransformer {
 public:
  using TransformerCallback = WTF::CrossThreadRepeatingFunction<void(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface>)>;

  // A RefCounted wrapper around the Transformer class which holds a
  // cross-thread safe weak pointer to a Transformer object. This allows us to
  // post tasks to the Transformer from classes the transformer owns without
  // creating a circular reference.
  class PLATFORM_EXPORT Broker : public WTF::ThreadSafeRefCounted<Broker> {
   public:
    void RegisterTransformedFrameSinkCallback(
        webrtc::scoped_refptr<webrtc::TransformedFrameCallback>,
        uint32_t ssrc);

    void UnregisterTransformedFrameSinkCallback(uint32_t ssrc);

    void TransformFrameOnSourceTaskRunner(
        std::unique_ptr<webrtc::TransformableVideoFrameInterface> frame);

    void SetTransformerCallback(TransformerCallback callback);

    void ResetTransformerCallback();

    void SetSourceTaskRunner(
        scoped_refptr<base::SingleThreadTaskRunner> task_runner);

    void SendFrameToSink(
        std::unique_ptr<webrtc::TransformableVideoFrameInterface> frame);

    void StartShortCircuiting();

   private:
    explicit Broker(RTCEncodedVideoStreamTransformer* transformer_);
    void ClearTransformer();
    friend class RTCEncodedVideoStreamTransformer;

    base::Lock transformer_lock_;
    raw_ptr<RTCEncodedVideoStreamTransformer> transformer_
        GUARDED_BY(transformer_lock_);
  };

  // Once an encoded Video frame is ready in WebRTC, it will be posted to the
  // realm_task_runner to be transformed - usually a JS thread where the JS
  // transform will execute.
  // If metronome is non-null, we will wait for the next tick before posting,
  // to reduce the wakeups of the target thread. Frames passed back from JS
  // after the transform are not delayed.
  explicit RTCEncodedVideoStreamTransformer(
      scoped_refptr<base::SingleThreadTaskRunner> realm_task_runner,
      std::unique_ptr<webrtc::Metronome> metronome);
  ~RTCEncodedVideoStreamTransformer();

  // Called by WebRTC to let us know about a callback object to send
  // transformed frames to the WebRTC decoder. Runs on the thread which
  // created this object. The callback can run on any thread.
  void RegisterTransformedFrameSinkCallback(
      webrtc::scoped_refptr<webrtc::TransformedFrameCallback>,
      uint32_t ssrc);

  // Called by WebRTC to let us know that any reference to the callback object
  // reported by RegisterTransformedFrameCallback() should be released since
  // the callback is no longer useful and is intended for destruction.
  // Runs on the thread which created this object.
  void UnregisterTransformedFrameSinkCallback(uint32_t ssrc);

  // Called by WebRTC to notify of new untransformed frames from the WebRTC
  // stack. Runs on the most recently set source task_runner - ie changes when
  // the stream is transferred.
  void TransformFrame(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface>);

  // Send a transformed frame to the WebRTC sink. Threadsafe.
  void SendFrameToSink(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface> frame);

  // Set a callback to be invoked on every untransformed frame. Is threadsafe.
  void SetTransformerCallback(TransformerCallback);

  // Removes the callback. Is threadsafe.
  void ResetTransformerCallback();

  // Returns true if a callback has been set with SetTransformerCallback(),
  // false otherwise. Is threadsafe.
  bool HasTransformerCallback();

  // Returns true if a webrtc::TransformedFrameCallback is registered.
  // Threadsafe.
  bool HasTransformedFrameCallback() const;

  // Returns true if a webrtc::TransformedFrameCallback is registered for
  // the given ssrc.
  bool HasTransformedFrameSinkCallback(uint32_t ssrc) const;

  webrtc::scoped_refptr<webrtc::FrameTransformerInterface> Delegate();

  // Set the TaskRunner used for the Source side - to deliver frames up to the
  // UnderlyingSource. Is threadsafe.
  void SetSourceTaskRunner(
      scoped_refptr<base::SingleThreadTaskRunner> realm_task_runner);

  scoped_refptr<Broker> GetBroker();

  void StartShortCircuiting();

 private:
  void LogMessage(const std::string& message);

  const scoped_refptr<Broker> broker_;
  const webrtc::scoped_refptr<webrtc::FrameTransformerInterface> delegate_;

  mutable base::Lock sink_lock_;
  Vector<std::pair<uint32_t,
                   webrtc::scoped_refptr<webrtc::TransformedFrameCallback>>>
      send_frame_to_sink_callbacks_ GUARDED_BY(sink_lock_);
  bool short_circuit_ GUARDED_BY(sink_lock_) = false;
  Vector<std::unique_ptr<webrtc::TransformableVideoFrameInterface>>
      buffered_frames_ GUARDED_BY(sink_lock_);
  uint16_t dropped_frames_count_ = 0;

  // source_lock_ must be taken *before* sink_lock_ to avoid deadlock.
  base::Lock source_lock_;
  TransformerCallback transformer_callback_ GUARDED_BY(source_lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_ENCODED_VIDEO_STREAM_TRANSFORMER_H_
