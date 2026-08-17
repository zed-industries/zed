// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_METRICS_H_

namespace blink {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class BreakoutBoxUsage {
  kReadableVideo = 0,
  kReadableVideoWorker = 1,
  kReadableAudio = 2,
  kReadableAudioWorker = 3,
  kWritableVideo = 4,
  kWritableVideoWorker = 5,
  kWritableAudio = 6,
  kWritableAudioWorker = 7,
  kReadableControlVideo = 8,
  kWritableControlVideo = 9,
  kMaxValue = kWritableControlVideo,
};

void RecordBreakoutBoxUsage(BreakoutBoxUsage);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_METRICS_H_
