/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_GAIN_MAP_INTERNAL_H_
#define MODULES_AUDIO_PROCESSING_AGC2_GAIN_MAP_INTERNAL_H_

namespace webrtc {

static constexpr int kGainMapSize = 256;
// Maps input volumes, which are values in the [0, 255] range, to gains in dB.
// The values below are generated with numpy as follows:
// SI = 2                        # Initial slope.
// SF = 0.25                     # Final slope.
// D = 8/256                     # Quantization factor.
// x = np.linspace(0, 255, 256)  # Input volumes.
// y = (SF * x + (SI - SF) * (1 - np.exp(-D*x)) / D - 56).round()
static const int kGainMap[kGainMapSize] = {
    -56, -54, -52, -50, -48, -47, -45, -43, -42, -40, -38, -37, -35, -34, -33,
    -31, -30, -29, -27, -26, -25, -24, -23, -22, -20, -19, -18, -17, -16, -15,
    -14, -14, -13, -12, -11, -10, -9,  -8,  -8,  -7,  -6,  -5,  -5,  -4,  -3,
    -2,  -2,  -1,  0,   0,   1,   1,   2,   3,   3,   4,   4,   5,   5,   6,
    6,   7,   7,   8,   8,   9,   9,   10,  10,  11,  11,  12,  12,  13,  13,
    13,  14,  14,  15,  15,  15,  16,  16,  17,  17,  17,  18,  18,  18,  19,
    19,  19,  20,  20,  21,  21,  21,  22,  22,  22,  23,  23,  23,  24,  24,
    24,  24,  25,  25,  25,  26,  26,  26,  27,  27,  27,  28,  28,  28,  28,
    29,  29,  29,  30,  30,  30,  30,  31,  31,  31,  32,  32,  32,  32,  33,
    33,  33,  33,  34,  34,  34,  35,  35,  35,  35,  36,  36,  36,  36,  37,
    37,  37,  38,  38,  38,  38,  39,  39,  39,  39,  40,  40,  40,  40,  41,
    41,  41,  41,  42,  42,  42,  42,  43,  43,  43,  44,  44,  44,  44,  45,
    45,  45,  45,  46,  46,  46,  46,  47,  47,  47,  47,  48,  48,  48,  48,
    49,  49,  49,  49,  50,  50,  50,  50,  51,  51,  51,  51,  52,  52,  52,
    52,  53,  53,  53,  53,  54,  54,  54,  54,  55,  55,  55,  55,  56,  56,
    56,  56,  57,  57,  57,  57,  58,  58,  58,  58,  59,  59,  59,  59,  60,
    60,  60,  60,  61,  61,  61,  61,  62,  62,  62,  62,  63,  63,  63,  63,
    64};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_GAIN_MAP_INTERNAL_H_
