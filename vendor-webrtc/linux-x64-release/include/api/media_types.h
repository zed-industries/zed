/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_MEDIA_TYPES_H_
#define API_MEDIA_TYPES_H_

#include <string>

#include "absl/base/macros.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

enum class MediaType {
  AUDIO,
  VIDEO,
  DATA,
  UNSUPPORTED,
  ANY,
  // Backwards compatibility values for webrtc::MediaType users
  // TODO: https://issues.webrtc.org/42222911 - remove
  MEDIA_TYPE_AUDIO [[deprecated("Use AUDIO")]] = AUDIO,
  MEDIA_TYPE_VIDEO [[deprecated("Use VIDEO")]] = VIDEO,
  MEDIA_TYPE_DATA [[deprecated("Use DATA")]] = DATA,
  MEDIA_TYPE_UNSUPPORTED [[deprecated("Use UNSUPPORTED")]] = UNSUPPORTED,
};

RTC_EXPORT std::string MediaTypeToString(MediaType type);

template <typename Sink>
void AbslStringify(Sink& sink, MediaType type) {
  sink.Append(MediaTypeToString(type));
}

extern const char kMediaTypeAudio[];
extern const char kMediaTypeVideo[];
extern const char kMediaTypeData[];

}  // namespace webrtc

// The cricket and webrtc have separate definitions for what a media type is.
// They used to be incompatible, but now cricket is defined in terms of the
// webrtc definition.

#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {

using MediaType ABSL_DEPRECATE_AND_INLINE() = webrtc::MediaType;
using webrtc::kMediaTypeAudio;
using webrtc::kMediaTypeData;
using webrtc::kMediaTypeVideo;
using webrtc::MediaTypeToString;

// Backwards compatibility values for cricket::MediaType users
// TODO: https://issues.webrtc.org/42222911 - remove
[[deprecated]] constexpr MediaType MEDIA_TYPE_AUDIO = webrtc::MediaType::AUDIO;
[[deprecated]] constexpr MediaType MEDIA_TYPE_VIDEO = webrtc::MediaType::VIDEO;
[[deprecated]] constexpr MediaType MEDIA_TYPE_DATA = webrtc::MediaType::DATA;
[[deprecated]] constexpr MediaType MEDIA_TYPE_UNSUPPORTED =
    webrtc::MediaType::UNSUPPORTED;

}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // API_MEDIA_TYPES_H_
