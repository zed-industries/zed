// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_REMOTEPLAYBACK_REMOTE_PLAYBACK_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_REMOTEPLAYBACK_REMOTE_PLAYBACK_METRICS_H_

namespace blink {
class ExecutionContext;

// Do not remove or renumber enums as this is used for metrics. When making
// changes, also update the enum list in tools/metrics/histograms/enums.xml to
// keep it in sync.
enum class RemotePlaybackInitiationLocation {
  kRemovePlaybackAPI = 0,
  kHTMLMediaElement = 1,
  kMaxValue = kHTMLMediaElement,
};

class RemotePlaybackMetrics {
 public:
  static void RecordRemotePlaybackLocation(
      RemotePlaybackInitiationLocation location);

  static void RecordRemotePlaybackStartSessionResult(
      ExecutionContext* execution_context,
      bool success);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_REMOTEPLAYBACK_REMOTE_PLAYBACK_METRICS_H_
