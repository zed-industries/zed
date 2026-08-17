/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_UTILITY_OOURA_FFT_TABLES_COMMON_H_
#define MODULES_AUDIO_PROCESSING_UTILITY_OOURA_FFT_TABLES_COMMON_H_

#include "common_audio/third_party/ooura/fft_size_128/ooura_fft.h"

namespace webrtc {

// This tables used to be computed at run-time. For example, refer to:
// https://code.google.com/p/webrtc/source/browse/trunk/webrtc/modules/audio_processing/utility/apm_rdft.c?r=6564
// to see the initialization code.
// Constants shared by all paths (C, SSE2, NEON).
const float rdft_w[64] = {
    1.0000000000f, 0.0000000000f, 0.7071067691f, 0.7071067691f, 0.9238795638f,
    0.3826834559f, 0.3826834559f, 0.9238795638f, 0.9807852507f, 0.1950903237f,
    0.5555702448f, 0.8314695954f, 0.8314695954f, 0.5555702448f, 0.1950903237f,
    0.9807852507f, 0.9951847196f, 0.0980171412f, 0.6343933344f, 0.7730104327f,
    0.8819212914f, 0.4713967443f, 0.2902846634f, 0.9569403529f, 0.9569403529f,
    0.2902846634f, 0.4713967443f, 0.8819212914f, 0.7730104327f, 0.6343933344f,
    0.0980171412f, 0.9951847196f, 0.7071067691f, 0.4993977249f, 0.4975923598f,
    0.4945882559f, 0.4903926253f, 0.4850156307f, 0.4784701765f, 0.4707720280f,
    0.4619397819f, 0.4519946277f, 0.4409606457f, 0.4288643003f, 0.4157347977f,
    0.4016037583f, 0.3865052164f, 0.3704755902f, 0.3535533845f, 0.3357794881f,
    0.3171966672f, 0.2978496552f, 0.2777851224f, 0.2570513785f, 0.2356983721f,
    0.2137775421f, 0.1913417280f, 0.1684449315f, 0.1451423317f, 0.1214900985f,
    0.0975451618f, 0.0733652338f, 0.0490085706f, 0.0245338380f,
};

// Constants used by the C and MIPS paths.
const float rdft_wk3ri_first[16] = {
    1.000000000f, 0.000000000f, 0.382683456f,  0.923879564f,
    0.831469536f, 0.555570245f, -0.195090353f, 0.980785251f,
    0.956940353f, 0.290284693f, 0.098017156f,  0.995184720f,
    0.634393334f, 0.773010492f, -0.471396863f, 0.881921172f,
};
const float rdft_wk3ri_second[16] = {
    -0.707106769f, 0.707106769f,  -0.923879564f, -0.382683456f,
    -0.980785251f, 0.195090353f,  -0.555570245f, -0.831469536f,
    -0.881921172f, 0.471396863f,  -0.773010492f, -0.634393334f,
    -0.995184720f, -0.098017156f, -0.290284693f, -0.956940353f,
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_UTILITY_OOURA_FFT_TABLES_COMMON_H_
