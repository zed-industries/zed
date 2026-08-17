/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

/*
 * This header file includes the descriptions of the core VAD calls.
 */

#ifndef COMMON_AUDIO_VAD_VAD_CORE_H_
#define COMMON_AUDIO_VAD_VAD_CORE_H_

#include "common_audio/signal_processing/include/signal_processing_library.h"

// TODO(https://bugs.webrtc.org/14476): When converted to C++, remove the macro.
#if defined(__cplusplus)
#define CONSTEXPR_INT(x) constexpr int x
#else
#define CONSTEXPR_INT(x) enum { x }
#endif

CONSTEXPR_INT(kNumChannels = 6);  // Number of frequency bands (named channels).
CONSTEXPR_INT(
    kNumGaussians = 2);  // Number of Gaussians per channel in the GMM.
CONSTEXPR_INT(kTableSize = kNumChannels * kNumGaussians);
CONSTEXPR_INT(
    kMinEnergy = 10);  // Minimum energy required to trigger audio signal.

typedef struct VadInstT_ {
  int vad;
  int32_t downsampling_filter_states[4];
  WebRtcSpl_State48khzTo8khz state_48_to_8;
  int16_t noise_means[kTableSize];
  int16_t speech_means[kTableSize];
  int16_t noise_stds[kTableSize];
  int16_t speech_stds[kTableSize];
  // TODO(bjornv): Change to `frame_count`.
  int32_t frame_counter;
  int16_t over_hang;  // Over Hang
  int16_t num_of_speech;
  // TODO(bjornv): Change to `age_vector`.
  int16_t index_vector[16 * kNumChannels];
  int16_t low_value_vector[16 * kNumChannels];
  // TODO(bjornv): Change to `median`.
  int16_t mean_value[kNumChannels];
  int16_t upper_state[5];
  int16_t lower_state[5];
  int16_t hp_filter_state[4];
  int16_t over_hang_max_1[3];
  int16_t over_hang_max_2[3];
  int16_t individual[3];
  int16_t total[3];

  int init_flag;
} VadInstT;

// Initializes the core VAD component. The default aggressiveness mode is
// controlled by `kDefaultMode` in vad_core.c.
//
// - self [i/o] : Instance that should be initialized
//
// returns      : 0 (OK), -1 (null pointer in or if the default mode can't be
//                set)
int WebRtcVad_InitCore(VadInstT* self);

/****************************************************************************
 * WebRtcVad_set_mode_core(...)
 *
 * This function changes the VAD settings
 *
 * Input:
 *      - inst      : VAD instance
 *      - mode      : Aggressiveness degree
 *                    0 (High quality) - 3 (Highly aggressive)
 *
 * Output:
 *      - inst      : Changed  instance
 *
 * Return value     :  0 - Ok
 *                    -1 - Error
 */

int WebRtcVad_set_mode_core(VadInstT* self, int mode);

/****************************************************************************
 * WebRtcVad_CalcVad48khz(...)
 * WebRtcVad_CalcVad32khz(...)
 * WebRtcVad_CalcVad16khz(...)
 * WebRtcVad_CalcVad8khz(...)
 *
 * Calculate probability for active speech and make VAD decision.
 *
 * Input:
 *      - inst          : Instance that should be initialized
 *      - speech_frame  : Input speech frame
 *      - frame_length  : Number of input samples
 *
 * Output:
 *      - inst          : Updated filter states etc.
 *
 * Return value         : VAD decision
 *                        0 - No active speech
 *                        1-6 - Active speech
 */
int WebRtcVad_CalcVad48khz(VadInstT* inst,
                           const int16_t* speech_frame,
                           size_t frame_length);
int WebRtcVad_CalcVad32khz(VadInstT* inst,
                           const int16_t* speech_frame,
                           size_t frame_length);
int WebRtcVad_CalcVad16khz(VadInstT* inst,
                           const int16_t* speech_frame,
                           size_t frame_length);
int WebRtcVad_CalcVad8khz(VadInstT* inst,
                          const int16_t* speech_frame,
                          size_t frame_length);

#endif  // COMMON_AUDIO_VAD_VAD_CORE_H_
