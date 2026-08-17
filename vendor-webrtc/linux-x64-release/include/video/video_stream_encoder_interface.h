/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_VIDEO_STREAM_ENCODER_INTERFACE_H_
#define VIDEO_VIDEO_STREAM_ENCODER_INTERFACE_H_

#include <vector>

#include "api/adaptation/resource.h"
#include "api/fec_controller_override.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"  // For DegradationPreference.
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/units/data_rate.h"
#include "api/video/video_bitrate_allocator.h"
#include "api/video/video_layers_allocation.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "api/video_codecs/video_encoder.h"
#include "video/config/video_encoder_config.h"

namespace webrtc {

// This interface represents a class responsible for creating and driving the
// encoder(s) for a single video stream. It is also responsible for adaptation
// decisions related to video quality, requesting reduced frame rate or
// resolution from the VideoSource when needed.
// TODO(bugs.webrtc.org/8830): This interface is under development. Changes
// under consideration include:
//
// 1. Taking out responsibility for adaptation decisions, instead only reporting
//    per-frame measurements to the decision maker.
//
// 2. Moving responsibility for simulcast and for software fallback into this
//    class.
class VideoStreamEncoderInterface {
 public:
  // Interface for receiving encoded video frames and notifications about
  // configuration changes.
  class EncoderSink : public EncodedImageCallback {
   public:
    virtual void OnEncoderConfigurationChanged(
        std::vector<VideoStream> streams,
        bool is_svc,
        VideoEncoderConfig::ContentType content_type,
        int min_transmit_bitrate_bps) = 0;

    virtual void OnBitrateAllocationUpdated(
        const VideoBitrateAllocation& allocation) = 0;

    virtual void OnVideoLayersAllocationUpdated(
        VideoLayersAllocation allocation) = 0;
  };

  virtual ~VideoStreamEncoderInterface() = default;

  // If the resource is overusing, the VideoStreamEncoder will try to reduce
  // resolution or frame rate until no resource is overusing.
  // TODO(https://crbug.com/webrtc/11565): When the ResourceAdaptationProcessor
  // is moved to Call this method could be deleted altogether in favor of
  // Call-level APIs only.
  virtual void AddAdaptationResource(scoped_refptr<Resource> resource) = 0;
  virtual std::vector<scoped_refptr<Resource>> GetAdaptationResources() = 0;

  // Sets the source that will provide video frames to the VideoStreamEncoder's
  // OnFrame method. `degradation_preference` control whether or not resolution
  // or frame rate may be reduced. The VideoStreamEncoder registers itself with
  // `source`, and signals adaptation decisions to the source in the form of
  // VideoSinkWants.
  // TODO(bugs.webrtc.org/14246): When adaptation logic is extracted from this
  // class, it no longer needs to know the source.
  virtual void SetSource(
      VideoSourceInterface<VideoFrame>* source,
      const DegradationPreference& degradation_preference) = 0;

  // Sets the `sink` that gets the encoded frames. `rotation_applied` means
  // that the source must support rotation. Only set `rotation_applied` if the
  // remote side does not support the rotation extension.
  virtual void SetSink(EncoderSink* sink, bool rotation_applied) = 0;

  // Sets an initial bitrate, later overriden by OnBitrateUpdated. Mainly
  // affects the resolution of the initial key frame: If incoming frames are
  // larger than reasonable for the start bitrate, and scaling is enabled,
  // VideoStreamEncoder asks the source to scale down and drops a few initial
  // frames.
  // TODO(nisse): This is a poor interface, and mixes bandwidth estimation and
  // codec configuration in an undesired way. For the actual send bandwidth, we
  // should always be somewhat conservative, but we may nevertheless want to let
  // the application configure a more optimistic quality for the initial
  // resolution. Should be replaced by a construction time setting.
  virtual void SetStartBitrate(int start_bitrate_bps) = 0;

  // Request a key frame. Used for signalling from the remote receiver with
  // no arguments and for RTCRtpSender.generateKeyFrame with a list of
  // rids/layers.
  virtual void SendKeyFrame(const std::vector<VideoFrameType>& layers = {}) = 0;

  // Inform the encoder that a loss has occurred.
  virtual void OnLossNotification(
      const VideoEncoder::LossNotification& loss_notification) = 0;

  // Set the currently estimated network properties. A `target_bitrate`
  // of zero pauses the encoder.
  // `stable_target_bitrate` is a filtered version of `target_bitrate`. It  is
  // always less or equal to it. It can be used to avoid rapid changes of
  // expensive encoding settings, such as resolution.
  // `link_allocation` is the bandwidth available for this video stream on the
  // network link. It is always at least `target_bitrate` but may be higher
  // if we are not network constrained.
  virtual void OnBitrateUpdated(DataRate target_bitrate,
                                DataRate stable_target_bitrate,
                                DataRate link_allocation,
                                uint8_t fraction_lost,
                                int64_t round_trip_time_ms,
                                double cwnd_reduce_ratio) = 0;

  // Set a FecControllerOverride, through which the encoder may override
  // decisions made by FecController.
  virtual void SetFecControllerOverride(
      FecControllerOverride* fec_controller_override) = 0;

  // Creates and configures an encoder with the given `config`. The
  // `max_data_payload_length` is used to support single NAL unit
  // packetization for H.264.
  virtual void ConfigureEncoder(VideoEncoderConfig config,
                                size_t max_data_payload_length) = 0;
  virtual void ConfigureEncoder(VideoEncoderConfig config,
                                size_t max_data_payload_length,
                                SetParametersCallback callback) = 0;

  // Permanently stop encoding. After this method has returned, it is
  // guaranteed that no encoded frames will be delivered to the sink.
  virtual void Stop() = 0;
};

}  // namespace webrtc

#endif  // VIDEO_VIDEO_STREAM_ENCODER_INTERFACE_H_
