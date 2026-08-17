/*
 *  Copyright (c) 2004 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_TEST_UTILS_H_
#define MEDIA_BASE_TEST_UTILS_H_

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "media/base/stream_params.h"
#include "rtc_base/arraysize.h"

namespace webrtc {
class VideoFrame;
}

namespace webrtc {

// Returns size of 420 image with rounding on chroma for odd sizes.
#define I420_SIZE(w, h) (w * h + (((w + 1) / 2) * ((h + 1) / 2)) * 2)
// Returns size of ARGB image.
#define ARGB_SIZE(w, h) (w * h * 4)

template <class T>
inline std::vector<T> MakeVector(const T a[], size_t s) {
  return std::vector<T>(a, a + s);
}
#define MAKE_VECTOR(a) webrtc::MakeVector(a, arraysize(a))

// Create Simulcast StreamParams with given `ssrcs` and `cname`.
StreamParams CreateSimStreamParams(const std::string& cname,
                                   const std::vector<uint32_t>& ssrcs);
// Create Simulcast stream with given `ssrcs` and `rtx_ssrcs`.
// The number of `rtx_ssrcs` must match number of `ssrcs`.
StreamParams CreateSimWithRtxStreamParams(
    const std::string& cname,
    const std::vector<uint32_t>& ssrcs,
    const std::vector<uint32_t>& rtx_ssrcs);

// Create StreamParams with single primary SSRC and corresponding FlexFEC SSRC.
StreamParams CreatePrimaryWithFecFrStreamParams(const std::string& cname,
                                                uint32_t primary_ssrc,
                                                uint32_t flexfec_ssrc);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CreatePrimaryWithFecFrStreamParams;
using ::webrtc::CreateSimStreamParams;
using ::webrtc::CreateSimWithRtxStreamParams;
using ::webrtc::MakeVector;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_TEST_UTILS_H_
