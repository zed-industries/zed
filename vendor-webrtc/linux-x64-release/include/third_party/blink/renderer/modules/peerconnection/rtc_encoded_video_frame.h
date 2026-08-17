// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_FRAME_H_

#include <stdint.h>

#include <memory>

#include "base/types/expected.h"
#include "base/unguessable_token.h"
#include "third_party/blink/public/common/features.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace webrtc {
class TransformableVideoFrameInterface;
}  // namespace webrtc

namespace blink {

class DOMArrayBuffer;
class RTCEncodedVideoFrameDelegate;
class RTCEncodedVideoFrameMetadata;
class RTCEncodedVideoFrameOptions;
class V8RTCEncodedVideoFrameType;

MODULES_EXPORT BASE_DECLARE_FEATURE(
    kAllowRTCEncodedVideoFrameSetMetadataAllFields);

class MODULES_EXPORT RTCEncodedVideoFrame final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static RTCEncodedVideoFrame* Create(ExecutionContext* context,
                                      RTCEncodedVideoFrame* original_frame,
                                      ExceptionState& exception_state);
  static RTCEncodedVideoFrame* Create(
      ExecutionContext* context,
      RTCEncodedVideoFrame* original_frame,
      const RTCEncodedVideoFrameOptions* options_dict,
      ExceptionState& exception_state);
  explicit RTCEncodedVideoFrame(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface> webrtc_frame);
  explicit RTCEncodedVideoFrame(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface> webrtc_frame,
      base::UnguessableToken owner_id,
      int64_t counter);
  explicit RTCEncodedVideoFrame(
      scoped_refptr<RTCEncodedVideoFrameDelegate> delegate);

  // rtc_encoded_video_frame.idl implementation.
  V8RTCEncodedVideoFrameType type() const;
  // Returns the RTP Packet Timestamp for this frame.
  uint32_t timestamp() const;
  DOMArrayBuffer* data(ExecutionContext* context) const;
  RTCEncodedVideoFrameMetadata* getMetadata(ExecutionContext* context) const;
  base::expected<void, String> SetMetadata(
      ExecutionContext* context,
      const RTCEncodedVideoFrameMetadata* metadata);
  void setMetadata(ExecutionContext* context,
                   RTCEncodedVideoFrameMetadata* metadata,
                   ExceptionState& exception_state);
  void setData(ExecutionContext*, DOMArrayBuffer*);
  String toString(ExecutionContext* context) const;

  base::UnguessableToken OwnerId();
  int64_t Counter();

  scoped_refptr<RTCEncodedVideoFrameDelegate> Delegate() const;
  void SyncDelegate() const;

  // Returns and transfers ownership of the internal WebRTC frame
  // backing this RTCEncodedVideoFrame, neutering all RTCEncodedVideoFrames
  // backed by that internal WebRTC frame.
  std::unique_ptr<webrtc::TransformableVideoFrameInterface> PassWebRtcFrame(
      v8::Isolate* isolate,
      bool detach_frame_data);

  void Trace(Visitor*) const override;

 private:
  const scoped_refptr<RTCEncodedVideoFrameDelegate> delegate_;

  // Exposes encoded frame data from |delegate_|.
  mutable Member<DOMArrayBuffer> frame_data_;
  base::UnguessableToken owner_id_;
  int64_t counter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_FRAME_H_
