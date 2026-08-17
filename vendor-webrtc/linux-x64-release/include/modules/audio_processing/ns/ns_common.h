/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_NS_COMMON_H_
#define MODULES_AUDIO_PROCESSING_NS_NS_COMMON_H_

#include <cstddef>

namespace webrtc {

constexpr size_t kFftSize = 256;
constexpr size_t kFftSizeBy2Plus1 = kFftSize / 2 + 1;
constexpr size_t kNsFrameSize = 160;
constexpr size_t kOverlapSize = kFftSize - kNsFrameSize;

constexpr int kShortStartupPhaseBlocks = 50;
constexpr int kLongStartupPhaseBlocks = 200;
constexpr int kFeatureUpdateWindowSize = 500;

constexpr float kLtrFeatureThr = 0.5f;
constexpr float kBinSizeLrt = 0.1f;
constexpr float kBinSizeSpecFlat = 0.05f;
constexpr float kBinSizeSpecDiff = 0.1f;

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_NS_COMMON_H_
