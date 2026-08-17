/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_ENCODED_FRAME_H_
#define API_VIDEO_ENCODED_FRAME_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <variant>

#include "api/units/timestamp.h"
#include "api/video/encoded_image.h"
#include "api/video/video_codec_type.h"
#include "common_video/frame_instrumentation_data.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "modules/video_coding/include/video_codec_interface.h"

namespace webrtc {

// TODO(philipel): Move transport specific info out of EncodedFrame.
// NOTE: This class is still under development and may change without notice.
class EncodedFrame : public EncodedImage {
 public:
  static const uint8_t kMaxFrameReferences = 5;

  EncodedFrame() = default;
  EncodedFrame(const EncodedFrame&) = default;
  virtual ~EncodedFrame() {}

  // When this frame was received.
  // TODO(bugs.webrtc.org/13756): Use Timestamp instead of int.
  virtual int64_t ReceivedTime() const { return -1; }
  // Returns a Timestamp from `ReceivedTime`, or nullopt if there is no receive
  // time.
  std::optional<webrtc::Timestamp> ReceivedTimestamp() const;

  // When this frame should be rendered.
  // TODO(bugs.webrtc.org/13756): Use Timestamp instead of int.
  virtual int64_t RenderTime() const { return _renderTimeMs; }
  // TODO(bugs.webrtc.org/13756): Migrate to ReceivedTimestamp.
  int64_t RenderTimeMs() const { return _renderTimeMs; }
  // Returns a Timestamp from `RenderTime`, or nullopt if there is no
  // render time.
  std::optional<webrtc::Timestamp> RenderTimestamp() const;

  // This information is currently needed by the timing calculation class.
  // TODO(philipel): Remove this function when a new timing class has
  //                 been implemented.
  virtual bool delayed_by_retransmission() const;

  bool is_keyframe() const { return num_references == 0; }

  void SetId(int64_t id) { id_ = id; }
  int64_t Id() const { return id_; }

  uint8_t PayloadType() const { return _payloadType; }

  void SetRenderTime(const int64_t renderTimeMs) {
    _renderTimeMs = renderTimeMs;
  }

  const webrtc::EncodedImage& EncodedImage() const {
    return static_cast<const webrtc::EncodedImage&>(*this);
  }

  const CodecSpecificInfo* CodecSpecific() const { return &_codecSpecificInfo; }
  void SetCodecSpecific(const CodecSpecificInfo* codec_specific) {
    _codecSpecificInfo = *codec_specific;
  }
  void SetFrameInstrumentationData(
      const std::optional<
          std::variant<FrameInstrumentationSyncData, FrameInstrumentationData>>
          frame_instrumentation) {
    _codecSpecificInfo.frame_instrumentation_data = frame_instrumentation;
  }

  // TODO(philipel): Add simple modify/access functions to prevent adding too
  // many `references`.
  size_t num_references = 0;
  int64_t references[kMaxFrameReferences];
  // Is this subframe the last one in the superframe (In RTP stream that would
  // mean that the last packet has a marker bit set).
  bool is_last_spatial_layer = true;

 protected:
  // TODO(https://bugs.webrtc.org/9378): Move RTP specifics down into a
  // transport-aware subclass, eg RtpFrameObject.
  void CopyCodecSpecific(const RTPVideoHeader* header);

  // TODO(https://bugs.webrtc.org/9378): Make fields private with
  // getters/setters as needed.
  int64_t _renderTimeMs = -1;
  uint8_t _payloadType = 0;
  CodecSpecificInfo _codecSpecificInfo;
  VideoCodecType _codec = kVideoCodecGeneric;

 private:
  // The ID of the frame is determined from RTP level information. The IDs are
  // used to describe order and dependencies between frames.
  int64_t id_ = -1;
};

}  // namespace webrtc

#endif  // API_VIDEO_ENCODED_FRAME_H_
