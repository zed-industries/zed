/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_SDP_VIDEO_FORMAT_H_
#define API_VIDEO_CODECS_SDP_VIDEO_FORMAT_H_

#include <map>
#include <optional>
#include <string>

#include "absl/container/inlined_vector.h"
#include "api/array_view.h"
#include "api/rtp_parameters.h"
#include "api/video_codecs/scalability_mode.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// SDP specification for a single video codec.
// NOTE: This class is still under development and may change without notice.
struct RTC_EXPORT SdpVideoFormat {
  using Parameters [[deprecated("Use webrtc::CodecParameterMap")]] =
      std::map<std::string, std::string>;

  explicit SdpVideoFormat(const std::string& name);
  SdpVideoFormat(const std::string& name, const CodecParameterMap& parameters);
  SdpVideoFormat(
      const std::string& name,
      const CodecParameterMap& parameters,
      const absl::InlinedVector<ScalabilityMode, kScalabilityModeCount>&
          scalability_modes);
  // Creates a new SdpVideoFormat object identical to the supplied
  // SdpVideoFormat except the scalability_modes that are set to be the same as
  // the supplied scalability modes.
  SdpVideoFormat(
      const SdpVideoFormat& format,
      const absl::InlinedVector<ScalabilityMode, kScalabilityModeCount>&
          scalability_modes);

  SdpVideoFormat(const SdpVideoFormat&);
  SdpVideoFormat(SdpVideoFormat&&);
  SdpVideoFormat& operator=(const SdpVideoFormat&);
  SdpVideoFormat& operator=(SdpVideoFormat&&);

  ~SdpVideoFormat();

  // Returns true if the SdpVideoFormats have the same names as well as codec
  // specific parameters. Please note that two SdpVideoFormats can represent the
  // same codec even though not all parameters are the same.
  bool IsSameCodec(const SdpVideoFormat& other) const;
  bool IsCodecInList(ArrayView<const webrtc::SdpVideoFormat> formats) const;

  std::string ToString() const;

  friend RTC_EXPORT bool operator==(const SdpVideoFormat& a,
                                    const SdpVideoFormat& b);
  friend RTC_EXPORT bool operator!=(const SdpVideoFormat& a,
                                    const SdpVideoFormat& b) {
    return !(a == b);
  }

  std::string name;
  CodecParameterMap parameters;
  absl::InlinedVector<ScalabilityMode, kScalabilityModeCount> scalability_modes;

  // Well-known video codecs and their format parameters.
  static const SdpVideoFormat VP8();
  static const SdpVideoFormat H264();
  static const SdpVideoFormat H265();
  static const SdpVideoFormat VP9Profile0();
  static const SdpVideoFormat VP9Profile1();
  static const SdpVideoFormat VP9Profile2();
  static const SdpVideoFormat VP9Profile3();
  static const SdpVideoFormat AV1Profile0();
  static const SdpVideoFormat AV1Profile1();

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const SdpVideoFormat& format) {
    sink.Append(format.ToString());
  }
};

// For not so good reasons sometimes additional parameters are added to an
// SdpVideoFormat, which makes instances that should compare equal to not match
// anymore. Until we stop misusing SdpVideoFormats provide this convenience
// function to perform fuzzy matching.
std::optional<SdpVideoFormat> FuzzyMatchSdpVideoFormat(
    ArrayView<const SdpVideoFormat> supported_formats,
    const SdpVideoFormat& format);

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_SDP_VIDEO_FORMAT_H_
