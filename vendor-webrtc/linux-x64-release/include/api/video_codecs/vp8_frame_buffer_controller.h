/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VP8_FRAME_BUFFER_CONTROLLER_H_
#define API_VIDEO_CODECS_VP8_FRAME_BUFFER_CONTROLLER_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "api/fec_controller_override.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/vp8_frame_config.h"

namespace webrtc {

// Some notes on the prerequisites of the TemporalLayers interface.
// * Vp8FrameBufferController is not thread safe, synchronization is the
//   caller's responsibility.
// * The encoder is assumed to encode all frames in order, and callbacks to
//   PopulateCodecSpecific() / OnEncodeDone() must happen in the same order.
//
// This means that in the case of pipelining encoders, it is OK to have a chain
// of calls such as this:
// - NextFrameConfig(timestampA)
// - NextFrameConfig(timestampB)
// - PopulateCodecSpecific(timestampA, ...)
// - NextFrameConfig(timestampC)
// - OnEncodeDone(timestampA, 1234, ...)
// - NextFrameConfig(timestampC)
// - OnEncodeDone(timestampB, 0, ...)
// - OnEncodeDone(timestampC, 1234, ...)
// Note that NextFrameConfig() for a new frame can happen before
// OnEncodeDone() for a previous one, but calls themselves must be both
// synchronized (e.g. run on a task queue) and in order (per type).
//
// TODO(eladalon): Revise comment (referring to PopulateCodecSpecific in this
// context is not very meaningful).

struct CodecSpecificInfo;

// Each member represents an override of the VPX configuration if the optional
// value is set.
struct Vp8EncoderConfig {
  struct TemporalLayerConfig {
    bool operator!=(const TemporalLayerConfig& other) const {
      return ts_number_layers != other.ts_number_layers ||
             ts_target_bitrate != other.ts_target_bitrate ||
             ts_rate_decimator != other.ts_rate_decimator ||
             ts_periodicity != other.ts_periodicity ||
             ts_layer_id != other.ts_layer_id;
    }

    static constexpr size_t kMaxPeriodicity = 16;
    static constexpr size_t kMaxLayers = 5;

    // Number of active temporal layers. Set to 0 if not used.
    uint32_t ts_number_layers;

    // Arrays of length `ts_number_layers`, indicating (cumulative) target
    // bitrate and rate decimator (e.g. 4 if every 4th frame is in the given
    // layer) for each active temporal layer, starting with temporal id 0.
    std::array<uint32_t, kMaxLayers> ts_target_bitrate;
    std::array<uint32_t, kMaxLayers> ts_rate_decimator;

    // The periodicity of the temporal pattern. Set to 0 if not used.
    uint32_t ts_periodicity;

    // Array of length `ts_periodicity` indicating the sequence of temporal id's
    // to assign to incoming frames.
    std::array<uint32_t, kMaxPeriodicity> ts_layer_id;
  };

  std::optional<TemporalLayerConfig> temporal_layer_config;

  // Target bitrate, in bps.
  std::optional<uint32_t> rc_target_bitrate;

  // Clamp QP to max. Use 0 to disable clamping.
  std::optional<uint32_t> rc_max_quantizer;

  // Error resilience mode.
  std::optional<uint32_t> g_error_resilient;

  // If set to true, all previous configuration overrides should be reset.
  bool reset_previous_configuration_overrides = false;
};

// This interface defines a way of delegating the logic of buffer management.
// Multiple streams may be controlled by a single controller, demuxing between
// them using stream_index.
class Vp8FrameBufferController {
 public:
  virtual ~Vp8FrameBufferController() = default;

  // Set limits on QP.
  // The limits are suggestion-only; the controller is allowed to exceed them.
  virtual void SetQpLimits(size_t stream_index, int min_qp, int max_qp) = 0;

  // Number of streamed controlled by `this`.
  virtual size_t StreamCount() const = 0;

  // If this method returns true, the encoder is free to drop frames for
  // instance in an effort to uphold encoding bitrate.
  // If this return false, the encoder must not drop any frames unless:
  //  1. Requested to do so via Vp8FrameConfig.drop_frame
  //  2. The frame to be encoded is requested to be a keyframe
  //  3. The encoder detected a large overshoot and decided to drop and then
  //     re-encode the image at a low bitrate. In this case the encoder should
  //     call OnFrameDropped() once to indicate drop, and then call
  //     OnEncodeDone() again when the frame has actually been encoded.
  virtual bool SupportsEncoderFrameDropping(size_t stream_index) const = 0;

  // New target bitrate for a stream (each entry in
  // `bitrates_bps` is for another temporal layer).
  virtual void OnRatesUpdated(size_t stream_index,
                              const std::vector<uint32_t>& bitrates_bps,
                              int framerate_fps) = 0;

  // Called by the encoder before encoding a frame. Returns a set of overrides
  // the controller wishes to enact in the encoder's configuration.
  // If a value is not overridden, previous overrides are still in effect.
  // However, if `Vp8EncoderConfig::reset_previous_configuration_overrides`
  // is set to `true`, all previous overrides are reset.
  virtual Vp8EncoderConfig UpdateConfiguration(size_t stream_index) = 0;

  // Returns the recommended VP8 encode flags needed.
  // The timestamp may be used as both a time and a unique identifier, and so
  // the caller must make sure no two frames use the same timestamp.
  // The timestamp uses a 90kHz RTP clock.
  // After calling this method, first call the actual encoder with the provided
  // frame configuration, and then OnEncodeDone() below.
  virtual Vp8FrameConfig NextFrameConfig(size_t stream_index,
                                         uint32_t rtp_timestamp) = 0;

  // Called after the encode step is done. `rtp_timestamp` must match the
  // parameter use in the NextFrameConfig() call.
  // `is_keyframe` must be true iff the encoder decided to encode this frame as
  // a keyframe.
  // If `info` is not null, the encoder may update `info` with codec specific
  // data such as temporal id. `qp` should indicate the frame-level QP this
  // frame was encoded at. If the encoder does not support extracting this, `qp`
  // should be set to 0.
  virtual void OnEncodeDone(size_t stream_index,
                            uint32_t rtp_timestamp,
                            size_t size_bytes,
                            bool is_keyframe,
                            int qp,
                            CodecSpecificInfo* info) = 0;

  // Called when a frame is dropped by the encoder.
  virtual void OnFrameDropped(size_t stream_index, uint32_t rtp_timestamp) = 0;

  // Called by the encoder when the packet loss rate changes.
  // `packet_loss_rate` runs between 0.0 (no loss) and 1.0 (everything lost).
  virtual void OnPacketLossRateUpdate(float packet_loss_rate) = 0;

  // Called by the encoder when the round trip time changes.
  virtual void OnRttUpdate(int64_t rtt_ms) = 0;

  // Called when a loss notification is received.
  virtual void OnLossNotification(
      const VideoEncoder::LossNotification& loss_notification) = 0;
};

// Interface for a factory of Vp8FrameBufferController instances.
class Vp8FrameBufferControllerFactory {
 public:
  virtual ~Vp8FrameBufferControllerFactory() = default;

  // Clones oneself. (Avoids Vp8FrameBufferControllerFactoryFactory.)
  virtual std::unique_ptr<Vp8FrameBufferControllerFactory> Clone() const = 0;

  // Create a Vp8FrameBufferController instance.
  virtual std::unique_ptr<Vp8FrameBufferController> Create(
      const VideoCodec& codec,
      const VideoEncoder::Settings& settings,
      FecControllerOverride* fec_controller_override) = 0;
};

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VP8_FRAME_BUFFER_CONTROLLER_H_
