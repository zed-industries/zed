/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_CHANNEL_SEND_FRAME_TRANSFORMER_DELEGATE_H_
#define AUDIO_CHANNEL_SEND_FRAME_TRANSFORMER_DELEGATE_H_

#include <memory>
#include <string>

#include "api/frame_transformer_interface.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"
#include "rtc_base/buffer.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

// Delegates calls to FrameTransformerInterface to transform frames, and to
// ChannelSend to send the transformed frames using `send_frame_callback_` on
// the `encoder_queue_`.
// OnTransformedFrame() can be called from any thread, the delegate ensures
// thread-safe access to the ChannelSend callback.
class ChannelSendFrameTransformerDelegate : public TransformedFrameCallback {
 public:
  using SendFrameCallback =
      std::function<int32_t(AudioFrameType frameType,
                            uint8_t payloadType,
                            uint32_t rtp_timestamp_with_offset,
                            webrtc::ArrayView<const uint8_t> payload,
                            int64_t absolute_capture_timestamp_ms,
                            webrtc::ArrayView<const uint32_t> csrcs,
                            std::optional<uint8_t> audio_level_dbov)>;
  ChannelSendFrameTransformerDelegate(
      SendFrameCallback send_frame_callback,
      scoped_refptr<FrameTransformerInterface> frame_transformer,
      TaskQueueBase* encoder_queue);

  // Registers `this` as callback for `frame_transformer_`, to get the
  // transformed frames.
  void Init();

  // Unregisters and releases the `frame_transformer_` reference, and resets
  // `send_frame_callback_` under lock. Called from ChannelSend destructor to
  // prevent running the callback on a dangling channel.
  void Reset();

  // Delegates the call to FrameTransformerInterface::TransformFrame, to
  // transform the frame asynchronously.
  void Transform(AudioFrameType frame_type,
                 uint8_t payload_type,
                 uint32_t rtp_timestamp,
                 const uint8_t* payload_data,
                 size_t payload_size,
                 int64_t absolute_capture_timestamp_ms,
                 uint32_t ssrc,
                 const std::string& codec_mime_type,
                 std::optional<uint8_t> audio_level_dbov);

  // Implements TransformedFrameCallback. Can be called on any thread.
  void OnTransformedFrame(
      std::unique_ptr<TransformableFrameInterface> frame) override;

  void StartShortCircuiting() override;

  // Delegates the call to ChannelSend::SendRtpAudio on the `encoder_queue_`,
  // by calling `send_audio_callback_`.
  void SendFrame(std::unique_ptr<TransformableFrameInterface> frame) const;

 protected:
  ~ChannelSendFrameTransformerDelegate() override = default;

 private:
  mutable Mutex send_lock_;
  SendFrameCallback send_frame_callback_ RTC_GUARDED_BY(send_lock_);
  scoped_refptr<FrameTransformerInterface> frame_transformer_;
  TaskQueueBase* const encoder_queue_;
  bool short_circuit_ RTC_GUARDED_BY(send_lock_) = false;
};

std::unique_ptr<TransformableAudioFrameInterface> CloneSenderAudioFrame(
    TransformableAudioFrameInterface* original);

}  // namespace webrtc
#endif  // AUDIO_CHANNEL_SEND_FRAME_TRANSFORMER_DELEGATE_H_
