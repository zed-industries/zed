// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_FRAME_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_FRAME_DELEGATE_H_

#include <stdint.h>

#include <memory>

#include "base/synchronization/lock.h"
#include "base/time/time.h"
#include "base/types/expected.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_encoded_video_frame_type.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/webrtc/api/frame_transformer_interface.h"
#include "third_party/webrtc/api/video/video_frame_metadata.h"

namespace blink {

class DOMArrayBuffer;

// This class wraps a WebRTC video frame and allows making shallow
// copies. Its purpose is to support making RTCEncodedVideoFrames
// serializable in the same process.
class RTCEncodedVideoFrameDelegate
    : public WTF::ThreadSafeRefCounted<RTCEncodedVideoFrameDelegate> {
 public:
  explicit RTCEncodedVideoFrameDelegate(
      std::unique_ptr<webrtc::TransformableVideoFrameInterface> webrtc_frame);

  V8RTCEncodedVideoFrameType::Enum Type() const;
  uint32_t RtpTimestamp() const;
  std::optional<webrtc::Timestamp> PresentationTimestamp() const;
  DOMArrayBuffer* CreateDataBuffer(v8::Isolate* isolate) const;
  void SetData(const DOMArrayBuffer* data);
  std::optional<uint8_t> PayloadType() const;
  std::optional<std::string> MimeType() const;
  std::optional<webrtc::VideoFrameMetadata> GetMetadata() const;
  std::optional<base::TimeTicks> ReceiveTime() const;
  std::optional<base::TimeTicks> CaptureTime() const;
  std::optional<base::TimeDelta> SenderCaptureTimeOffset() const;
  base::expected<void, String> SetMetadata(
      const webrtc::VideoFrameMetadata& metadata,
      uint32_t rtpTimestamp);
  std::unique_ptr<webrtc::TransformableVideoFrameInterface> PassWebRtcFrame();
  std::unique_ptr<webrtc::TransformableVideoFrameInterface> CloneWebRtcFrame();

 private:
  mutable base::Lock lock_;
  std::unique_ptr<webrtc::TransformableVideoFrameInterface> webrtc_frame_
      GUARDED_BY(lock_);
};

class MODULES_EXPORT RTCEncodedVideoFramesAttachment
    : public SerializedScriptValue::Attachment {
 public:
  static const void* const kAttachmentKey;
  RTCEncodedVideoFramesAttachment() = default;
  ~RTCEncodedVideoFramesAttachment() override = default;

  bool IsLockedToAgentCluster() const override {
    return !encoded_video_frames_.empty();
  }

  Vector<scoped_refptr<RTCEncodedVideoFrameDelegate>>& EncodedVideoFrames() {
    return encoded_video_frames_;
  }

  const Vector<scoped_refptr<RTCEncodedVideoFrameDelegate>>&
  EncodedVideoFrames() const {
    return encoded_video_frames_;
  }

 private:
  Vector<scoped_refptr<RTCEncodedVideoFrameDelegate>> encoded_video_frames_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_VIDEO_FRAME_DELEGATE_H_
