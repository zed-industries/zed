/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_SENDER_VIDEO_FRAME_TRANSFORMER_DELEGATE_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_SENDER_VIDEO_FRAME_TRANSFORMER_DELEGATE_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/frame_transformer_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/encoded_image.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_layers_allocation.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Interface for sending videoframes on an RTP connection, after a transform
// have been applied.
class RTPVideoFrameSenderInterface {
 public:
  virtual bool SendVideo(int payload_type,
                         std::optional<VideoCodecType> codec_type,
                         uint32_t rtp_timestamp,
                         Timestamp capture_time,
                         ArrayView<const uint8_t> payload,
                         size_t encoder_output_size,
                         RTPVideoHeader video_header,
                         TimeDelta expected_retransmission_time,
                         std::vector<uint32_t> csrcs) = 0;

  virtual void SetVideoStructureAfterTransformation(
      const FrameDependencyStructure* video_structure) = 0;
  virtual void SetVideoLayersAllocationAfterTransformation(
      VideoLayersAllocation allocation) = 0;

 protected:
  virtual ~RTPVideoFrameSenderInterface() = default;
};

// Delegates calls to FrameTransformerInterface to transform frames, and to
// RTPSenderVideo to send the transformed frames. Ensures thread-safe access to
// the sender.
class RTPSenderVideoFrameTransformerDelegate : public TransformedFrameCallback {
 public:
  RTPSenderVideoFrameTransformerDelegate(
      RTPVideoFrameSenderInterface* sender,
      scoped_refptr<FrameTransformerInterface> frame_transformer,
      uint32_t ssrc,
      TaskQueueFactory* send_transport_queue);

  void Init();

  // Delegates the call to FrameTransformerInterface::TransformFrame.
  bool TransformFrame(int payload_type,
                      std::optional<VideoCodecType> codec_type,
                      uint32_t rtp_timestamp,
                      const EncodedImage& encoded_image,
                      RTPVideoHeader video_header,
                      TimeDelta expected_retransmission_time);

  // Implements TransformedFrameCallback. Can be called on any thread. Posts
  // the transformed frame to be sent on the `encoder_queue_`.
  void OnTransformedFrame(
      std::unique_ptr<TransformableFrameInterface> frame) override;

  void StartShortCircuiting() override;

  // Delegates the call to RTPSendVideo::SendVideo on the `encoder_queue_`.
  void SendVideo(std::unique_ptr<TransformableFrameInterface> frame) const
      RTC_RUN_ON(transformation_queue_);

  // Delegates the call to RTPSendVideo::SetVideoStructureAfterTransformation
  // under `sender_lock_`.
  void SetVideoStructureUnderLock(
      const FrameDependencyStructure* video_structure);

  // Delegates the call to
  // RTPSendVideo::SetVideoLayersAllocationAfterTransformation under
  // `sender_lock_`.
  void SetVideoLayersAllocationUnderLock(VideoLayersAllocation allocation);

  // Unregisters and releases the `frame_transformer_` reference, and resets
  // `sender_` under lock. Called from RTPSenderVideo destructor to prevent the
  // `sender_` to dangle.
  void Reset();

 protected:
  ~RTPSenderVideoFrameTransformerDelegate() override = default;

 private:
  void EnsureEncoderQueueCreated();

  mutable Mutex sender_lock_;
  RTPVideoFrameSenderInterface* sender_ RTC_GUARDED_BY(sender_lock_);
  scoped_refptr<FrameTransformerInterface> frame_transformer_;
  const uint32_t ssrc_;
  // Used when the encoded frames arrives without a current task queue. This can
  // happen if a hardware encoder was used.
  std::unique_ptr<TaskQueueBase, TaskQueueDeleter> transformation_queue_;
  bool short_circuit_ RTC_GUARDED_BY(sender_lock_) = false;
};

// Method to support cloning a Sender frame from another frame
std::unique_ptr<TransformableVideoFrameInterface> CloneSenderVideoFrame(
    TransformableVideoFrameInterface* original);

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_SENDER_VIDEO_FRAME_TRANSFORMER_DELEGATE_H_
