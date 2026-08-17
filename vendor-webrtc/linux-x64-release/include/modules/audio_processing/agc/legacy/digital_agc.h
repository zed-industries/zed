/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_LEGACY_DIGITAL_AGC_H_
#define MODULES_AUDIO_PROCESSING_AGC_LEGACY_DIGITAL_AGC_H_

#include "common_audio/signal_processing/include/signal_processing_library.h"

namespace webrtc {

typedef struct {
  int32_t downState[8];
  int16_t HPstate;
  int16_t counter;
  int16_t logRatio;           // log( P(active) / P(inactive) ) (Q10)
  int16_t meanLongTerm;       // Q10
  int32_t varianceLongTerm;   // Q8
  int16_t stdLongTerm;        // Q10
  int16_t meanShortTerm;      // Q10
  int32_t varianceShortTerm;  // Q8
  int16_t stdShortTerm;       // Q10
} AgcVad;                     // total = 54 bytes

typedef struct {
  int32_t capacitorSlow;
  int32_t capacitorFast;
  int32_t gain;
  int32_t gainTable[32];
  int16_t gatePrevious;
  int16_t agcMode;
  AgcVad vadNearend;
  AgcVad vadFarend;
} DigitalAgc;

int32_t WebRtcAgc_InitDigital(DigitalAgc* digitalAgcInst, int16_t agcMode);

int32_t WebRtcAgc_ComputeDigitalGains(DigitalAgc* digitalAgcInst,
                                      const int16_t* const* inNear,
                                      size_t num_bands,
                                      uint32_t FS,
                                      int16_t lowLevelSignal,
                                      int32_t gains[11]);

int32_t WebRtcAgc_ApplyDigitalGains(const int32_t gains[11],
                                    size_t num_bands,
                                    uint32_t FS,
                                    const int16_t* const* in_near,
                                    int16_t* const* out);

int32_t WebRtcAgc_AddFarendToDigital(DigitalAgc* digitalAgcInst,
                                     const int16_t* inFar,
                                     size_t nrSamples);

void WebRtcAgc_InitVad(AgcVad* vadInst);

int16_t WebRtcAgc_ProcessVad(AgcVad* vadInst,    // (i) VAD state
                             const int16_t* in,  // (i) Speech signal
                             size_t nrSamples);  // (i) number of samples

int32_t WebRtcAgc_CalculateGainTable(int32_t* gainTable,         // Q16
                                     int16_t compressionGaindB,  // Q0 (in dB)
                                     int16_t targetLevelDbfs,    // Q0 (in dB)
                                     uint8_t limiterEnable,
                                     int16_t analogTarget);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_LEGACY_DIGITAL_AGC_H_
