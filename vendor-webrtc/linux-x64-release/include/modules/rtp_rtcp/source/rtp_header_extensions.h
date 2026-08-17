/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTP_HEADER_EXTENSIONS_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_HEADER_EXTENSIONS_H_

#include <stddef.h>
#include <stdint.h>

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtp_headers.h"
#include "api/rtp_parameters.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/color_space.h"
#include "api/video/hdr_metadata.h"
#include "api/video/video_content_type.h"
#include "api/video/video_rotation.h"
#include "api/video/video_timing.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/checks.h"
#include "system_wrappers/include/ntp_time.h"

// This file contains class definitions for reading/writing each RTP header
// extension. Each class must be defined such that it is compatible with being
// an argument to the templated RtpPacket::GetExtension and
// RtpPacketToSend::SetExtension methods. New header extensions must have class
// names ending with "Extension", for the purpose of avoiding collisions with
// RTP extension information exposed in the public API of WebRTC.

namespace webrtc {

class AbsoluteSendTime {
 public:
  using value_type = uint32_t;
  static constexpr RTPExtensionType kId = kRtpExtensionAbsoluteSendTime;
  static constexpr uint8_t kValueSizeBytes = 3;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kAbsSendTimeUri;
  }

  static bool Parse(ArrayView<const uint8_t> data, uint32_t* time_24bits);
  static size_t ValueSize(uint32_t /* time_24bits */) {
    return kValueSizeBytes;
  }
  static bool Write(ArrayView<uint8_t> data, uint32_t time_24bits);

  static constexpr uint32_t To24Bits(Timestamp time) {
    int64_t time_us = time.us() % (int64_t{1 << 6} * 1'000'000);
    int64_t time6x18 = (time_us << 18) / 1'000'000;
    RTC_DCHECK_GE(time6x18, 0);
    RTC_DCHECK_LT(time6x18, 1 << 24);
    return static_cast<uint32_t>(time6x18);
  }

  static uint32_t To24Bits(NtpTime ntp_time) {
    uint64_t ntp_time32x32 = static_cast<uint64_t>(ntp_time);
    return (ntp_time32x32 >> 14) & 0x00FF'FFFF;
  }

  static constexpr Timestamp ToTimestamp(uint32_t time_24bits) {
    RTC_DCHECK_LT(time_24bits, (1 << 24));
    return Timestamp::Micros((time_24bits * int64_t{1'000'000}) >> 18);
  }
};

class AbsoluteCaptureTimeExtension {
 public:
  using value_type = AbsoluteCaptureTime;
  static constexpr RTPExtensionType kId = kRtpExtensionAbsoluteCaptureTime;
  static constexpr uint8_t kValueSizeBytes = 16;
  static constexpr uint8_t kValueSizeBytesWithoutEstimatedCaptureClockOffset =
      8;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kAbsoluteCaptureTimeUri;
  }

  static bool Parse(ArrayView<const uint8_t> data,
                    AbsoluteCaptureTime* extension);
  static size_t ValueSize(const AbsoluteCaptureTime& extension);
  static bool Write(ArrayView<uint8_t> data,
                    const AbsoluteCaptureTime& extension);
};

class AudioLevelExtension {
 public:
  using value_type = AudioLevel;
  static constexpr RTPExtensionType kId = kRtpExtensionAudioLevel;
  static constexpr uint8_t kValueSizeBytes = 1;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kAudioLevelUri;
  }

  static bool Parse(ArrayView<const uint8_t> data, AudioLevel* extension);
  static size_t ValueSize(const AudioLevel& /* extension */) {
    return kValueSizeBytes;
  }
  static bool Write(ArrayView<uint8_t> data, const AudioLevel& extension);
};

class CsrcAudioLevel {
 public:
  static constexpr RTPExtensionType kId = kRtpExtensionCsrcAudioLevel;
  static constexpr uint8_t kMaxValueSizeBytes = 15;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kCsrcAudioLevelsUri;
  }

  static bool Parse(ArrayView<const uint8_t> data,
                    std::vector<uint8_t>* csrc_audio_levels);
  static size_t ValueSize(ArrayView<const uint8_t> csrc_audio_levels);
  static bool Write(ArrayView<uint8_t> data,
                    ArrayView<const uint8_t> csrc_audio_levels);
};

class TransmissionOffset {
 public:
  using value_type = int32_t;
  static constexpr RTPExtensionType kId = kRtpExtensionTransmissionTimeOffset;
  static constexpr uint8_t kValueSizeBytes = 3;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kTimestampOffsetUri;
  }

  static bool Parse(ArrayView<const uint8_t> data, int32_t* rtp_time);
  static size_t ValueSize(int32_t /* rtp_time */) { return kValueSizeBytes; }
  static bool Write(ArrayView<uint8_t> data, int32_t rtp_time);
};

class TransportSequenceNumber {
 public:
  using value_type = uint16_t;
  static constexpr RTPExtensionType kId = kRtpExtensionTransportSequenceNumber;
  static constexpr uint8_t kValueSizeBytes = 2;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kTransportSequenceNumberUri;
  }

  static bool Parse(ArrayView<const uint8_t> data,
                    uint16_t* transport_sequence_number);
  static size_t ValueSize(uint16_t /*transport_sequence_number*/) {
    return kValueSizeBytes;
  }
  static bool Write(ArrayView<uint8_t> data,
                    uint16_t transport_sequence_number);
};

class TransportSequenceNumberV2 {
 public:
  static constexpr RTPExtensionType kId =
      kRtpExtensionTransportSequenceNumber02;
  static constexpr uint8_t kValueSizeBytes = 4;
  static constexpr uint8_t kValueSizeBytesWithoutFeedbackRequest = 2;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kTransportSequenceNumberV2Uri;
  }

  static bool Parse(ArrayView<const uint8_t> data,
                    uint16_t* transport_sequence_number,
                    std::optional<FeedbackRequest>* feedback_request);
  static size_t ValueSize(
      uint16_t /*transport_sequence_number*/,
      const std::optional<FeedbackRequest>& feedback_request) {
    return feedback_request ? kValueSizeBytes
                            : kValueSizeBytesWithoutFeedbackRequest;
  }
  static bool Write(ArrayView<uint8_t> data,
                    uint16_t transport_sequence_number,
                    const std::optional<FeedbackRequest>& feedback_request);

 private:
  static constexpr uint16_t kIncludeTimestampsBit = 1 << 15;
};

class VideoOrientation {
 public:
  using value_type = VideoRotation;
  static constexpr RTPExtensionType kId = kRtpExtensionVideoRotation;
  static constexpr uint8_t kValueSizeBytes = 1;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kVideoRotationUri;
  }

  static bool Parse(ArrayView<const uint8_t> data, VideoRotation* value);
  static size_t ValueSize(VideoRotation) { return kValueSizeBytes; }
  static bool Write(ArrayView<uint8_t> data, VideoRotation value);
  static bool Parse(ArrayView<const uint8_t> data, uint8_t* value);
  static size_t ValueSize(uint8_t /* value */) { return kValueSizeBytes; }
  static bool Write(ArrayView<uint8_t> data, uint8_t value);
};

class PlayoutDelayLimits {
 public:
  using value_type = VideoPlayoutDelay;
  static constexpr RTPExtensionType kId = kRtpExtensionPlayoutDelay;
  static constexpr uint8_t kValueSizeBytes = 3;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kPlayoutDelayUri;
  }

  // Playout delay in milliseconds. A playout delay limit (min or max)
  // has 12 bits allocated. This allows a range of 0-4095 values which
  // translates to a range of 0-40950 in milliseconds.
  static constexpr TimeDelta kGranularity = TimeDelta::Millis(10);
  // Maximum playout delay value in milliseconds.
  static constexpr TimeDelta kMax = 0xfff * kGranularity;  // 40950.

  static bool Parse(ArrayView<const uint8_t> data,
                    VideoPlayoutDelay* playout_delay);
  static size_t ValueSize(const VideoPlayoutDelay&) { return kValueSizeBytes; }
  static bool Write(ArrayView<uint8_t> data,
                    const VideoPlayoutDelay& playout_delay);
};

class VideoContentTypeExtension {
 public:
  using value_type = VideoContentType;
  static constexpr RTPExtensionType kId = kRtpExtensionVideoContentType;
  static constexpr uint8_t kValueSizeBytes = 1;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kVideoContentTypeUri;
  }

  static bool Parse(ArrayView<const uint8_t> data,
                    VideoContentType* content_type);
  static size_t ValueSize(VideoContentType) { return kValueSizeBytes; }
  static bool Write(ArrayView<uint8_t> data, VideoContentType content_type);
};

class VideoTimingExtension {
 public:
  using value_type = VideoSendTiming;
  static constexpr RTPExtensionType kId = kRtpExtensionVideoTiming;
  static constexpr uint8_t kValueSizeBytes = 13;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kVideoTimingUri;
  }

  // Offsets of the fields in the RTP header extension, counting from the first
  // byte after the one-byte header.
  static constexpr uint8_t kFlagsOffset = 0;
  static constexpr uint8_t kEncodeStartDeltaOffset = 1;
  static constexpr uint8_t kEncodeFinishDeltaOffset = 3;
  static constexpr uint8_t kPacketizationFinishDeltaOffset = 5;
  static constexpr uint8_t kPacerExitDeltaOffset = 7;
  static constexpr uint8_t kNetworkTimestampDeltaOffset = 9;
  static constexpr uint8_t kNetwork2TimestampDeltaOffset = 11;

  static bool Parse(ArrayView<const uint8_t> data, VideoSendTiming* timing);
  static size_t ValueSize(const VideoSendTiming&) { return kValueSizeBytes; }
  static bool Write(ArrayView<uint8_t> data, const VideoSendTiming& timing);

  static size_t ValueSize(uint16_t /* time_delta_ms */, uint8_t /* idx */) {
    return kValueSizeBytes;
  }
  // Writes only single time delta to position idx.
  static bool Write(ArrayView<uint8_t> data,
                    uint16_t time_delta_ms,
                    uint8_t offset);
};

class ColorSpaceExtension {
 public:
  using value_type = ColorSpace;
  static constexpr RTPExtensionType kId = kRtpExtensionColorSpace;
  static constexpr uint8_t kValueSizeBytes = 28;
  static constexpr uint8_t kValueSizeBytesWithoutHdrMetadata = 4;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kColorSpaceUri;
  }

  static bool Parse(ArrayView<const uint8_t> data, ColorSpace* color_space);
  static size_t ValueSize(const ColorSpace& color_space) {
    return color_space.hdr_metadata() ? kValueSizeBytes
                                      : kValueSizeBytesWithoutHdrMetadata;
  }
  static bool Write(ArrayView<uint8_t> data, const ColorSpace& color_space);

 private:
  static constexpr int kChromaticityDenominator = 50000;  // 0.00002 resolution.
  static constexpr int kLuminanceMaxDenominator = 1;      // 1 resolution.
  static constexpr int kLuminanceMinDenominator = 10000;  // 0.0001 resolution.

  static uint8_t CombineRangeAndChromaSiting(
      ColorSpace::RangeID range,
      ColorSpace::ChromaSiting chroma_siting_horizontal,
      ColorSpace::ChromaSiting chroma_siting_vertical);
  static size_t ParseHdrMetadata(ArrayView<const uint8_t> data,
                                 HdrMetadata* hdr_metadata);
  static size_t ParseChromaticity(const uint8_t* data,
                                  HdrMasteringMetadata::Chromaticity* p);
  static size_t ParseLuminance(const uint8_t* data, float* f, int denominator);
  static size_t WriteHdrMetadata(ArrayView<uint8_t> data,
                                 const HdrMetadata& hdr_metadata);
  static size_t WriteChromaticity(uint8_t* data,
                                  const HdrMasteringMetadata::Chromaticity& p);
  static size_t WriteLuminance(uint8_t* data, float f, int denominator);
};

// Base extension class for RTP header extensions which are strings.
// Subclasses must defined kId and kUri static constexpr members.
class BaseRtpStringExtension {
 public:
  using value_type = std::string;
  // String RTP header extensions are limited to 16 bytes because it is the
  // maximum length that can be encoded with one-byte header extensions.
  static constexpr uint8_t kMaxValueSizeBytes = 16;

  static bool Parse(ArrayView<const uint8_t> data, std::string* str);
  static size_t ValueSize(absl::string_view str) { return str.size(); }
  static bool Write(ArrayView<uint8_t> data, absl::string_view str);
};

class RtpStreamId : public BaseRtpStringExtension {
 public:
  static constexpr RTPExtensionType kId = kRtpExtensionRtpStreamId;
  static constexpr absl::string_view Uri() { return RtpExtension::kRidUri; }
};

class RepairedRtpStreamId : public BaseRtpStringExtension {
 public:
  static constexpr RTPExtensionType kId = kRtpExtensionRepairedRtpStreamId;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kRepairedRidUri;
  }
};

class RtpMid : public BaseRtpStringExtension {
 public:
  static constexpr RTPExtensionType kId = kRtpExtensionMid;
  static constexpr absl::string_view Uri() { return RtpExtension::kMidUri; }
};

class InbandComfortNoiseExtension {
 public:
  using value_type = std::optional<uint8_t>;

  static constexpr RTPExtensionType kId = kRtpExtensionInbandComfortNoise;
  static constexpr uint8_t kValueSizeBytes = 1;
  static constexpr const char kUri[] =
      "http://www.webrtc.org/experiments/rtp-hdrext/inband-cn";
  static constexpr absl::string_view Uri() { return kUri; }

  static bool Parse(ArrayView<const uint8_t> data,
                    std::optional<uint8_t>* level);
  static size_t ValueSize(std::optional<uint8_t> /* level */) {
    return kValueSizeBytes;
  }
  static bool Write(ArrayView<uint8_t> data, std::optional<uint8_t> level);
};

class VideoFrameTrackingIdExtension {
 public:
  using value_type = uint16_t;
  static constexpr RTPExtensionType kId = kRtpExtensionVideoFrameTrackingId;
  static constexpr uint8_t kValueSizeBytes = 2;
  static constexpr absl::string_view Uri() {
    return RtpExtension::kVideoFrameTrackingIdUri;
  }

  static bool Parse(ArrayView<const uint8_t> data,
                    uint16_t* video_frame_tracking_id);
  static size_t ValueSize(uint16_t /*video_frame_tracking_id*/) {
    return kValueSizeBytes;
  }
  static bool Write(ArrayView<uint8_t> data, uint16_t video_frame_tracking_id);
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_RTP_HEADER_EXTENSIONS_H_
