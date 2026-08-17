/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_ECHO_DETECTOR_CREATOR_H_
#define API_AUDIO_ECHO_DETECTOR_CREATOR_H_

#include "api/audio/audio_processing.h"
#include "api/scoped_refptr.h"

namespace webrtc {

// Returns an instance of the WebRTC implementation of a residual echo detector.
// It can be provided to the webrtc::BuiltinAudioProcessingBuilder to obtain the
// usual residual echo metrics.
scoped_refptr<EchoDetector> CreateEchoDetector();

}  // namespace webrtc

#endif  // API_AUDIO_ECHO_DETECTOR_CREATOR_H_
