/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_ENGINE_WEBRTC_MEDIA_ENGINE_H_
#define MEDIA_ENGINE_WEBRTC_MEDIA_ENGINE_H_

#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/field_trials_view.h"
#include "api/rtp_parameters.h"
#include "api/transport/bitrate_settings.h"
#include "media/base/codec.h"

namespace webrtc {

// Verify that extension IDs are within 1-byte extension range and are not
// overlapping, and that they form a legal change from previously registerd
// extensions (if any).
bool ValidateRtpExtensions(ArrayView<const RtpExtension> extennsions,
                           ArrayView<const RtpExtension> old_extensions);

// Discard any extensions not validated by the 'supported' predicate. Duplicate
// extensions are removed if 'filter_redundant_extensions' is set, and also any
// mutually exclusive extensions (see implementation for details) are removed.
std::vector<RtpExtension> FilterRtpExtensions(
    const std::vector<RtpExtension>& extensions,
    bool (*supported)(absl::string_view),
    bool filter_redundant_extensions,
    const FieldTrialsView& trials);

BitrateConstraints GetBitrateConfigForCodec(const Codec& codec);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FilterRtpExtensions;
using ::webrtc::GetBitrateConfigForCodec;
using ::webrtc::ValidateRtpExtensions;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_ENGINE_WEBRTC_MEDIA_ENGINE_H_
