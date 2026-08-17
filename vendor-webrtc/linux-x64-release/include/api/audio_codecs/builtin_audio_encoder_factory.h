/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_BUILTIN_AUDIO_ENCODER_FACTORY_H_
#define API_AUDIO_CODECS_BUILTIN_AUDIO_ENCODER_FACTORY_H_

#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/scoped_refptr.h"

namespace webrtc {

// Creates a new factory that can create the built-in types of audio encoders.
// Note: This will link with all the code implementing those codecs, so if you
// only need a subset of the codecs, consider using
// CreateAudioEncoderFactory<...codecs listed here...>() or
// CreateOpusAudioEncoderFactory() instead.
scoped_refptr<AudioEncoderFactory> CreateBuiltinAudioEncoderFactory();

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_BUILTIN_AUDIO_ENCODER_FACTORY_H_
