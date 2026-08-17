// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_WEBRTC_UMA_HISTOGRAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_WEBRTC_UMA_HISTOGRAMS_H_

#include <array>

#include "base/memory/singleton.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/public/common/mediastream/media_stream_request.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-shared.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_api_name.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

PLATFORM_EXPORT void LogUserMediaRequestResult(
    mojom::MediaStreamRequestResult result);

// Helper method used to collect information about the number of times
// different WebRTC APIs are called from JavaScript.
//
// This contributes to two histograms; the former is a raw count of
// the number of times the APIs are called, and be viewed at
// chrome://histograms/WebRTC.webkitApiCount.
//
// The latter is a count of the number of times the APIs are called
// that gets incremented only once per "session" as established by the
// PerSessionWebRTCAPIMetrics singleton below. It can be viewed at
// chrome://histograms/WebRTC.webkitApiCountPerSession.
PLATFORM_EXPORT void UpdateWebRTCMethodCount(RTCAPIName api_name);

// A singleton that keeps track of the number of MediaStreams being
// sent over PeerConnections. It uses the transition to zero such
// streams to demarcate the start of a new "session". Note that this
// is a rough approximation of sessions, as you could conceivably have
// multiple tabs using this renderer process, and each of them using
// PeerConnections.
//
// The UpdateWebRTCMethodCount function above uses this class to log a
// metric at most once per session.
class PLATFORM_EXPORT PerSessionWebRTCAPIMetrics {
 public:
  PerSessionWebRTCAPIMetrics(const PerSessionWebRTCAPIMetrics&) = delete;
  PerSessionWebRTCAPIMetrics& operator=(const PerSessionWebRTCAPIMetrics&) =
      delete;
  virtual ~PerSessionWebRTCAPIMetrics();

  static PerSessionWebRTCAPIMetrics* GetInstance();

  // Increment/decrement the number of streams being sent or received
  // over any current PeerConnection.
  void IncrementStreamCounter();
  void DecrementStreamCounter();

 protected:
  friend struct base::DefaultSingletonTraits<PerSessionWebRTCAPIMetrics>;
  friend PLATFORM_EXPORT void UpdateWebRTCMethodCount(RTCAPIName);

  // Protected so that unit tests can test without this being a
  // singleton.
  PerSessionWebRTCAPIMetrics();

  // Overridable by unit tests.
  virtual void LogUsage(RTCAPIName api_name);

  // Called by UpdateWebRTCMethodCount above. Protected rather than
  // private so that unit tests can call it.
  void LogUsageOnlyOnce(RTCAPIName api_name);

 private:
  void ResetUsage();

  int num_streams_;
  std::array<bool, static_cast<int>(RTCAPIName::kInvalidName)> has_used_api_;

  THREAD_CHECKER(thread_checker_);
};

}  //  namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_WEBRTC_UMA_HISTOGRAMS_H_
