// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_FALLBACK_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_FALLBACK_RECORDER_H_

#include "media/base/video_codecs.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class RTCVideoDecoderFallbackReason {
  kSpatialLayers = 0,
  kConsecutivePendingBufferOverflow = 1,
  kReinitializationFailed = 2,
  kPreviousErrorOnDecode = 3,
  kPreviousErrorOnRegisterCallback = 4,
  kConsecutivePendingBufferOverflowDuringInit = 5,
  kParseErrorOnResolutionCheck = 6,
  kTooManyInstancesAndSmallResolution = 7,
  kMaxValue = kTooManyInstancesAndSmallResolution,
};

void RecordRTCVideoDecoderFallbackReason(
    media::VideoCodec codec,
    RTCVideoDecoderFallbackReason fallback_reason);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_VIDEO_DECODER_FALLBACK_RECORDER_H_
