/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC_LEGACY_GAIN_CONTROL_H_
#define MODULES_AUDIO_PROCESSING_AGC_LEGACY_GAIN_CONTROL_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

enum {
  kAgcModeUnchanged,
  kAgcModeAdaptiveAnalog,
  kAgcModeAdaptiveDigital,
  kAgcModeFixedDigital
};

enum { kAgcFalse = 0, kAgcTrue };

typedef struct {
  int16_t targetLevelDbfs;    // default 3 (-3 dBOv)
  int16_t compressionGaindB;  // default 9 dB
  uint8_t limiterEnable;      // default kAgcTrue (on)
} WebRtcAgcConfig;

/*
 * This function analyses the number of samples passed to
 * farend and produces any error code that could arise.
 *
 * Input:
 *      - agcInst           : AGC instance.
 *      - samples           : Number of samples in input vector.
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error.
 */
int WebRtcAgc_GetAddFarendError(void* state, size_t samples);

/*
 * This function processes a 10 ms frame of far-end speech to determine
 * if there is active speech. The length of the input speech vector must be
 * given in samples (80 when FS=8000, and 160 when FS=16000, FS=32000 or
 * FS=48000).
 *
 * Input:
 *      - agcInst           : AGC instance.
 *      - inFar             : Far-end input speech vector
 *      - samples           : Number of samples in input vector
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_AddFarend(void* agcInst, const int16_t* inFar, size_t samples);

/*
 * This function processes a 10 ms frame of microphone speech to determine
 * if there is active speech. The length of the input speech vector must be
 * given in samples (80 when FS=8000, and 160 when FS=16000, FS=32000 or
 * FS=48000). For very low input levels, the input signal is increased in level
 * by multiplying and overwriting the samples in inMic[].
 *
 * This function should be called before any further processing of the
 * near-end microphone signal.
 *
 * Input:
 *      - agcInst           : AGC instance.
 *      - inMic             : Microphone input speech vector for each band
 *      - num_bands         : Number of bands in input vector
 *      - samples           : Number of samples in input vector
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_AddMic(void* agcInst,
                     int16_t* const* inMic,
                     size_t num_bands,
                     size_t samples);

/*
 * This function replaces the analog microphone with a virtual one.
 * It is a digital gain applied to the input signal and is used in the
 * agcAdaptiveDigital mode where no microphone level is adjustable. The length
 * of the input speech vector must be given in samples (80 when FS=8000, and 160
 * when FS=16000, FS=32000 or FS=48000).
 *
 * Input:
 *      - agcInst           : AGC instance.
 *      - inMic             : Microphone input speech vector for each band
 *      - num_bands         : Number of bands in input vector
 *      - samples           : Number of samples in input vector
 *      - micLevelIn        : Input level of microphone (static)
 *
 * Output:
 *      - inMic             : Microphone output after processing (L band)
 *      - inMic_H           : Microphone output after processing (H band)
 *      - micLevelOut       : Adjusted microphone level after processing
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_VirtualMic(void* agcInst,
                         int16_t* const* inMic,
                         size_t num_bands,
                         size_t samples,
                         int32_t micLevelIn,
                         int32_t* micLevelOut);

/*
 * This function analyses a 10 ms frame and produces the analog and digital
 * gains required to normalize the signal. The gain adjustments are done only
 * during active periods of speech. The length of the speech vectors must be
 * given in samples (80 when FS=8000, and 160 when FS=16000, FS=32000 or
 * FS=48000). The echo parameter can be used to ensure the AGC will not adjust
 * upward in the presence of echo.
 *
 * This function should be called after processing the near-end microphone
 * signal, in any case after any echo cancellation.
 *
 * Input:
 *      - agcInst           : AGC instance
 *      - inNear            : Near-end input speech vector for each band
 *      - num_bands         : Number of bands in input/output vector
 *      - samples           : Number of samples in input/output vector
 *      - inMicLevel        : Current microphone volume level
 *      - echo              : Set to 0 if the signal passed to add_mic is
 *                            almost certainly free of echo; otherwise set
 *                            to 1. If you have no information regarding echo
 *                            set to 0.
 *
 * Output:
 *      - outMicLevel       : Adjusted microphone volume level
 *      - saturationWarning : A returned value of 1 indicates a saturation event
 *                            has occurred and the volume cannot be further
 *                            reduced. Otherwise will be set to 0.
 *      - gains             : Vector of gains to apply for digital normalization
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_Analyze(void* agcInst,
                      const int16_t* const* inNear,
                      size_t num_bands,
                      size_t samples,
                      int32_t inMicLevel,
                      int32_t* outMicLevel,
                      int16_t echo,
                      uint8_t* saturationWarning,
                      int32_t gains[11]);

/*
 * This function processes a 10 ms frame by applying precomputed digital gains.
 *
 * Input:
 *      - agcInst           : AGC instance
 *      - gains             : Vector of gains to apply for digital normalization
 *      - in_near           : Near-end input speech vector for each band
 *      - num_bands         : Number of bands in input/output vector
 *
 * Output:
 *      - out               : Gain-adjusted near-end speech vector
 *                          : May be the same vector as the input.
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_Process(const void* agcInst,
                      const int32_t gains[11],
                      const int16_t* const* in_near,
                      size_t num_bands,
                      int16_t* const* out);

/*
 * This function sets the config parameters (targetLevelDbfs,
 * compressionGaindB and limiterEnable).
 *
 * Input:
 *      - agcInst           : AGC instance
 *      - config            : config struct
 *
 * Output:
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_set_config(void* agcInst, WebRtcAgcConfig config);

/*
 * This function returns the config parameters (targetLevelDbfs,
 * compressionGaindB and limiterEnable).
 *
 * Input:
 *      - agcInst           : AGC instance
 *
 * Output:
 *      - config            : config struct
 *
 * Return value:
 *                          :  0 - Normal operation.
 *                          : -1 - Error
 */
int WebRtcAgc_get_config(void* agcInst, WebRtcAgcConfig* config);

/*
 * This function creates and returns an AGC instance, which will contain the
 * state information for one (duplex) channel.
 */
void* WebRtcAgc_Create(void);

/*
 * This function frees the AGC instance created at the beginning.
 *
 * Input:
 *      - agcInst           : AGC instance.
 */
void WebRtcAgc_Free(void* agcInst);

/*
 * This function initializes an AGC instance.
 *
 * Input:
 *      - agcInst           : AGC instance.
 *      - minLevel          : Minimum possible mic level
 *      - maxLevel          : Maximum possible mic level
 *      - agcMode           : 0 - Unchanged
 *                          : 1 - Adaptive Analog Automatic Gain Control -3dBOv
 *                          : 2 - Adaptive Digital Automatic Gain Control -3dBOv
 *                          : 3 - Fixed Digital Gain 0dB
 *      - fs                : Sampling frequency
 *
 * Return value             :  0 - Ok
 *                            -1 - Error
 */
int WebRtcAgc_Init(void* agcInst,
                   int32_t minLevel,
                   int32_t maxLevel,
                   int16_t agcMode,
                   uint32_t fs);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC_LEGACY_GAIN_CONTROL_H_
