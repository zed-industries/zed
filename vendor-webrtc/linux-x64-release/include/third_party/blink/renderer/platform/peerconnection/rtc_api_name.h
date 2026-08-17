// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_API_NAME_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_API_NAME_H_

namespace blink {

// Helper enum used for histogramming calls to WebRTC APIs from JavaScript.
// The entries are linked to UMA values in //tools/metrics/histograms/enums.xml
// and shouldn't be renumbered or removed.
enum class RTCAPIName {
  kGetUserMedia,
  kPeerConnection,
  kDeprecatedPeerConnection,
  kRTCPeerConnection,
  kEnumerateDevices,
  kMediaStreamRecorder,
  kCanvasCaptureStream,
  kVideoCaptureStream,
  kGetDisplayMedia,
  kGetCurrentBrowsingContextMedia,  // Deprecated.
  kGetDisplayMediaSet,              // Deprecated.
  kGetAllScreensMedia,
  kSetPreferredSinkId,
  kInvalidName
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_API_NAME_H_
