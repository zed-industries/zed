/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TRANSPORT_RTP_RTP_SOURCE_H_
#define API_TRANSPORT_RTP_RTP_SOURCE_H_

#include <stdint.h>

#include <optional>

#include "api/rtp_headers.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"

namespace webrtc {

enum class RtpSourceType {
  SSRC,
  CSRC,
};

class RtpSource {
 public:
  struct Extensions {
    std::optional<uint8_t> audio_level;

    // Fields from the Absolute Capture Time header extension:
    // http://www.webrtc.org/experiments/rtp-hdrext/abs-capture-time
    std::optional<AbsoluteCaptureTime> absolute_capture_time;

    // Clock offset between the local clock and the capturer's clock.
    // Do not confuse with `AbsoluteCaptureTime::estimated_capture_clock_offset`
    // which instead represents the clock offset between a remote sender and the
    // capturer. The following holds:
    //   Capture's NTP Clock = Local NTP Clock + Local-Capture Clock Offset
    std::optional<TimeDelta> local_capture_clock_offset;
  };

  RtpSource() = delete;

  RtpSource(Timestamp timestamp,
            uint32_t source_id,
            RtpSourceType source_type,
            uint32_t rtp_timestamp,
            const RtpSource::Extensions& extensions)
      : timestamp_(timestamp),
        source_id_(source_id),
        source_type_(source_type),
        extensions_(extensions),
        rtp_timestamp_(rtp_timestamp) {}

  RtpSource(const RtpSource&) = default;
  RtpSource& operator=(const RtpSource&) = default;
  ~RtpSource() = default;

  Timestamp timestamp() const { return timestamp_; }

  // The identifier of the source can be the CSRC or the SSRC.
  uint32_t source_id() const { return source_id_; }

  // The source can be either a contributing source or a synchronization source.
  RtpSourceType source_type() const { return source_type_; }

  std::optional<uint8_t> audio_level() const { return extensions_.audio_level; }

  void set_audio_level(const std::optional<uint8_t>& level) {
    extensions_.audio_level = level;
  }

  uint32_t rtp_timestamp() const { return rtp_timestamp_; }

  std::optional<AbsoluteCaptureTime> absolute_capture_time() const {
    return extensions_.absolute_capture_time;
  }

  std::optional<TimeDelta> local_capture_clock_offset() const {
    return extensions_.local_capture_clock_offset;
  }

  bool operator==(const RtpSource& o) const {
    return timestamp_ == o.timestamp() && source_id_ == o.source_id() &&
           source_type_ == o.source_type() &&
           extensions_.audio_level == o.extensions_.audio_level &&
           extensions_.absolute_capture_time ==
               o.extensions_.absolute_capture_time &&
           rtp_timestamp_ == o.rtp_timestamp();
  }

 private:
  Timestamp timestamp_;
  uint32_t source_id_;
  RtpSourceType source_type_;
  RtpSource::Extensions extensions_;
  uint32_t rtp_timestamp_;
};

}  // namespace webrtc

#endif  // API_TRANSPORT_RTP_RTP_SOURCE_H_
