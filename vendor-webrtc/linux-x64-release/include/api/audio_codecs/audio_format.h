/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_AUDIO_FORMAT_H_
#define API_AUDIO_CODECS_AUDIO_FORMAT_H_

#include <stddef.h>

#include <map>
#include <string>

#include "absl/strings/string_view.h"
#include "api/rtp_parameters.h"
#include "rtc_base/checks.h"
#include "rtc_base/strings/string_builder.h"
#include "rtc_base/system/rtc_export.h"  // IWYU pragma: private

namespace webrtc {

// SDP specification for a single audio codec.
struct RTC_EXPORT SdpAudioFormat {
  using Parameters [[deprecated("Use webrtc::CodecParameterMap")]] =
      std::map<std::string, std::string>;

  SdpAudioFormat(const SdpAudioFormat&);
  SdpAudioFormat(SdpAudioFormat&&);
  SdpAudioFormat(absl::string_view name, int clockrate_hz, size_t num_channels);
  SdpAudioFormat(absl::string_view name,
                 int clockrate_hz,
                 size_t num_channels,
                 const CodecParameterMap& param);
  SdpAudioFormat(absl::string_view name,
                 int clockrate_hz,
                 size_t num_channels,
                 CodecParameterMap&& param);
  ~SdpAudioFormat();

  // Returns true if this format is compatible with `o`. In SDP terminology:
  // would it represent the same codec between an offer and an answer? As
  // opposed to operator==, this method disregards codec parameters.
  bool Matches(const SdpAudioFormat& o) const;

  SdpAudioFormat& operator=(const SdpAudioFormat&);
  SdpAudioFormat& operator=(SdpAudioFormat&&);

  friend bool operator==(const SdpAudioFormat& a, const SdpAudioFormat& b);
  friend bool operator!=(const SdpAudioFormat& a, const SdpAudioFormat& b) {
    return !(a == b);
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const SdpAudioFormat& saf) {
    StringBuilder sb("{");
    bool first = true;
    for (const auto& [key, value] : saf.parameters) {
      if (!first) {
        sb << ", ";
      }
      first = false;
      sb << key << ": " << value;
    }
    sb << "}";
    absl::Format(
        &sink, "{name: %s, clockrate_hz: %d, num_channels: %d, parameters: %v}",
        saf.name, saf.clockrate_hz, saf.num_channels, sb.Release());
  }

  std::string name;
  int clockrate_hz;
  size_t num_channels;
  CodecParameterMap parameters;
};

// Information about how an audio format is treated by the codec implementation.
// Contains basic information, such as sample rate and number of channels, which
// isn't uniformly presented by SDP. Also contains flags indicating support for
// integrating with other parts of WebRTC, like external VAD and comfort noise
// level calculation.
//
// To avoid API breakage, and make the code clearer, AudioCodecInfo should not
// be directly initializable with any flags indicating optional support. If it
// were, these initializers would break any time a new flag was added. It's also
// more difficult to understand:
//   AudioCodecInfo info{16000, 1, 32000, true, false, false, true, true};
// than
//   AudioCodecInfo info(16000, 1, 32000);
//   info.allow_comfort_noise = true;
//   info.future_flag_b = true;
//   info.future_flag_c = true;
struct AudioCodecInfo {
  AudioCodecInfo(int sample_rate_hz, size_t num_channels, int bitrate_bps);
  AudioCodecInfo(int sample_rate_hz,
                 size_t num_channels,
                 int default_bitrate_bps,
                 int min_bitrate_bps,
                 int max_bitrate_bps);
  AudioCodecInfo(const AudioCodecInfo& b) = default;
  ~AudioCodecInfo() = default;

  bool operator==(const AudioCodecInfo& b) const {
    return sample_rate_hz == b.sample_rate_hz &&
           num_channels == b.num_channels &&
           default_bitrate_bps == b.default_bitrate_bps &&
           min_bitrate_bps == b.min_bitrate_bps &&
           max_bitrate_bps == b.max_bitrate_bps &&
           allow_comfort_noise == b.allow_comfort_noise &&
           supports_network_adaption == b.supports_network_adaption;
  }

  bool operator!=(const AudioCodecInfo& b) const { return !(*this == b); }

  bool HasFixedBitrate() const {
    RTC_DCHECK_GE(min_bitrate_bps, 0);
    RTC_DCHECK_LE(min_bitrate_bps, default_bitrate_bps);
    RTC_DCHECK_GE(max_bitrate_bps, default_bitrate_bps);
    return min_bitrate_bps == max_bitrate_bps;
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const AudioCodecInfo& aci) {
    absl::Format(&sink,
                 "{sample_rate_hz: %d, num_channels: %d, default_bitrate_bps: "
                 "%d, min_bitrate_bps: %d, max_bitrate_bps: %d, "
                 "allow_comfort_noise: %v, supports_network_adaption: %v}",
                 aci.sample_rate_hz, aci.num_channels, aci.default_bitrate_bps,
                 aci.min_bitrate_bps, aci.max_bitrate_bps,
                 aci.allow_comfort_noise, aci.supports_network_adaption);
  }

  int sample_rate_hz;
  size_t num_channels;
  int default_bitrate_bps;
  int min_bitrate_bps;
  int max_bitrate_bps;

  bool allow_comfort_noise = true;  // This codec can be used with an external
                                    // comfort noise generator.
  bool supports_network_adaption = false;  // This codec can adapt to varying
                                           // network conditions.
};

// AudioCodecSpec ties an audio format to specific information about the codec
// and its implementation.
struct AudioCodecSpec {
  bool operator==(const AudioCodecSpec& b) const {
    return format == b.format && info == b.info;
  }

  bool operator!=(const AudioCodecSpec& b) const { return !(*this == b); }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const AudioCodecSpec& acs) {
    absl::Format(&sink, "{format: %v, info: %v}", acs.format, acs.info);
  }

  SdpAudioFormat format;
  AudioCodecInfo info;
};

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_AUDIO_FORMAT_H_
