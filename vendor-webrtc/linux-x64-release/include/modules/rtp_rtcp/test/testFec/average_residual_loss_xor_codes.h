/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_TEST_TESTFEC_AVERAGE_RESIDUAL_LOSS_XOR_CODES_H_
#define MODULES_RTP_RTCP_TEST_TESTFEC_AVERAGE_RESIDUAL_LOSS_XOR_CODES_H_

namespace webrtc {

// Maximum number of media packets allowed in this test. The burst mask types
// are currently defined up to (kMaxMediaPacketsTest, kMaxMediaPacketsTest).
const int kMaxMediaPacketsTest = 12;

// Maximum number of FEC codes considered in this test.
const int kNumberCodes = kMaxMediaPacketsTest * (kMaxMediaPacketsTest + 1) / 2;

// For the random mask type: reference level for the maximum average residual
// loss expected for each code size up to:
// (kMaxMediaPacketsTest, kMaxMediaPacketsTest).
const float kMaxResidualLossRandomMask[kNumberCodes] = {
    0.009463f, 0.022436f, 0.007376f, 0.033895f, 0.012423f, 0.004644f, 0.043438f,
    0.019937f, 0.008820f, 0.003438f, 0.051282f, 0.025795f, 0.012872f, 0.006458f,
    0.003195f, 0.057728f, 0.032146f, 0.016708f, 0.009242f, 0.005054f, 0.003078f,
    0.063050f, 0.037261f, 0.021767f, 0.012447f, 0.007099f, 0.003826f, 0.002504f,
    0.067476f, 0.042348f, 0.026169f, 0.015695f, 0.009478f, 0.005887f, 0.003568f,
    0.001689f, 0.071187f, 0.046575f, 0.031697f, 0.019797f, 0.012433f, 0.007027f,
    0.004845f, 0.002777f, 0.001753f, 0.074326f, 0.050628f, 0.034978f, 0.021955f,
    0.014821f, 0.009462f, 0.006393f, 0.004181f, 0.003105f, 0.001231f, 0.077008f,
    0.054226f, 0.038407f, 0.026251f, 0.018634f, 0.011568f, 0.008130f, 0.004957f,
    0.003334f, 0.002069f, 0.001304f, 0.079318f, 0.057180f, 0.041268f, 0.028842f,
    0.020033f, 0.014061f, 0.009636f, 0.006411f, 0.004583f, 0.002817f, 0.001770f,
    0.001258f};

// For the bursty mask type: reference level for the maximum average residual
// loss expected for each code size up to:
// (kMaxMediaPacketsTestf, kMaxMediaPacketsTest).
const float kMaxResidualLossBurstyMask[kNumberCodes] = {
    0.033236f, 0.053344f, 0.026616f, 0.064129f, 0.036589f, 0.021892f, 0.071055f,
    0.043890f, 0.028009f, 0.018524f, 0.075968f, 0.049828f, 0.033288f, 0.022791f,
    0.016088f, 0.079672f, 0.054586f, 0.037872f, 0.026679f, 0.019326f, 0.014293f,
    0.082582f, 0.058719f, 0.042045f, 0.030504f, 0.022391f, 0.016894f, 0.012946f,
    0.084935f, 0.062169f, 0.045620f, 0.033713f, 0.025570f, 0.019439f, 0.015121f,
    0.011920f, 0.086881f, 0.065267f, 0.048721f, 0.037613f, 0.028278f, 0.022152f,
    0.017314f, 0.013791f, 0.011130f, 0.088516f, 0.067911f, 0.051709f, 0.040819f,
    0.030777f, 0.024547f, 0.019689f, 0.015877f, 0.012773f, 0.010516f, 0.089909f,
    0.070332f, 0.054402f, 0.043210f, 0.034096f, 0.026625f, 0.021823f, 0.017648f,
    0.014649f, 0.011982f, 0.010035f, 0.091109f, 0.072428f, 0.056775f, 0.045418f,
    0.036679f, 0.028599f, 0.023693f, 0.019966f, 0.016603f, 0.013690f, 0.011359f,
    0.009657f};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_TEST_TESTFEC_AVERAGE_RESIDUAL_LOSS_XOR_CODES_H_
