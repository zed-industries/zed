// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_AUDIO_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_AUDIO_FRAME_H_

#include <stdint.h>

#include <optional>

#include "base/types/expected.h"
#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace webrtc {
class TransformableAudioFrameInterface;
}  // namespace webrtc

namespace blink {

class DOMArrayBuffer;
class RTCEncodedAudioFrameDelegate;
class RTCEncodedAudioFrameMetadata;
class RTCEncodedAudioFrameOptions;

class MODULES_EXPORT RTCEncodedAudioFrame final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static RTCEncodedAudioFrame* Create(ExecutionContext*,
                                      RTCEncodedAudioFrame* original_frame,
                                      ExceptionState& exception_state);
  static RTCEncodedAudioFrame* Create(
      ExecutionContext*,
      RTCEncodedAudioFrame* original_frame,
      const RTCEncodedAudioFrameOptions* options_dict,
      ExceptionState& exception_state);
  explicit RTCEncodedAudioFrame(
      std::unique_ptr<webrtc::TransformableAudioFrameInterface> webrtc_frame);
  explicit RTCEncodedAudioFrame(
      std::unique_ptr<webrtc::TransformableAudioFrameInterface> webrtc_frame,
      base::UnguessableToken owner_id,
      int64_t counter);
  explicit RTCEncodedAudioFrame(
      scoped_refptr<RTCEncodedAudioFrameDelegate> delegate);

  // rtc_encoded_audio_frame.idl implementation.
  // Returns the RTP Packet Timestamp for this frame.
  uint32_t timestamp() const;
  std::optional<uint16_t> sequenceNumber() const;
  DOMArrayBuffer* data(ExecutionContext* context) const;
  RTCEncodedAudioFrameMetadata* getMetadata(ExecutionContext*) const;
  base::expected<void, String> SetMetadata(ExecutionContext*,
                                           const RTCEncodedAudioFrameMetadata*);
  void setMetadata(ExecutionContext*,
                   RTCEncodedAudioFrameMetadata*,
                   ExceptionState&);
  void setData(ExecutionContext*, DOMArrayBuffer*);
  String toString(ExecutionContext* context) const;

  base::UnguessableToken OwnerId();
  int64_t Counter();

  scoped_refptr<RTCEncodedAudioFrameDelegate> Delegate() const;
  void SyncDelegate() const;

  // Returns and transfers ownership of the internal WebRTC frame
  // backing this RTCEncodedAudioFrame, neutering all RTCEncodedAudioFrames
  // backed by that internal WebRTC frame.
  std::unique_ptr<webrtc::TransformableAudioFrameInterface> PassWebRtcFrame(
      v8::Isolate* isolate,
      bool detach_frame_data);

  void Trace(Visitor*) const override;

 private:
  scoped_refptr<RTCEncodedAudioFrameDelegate> delegate_;
  mutable Member<DOMArrayBuffer> frame_data_;
  base::UnguessableToken owner_id_;
  int64_t counter_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ENCODED_AUDIO_FRAME_H_
