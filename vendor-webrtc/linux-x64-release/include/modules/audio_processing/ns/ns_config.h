/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_NS_CONFIG_H_
#define MODULES_AUDIO_PROCESSING_NS_NS_CONFIG_H_

namespace webrtc {

// Config struct for the noise suppressor
struct NsConfig {
  enum class SuppressionLevel { k6dB, k12dB, k18dB, k21dB };
  SuppressionLevel target_level = SuppressionLevel::k12dB;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_NS_CONFIG_H_
