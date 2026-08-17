/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Performs echo control (suppression) with fft routines in fixed-point.

#ifndef MODULES_AUDIO_PROCESSING_AECM_AECM_CORE_H_
#define MODULES_AUDIO_PROCESSING_AECM_AECM_CORE_H_

extern "C" {
#include "common_audio/ring_buffer.h"
#include "common_audio/signal_processing/include/signal_processing_library.h"
}
#include "modules/audio_processing/aecm/aecm_defines.h"

struct RealFFT;

namespace webrtc {

#ifdef _MSC_VER  // visual c++
#define ALIGN8_BEG __declspec(align(8))
#define ALIGN8_END
#else  // gcc or icc
#define ALIGN8_BEG
#define ALIGN8_END __attribute__((aligned(8)))
#endif

typedef struct {
  int16_t real;
  int16_t imag;
} ComplexInt16;

typedef struct {
  int farBufWritePos;
  int farBufReadPos;
  int knownDelay;
  int lastKnownDelay;
  int firstVAD;  // Parameter to control poorly initialized channels

  RingBuffer* farFrameBuf;
  RingBuffer* nearNoisyFrameBuf;
  RingBuffer* nearCleanFrameBuf;
  RingBuffer* outFrameBuf;

  int16_t farBuf[FAR_BUF_LEN];

  int16_t mult;
  uint32_t seed;

  // Delay estimation variables
  void* delay_estimator_farend;
  void* delay_estimator;
  uint16_t currentDelay;
  // Far end history variables
  // TODO(bjornv): Replace `far_history` with ring_buffer.
  uint16_t far_history[PART_LEN1 * MAX_DELAY];
  int far_history_pos;
  int far_q_domains[MAX_DELAY];

  int16_t nlpFlag;
  int16_t fixedDelay;

  uint32_t totCount;

  int16_t dfaCleanQDomain;
  int16_t dfaCleanQDomainOld;
  int16_t dfaNoisyQDomain;
  int16_t dfaNoisyQDomainOld;

  int16_t nearLogEnergy[MAX_BUF_LEN];
  int16_t farLogEnergy;
  int16_t echoAdaptLogEnergy[MAX_BUF_LEN];
  int16_t echoStoredLogEnergy[MAX_BUF_LEN];

  // The extra 16 or 32 bytes in the following buffers are for alignment based
  // Neon code.
  // It's designed this way since the current GCC compiler can't align a
  // buffer in 16 or 32 byte boundaries properly.
  int16_t channelStored_buf[PART_LEN1 + 8];
  int16_t channelAdapt16_buf[PART_LEN1 + 8];
  int32_t channelAdapt32_buf[PART_LEN1 + 8];
  int16_t xBuf_buf[PART_LEN2 + 16];       // farend
  int16_t dBufClean_buf[PART_LEN2 + 16];  // nearend
  int16_t dBufNoisy_buf[PART_LEN2 + 16];  // nearend
  int16_t outBuf_buf[PART_LEN + 8];

  // Pointers to the above buffers
  int16_t* channelStored;
  int16_t* channelAdapt16;
  int32_t* channelAdapt32;
  int16_t* xBuf;
  int16_t* dBufClean;
  int16_t* dBufNoisy;
  int16_t* outBuf;

  int32_t echoFilt[PART_LEN1];
  int16_t nearFilt[PART_LEN1];
  int32_t noiseEst[PART_LEN1];
  int noiseEstTooLowCtr[PART_LEN1];
  int noiseEstTooHighCtr[PART_LEN1];
  int16_t noiseEstCtr;
  int16_t cngMode;

  int32_t mseAdaptOld;
  int32_t mseStoredOld;
  int32_t mseThreshold;

  int16_t farEnergyMin;
  int16_t farEnergyMax;
  int16_t farEnergyMaxMin;
  int16_t farEnergyVAD;
  int16_t farEnergyMSE;
  int currentVADValue;
  int16_t vadUpdateCount;

  int16_t startupState;
  int16_t mseChannelCount;
  int16_t supGain;
  int16_t supGainOld;

  int16_t supGainErrParamA;
  int16_t supGainErrParamD;
  int16_t supGainErrParamDiffAB;
  int16_t supGainErrParamDiffBD;

  struct RealFFT* real_fft;

#ifdef AEC_DEBUG
  FILE* farFile;
  FILE* nearFile;
  FILE* outFile;
#endif
} AecmCore;

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_CreateCore()
//
// Allocates the memory needed by the AECM. The memory needs to be
// initialized separately using the WebRtcAecm_InitCore() function.
// Returns a pointer to the instance and a nullptr at failure.
AecmCore* WebRtcAecm_CreateCore();

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_InitCore(...)
//
// This function initializes the AECM instant created with
// WebRtcAecm_CreateCore()
// Input:
//      - aecm          : Pointer to the AECM instance
//      - samplingFreq  : Sampling Frequency
//
// Output:
//      - aecm          : Initialized instance
//
// Return value         :  0 - Ok
//                        -1 - Error
//
int WebRtcAecm_InitCore(AecmCore* const aecm, int samplingFreq);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_FreeCore(...)
//
// This function releases the memory allocated by WebRtcAecm_CreateCore()
// Input:
//      - aecm          : Pointer to the AECM instance
//
void WebRtcAecm_FreeCore(AecmCore* aecm);

int WebRtcAecm_Control(AecmCore* aecm, int delay, int nlpFlag);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_InitEchoPathCore(...)
//
// This function resets the echo channel adaptation with the specified channel.
// Input:
//      - aecm          : Pointer to the AECM instance
//      - echo_path     : Pointer to the data that should initialize the echo
//                        path
//
// Output:
//      - aecm          : Initialized instance
//
void WebRtcAecm_InitEchoPathCore(AecmCore* aecm, const int16_t* echo_path);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_ProcessFrame(...)
//
// This function processes frames and sends blocks to
// WebRtcAecm_ProcessBlock(...)
//
// Inputs:
//      - aecm          : Pointer to the AECM instance
//      - farend        : In buffer containing one frame of echo signal
//      - nearendNoisy  : In buffer containing one frame of nearend+echo signal
//                        without NS
//      - nearendClean  : In buffer containing one frame of nearend+echo signal
//                        with NS
//
// Output:
//      - out           : Out buffer, one frame of nearend signal          :
//
//
int WebRtcAecm_ProcessFrame(AecmCore* aecm,
                            const int16_t* farend,
                            const int16_t* nearendNoisy,
                            const int16_t* nearendClean,
                            int16_t* out);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_ProcessBlock(...)
//
// This function is called for every block within one frame
// This function is called by WebRtcAecm_ProcessFrame(...)
//
// Inputs:
//      - aecm          : Pointer to the AECM instance
//      - farend        : In buffer containing one block of echo signal
//      - nearendNoisy  : In buffer containing one frame of nearend+echo signal
//                        without NS
//      - nearendClean  : In buffer containing one frame of nearend+echo signal
//                        with NS
//
// Output:
//      - out           : Out buffer, one block of nearend signal          :
//
//
int WebRtcAecm_ProcessBlock(AecmCore* aecm,
                            const int16_t* farend,
                            const int16_t* nearendNoisy,
                            const int16_t* noisyClean,
                            int16_t* out);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_BufferFarFrame()
//
// Inserts a frame of data into farend buffer.
//
// Inputs:
//      - aecm          : Pointer to the AECM instance
//      - farend        : In buffer containing one frame of farend signal
//      - farLen        : Length of frame
//
void WebRtcAecm_BufferFarFrame(AecmCore* const aecm,
                               const int16_t* const farend,
                               int farLen);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_FetchFarFrame()
//
// Read the farend buffer to account for known delay
//
// Inputs:
//      - aecm          : Pointer to the AECM instance
//      - farend        : In buffer containing one frame of farend signal
//      - farLen        : Length of frame
//      - knownDelay    : known delay
//
void WebRtcAecm_FetchFarFrame(AecmCore* const aecm,
                              int16_t* const farend,
                              int farLen,
                              int knownDelay);

// All the functions below are intended to be private

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_UpdateFarHistory()
//
// Moves the pointer to the next entry and inserts `far_spectrum` and
// corresponding Q-domain in its buffer.
//
// Inputs:
//      - self          : Pointer to the delay estimation instance
//      - far_spectrum  : Pointer to the far end spectrum
//      - far_q         : Q-domain of far end spectrum
//
void WebRtcAecm_UpdateFarHistory(AecmCore* self,
                                 uint16_t* far_spectrum,
                                 int far_q);

////////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_AlignedFarend()
//
// Returns a pointer to the far end spectrum aligned to current near end
// spectrum. The function WebRtc_DelayEstimatorProcessFix(...) should have been
// called before AlignedFarend(...). Otherwise, you get the pointer to the
// previous frame. The memory is only valid until the next call of
// WebRtc_DelayEstimatorProcessFix(...).
//
// Inputs:
//      - self              : Pointer to the AECM instance.
//      - delay             : Current delay estimate.
//
// Output:
//      - far_q             : The Q-domain of the aligned far end spectrum
//
// Return value:
//      - far_spectrum      : Pointer to the aligned far end spectrum
//                            NULL - Error
//
const uint16_t* WebRtcAecm_AlignedFarend(AecmCore* self, int* far_q, int delay);

///////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_CalcSuppressionGain()
//
// This function calculates the suppression gain that is used in the
// Wiener filter.
//
// Inputs:
//      - aecm              : Pointer to the AECM instance.
//
// Return value:
//      - supGain           : Suppression gain with which to scale the noise
//                            level (Q14).
//
int16_t WebRtcAecm_CalcSuppressionGain(AecmCore* const aecm);

///////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_CalcEnergies()
//
// This function calculates the log of energies for nearend, farend and
// estimated echoes. There is also an update of energy decision levels,
// i.e. internal VAD.
//
// Inputs:
//      - aecm              : Pointer to the AECM instance.
//      - far_spectrum      : Pointer to farend spectrum.
//      - far_q             : Q-domain of farend spectrum.
//      - nearEner          : Near end energy for current block in
//                            Q(aecm->dfaQDomain).
//
// Output:
//     - echoEst            : Estimated echo in Q(xfa_q+RESOLUTION_CHANNEL16).
//
void WebRtcAecm_CalcEnergies(AecmCore* aecm,
                             const uint16_t* far_spectrum,
                             int16_t far_q,
                             uint32_t nearEner,
                             int32_t* echoEst);

///////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_CalcStepSize()
//
// This function calculates the step size used in channel estimation
//
// Inputs:
//      - aecm              : Pointer to the AECM instance.
//
// Return value:
//      - mu                : Stepsize in log2(), i.e. number of shifts.
//
int16_t WebRtcAecm_CalcStepSize(AecmCore* const aecm);

///////////////////////////////////////////////////////////////////////////////
// WebRtcAecm_UpdateChannel(...)
//
// This function performs channel estimation.
// NLMS and decision on channel storage.
//
// Inputs:
//      - aecm              : Pointer to the AECM instance.
//      - far_spectrum      : Absolute value of the farend signal in Q(far_q)
//      - far_q             : Q-domain of the farend signal
//      - dfa               : Absolute value of the nearend signal
//                            (Q[aecm->dfaQDomain])
//      - mu                : NLMS step size.
// Input/Output:
//      - echoEst           : Estimated echo in Q(far_q+RESOLUTION_CHANNEL16).
//
void WebRtcAecm_UpdateChannel(AecmCore* aecm,
                              const uint16_t* far_spectrum,
                              int16_t far_q,
                              const uint16_t* const dfa,
                              int16_t mu,
                              int32_t* echoEst);

extern const int16_t WebRtcAecm_kCosTable[];
extern const int16_t WebRtcAecm_kSinTable[];

///////////////////////////////////////////////////////////////////////////////
// Some function pointers, for internal functions shared by ARM NEON and
// generic C code.
//
typedef void (*CalcLinearEnergies)(AecmCore* aecm,
                                   const uint16_t* far_spectrum,
                                   int32_t* echoEst,
                                   uint32_t* far_energy,
                                   uint32_t* echo_energy_adapt,
                                   uint32_t* echo_energy_stored);
extern CalcLinearEnergies WebRtcAecm_CalcLinearEnergies;

typedef void (*StoreAdaptiveChannel)(AecmCore* aecm,
                                     const uint16_t* far_spectrum,
                                     int32_t* echo_est);
extern StoreAdaptiveChannel WebRtcAecm_StoreAdaptiveChannel;

typedef void (*ResetAdaptiveChannel)(AecmCore* aecm);
extern ResetAdaptiveChannel WebRtcAecm_ResetAdaptiveChannel;

// For the above function pointers, functions for generic platforms are declared
// and defined as static in file aecm_core.c, while those for ARM Neon platforms
// are declared below and defined in file aecm_core_neon.c.
#if defined(WEBRTC_HAS_NEON)
void WebRtcAecm_CalcLinearEnergiesNeon(AecmCore* aecm,
                                       const uint16_t* far_spectrum,
                                       int32_t* echo_est,
                                       uint32_t* far_energy,
                                       uint32_t* echo_energy_adapt,
                                       uint32_t* echo_energy_stored);

void WebRtcAecm_StoreAdaptiveChannelNeon(AecmCore* aecm,
                                         const uint16_t* far_spectrum,
                                         int32_t* echo_est);

void WebRtcAecm_ResetAdaptiveChannelNeon(AecmCore* aecm);
#endif

#if defined(MIPS32_LE)
void WebRtcAecm_CalcLinearEnergies_mips(AecmCore* aecm,
                                        const uint16_t* far_spectrum,
                                        int32_t* echo_est,
                                        uint32_t* far_energy,
                                        uint32_t* echo_energy_adapt,
                                        uint32_t* echo_energy_stored);
#if defined(MIPS_DSP_R1_LE)
void WebRtcAecm_StoreAdaptiveChannel_mips(AecmCore* aecm,
                                          const uint16_t* far_spectrum,
                                          int32_t* echo_est);

void WebRtcAecm_ResetAdaptiveChannel_mips(AecmCore* aecm);
#endif
#endif

}  // namespace webrtc

#endif
