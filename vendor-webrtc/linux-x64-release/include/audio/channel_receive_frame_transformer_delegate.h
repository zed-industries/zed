/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_CHANNEL_RECEIVE_FRAME_TRANSFORMER_DELEGATE_H_
#define AUDIO_CHANNEL_RECEIVE_FRAME_TRANSFORMER_DELEGATE_H_

#include <memory>
#include <string>

#include "api/frame_transformer_interface.h"
#include "api/rtp_headers.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/timestamp.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread.h"

namespace webrtc {

// Delegates calls to FrameTransformerInterface to transform frames, and to
// ChannelReceive to receive the transformed frames using the
// `receive_frame_callback_` on the `channel_receive_thread_`.
class ChannelReceiveFrameTransformerDelegate : public TransformedFrameCallback {
 public:
  using ReceiveFrameCallback =
      std::function<void(webrtc::ArrayView<const uint8_t> packet,
                         const RTPHeader& header,
                         Timestamp receive_time)>;
  ChannelReceiveFrameTransformerDelegate(
      ReceiveFrameCallback receive_frame_callback,
      scoped_refptr<FrameTransformerInterface> frame_transformer,
      TaskQueueBase* channel_receive_thread);

  // Registers `this` as callback for `frame_transformer_`, to get the
  // transformed frames.
  void Init();

  // Unregisters and releases the `frame_transformer_` reference, and resets
  // `receive_frame_callback_` on `channel_receive_thread_`. Called from
  // ChannelReceive destructor to prevent running the callback on a dangling
  // channel.
  void Reset();

  // Delegates the call to FrameTransformerInterface::Transform, to transform
  // the frame asynchronously.
  void Transform(ArrayView<const uint8_t> packet,
                 const RTPHeader& header,
                 uint32_t ssrc,
                 const std::string& codec_mime_type,
                 Timestamp receive_time);

  // Implements TransformedFrameCallback. Can be called on any thread.
  void OnTransformedFrame(
      std::unique_ptr<TransformableFrameInterface> frame) override;

  void StartShortCircuiting() override;

  // Delegates the call to ChannelReceive::OnReceivedPayloadData on the
  // `channel_receive_thread_`, by calling `receive_frame_callback_`.
  void ReceiveFrame(std::unique_ptr<TransformableFrameInterface> frame) const;

  scoped_refptr<FrameTransformerInterface> FrameTransformer();

 protected:
  ~ChannelReceiveFrameTransformerDelegate() override = default;

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
  ReceiveFrameCallback receive_frame_callback_
      RTC_GUARDED_BY(sequence_checker_);
  scoped_refptr<FrameTransformerInterface> frame_transformer_
      RTC_GUARDED_BY(sequence_checker_);
  TaskQueueBase* const channel_receive_thread_;
  bool short_circuit_ RTC_GUARDED_BY(sequence_checker_) = false;
};

std::unique_ptr<TransformableAudioFrameInterface> CloneReceiverAudioFrame(
    TransformableAudioFrameInterface* original);

}  // namespace webrtc
#endif  // AUDIO_CHANNEL_RECEIVE_FRAME_TRANSFORMER_DELEGATE_H_
